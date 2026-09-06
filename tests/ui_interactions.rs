use std::{
    io,
    io::Write as _,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use eframe::egui;
use fast_markdown_viewer::app::{AppServices, InitialState, ViewerApp};
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
    markdown_paths: Mutex<Vec<PathBuf>>,
}

impl AppServices for TestServices {
    fn choose_markdown_file(&self) -> Option<PathBuf> {
        self.selected_file.lock().unwrap().take()
    }

    fn open_browser(&self, url: &Url) -> io::Result<()> {
        self.browser_urls.lock().unwrap().push(url.clone());
        Ok(())
    }

    fn open_markdown(&self, path: &Path) -> io::Result<()> {
        self.markdown_paths.lock().unwrap().push(path.to_owned());
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
fn dropping_one_file_loads_it_and_multiple_files_show_an_error() {
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
    assert_eq!(
        app.error_message(),
        Some("Drop one Markdown file at a time")
    );
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
    assert_eq!(
        services.markdown_paths.lock().unwrap().as_slice(),
        &[file
            .path()
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .join("chapter.md")]
    );
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
