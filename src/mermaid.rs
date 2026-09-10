use eframe::egui;
use std::{
    collections::HashMap,
    path::Path,
    process::{Command, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

static BUSY: AtomicBool = AtomicBool::new(false);
const INPUT_LIMIT: usize = 64 * 1024;
const CACHE_LIMIT: usize = 32 * 1024 * 1024;

#[derive(Clone)]
enum Entry {
    Pending,
    Ready(egui::TextureHandle),
    Failed(String),
}
#[derive(Default)]
struct State {
    entries: HashMap<String, Entry>,
    bytes: usize,
}
#[derive(Clone, Default)]
pub struct MermaidRenderer {
    state: Arc<Mutex<State>>,
}

impl MermaidRenderer {
    pub fn show(&self, ui: &mut egui::Ui, source: &str) {
        if source.len() > INPUT_LIMIT {
            ui.weak("Mermaid: diagram exceeds 64 KiB. Source shown below.");
            return;
        }
        let entry = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .entries
            .get(source)
            .cloned();
        match entry {
            Some(Entry::Ready(texture)) => {
                ui.add(
                    egui::Image::new(&texture)
                        .max_width(ui.available_width())
                        .alt_text("Mermaid diagram; source below"),
                );
            }
            Some(Entry::Failed(error)) => {
                ui.weak(format!("Mermaid unavailable: {error}. Source shown below."));
            }
            Some(Entry::Pending) => {
                ui.weak("Rendering Mermaid diagram…");
            }
            None => {
                let count = self
                    .state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .entries
                    .len();
                if count >= 32 {
                    ui.weak("Mermaid: diagram cache is full. Source shown below.");
                    return;
                }
                if BUSY
                    .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    let source = source.to_owned();
                    let state = self.state.clone();
                    let context = ui.ctx().clone();
                    state
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .entries
                        .insert(source.clone(), Entry::Pending);
                    let failure_source = source.clone();
                    let failure_state = state.clone();
                    let result = std::thread::Builder::new()
                        .name("Mermaid renderer".into())
                        .spawn(move || {
                            let result = render_in_helper(&source);
                            let mut state = state
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner);
                            let entry = match result {
                                Ok(image)
                                    if state.bytes + image.pixels.len() * 4 <= CACHE_LIMIT =>
                                {
                                    state.bytes += image.pixels.len() * 4;
                                    Entry::Ready(context.load_texture(
                                        "Mermaid",
                                        image,
                                        egui::TextureOptions::LINEAR,
                                    ))
                                }
                                Ok(_) => Entry::Failed("32 MiB image cache limit reached".into()),
                                Err(error) => Entry::Failed(error),
                            };
                            state.entries.insert(source, entry);
                            BUSY.store(false, Ordering::Release);
                            context.request_repaint();
                        });
                    if let Err(error) = result {
                        BUSY.store(false, Ordering::Release);
                        failure_state
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .entries
                            .insert(failure_source, Entry::Failed(error.to_string()));
                    }
                }
                ui.weak("Waiting to render Mermaid diagram…");
                ui.ctx().request_repaint_after(Duration::from_millis(100));
            }
        }
    }
}

fn render_in_helper(source: &str) -> Result<egui::ColorImage, String> {
    let directory = tempfile::tempdir().map_err(|e| e.to_string())?;
    let input = directory.path().join("diagram.mmd");
    let output = directory.path().join("diagram.png");
    std::fs::write(&input, source).map_err(|e| e.to_string())?;
    let mut command = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
    command
        .arg("--internal-mermaid")
        .arg(&input)
        .arg(&output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut child = command.spawn().map_err(|e| e.to_string())?;
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if start.elapsed() < Duration::from_secs(10) => {
                std::thread::sleep(Duration::from_millis(20));
            }
            other => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(match other {
                    Err(e) => e.to_string(),
                    _ => "rendering timed out".into(),
                });
            }
        }
    };
    if !status.success() {
        return Err(std::fs::read_to_string(output.with_extension("error"))
            .unwrap_or_else(|_| "renderer stopped unexpectedly".into()));
    }
    let bytes = std::fs::read(output).map_err(|e| e.to_string())?;
    let image = image::load_from_memory(&bytes)
        .map_err(|e| e.to_string())?
        .to_rgba8();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        image.as_raw(),
    ))
}

/// Dispatch the private rendering process before creating any windows.
/// Returns normally only when this is not a helper invocation.
pub fn handle_helper_args() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.get(1).is_none_or(|arg| arg != "--internal-mermaid") {
        return;
    }
    if args.len() != 4 {
        std::process::exit(2);
    }
    let input = Path::new(&args[2]);
    let output = Path::new(&args[3]);
    let result = std::fs::metadata(input)
        .map_err(|e| e.to_string())
        .and_then(|metadata| {
            if metadata.len() > INPUT_LIMIT as u64 {
                return Err("diagram exceeds 64 KiB".into());
            }
            crate::mermaid_worker::render(input, output)
        });
    match result {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            let _ = std::fs::write(output.with_extension("error"), error);
            std::process::exit(2);
        }
    }
}
