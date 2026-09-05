#![windows_subsystem = "windows"]

use std::{cell::Cell, path::PathBuf};

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use windows_canvas::*;
use windows_window::*;

fn main() -> windows_core::Result<()> {
    let Some(path) = std::env::args_os().nth(1).map(PathBuf::from) else {
        return Ok(());
    };
    let markdown = std::fs::read_to_string(path).unwrap_or_else(|error| error.to_string());
    let document = visible_text(&markdown);

    let window = Window::new("FastMarkdownViewer Direct2D prototype")
        .size(900, 700)
        .create()?;
    let device = GpuDevice::new()?;
    let (width, height) = window.client_size();
    let mut chain = device.create_swap_chain_for_window(&window, width as u32, height as u32)?;
    let has_drawn = Cell::new(false);

    run_with(|| {
        if has_drawn.replace(true) {
            return Ok(false);
        }
        let width = chain.width() as f32;
        let height = chain.height() as f32;
        let session = chain.begin_draw()?;
        session.clear(ColorF::WHITE);
        let brush = session.create_solid_brush(ColorF::new(0.08, 0.1, 0.13, 1.0))?;
        let format = TextFormat::new("Segoe UI", 17.0)?;
        session.draw_text(
            &document,
            &format,
            &Rect::new(28.0, 24.0, width - 28.0, height - 24.0),
            &brush,
        );
        drop(session);
        chain.present()?;
        Ok(false)
    })
}

fn visible_text(markdown: &str) -> String {
    let mut output = String::with_capacity(markdown.len());
    let options = Options::all();
    for event in Parser::new_ext(markdown, options) {
        match event {
            Event::Text(text)
            | Event::Code(text)
            | Event::InlineHtml(text)
            | Event::Html(text)
            | Event::InlineMath(text)
            | Event::DisplayMath(text) => output.push_str(&text),
            Event::SoftBreak | Event::HardBreak | Event::Rule => output.push('\n'),
            Event::TaskListMarker(checked) => {
                output.push_str(if checked { "☑ " } else { "☐ " });
            }
            Event::Start(Tag::Item) => output.push_str("• "),
            Event::End(
                TagEnd::Paragraph
                | TagEnd::Heading(_)
                | TagEnd::Item
                | TagEnd::CodeBlock
                | TagEnd::TableRow,
            ) => output.push('\n'),
            Event::Start(_) | Event::End(_) | Event::FootnoteReference(_) => {}
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_renderable_text() {
        let text = visible_text("# Title\n\n- [x] **done**\n");
        assert!(text.contains("Title"));
        assert!(text.contains("☑ done"));
    }
}
