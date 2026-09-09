use std::{
    io,
    io::Write as _,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use eframe::egui;
use fast_markdown_viewer::{
    app::{AppServices, InitialState, ViewerApp},
    fonts,
};
use url::Url;

const VIEWPORT_WIDTH: f32 = 720.0;
const VIEWPORT_HEIGHT: f32 = 240.0;

#[derive(Debug)]
struct TestDrop(PathBuf);

impl egui::DroppedFile for TestDrop {
    fn path(&self) -> &Path {
        &self.0
    }

    fn bytes(&self) -> Result<Vec<u8>, String> {
        std::fs::read(&self.0).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Default)]
struct TestServices {
    selected_file: Mutex<Option<PathBuf>>,
    browser_urls: Mutex<Vec<Url>>,
    revealed_paths: Mutex<Vec<PathBuf>>,
}

impl AppServices for TestServices {
    fn choose_markdown_file(&self) -> Option<PathBuf> {
        self.selected_file.lock().unwrap().take()
    }

    fn open_browser(&self, url: &Url) -> io::Result<()> {
        self.browser_urls.lock().unwrap().push(url.clone());
        Ok(())
    }

    fn reveal_in_file_manager(&self, path: &Path) -> io::Result<()> {
        self.revealed_paths.lock().unwrap().push(path.to_owned());
        Ok(())
    }
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("feature-matrix.md")
}

fn input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
        )),
        events,
        ..Default::default()
    }
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn run_frame(
    context: &egui::Context,
    app: &mut ViewerApp,
    raw_input: egui::RawInput,
) -> egui::FullOutput {
    let mut output = context.run_ui(raw_input, |ui| app.show(ui));
    output.textures_delta.clear();
    output
}

fn accesskit_update(output: &egui::FullOutput) -> &egui::accesskit::TreeUpdate {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("accessibility tree was not produced")
}

fn node_with_text(
    update: &egui::accesskit::TreeUpdate,
    role: egui::accesskit::Role,
    expected: &str,
) -> egui::accesskit::NodeId {
    update
        .nodes
        .iter()
        .find_map(|(id, node)| {
            if node.role() != role {
                return None;
            }
            let own_text = node.label().or_else(|| node.value()).unwrap_or_default();
            let child_text = node
                .children()
                .iter()
                .filter_map(|child_id| {
                    update
                        .nodes
                        .iter()
                        .find(|(candidate_id, _)| candidate_id == child_id)
                        .and_then(|(_, child)| child.value().or_else(|| child.label()))
                })
                .collect::<String>();
            (own_text.contains(expected) || child_text.contains(expected)).then_some(*id)
        })
        .unwrap_or_else(|| {
            let summary = update
                .nodes
                .iter()
                .map(|(id, node)| (id, node.role(), node.label(), node.value(), node.children()))
                .collect::<Vec<_>>();
            panic!("no {role:?} node contained {expected:?}; tree: {summary:#?}")
        })
}

fn accesskit_action(
    action: egui::accesskit::Action,
    target_node: egui::accesskit::NodeId,
    data: Option<egui::accesskit::ActionData>,
) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node,
        data,
    })
}

#[test]
fn empty_window_paints_the_whole_viewport_and_exposes_main_menus() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Empty);
    let output = run_frame(&context, &mut app, input(Vec::new()));
    let viewport = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    );

    assert!(output.shapes.iter().any(|clipped| {
        matches!(
            &clipped.shape,
            egui::epaint::Shape::Rect(rect)
                if rect.fill != egui::Color32::TRANSPARENT
                    && rect.rect.contains(viewport.min)
                    && rect.rect.contains(viewport.max - egui::vec2(0.1, 0.1))
        )
    }));

    let update = accesskit_update(&output);
    for expected in ["File", "Settings", "Open Markdown file"] {
        assert!(
            update.nodes.iter().any(|(_, node)| {
                node.label()
                    .or_else(|| node.value())
                    .is_some_and(|text| text.contains(expected))
            }),
            "accessibility tree did not contain {expected:?}"
        );
    }
}

