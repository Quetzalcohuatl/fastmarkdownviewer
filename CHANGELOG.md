# Changelog

All notable changes are documented here. This project follows Semantic Versioning.

## [Unreleased]

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

## [0.1.0-alpha.3] - 2026-09-05

The first public alpha of the Windows x64 viewer.

## [0.1.0-alpha.2] - 2026-09-05

Release automation reached packaging but failed before publishing because the
runner already contained a newer Inno Setup than the pinned install command.

## [0.1.0-alpha.1] - 2026-09-05

Release automation preflight failed before building or publishing binaries.

## [0.1.0] - Unreleased

The first public release will be cut only after the Windows 10 and Windows 11 acceptance matrix passes.
