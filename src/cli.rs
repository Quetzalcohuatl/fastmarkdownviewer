use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Open(Vec<PathBuf>),
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
        return Command::Open(Vec::new());
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
            "Usage: FastMarkdownViewer.exe [PATH ...]\n       FastMarkdownViewer.exe --version"
                .to_owned(),
        );
    }

    Command::Open(
        std::iter::once(first)
            .chain(args)
            .map(PathBuf::from)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<OsString> {
        values.iter().map(OsString::from).collect()
    }

    #[test]
    fn accepts_zero_or_more_paths() {
        assert_eq!(parse(strings(&["fmv"])), Command::Open(Vec::new()));
        assert_eq!(
            parse(strings(&["fmv", "a file.md"])),
            Command::Open(vec![PathBuf::from("a file.md")])
        );
        assert_eq!(
            parse(strings(&["fmv", "a file.md", "b.md"])),
            Command::Open(vec![PathBuf::from("a file.md"), PathBuf::from("b.md")])
        );
    }

    #[test]
    fn accepts_version_flag() {
        assert_eq!(parse(strings(&["fmv", "--version"])), Command::Version);
    }
}
