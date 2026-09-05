use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, mpsc},
    thread,
};

use eframe::egui;
use ratex_layout::{LayoutOptions, layout, to_display_list};
use ratex_svg::{SvgOptions, render_to_svg};
use ratex_types::{color::Color, math_style::MathStyle};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct MathKey {
    formula: String,
    inline: bool,
    pixels_per_point_bits: u32,
    color: [u8; 4],
}

#[derive(Clone)]
enum CacheEntry {
    Pending,
    Ready(Arc<[u8]>),
    Failed,
}

struct Job {
    key: MathKey,
    context: egui::Context,
}

#[derive(Clone, Default)]
pub struct MathRenderer {
    cache: Arc<Mutex<HashMap<MathKey, CacheEntry>>>,
    sender: Arc<Mutex<Option<mpsc::Sender<Job>>>>,
}

impl MathRenderer {
    pub fn show(&self, ui: &mut egui::Ui, formula: &str, inline: bool) {
        let text_color = ui.visuals().text_color();
        let pixels_per_point = ui.pixels_per_point();
        let key = MathKey {
            formula: formula.to_owned(),
            inline,
            pixels_per_point_bits: pixels_per_point.to_bits(),
            color: text_color.to_array(),
        };

        let entry = lock(&self.cache).get(&key).cloned();
        match entry {
            Some(CacheEntry::Ready(svg)) => {
                let uri = format!(
                    "bytes://fast-markdown-viewer/math-{:x}.svg",
                    stable_hash(&key)
                );
                ui.add(
                    egui::Image::new(egui::ImageSource::Bytes {
                        uri: uri.into(),
                        bytes: egui::load::Bytes::Shared(svg),
                    })
                    .fit_to_original_size(1.0 / pixels_per_point)
                    .max_width(ui.available_width())
                    .alt_text(formula),
                );
            }
            Some(CacheEntry::Pending | CacheEntry::Failed) => placeholder(ui, formula, inline),
            None => {
                lock(&self.cache).insert(key.clone(), CacheEntry::Pending);
                self.queue(Job {
                    key,
                    context: ui.ctx().clone(),
                });
                placeholder(ui, formula, inline);
            }
        }
    }

    fn queue(&self, job: Job) {
        let mut sender = lock(&self.sender);
        let sender = sender.get_or_insert_with(|| {
            let (tx, rx) = mpsc::channel::<Job>();
            let cache = Arc::clone(&self.cache);
            thread::Builder::new()
                .name("FastMarkdownViewer math".to_owned())
                .spawn(move || worker(rx, cache))
                .expect("failed to start the math renderer");
            tx
        });
        if sender.send(job).is_err() {
            lock(&self.cache).clear();
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // Both values are moved into and owned by the worker.
fn worker(receiver: mpsc::Receiver<Job>, cache: Arc<Mutex<HashMap<MathKey, CacheEntry>>>) {
    while let Ok(job) = receiver.recv() {
        let result = render_formula_svg(
            &job.key.formula,
            job.key.inline,
            f64::from(f32::from_bits(job.key.pixels_per_point_bits)),
            job.key.color,
        );
        lock(&cache).insert(
            job.key,
            result.map_or(CacheEntry::Failed, |svg| CacheEntry::Ready(svg.into())),
        );
        job.context.request_repaint();
    }
}

fn placeholder(ui: &mut egui::Ui, formula: &str, inline: bool) {
    let source = if inline {
        format!("${formula}$")
    } else {
        format!("$$ {formula} $$")
    };
    ui.label(egui::RichText::new(source).monospace().weak());
}

fn stable_hash(key: &MathKey) -> u64 {
    use std::hash::{Hash as _, Hasher as _};
    let mut state = std::hash::DefaultHasher::new();
    key.hash(&mut state);
    state.finish()
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Render a TeX formula to a self-contained SVG.
///
/// # Errors
///
/// Returns an error when `RaTeX` cannot parse the formula.
pub fn render_formula_svg(
    formula: &str,
    inline: bool,
    pixels_per_point: f64,
    rgba: [u8; 4],
) -> Result<Vec<u8>, String> {
    let nodes = ratex_parser::parse(formula).map_err(|error| error.to_string())?;
    let style = if inline {
        MathStyle::Text
    } else {
        MathStyle::Display
    };
    let color = Color::new(
        f32::from(rgba[0]) / 255.0,
        f32::from(rgba[1]) / 255.0,
        f32::from(rgba[2]) / 255.0,
        f32::from(rgba[3]) / 255.0,
    );
    let options = LayoutOptions::default().with_style(style).with_color(color);
    let display = to_display_list(&layout(&nodes, &options));
    let svg = render_to_svg(
        &display,
        &SvgOptions {
            font_size: 18.0 * pixels_per_point,
            padding: 2.0 * pixels_per_point,
            stroke_width: pixels_per_point.max(1.0),
            embed_glyphs: true,
            font_dir: String::new(),
        },
    );
    Ok(svg.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_inline_and_display_equations() {
        for (formula, inline) in [
            (r"x^2 + y_1", true),
            (r"\frac{a}{b}", false),
            (r"\sqrt{x}", false),
            (r"\begin{matrix}a&b\\c&d\end{matrix}", false),
        ] {
            let svg = render_formula_svg(formula, inline, 1.0, [20, 30, 40, 255]).unwrap();
            assert!(svg.starts_with(b"<svg"));
        }
    }

    #[test]
    fn malformed_equation_falls_back() {
        assert!(render_formula_svg(r"\frac{", false, 1.0, [0, 0, 0, 255]).is_err());
    }
}
