use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use fast_markdown_viewer::appearance::ThemeChoice;

#[test]
fn mermaid_callback_keeps_source_and_ignores_other_fences() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let calls = std::cell::RefCell::new(Vec::new());
    let callback = |ui: &mut egui::Ui, source: &str| {
        calls.borrow_mut().push(source.to_owned());
        ui.label("Rendered diagram");
    };
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        CommonMarkViewer::new()
            .render_diagram_fn(Some(&callback))
            .show(
                ui,
                &mut cache,
                "```mermaid\nflowchart LR\nA --> B\n```\n\n```rust\nfn main() {}\n```",
            );
    });
    assert!(!calls.borrow().is_empty());
    assert!(
        calls
            .borrow()
            .iter()
            .all(|source| source == "flowchart LR\nA --> B\n")
    );
    assert!(
        cache
            .navigation
            .regions
            .iter()
            .any(|region| region.galley.text().contains("A --> B"))
    );
    assert!(
        cache
            .navigation
            .regions
            .iter()
            .any(|region| region.galley.text().contains("fn main"))
    );
    output.textures_delta.clear();
}

#[test]
fn feature_matrix_renders_without_panicking() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    let source = include_str!("fixtures/feature-matrix.md");

    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let placeholder = |ui: &mut egui::Ui, formula: &str, _inline: bool| {
                ui.label(formula);
            };
            CommonMarkViewer::new()
                .render_math_fn(Some(&placeholder))
                .show(ui, &mut cache, source);
        });
    });
    output.textures_delta.clear();
}

#[test]
fn syntax_highlighting_is_lazy_and_theme_aware() {
    let context = egui::Context::default();
    let mut cache = CommonMarkCache::default();
    context.set_theme(egui::ThemePreference::Light);
    let source = "```rust\nfn main() { let answer = 42; println!(\"hello\"); }\n```";
    let render = |cache: &mut CommonMarkCache, visible: bool| {
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            if !visible {
                ui.set_clip_rect(egui::Rect::NOTHING);
            }
            cache.navigation.clear();
            CommonMarkViewer::new().show(ui, cache, source);
        });
        output.textures_delta.clear();
        cache
            .navigation
            .regions
            .iter()
            .find(|region| region.galley.text().starts_with("fn main"))
            .unwrap()
            .galley
            .job
            .sections
            .iter()
            .map(|section| section.format.color)
            .collect::<Vec<_>>()
    };
    assert_eq!(render(&mut cache, false).len(), 1);
    assert!(format!("{cache:?}").contains("worker_started: false"));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let light_colors = loop {
        let colors = render(&mut cache, true);
        if colors.iter().any(|color| *color != colors[0]) {
            break colors;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "highlight worker did not finish"
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    };
    context.set_theme(egui::ThemePreference::Dark);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let colors = render(&mut cache, true);
        if colors.len() > 1 && colors != light_colors {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "theme change did not rehighlight"
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    // Same-brightness palette changes must invalidate the syntax cache too.
    let mut previous = Vec::new();
    for theme in [
        ThemeChoice::SolarizedLight,
        ThemeChoice::QuietLight,
        ThemeChoice::SolarizedDark,
        ThemeChoice::Monokai,
        ThemeChoice::TomorrowNightBlue,
    ] {
        theme.apply(&context);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            let colors = render(&mut cache, true);
            if colors.len() > 1 && colors != previous {
                previous = colors;
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "{} did not rehighlight",
                theme.label()
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
