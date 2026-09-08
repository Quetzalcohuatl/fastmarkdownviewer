//! CPU-side scrolling and tab lifecycle benchmark; no GPU/present-time claims.
//! The parent runner samples process memory at each PHASE line, then writes a newline.
use eframe::egui;
use fast_markdown_viewer::{
    app::{InitialState, ViewerApp},
    fonts, network,
};
use std::{
    io::{self, Write as _},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
#[derive(Debug)]
struct DropFile(PathBuf);
impl egui::DroppedFile for DropFile {
    fn path(&self) -> &Path {
        &self.0
    }
    fn bytes(&self) -> Result<Vec<u8>, String> {
        std::fs::read(&self.0).map_err(|error| error.to_string())
    }
}
fn frame(
    context: &egui::Context,
    viewer: &mut ViewerApp,
    events: Vec<egui::Event>,
    files: Vec<Arc<dyn egui::DroppedFile + Send + Sync>>,
) {
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 700.0),
            )),
            events,
            dropped_files: files,
            ..Default::default()
        },
        |ui| viewer.show(ui),
    );
    output.textures_delta.clear();
}
fn phase(name: &str) {
    println!("PHASE {name}");
    io::stdout().flush().unwrap();
    let mut ack = String::new();
    io::stdin().read_line(&mut ack).unwrap();
}
fn main() {
    let path = PathBuf::from(std::env::args_os().nth(1).expect("fixture path"));
    let context = egui::Context::default();
    fonts::install(&context);
    network::install(&context);
    let mut viewer = ViewerApp::new(InitialState::Path(path.clone()));
    for _ in 0..5 {
        frame(&context, &mut viewer, vec![], vec![]);
        std::thread::sleep(Duration::from_millis(10));
    }
    phase("one_tab");
    let mut samples = Vec::new();
    for _ in 0..30 {
        let start = Instant::now();
        frame(
            &context,
            &mut viewer,
            vec![
                egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, -300.0),
                    modifiers: egui::Modifiers::NONE,
                    phase: egui::TouchPhase::Move,
                },
                egui::Event::PointerMoved(egui::pos2(550.0, 350.0)),
            ],
            vec![],
        );
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
        std::thread::sleep(Duration::from_millis(10));
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "SCROLL {},{},{}",
        samples[15],
        samples[28],
        viewer.document_scroll_offset()
    );
    let copies = tempfile::tempdir().unwrap();
    let mut files: Vec<Arc<dyn egui::DroppedFile + Send + Sync>> = Vec::new();
    for index in 0..4 {
        let copy = copies.path().join(format!("tab-{index}.md"));
        std::fs::copy(&path, &copy).unwrap();
        files.push(Arc::new(DropFile(copy)));
    }
    frame(&context, &mut viewer, vec![], files);
    for _ in 0..5 {
        frame(&context, &mut viewer, vec![], vec![]);
        std::thread::sleep(Duration::from_millis(10));
    }
    phase("five_tabs");
    for _ in 0..5 {
        frame(
            &context,
            &mut viewer,
            vec![egui::Event::Key {
                key: egui::Key::W,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            }],
            vec![],
        );
    }
    for _ in 0..5 {
        frame(&context, &mut viewer, vec![], vec![]);
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(viewer.tab_count(), 0);
    phase("closed_tabs");
}
