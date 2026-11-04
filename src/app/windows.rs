//! Native window ownership. Only this host creates/removes deferred viewports.
use super::{AppServices, DocumentTab, InitialState, PlatformServices, ViewerWindow};
use eframe::egui;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

struct DetachedWindow {
    state: Arc<Mutex<ViewerWindow>>,
    builder: egui::ViewportBuilder,
}

struct Spawn {
    window: ViewerWindow,
    position: Option<egui::Pos2>,
    size: egui::Vec2,
}

enum WindowEvent {
    Spawn(Box<Spawn>),
    OpenFiles {
        source: egui::ViewportId,
        paths: Vec<PathBuf>,
    },
    MoveTab {
        tab_id: u64,
        source: egui::ViewportId,
        destination: egui::ViewportId,
    },
    Close(egui::ViewportId),
}

/// One process with independently repainted native document windows.
pub struct ViewerApp {
    launch_session: Option<crate::launch::Session>,
    #[cfg(target_os = "macos")]
    native_events: Option<fmv_macos_events::Inbox>,
    persistence: Option<(std::path::PathBuf, egui::Context)>,
    exit_session: Option<Vec<crate::persistence::Window>>,
    root: ViewerWindow,
    detached: BTreeMap<egui::ViewportId, DetachedWindow>,
    pending: Arc<Mutex<Vec<WindowEvent>>>,
    next_window_id: u64,
    root_closed: bool,
}

impl ViewerApp {
    #[must_use]
    pub fn new(initial: InitialState) -> Self {
        Self::with_services(initial, Arc::new(PlatformServices))
    }

    #[must_use]
    pub fn with_services(initial: InitialState, services: Arc<dyn AppServices>) -> Self {
        Self {
            launch_session: None,
            #[cfg(target_os = "macos")]
            native_events: None,
            persistence: None,
            exit_session: None,
            root: ViewerWindow::with_services(initial, services),
            detached: BTreeMap::new(),
            pending: Arc::default(),
            next_window_id: 0,
            root_closed: false,
        }
    }

    /// Restore local preferences; reopen the last session only for a bare launch.
    #[must_use]
    pub fn with_saved_state(initial: InitialState, context: &egui::Context) -> Self {
        let Some(path) = crate::persistence::path() else {
            crate::appearance::ThemeChoice::default().apply(context);
            return Self::new(initial);
        };
        Self::restore_from(initial, context, path)
    }

    fn restore_from(
        initial: InitialState,
        context: &egui::Context,
        path: std::path::PathBuf,
    ) -> Self {
        let state = crate::persistence::read(&path);
        let restore_session = matches!(initial, InitialState::Empty);
        let mut app = Self::new(initial);
        *app.root.theme.lock().expect("appearance lock") = state.theme;
        state.theme.apply(context);
        context.set_zoom_factor(state.zoom);
        crate::appearance::set_text_size_percent(context, state.text_size_percent);
        crate::fonts::restore(context, &state.fonts);
        crate::network::set_automatic_images(context, state.automatic_images);
        context.data_mut(|data| data.insert_temp(egui::Id::new("word_wrap"), state.word_wrap));
        state.document_width.apply(context);
        if restore_session {
            let mut windows = state.windows.into_iter();
            if let Some(window) = windows.next() {
                app.root.restore_session(window);
            }
            for saved in windows.filter(|window| !window.tabs.is_empty()) {
                let mut window =
                    ViewerWindow::with_services(InitialState::Empty, app.root.services.clone());
                window.theme = app.root.theme.clone();
                window.tab_ids = app.root.tab_ids.clone();
                window.restore_session(saved);
                app.spawn(Spawn {
                    window,
                    position: None,
                    size: egui::vec2(900.0, 700.0),
                });
            }
        }
        app.persistence = Some((path, context.clone()));
        app
    }

    fn session(&self) -> Vec<crate::persistence::Window> {
        let mut windows = Vec::new();
        if !self.root_closed {
            windows.push(self.root.session());
        }
        windows.extend(
            self.detached
                .values()
                .map(|window| window.state.lock().expect("document window lock").session()),
        );
        windows
    }

    /// Connect Finder open/quit events to the existing document and exit paths.
    #[cfg(target_os = "macos")]
    #[must_use]
    pub fn with_native_events(
        mut self,
        inbox: fmv_macos_events::Inbox,
        context: &egui::Context,
    ) -> Self {
        fmv_macos_events::install_edit_menu();
        let wake_context = context.clone();
        inbox.set_wake(move || wake_context.request_repaint_of(egui::ViewportId::ROOT));
        self.native_events = Some(inbox);
        self.process_native_events(context);
        self
    }

