//! Temporary diagram tiles: the total diagram has no bitmap or cache-size cutoff.
use eframe::egui;
use std::{collections::HashSet, path::Path, sync::Mutex};

pub(crate) const TILE_EDGE: u32 = 512;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) struct RasterRequest {
    pub max_width: f32,
    pub pixel_scale: f32,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) struct Geometry {
    pub source_width: f32,
    pub source_height: f32,
    pub width: u32,
    pub height: u32,
}

impl Geometry {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn new(width: f32, height: f32, request: RasterRequest) -> Result<Self, String> {
        if [width, height, request.max_width, request.pixel_scale]
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
        {
            return Err("Invalid diagram dimensions".into());
        }
        let scale = (request.max_width / width).min(1.0) * request.pixel_scale;
        let pixel_width = (width * scale).ceil().max(1.0);
        let pixels = [pixel_width, (height * pixel_width / width).ceil().max(1.0)];
        if pixels
            .iter()
            .any(|value| !value.is_finite() || *value >= u32::MAX as f32)
        {
            return Err("Diagram dimensions cannot be represented by the renderer".into());
        }
        Ok(Self {
            source_width: width,
            source_height: height,
            width: pixels[0] as u32,
            height: pixels[1] as u32,
        })
    }
}

pub(crate) fn tile_path(directory: &Path, column: u32, row: u32) -> std::path::PathBuf {
    directory.join(format!("tile-{column}-{row}.png"))
}

pub(crate) struct Diagram {
    pub directory: Option<tempfile::TempDir>,
    pub geometry: Geometry,
    pub context: egui::Context,
    pub used_uris: Mutex<HashSet<String>>,
}

impl Diagram {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn show(&self, ui: &mut egui::Ui) {
        let geometry = self.geometry;
        let width = geometry.source_width.min(ui.available_width()).max(1.0);
        let size = egui::vec2(
            width,
            width * geometry.source_height / geometry.source_width,
        );
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        let clip = rect.intersect(ui.clip_rect());
        if !clip.is_positive() {
            return;
        }
        let scale = size / egui::vec2(geometry.width as f32, geometry.height as f32);
        let first = (clip.min - rect.min) / scale / TILE_EDGE as f32;
        let last = (clip.max - rect.min) / scale / TILE_EDGE as f32;
        let rows = geometry.height.div_ceil(TILE_EDGE);
        let columns = geometry.width.div_ceil(TILE_EDGE);
        for row in (first.y.floor().max(0.0) as u32)..=(last.y as u32).min(rows - 1) {
            for column in (first.x.floor().max(0.0) as u32)..=(last.x as u32).min(columns - 1) {
                let path = tile_path(self.directory.as_ref().unwrap().path(), column, row);
                // The helper publishes each tile atomically; don't cache a missing-file error.
                if !path.is_file() {
                    continue;
                }
                let Ok(uri) = url::Url::from_file_path(path) else {
                    continue;
                };
                let uri = uri.to_string();
                self.used_uris
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .insert(uri.clone());
                let x = column * TILE_EDGE;
                let y = row * TILE_EDGE;
                let tile_size = egui::vec2(
                    (geometry.width - x).min(TILE_EDGE) as f32,
                    (geometry.height - y).min(TILE_EDGE) as f32,
                );
                let tile_rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2(x as f32, y as f32) * scale,
                    tile_size * scale,
                );
                ui.put(
                    tile_rect,
                    egui::Image::new(uri)
                        .fit_to_exact_size(tile_rect.size())
                        .show_loading_spinner(false),
                );
            }
        }
    }
}

impl Drop for Diagram {
    fn drop(&mut self) {
        for uri in self
            .used_uris
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .drain()
        {
            self.context.forget_image(&uri);
        }
        if let Some(directory) = self.directory.take() {
            // Deleting thousands of tiles must not stall a theme change or tab close.
            let _ = std::thread::Builder::new()
                .name("Mermaid cleanup".into())
                .spawn(move || drop(directory));
        }
    }
}
