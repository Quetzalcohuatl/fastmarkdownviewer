use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use url::Url;

use crate::render_source::normalize_for_rendering;

pub const MAX_DOCUMENT_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf,
    pub title: String,
    pub source: String,
    pub base_uri: String,
}

#[derive(Debug)]
pub enum LoadError {
    Metadata { path: PathBuf, source: io::Error },
    NotAFile(PathBuf),
    TooLarge { path: PathBuf, size: u64 },
    Read { path: PathBuf, source: io::Error },
    InvalidUtf8(PathBuf),
    InvalidBasePath(PathBuf),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Metadata { path, source } => {
                write!(f, "Cannot open {}: {source}", path.display())
            }
            Self::NotAFile(path) => write!(f, "{} is not a regular file", path.display()),
            Self::TooLarge { path, size } => {
                let whole = size / 1_048_576;
                let tenths = (size % 1_048_576) * 10 / 1_048_576;
                write!(
                    f,
                    "{} is too large ({whole}.{tenths} MiB). The v0.1 limit is 32 MiB.",
                    path.display()
                )
            }
            Self::Read { path, source } => write!(f, "Cannot read {}: {source}", path.display()),
            Self::InvalidUtf8(path) => write!(
                f,
                "{} is not valid UTF-8. FastMarkdownViewer accepts UTF-8 Markdown files.",
                path.display()
            ),
            Self::InvalidBasePath(path) => {
                write!(f, "Cannot resolve the folder containing {}", path.display())
            }
        }
    }
}

impl std::error::Error for LoadError {}

impl Document {
    /// Load and validate one UTF-8 text document.
    ///
    /// # Errors
    ///
    /// Returns a [`LoadError`] for missing, unreadable, non-file, oversized, or
    /// invalid UTF-8 input, or when its resource base cannot be represented.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        let path = path.as_ref();
        let metadata = fs::metadata(path).map_err(|source| LoadError::Metadata {
            path: path.to_owned(),
            source,
        })?;
        if !metadata.is_file() {
            return Err(LoadError::NotAFile(path.to_owned()));
        }
        if metadata.len() > MAX_DOCUMENT_BYTES {
            return Err(LoadError::TooLarge {
                path: path.to_owned(),
                size: metadata.len(),
            });
        }

        let bytes = fs::read(path).map_err(|source| LoadError::Read {
            path: path.to_owned(),
            source,
        })?;
        let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(&bytes);
        let source =
            std::str::from_utf8(bytes).map_err(|_| LoadError::InvalidUtf8(path.to_owned()))?;
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_owned());
        let parent = canonical
            .parent()
            .ok_or_else(|| LoadError::InvalidBasePath(canonical.clone()))?;
        let base_uri = Url::from_directory_path(parent)
            .map_err(|()| LoadError::InvalidBasePath(canonical.clone()))?
            .to_string();
        let title = canonical.file_name().map_or_else(
            || "Markdown".to_owned(),
            |name| name.to_string_lossy().into_owned(),
        );

        Ok(Self {
            path: canonical,
            title,
            source: normalize_for_rendering(source),
            base_uri,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn loads_utf8_and_strips_bom() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(b"\xEF\xBB\xBF# Hello").unwrap();
        let document = Document::load(file.path()).unwrap();
        assert_eq!(document.source, "# Hello");
    }

    #[test]
    fn rejects_invalid_utf8() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(&[0xFF, 0xFE]).unwrap();
        assert!(matches!(
            Document::load(file.path()),
            Err(LoadError::InvalidUtf8(_))
        ));
    }

    #[test]
    fn rejects_directories() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            Document::load(dir.path()),
            Err(LoadError::NotAFile(_))
        ));
    }
}
