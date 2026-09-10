//! Exercise the actual executable's isolated Mermaid renderer.
use std::{fmt::Write, fs, process::Command};

fn render(source: &str) -> (std::process::ExitStatus, tempfile::TempDir) {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.mmd");
    let output = directory.path().join("output.png");
    fs::write(&input, source).unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_FastMarkdownViewer"))
        .arg("--internal-mermaid")
        .arg(input)
        .arg(output)
        .status()
        .unwrap();
    (status, directory)
}

#[test]
fn executable_renders_diagram_pixels() {
    let (status, directory) = render("flowchart LR\n A[Open document] --> B[Render Mermaid]");
    assert!(status.success());
    let image = image::open(directory.path().join("output.png"))
        .unwrap()
        .to_rgba8();
    assert!(image.width() > 100 && image.height() > 20);
    assert!(image.pixels().any(|p| p.0[0] < 150 && p.0[3] > 0));
}

#[cfg(target_os = "windows")]
#[test]
fn diagram_labels_render_without_windows_fonts() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.mmd");
    let output = directory.path().join("output.png");
    fs::write(
        &input,
        "flowchart LR\n A[Portable font fallback] --> B[Readable]",
    )
    .unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_FastMarkdownViewer"))
        .env("WINDIR", directory.path())
        .env_remove("LOCALAPPDATA")
        .args([
            std::ffi::OsStr::new("--internal-mermaid"),
            input.as_os_str(),
            output.as_os_str(),
        ])
        .status()
        .unwrap();
    assert!(
        status.success(),
        "bundled fonts must work without Windows font files"
    );
    assert!(image::open(output).unwrap().width() > 100);
}

#[test]
fn malformed_diagram_returns_error_without_image() {
    let (status, directory) = render("flowchart LR\n A[Unclosed label");
    assert_eq!(status.code(), Some(2));
    assert!(!directory.path().join("output.png").exists());
    assert!(
        !fs::read_to_string(directory.path().join("output.error"))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn oversized_input_is_rejected() {
    let (status, directory) = render(&"x".repeat(65 * 1024));
    assert_eq!(status.code(), Some(2));
    assert!(
        fs::read_to_string(directory.path().join("output.error"))
            .unwrap()
            .contains("64 KiB")
    );
}

#[test]
fn previously_crashing_chain_is_rejected() {
    let mut source = String::from("flowchart LR\n");
    for i in 0..2000 {
        writeln!(source, "N{i} --> N{}", i + 1).unwrap();
    }
    let (status, directory) = render(&source);
    assert_eq!(status.code(), Some(2));
    assert!(
        !fs::read_to_string(directory.path().join("output.error"))
            .unwrap()
            .is_empty()
    );
}