    #[cfg(target_os = "macos")]
    fn process_native_events(&mut self, context: &egui::Context) {
        let events = self
            .native_events
            .as_ref()
            .map(fmv_macos_events::Inbox::drain)
            .unwrap_or_default();
        for event in events {
            match event {
                fmv_macos_events::Event::Open(paths) => self.open_external_files(&paths, context),
                fmv_macos_events::Event::Quit => {
                    context
                        .data_mut(|data| data.insert_temp(egui::Id::new("quit_application"), true));
                }
            }
        }
    }

    /// Listen for files opened through the desktop or another command-line launch.
    #[must_use]
    pub fn with_launch_session(
        mut self,
        session: Option<crate::launch::Session>,
        context: &egui::Context,
    ) -> Self {
        if let Some(session) = &session {
            session.attach(context);
        }
        self.launch_session = session;
        self
    }

    fn process_launch_requests(&mut self, context: &egui::Context) {
        while let Some(request) = self
            .launch_session
            .as_ref()
            .and_then(crate::launch::Session::next)
        {
            let paths = request.acknowledge();
            self.open_external_files(&paths, context);
        }
    }

    fn open_external_files(&mut self, paths: &[PathBuf], context: &egui::Context) {
        let root = egui::ViewportId::ROOT;
        let last_focused = context
            .data(|data| data.get_temp::<egui::ViewportId>(egui::Id::new("last_focused_window")));
        let fallback = if self.root_closed {
            self.detached.keys().next().copied().unwrap_or(root)
        } else {
            root
        };
        let target = last_focused
            .filter(|id| {
                if *id == root {
                    !self.root_closed
                } else {
                    self.detached.contains_key(id)
                }
            })
            .unwrap_or(fallback);
        if target == root && self.root_closed {
            self.root_closed = false;
            self.root.close_requested = false;
            context.send_viewport_cmd_to(root, egui::ViewportCommand::Visible(true));
        }
        self.open_files(target, paths, context);
    }

    fn open_files(&mut self, target: egui::ViewportId, paths: &[PathBuf], context: &egui::Context) {
        let root = egui::ViewportId::ROOT;
        let mut focused = target;
        let mut errors = Vec::new();
        for path in paths {
            let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
            let existing = if !self.root_closed && self.root.activate_tab(&canonical) {
                Some(root)
            } else {
                self.detached.iter().find_map(|(&id, window)| {
                    let mut window = window.state.lock().expect("document window lock");
                    window.activate_tab(&canonical).then_some(id)
                })
            };
            if let Some(id) = existing {
                focused = id;
            } else {
                self.with_window(target, |window| {
                    window.load(path);
                    errors.extend(window.error.take());
                });
                focused = target;
            }
        }
        if !errors.is_empty() {
            self.with_window(target, |window| window.error = Some(errors.join("\n")));
            focused = target;
        }
        context.send_viewport_cmd_to(focused, egui::ViewportCommand::Minimized(false));
        context.send_viewport_cmd_to(focused, egui::ViewportCommand::Focus);
    }

    fn with_window(&mut self, id: egui::ViewportId, action: impl FnOnce(&mut ViewerWindow)) {
        if id == egui::ViewportId::ROOT {
            action(&mut self.root);
        } else if let Some(window) = self.detached.get(&id) {
            action(&mut window.state.lock().expect("document window lock"));
        }
    }

