fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    let source = std::fs::read_to_string(&args[1]).unwrap();
    let start = std::time::Instant::now();
    #[cfg(feature = "mmdr")]
    let svg = mermaid_rs_renderer::render(&source).map_err(|error| error.to_string());
    #[cfg(feature = "merman")]
    let svg = merman::render::HeadlessRenderer::default()
        .render_svg_resvg_safe_sync(&source)
        .map_err(|error| error.to_string())
        .and_then(|svg| svg.ok_or_else(|| "No diagram produced".to_owned()));
    #[cfg(not(any(feature = "mmdr", feature = "merman")))]
    let svg: Result<String, String> = Ok(source);
    let svg = match svg {
        Ok(svg) => svg,
        Err(error) => {
            eprintln!("ERROR: {error}");
            std::process::exit(2);
        }
    };
    println!(
        "render_ms={:.3} svg_bytes={}",
        start.elapsed().as_secs_f64() * 1000.0,
        svg.len()
    );
    std::fs::write(&args[2], &svg).unwrap();
    // Exactly the shipping resvg feature set, deliberately without text/system-font support.
    let tree = resvg::usvg::Tree::from_str(&svg, &resvg::usvg::Options::default()).unwrap();
    let size = tree.size().to_int_size();
    assert!(u64::from(size.width()) * u64::from(size.height()) <= 40_000_000);
    let mut pixels = resvg::tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixels.as_mut(),
    );
    pixels
        .save_png(std::path::Path::new(&args[2]).with_extension("png"))
        .unwrap();
}
