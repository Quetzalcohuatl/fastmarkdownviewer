#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::process::Command;
use std::{io, path::Path};

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

/// Reveal a file in the system file manager, selecting it where supported.
///
/// # Errors
///
/// Returns an OS error when the path is missing or the file manager cannot start.
pub fn reveal_in_file_manager(path: &Path) -> io::Result<()> {
    let path = path.canonicalize()?;
    #[cfg(target_os = "windows")]
    {
        let text = path.to_string_lossy();
        let normal = text.strip_prefix(r"\\?\UNC\").map_or_else(
            || text.strip_prefix(r"\\?\").unwrap_or(&text).to_owned(),
            |unc| format!(r"\\{unc}"),
        );
        Command::new("explorer.exe")
            .arg(format!("/select,{normal}"))
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    Command::new("open").arg("-R").arg(&path).spawn()?;
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    open::that(
        path.parent()
            .ok_or_else(|| io::Error::other("No parent folder"))?,
    )?;
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

/// Format the primary keyboard modifier for this desktop.
#[must_use]
pub fn shortcut_label(label: &str) -> std::borrow::Cow<'_, str> {
    if cfg!(target_os = "macos") && !label.contains("Ctrl+Tab") {
        label.replace("Ctrl+", "Cmd+").into()
    } else {
        label.into()
    }
}