    #[must_use]
    pub fn document_path(&self) -> Option<&Path> {
        self.root.document_path()
    }
    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        self.root.error_message()
    }
    #[must_use]
    pub fn document_scroll_offset(&self) -> f32 {
        self.root.document_scroll_offset()
    }
    #[must_use]
    pub fn tab_count(&self) -> usize {
        self.root.tab_count()
    }
    #[must_use]
    pub const fn outline_visible(&self) -> bool {
        self.root.outline_visible()
    }
    #[must_use]
    pub fn search_status(&self) -> Option<(usize, usize)> {
        self.root.search_status()
    }
    #[must_use]
    pub fn detached_window_count(&self) -> usize {
        self.detached.len()
    }

    fn spawn(&mut self, spawn: Spawn) {
        let id = egui::ViewportId::from_hash_of(("document_window", self.next_window_id));
        self.next_window_id += 1;
        let mut builder = egui::ViewportBuilder::default()
            .with_title(format!(
                "{} — FastMarkdownViewer",
                spawn.window.tabs[0].document.title
            ))
            .with_app_id("FastMarkdownViewer")
            .with_inner_size(spawn.size)
            .with_min_inner_size([420.0, 280.0])
            .with_icon(crate::platform::app_icon());
        if let Some(position) = spawn.position {
            builder = builder.with_position(position);
        }
        self.detached.insert(
            id,
            DetachedWindow {
                state: Arc::new(Mutex::new(spawn.window)),
                builder,
            },
        );
    }

    fn drop_destination(
        context: &egui::Context,
        pointer: egui::Pos2,
        root_visible: bool,
    ) -> Option<egui::ViewportId> {
        let zoom = context.zoom_factor();
        context.input(|input| {
            let source = input.viewport();
            // Each viewport reports desktop positions in its own scale. Compare in pixels
            // so drops also work across monitors with different DPI settings.
            let point = (source.inner_rect?.min + pointer.to_vec2()) * input.pixels_per_point();
            input
                .raw
                .viewports
                .iter()
                .filter(|(id, info)| {
                    **id != input.raw.viewport_id
                        && (root_visible || **id != egui::ViewportId::ROOT)
                        && info.visible() != Some(false)
                        && !info.close_requested()
                        && info.outer_rect.or(info.inner_rect).is_some_and(|bounds| {
                            (bounds * (info.native_pixels_per_point.unwrap_or(1.0) * zoom))
                                .contains(point)
                        })
                })
                .max_by_key(|(_, info)| info.focused == Some(true))
                .map(|(&id, _)| id)
        })
    }

    fn take_moving_tab(window: &mut ViewerWindow, tab_id: u64) -> Option<DocumentTab> {
        let index = window.tabs.iter().position(|tab| tab.id == tab_id)?;
        let mut tab = window.remove_tab(index);
        // Geometry belongs to the old viewport; reading position, query and caches move intact.
        tab.markdown_cache.navigation.clear();
        tab.markdown_cache.navigation.scroll_target = None;
        window.tab_drag = None;
        Some(tab)
    }

    fn take_window_event(
        window: &mut ViewerWindow,
        context: &egui::Context,
        root_visible: bool,
    ) -> Option<WindowEvent> {
        if !window.picked_files.is_empty() {
            return Some(WindowEvent::OpenFiles {
                source: context.viewport_id(),
                paths: std::mem::take(&mut window.picked_files),
            });
        }
        let request = window.detach_request.take()?;
        if let Some(pointer) = request.pointer
            && let Some(destination) = Self::drop_destination(context, pointer, root_visible)
        {
            return Some(WindowEvent::MoveTab {
                tab_id: request.tab_id,
                source: context.viewport_id(),
                destination,
            });
        }
        let tab = Self::take_moving_tab(window, request.tab_id)?;
        let child = ViewerWindow {
            tabs: vec![tab],
            active: 0,
            tab_ids: window.tab_ids.clone(),
            error: None,
            services: window.services.clone(),
            theme: window.theme.clone(),
            show_outline: window.show_outline,
            focus_search: false,
            tab_drag: None,
            drag_cancelled: false,
            detach_request: None,
            picked_files: Vec::new(),
            close_requested: false,
            rename_dialog: None,
            native_title: None,
        };
        Some(WindowEvent::Spawn(Box::new(Spawn {
            window: child,
            position: request.position,
            size: request.size,
        })))
    }

    fn move_tab(
        &mut self,
        tab_id: u64,
        source: egui::ViewportId,
        destination: egui::ViewportId,
    ) -> Option<()> {
        let root = egui::ViewportId::ROOT;
        if source == destination || (self.root_closed && (source == root || destination == root)) {
            return None;
        }
        let detached = |id| {
            let window = self.detached.get(&id)?;
            Some(window.state.lock().expect("document window lock"))
        };
        let (mut from, mut to);
        let (source, destination) = if source == root {
            to = detached(destination)?;
            (&mut self.root, &mut *to)
        } else if destination == root {
            from = detached(source)?;
            (&mut *from, &mut self.root)
        } else {
            from = detached(source)?;
            to = detached(destination)?;
            (&mut *from, &mut *to)
        };
        if source.close_requested || destination.close_requested {
            return None;
        }
        let tab = Self::take_moving_tab(source, tab_id)?;
        // Keep one tab per file, using the state the user just dragged in.
        if let Some(index) = destination
            .tabs
            .iter()
            .position(|other| other.document.path == tab.document.path)
        {
            destination.tabs[index] = tab;
            destination.active = index;
        } else {
            destination.active = destination.tabs.len();
            destination.tabs.push(tab);
        }
        destination.error = None;
        Some(())
    }

    fn process_events(&mut self, context: &egui::Context) {
        let events = std::mem::take(&mut *self.pending.lock().expect("window events lock"));
        for event in events {
            self.handle_event(event, context);
        }
    }

    fn handle_event(&mut self, event: WindowEvent, context: &egui::Context) {
        match event {
            WindowEvent::Spawn(spawn) => self.spawn(*spawn),
            WindowEvent::OpenFiles { source, paths } => self.open_files(source, &paths, context),
            WindowEvent::MoveTab {
                tab_id,
                source,
                destination,
            } => {
                let root = egui::ViewportId::ROOT;
                let root_closing = (source == root || destination == root)
                    && context.input(|input| input.viewport().close_requested());
                if !root_closing && self.move_tab(tab_id, source, destination).is_some() {
                    context.request_repaint_of(source);
                    context.send_viewport_cmd_to(destination, egui::ViewportCommand::Focus);
                }
            }
            WindowEvent::Close(id) => {
                if self.root_closed && self.detached.len() == 1 {
                    self.exit_session = Some(self.session());
                }
                self.detached.remove(&id);
            }
        }
    }

    fn handle_root_close(&mut self, context: &egui::Context) {
        if context
            .data_mut(|data| data.remove_temp::<bool>(egui::Id::new("quit_application")))
            .unwrap_or(false)
        {
            self.exit_session = Some(self.session());
            self.root_closed = true;
            self.detached.clear();
            context.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }
        let close =
            self.root.close_requested || context.input(|input| input.viewport().close_requested());
        if close && !self.root_closed {
            if self.detached.is_empty() {
                self.exit_session = Some(self.session());
                context.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                // egui's root must exist until the process exits. Hide it while independent
                // document windows remain, and release its documents immediately.
                self.root_closed = true;
                for tab in &self.root.tabs {
                    for uri in tab.image_uris.lock().expect("image references lock").iter() {
                        context.forget_image(uri);
                    }
                }
                self.root.tabs.clear();
                context.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                context.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            }
        }
        if self.root_closed && self.detached.is_empty() {
            context.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    /// Render the root and register independently repainted document windows.
    ///
    /// # Panics
    /// Panics if a window-state mutex was poisoned by an earlier panic.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let context = ui.ctx().clone();
        self.process_events(&context);
        if !self.root_closed {
            self.root.show(ui);

            if let Some(event) = Self::take_window_event(&mut self.root, &context, true) {
                self.handle_event(event, &context);
                context.request_repaint();
            }
        }

        self.handle_root_close(&context);

        for (&id, window) in &self.detached {
            let state = window.state.clone();
            let pending = self.pending.clone();
            let root_visible = !self.root_closed;
            context.show_viewport_deferred(id, window.builder.clone(), move |ui, _class| {
                let mut window = state.lock().expect("document window lock");
                let native_close = ui.input(|input| input.viewport().close_requested());
                if !native_close && !window.close_requested {
                    window.show(ui);
                }
                if native_close || window.close_requested {
                    for tab in &window.tabs {
                        for uri in tab.image_uris.lock().expect("image references lock").iter() {
                            ui.ctx().forget_image(uri);
                        }
                    }
                    window.close_requested = true;
                    pending
                        .lock()
                        .expect("window events lock")
                        .push(WindowEvent::Close(id));
                    ui.ctx().request_repaint_of(egui::ViewportId::ROOT);
                } else if let Some(event) =
                    Self::take_window_event(&mut window, ui.ctx(), root_visible)
                {
                    pending.lock().expect("window events lock").push(event);
                    ui.ctx().request_repaint_of(egui::ViewportId::ROOT);
                }
            });
        }
    }
}

