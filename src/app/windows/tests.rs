use super::*;

fn input(events: Vec<egui::Event>) -> egui::RawInput {
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
        events,
        ..Default::default()
    };
    let viewport = input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap();
    // A monitor to the left of the primary display, with native title bar/borders.
    viewport.inner_rect = Some(egui::Rect::from_min_size(
        egui::pos2(-1200.0, 80.0),
        egui::vec2(800.0, 600.0),
    ));
    viewport.outer_rect = Some(egui::Rect::from_min_max(
        egui::pos2(-1208.0, 50.0),
        egui::pos2(-392.0, 688.0),
    ));
    input
}

fn frame(context: &egui::Context, app: &mut ViewerApp, input: egui::RawInput) -> egui::FullOutput {
    let mut output = context.run_ui(input, |ui| app.show(ui));
    output.textures_delta.clear();
    output
}

fn button(position: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: position,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

#[allow(clippy::cast_possible_truncation)]
fn tab_center(output: &egui::FullOutput, title: &str) -> egui::Pos2 {
    let rect = output
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some(title)
        })
        .unwrap()
        .1
        .bounds()
        .unwrap();
    egui::pos2(
        f64::midpoint(rect.x0, rect.x1) as f32,
        f64::midpoint(rect.y0, rect.y1) as f32,
    )
}

fn fixture() -> (tempfile::TempDir, egui::Context, ViewerApp, egui::Pos2) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tear-out.md");
    std::fs::write(
        &path,
        format!("# Heading\n\n{}", "marker paragraph\n\n".repeat(100)),
    )
    .unwrap();
    let context = egui::Context::default();
    context.set_embed_viewports(false);
    context.enable_accesskit();
    let mut app = ViewerApp::new(InitialState::Path(path));
    let output = frame(&context, &mut app, input(Vec::new()));
    let center = tab_center(&output, "tear-out.md");
    (directory, context, app, center)
}

fn drag(
    context: &egui::Context,
    app: &mut ViewerApp,
    start: egui::Pos2,
    end: egui::Pos2,
) -> egui::FullOutput {
    frame(
        context,
        app,
        input(vec![egui::Event::PointerMoved(start), button(start, true)]),
    );
    frame(context, app, input(vec![egui::Event::PointerMoved(end)]));
    frame(context, app, input(vec![button(end, false)]))
}

fn child_frame(
    context: &egui::Context,
    id: egui::ViewportId,
    callback: &egui::DeferredViewportUiCallback,
    events: Vec<egui::Event>,
    close: bool,
) -> egui::FullOutput {
    let mut raw = input(events);
    raw.viewport_id = id;
    let mut info = raw.viewports[&egui::ViewportId::ROOT].clone();
    info.parent = Some(egui::ViewportId::ROOT);
    if close {
        info.events.push(egui::ViewportEvent::Close);
    }
    raw.viewports.insert(id, info);
    let mut output = context.run_ui(raw, |ui| callback(ui));
    output.textures_delta.clear();
    output
}

#[test]
fn outside_release_moves_document_and_reading_state_without_reloading() {
    let (_directory, context, mut app, start) = fixture();
    app.root.tabs[0].search.open = true;
    app.root.tabs[0].search.query = "marker".into();
    frame(&context, &mut app, input(Vec::new()));
    for _ in 0..30 {
        frame(&context, &mut app, input(Vec::new()));
    }
    app.root.tabs[0].scroll_offset = 250.0;
    app.root.show_outline = false;
    let original_source = app.root.tabs[0].document.source.as_ptr();
    let outside = egui::pos2(-80.0, 150.0);
    let output = drag(&context, &mut app, start, outside);
    assert_eq!(app.tab_count(), 0);
    assert_eq!(app.detached_window_count(), 1);
    let (&id, child) = app.detached.first_key_value().unwrap();
    assert!(output.viewport_output[&id].class == egui::ViewportClass::Deferred);
    assert_eq!(child.builder.position, Some(egui::pos2(-1360.0, 210.0)));
    let state = child.state.lock().unwrap();
    assert_eq!(state.tabs[0].document.source.as_ptr(), original_source);
    assert_eq!(state.search_status(), Some((1, 100)));
    assert!((state.tabs[0].scroll_offset - 250.0).abs() < 1.0);
    assert!(!state.show_outline);
    assert!(Arc::ptr_eq(&state.tab_ids, &app.root.tab_ids));
    assert!(Arc::ptr_eq(&state.theme, &app.root.theme));
}

#[test]
fn internal_releases_and_escape_cancel_without_losing_the_tab() {
    let (_directory, context, mut app, start) = fixture();
    // Releasing over content or native title bar is still inside the original window.
    drag(&context, &mut app, start, egui::pos2(400.0, 180.0));
    drag(&context, &mut app, start, egui::pos2(400.0, -15.0));
    assert_eq!(app.detached_window_count(), 0);
    let outside = egui::pos2(900.0, 200.0);
    frame(
        &context,
        &mut app,
        input(vec![egui::Event::PointerMoved(start), button(start, true)]),
    );
    frame(
        &context,
        &mut app,
        input(vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]),
    );
    frame(
        &context,
        &mut app,
        input(vec![
            egui::Event::PointerMoved(outside),
            button(outside, false),
        ]),
    );
    assert_eq!(app.tab_count(), 1);
    assert_eq!(app.detached_window_count(), 0);
    // Cancellation must not disable later drags.
    drag(&context, &mut app, start, outside);
    assert_eq!(app.detached_window_count(), 1);
}