#[test]
fn control_wheel_zooms_in_and_out() {
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(fixture()));
    run_frame(&context, &mut app, input(Vec::new()));
    let initial_zoom = context.zoom_factor();
    let modifiers = egui::Modifiers::CTRL | egui::Modifiers::COMMAND;

    run_frame(
        &context,
        &mut app,
        input(vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, 120.0),
            modifiers,
            phase: egui::TouchPhase::Move,
        }]),
    );
    assert!(app.document_scroll_offset().abs() < f32::EPSILON);
    run_frame(&context, &mut app, input(Vec::new()));
    let zoomed_in = context.zoom_factor();
    assert!(zoomed_in > initial_zoom);

    run_frame(
        &context,
        &mut app,
        input(vec![egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -120.0),
            modifiers,
            phase: egui::TouchPhase::Move,
        }]),
    );
    run_frame(&context, &mut app, input(Vec::new()));
    assert!(context.zoom_factor() < zoomed_in);
}

#[test]
fn find_and_heading_shortcuts_toggle_and_release_keyboard_focus() {
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(fixture()));
    run_frame(&context, &mut app, input(Vec::new()));
    for expected in [true, false, true, false] {
        run_frame(
            &context,
            &mut app,
            input(vec![key(egui::Key::F, egui::Modifiers::COMMAND)]),
        );
        assert_eq!(app.search_status().is_some(), expected);
    }
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::PageDown, egui::Modifiers::NONE)]),
    );
    assert!(app.document_scroll_offset() > 0.0);
    for expected in [false, true] {
        run_frame(
            &context,
            &mut app,
            input(vec![key(egui::Key::H, egui::Modifiers::COMMAND)]),
        );
        assert_eq!(app.outline_visible(), expected);
    }
}

#[test]
fn empty_window_explains_find_and_heading_shortcuts() {
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Empty);
    let mut raw = input(Vec::new());
    raw.screen_rect = Some(egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(900.0, 700.0),
    ));
    let output = run_frame(&context, &mut app, raw);
    let text = accesskit_update(&output)
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().or_else(|| node.value()))
        .collect::<String>();
    for expected in [
        "Ctrl+F",
        "Ctrl+H",
        "Show / hide Find",
        "Show / hide headings",
        "Drag a tab outside",
    ] {
        assert!(text.contains(expected), "missing shortcut help: {expected}");
    }
}

#[test]
fn default_fonts_cover_common_unicode_emoji() {
    let context = egui::Context::default();
    fonts::install(&context);
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Path(fixture()));
    let output = run_frame(&context, &mut app, input(Vec::new()));
    let rendered_text = accesskit_update(&output)
        .nodes
        .iter()
        .filter_map(|(_, node)| node.value().or_else(|| node.label()))
        .collect::<String>();
    assert!(rendered_text.contains("😀 🎉 ✅ ❤️"));

    let font = egui::FontId::proportional(16.0);
    for character in ['😀', '🎉', '✅', '❤'] {
        assert!(has_actual_glyph(&context, &font, character));
    }
}

#[test]
fn mouse_wheel_scrolls_the_real_document_surface_without_panicking() {
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(fixture()));
    run_frame(&context, &mut app, input(Vec::new()));

    for _ in 0..6 {
        run_frame(
            &context,
            &mut app,
            input(vec![
                egui::Event::PointerMoved(egui::pos2(300.0, 120.0)),
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -180.0),
                    modifiers: egui::Modifiers::NONE,
                    phase: egui::TouchPhase::Move,
                },
            ]),
        );
    }

    assert!(
        app.document_scroll_offset() > 0.0,
        "wheel input did not move the document viewport"
    );
}

#[test]
fn page_space_home_end_and_arrow_keys_scroll_the_document() {
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(fixture()));
    run_frame(&context, &mut app, input(Vec::new()));

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::PageDown, egui::Modifiers::NONE)]),
    );
    let page_offset = app.document_scroll_offset();
    assert!(page_offset > 0.0, "Page Down did not move the viewport");

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::ArrowDown, egui::Modifiers::NONE)]),
    );
    let arrow_offset = app.document_scroll_offset();
    assert!(arrow_offset > page_offset);

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::PageUp, egui::Modifiers::NONE)]),
    );
    let page_up_offset = app.document_scroll_offset();
    assert!(page_up_offset < arrow_offset, "Page Up did not move upward");

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Space, egui::Modifiers::NONE)]),
    );
    assert!(
        app.document_scroll_offset() > page_up_offset,
        "Space did not move downward"
    );

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::End, egui::Modifiers::NONE)]),
    );
    assert!(app.document_scroll_offset() > page_offset);

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Home, egui::Modifiers::NONE)]),
    );
    assert!(app.document_scroll_offset().abs() < f32::EPSILON);
}

