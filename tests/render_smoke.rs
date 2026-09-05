use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

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
