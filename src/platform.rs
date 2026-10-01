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

/// Decode the platform artwork once; detached windows reuse the cached pixels.
/// The OS packages also carry multi-resolution icons for launchers and file managers.
///
/// # Panics
///
/// Panics if the embedded, build-verified PNG asset is corrupt.
#[must_use]
pub fn app_icon() -> eframe::egui::IconData {
    static ICON: std::sync::OnceLock<eframe::egui::IconData> = std::sync::OnceLock::new();
    ICON.get_or_init(|| {
        #[cfg(target_os = "windows")]
        let png = include_bytes!("../assets/icons/windows/256.png").as_slice();
        #[cfg(target_os = "macos")]
        let png = include_bytes!("../assets/icons/macos.iconset/icon_128x128@2x.png").as_slice();
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let png = include_bytes!("../assets/icons/linux/256.png").as_slice();
        eframe::icon_data::from_png_bytes(png).expect("Bundled application icon is a valid PNG")
    })
    .clone()
}

/// Format the primary keyboard modifier for this desktop.
#[must_use]
pub fn shortcut_label(label: &str) -> std::borrow::Cow<'_, str> {
    if cfg!(target_os = "macos") && !label.contains("Ctrl+Tab") {
        // Cmd+H belongs to macOS (Hide). Advertise our existing alternate instead.
        label
            .replace("Ctrl+H", "Cmd+Shift+O")
            .replace("Ctrl+", "Cmd+")
            .into()
    } else {
        label.into()
    }
}
