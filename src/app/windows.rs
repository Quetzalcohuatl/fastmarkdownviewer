//! Native window ownership. Only this host creates/removes deferred viewports.
use super::{AppServices, InitialState, PlatformServices, ViewerWindow};
use eframe::egui;
use std::{
    collections::BTreeMap,
    path::Path,
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
    Close(egui::ViewportId),
}

/// One process with independently repainted native document windows.
pub struct ViewerApp {
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
            root: ViewerWindow::with_services(initial, services),
            detached: BTreeMap::new(),
            pending: Arc::default(),
            next_window_id: 0,
            root_closed: false,
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

    fn take_spawn(window: &mut ViewerWindow) -> Option<Spawn> {
        let request = window.detach_request.take()?;
        let index = window
            .tabs
            .iter()
            .position(|tab| tab.id == request.tab_id)?;
        let mut tab = window.remove_tab(index);
        // Geometry belongs to the old viewport; reading position, query and caches move intact.
        tab.markdown_cache.navigation.clear();
        tab.markdown_cache.navigation.scroll_target = None;
        window.tab_drag = None;
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
            close_requested: false,
            rename_dialog: None,
        };
        Some(Spawn {
            window: child,
            position: request.position,
            size: request.size,
        })
    }

    fn process_events(&mut self) {
        let events = std::mem::take(&mut *self.pending.lock().expect("window events lock"));
        for event in events {
            match event {
                WindowEvent::Spawn(spawn) => self.spawn(*spawn),
                WindowEvent::Close(id) => {
                    self.detached.remove(&id);
                }
            }
        }
    }

    fn handle_root_close(&mut self, context: &egui::Context) {
        let close =
            self.root.close_requested || context.input(|input| input.viewport().close_requested());
        if close && !self.root_closed {
            if self.detached.is_empty() {
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
        self.process_events();
        if !self.root_closed {
            self.root.show(ui);

            if let Some(spawn) = Self::take_spawn(&mut self.root) {
                self.spawn(spawn);
                context.request_repaint();
            }
        }

        self.handle_root_close(&context);

        for (&id, window) in &self.detached {
            let state = window.state.clone();
            let pending = self.pending.clone();
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
                } else if let Some(spawn) = Self::take_spawn(&mut window) {
                    pending
                        .lock()
                        .expect("window events lock")
                        .push(WindowEvent::Spawn(Box::new(spawn)));
                    ui.ctx().request_repaint_of(egui::ViewportId::ROOT);
                }
            });
        }
    }
}

impl eframe::App for ViewerApp {
    fn logic(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        // eframe can skip UI when every window is minimized/occluded. Lifetime
        // events must still be processed so the final close always exits.
        self.process_events();
        self.handle_root_close(context);
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
