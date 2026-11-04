//! Route desktop and command-line opens to the viewer that owns this profile.
use std::{
    ffi::OsString,
    fs::{self, File, TryLockError},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
#[cfg(windows)]
use uds_windows::{UnixListener, UnixStream};

const MAX_REQUEST_BYTES: usize = 4 * 1024 * 1024;
const ACK: [u8; 1] = [1];

pub enum Launch {
    Primary(Session),
    Forwarded,
    WithoutProfile,
}

pub struct Session {
    requests: Receiver<OpenRequest>,
    context: Arc<Mutex<Option<eframe::egui::Context>>>,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
    socket: PathBuf,
    address_file: PathBuf,
    _socket_dir: tempfile::TempDir,
    _lock: File,
}

pub(crate) struct OpenRequest {
    paths: Vec<PathBuf>,
    reply: UnixStream,
}

impl OpenRequest {
    pub fn acknowledge(mut self) -> Vec<PathBuf> {
        let _ = self.reply.write_all(&ACK);
        self.paths
    }
}

impl Session {
    /// Claim the profile or deliver the files to its current viewer.
    ///
    /// # Errors
    /// Returns an error if the profile or its local socket cannot be used.
    pub fn start(paths: &[PathBuf]) -> io::Result<Launch> {
        let state = crate::persistence::path();
        let Some(profile) = state.as_deref().and_then(Path::parent) else {
            return Ok(Launch::WithoutProfile);
        };
        Self::start_in(profile, paths)
    }

    fn start_in(profile: &Path, paths: &[PathBuf]) -> io::Result<Launch> {
        fs::create_dir_all(profile)?;
        let lock = File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .open(profile.join("viewer.lock"))?;
        let address_file = profile.join("viewer.socket");
        let deadline = Instant::now() + Duration::from_secs(12);
        loop {
            match lock.try_lock() {
                Ok(()) => return Ok(Launch::Primary(Self::listen(lock, address_file)?)),
                Err(TryLockError::Error(error)) => return Err(error),
                Err(TryLockError::WouldBlock) => {
                    // The owner may still be publishing its address, or may be exiting.
                    match forward(&address_file, paths) {
                        Ok(()) => return Ok(Launch::Forwarded),
                        Err(error) if Instant::now() >= deadline => return Err(error),
                        Err(_) => thread::sleep(Duration::from_millis(40)),
                    }
                }
            }
        }
    }

    fn listen(lock: File, address_file: PathBuf) -> io::Result<Self> {
        // A short, private temp directory keeps long profile paths out of the socket address.
        let mut builder = tempfile::Builder::new();
        builder.prefix("fmv-");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            builder.permissions(fs::Permissions::from_mode(0o700));
        }
        let socket_dir = builder.tempdir()?;
        let socket = socket_dir.path().join("open");
        let listener = UnixListener::bind(&socket)?;
        let address = tempfile::NamedTempFile::new_in(
            address_file.parent().expect("socket address has a parent"),
        )?;
        serde_json::to_writer(address.as_file(), socket.as_os_str())?;
        address.persist(&address_file)?;

        let (sender, requests) = mpsc::sync_channel(64);
        let context = Arc::new(Mutex::new(None));
        let stop = Arc::new(AtomicBool::new(false));
        let wake_context = Arc::clone(&context);
        let worker_stop = Arc::clone(&stop);
        let worker = thread::Builder::new()
            .name("fmv-open".into())
            .spawn(move || {
                for stream in listener.incoming() {
                    if worker_stop.load(Ordering::Acquire) {
                        break;
                    }
                    let Ok(stream) = stream else { break };
                    if receive(stream, &sender).is_ok() {
                        let context = wake_context.lock().expect("launch context lock");
                        if let Some(context) = context.as_ref() {
                            repaint_root(context);
                        }
                    }
                }
            })?;
        Ok(Self {
            requests,
            context,
            stop,
            worker: Some(worker),
            socket,
            address_file,
            _socket_dir: socket_dir,
            _lock: lock,
        })
    }

    pub(crate) fn attach(&self, context: &eframe::egui::Context) {
        *self.context.lock().expect("launch context lock") = Some(context.clone());
        repaint_root(context);
    }

    pub(crate) fn next(&self) -> Option<OpenRequest> {
        self.requests.try_recv().ok()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        // Wake a worker blocked in accept before removing the socket and releasing the lock.
        if UnixStream::connect(&self.socket).is_ok()
            && let Some(worker) = self.worker.take()
        {
            let _ = worker.join();
        }
        let _ = fs::remove_file(&self.address_file);
    }
}

fn repaint_root(context: &eframe::egui::Context) {
    context.request_repaint_of(eframe::egui::ViewportId::ROOT);
}

fn forward(address_file: &Path, paths: &[PathBuf]) -> io::Result<()> {
    let socket: OsString = serde_json::from_reader(File::open(address_file)?.take(8192))?;
    let paths: Vec<_> = paths.iter().map(|path| path.as_os_str()).collect();
    let data = serde_json::to_vec(&paths)?;
    if data.len() > MAX_REQUEST_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Too many file paths",
        ));
    }
    let mut stream = UnixStream::connect(Path::new(&socket))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    stream.set_read_timeout(Some(Duration::from_secs(4)))?;
    let len = u32::try_from(data.len()).expect("request length is bounded");
    stream.write_all(&len.to_le_bytes())?;
    stream.write_all(&data)?;
    let mut ack = [0];
    stream.read_exact(&mut ack)?;
    if ack != ACK {
        return Err(io::ErrorKind::InvalidData.into());
    }
    Ok(())
}

fn receive(mut stream: UnixStream, sender: &SyncSender<OpenRequest>) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    stream.set_write_timeout(Some(Duration::from_secs(1)))?;
    let mut len = [0; 4];
    stream.read_exact(&mut len)?;
    let len = u32::from_le_bytes(len) as usize;
    if len > MAX_REQUEST_BYTES {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let mut data = vec![0; len];
    stream.read_exact(&mut data)?;
    let paths: Vec<OsString> = serde_json::from_slice(&data)?;
    let paths: Vec<_> = paths.into_iter().map(PathBuf::from).collect();
    if paths.iter().any(|path| !path.is_absolute()) {
        return Err(io::ErrorKind::InvalidData.into());
    }
    sender
        .try_send(OpenRequest {
            paths,
            reply: stream,
        })
        .map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_launch_delivers_paths_and_profile_can_be_reopened() {
        let profile = tempfile::tempdir().unwrap();
        let Launch::Primary(primary) = Session::start_in(profile.path(), &[]).unwrap() else {
            panic!("a fresh profile should start its own session");
        };
        let paths = [
            profile.path().join("First 日本語.md"),
            profile.path().join("Second.md"),
        ];
        thread::scope(|scope| {
            let secondary = scope.spawn(|| Session::start_in(profile.path(), &paths));
            let request = primary
                .requests
                .recv_timeout(Duration::from_secs(3))
                .unwrap();
            assert_eq!(request.acknowledge(), paths);
            assert!(matches!(
                secondary.join().unwrap().unwrap(),
                Launch::Forwarded
            ));
        });
        drop(primary);
        assert!(matches!(
            Session::start_in(profile.path(), &[]).unwrap(),
            Launch::Primary(_)
        ));
    }
}
