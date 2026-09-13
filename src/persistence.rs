//! Small, local exit-time snapshot. Never stores document contents or render caches.
use crate::appearance::ThemeChoice;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

const MAX_STATE_BYTES: u64 = 1024 * 1024;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct State {
    pub version: u32,
    pub theme: ThemeChoice,
    pub fonts: [Option<String>; 2],
    pub zoom: f32,
    pub automatic_images: bool,
    pub word_wrap: bool,
    pub windows: Vec<Window>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            version: 1,
            theme: ThemeChoice::System,
            fonts: [None, None],
            zoom: 1.0,
            automatic_images: true,
            word_wrap: true,
            windows: Vec::new(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct Window {
    pub tabs: Vec<Tab>,
    pub active: usize,
    pub outline: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Tab {
    pub path: PathBuf,
    pub scroll: f32,
}

pub(crate) fn path() -> Option<PathBuf> {
    let env_path = |name| {
        std::env::var_os(name)
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    };
    #[cfg(target_os = "windows")]
    let base = env_path("APPDATA")?;
    #[cfg(target_os = "macos")]
    let base = env_path("HOME")?.join("Library/Application Support");
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let base = env_path("XDG_CONFIG_HOME")
        .filter(|path| path.is_absolute())
        .or_else(|| env_path("HOME").map(|home| home.join(".config")))?;
    Some(base.join("FastMarkdownViewer").join("state.json"))
}

pub(crate) fn read(path: &Path) -> State {
    let read = || -> Option<State> {
        let mut bytes = Vec::new();
        std::fs::File::open(path)
            .ok()?
            .take(MAX_STATE_BYTES + 1)
            .read_to_end(&mut bytes)
            .ok()?;
        if bytes.len() as u64 > MAX_STATE_BYTES {
            return None;
        }
        let mut state: State = serde_json::from_slice(&bytes).ok()?;
        if state.version != 1 {
            return None;
        }
        state.zoom = if state.zoom.is_finite() {
            state.zoom.clamp(0.5, 3.0)
        } else {
            1.0
        };
        // Bound restoration independently of how much a corrupt file claims to contain.
        state.windows.truncate(32);
        for window in &mut state.windows {
            window.tabs.truncate(256);
            window.tabs.retain(|tab| tab.path.is_absolute());
            for tab in &mut window.tabs {
                tab.scroll = if tab.scroll.is_finite() {
                    tab.scroll.max(0.0)
                } else {
                    0.0
                };
            }
        }
        Some(state)
    };
    read().unwrap_or_default()
}

pub(crate) fn write(path: &Path, state: &State) -> io::Result<()> {
    let bytes = serde_json::to_vec(state)?;
    if bytes.len() as u64 > MAX_STATE_BYTES {
        return Err(io::Error::other("Session exceeds the 1 MiB settings limit"));
    }
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("Missing settings directory"))?;
    std::fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(&bytes)?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atomic_replace_and_invalid_state_fallback() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings/state.json");
        let mut state = State {
            theme: ThemeChoice::Light,
            zoom: 1.5,
            ..State::default()
        };
        write(&path, &state).unwrap();
        assert_eq!(read(&path).theme, ThemeChoice::Light);
        state.theme = ThemeChoice::Monokai;
        write(&path, &state).unwrap();
        assert_eq!(read(&path).theme, ThemeChoice::Monokai);
        std::fs::write(&path, b"{truncated").unwrap();
        assert_eq!(read(&path).theme, ThemeChoice::System);
        std::fs::write(&path, br#"{"version":99,"zoom":2}"#).unwrap();
        assert!((read(&path).zoom - 1.0).abs() < f32::EPSILON);
        std::fs::write(&path, br#"{"zoom":999}"#).unwrap();
        assert!((read(&path).zoom - 3.0).abs() < f32::EPSILON);
    }
}
