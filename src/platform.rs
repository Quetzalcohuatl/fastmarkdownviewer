use std::{io, path::Path, process::Command};

#[must_use]
pub fn choose_markdown_file() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Markdown", &["md", "markdown"])
        .add_filter("Text", &["txt"])
        .pick_file()
}

/// Open a validated HTTP(S) URL in the default browser.
///
/// # Errors
///
/// Returns an OS error when the shell cannot open the browser.
pub fn open_browser(url: &url::Url) -> io::Result<()> {
    open::that(url.as_str())
}

/// Start an independent viewer process for a Markdown path.
///
/// # Errors
///
/// Returns an OS error when the executable cannot be located or started.
pub fn open_markdown(path: &Path) -> io::Result<()> {
    Command::new(std::env::current_exe()?).arg(path).spawn()?;
    Ok(())
}
