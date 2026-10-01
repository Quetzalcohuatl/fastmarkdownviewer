use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::time::{Duration, Instant};

fn render(
    context: &egui::Context,
    cache: &mut CommonMarkCache,
    source: &str,
) -> egui::text::LayoutJob {
    cache.navigation.clear();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        CommonMarkViewer::new().show(ui, cache, source);
    });
    output.textures_delta.clear();
    cache
        .navigation
        .regions
        .iter()
        .find(|region| region.galley.text().starts_with("let value"))
        .expect("code stays readable while highlighting loads")
        .galley
        .job
        .as_ref()
        .clone()
}

#[test]
fn large_code_appears_plain_then_highlights_without_size_or_span_cutoffs() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    // Cross all three former cutoffs: input bytes, token count, and formatted bytes.
    let text = "let value = 123; // readable immediately\n".repeat(20_000);
    assert!(text.len() > 256 * 1024);
    let source = format!("```rust\n{text}```\n");
    let start = Instant::now();
    let initial = render(&context, &mut cache, &source);
    assert_eq!(
        initial.sections.len(),
        1,
        "first frame must not wait for syntax work"
    );
    let first_frame = start.elapsed();
    let deadline = Instant::now() + Duration::from_secs(90);
    let highlighted = loop {
        std::thread::sleep(Duration::from_millis(100));
        let job = render(&context, &mut cache, &source);
        if job.sections.len() > 1 {
            break job;
        }
        assert!(
            Instant::now() < deadline,
            "large block never received highlighting"
        );
    };
    assert_eq!(highlighted.text, initial.text);
    assert!(highlighted.sections.len() > 16_384);
    assert!(
        highlighted.text.len()
            + highlighted.sections.len() * size_of::<egui::text::LayoutSection>()
            > 8 * 1024 * 1024
    );
    assert!(
        highlighted
            .sections
            .iter()
            .any(|section| section.format.color != highlighted.sections[0].format.color)
    );
    assert_eq!(
        highlighted.sections.last().unwrap().byte_range.end,
        egui::text::ByteIndex(highlighted.text.len())
    );
    // An oversized completed result must stay cached instead of reverting to plain text.
    assert_eq!(
        render(&context, &mut cache, &source).sections,
        highlighted.sections
    );
    eprintln!(
        "large code: first plain frame {first_frame:?}, complete {:?}",
        start.elapsed()
    );
}
