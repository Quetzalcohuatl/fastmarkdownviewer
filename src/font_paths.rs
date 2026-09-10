//! Curated font discovery without loading every installed font into memory.
use std::path::PathBuf;

pub(crate) fn find(name: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let system = std::env::var_os("WINDIR")
            .map_or_else(|| PathBuf::from("C:/Windows"), PathBuf::from)
            .join("Fonts")
            .join(name);
        if system.is_file() {
            return Some(system);
        }
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|path| path.join("Microsoft/Windows/Fonts").join(name))
            .filter(|path| path.is_file())
    }
    #[cfg(not(target_os = "windows"))]
    {
        use std::{collections::BTreeMap, sync::OnceLock};
        static FILES: OnceLock<BTreeMap<String, PathBuf>> = OnceLock::new();
        FILES.get_or_init(index).get(name).cloned()
    }
}

#[cfg(not(target_os = "windows"))]
fn index() -> std::collections::BTreeMap<String, PathBuf> {
    fn visit(path: &std::path::Path, files: &mut std::collections::BTreeMap<String, PathBuf>) {
        let Ok(entries) = std::fs::read_dir(path) else {
            return;
        };
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_dir() {
                visit(&entry.path(), files);
            } else if kind.is_file() {
                files
                    .entry(entry.file_name().to_string_lossy().into_owned())
                    .or_insert_with(|| entry.path());
            }
        }
    }
    let mut roots = Vec::new();
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        #[cfg(target_os = "macos")]
        roots.push(home.join("Library/Fonts"));
        #[cfg(not(target_os = "macos"))]
        {
            roots.push(
                std::env::var_os("XDG_DATA_HOME")
                    .map_or_else(|| home.join(".local/share"), PathBuf::from)
                    .join("fonts"),
            );
            roots.push(home.join(".fonts"));
        }
    }
    #[cfg(target_os = "macos")]
    roots.extend([
        PathBuf::from("/Library/Fonts"),
        PathBuf::from("/System/Library/Fonts"),
    ]);
    #[cfg(not(target_os = "macos"))]
    roots.extend([
        PathBuf::from("/usr/local/share/fonts"),
        PathBuf::from("/usr/share/fonts"),
    ]);
    let mut files = std::collections::BTreeMap::new();
    for root in roots {
        visit(&root, &mut files);
    }
    files
}
