//! Developer-only asset generation. Run `cargo run --locked --example generate_icons`.
//! Shipping builds embed the generated files; no SVG rendering happens at startup.
use std::{error::Error, fs, path::Path};

fn render(root: &Path, source: &str, size: u32, destination: &Path) -> Result<(), Box<dyn Error>> {
    let svg = fs::read(root.join(format!("{source}.svg")))?;
    let tree = resvg::usvg::Tree::from_data(&svg, &resvg::usvg::Options::default())?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size).ok_or("Invalid icon size")?;
    #[allow(clippy::cast_precision_loss)] // Icon dimensions never exceed 1024.
    let scale = size as f32 / tree.size().width();
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    fs::create_dir_all(destination.parent().ok_or("Missing output directory")?)?;
    // tiny-skia stores premultiplied RGBA; PNG needs straight alpha.
    let rgba: Vec<u8> = pixmap
        .pixels()
        .iter()
        .flat_map(|pixel| {
            let pixel = pixel.demultiply();
            [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()]
        })
        .collect();
    image::save_buffer(destination, &rgba, size, size, image::ColorType::Rgba8)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/icons");
    let windows_sizes = [
        16_u32, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 128, 256,
    ];
    let mut frames = Vec::new();
    for size in windows_sizes {
        let path = root.join(format!("windows/{size}.png"));
        let source = if size <= 40 {
            "document-small"
        } else {
            "windows"
        };
        render(&root, source, size, &path)?;
        frames.push(fs::read(path)?);
    }
    // PNG-backed ICO entries preserve alpha and exact sizes on supported Windows.
    let mut ico = vec![0, 0, 1, 0];
    ico.extend_from_slice(&u16::try_from(frames.len())?.to_le_bytes());
    let mut offset = 6 + frames.len() * 16;
    for (size, frame) in windows_sizes.iter().zip(&frames) {
        let dimension = u8::try_from(*size).unwrap_or(0); // ICO uses zero for 256.
        ico.extend_from_slice(&[dimension, dimension, 0, 0]);
        ico.extend_from_slice(&1_u16.to_le_bytes());
        ico.extend_from_slice(&32_u16.to_le_bytes());
        ico.extend_from_slice(&u32::try_from(frame.len())?.to_le_bytes());
        ico.extend_from_slice(&u32::try_from(offset)?.to_le_bytes());
        offset += frame.len();
    }
    for frame in frames {
        ico.extend_from_slice(&frame);
    }
    fs::write(root.join("windows/app.ico"), ico)?;

    for nominal in [16, 32, 128, 256, 512] {
        for density in [1, 2] {
            let suffix = if density == 2 { "@2x" } else { "" };
            let source = if nominal <= 32 {
                "macos-small"
            } else {
                "macos"
            };
            render(
                &root,
                source,
                nominal * density,
                &root.join(format!(
                    "macos.iconset/icon_{nominal}x{nominal}{suffix}.png"
                )),
            )?;
        }
    }
    for size in [16, 24, 32, 48, 64, 128, 256, 512] {
        let source = if size <= 32 {
            "document-small"
        } else {
            "linux"
        };
        render(&root, source, size, &root.join(format!("linux/{size}.png")))?;
    }
    println!("Generated Windows ICO, macOS iconset, and Linux PNG icons.");
    Ok(())
}
