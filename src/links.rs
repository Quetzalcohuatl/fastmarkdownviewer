use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkAction {
    Browser(Url),
    Markdown {
        path: PathBuf,
        fragment: Option<String>,
    },
    Anchor(String),
    Inert,
}

/// Resolve a destination without handing an unvalidated scheme to the OS.
#[must_use]
pub fn resolve(destination: &str, document_path: Option<&Path>) -> LinkAction {
    let destination = destination.trim();
    if let Some(fragment) = destination.strip_prefix('#') {
        return LinkAction::Anchor(decode_fragment(fragment));
    }
    if destination.is_empty() {
        return LinkAction::Inert;
    }
    let (path_text, fragment) = destination
        .split_once('#')
        .map_or((destination, None), |(path, fragment)| {
            (path, Some(decode_fragment(fragment)))
        });
    // Native absolute paths must be handled before URL parsing sees a drive letter as a scheme.
    if Path::new(path_text).is_absolute() && !path_text.starts_with("//") {
        return markdown(PathBuf::from(path_text), fragment);
    }
    if let Ok(url) = Url::parse(destination) {
        return match url.scheme() {
            "http" | "https" if url.username().is_empty() && url.password().is_none() => {
                LinkAction::Browser(url)
            }
            "file" => file_action(url),
            _ => LinkAction::Inert,
        };
    }
    let Some(base) = document_path.and_then(|path| Url::from_file_path(path).ok()) else {
        return LinkAction::Inert;
    };
    base.join(destination).map_or(LinkAction::Inert, |url| {
        if url.scheme() == "file" {
            file_action(url)
        } else {
            LinkAction::Inert
        }
    })
}
fn file_action(mut url: Url) -> LinkAction {
    let fragment = url.fragment().map(decode_fragment);
    url.set_fragment(None);
    url.set_query(None);
    url.to_file_path()
        .map_or(LinkAction::Inert, |path| markdown(path, fragment))
}
fn markdown(path: PathBuf, fragment: Option<String>) -> LinkAction {
    if is_markdown(&path) {
        LinkAction::Markdown { path, fragment }
    } else {
        LinkAction::Inert
    }
}
#[must_use]
pub fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("md") || extension.eq_ignore_ascii_case("markdown")
        })
}
fn decode_fragment(fragment: &str) -> String {
    // URL form decoding handles percent-encoded UTF-8; protect literal plus signs first.
    url::form_urlencoded::parse(
        format!("f={}", fragment.replace('+', "%2B").replace('&', "%26")).as_bytes(),
    )
    .next()
    .map_or_else(String::new, |(_, value)| value.into_owned())
}

/// Practical heading slugs: Unicode lowercase, whitespace as hyphens, punctuation removed.
#[must_use]
pub fn heading_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_whitespace() {
                Some('-')
            } else if c.is_alphanumeric() || matches!(c, '-' | '_') {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_links_keep_fragments_and_decode_paths() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("README.md");
        for (link, name, fragment) in [
            ("foo.md", "foo.md", None),
            (
                "docs/bar.markdown#Some%20heading",
                "docs/bar.markdown",
                Some("Some heading"),
            ),
            ("chapter%20one.md", "chapter one.md", None),
        ] {
            assert_eq!(
                resolve(link, Some(&source)),
                LinkAction::Markdown {
                    path: dir.path().join(name),
                    fragment: fragment.map(str::to_owned)
                }
            );
        }
        let target = dir.path().join("absolute.md");
        assert_eq!(
            resolve(target.to_str().unwrap(), Some(&source)),
            markdown(target.clone(), None)
        );
        assert_eq!(
            resolve(
                &format!("{}#hello", Url::from_file_path(&target).unwrap()),
                Some(&source)
            ),
            markdown(target, Some("hello".into()))
        );
        assert_eq!(
            resolve("#caf%C3%A9+tea", Some(&source)),
            LinkAction::Anchor("café+tea".into())
        );
    }
    #[test]
    fn unsafe_links_remain_inert() {
        for link in [
            "javascript:alert(1)",
            "mailto:x@y",
            "https://user:pass@example.com",
            "file:///C:/data.exe",
        ] {
            assert_eq!(resolve(link, None), LinkAction::Inert);
        }
        assert!(matches!(
            resolve("https://example.com/a?b=c#d", None),
            LinkAction::Browser(_)
        ));
    }
    #[test]
    fn practical_heading_slugs() {
        assert_eq!(heading_slug("Hello, **World**!"), "hello-world");
        assert_eq!(heading_slug("日本語 Café"), "日本語-café");
    }
}
