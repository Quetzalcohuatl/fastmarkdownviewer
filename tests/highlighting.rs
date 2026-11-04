use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::time::{Duration, Instant};

fn render(
    context: &egui::Context,
    cache: &mut CommonMarkCache,
    source: &str,
    prefix: &str,
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
        .find(|region| region.galley.text().starts_with(prefix))
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
    let initial = render(&context, &mut cache, &source, "let value");
    assert_eq!(
        initial.sections.len(),
        1,
        "first frame must not wait for syntax work"
    );
    let first_frame = start.elapsed();
    let deadline = Instant::now() + Duration::from_secs(90);
    let highlighted = loop {
        std::thread::sleep(Duration::from_millis(100));
        let job = render(&context, &mut cache, &source, "let value");
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
        render(&context, &mut cache, &source, "let value").sections,
        highlighted.sections
    );
    eprintln!(
        "large code: first plain frame {first_frame:?}, complete {:?}",
        start.elapsed()
    );
}

#[test]
fn python_console_dates_keep_their_numeric_color() {
    let context = egui::Context::default();
    fast_markdown_viewer::appearance::ThemeChoice::Monokai.apply(&context);
    let mut cache = CommonMarkCache::default();
    let console = concat!(
        ">>> prices[\"EXAMPLE\"].head(3)\n",
        "2021-01-04     10.25\n",
        "2021-01-08      8.50\n",
        "2021-01-11      8.75\n",
        "\n",
        ">>> prices[\"EXAMPLE\"].tail(2)\n",
        "2025-12-24     15.25\n",
        "8  2025-12-26   15.50\n",
        ">>> 2025-12-24"
    );
    let source = format!("```python\n{console}\n```\n");
    let deadline = Instant::now() + Duration::from_secs(10);
    let job = loop {
        let job = render(&context, &mut cache, &source, ">>> prices");
        if job.sections.len() > 1 {
            break job;
        }
        assert!(
            Instant::now() < deadline,
            "Python highlighting did not finish"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(job.text, console);
    let color_at = |byte| {
        job.sections
            .iter()
            .find(|section| section.byte_range.contains(&egui::text::ByteIndex(byte)))
            .unwrap()
            .format
            .color
    };
    let numeric = egui::Color32::from_rgb(0xae, 0x81, 0xff);
    for date in [
        "2021-01-04",
        "2021-01-08",
        "2021-01-11",
        "2025-12-24",
        "2025-12-26",
    ] {
        let start = job.text.find(date).unwrap();
        for byte in start..start + date.len() {
            assert_eq!(color_at(byte), numeric, "inconsistent color in {date}");
        }
    }
    assert_eq!(color_at(job.text.find("8.50").unwrap()), numeric);
    let subtraction = job.text.rfind("2025-12-24").unwrap();
    assert_eq!(color_at(subtraction), numeric);
    assert_ne!(
        color_at(subtraction + 4),
        numeric,
        "Python subtraction stays syntax highlighted"
    );
}
