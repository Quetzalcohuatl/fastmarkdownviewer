use egui::{Galley, Pos2, Rect, Ui};
use std::{ops::Range, sync::Arc};

#[derive(Debug, Clone)]
pub struct Heading {
    pub id: Option<String>,
    pub level: u8,
    pub text: String,
    pub position: Pos2,
}

#[derive(Debug)]
pub struct TextRegion {
    pub bytes: Range<usize>,
    pub position: Pos2,
    pub galley: Arc<Galley>,
    pub clip: Rect,
    char_offsets: Option<Vec<usize>>,
}

/// Geometry is rebuilt with layout, so navigation survives wrapping, zoom and images loading.
#[derive(Debug)]
pub struct Navigation {
    pub text: String,
    pub regions: Vec<TextRegion>,
    pub headings: Vec<Heading>,
    pub capture_text: bool,
    pub scroll_target: Option<Range<usize>>,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            text: String::new(),
            regions: Vec::new(),
            headings: Vec::new(),
            capture_text: true,
            scroll_target: None,
        }
    }
}

impl Navigation {
    pub fn clear(&mut self) {
        self.text.clear();
        self.regions.clear();
        self.headings.clear();
    }

    pub fn separator(&mut self, separator: &str) {
        if self.capture_text {
            self.text.push_str(separator);
        }
    }

    pub fn record(&mut self, position: Pos2, galley: Arc<Galley>, clip: Rect) {
        if !self.capture_text {
            return;
        }
        let start = self.text.len();
        self.text.push_str(galley.text());
        let char_offsets = if galley.text().is_ascii() {
            None
        } else {
            Some(
                galley
                    .text()
                    .char_indices()
                    .map(|(index, _)| index)
                    .chain(std::iter::once(galley.text().len()))
                    .collect(),
            )
        };
        self.regions.push(TextRegion {
            bytes: start..self.text.len(),
            position,
            galley,
            clip,
            char_offsets,
        });
    }

    pub fn scroll_recorded(&self, ui: &mut Ui) {
        if let (Some(target), Some(region)) = (&self.scroll_target, self.regions.last()) {
            if target.start >= region.bytes.start && target.start < region.bytes.end {
                if let Some((rect, _)) = self.match_rects(target.clone()).first() {
                    ui.scroll_to_rect(*rect, Some(egui::Align::Center));
                }
            }
        }
    }

    /// Exact per-row rectangles, including matches spanning styled inline widgets.
    pub fn match_rects(&self, bytes: Range<usize>) -> Vec<(Rect, Rect)> {
        self.match_rects_impl(bytes, false)
    }

    pub fn visible_match_rects(&self, bytes: Range<usize>) -> Vec<(Rect, Rect)> {
        self.match_rects_impl(bytes, true)
    }

    fn match_rects_impl(&self, bytes: Range<usize>, visible_only: bool) -> Vec<(Rect, Rect)> {
        let mut rects = Vec::new();
        let first = self
            .regions
            .partition_point(|region| region.bytes.end <= bytes.start);
        for region in &self.regions[first..] {
            if region.bytes.start >= bytes.end {
                break;
            }
            if visible_only
                && !region
                    .galley
                    .rect
                    .translate(region.position.to_vec2())
                    .intersects(region.clip)
            {
                continue;
            }
            let start = bytes.start.max(region.bytes.start);
            let end = bytes.end.min(region.bytes.end);
            if start >= end {
                continue;
            }
            let char_index = |byte: usize| {
                region.char_offsets.as_ref().map_or(byte, |offsets| {
                    offsets.partition_point(|index| *index < byte)
                })
            };
            let start = char_index(start - region.bytes.start);
            let end = char_index(end - region.bytes.start);
            let min = region
                .galley
                .layout_from_cursor(egui::text::CCursor::new(start));
            let max = region
                .galley
                .layout_from_cursor(egui::text::CCursor::new(end));
            for index in min.row..=max.row {
                let row = &region.galley.rows[index];
                let left = if index == min.row {
                    row.x_offset(min.column)
                } else {
                    0.0
                };
                let right = if index == max.row {
                    row.x_offset(max.column)
                } else {
                    row.size.x
                };
                let rect = Rect::from_min_max(egui::pos2(left, 0.0), egui::pos2(right, row.size.y))
                    .translate(region.position.to_vec2() + row.pos.to_vec2());
                rects.push((rect, region.clip));
            }
        }
        rects
    }
}

/// Same layout, selection and accessibility path as egui's Label, with geometry capture.
pub fn label(
    ui: &mut Ui,
    text: egui::WidgetText,
    navigation: &mut Navigation,
    link: bool,
) -> egui::Response {
    let mut label = egui::Label::new(text);
    if link {
        label = label.sense(egui::Sense::click());
    }
    let (position, galley, response) = label.layout_in_ui(ui);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), galley.text())
    });
    navigation.record(position, galley.clone(), ui.clip_rect());
    if ui.is_rect_visible(response.rect) {
        egui::text_selection::LabelSelectionState::label_text_selection(
            ui,
            &response,
            position,
            galley,
            ui.visuals().text_color(),
            egui::Stroke::NONE,
        );
    }
    response
}
