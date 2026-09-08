use eframe::egui;
use egui_commonmark_backend::navigation::Navigation;
use std::ops::Range;

#[derive(Default)]
pub struct Search {
    pub open: bool,
    pub query: String,
    pub case_sensitive: bool,
    pub matches: Vec<Range<usize>>,
    pub selected: usize,
    pub jump: bool,
    previous: Option<(String, bool)>,
}

impl Search {
    pub fn current(&self) -> usize {
        if self.matches.is_empty() {
            0
        } else {
            self.selected + 1
        }
    }

    pub fn advance(&mut self, backwards: bool) {
        if !self.matches.is_empty() {
            let count = self.matches.len();
            self.selected = (self.selected + if backwards { count - 1 } else { 1 }) % count;
            self.jump = true;
        }
    }

    pub fn paint(&mut self, ui: &mut egui::Ui, navigation: &mut Navigation) {
        if !self.open {
            return;
        }
        let signature = (self.query.clone(), self.case_sensitive);
        if self.previous.as_ref() != Some(&signature) {
            self.matches = find(&navigation.text, &self.query, self.case_sensitive);
            self.selected = 0;
            self.jump = true;
            self.previous = Some(signature);
            ui.ctx().request_repaint();
        }
        if self.jump {
            navigation.scroll_target = self.matches.get(self.selected).cloned();
            ui.ctx().request_repaint();
        }
        for (index, range) in self.matches.iter().enumerate() {
            let rects = if self.jump && index == self.selected {
                navigation.match_rects(range.clone())
            } else {
                navigation.visible_match_rects(range.clone())
            };
            if self.jump
                && index == self.selected
                && let Some((rect, _)) = rects.first()
            {
                ui.scroll_to_rect(*rect, Some(egui::Align::Center));
            }
            let color = if index == self.selected {
                egui::Color32::from_rgba_unmultiplied(255, 140, 0, 100)
            } else {
                egui::Color32::from_rgba_unmultiplied(240, 210, 0, 65)
            };
            for (rect, clip) in rects {
                ui.painter()
                    .with_clip_rect(clip)
                    .rect_filled(rect, 1.0, color);
            }
        }
        self.jump = false;
    }
}

fn find(text: &str, query: &str, case_sensitive: bool) -> Vec<Range<usize>> {
    if query.is_empty() {
        return Vec::new();
    }
    regex::RegexBuilder::new(&regex::escape(query))
        .case_insensitive(!case_sensitive)
        .build()
        .map_or_else(
            |_| Vec::new(),
            |pattern| pattern.find_iter(text).map(|found| found.range()).collect(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_literal_search_and_wrapping() {
        assert_eq!(
            find("日本語 🙂 日本語", "日本語", false),
            vec![0..9, 15..24]
        );
        assert_eq!(find("Hello HELLO", "hello", false), vec![0..5, 6..11]);
        assert!(find("Hello", "hello", true).is_empty());
        assert_eq!(find("a.*b", ".*", false), vec![1..3]);
        let mut search = Search {
            matches: vec![0..1, 2..3],
            ..Default::default()
        };
        search.advance(true);
        assert_eq!(search.current(), 2);
        search.advance(false);
        assert_eq!(search.current(), 1);
    }
}
