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
    {
        // Explicit font files avoid a second system-wide font scan in usvg.
        // The Mermaid library may still initialize its own measurement font database.
        let windows = std::env::var_os("WINDIR").ok_or("WINDIR is not set")?;
        let fonts = Path::new(&windows).join("Fonts");
        for name in ["segoeui.ttf", "segoeuib.ttf", "segoeuii.ttf", "consola.ttf"] {
            let bytes = std::fs::read(fonts.join(name)).map_err(|e| e.to_string())?;
            options.fontdb_mut().load_font_data(bytes);
        }
        // Load large supplemental fonts only for scripts present in this diagram.
        // These ranges are a heuristic, not universal glyph coverage.
        let mut supplemental = Vec::new();
        if svg
            .chars()
            .any(|c| matches!(c as u32, 0x3040..=0x30ff | 0x3400..=0x9fff))
        {
            supplemental.push("msyh.ttc");
            supplemental.push("msgothic.ttc");
        }
        if svg
            .chars()
            .any(|c| matches!(c as u32, 0x1100..=0x11ff | 0x3130..=0x318f | 0xac00..=0xd7af))
        {
            supplemental.push("malgun.ttf");
        }
        for name in supplemental {
            if let Ok(bytes) = std::fs::read(fonts.join(name)) {
                options.fontdb_mut().load_font_data(bytes);
            }
        }
        options.font_family = "Segoe UI".into();
        options.fontdb_mut().set_sans_serif_family("Segoe UI");
        options.fontdb_mut().set_serif_family("Segoe UI");
        options.fontdb_mut().set_monospace_family("Consolas");
    }
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
