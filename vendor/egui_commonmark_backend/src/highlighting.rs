use egui::{FontId, Pos2, TextFormat, TextStyle, Ui, text::LayoutJob};
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    sync::{OnceLock, mpsc},
};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

const MAX_BLOCK_BYTES: usize = 256 * 1024;
const MAX_CACHE_ENTRIES: usize = 128;
const MAX_CACHE_BYTES: usize = 8 * 1024 * 1024;

/// Select a built-in syntax palette. Unknown names fall back to the light/dark default.
pub fn set_syntax_theme(context: &egui::Context, name: &str) {
    context.data_mut(|data| data.insert_temp(egui::Id::new("markdown_syntax_theme"), name.to_owned()));
    context.request_repaint();
}

struct Request {
    key: u64,
    text: String,
    language: String,
    dark: bool,
    theme: String,
    font: FontId,
    context: egui::Context,
}
struct Worker {
    sender: mpsc::SyncSender<Request>,
    receiver: mpsc::Receiver<(u64, LayoutJob)>,
}

#[derive(Default)]
pub(crate) struct HighlightCache {
    worker: Option<Worker>,
    pending: HashSet<u64>,
    jobs: HashMap<u64, LayoutJob>,
    cached_bytes: usize,
}

impl std::fmt::Debug for HighlightCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HighlightCache")
            .field("cached", &self.jobs.len())
            .field("worker_started", &self.worker.is_some())
            .finish()
    }
}

impl HighlightCache {
    pub fn layout(&mut self, ui: &Ui, position: Pos2, language: &str, text: &str) -> LayoutJob {
        let font = TextStyle::Monospace.resolve(ui.style());
        let plain = || {
            LayoutJob::simple(
                text.to_owned(),
                font.clone(),
                ui.visuals().text_color(),
                f32::INFINITY,
            )
        };
        let language = language
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if language.is_empty() || text.len() > MAX_BLOCK_BYTES {
            return plain();
        }
        let dark = ui.visuals().dark_mode;
        let theme = ui.ctx().data(|data| data.get_temp::<String>(egui::Id::new("markdown_syntax_theme"))).unwrap_or_default();
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        (text, &language, dark, &theme, &font).hash(&mut hash);
        let key = hash.finish();
        if let Some(worker) = &self.worker {
            while let Ok((key, job)) = worker.receiver.try_recv() {
                self.pending.remove(&key);
                let bytes = job.text.len()
                    + job.sections.len() * std::mem::size_of::<egui::text::LayoutSection>();
                if self.jobs.len() >= MAX_CACHE_ENTRIES
                    || self.cached_bytes + bytes > MAX_CACHE_BYTES
                {
                    self.jobs.clear();
                    self.cached_bytes = 0;
                }
                self.cached_bytes += bytes;
                self.jobs.insert(key, job);
            }
        }
        if let Some(job) = self.jobs.get(&key) {
            return job.clone();
        }
        // Plain layout establishes the exact bounds before scheduling any syntax work.
        let job = plain();
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job.clone()));
        if !ui.is_rect_visible(galley.rect.translate(position.to_vec2())) {
            return job;
        }
        if self.worker.is_none() {
            self.worker = Worker::start();
        }
        if !self.pending.contains(&key) {
            if let Some(worker) = &self.worker {
                let request = Request {
                    key,
                    text: text.to_owned(),
                    language,
                    dark,
                    theme,
                    font,
                    context: ui.ctx().clone(),
                };
                if worker.sender.try_send(request).is_ok() {
                    self.pending.insert(key);
                } else {
                    ui.ctx()
                        .request_repaint_after(std::time::Duration::from_millis(50));
                }
            }
        }
        job
    }
}

