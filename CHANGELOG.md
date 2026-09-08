# Changelog

All notable changes are documented here. This project follows Semantic Versioning.

## [Unreleased]

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
