use eframe::egui;
use std::{
    collections::HashMap,
    path::Path,
    process::{Command, Stdio},
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

static BUSY: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct Appearance {
    pub background: [u8; 3],
    pub surface: [u8; 3],
    pub text: [u8; 3],
    pub muted: [u8; 3],
    pub accent: [u8; 3],
    pub selection: [u8; 3],
    pub font: Option<String>,
}

impl Appearance {
    fn from_visuals(visuals: &egui::Visuals, font: Option<String>) -> Self {
        Self {
            background: visuals.panel_fill.to_array()[..3].try_into().unwrap(),
            surface: visuals.window_fill.to_array()[..3].try_into().unwrap(),
            text: visuals.text_color().to_array()[..3].try_into().unwrap(),
            muted: visuals.weak_text_color().to_array()[..3]
                .try_into()
                .unwrap(),
            accent: visuals.hyperlink_color.to_array()[..3].try_into().unwrap(),
            selection: visuals.selection.bg_fill.to_array()[..3]
                .try_into()
                .unwrap(),
            font,
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::from_visuals(&egui::Visuals::light(), None)
    }
}

use crate::mermaid_tiles::{Diagram, Geometry, RasterRequest};

#[derive(Clone)]
enum Entry {
    Pending {
        cancel: Arc<AtomicBool>,
        diagram: Option<Arc<Diagram>>,
    },
    Ready(Arc<Diagram>),
    Failed(String),
}
#[derive(Default)]
struct State {
    entries: HashMap<String, Entry>,
    appearance: Option<Appearance>,
    generation: u64,
}
impl State {
    fn set_appearance(&mut self, appearance: &Appearance) {
        if self.appearance.as_ref() != Some(appearance) {
            self.entries.clear();
            self.generation += 1;
            self.appearance = Some(appearance.clone());
        }
    }
}
#[derive(Clone, Default)]
pub struct MermaidRenderer {
    state: Arc<Mutex<State>>,
}

impl MermaidRenderer {
    pub fn show(&self, ui: &mut egui::Ui, source: &str) {
        let appearance =
            Appearance::from_visuals(ui.visuals(), crate::fonts::diagram_font(ui.ctx()));
        let (entry, generation) = {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state.set_appearance(&appearance);
            (state.entries.get(source).cloned(), state.generation)
        };
        match entry {
            Some(Entry::Ready(diagram)) => diagram.show(ui),
            Some(Entry::Pending { cancel, diagram }) => {
                ui.horizontal(|ui| {
                    ui.weak("Rendering Mermaid diagram…");
                    if ui.small_button("Cancel diagram").clicked() {
                        cancel.store(true, Ordering::Release);
                    }
                });
                if let Some(diagram) = diagram {
                    diagram.show(ui);
                }
            }
            Some(Entry::Failed(error)) => {
                ui.horizontal(|ui| {
                    ui.weak(format!("Mermaid unavailable: {error}. Source shown below."));
                    if ui.small_button("Retry diagram").clicked() {
                        self.state
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .entries
                            .remove(source);
                        ui.ctx().request_repaint();
                    }
                });
            }
            None => {
                let response = ui.weak("Mermaid diagram will load when visible…");
                if !ui.is_rect_visible(response.rect) {
                    return;
                }
                if BUSY
                    .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    self.start(ui, source, appearance, generation);
                }
                ui.ctx().request_repaint_after(Duration::from_millis(100));
            }
        }
    }

    fn start(&self, ui: &egui::Ui, source: &str, appearance: Appearance, generation: u64) {
        let cancel = Arc::new(AtomicBool::new(false));
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .entries
            .insert(
                source.to_owned(),
                Entry::Pending {
                    cancel: cancel.clone(),
                    diagram: None,
                },
            );
        let job = Job {
            source: source.to_owned(),
            state: Arc::downgrade(&self.state),
            generation,
            cancel,
            context: ui.ctx().clone(),
            viewport: ui.ctx().viewport_id(),
        };
        let request = RasterRequest {
            max_width: ui.available_width().max(1.0),
            pixel_scale: ui.pixels_per_point(),
        };
        let result = std::thread::Builder::new()
            .name("Mermaid renderer".into())
            .spawn(move || {
                let _busy = Busy;
                let result = render_in_helper(&job, &appearance, request);
                job.update(result.map_or_else(Entry::Failed, Entry::Ready));
            });
        if let Err(error) = result {
            BUSY.store(false, Ordering::Release);
            self.state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .entries
                .insert(source.to_owned(), Entry::Failed(error.to_string()));
        }
    }
}

struct Busy;
impl Drop for Busy {
    fn drop(&mut self) {
        BUSY.store(false, Ordering::Release);
    }
}
struct Job {
    source: String,
    state: Weak<Mutex<State>>,
    generation: u64,
    cancel: Arc<AtomicBool>,
    context: egui::Context,
    viewport: egui::ViewportId,
}
impl Job {
    fn cancelled(&self) -> bool {
        self.cancel.load(Ordering::Acquire)
            || self.state.upgrade().is_none_or(|state| {
                state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .generation
                    != self.generation
            })
    }
    fn update(&self, entry: Entry) {
        if let Some(state) = self.state.upgrade() {
            let mut state = state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.generation == self.generation {
                state.entries.insert(self.source.clone(), entry);
            }
        }
        self.context.request_repaint_of(self.viewport);
    }
}

fn render_in_helper(
    job: &Job,
    appearance: &Appearance,
    request: RasterRequest,
) -> Result<Arc<Diagram>, String> {
    let mut directory = Some(tempfile::tempdir().map_err(|e| e.to_string())?);
    let input = directory.as_ref().unwrap().path().join("diagram.mmd");
    let output = directory.as_ref().unwrap().path().join("diagram.json");
    std::fs::write(&input, &job.source).map_err(|e| e.to_string())?;
    let mut command = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
    command
        .arg("--internal-mermaid")
        .arg(&input)
        .arg(&output)
        .arg(serde_json::to_string(appearance).map_err(|e| e.to_string())?)
        .arg(serde_json::to_string(&request).map_err(|e| e.to_string())?)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    let mut child = command.spawn().map_err(|e| e.to_string())?;
    let mut diagram = None;
    let result = loop {
        if job.cancelled() {
            break Err("Rendering cancelled".into());
        }
        if diagram.is_none() && output.is_file() {
            match read_geometry(&output) {
                Ok(geometry) => {
                    let rendered = Arc::new(Diagram {
                        directory: directory.take(),
                        geometry,
                        context: job.context.clone(),
                        used_uris: Mutex::default(),
                    });
                    job.update(Entry::Pending {
                        cancel: job.cancel.clone(),
                        diagram: Some(rendered.clone()),
                    });
                    diagram = Some(rendered);
                }
                Err(error) => break Err(error),
            }
        }
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                // A very small diagram can finish between the manifest check and try_wait.
                break diagram.map_or_else(
                    || {
                        read_geometry(&output).map(|geometry| {
                            Arc::new(Diagram {
                                directory: directory.take(),
                                geometry,
                                context: job.context.clone(),
                                used_uris: Mutex::default(),
                            })
                        })
                    },
                    Ok,
                );
            }
            Ok(Some(_)) => {
                break Err(std::fs::read_to_string(output.with_extension("error"))
                    .unwrap_or_else(|_| "Renderer stopped unexpectedly".into()));
            }
            Err(error) => break Err(error.to_string()),
            Ok(None) => {
                if diagram.is_some() {
                    job.context.request_repaint_of(job.viewport);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    };
    // On cancellation or IPC errors, never leave an orphaned renderer running.
    if result.is_err() {
        let _ = child.kill();
    }
    let _ = child.wait();
    result
}

fn read_geometry(output: &Path) -> Result<Geometry, String> {
    let bytes = std::fs::read(output).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

/// Dispatch private rendering before creating any windows.
pub fn handle_helper_args() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.get(1).is_none_or(|arg| arg != "--internal-mermaid") {
        return;
    }
    if !(4..=6).contains(&args.len()) {
        std::process::exit(2);
    }
    let input = Path::new(&args[2]);
    let output = Path::new(&args[3]);
    let result = (|| {
        let appearance = args.get(4).map_or_else(
            || Ok(Appearance::default()),
            |json| serde_json::from_str(&json.to_string_lossy()).map_err(|e| e.to_string()),
        )?;
        if let Some(request) = args.get(5) {
            let request =
                serde_json::from_str(&request.to_string_lossy()).map_err(|e| e.to_string())?;
            crate::mermaid_worker::render_tiles(input, output, &appearance, request)
        } else {
            crate::mermaid_worker::render(input, output, &appearance)
        }
    })();
    match result {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            let _ = std::fs::write(output.with_extension("error"), error);
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn appearance_changes_invalidate_work_and_close_cancels_it() {
        let state = Arc::new(Mutex::new(State::default()));
        let mut appearance = Appearance::default();
        state.lock().unwrap().set_appearance(&appearance);
        let job = Job {
            source: "diagram".into(),
            state: Arc::downgrade(&state),
            generation: 1,
            cancel: Arc::default(),
            context: egui::Context::default(),
            viewport: egui::ViewportId::ROOT,
        };
        job.update(Entry::Pending {
            cancel: job.cancel.clone(),
            diagram: None,
        });
        state.lock().unwrap().set_appearance(&appearance);
        assert!(!job.cancelled());
        assert_eq!(state.lock().unwrap().entries.len(), 1);
        appearance.background = [39, 40, 34];
        state.lock().unwrap().set_appearance(&appearance);
        assert!(job.cancelled());
        job.update(Entry::Failed("stale result".into()));
        assert!(state.lock().unwrap().entries.is_empty());
        drop(state);
        assert!(job.cancelled());
    }

    #[test]
    fn offscreen_diagrams_do_not_start_rendering() {
        let context = egui::Context::default();
        let renderer = MermaidRenderer::default();
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            ui.add_space(100_000.0);
            renderer.show(ui, "flowchart LR\nA --> B");
        });
        output.textures_delta.clear();
        assert!(renderer.state.lock().unwrap().entries.is_empty());
    }
}