#[test]
fn closing_original_keeps_child_alive_and_last_window_close_exits() {
    let (_directory, context, mut app, start) = fixture();
    let output = drag(&context, &mut app, start, egui::pos2(900.0, 150.0));
    let id = *app.detached.first_key_value().unwrap().0;
    let callback = output.viewport_output[&id].viewport_ui_cb.clone().unwrap();
    let mut close_root = input(Vec::new());
    close_root
        .viewports
        .get_mut(&egui::ViewportId::ROOT)
        .unwrap()
        .events
        .push(egui::ViewportEvent::Close);
    let output = frame(&context, &mut app, close_root);
    assert!(app.root_closed);
    let commands = &output.viewport_output[&egui::ViewportId::ROOT].commands;
    assert!(commands.contains(&egui::ViewportCommand::CancelClose));
    assert!(commands.contains(&egui::ViewportCommand::Visible(false)));
    child_frame(&context, id, &*callback, Vec::new(), false);
    assert_eq!(app.detached[&id].state.lock().unwrap().tab_count(), 1);
    child_frame(&context, id, &*callback, Vec::new(), true);
    let output = frame(&context, &mut app, input(Vec::new()));
    assert_eq!(app.detached_window_count(), 0);
    assert!(
        output.viewport_output[&egui::ViewportId::ROOT]
            .commands
            .contains(&egui::ViewportCommand::Close)
    );
}

#[test]
fn detached_windows_can_tear_out_again_with_unique_ids() {
    let (_directory, context, mut app, start) = fixture();
    let output = drag(&context, &mut app, start, egui::pos2(900.0, 150.0));
    let first = *app.detached.first_key_value().unwrap().0;
    let callback = output.viewport_output[&first]
        .viewport_ui_cb
        .clone()
        .unwrap();
    let child = child_frame(&context, first, &*callback, Vec::new(), false);
    let center = tab_center(&child, "tear-out.md");
    child_frame(
        &context,
        first,
        &*callback,
        vec![egui::Event::PointerMoved(center), button(center, true)],
        false,
    );
    let outside = egui::pos2(900.0, 180.0);
    child_frame(
        &context,
        first,
        &*callback,
        vec![egui::Event::PointerMoved(outside)],
        false,
    );
    child_frame(
        &context,
        first,
        &*callback,
        vec![button(outside, false)],
        false,
    );
    frame(&context, &mut app, input(Vec::new()));
    assert_eq!(app.detached_window_count(), 2);
    assert_eq!(app.detached[&first].state.lock().unwrap().tab_count(), 0);
    let second = app.detached.keys().find(|id| **id != first).unwrap();
    assert_eq!(app.detached[second].state.lock().unwrap().tab_count(), 1);
}

#[test]
fn dragging_tabs_reorders_without_changing_active_document_or_tear_out() {
    let (directory, context, mut app, _) = fixture();
    let second = directory.path().join("second.md");
    std::fs::write(&second, "# Second").unwrap();
    app.root.load(&second);
    let output = frame(&context, &mut app, input(Vec::new()));
    let first = tab_center(&output, "tear-out.md");
    let second_center = tab_center(&output, "second.md");
    let active_id = app.root.tabs[app.root.active].id;
    drag(&context, &mut app, first, second_center);
    assert_eq!(app.root.tabs[0].document.title, "second.md");
    assert_eq!(app.root.tabs[1].document.title, "tear-out.md");
    assert_eq!(app.root.tabs[app.root.active].id, active_id);
    assert_eq!(app.detached_window_count(), 0);
    let output = frame(&context, &mut app, input(Vec::new()));
    let moved = tab_center(&output, "tear-out.md");
    drag(&context, &mut app, moved, egui::pos2(920.0, 150.0));
    assert_eq!(app.detached_window_count(), 1);
    assert_eq!(app.root.tabs[0].document.title, "second.md");
}

#[test]
fn reload_keeps_tab_and_reading_state_and_failed_reload_keeps_document() {
    let (directory, context, mut app, _) = fixture();
    let path = directory.path().join("tear-out.md");
    frame(
        &context,
        &mut app,
        input(vec![egui::Event::Key {
            key: egui::Key::PageDown,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]),
    );
    let scroll = app.document_scroll_offset();
    let id = app.root.tabs[0].id;
    std::fs::write(
        &path,
        format!("# Updated\n\n{}", "New paragraph.\n\n".repeat(120)),
    )
    .unwrap();
    for (key, modifiers) in [
        (egui::Key::F5, egui::Modifiers::NONE),
        (egui::Key::R, egui::Modifiers::COMMAND),
    ] {
        frame(
            &context,
            &mut app,
            input(vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            }]),
        );
        assert!(app.root.tabs[0].document.source.contains("Updated"));
        assert_eq!(app.tab_count(), 1);
        assert_eq!(app.root.tabs[0].id, id);
        assert!((app.document_scroll_offset() - scroll).abs() < 2.0);
    }
    std::fs::write(&path, [0xff]).unwrap();
    app.root.reload(&context);
    assert!(app.error_message().unwrap().contains("Could not reload"));
    assert!(app.root.tabs[0].document.source.contains("Updated"));
}
