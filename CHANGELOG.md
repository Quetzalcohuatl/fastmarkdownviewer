# Changelog

All notable changes are documented here. This project follows Semantic Versioning.

## [Unreleased]

## [0.1.4] - 2026-09-09

- Add Solarized Light/Dark, Quiet Light, Monokai, and Tomorrow Night Blue palettes with matching lazy code highlighting.
- Add separate session-wide text/code font menus using installed Windows fonts while retaining emoji and multilingual fallbacks.
- Add two research-backed manual torture-test suites with companion assets, covering supported behavior and unclaimed features.

## [0.1.3] - 2026-09-09

- Fix multiline code backgrounds and table row/column alignment; remove stray cell delimiters and empty trailing rows.
- Add Settings → Word wrap, enabled by default for prose, table cells, and code. Disabling it preserves long lines with horizontal scrolling.

- Add document-level horizontal overflow scrolling and visible, space-reserving scrollbars for the document, tables, and code blocks; keep long words wrapping and code frames inside their own container.

- Fixed nested blockquote boundaries, definition-list indentation, and nested inline style restoration.
- Preserve spaces between formatted text spans.
- Wrap prose in a bounded reading column and size table columns by content, with cell wrapping and independent horizontal overflow.
- Replace failed-image dots with readable alt-text placeholders, error details, and Retry.
- Load Windows Indic and Thai/Lao font fallbacks on demand; verify Hindi/Devanagari and Thai sample glyphs.

## [0.1.2] - 2026-09-09

- Local Markdown links open or activate tabs in the current window, including relative paths and heading fragments.
- Heading anchors use outline geometry, practical lowercase slugs, explicit IDs, and consistent duplicate suffixes.
- Added tab context-menu Rename file and Show in Explorer actions. Rename changes the filename on disk, rejects collisions/non-Markdown extensions, and preserves tab state.
- Evaluated two native Mermaid renderers in an isolated corpus harness; deferred app integration because of SVG text compatibility and size costs. See `docs/MERMAID_EVALUATION.md`.

## [0.1.1] - 2026-09-08

- Added a session-wide automatic remote-image setting, enabled by default, with individual Load image buttons when disabled.
- Bounded image caches and concurrent loading/decoding; release redundant image copies after texture upload and invalidate images when tabs close or reload.
- Added F5 / Ctrl+R reload, preserving tab identity and reading position; failed reloads preserve the open document.
- Added tab reordering by dragging onto another tab, retaining native window tear-out.
- Limit local image inputs to 10 MiB and validate SVG raster dimensions before allocation.
- Added reproducible exploratory comparisons against v0.1.0, including native process memory and CPU layout measurements.

## [0.1.0] - 2026-09-08

First stable Windows x64 release, including the beta.3 reading tools and documented limitations.

- Find supports `*` for zero or more characters on the same line, with `\*` for a literal asterisk.
- Updated the website, download links, and release documentation for stable v0.1.0.

## [0.1.0-beta.3] - 2026-09-08

### Added

- Ctrl+F find with Unicode-aware case matching, highlighted results, and wrapping next/previous navigation, including horizontal code scrolling.
- Lazy background syntect code highlighting with bounded per-tab caches.
- A resizable, toggleable heading outline supporting duplicate and Setext headings.
- Document tabs with independent scroll/search state, close/cycle shortcuts, and multiple-file drops.
- Drag tabs outside a window to create a new native window, preserving document state; a tab context-menu action provides the same operation.
- Ctrl+F toggles Find, Ctrl+H toggles headings, and the empty-window page lists shortcuts.
- On-demand Windows font fallbacks for East Asian scripts and Arabic/Hebrew glyphs.
- Automated reading-tool regressions and a real-framebuffer visual-check example.

### Changed

- Opening an invalid file preserves existing documents.
- Fenced code language hints are preserved; fenced math still renders as math.

### Limitations

- Moving tabs into existing windows and reordering tabs are not yet supported.
- Native accessibility in detached child windows is limited by the current eframe backend; separate application launches retain accessibility.
- Complete bidirectional paragraph layout still depends on future renderer support.

## [0.1.0-beta.2] - 2026-09-07

### Added

- A compact File menu with Open and Exit actions.
- A Settings menu with per-window system, light, and dark themes plus text zoom.
- Ctrl+mouse-wheel zoom with a 50% to 300% range.
- A bundled monochrome Noto Emoji fallback font.

### Fixed

- The empty landing view now paints the complete window instead of leaving a
  black lower region.

## [0.1.0-beta.1] - 2026-09-06

### Changed

- Errors can be dismissed with the visible Dismiss button or Escape.
- Release automation now distinguishes prerelease tags from stable tags instead
  of marking every release as a prerelease.
- The release bar was simplified around the product's core open, render, scroll,
  install, and safety behavior.

## [0.1.0-alpha.3] - 2026-09-05

The first public alpha of the Windows x64 viewer.

### Added

- Windows-first read-only Markdown viewer foundation
- CommonMark/GFM rendering, lazy RaTeX math, and bounded asynchronous images
- Portable and per-user installer release automation
- No-JavaScript GitHub Pages project site
- Permanent UI regressions for scrolling, drag-and-drop, Ctrl+O, links,
  selection/copy, theme changes, and common DPI scales
- Native Windows executable smoke gate for scroll keys and independent windows

### Fixed

- Replaced the crash-prone hidden `egui_commonmark` paginated renderer with its
  stable renderer inside an egui scroll area
- Clamped End-key scrolling to the measured document range to prevent invalid
  geometry

## [0.1.0-alpha.2] - 2026-09-05

Release automation reached packaging but failed before publishing because the
runner already contained a newer Inno Setup than the pinned install command.

## [0.1.0-alpha.1] - 2026-09-05

Release automation preflight failed before building or publishing binaries.
