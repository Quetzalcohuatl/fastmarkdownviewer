# FastMarkdownViewer patches

Vendored `egui_commonmark` 0.25.0, under its original MIT OR Apache-2.0 license.

- Record heading levels, text, and layout positions during parsing, including ATX, Setext, and duplicate headings.
- Preserve explicit heading IDs in navigation records so application anchors and outline navigation share the same geometry.
- Render body labels through the backend's geometry-aware selectable label helper.
- Record semantic spaces/newlines for search across inline formatting without joining unrelated blocks or table cells.
- Expose an optional image gate callback for application-controlled remote image placeholders.
- Recursively render balanced nested blockquotes/definitions, restore outer inline styles, and wrap proportionally sized table cells within a horizontal scroll area.
- Lay out table rows explicitly with shared column origins and a height equal to the tallest cell. Expose a default-on wrapping option for prose, tables, and code.

Use the stable `show` path; upstream `show_scrollable` can split nested list parser state. Original licenses and crate provenance are retained. No upstream submission is claimed.

- Expose an optional Mermaid diagram callback while retaining source rendering.
