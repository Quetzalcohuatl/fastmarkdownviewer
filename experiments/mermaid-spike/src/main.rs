use std::{path::Path, time::Instant};

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        return Err("Usage: mermaid-spike INPUT OUTPUT.svg".into());
    }
    let total = Instant::now();
    let source = std::fs::read_to_string(&args[1]).map_err(|e| e.to_string())?;
    if source.len() > 64 * 1024 {
        return Err("Diagram exceeds prototype's 64 KiB input limit".into());
    }
    let start = Instant::now();
    #[cfg(feature = "mmdr")]
    let svg =
        mermaid_rs_renderer::render_strict(&source, mermaid_rs_renderer::RenderOptions::default())
            .map_err(|e| e.to_string())?;
    #[cfg(feature = "merman")]
    let svg = merman::render::HeadlessRenderer::default()
        .render_svg_resvg_safe_sync(&source)
        .map_err(|e| e.to_string())?
        .ok_or("No diagram produced")?;
    #[cfg(feature = "rusty")]
    let svg = rusty_mermaid::to_svg(&source, &rusty_mermaid::Theme::default())
        .map_err(|e| e.to_string())?;
    #[cfg(feature = "selkie")]
    let svg = selkie::parse(&source)
        .and_then(|diagram| selkie::render(&diagram))
        .map_err(|e| e.to_string())?;
    #[cfg(not(any(
        feature = "mmdr",
        feature = "merman",
        feature = "rusty",
        feature = "selkie"
    )))]
    let svg = source;
    let render_ms = start.elapsed().as_secs_f64() * 1000.0;
    if svg.len() > 10 * 1024 * 1024 {
        return Err("SVG exceeds prototype's 10 MiB output limit".into());
    }
    let start = Instant::now();
    #[allow(unused_mut)]
    let mut options = resvg::usvg::Options::default();
    #[cfg(feature = "svg-text")]
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
        // These ranges are a prototype heuristic, not universal glyph coverage.
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
    let font_ms = start.elapsed().as_secs_f64() * 1000.0;
    let start = Instant::now();
    let tree = resvg::usvg::Tree::from_str(&svg, &options).map_err(|e| e.to_string())?;
    let size = tree.size().to_int_size();
    if u64::from(size.width()) * u64::from(size.height()) > 16_000_000 {
        return Err("Diagram exceeds prototype's 16 megapixel limit".into());
    }
    let mut pixels = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or("Cannot allocate pixels")?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixels.as_mut(),
    );
    let raster_ms = start.elapsed().as_secs_f64() * 1000.0;
    let pipeline_ms = total.elapsed().as_secs_f64() * 1000.0;
    std::fs::write(&args[2], &svg).map_err(|e| e.to_string())?;
    pixels
        .save_png(Path::new(&args[2]).with_extension("png"))
        .map_err(|e| e.to_string())?;
    println!(
        "render_ms={render_ms:.3} font_ms={font_ms:.3} raster_ms={raster_ms:.3} pipeline_ms={pipeline_ms:.3} svg_bytes={} width={} height={}",
        svg.len(),
        size.width(),
        size.height()
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: {error}");
        std::process::exit(2);
    }
}
