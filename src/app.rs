use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
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

const LINE_SCROLL_POINTS: f32 = 48.0;
const PAGE_SCROLL_FRACTION: f32 = 0.9;

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

pub struct ViewerApp {
    document: Option<Document>,
    error: Option<String>,
    markdown_cache: CommonMarkCache,
    math: MathRenderer,
    services: Arc<dyn AppServices>,
    document_scroll_offset: f32,
    document_max_scroll_offset: f32,
}

impl ViewerApp {
    #[must_use]
    pub fn new(initial: InitialState) -> Self {
        Self::with_services(initial, Arc::new(PlatformServices))
    }

    #[must_use]
    pub fn with_services(initial: InitialState, services: Arc<dyn AppServices>) -> Self {
        let mut app = Self {
            document: None,
            error: None,
            markdown_cache: CommonMarkCache::default(),
            math: MathRenderer::default(),
            services,
            document_scroll_offset: 0.0,
            document_max_scroll_offset: 0.0,
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
        self.document
            .as_ref()
            .map(|document| document.path.as_path())
    }

    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }

    #[must_use]
    pub const fn document_scroll_offset(&self) -> f32 {
        self.document_scroll_offset
    }

    fn load(&mut self, path: &Path) {
        match Document::load(path) {
            Ok(document) => {
                self.document = Some(document);
                self.error = None;
                self.markdown_cache = CommonMarkCache::default();
                self.math = MathRenderer::default();
                self.document_scroll_offset = 0.0;
                self.document_max_scroll_offset = 0.0;
            }
            Err(error) => {
                self.document = None;
                self.error = Some(error.to_string());
                self.document_scroll_offset = 0.0;
                self.document_max_scroll_offset = 0.0;
            }
        }
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
        match paths.as_slice() {
            [] => {}
            [path] => self.load(path),
            _ => self.error = Some("Drop one Markdown file at a time".to_owned()),
        }
    }

    fn handle_link(&mut self, destination: &str) {
        let document_path = self
            .document
            .as_ref()
            .map(|document| document.path.as_path());
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

    fn apply_keyboard_scroll(&mut self, context: &egui::Context, page_height: f32) {
        enum Command {
            Delta(f32),
            Start,
            End,
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
                self.document_scroll_offset = (self.document_scroll_offset + delta).max(0.0);
            }
            Some(Command::Start) => self.document_scroll_offset = 0.0,
            Some(Command::End) => {
                self.document_scroll_offset = self.document_max_scroll_offset;
            }
            None => {}
        }
    }

    fn document_ui(&mut self, ui: &mut egui::Ui) {
        let Some(document) = &self.document else {
            return;
        };
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
    }

    fn landing_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space((ui.available_height() * 0.25).max(24.0));
            ui.heading("FastMarkdownViewer");
            ui.label("Drop a Markdown file here, or open one.");
            ui.add_space(12.0);
            if ui.button("Open Markdown file…").clicked() {
                self.open_dialog();
            }
            ui.add_space(8.0);
            ui.weak("Read-only · no history · no telemetry");
        });
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let context = ui.ctx().clone();
        self.handle_drops(&context);
        if context.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::O)) {
            self.open_dialog();
        }

        if let Some(document) = &self.document {
            context.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                "{} — FastMarkdownViewer",
                document.title
            )));
        }

        egui::Frame::central_panel(ui.style())
            .inner_margin(egui::Margin::symmetric(18, 12))
            .show(ui, |ui| {
                if let Some(error) = &self.error {
                    egui::Frame::new()
                        .fill(ui.visuals().error_fg_color.gamma_multiply(0.08))
                        .corner_radius(5)
                        .inner_margin(8)
                        .show(ui, |ui| {
                            ui.colored_label(ui.visuals().error_fg_color, error);
                        });
                    ui.add_space(6.0);
                }
                if self.document.is_some() {
                    self.apply_keyboard_scroll(ui.ctx(), ui.available_height());

                    // egui_commonmark's hidden `show_scrollable` API can split its
                    // parser event stream inside a nested list and panic. Keep the
                    // stable renderer inside egui's scroll area until that upstream
                    // path can preserve parser state across page boundaries.
                    let output = egui::ScrollArea::vertical()
                        .id_salt("document_scroll_area")
                        .vertical_scroll_offset(self.document_scroll_offset)
                        .auto_shrink([false, false])
                        .show(ui, |ui| self.document_ui(ui));
                    self.document_scroll_offset = output.state.offset.y;
                    self.document_max_scroll_offset =
                        (output.content_size.y - output.inner_rect.height()).max(0.0);
                } else {
                    self.landing_ui(ui);
                }
            });

        self.intercept_links(&context);
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}
