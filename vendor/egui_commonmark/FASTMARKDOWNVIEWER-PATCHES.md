# FastMarkdownViewer patches

Vendored `egui_commonmark` 0.25.0, under its original MIT OR Apache-2.0 license.

- Record heading levels, text, and layout positions during parsing, including ATX, Setext, and duplicate headings.
- Preserve explicit heading IDs in navigation records so application anchors and outline navigation share the same geometry.
- Render body labels through the backend's geometry-aware selectable label helper.
- Record semantic spaces/newlines for search across inline formatting without joining unrelated blocks or table cells.
- Expose an optional image gate callback for application-controlled remote image placeholders.

Use the stable `show` path; upstream `show_scrollable` can split nested list parser state. Original licenses and crate provenance are retained. No upstream submission is claimed.
