use eframe::egui;
use egui_commonmark_backend::navigation::Navigation;
use std::ops::Range;

#[derive(Default)]
#[allow(clippy::struct_excessive_bools)] // Independent UI preferences and navigation flags.
pub struct Search {
    pub open: bool,
    pub query: String,
    pub case_sensitive: bool,
    pub matches: Vec<Range<usize>>,
    pub selected: usize,
    pub jump: bool,
    previous: Option<(String, bool)>,
    preserve_position: bool,
}

impl Search {
    pub fn invalidate(&mut self) {
        self.previous = None;
        self.matches.clear();
        self.selected = 0;
        self.preserve_position = true;
    }
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
            self.jump = !std::mem::take(&mut self.preserve_position);
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
                && let Some((rect, clip)) = rects.first()
            {
                // Nested objects reveal their own horizontal search target.
                // Keep the parent jump within that object's horizontal viewport.
                let mut target = *rect;
                target.min.x = target.min.x.clamp(clip.left(), clip.right());
                target.max.x = target.max.x.clamp(clip.left(), clip.right());
                ui.scroll_to_rect(target, Some(egui::Align::Center));
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
    let mut pattern = String::new();
    let mut characters = query.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '*' {
            pattern.push_str(".*");
            while characters.peek() == Some(&'*') {
                characters.next();
            }
        } else {
            let literal = if character == '\\' && matches!(characters.peek(), Some('*' | '\\')) {
                characters.next().unwrap()
            } else {
                character
            };
            pattern.push_str(&regex::escape(&literal.to_string()));
        }
    }
    regex::RegexBuilder::new(&pattern)
        .case_insensitive(!case_sensitive)
        .build()
        .map_or_else(
            |_| Vec::new(),
            |pattern| {
                pattern
                    .find_iter(text)
                    .filter(|found| !found.is_empty())
                    .map(|found| found.range())
                    .collect()
            },
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
        assert_eq!(find("a.*b", r".\*", false), vec![1..3]);
        let mut search = Search {
            matches: vec![0..1, 2..3],
            ..Default::default()
        };
        search.advance(true);
        assert_eq!(search.current(), 2);
        search.advance(false);
        assert_eq!(search.current(), 1);
    }

    #[test]
    fn wildcard_matches_with_literal_escaping_and_unicode() {
        assert_eq!(
            find("hello world\nhello beautiful world", "hello*world", false),
            vec![0..11, 12..33]
        );
        assert_eq!(find("ab aXYZb", "a**b", false), vec![0..8]);
        assert_eq!(find("日本語🙂中文", "日本*中文", false), vec![0..19]);
        assert_eq!(find("HELLO world", "hello*world", false), vec![0..11]);
        assert!(find("HELLO world", "hello*world", true).is_empty());
        assert!(find("hello\nworld", "hello*world", false).is_empty());
        assert_eq!(find("a.*b [x]", r".\*", false), vec![1..3]);
        assert_eq!(find("a.*b [x]", "[x]", false), vec![5..8]);
        assert_eq!(find("one\n\ntwo", "*", false), vec![0..3, 5..8]);
        assert!(find("", "*", false).is_empty());
    }
}
