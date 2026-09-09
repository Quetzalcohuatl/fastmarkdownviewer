use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

fn render(
    context: &egui::Context,
    cache: &mut CommonMarkCache,
    source: &str,
    width: f32,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    cache.navigation.clear();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 2000.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            CommonMarkViewer::new().show(ui, cache, source);
        },
    );
    output.textures_delta.clear();
    output
}
fn position(cache: &CommonMarkCache, text: &str) -> egui::Pos2 {
    cache
        .navigation
        .regions
        .iter()
        .find(|region| region.galley.text() == text)
        .unwrap_or_else(|| panic!("Missing {text}: {}", cache.navigation.text))
        .position
}

#[test]
fn nested_quotes_and_definitions_preserve_structure_and_following_content() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let source = "> Outer\n>\n> > Inner\n> >\n> > > Deep\n> >\n> > Inner again\n>\n> Outer again\n\nOutside\n\nTerm\n: First definition\n: Second definition\n\nAfter definitions";
    render(&context, &mut cache, source, 480.0, vec![]);
    assert!(position(&cache, "Deep").x > position(&cache, "Inner").x);
    assert!(position(&cache, "Inner").x > position(&cache, "Outer").x);
    assert!(position(&cache, "Outside").x < position(&cache, "Outer").x);
    assert!(position(&cache, "Outer again").y > position(&cache, "Inner again").y);
    assert!(position(&cache, "First definition").x > position(&cache, "Term").x);
    assert!(position(&cache, "Second definition").y > position(&cache, "First definition").y);
    assert!(position(&cache, "After definitions").y > position(&cache, "Second definition").y);
}

#[test]
fn nested_inline_styles_restore_outer_formatting() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    render(
        &context,
        &mut cache,
        "*outer **bold *inner* tail** after `code` [link](https://example.com)* plain\n\n~~strike **bold** tail~~ done",
        600.0,
        vec![],
    );
    for text in ["inner", " tail", " after ", "code", "link"] {
        let region = cache
            .navigation
            .regions
            .iter()
            .find(|region| region.galley.text() == text)
            .unwrap_or_else(|| panic!("Missing {text}: {}", cache.navigation.text));
        assert!(
            region
                .galley
                .job
                .sections
                .iter()
                .all(|section| section.format.italics),
            "lost outer italic: {text}"
        );
    }
    let plain = cache
        .navigation
        .regions
        .iter()
        .find(|region| region.galley.text() == " plain")
        .unwrap();
    assert!(!plain.galley.job.sections[0].format.italics);
    let tail = cache
        .navigation
        .regions
        .iter()
        .rev()
        .find(|region| region.galley.text() == " tail")
        .unwrap();
    assert!(tail.galley.job.sections[0].format.strikethrough.width > 0.0);
}

#[test]
fn spaces_between_inline_spans_have_width() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    // Use valid emphasis with a space emitted between two styled widgets.
    render(&context, &mut cache, "**bold** **next**", 600.0, vec![]);
    let regions = &cache.navigation.regions;
    let space = regions
        .iter()
        .find(|region| region.galley.text() == " ")
        .unwrap();
    assert!(space.galley.size().x > 2.0);
    let bold = regions
        .iter()
        .find(|region| region.galley.text() == "bold")
        .unwrap();
    let next = regions
        .iter()
        .find(|region| region.galley.text() == "next")
        .unwrap();
    let end = bold.position.x + bold.galley.rows[0].rect().right();
    let start = next.position.x + next.galley.rows[0].rect_without_leading_space().left();
    assert!(start - end > 2.0, "words run together: {start} - {end}");
    render(&context, &mut cache, "**bold** next", 600.0, vec![]);
    let bold = &cache.navigation.regions[0];
    let next = &cache.navigation.regions[1];
    let end = bold.position.x + bold.galley.rows[0].rect().right();
    let first_letter = &next.galley.rows[0].glyphs[1];
    let start = next.position.x + next.galley.rows[0].pos.x + first_letter.pos.x;
    assert!(start - end > 2.0, "leading space lost: {start} - {end}");
}

