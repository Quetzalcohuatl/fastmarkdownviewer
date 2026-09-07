use std::sync::Arc;

use eframe::egui;

/// Install the bundled monochrome emoji font as the last fallback for document
/// text and code.
pub fn install(context: &egui::Context) {
    const NAME: &str = "Noto Emoji";
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        NAME.to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoEmoji-Variable.ttf"
        ))),
    );
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts
            .families
            .entry(family)
            .or_default()
            .push(NAME.to_owned());
    }
    context.set_fonts(fonts);
}
