#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui;
use fast_markdown_viewer::{
    app::{InitialState, ViewerApp},
    cli::{self, Command},
    fonts, network,
};

fn main() -> eframe::Result {
    let command = cli::parse(std::env::args_os());
    if command == Command::Version {
        println!("FastMarkdownViewer {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let initial = match command {
        Command::Open(Some(path)) => InitialState::Path(path),
        Command::Open(None) => InitialState::Empty,
        Command::Error(error) => InitialState::Error(error),
        Command::Version => unreachable!(),
    };
    let title = match &initial {
        InitialState::Path(path) => path.file_name().map_or_else(
            || "FastMarkdownViewer".to_owned(),
            |name| format!("{} — FastMarkdownViewer", name.to_string_lossy()),
        ),
        InitialState::Empty | InitialState::Error(_) => "FastMarkdownViewer".to_owned(),
    };
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_app_id("FastMarkdownViewer")
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([420.0, 280.0])
            .with_icon(app_icon()),
        renderer: selected_renderer(),
        persist_window: false,
        persistence_path: None,
        ..Default::default()
    };

    eframe::run_native(
        "FastMarkdownViewer",
        native_options,
        Box::new(move |creation_context| {
            network::install(&creation_context.egui_ctx);
            fonts::install(&creation_context.egui_ctx);
            creation_context
                .egui_ctx
                .set_theme(egui::ThemePreference::System);
            creation_context.egui_ctx.all_styles_mut(|style| {
                style.spacing.item_spacing.y = 7.0;
                style.visuals.selection.stroke.width = 1.0;
            });
            Ok(Box::new(ViewerApp::new(initial)))
        }),
    )
}

#[cfg(all(feature = "renderer-glow", feature = "renderer-wgpu"))]
compile_error!("select only one renderer feature");

#[cfg(not(any(feature = "renderer-glow", feature = "renderer-wgpu")))]
compile_error!("enable renderer-glow or renderer-wgpu");

#[cfg(feature = "renderer-glow")]
const fn selected_renderer() -> eframe::Renderer {
    eframe::Renderer::Glow
}

#[cfg(all(feature = "renderer-wgpu", not(feature = "renderer-glow")))]
const fn selected_renderer() -> eframe::Renderer {
    eframe::Renderer::Wgpu
}

fn app_icon() -> egui::IconData {
    const SIZE: usize = 32;
    const SIZE_U32: u32 = 32;
    let mut rgba = Vec::with_capacity(SIZE * SIZE * 4);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let page = (5..27).contains(&x) && (3..29).contains(&y);
            let fold = x >= 21 && y < 9 && x - 21 >= 8 - y;
            let line = (9..24).contains(&x) && matches!(y, 12 | 17 | 22) && (!fold || y >= 12);
            let color = if line {
                [31, 41, 55, 255]
            } else if page && !fold {
                [243, 244, 246, 255]
            } else if page {
                [145, 164, 188, 255]
            } else {
                [30, 111, 214, 255]
            };
            rgba.extend_from_slice(&color);
        }
    }
    egui::IconData {
        rgba,
        width: SIZE_U32,
        height: SIZE_U32,
    }
}