#[test]
fn dropping_files_opens_tabs_and_deduplicates_paths() {
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Empty);
    let path = fixture();
    let mut drop_input = input(Vec::new());
    drop_input.dropped_files = vec![Arc::new(TestDrop(path.clone()))];
    run_frame(&context, &mut app, drop_input);

    assert_eq!(
        app.document_path(),
        Some(path.canonicalize().unwrap().as_path())
    );
    assert!(app.error_message().is_none());

    let mut multiple_input = input(Vec::new());
    multiple_input.dropped_files = vec![Arc::new(TestDrop(path.clone())), Arc::new(TestDrop(path))];
    run_frame(&context, &mut app, multiple_input);
    assert!(app.error_message().is_none());
    assert_eq!(app.tab_count(), 1);
}

#[test]
fn control_o_uses_the_picker_and_loads_the_selected_file() {
    let context = egui::Context::default();
    let services = Arc::new(TestServices::default());
    let path = fixture();
    *services.selected_file.lock().unwrap() = Some(path.clone());
    let mut app = ViewerApp::with_services(InitialState::Empty, services);

    let windows_command = egui::Modifiers::CTRL | egui::Modifiers::COMMAND;
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::O, windows_command)]),
    );

    assert_eq!(
        app.document_path(),
        Some(path.canonicalize().unwrap().as_path())
    );
    assert!(app.error_message().is_none());
}

#[test]
fn invalid_drop_is_a_visible_error_instead_of_a_crash() {
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Empty);
    let missing = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitely-missing.md");
    let mut drop_input = input(Vec::new());
    drop_input.dropped_files = vec![Arc::new(TestDrop(missing))];
    run_frame(&context, &mut app, drop_input);

    assert!(app.document_path().is_none());
    assert!(
        app.error_message()
            .is_some_and(|error| error.contains("Cannot open"))
    );

    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Escape, egui::Modifiers::NONE)]),
    );
    assert!(app.error_message().is_none());
}

