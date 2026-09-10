use std::path::Path;

pub fn render(input: &Path, output: &Path) -> Result<(), String> {
    let source = std::fs::read_to_string(input).map_err(|e| e.to_string())?;
    if source.len() > 64 * 1024 {
        return Err("Diagram exceeds 64 KiB input limit".into());
    }
    let svg = rusty_mermaid::to_svg(&source, &rusty_mermaid::Theme::default())
        .map_err(|e| e.to_string())?;
    if svg.len() > 10 * 1024 * 1024 {
        return Err("SVG exceeds 10 MiB output limit".into());
    }
    let mut options = resvg::usvg::Options::default();
    // Bundled egui fonts keep labels readable even on systems with no installed fonts.
    for font in eframe::egui::FontDefinitions::default().font_data.values() {
        options.fontdb_mut().load_font_data(font.font.to_vec());
    }
    for name in [
        "segoeui.ttf",
        "segoeuib.ttf",
        "segoeuii.ttf",
        "consola.ttf",
        "DejaVuSans.ttf",
        "DejaVuSans-Bold.ttf",
        "DejaVuSansMono.ttf",
        "Arial.ttf",
        "Arial Bold.ttf",
        "Menlo.ttc",
    ] {
        if let Some(path) = crate::font_paths::find(name)
            && let Ok(bytes) = std::fs::read(path)
        {
            options.fontdb_mut().load_font_data(bytes);
        }
    }
    let supplemental: &[&str] = if svg.chars().any(|c| {
        matches!(c as u32,
        0x1100..=0x11ff | 0x3040..=0x31ff | 0x3400..=0x9fff | 0xac00..=0xd7af)
    }) {
        &[
            "msyh.ttc",
            "msgothic.ttc",
            "malgun.ttf",
            "NotoSansCJK-Regular.ttc",
            "PingFang.ttc",
            "AppleSDGothicNeo.ttc",
        ]
    } else {
        &[]
    };
    for name in supplemental {
        if let Some(path) = crate::font_paths::find(name)
            && let Ok(bytes) = std::fs::read(path)
        {
            options.fontdb_mut().load_font_data(bytes);
        }
    }
    let sans = ["Segoe UI", "DejaVu Sans", "Arial", "Ubuntu"]
        .into_iter()
        .find(|name| {
            options
                .fontdb
                .faces()
                .any(|face| face.families.iter().any(|(family, _)| family == name))
        })
        .ok_or("No usable diagram font is available")?;
    options.font_family = sans.into();
    options.fontdb_mut().set_sans_serif_family(sans);
    options.fontdb_mut().set_serif_family(sans);
    let mono = ["Consolas", "DejaVu Sans Mono", "Menlo", "Hack"]
        .into_iter()
        .find(|name| {
            options
                .fontdb
                .faces()
                .any(|face| face.families.iter().any(|(family, _)| family == name))
        })
        .unwrap_or(sans);
    options.fontdb_mut().set_monospace_family(mono);
    let tree = resvg::usvg::Tree::from_str(&svg, &options).map_err(|e| e.to_string())?;
    let size = tree.size().to_int_size();
    if u64::from(size.width()) * u64::from(size.height()) > 4_000_000 {
        return Err("Diagram exceeds 4 megapixel limit".into());
    }
    let mut pixels = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or("Cannot allocate pixels")?;
    pixels.fill(resvg::tiny_skia::Color::WHITE);
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixels.as_mut(),
    );
    pixels.save_png(output).map_err(|e| e.to_string())?;
    Ok(())
}
