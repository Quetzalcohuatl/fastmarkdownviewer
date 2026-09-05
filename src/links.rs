use std::path::{Path, PathBuf};

use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkAction {
    Browser(Url),
    Markdown(PathBuf),
    Inert,
}

/// Resolve a clicked destination without ever handing an unvalidated scheme to
/// the operating system.
#[must_use]
pub fn resolve(destination: &str, document_path: Option<&Path>) -> LinkAction {
    let destination = destination.trim();
    if destination.is_empty() || destination.starts_with('#') {
        return LinkAction::Inert;
    }

    if let Ok(url) = Url::parse(destination) {
        return match url.scheme() {
            "http" | "https" if url.username().is_empty() && url.password().is_none() => {
                LinkAction::Browser(url)
            }
            "file" => url
                .to_file_path()
                .ok()
                .filter(|path| is_markdown(path))
                .map_or(LinkAction::Inert, LinkAction::Markdown),
            _ => LinkAction::Inert,
        };
    }

    let Some(parent) = document_path.and_then(Path::parent) else {
        return LinkAction::Inert;
    };
    let candidate = parent.join(destination);
    if is_markdown(&candidate) {
        LinkAction::Markdown(candidate)
    } else {
        LinkAction::Inert
    }
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("md") || extension.eq_ignore_ascii_case("markdown")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permits_only_browser_links_without_credentials() {
        assert!(matches!(
            resolve("https://example.com/readme", None),
            LinkAction::Browser(_)
        ));
        assert_eq!(
            resolve("https://user:secret@example.com", None),
            LinkAction::Inert
        );
        assert_eq!(resolve("javascript:alert(1)", None), LinkAction::Inert);
    }

    #[test]
    fn resolves_relative_markdown_but_not_other_files() {
        let source = Path::new("C:/notes/index.md");
        assert_eq!(
            resolve("chapter one.md", Some(source)),
            LinkAction::Markdown(PathBuf::from("C:/notes/chapter one.md"))
        );
        assert_eq!(resolve("data.exe", Some(source)), LinkAction::Inert);
    }
}