#[test]
fn accessibility_clicks_dispatch_safe_links_and_leave_unsafe_links_inert() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(
        file,
        "[Browser test](https://example.com/test) · [Chapter test](chapter.md) · [Unsafe test](javascript:alert(1))"
    )
    .unwrap();

    let context = egui::Context::default();
    context.enable_accesskit();
    let services = Arc::new(TestServices::default());
    let mut app =
        ViewerApp::with_services(InitialState::Path(file.path().to_owned()), services.clone());
    let output = run_frame(&context, &mut app, input(Vec::new()));
    let update = accesskit_update(&output);
    let browser = node_with_text(update, egui::accesskit::Role::Label, "Browser test");
    let chapter = node_with_text(update, egui::accesskit::Role::Label, "Chapter test");
    let unsafe_link = node_with_text(update, egui::accesskit::Role::Label, "Unsafe test");

    for node in [browser, chapter, unsafe_link] {
        run_frame(
            &context,
            &mut app,
            input(vec![accesskit_action(
                egui::accesskit::Action::Click,
                node,
                None,
            )]),
        );
    }

    assert_eq!(
        services.browser_urls.lock().unwrap().as_slice(),
        &[Url::parse("https://example.com/test").unwrap()]
    );
    assert!(services.revealed_paths.lock().unwrap().is_empty());
    assert!(app.error_message().unwrap().contains("chapter.md"));
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn accessible_text_selection_can_be_copied() {
    const SENTENCE: &str = "Selectable regression sentence.";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(file, "{SENTENCE}").unwrap();

    let context = egui::Context::default();
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Path(file.path().to_owned()));
    let output = run_frame(&context, &mut app, input(Vec::new()));
    let update = accesskit_update(&output);
    let (run_id, run) = update
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::TextRun
                && node.value().is_some_and(|value| value.contains(SENTENCE))
        })
        .expect("selectable text run was not exposed to accessibility");
    let parent_id = update
        .nodes
        .iter()
        .find(|(_, node)| node.children().contains(run_id))
        .map(|(id, _)| *id)
        .expect("text run had no selectable parent");
    let run_length = run.character_lengths().len();
    let bounds = run.bounds().expect("text run had no screen bounds");
    let click_position = egui::pos2(
        f64::midpoint(bounds.x0, bounds.x1) as f32,
        f64::midpoint(bounds.y0, bounds.y1) as f32,
    );
    run_frame(
        &context,
        &mut app,
        input(vec![
            egui::Event::PointerMoved(click_position),
            egui::Event::PointerButton {
                pos: click_position,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: click_position,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ]),
    );
    let selection = egui::accesskit::TextSelection {
        anchor: egui::accesskit::TextPosition {
            node: *run_id,
            character_index: 0,
        },
        focus: egui::accesskit::TextPosition {
            node: *run_id,
            character_index: run_length,
        },
    };

    let selected = run_frame(
        &context,
        &mut app,
        input(vec![accesskit_action(
            egui::accesskit::Action::SetTextSelection,
            parent_id,
            Some(egui::accesskit::ActionData::SetTextSelection(selection)),
        )]),
    );
    let selected_update = accesskit_update(&selected);
    let selected_node = selected_update
        .nodes
        .iter()
        .find(|(id, _)| *id == parent_id)
        .map(|(_, node)| node)
        .expect("selected text parent was not updated");
    assert!(
        selected_node.text_selection().is_some(),
        "accessibility selection was not applied"
    );
    let copied = run_frame(&context, &mut app, input(vec![egui::Event::Copy]));
    assert!(
        copied.platform_output.commands.iter().any(|command| {
            matches!(command, egui::OutputCommand::CopyText(text) if text.contains(SENTENCE))
        }),
        "copy output was {:?}",
        copied.platform_output.commands
    );
}

#[test]
fn system_theme_and_common_windows_scale_factors_render_without_panicking() {
    for (scale, theme) in [
        (1.0, egui::Theme::Light),
        (1.5, egui::Theme::Dark),
        (2.0, egui::Theme::Light),
    ] {
        let context = egui::Context::default();
        context.set_theme(egui::ThemePreference::System);
        let mut app = ViewerApp::new(InitialState::Path(fixture()));
        let mut raw_input = input(Vec::new());
        raw_input.system_theme = Some(theme);
        raw_input
            .viewports
            .get_mut(&egui::ViewportId::ROOT)
            .unwrap()
            .native_pixels_per_point = Some(scale);
        run_frame(&context, &mut app, raw_input);
        assert_eq!(context.theme(), theme);
    }
}

#[test]
fn find_searches_rendered_text_and_code_without_stealing_typing_keys() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(
        file,
        "# Search fixture\n\nHello **world**\n\n{}\n\n```rust\n// hello world\n```\n",
        "Filler paragraph.\n\n".repeat(70)
    )
    .unwrap();
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(file.path().to_owned()));
    run_frame(&context, &mut app, input(Vec::new()));
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::F, egui::Modifiers::COMMAND)]),
    );
    run_frame(
        &context,
        &mut app,
        input(vec![egui::Event::Text("hello*world".into())]),
    );
    assert_eq!(app.search_status(), Some((1, 2)));
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]),
    );
    for _ in 0..20 {
        run_frame(&context, &mut app, input(Vec::new()));
    }
    assert_eq!(app.search_status(), Some((2, 2)));
    assert!(app.document_scroll_offset() > 500.0);
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Enter, egui::Modifiers::SHIFT)]),
    );
    assert_eq!(app.search_status(), Some((1, 2)));
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Escape, egui::Modifiers::NONE)]),
    );
    assert_eq!(app.search_status(), None);
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Home, egui::Modifiers::NONE)]),
    );
    assert!(app.document_scroll_offset() < 1.0);
}

