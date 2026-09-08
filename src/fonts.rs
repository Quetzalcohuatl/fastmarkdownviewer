use eframe::egui;
use std::{collections::HashSet, path::PathBuf, sync::Arc};

#[derive(Clone, Default)]
struct FontState {
    definitions: egui::FontDefinitions,
    attempted: HashSet<&'static str>,
}

fn state_id() -> egui::Id {
    egui::Id::new("document_font_fallbacks")
}

/// Install emoji and small system fallbacks. Large East Asian fonts load only when needed.
pub fn install(context: &egui::Context) {
    let mut state = FontState {
        definitions: egui::FontDefinitions::default(),
        attempted: HashSet::new(),
    };
    add(
        &mut state.definitions,
        "Noto Emoji",
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoEmoji-Variable.ttf")),
    );
    for name in ["segoeui.ttf", "arial.ttf"] {
        load_system(&mut state, name);
    }
    context.set_fonts(state.definitions.clone());
    context.data_mut(|data| data.insert_temp(state_id(), state));
}

fn add(definitions: &mut egui::FontDefinitions, name: &str, data: egui::FontData) {
    definitions
        .font_data
        .insert(name.to_owned(), Arc::new(data));
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        definitions
            .families
            .entry(family)
            .or_default()
            .push(name.to_owned());
    }
}

fn load_system(state: &mut FontState, name: &'static str) -> bool {
    if !state.attempted.insert(name) {
        return false;
    }
    let directory = std::env::var_os("WINDIR")
        .map_or_else(|| PathBuf::from("C:/Windows"), PathBuf::from)
        .join("Fonts");
    let Ok(bytes) = std::fs::read(directory.join(name)) else {
        return false;
    };
    add(
        &mut state.definitions,
        name,
        egui::FontData::from_owned(bytes),
    );
    true
}

/// Extend both proportional and code fonts for scripts present in the document.
pub fn ensure_for_text(context: &egui::Context, text: &str) {
    let Some(mut state) = context.data_mut(|data| data.remove_temp::<FontState>(state_id())) else {
        return;
    };
    let mut changed = false;
    let mut japanese = false;
    let mut chinese = false;
    let mut korean = false;
    for character in text.chars() {
        match character {
            '\u{3040}'..='\u{30ff}' | '\u{31f0}'..='\u{31ff}' => japanese = true,
            '\u{3400}'..='\u{9fff}' | '\u{f900}'..='\u{faff}' | '\u{20000}'..='\u{323af}' => {
                chinese = true;
            }
            '\u{1100}'..='\u{11ff}' | '\u{3130}'..='\u{318f}' | '\u{ac00}'..='\u{d7af}' => {
                korean = true;
            }
            _ => {}
        }
    }
    if japanese {
        changed |= load_system(&mut state, "msgothic.ttc");
    }
    if chinese {
        changed |= load_system(&mut state, "msyh.ttc");
    }
    if korean {
        changed |= load_system(&mut state, "malgun.ttf");
    }
    if changed {
        context.set_fonts(state.definitions.clone());
    }
    context.data_mut(|data| data.insert_temp(state_id(), state));
}