#[test]
fn missing_images_show_alt_text_and_retry() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let mut texts = Vec::new();
    // The file loader reports errors asynchronously; wait for its result rather
    // than assuming it arrives on a particular frame.
    for _ in 0..100 {
        let output = render(
            &context,
            &mut cache,
            "![Missing diagram](file:///nonexistent-diagram.png)",
            600.0,
            vec![],
        );
        texts = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect();
        if texts.iter().any(|text| text == "Retry") {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert!(
        texts
            .iter()
            .any(|text| text == "Image unavailable: Missing diagram"),
        "{texts:?}"
    );
    assert!(texts.iter().any(|text| text == "Retry"));
}

#[test]
fn wide_tables_scroll_without_widening_prose() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let prose = "Ordinary prose wraps within the viewport even after an unusually wide table with long tokens. ".repeat(5);
    let source = format!(
        "| ID | Required | Description | Long field | Notes |\n| -: | --- | --- | --- | --- |\n| 1 | Yes | {} | {} | notes |\n\n{prose}",
        "description ".repeat(20),
        "LongToken".repeat(20)
    );
    for _ in 0..8 {
        render(&context, &mut cache, &source, 260.0, vec![]);
    }
    let before = position(&cache, "Notes");
    let pointer = position(&cache, "ID") + egui::vec2(5.0, 5.0);
    for _ in 0..20 {
        render(
            &context,
            &mut cache,
            &source,
            260.0,
            vec![
                egui::Event::PointerMoved(pointer),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(-50.0, 0.0),
                    modifiers: egui::Modifiers::NONE,
                    phase: egui::TouchPhase::Move,
                },
            ],
        );
    }
    assert!(
        position(&cache, "Notes").x < before.x - 20.0,
        "table did not scroll"
    );
    let region = cache
        .navigation
        .regions
        .iter()
        .find(|region| region.galley.text().starts_with("Ordinary prose"))
        .unwrap();
    assert!(region.galley.rows.len() > 5);
    assert!(region.position.x >= 0.0);
    assert!(
        region.position.x + region.galley.rect.right() <= 261.0,
        "prose overflow: {:?}",
        region.galley.rect
    );
}

#[test]
fn thousand_character_words_wrap_in_prose_and_table_cells() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let token = "a".repeat(1000);
    let source = format!("{token}\n\n| Long token |\n| --- |\n| {token} |\n");
    for _ in 0..8 {
        render(&context, &mut cache, &source, 260.0, vec![]);
    }
    let regions: Vec<_> = cache
        .navigation
        .regions
        .iter()
        .filter(|region| region.galley.text() == token)
        .collect();
    assert_eq!(regions.len(), 2);
    for region in regions {
        assert!(region.galley.rows.len() > 10);
        assert!(region.galley.rows.iter().all(|row| row.size.x <= 260.0));
        assert_eq!(region.galley.text().len(), 1000);
    }
}

#[test]
fn wrapped_table_rows_share_columns_and_do_not_overlap() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let token = "a".repeat(200);
    let source = format!(
        "| ID | Description | Notes |\n| --- | --- | --- |\n| 1 | {token} | note one |\n| 2 | short | note two |"
    );
    for _ in 0..8 {
        render(&context, &mut cache, &source, 480.0, vec![]);
    }
    assert!((position(&cache, "1").y - position(&cache, "note one").y).abs() < 1.0);
    assert!((position(&cache, "1").y - position(&cache, &token).y).abs() < 1.0);
    assert!((position(&cache, "1").x - position(&cache, "2").x).abs() < 1.0);
    assert!((position(&cache, &token).x - position(&cache, "short").x).abs() < 1.0);
    let long = cache
        .navigation
        .regions
        .iter()
        .find(|r| r.galley.text() == token)
        .unwrap();
    assert!(position(&cache, "2").y >= long.position.y + long.galley.rect.bottom());
}

#[test]
fn wrapping_can_be_toggled_for_prose_tables_and_code() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let token = "a".repeat(200);
    let source = format!("{token}\n\n| Field |\n| --- |\n| {token} |\n\n```\n{token}\n```");
    for wrap in [true, false, true] {
        for _ in 0..8 {
            cache.navigation.clear();
            let mut output = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(300.0, 2000.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    CommonMarkViewer::new()
                        .wrap(wrap)
                        .show(ui, &mut cache, &source);
                },
            );
            output.textures_delta.clear();
        }
        let regions: Vec<_> = cache
            .navigation
            .regions
            .iter()
            .filter(|r| r.galley.text() == token)
            .collect();
        assert_eq!(regions.len(), 3);
        assert!(
            position(&cache, "Field").y > regions[0].position.y + regions[0].galley.rect.bottom(),
            "disabling wrap must preserve block boundaries"
        );
        for region in regions {
            assert_eq!(region.galley.rows.len() > 1, wrap);
        }
    }
}

#[test]
fn code_background_contains_every_line() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let code = "first\nsecond\nthird\nfourth";
    let source = format!("```\n{code}\n```");
    for _ in 0..4 {
        render(&context, &mut cache, &source, 400.0, vec![]);
    }
    let output = render(&context, &mut cache, &source, 400.0, vec![]);
    let region = cache
        .navigation
        .regions
        .iter()
        .find(|r| r.galley.text() == code)
        .unwrap();
    let bounds = region.galley.rect.translate(region.position.to_vec2());
    let fill = context.style_of(context.theme()).visuals.extreme_bg_color;
    assert!(
        output.shapes.iter().any(|shape| matches!(&shape.shape,
        egui::Shape::Rect(rect) if rect.fill == fill && rect.rect.contains_rect(bounds))),
        "code background does not contain all lines"
    );
}