#[test]
fn tabs_preserve_scroll_and_bad_opens_preserve_documents() {
    let directory = tempfile::tempdir().unwrap();
    let first = directory.path().join("first.md");
    let second = directory.path().join("second.md");
    std::fs::write(&first, "Paragraph.\n\n".repeat(100)).unwrap();
    std::fs::write(&second, "# Second").unwrap();
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(first.clone()));
    run_frame(&context, &mut app, input(Vec::new()));
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::PageDown, egui::Modifiers::NONE)]),
    );
    let scroll = app.document_scroll_offset();
    let mut dropped = input(Vec::new());
    dropped.dropped_files = vec![
        Arc::new(TestDrop(second.clone())),
        Arc::new(TestDrop(directory.path().join("missing.md"))),
    ];
    run_frame(&context, &mut app, dropped);
    assert_eq!(app.tab_count(), 2);
    assert_eq!(
        app.document_path(),
        Some(second.canonicalize().unwrap().as_path())
    );
    assert!(app.error_message().is_some());
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Tab, egui::Modifiers::CTRL)]),
    );
    assert_eq!(
        app.document_path(),
        Some(first.canonicalize().unwrap().as_path())
    );
    assert!((app.document_scroll_offset() - scroll).abs() < 1.0);
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::W, egui::Modifiers::COMMAND)]),
    );
    assert_eq!(app.tab_count(), 1);
    assert_eq!(
        app.document_path(),
        Some(second.canonicalize().unwrap().as_path())
    );
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::W, egui::Modifiers::COMMAND)]),
    );
    assert_eq!(app.tab_count(), 0);
    assert!(app.document_path().is_none());
}

#[test]
fn outline_navigates_duplicate_and_setext_headings_and_can_be_hidden() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(
        file,
        "# Duplicate\n\n{}\n\nDuplicate\n=========\n",
        "Paragraph.\n\n".repeat(80)
    )
    .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Path(file.path().to_owned()));
    run_frame(&context, &mut app, input(Vec::new()));
    let output = run_frame(&context, &mut app, input(Vec::new()));
    let mut headings: Vec<_> = accesskit_update(&output)
        .nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Duplicate")
        })
        .map(|(id, node)| (*id, node.bounds().unwrap().y0))
        .collect();
    headings.sort_by(|a, b| a.1.total_cmp(&b.1));
    assert_eq!(headings.len(), 2);
    run_frame(
        &context,
        &mut app,
        input(vec![accesskit_action(
            egui::accesskit::Action::Click,
            headings[1].0,
            None,
        )]),
    );
    for _ in 0..20 {
        run_frame(&context, &mut app, input(Vec::new()));
    }
    assert!(app.document_scroll_offset() > 500.0);
    run_frame(
        &context,
        &mut app,
        input(vec![key(
            egui::Key::O,
            egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        )]),
    );
    assert!(!app.outline_visible());
}

#[test]
#[cfg(target_os = "windows")]
fn windows_fallbacks_contain_actual_multilingual_glyphs_in_text_and_code() {
    const SAMPLE: &str = "English 日本語 中文 한국어 العربية עברית 🙂";
    let context = egui::Context::default();
    fonts::install(&context);
    fonts::ensure_for_text(&context, SAMPLE);
    let mut output = context.run_ui(input(Vec::new()), |ui| {
        ui.label(SAMPLE);
    });
    output.textures_delta.clear();
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        let font = egui::FontId::new(16.0, family);
        for character in SAMPLE
            .chars()
            .filter(|character| !character.is_whitespace())
        {
            assert!(
                has_actual_glyph(&context, &font, character),
                "Missing glyph: {character}"
            );
        }
    }
}

fn has_actual_glyph(context: &egui::Context, font: &egui::FontId, character: char) -> bool {
    // egui 0.36's has_glyph compares face identity with the replacement face,
    // giving false negatives for valid characters in that same fallback face.
    context.fonts_mut(|fonts| {
        let definitions = fonts.definitions();
        definitions.families[&font.family].iter().any(|name| {
            let data = &definitions.font_data[name];
            ttf_parser::Face::parse(&data.font, data.index)
                .is_ok_and(|face| face.glyph_index(character).is_some())
        })
    })
}

