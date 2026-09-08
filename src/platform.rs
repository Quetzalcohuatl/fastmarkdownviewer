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

#[must_use]
pub fn app_icon() -> eframe::egui::IconData {
    const SIZE: usize = 32;
    const SIZE_U32: u32 = 32;
    let mut rgba = Vec::with_capacity(SIZE * SIZE * 4);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let page = (5..27).contains(&x) && (3..29).contains(&y);
            let fold = x >= 21 && y < 9 && x - 21 >= 8 - y;
            let line = (9..24).contains(&x) && matches!(y, 12 | 17 | 22) && (!fold || y >= 12);
            let color = if line {
                [31, 41, 55, 255]
            } else if page && !fold {
                [243, 244, 246, 255]
            } else if page {
                [145, 164, 188, 255]
            } else {
                [30, 111, 214, 255]
            };
            rgba.extend_from_slice(&color);
        }
    }
    eframe::egui::IconData {
        rgba,
        width: SIZE_U32,
        height: SIZE_U32,
    }
}
