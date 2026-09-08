use std::{
    io,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use url::Url;

use crate::{
    document::Document,
    links::{self, LinkAction},
    math::MathRenderer,
    platform,
};

mod windows;
pub use windows::ViewerApp;

const LINE_SCROLL_POINTS: f32 = 48.0;
const PAGE_SCROLL_FRACTION: f32 = 0.9;
const MIN_ZOOM_FACTOR: f32 = 0.5;
const MAX_ZOOM_FACTOR: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeChoice {
    System,
    Light,
    Dark,
}

impl ThemeChoice {
    const fn preference(self) -> egui::ThemePreference {
        match self {
            Self::System => egui::ThemePreference::System,
            Self::Light => egui::ThemePreference::Light,
            Self::Dark => egui::ThemePreference::Dark,
        }
    }
}

#[derive(Debug)]
pub enum InitialState {
    Empty,
    Path(PathBuf),
    Error(String),
}

pub trait AppServices: Send + Sync {
    fn choose_markdown_file(&self) -> Option<PathBuf>;

    /// Open a validated web URL.
    ///
    /// # Errors
    ///
    /// Returns an error when the platform cannot open the URL.
    fn open_browser(&self, url: &Url) -> io::Result<()>;

    /// Open a validated Markdown path in another viewer process.
    ///
    /// # Errors
    ///
    /// Returns an error when the platform cannot start another viewer.
    fn open_markdown(&self, path: &Path) -> io::Result<()>;
}

#[derive(Debug, Default)]
pub struct PlatformServices;

impl AppServices for PlatformServices {
    fn choose_markdown_file(&self) -> Option<PathBuf> {
        platform::choose_markdown_file()
    }

    fn open_browser(&self, url: &Url) -> io::Result<()> {
        platform::open_browser(url)
    }

    fn open_markdown(&self, path: &Path) -> io::Result<()> {
        platform::open_markdown(path)
    }
}

/// Self-contained document state moved intact between native windows.
struct DocumentTab {
    id: u64,
    document: Document,
    markdown_cache: CommonMarkCache,
    math: MathRenderer,
    scroll_offset: f32,
    max_scroll_offset: f32,
    search: crate::search::Search,
    heading_target: Option<usize>,
    fonts_checked: bool,
}

#[derive(Clone, Copy)]
struct TabDrag {
    tab_id: u64,
    origin: egui::Pos2,
    latest: egui::Pos2,
}

struct DetachRequest {
    tab_id: u64,
    position: Option<egui::Pos2>,
    size: egui::Vec2,
}

// Independent UI preferences and one-shot input/lifetime flags are not exclusive states.
#[allow(clippy::struct_excessive_bools)]
struct ViewerWindow {
    tabs: Vec<DocumentTab>,
    active: usize,
    tab_ids: Arc<AtomicU64>,
    error: Option<String>,
    services: Arc<dyn AppServices>,
    theme: Arc<Mutex<ThemeChoice>>,
    show_outline: bool,
    focus_search: bool,
    tab_drag: Option<TabDrag>,
    drag_cancelled: bool,
    detach_request: Option<DetachRequest>,
    close_requested: bool,
}

impl ViewerWindow {
    #[must_use]
    pub fn with_services(initial: InitialState, services: Arc<dyn AppServices>) -> Self {
        let mut app = Self {
            tabs: Vec::new(),
            active: 0,
            tab_ids: Arc::new(AtomicU64::new(0)),
            error: None,
            services,
            theme: Arc::new(Mutex::new(ThemeChoice::System)),
            show_outline: true,
            focus_search: false,
            tab_drag: None,
            drag_cancelled: false,
            detach_request: None,
            close_requested: false,
        };
        match initial {
            InitialState::Empty => {}
            InitialState::Path(path) => app.load(&path),
            InitialState::Error(error) => app.error = Some(error),
        }
        app
    }

    #[must_use]
    pub fn document_path(&self) -> Option<&Path> {
        self.tabs
            .get(self.active)
            .map(|tab| tab.document.path.as_path())
    }

    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }

    #[must_use]
    pub fn document_scroll_offset(&self) -> f32 {
        self.tabs
            .get(self.active)
            .map_or(0.0, |tab| tab.scroll_offset)
    }

    #[must_use]
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    #[must_use]
    pub const fn outline_visible(&self) -> bool {
        self.show_outline
    }

    #[must_use]
    pub fn search_status(&self) -> Option<(usize, usize)> {
        self.tabs
            .get(self.active)
            .filter(|tab| tab.search.open)
            .map(|tab| (tab.search.current(), tab.search.matches.len()))
    }

    fn load(&mut self, path: &Path) {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_owned());
        if let Some(index) = self
            .tabs
            .iter()
            .position(|tab| tab.document.path == canonical)
        {
            self.active = index;
            self.error = None;
            return;
        }
        match Document::load(path) {
            Ok(document) => {
                self.tabs.push(DocumentTab {
                    id: self.tab_ids.fetch_add(1, Ordering::Relaxed),
                    document,
                    markdown_cache: CommonMarkCache::default(),
                    math: MathRenderer::default(),
                    scroll_offset: 0.0,
                    max_scroll_offset: 0.0,
                    search: crate::search::Search::default(),
                    heading_target: None,
                    fonts_checked: false,
                });
                self.active = self.tabs.len() - 1;
                self.error = None;
            }
            Err(error) => self.error = Some(error.to_string()),
        }
    }

    fn close_tab(&mut self, index: usize) {
        self.remove_tab(index);
    }

    fn remove_tab(&mut self, index: usize) -> DocumentTab {
        let tab = self.tabs.remove(index);
        if index < self.active {
            self.active -= 1;
        }
        self.active = self.active.min(self.tabs.len().saturating_sub(1));
        tab
    }

    fn open_dialog(&mut self) {
        if let Some(path) = self.services.choose_markdown_file() {
            self.load(&path);
        }
    }

    fn handle_drops(&mut self, context: &egui::Context) {
        let paths = context.input(|input| {
            input
                .raw
                .dropped_files
                .iter()
                .map(|file| file.path().to_owned())
                .collect::<Vec<_>>()
        });
        let mut errors = Vec::new();
        for path in paths {
            self.load(&path);
            if let Some(error) = self.error.take() {
                errors.push(error);
            }
        }
        if !errors.is_empty() {
            self.error = Some(errors.join("\n"));
        }
    }

    fn handle_link(&mut self, destination: &str) {
        let document_path = self.document_path();
        let result = match links::resolve(destination, document_path) {
            LinkAction::Browser(url) => self.services.open_browser(&url),
            LinkAction::Markdown(path) => self.services.open_markdown(&path),
            LinkAction::Inert => return,
        };
        if let Err(error) = result {
            self.error = Some(format!("Could not open link: {error}"));
        }
    }

    fn intercept_links(&mut self, context: &egui::Context) {
        let destinations = context.output_mut(|output| {
            let mut retained = Vec::with_capacity(output.commands.len());
            let mut destinations = Vec::new();
            for command in std::mem::take(&mut output.commands) {
                if let egui::OutputCommand::OpenUrl(open) = command {
                    destinations.push(open.url);
                } else {
                    retained.push(command);
                }
            }
            output.commands = retained;
            destinations
        });
        for destination in destinations {
            self.handle_link(&destination);
        }
    }

    fn landing_ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(((ui.available_height() - 360.0) * 0.35).max(16.0));
                ui.heading("FastMarkdownViewer");
                ui.label("Drop a Markdown file here, or open one.");
                ui.add_space(12.0);
                if ui.button("Open Markdown file…").clicked() {
                    self.open_dialog();
                }
                ui.add_space(8.0);
                ui.weak("Read-only · no history · no telemetry");
                ui.add_space(20.0);
                ui.strong("Keyboard shortcuts");
                for (shortcut, action) in [
                    ("Ctrl+O", "Open a Markdown file"),
                    ("Ctrl+F", "Show / hide Find"),
                    ("Ctrl+H", "Show / hide headings"),
                    ("Enter / Shift+Enter", "Next / previous match"),
                    ("Ctrl+Tab / Ctrl+Shift+Tab", "Next / previous tab"),
                    ("Ctrl+W", "Close tab"),
                    ("Ctrl+mouse wheel", "Zoom text"),
                ] {
                    ui.label(format!("{shortcut}   —   {action}"));
                }
                ui.add_space(8.0);
                ui.weak("Drag a tab outside the window to open it in a new window.");
            });
        });
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open…").clicked() {
                    self.open_dialog();
                    ui.close();
                }
                if ui
                    .add_enabled(
                        !self.tabs.is_empty(),
                        egui::Button::new("Close tab    Ctrl+W"),
                    )
                    .clicked()
                {
                    self.close_tab(self.active);
                    ui.close();
                }
                ui.separator();
                if ui.button("Close window").clicked() {
                    self.close_requested = true;
                }
            });
            if ui
                .add_enabled(!self.tabs.is_empty(), egui::Button::new("Find"))
                .on_hover_text("Show / hide Find (Ctrl+F)")
                .clicked()
            {
                self.open_find();
            }
            ui.menu_button("Settings", |ui| {
                ui.checkbox(&mut self.show_outline, "Outline sidebar    Ctrl+H");
                ui.separator();
                ui.label("Theme");
                let mut theme = *self.theme.lock().expect("appearance lock");
                let previous = theme;
                ui.radio_value(&mut theme, ThemeChoice::System, "Use Windows setting");
                ui.radio_value(&mut theme, ThemeChoice::Light, "Light");
                ui.radio_value(&mut theme, ThemeChoice::Dark, "Dark");
                if theme != previous {
                    *self.theme.lock().expect("appearance lock") = theme;
                    ui.ctx().set_theme(theme.preference());
                }

                ui.separator();
                ui.label(format!("Text size: {:.0}%", ui.ctx().zoom_factor() * 100.0));
                egui::gui_zoom::zoom_menu_buttons(ui);
                ui.weak("Theme and text size are shared by this session’s windows.");
            });
        });
        ui.separator();
    }

    fn open_find(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.active) {
            tab.search.open = true;
            self.focus_search = true;
        }
    }

    fn shortcuts(&mut self, context: &egui::Context) {
        if context.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::F)) {
            if let Some(tab) = self.tabs.get_mut(self.active)
                && tab.search.open
            {
                tab.search.open = false;
                context.memory_mut(|memory| {
                    if let Some(id) = memory.focused() {
                        memory.surrender_focus(id);
                    }
                });
            } else {
                self.open_find();
            }
        }
        if context.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::W))
            && !self.tabs.is_empty()
        {
            self.close_tab(self.active);
        }
        if context.input_mut(|input| {
            input.consume_key(egui::Modifiers::COMMAND, egui::Key::H)
                || input.consume_key(
                    egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
                    egui::Key::O,
                )
        }) {
            self.show_outline = !self.show_outline;
        }
        if !self.tabs.is_empty() {
            let backwards = context.input_mut(|input| {
                input.consume_key(
                    egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
                    egui::Key::Tab,
                )
            });
            let forwards =
                context.input_mut(|input| input.consume_key(egui::Modifiers::CTRL, egui::Key::Tab));
            if backwards || forwards {
                self.active = (self.active + if backwards { self.tabs.len() - 1 } else { 1 })
                    % self.tabs.len();
            }
        }
        if let Some(tab) = self.tabs.get_mut(self.active) {
            let previous = context.input_mut(|input| {
                input.consume_key(egui::Modifiers::SHIFT, egui::Key::F3)
                    || (tab.search.open
                        && input.consume_key(egui::Modifiers::SHIFT, egui::Key::Enter))
            });
            let next = context.input_mut(|input| {
                input.consume_key(egui::Modifiers::NONE, egui::Key::F3)
                    || (tab.search.open
                        && input.consume_key(egui::Modifiers::NONE, egui::Key::Enter))
            });
            if previous || next {
                tab.search.open = true;
                tab.search.advance(previous);
            }
        }
        if context.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            if self.tab_drag.take().is_some() {
                self.drag_cancelled = true;
                return;
            }
            if let Some(tab) = self.tabs.get_mut(self.active)
                && tab.search.open
            {
                tab.search.open = false;
                context.memory_mut(|memory| {
                    if let Some(id) = memory.focused() {
                        memory.surrender_focus(id);
                    }
                });
            } else {
                self.error = None;
            }
        }
    }

    fn tabs_ui(&mut self, ui: &mut egui::Ui) {
        if self.tabs.is_empty() {
            return;
        }
        let mut close = None;
        let mut detach = None;
        egui::ScrollArea::horizontal()
            .id_salt("tabs")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (index, tab) in self.tabs.iter().enumerate() {
                        ui.push_id(tab.id, |ui| {
                            let response = ui
                                .add(egui::Button::selectable(index == self.active, &tab.document.title)
                                    .sense(egui::Sense::click_and_drag()))
                                .on_hover_cursor(egui::CursorIcon::Grab)
                                .on_hover_text(format!("{}\nDrag outside this window to move the tab into a new window.", tab.document.path.display()));
                            if response.is_pointer_button_down_on()
                                && ui.input(|input| input.pointer.primary_down())
                                && self.tab_drag.is_none()
                                && !self.drag_cancelled
                                && let Some(origin) = ui.input(|input| input.pointer.press_origin())
                            {
                                self.tab_drag = Some(TabDrag { tab_id: tab.id, origin, latest: origin });
                            }
                            response.context_menu(|ui| {
                                if ui.button("Move to new window").clicked() {
                                    detach = Some(tab.id);
                                    ui.close();
                                }
                            });
                            if response.clicked() {
                                self.active = index;
                            }
                            if response.middle_clicked() {
                                close = Some(index);
                            }
                            if ui
                                .small_button("×")
                                .on_hover_text("Close tab (Ctrl+W)")
                                .clicked()
                            {
                                close = Some(index);
                            }
                            ui.separator();
                        });
                    }
                    if ui
                        .button("+")
                        .on_hover_text("Open file in a tab (Ctrl+O)")
                        .clicked()
                    {
                        self.open_dialog();
                    }
                });
            });
        if let Some(index) = close {
            self.close_tab(index);
            self.tab_drag = None;
        }
        if let Some(tab_id) = detach {
            self.request_detach(ui.ctx(), tab_id, None);
        }
        self.finish_tab_drag(ui.ctx());
        ui.separator();
    }

    fn finish_tab_drag(&mut self, context: &egui::Context) {
        if self.drag_cancelled {
            if !context.input(|input| input.pointer.primary_down()) {
                self.drag_cancelled = false;
            }
            return;
        }
        let Some(mut drag) = self.tab_drag else {
            return;
        };
        let (released, down, focused, bounds) = context.input(|input| {
            if let Some(position) = input.pointer.latest_pos() {
                drag.latest = position;
            }
            let viewport = input.viewport();
            let bounds = viewport
                .outer_rect
                .zip(viewport.inner_rect)
                .map_or(input.viewport_rect(), |(outer, inner)| {
                    outer.translate(-inner.min.to_vec2())
                });
            (
                input.pointer.button_released(egui::PointerButton::Primary),
                input.pointer.primary_down(),
                input.focused,
                bounds,
            )
        });
        self.tab_drag = Some(drag);
        if !focused {
            self.tab_drag = None;
            self.drag_cancelled = down;
        } else if released {
            self.tab_drag = None;
            // A normal click or a release over the title bar/document must not tear out a tab.
            if drag.latest.distance(drag.origin) >= 12.0
                && !bounds.expand(8.0).contains(drag.latest)
            {
                self.request_detach(context, drag.tab_id, Some(drag.latest));
            }
        } else if !down {
            self.tab_drag = None;
        } else if drag.latest.distance(drag.origin) >= 12.0 {
            context.set_cursor_icon(egui::CursorIcon::Grabbing);
        }
    }

    fn request_detach(
        &mut self,
        context: &egui::Context,
        tab_id: u64,
        pointer: Option<egui::Pos2>,
    ) {
        let (position, size) = context.input(|input| {
            let viewport = input.viewport();
            let position = viewport.inner_rect.map(|inner| {
                pointer.map_or(inner.min + egui::vec2(40.0, 40.0), |pointer| {
                    inner.min + pointer.to_vec2() - egui::vec2(80.0, 20.0)
                })
            });
            // Keep a detached maximized window manageable, including on another monitor.
            let size = input
                .viewport_rect()
                .size()
                .clamp(egui::vec2(420.0, 280.0), egui::vec2(1000.0, 800.0));
            (position, size)
        });
        self.detach_request = Some(DetachRequest {
            tab_id,
            position,
            size,
        });
    }

    fn find_ui(&mut self, ui: &mut egui::Ui) {
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return;
        };
        if !tab.search.open {
            return;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("Find:");
            let response = ui.add(
                egui::TextEdit::singleline(&mut tab.search.query)
                    .id_salt(("find", tab.id))
                    .hint_text("Find in document…")
                    .desired_width(180.0),
            );
            if self.focus_search {
                response.request_focus();
                self.focus_search = false;
            }
            ui.label(format!(
                "{} / {}",
                tab.search.current(),
                tab.search.matches.len()
            ));
            if ui
                .add_enabled(
                    !tab.search.matches.is_empty(),
                    egui::Button::new("Previous"),
                )
                .clicked()
            {
                tab.search.advance(true);
            }
            if ui
                .add_enabled(!tab.search.matches.is_empty(), egui::Button::new("Next"))
                .clicked()
            {
                tab.search.advance(false);
            }
            ui.checkbox(&mut tab.search.case_sensitive, "Match case");
            if ui
                .small_button("Close find")
                .on_hover_text("Escape")
                .clicked()
            {
                tab.search.open = false;
                response.surrender_focus();
            }
        });
        ui.separator();
    }

    fn reading_ui(&mut self, ui: &mut egui::Ui) {
        let tab = &mut self.tabs[self.active];
        if self.show_outline {
            egui::Panel::left("outline")
                .resizable(true)
                .default_size(190.0)
                .size_range(100.0..=320.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong("Outline");
                        if ui.small_button("Hide").clicked() {
                            self.show_outline = false;
                        }
                    });
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .id_salt(("outline", tab.id))
                        .show(ui, |ui| {
                            if tab.markdown_cache.navigation.headings.is_empty() {
                                ui.weak("No headings");
                            }
                            for (index, heading) in
                                tab.markdown_cache.navigation.headings.iter().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    ui.add_space(f32::from(heading.level.saturating_sub(1)) * 10.0);
                                    if ui
                                        .add(egui::Button::new(&heading.text).frame(false).wrap())
                                        .clicked()
                                    {
                                        tab.heading_target = Some(index);
                                    }
                                });
                            }
                        });
                });
        }
        tab.apply_keyboard_scroll(ui.ctx(), ui.available_height());
        // Keep the complete parser stream intact: upstream's virtualized path can split nested lists.
        let output = egui::ScrollArea::vertical()
            .id_salt(("document", tab.id))
            .vertical_scroll_offset(tab.scroll_offset)
            .auto_shrink([false, false])
            .show(ui, |ui| tab.document_ui(ui));
        tab.scroll_offset = output.state.offset.y;
        tab.max_scroll_offset = (output.content_size.y - output.inner_rect.height()).max(0.0);
    }

    fn apply_zoom_input(context: &egui::Context) {
        let wheel_points = context.input_mut(|input| {
            let mut wheel_points = 0.0;
            input.events.retain(|event| {
                let points = match event {
                    egui::Event::MouseWheel {
                        unit,
                        delta,
                        modifiers,
                        ..
                    } if modifiers.ctrl || modifiers.command => match unit {
                        egui::MouseWheelUnit::Point => delta.y,
                        egui::MouseWheelUnit::Line => delta.y * 24.0,
                        egui::MouseWheelUnit::Page => delta.y * 120.0,
                    },
                    _ => return true,
                };
                wheel_points += points;
                false
            });
            wheel_points
        });
        if wheel_points.abs() > f32::EPSILON {
            let zoom_delta = (wheel_points * 0.0015).exp();
            context.set_zoom_factor(
                (context.zoom_factor() * zoom_delta).clamp(MIN_ZOOM_FACTOR, MAX_ZOOM_FACTOR),
            );
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let context = ui.ctx().clone();
        Self::apply_zoom_input(&context);
        self.handle_drops(&context);
        self.shortcuts(&context);
        if context.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::O)) {
            self.open_dialog();
        }
        if let Some(tab) = self.tabs.get_mut(self.active)
            && !tab.fonts_checked
        {
            crate::fonts::ensure_for_text(&context, &tab.document.source);
            tab.fonts_checked = true;
        }

        if let Some(tab) = self.tabs.get(self.active) {
            context.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                "{} — FastMarkdownViewer",
                tab.document.title
            )));
        } else {
            context.send_viewport_cmd(egui::ViewportCommand::Title("FastMarkdownViewer".into()));
        }

        ui.painter()
            .rect_filled(ui.max_rect(), 0.0, ui.visuals().panel_fill);
        egui::Frame::central_panel(ui.style())
            .inner_margin(egui::Margin::symmetric(18, 12))
            .show(ui, |ui| {
                self.menu_bar(ui);
                self.tabs_ui(ui);
                self.find_ui(ui);
                if let Some(error) = self.error.clone() {
                    let mut dismiss = false;
                    egui::Frame::new()
                        .fill(ui.visuals().error_fg_color.gamma_multiply(0.08))
                        .corner_radius(5)
                        .inner_margin(8)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.colored_label(ui.visuals().error_fg_color, error);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        dismiss = ui.small_button("Dismiss").clicked();
                                    },
                                );
                            });
                        });
                    if dismiss {
                        self.error = None;
                    }
                    ui.add_space(6.0);
                }
                if self.tabs.is_empty() {
                    self.landing_ui(ui);
                } else {
                    self.reading_ui(ui);
                }
            });

        self.intercept_links(&context);
    }
}