#[test]
fn find_reveals_matches_beyond_a_code_blocks_horizontal_viewport() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(
        file,
        "```text\n{}far_away_match\n```",
        "padding ".repeat(60)
    )
    .unwrap();
    let context = egui::Context::default();
    let mut app = ViewerApp::new(InitialState::Path(file.path().to_owned()));
    run_frame(&context, &mut app, input(Vec::new()));
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::F, egui::Modifiers::COMMAND)]),
    );
    run_frame(
        &context,
        &mut app,
        input(vec![egui::Event::Text("far_away_match".into())]),
    );
    for _ in 0..30 {
        run_frame(&context, &mut app, input(Vec::new()));
    }
    let output = run_frame(&context, &mut app, input(Vec::new()));
    let highlight = egui::Color32::from_rgba_unmultiplied(255, 140, 0, 100);
    assert!(output.shapes.iter().any(|shape| {
        matches!(&shape.shape, egui::Shape::Rect(rect) if rect.fill == highlight && shape.clip_rect.contains_rect(rect.rect))
    }), "active code match was not revealed inside the horizontal clip");
}

#[test]
fn remote_images_default_on_can_be_blocked_and_loaded_individually() {
    use fast_markdown_viewer::network;
    #[derive(Default)]
    struct TestImages(std::sync::atomic::AtomicUsize);
    impl egui::load::BytesLoader for TestImages {
        fn id(&self) -> &'static str {
            "test remote image bytes"
        }
        fn load(&self, _: &egui::Context, uri: &str) -> egui::load::BytesLoadResult {
            if !uri.starts_with("https://") {
                return Err(egui::load::LoadError::NotSupported);
            }
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(egui::load::BytesPoll::Ready { size: None, bytes: egui::load::Bytes::Static(br#"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="20" height="20" fill="red"/></svg>"#), mime: Some("image/svg+xml".into()) })
        }
        fn forget(&self, _: &str) {}
        fn forget_all(&self) {}
        fn byte_size(&self) -> usize {
            0
        }
    }
    let mut file = tempfile::NamedTempFile::new().unwrap();
    writeln!(
        file,
        "![One](https://example.com/one.svg)\n\n![Two](https://example.com/two.svg)"
    )
    .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    network::install(&context);
    let images = Arc::new(TestImages::default());
    context.add_bytes_loader(images.clone());
    let mut app = ViewerApp::new(InitialState::Path(file.path().to_owned()));
    assert!(network::automatic_images(&context));
    network::set_automatic_images(&context, false);
    let output = run_frame(&context, &mut app, input(Vec::new()));
    assert_eq!(images.0.load(std::sync::atomic::Ordering::Relaxed), 0);
    let button = node_with_text(
        accesskit_update(&output),
        egui::accesskit::Role::Button,
        "Load image",
    );
    run_frame(
        &context,
        &mut app,
        input(vec![accesskit_action(
            egui::accesskit::Action::Click,
            button,
            None,
        )]),
    );
    for _ in 0..20 {
        run_frame(&context, &mut app, input(Vec::new()));
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert_eq!(images.0.load(std::sync::atomic::Ordering::Relaxed), 1);
    let output = run_frame(&context, &mut app, input(Vec::new()));
    assert_eq!(
        accesskit_update(&output)
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button
                && node.label() == Some("Load image"))
            .count(),
        1
    );
    network::set_automatic_images(&context, true);
    for _ in 0..20 {
        run_frame(&context, &mut app, input(Vec::new()));
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert_eq!(images.0.load(std::sync::atomic::Ordering::Relaxed), 2);
    network::set_automatic_images(&context, false);
    let output = run_frame(&context, &mut app, input(Vec::new()));
    assert_eq!(
        accesskit_update(&output)
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button
                && node.label() == Some("Load image"))
            .count(),
        2
    );
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::W, egui::Modifiers::COMMAND)]),
    );
    assert_eq!(
        context
            .loaders()
            .texture
            .lock()
            .iter()
            .map(|loader| loader.byte_size())
            .sum::<usize>(),
        0
    );
}

