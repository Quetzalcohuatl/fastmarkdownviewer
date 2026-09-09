use crate::{document::Document, links::is_markdown};
use std::path::{Path, PathBuf};

/// Validate a filename-only rename in the current folder.
/// # Errors
/// Rejects invalid filenames, paths, reserved names, and non-Markdown extensions.
pub fn rename_path(source: &Path, name: &str) -> Result<PathBuf, String> {
    if name.is_empty()
        || name.trim() != name
        || name.ends_with('.')
        || name
            .chars()
            .any(|c| c.is_control() || "<>:\"/\\|?*".contains(c))
    {
        return Err(
            "Enter a filename without folders, reserved characters, or trailing spaces/dots."
                .into(),
        );
    }
    let stem = name
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    if stem.is_empty()
        || ["CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"].contains(&stem.as_str())
        || (stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.len() == 4
            && matches!(stem.as_bytes()[3], b'1'..=b'9')
    {
        return Err("That filename is reserved by the operating system.".into());
    }
    let mut filename = PathBuf::from(name);
    if filename.extension().is_none() {
        filename.set_extension(if is_markdown(source) {
            source
                .extension()
                .unwrap_or_else(|| std::ffi::OsStr::new("md"))
        } else {
            std::ffi::OsStr::new("md")
        });
    }
    if !is_markdown(&filename) {
        return Err("Use a .md or .markdown extension.".into());
    }
    Ok(source
        .parent()
        .ok_or("The file has no parent folder.")?
        .join(filename))
}

/// Rename without overwriting another file, then reload all document metadata.
/// # Errors
/// Returns validation, filesystem, or document-load errors. Filesystems must support hard links.
pub fn rename_document(source: &Path, name: &str) -> Result<Document, String> {
    let destination = rename_path(source, name)?;
    if destination == source {
        return Document::load(source).map_err(|error| error.to_string());
    }
    if destination
        .try_exists()
        .map_err(|error| error.to_string())?
    {
        return Err("A file with that name already exists.".into());
    }
    let metadata = std::fs::symlink_metadata(source)
        .map_err(|error| format!("Cannot rename file: {error}"))?;
    if !metadata.file_type().is_file() {
        return Err("Only regular files can be renamed.".into());
    }
    Document::load(source).map_err(|error| error.to_string())?;
    // Creating the new directory entry is exclusive: even a concurrent collision cannot overwrite.
    // Keep the original until the new path has loaded successfully.
    std::fs::hard_link(source, &destination).map_err(|error| {
        format!("Cannot rename file (the filesystem must support hard links): {error}")
    })?;
    let result = Document::load(&destination)
        .map_err(|error| error.to_string())
        .and_then(|document| {
            std::fs::remove_file(source)
                .map_err(|error| format!("Cannot remove the old filename: {error}"))?;
            Ok(document)
        });
    if result.is_err() {
        std::fs::remove_file(&destination).map_err(|error| {
            format!(
                "Rename failed; the original remains, but cleanup of {} failed: {error}",
                destination.display()
            )
        })?;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_names_and_preserves_markdown_extension() {
        let source = Path::new("notes/readme.markdown");
        assert_eq!(
            rename_path(source, "new name").unwrap(),
            Path::new("notes/new name.markdown")
        );
        assert_eq!(
            rename_path(source, "new.md").unwrap(),
            Path::new("notes/new.md")
        );
        for name in [
            "",
            "../move.md",
            "sub/file.md",
            "a\\b.md",
            "CON.md",
            "x.exe",
            "x.txt",
            "a.",
            "a.md ",
            "C:foo.md",
            "LPT1.md",
        ] {
            assert!(rename_path(source, name).is_err(), "accepted {name}");
        }
    }
    #[test]
    fn rename_reloads_paths_and_rejects_collisions() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("old.md");
        std::fs::write(&source, "# Hello").unwrap();
        let destination = dir.path().join("new.md");
        std::fs::write(&destination, "existing").unwrap();
        assert!(rename_document(&source, "new.md").is_err());
        assert_eq!(std::fs::read_to_string(&destination).unwrap(), "existing");
        std::fs::remove_file(&destination).unwrap();
        let document = rename_document(&source, "new").unwrap();
        assert!(!source.exists());
        assert_eq!(document.path, destination.canonicalize().unwrap());
        assert_eq!(document.title, "new.md");
        assert_eq!(
            document.base_uri,
            url::Url::from_directory_path(document.path.parent().unwrap())
                .unwrap()
                .as_str()
        );
        assert!(rename_document(&source, "missing").is_err());
    }
}
