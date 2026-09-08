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

struct Request {
    key: u64,
    text: String,
    language: String,
    dark: bool,
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
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        (text, &language, dark, &font).hash(&mut hash);
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
    let theme = &themes.themes[if request.dark {
        "base16-ocean.dark"
    } else {
        "base16-ocean.light"
    }];
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
                    let c = style.foreground;
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