#[test]
fn clicking_heading_and_cross_document_links_navigates_in_window() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("README.md");
    let other = dir.path().join("other.md");
    std::fs::write(
        &source,
        format!(
            "[Jump](#section)\n\n[Other](other.md#target)\n\n{}\n# Section\n\n{}",
            "body\n\n".repeat(40),
            "tail\n\n".repeat(20)
        ),
    )
    .unwrap();
    std::fs::write(
        &other,
        format!(
            "# Other\n\n{}\n# Target\n\n{}",
            "body\n\n".repeat(40),
            "tail\n\n".repeat(20)
        ),
    )
    .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Path(source));
    let output = run_frame(&context, &mut app, input(vec![]));
    let jump = node_with_text(
        accesskit_update(&output),
        egui::accesskit::Role::Label,
        "Jump",
    );
    run_frame(
        &context,
        &mut app,
        input(vec![accesskit_action(
            egui::accesskit::Action::Click,
            jump,
            None,
        )]),
    );
    for _ in 0..90 {
        run_frame(&context, &mut app, input(vec![]));
    }
    assert!(app.document_scroll_offset() > 500.0);
    run_frame(
        &context,
        &mut app,
        input(vec![key(egui::Key::Home, egui::Modifiers::NONE)]),
    );
    let output = run_frame(&context, &mut app, input(vec![]));
    let other_link = node_with_text(
        accesskit_update(&output),
        egui::accesskit::Role::Label,
        "Other",
    );
    run_frame(
        &context,
        &mut app,
        input(vec![accesskit_action(
            egui::accesskit::Action::Click,
            other_link,
            None,
        )]),
    );
    for _ in 0..90 {
        run_frame(&context, &mut app, input(vec![]));
    }
    assert_eq!(app.tab_count(), 2);
    assert_eq!(
        app.document_path(),
        Some(other.canonicalize().unwrap().as_path())
    );
    assert!(app.document_scroll_offset() > 500.0);
}

#[test]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::too_many_lines)] // Exercises the complete context-menu/dialog transaction.
fn tab_menu_reveals_and_renames_the_real_file() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("old.md");
    std::fs::write(&source, "# Original").unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let services = Arc::new(TestServices::default());
    let mut app = ViewerApp::with_services(InitialState::Path(source.clone()), services.clone());
    for action in [
        if cfg!(target_os = "windows") {
            "Show in Explorer"
        } else {
            "Show in file manager"
        },
        "Rename file…",
    ] {
        let output = run_frame(&context, &mut app, input(vec![]));
        let update = accesskit_update(&output);
        let tab = node_with_text(update, egui::accesskit::Role::Button, "old.md");
        let bounds = update
            .nodes
            .iter()
            .find(|(id, _)| *id == tab)
            .unwrap()
            .1
            .bounds()
            .unwrap();
        let pos = egui::pos2(
            bounds.x0.midpoint(bounds.x1) as f32,
            bounds.y0.midpoint(bounds.y1) as f32,
        );
        for pressed in [true, false] {
            run_frame(
                &context,
                &mut app,
                input(vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Secondary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ]),
            );
        }
        let output = run_frame(&context, &mut app, input(vec![]));
        let item = node_with_text(
            accesskit_update(&output),
            egui::accesskit::Role::Button,
            action,
        );
        run_frame(
            &context,
            &mut app,
            input(vec![accesskit_action(
                egui::accesskit::Action::Click,
                item,
                None,
            )]),
        );
    }
    assert_eq!(
        services.revealed_paths.lock().unwrap().as_slice(),
        &[source.canonicalize().unwrap()]
    );
    run_frame(&context, &mut app, input(vec![]));
    run_frame(
        &context,
        &mut app,
        input(vec![
            key(egui::Key::A, egui::Modifiers::COMMAND),
            egui::Event::Text("renamed".into()),
        ]),
    );
    let output = run_frame(&context, &mut app, input(vec![]));
    let rename = node_with_text(
        accesskit_update(&output),
        egui::accesskit::Role::Button,
        "Rename",
    );
    run_frame(
        &context,
        &mut app,
        input(vec![accesskit_action(
            egui::accesskit::Action::Click,
            rename,
            None,
        )]),
    );
    assert!(!source.exists());
    assert_eq!(
        app.document_path(),
        Some(
            dir.path()
                .join("renamed.md")
                .canonicalize()
                .unwrap()
                .as_path()
        )
    );
    assert_eq!(app.tab_count(), 1);
}
