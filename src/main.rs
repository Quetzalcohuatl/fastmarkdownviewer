#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui;
use fast_markdown_viewer::{
    app::{InitialState, ViewerApp},
    cli::{self, Command},
    fonts, graphics,
    launch::{Launch, Session},
    network, platform,
};

fn main() -> eframe::Result {
    fast_markdown_viewer::mermaid::handle_helper_args();
    let command = cli::parse(std::env::args_os());
    if command == Command::Version {
        println!("FastMarkdownViewer {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let (initial, session) = match command {
        Command::Open(paths) => {
            let paths: Vec<_> = paths
                .iter()
                .map(std::path::absolute)
                .collect::<std::io::Result<_>>()
                .map_err(|error| eframe::Error::AppCreation(Box::new(error)))?;
            let session = match Session::start(&paths) {
                Ok(Launch::Primary(session)) => Some(session),
                Ok(Launch::Forwarded) => return Ok(()),
                Ok(Launch::WithoutProfile) => None,
                Err(error) => {
                    show_launch_message(
                        &format!("Could not open files in the viewer. {error}"),
                        rfd::MessageLevel::Error,
                    );
                    return Err(eframe::Error::AppCreation(Box::new(error)));
                }
            };
            let initial = if paths.is_empty() {
                InitialState::Empty
            } else {
                InitialState::Paths(paths)
            };
            (initial, session)
        }
        Command::Error(error) => {
            show_launch_message(&error, rfd::MessageLevel::Info);
            return Ok(());
        }
        Command::Version => unreachable!(),
    };
    #[cfg(target_os = "macos")]
    let (_apple_events, inbox) = fmv_macos_events::Registration::install();

    let title = if let InitialState::Paths(paths) = &initial
        && let Some(name) = paths.last().and_then(|path| path.file_name())
    {
        format!("{} — FastMarkdownViewer", name.to_string_lossy())
    } else {
        "FastMarkdownViewer".to_owned()
    };
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_app_id("FastMarkdownViewer")
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([420.0, 280.0])
            .with_icon(platform::app_icon()),
        persist_window: false,
        persistence_path: None,
        ..Default::default()
    };

    graphics::run_native(
        "FastMarkdownViewer",
        native_options,
        Box::new(move |creation_context| {
            network::install(&creation_context.egui_ctx);
            fonts::install(&creation_context.egui_ctx);
            fast_markdown_viewer::appearance::ThemeChoice::default()
                .apply(&creation_context.egui_ctx);
            creation_context.egui_ctx.all_styles_mut(|style| {
                style.spacing.item_spacing.y = 7.0;
                style.visuals.selection.stroke.width = 1.0;
            });
            let app = ViewerApp::with_saved_state(initial, &creation_context.egui_ctx);
            let app = app.with_launch_session(session, &creation_context.egui_ctx);
            #[cfg(target_os = "macos")]
            let app = app.with_native_events(inbox, &creation_context.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

fn show_launch_message(message: &str, level: rfd::MessageLevel) {
    rfd::MessageDialog::new()
        .set_title("FastMarkdownViewer")
        .set_description(message)
        .set_level(level)
        .show();
}
