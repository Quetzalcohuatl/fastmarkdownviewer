//! Exercise the actual executable's isolated Mermaid renderer.
use std::{fmt::Write, fs, process::Command};

fn render(source: &str) -> (std::process::ExitStatus, tempfile::TempDir) {
    render_with_appearance(source, None)
}

fn render_with_appearance(
    source: &str,
    appearance: Option<serde_json::Value>,
) -> (std::process::ExitStatus, tempfile::TempDir) {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("input.mmd");
    let output = directory.path().join("output.png");
    fs::write(&input, source).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_FastMarkdownViewer"));
    command.arg("--internal-mermaid").arg(input).arg(output);
    if let Some(appearance) = appearance {
        command.arg(appearance.to_string());
    }
    let status = command.status().unwrap();
    (status, directory)
}

#[test]
fn diagram_background_and_text_follow_every_reader_palette() {
    use fast_markdown_viewer::appearance::ThemeChoice;
    let context = eframe::egui::Context::default();
    for theme in ThemeChoice::ALL {
        theme.apply(&context);
        let visuals = context.style_of(context.theme()).visuals.clone();
        let rgb = |color: eframe::egui::Color32| [color.r(), color.g(), color.b()];
        let background = rgb(visuals.panel_fill);
        let text = rgb(visuals.text_color());
        let (status, dir) = render_with_appearance(
            "flowchart LR\n A[Readable labels] --> B[Theme]",
            Some(serde_json::json!({
                "background": background, "surface": rgb(visuals.window_fill),
                "text": text, "muted": rgb(visuals.weak_text_color()),
                "accent": rgb(visuals.hyperlink_color), "selection": rgb(visuals.selection.bg_fill),
                "font": null,
            })),
        );
        assert!(status.success(), "{}", theme.label());
        let image = image::open(dir.path().join("output.png"))
            .unwrap()
            .to_rgba8();
        assert_eq!(
            image.get_pixel(0, 0).0,
            [background[0], background[1], background[2], 255],
            "{}",
            theme.label()
        );
        assert!(
            image.pixels().any(|pixel| pixel.0[..3] == text),
            "missing foreground for {}",
            theme.label()
        );
    }
}

#[test]
fn selected_text_font_changes_labels_and_missing_font_falls_back() {
    #[cfg(target_os = "windows")]
    let font = "georgia.ttf";
    #[cfg(target_os = "macos")]
    let font = "Helvetica.ttc";
    #[cfg(target_os = "linux")]
    let font = "DejaVuSans.ttf";
    let mut images = Vec::new();
    for font in [None, Some(font), Some("fmv-missing-font.ttf")] {
        let (status, dir) = render_with_appearance(
            "flowchart LR\n A[Different letter shapes WWW iii] --> B[Readable]",
            Some(serde_json::json!({
                "background": [39,40,34], "surface": [52,53,47], "text": [248,248,242],
                "muted": [176,176,155], "accent": [166,226,46], "selection": [73,72,62], "font": font,
            })),
        );
        assert!(status.success());
        images.push(
            image::open(dir.path().join("output.png"))
                .unwrap()
                .to_rgba8(),
        );
    }
    assert_ne!(
        images[0], images[1],
        "selected font must change rendered glyphs"
    );
    assert_eq!(
        images[0], images[2],
        "missing font must fall back consistently"
    );
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
fn source_beyond_the_former_64_kib_cutoff_renders() {
    let source = format!("flowchart LR\n%% {}\nA --> B", "comment ".repeat(10_000));
    let (status, directory) = render(&source);
    assert!(status.success());
    assert!(directory.path().join("output.png").is_file());
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

#[test]
fn large_diagram_tiles_cover_the_entire_image_without_a_megapixel_cutoff() {
    let mut source = String::from("sequenceDiagram\nparticipant A\nparticipant B\n");
    for index in 0..70 {
        writeln!(
            source,
            "A->>B: Message {index} {}",
            "wide label ".repeat(22)
        )
        .unwrap();
    }
    let (status, directory) = render(&source);
    assert!(status.success());
    let full = image::open(directory.path().join("output.png"))
        .unwrap()
        .to_rgba8();
    assert!(u64::from(full.width()) * u64::from(full.height()) > 4_000_000);
    let manifest = directory.path().join("tiles.json");
    let appearance = serde_json::json!({
        "background": [248,248,248], "surface": [248,248,248], "text": [60,60,60],
        "muted": [100,100,100], "accent": [0,90,170], "selection": [180,210,240], "font": null,
    });
    let status = Command::new(env!("CARGO_BIN_EXE_FastMarkdownViewer"))
        .arg("--internal-mermaid")
        .arg(directory.path().join("input.mmd"))
        .arg(&manifest)
        .arg(appearance.to_string())
        .arg(r#"{"max_width":50000,"pixel_scale":1}"#)
        .status()
        .unwrap();
    assert!(status.success());
    let geometry: serde_json::Value = serde_json::from_slice(&fs::read(manifest).unwrap()).unwrap();
    let width = u32::try_from(geometry["width"].as_u64().unwrap()).unwrap();
    let height = u32::try_from(geometry["height"].as_u64().unwrap()).unwrap();
    // Tiles round up fractional SVG extents; the legacy PNG rounds to nearest.
    assert!(width.abs_diff(full.width()) <= 1);
    assert!(height.abs_diff(full.height()) <= 3);
    let mut pixels = 0_u64;
    for row in 0..height.div_ceil(512) {
        for column in 0..width.div_ceil(512) {
            let tile = image::open(directory.path().join(format!("tile-{column}-{row}.png")))
                .unwrap()
                .to_rgba8();
            assert!(tile.width() <= 512 && tile.height() <= 512);
            assert_eq!(tile.width(), (width - column * 512).min(512));
            assert_eq!(tile.height(), (height - row * 512).min(512));
            pixels += u64::from(tile.width()) * u64::from(tile.height());
        }
    }
    assert_eq!(pixels, u64::from(width) * u64::from(height));
}
