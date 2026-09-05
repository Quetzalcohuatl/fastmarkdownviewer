use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Open(Option<PathBuf>),
    Version,
    Error(String),
}

pub fn parse<I>(args: I) -> Command
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let _program = args.next();
    let Some(first) = args.next() else {
        return Command::Open(None);
    };

    if first == "--version" || first == "-V" {
        return if args.next().is_none() {
            Command::Version
        } else {
            Command::Error("--version does not accept a file path".to_owned())
        };
    }

    if first == "--help" || first == "-h" {
        return Command::Error(
            "Usage: FastMarkdownViewer.exe [PATH]\n       FastMarkdownViewer.exe --version"
                .to_owned(),
        );
    }

    if args.next().is_some() {
        return Command::Error("Open one Markdown file at a time".to_owned());
    }

    Command::Open(Some(PathBuf::from(first)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<OsString> {
        values.iter().map(OsString::from).collect()
    }

    #[test]
    fn accepts_zero_or_one_path() {
        assert_eq!(parse(strings(&["fmv"])), Command::Open(None));
        assert_eq!(
            parse(strings(&["fmv", "a file.md"])),
            Command::Open(Some(PathBuf::from("a file.md")))
        );
    }

    #[test]
    fn rejects_multiple_paths() {
        assert!(matches!(
            parse(strings(&["fmv", "a.md", "b.md"])),
            Command::Error(_)
        ));
    }

    #[test]
    fn accepts_version_flag() {
        assert_eq!(parse(strings(&["fmv", "--version"])), Command::Version);
    }
}