impl Worker {
    fn start() -> Option<Self> {
        let (sender, requests) = mpsc::sync_channel::<Request>(4);
        let (results, receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("markdown-syntax".into())
            .spawn(move || {
                // Shared across tabs, initialized on this worker only on the first visible code request.
                static SYNTAX: OnceLock<SyntaxSet> = OnceLock::new();
                static THEMES: OnceLock<ThemeSet> = OnceLock::new();
                while let Ok(request) = requests.recv() {
                    let syntax = SYNTAX.get_or_init(SyntaxSet::load_defaults_newlines);
                    let themes = THEMES.get_or_init(ThemeSet::load_defaults);
                    let job = highlight(syntax, themes, &request);
                    if results.send((request.key, job)).is_err() {
                        break;
                    }
                    request.context.request_repaint();
                }
            })
            .ok()?;
        Some(Self { sender, receiver })
    }
}

fn highlight(syntax: &SyntaxSet, themes: &ThemeSet, request: &Request) -> LayoutJob {
    let language = match request.language.as_str() {
        "js" => "javascript",
        "ts" | "typescript" => "javascript",
        "sh" | "shell" | "shellscript" => "bash",
        "yml" => "yaml",
        other => other,
    };
    let definition = syntax
        .find_syntax_by_token(language)
        .unwrap_or_else(|| syntax.find_syntax_plain_text());
    let fallback = if request.dark { "base16-ocean.dark" } else { "base16-ocean.light" };
    let theme = themes.themes.get(&request.theme).unwrap_or(&themes.themes[fallback]);
    let mut highlighter = HighlightLines::new(definition, theme);
    let mut job = LayoutJob::default();
    for line in LinesWithEndings::from(&request.text) {
        if job.sections.len() > 16_384 {
            return LayoutJob::simple(
                request.text.clone(),
                request.font.clone(),
                if request.dark {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                },
                f32::INFINITY,
            );
        }
        match highlighter.highlight_line(line, syntax) {
            Ok(spans) => {
                for (style, text) in spans {
                    let c = palette_color(style.foreground, &request.theme);
                    job.append(
                        text,
                        0.0,
                        TextFormat::simple(
                            request.font.clone(),
                            egui::Color32::from_rgb(c.r, c.g, c.b),
                        ),
                    );
                }
            }
            Err(_) => {
                let c = theme
                    .settings
                    .foreground
                    .unwrap_or(syntect::highlighting::Color::BLACK);
                job.append(
                    line,
                    0.0,
                    TextFormat::simple(
                        request.font.clone(),
                        egui::Color32::from_rgb(c.r, c.g, c.b),
                    ),
                );
            }
        }
    }
    if job.text.len() + job.sections.len() * std::mem::size_of::<egui::text::LayoutSection>()
        > MAX_CACHE_BYTES
    {
        return LayoutJob::simple(
            request.text.clone(),
            request.font.clone(),
            if request.dark {
                egui::Color32::WHITE
            } else {
                egui::Color32::BLACK
            },
            f32::INFINITY,
        );
    }
    job
}

// Map the bundled Ocean syntax categories into the familiar editor palettes.
// Other built-ins (including both Solarized variants) use syntect's own colors.
fn palette_color(mut color: syntect::highlighting::Color, theme: &str) -> syntect::highlighting::Color {
    let source = [0xc0c5ce, 0x65737e, 0xa7adba, 0xbf616a, 0xd08770, 0xebcb8b, 0xa3be8c, 0x96b5b4, 0x8fa1b3, 0xb48ead, 0xab7967];
    let target = match theme {
        "Monokai" => [0xf8f8f2, 0xa6a68d, 0xf8f8f2, 0xf92672, 0xae81ff, 0xe6db74, 0xa6e22e, 0x66d9ef, 0x66d9ef, 0xf92672, 0xfd971f],
        "Tomorrow Night Blue" => [0xffffff, 0x8fa9cc, 0xffffff, 0xff9da4, 0xffc58f, 0xffeead, 0xd1f1a9, 0x99ffff, 0xbbdaff, 0xebbbff, 0xffc58f],
        _ => return color,
    };
    let value = u32::from_be_bytes([0, color.r, color.g, color.b]);
    if let Some(index) = source.iter().position(|candidate| *candidate == value) {
        let [_, r, g, b] = u32::to_be_bytes(target[index]);
        color.r = r;
        color.g = g;
        color.b = b;
    }
    color
}
