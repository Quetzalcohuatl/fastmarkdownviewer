//! Exercise the registered loader pipeline, including generated math and local
//! files, so dependency feature changes cannot silently remove image support.
use eframe::egui::{self, load::ImagePoll};
use fast_markdown_viewer::{math, network};
use std::{
    io::Cursor,
    sync::Arc,
    time::{Duration, Instant},
};

fn load(context: &egui::Context, uri: &str) -> Arc<egui::ColorImage> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match context
            .try_load_image(uri, egui::load::SizeHint::Width(80))
            .unwrap()
        {
            ImagePoll::Ready { image } => return image,
            ImagePoll::Pending { .. } => {
                assert!(Instant::now() < deadline, "image loading timed out: {uri}");
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

#[test]
fn registered_loaders_render_included_and_local_svg() {
    let context = egui::Context::default();
    network::install(&context);
    network::install(&context);
    let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><rect width="40" height="20" fill="red"/></svg>"#;
    context.include_bytes("bytes://rectangle.svg", svg.as_slice());
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("local image.svg");
    std::fs::write(&path, svg).unwrap();
    let local = url::Url::from_file_path(path).unwrap();
    for uri in ["bytes://rectangle.svg", local.as_str()] {
        let image = load(&context, uri);
        assert_eq!(image.size, [80, 40]);
        assert_eq!(image.source_size, egui::vec2(40.0, 20.0));
        assert!(
            image
                .pixels
                .iter()
                .all(|pixel| *pixel == egui::Color32::RED)
        );
        context.forget_image(uri);
    }
}

#[test]
fn registered_loaders_rasterize_generated_math() {
    let context = egui::Context::default();
    network::install(&context);
    let svg = math::render_formula_svg(r"\frac{1}{2} + x^2", false, 1.0, [0, 0, 0, 255]).unwrap();
    context.include_bytes("bytes://equation.svg", svg);
    let image = load(&context, "bytes://equation.svg");
    assert_eq!(image.size[0], 80);
    assert!(image.size[1] > 0);
    assert!(image.pixels.iter().any(|pixel| pixel.a() > 0));
}

#[test]
fn registered_loaders_decode_supported_raster_formats() {
    let context = egui::Context::default();
    network::install(&context);
    let source =
        image::DynamicImage::ImageRgb8(image::RgbImage::from_pixel(8, 4, image::Rgb([255, 0, 0])));
    let dir = tempfile::tempdir().unwrap();
    for (extension, format) in [
        ("png", image::ImageFormat::Png),
        ("jpg", image::ImageFormat::Jpeg),
        ("gif", image::ImageFormat::Gif),
        ("webp", image::ImageFormat::WebP),
    ] {
        let mut bytes = Cursor::new(Vec::new());
        source.write_to(&mut bytes, format).unwrap();
        let uri = format!("bytes://sample.{extension}");
        let path = dir.path().join(format!("local image.{extension}"));
        std::fs::write(&path, bytes.get_ref()).unwrap();
        context.include_bytes(uri.clone(), bytes.into_inner());
        let local = url::Url::from_file_path(path).unwrap();
        for uri in [uri.as_str(), local.as_str()] {
            let decoded = load(&context, uri);
            assert_eq!(decoded.size, [8, 4], "{extension}");
            assert!(
                decoded
                    .pixels
                    .iter()
                    .all(|pixel| pixel.r() > 240 && pixel.g() < 15 && pixel.b() < 15),
                "{extension}"
            );
        }
    }
}