impl Drop for ViewerApp {
    fn drop(&mut self) {
        let Some((path, context)) = &self.persistence else {
            return;
        };
        let state = crate::persistence::State {
            theme: *self.root.theme.lock().expect("appearance lock"),
            fonts: crate::fonts::preferences(context),
            document_width: crate::appearance::DocumentWidth::current(context),
            zoom: context.zoom_factor(),
            text_size_percent: crate::appearance::text_size_percent(context),
            automatic_images: crate::network::automatic_images(context),
            word_wrap: context.data(|data| {
                data.get_temp::<bool>(egui::Id::new("word_wrap"))
                    .unwrap_or(true)
            }),
            windows: self.exit_session.take().unwrap_or_else(|| self.session()),
            ..crate::persistence::State::default()
        };
        if let Err(error) = crate::persistence::write(path, &state) {
            eprintln!("Could not save FastMarkdownViewer settings: {error}");
        }
    }
}

impl eframe::App for ViewerApp {
    fn logic(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "macos")]
        self.process_native_events(context);
        // eframe can skip UI when every window is minimized/occluded. Lifetime
        // events must still be processed so the final close always exits.
        self.process_events(context);
        self.handle_root_close(context);
        if self.exit_session.is_none() {
            self.process_launch_requests(context);
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
    }
    fn persist_egui_memory(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
