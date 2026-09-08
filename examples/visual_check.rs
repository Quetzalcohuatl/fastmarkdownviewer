//! Capture the real renderer framebuffer for manual visual regression checks.
//! `cargo run --example visual_check -- input.md output.png [search text]`
//! Use `-` as the input path to capture the empty-window shortcut page.
use eframe::egui;
use fast_markdown_viewer::{
    app::{InitialState, ViewerApp},
    fonts, network,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

struct Capture {
    viewer: ViewerApp,
    output: PathBuf,
    start: Instant,
    requested: bool,
    query: Option<String>,
    find_opened: bool,
}

impl eframe::App for Capture {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let screenshot = ui.input(|input| {
            input.events.iter().find_map(|event| {
                if let egui::Event::Screenshot { image, .. } = event {
                    Some(image.clone())
                } else {
                    None
                }
            })
        });
        if let Some(screenshot) = screenshot {
            let bytes: Vec<_> = screenshot
                .pixels
                .iter()
                .flat_map(egui::Color32::to_array)
                .collect();
            image::save_buffer(
                &self.output,
                &bytes,
                u32::try_from(screenshot.width()).unwrap(),
                u32::try_from(screenshot.height()).unwrap(),
                image::ColorType::Rgba8,
            )
            .unwrap();
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if self.start.elapsed() > Duration::from_millis(300) && self.query.is_some() {
            if self.find_opened {
                ui.input_mut(|input| {
                    input
                        .events
                        .push(egui::Event::Text(self.query.take().unwrap()));
                });
            } else {
                ui.input_mut(|input| {
                    input.events.push(egui::Event::Key {
                        key: egui::Key::F,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::COMMAND,
                    });
                });
                self.find_opened = true;
            }
        }
        self.viewer.show(ui);
        if !self.requested && self.start.elapsed() > Duration::from_secs(2) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            self.requested = true;
        }
        assert!(
            self.start.elapsed() < Duration::from_secs(15),
            "framebuffer capture timed out"
        );
        ui.ctx().request_repaint_after(Duration::from_millis(16));
    }
}

fn main() -> eframe::Result {
    let mut arguments = std::env::args_os().skip(1);
    let document = PathBuf::from(arguments.next().expect("input Markdown path"));
    let output = PathBuf::from(arguments.next().expect("output PNG path"));
    let query = arguments
        .next()
        .map(|value| value.to_string_lossy().into_owned());
    eframe::run_native(
        "Viewer visual check",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
            renderer: selected_renderer(),
            ..Default::default()
        },
        Box::new(move |creation| {
            fonts::install(&creation.egui_ctx);
            network::install(&creation.egui_ctx);
            Ok(Box::new(Capture {
                viewer: ViewerApp::new(if document.as_os_str() == "-" {
                    InitialState::Empty
                } else {
                    InitialState::Path(document)
                }),
                output,
                start: Instant::now(),
                requested: false,
                query,
                find_opened: false,
            }))
        }),
    )
}

#[cfg(feature = "renderer-glow")]
const fn selected_renderer() -> eframe::Renderer {
    eframe::Renderer::Glow
}
#[cfg(all(feature = "renderer-wgpu", not(feature = "renderer-glow")))]
const fn selected_renderer() -> eframe::Renderer {
    eframe::Renderer::Wgpu
}