impl DocumentTab {
    fn apply_keyboard_scroll(&mut self, context: &egui::Context, page_height: f32) {
        enum Command {
            Delta(f32),
            Start,
            End,
        }

        if context.egui_wants_keyboard_input() {
            return;
        }
        let command = context.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                Some(Command::Start)
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                Some(Command::End)
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::PageDown)
                || input.consume_key(egui::Modifiers::NONE, egui::Key::Space)
            {
                Some(Command::Delta(page_height * PAGE_SCROLL_FRACTION))
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::PageUp) {
                Some(Command::Delta(-page_height * PAGE_SCROLL_FRACTION))
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                Some(Command::Delta(LINE_SCROLL_POINTS))
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                Some(Command::Delta(-LINE_SCROLL_POINTS))
            } else {
                None
            }
        });

        match command {
            Some(Command::Delta(delta)) => {
                self.scroll_offset = (self.scroll_offset + delta).max(0.0);
            }
            Some(Command::Start) => self.scroll_offset = 0.0,
            Some(Command::End) => {
                self.scroll_offset = self.max_scroll_offset;
            }
            None => {}
        }
    }

    fn document_ui(&mut self, ui: &mut egui::Ui) {
        let document = &self.document;
        self.markdown_cache.navigation.capture_text = self.search.open;
        self.markdown_cache.navigation.clear();
        let math = self.math.clone();
        let render_math = move |ui: &mut egui::Ui, formula: &str, inline: bool| {
            math.show(ui, formula, inline);
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let max_image_width = ui.available_width().max(1.0) as usize;
        CommonMarkViewer::new()
            .default_implicit_uri_scheme(document.base_uri.clone())
            .max_image_width(Some(max_image_width))
            .show_alt_text_on_hover(true)
            .enable_scroll_to_heading(true)
            .render_math_fn(Some(&render_math))
            .show(ui, &mut self.markdown_cache, &document.source);
        let navigation = &mut self.markdown_cache.navigation;
        navigation.scroll_target = None;
        if let Some(index) = self.heading_target.take()
            && let Some(heading) = navigation.headings.get(index)
        {
            ui.scroll_to_rect(
                egui::Rect::from_min_size(heading.position, egui::vec2(1.0, 24.0)),
                Some(egui::Align::TOP),
            );
        }
        self.search.paint(ui, navigation);
    }
}
