# FastMarkdownViewer

FastMarkdownViewer is a Windows-first, read-only Markdown viewer. Its job is deliberately narrow: double-click a Markdown file and see a rendered document quickly.

> **Beta:** v0.1.0-beta.2 keeps the product deliberately small while hardening the core Windows workflow. Performance claims will be published only when they are backed by repeatable measurements.

## What v0.1 includes

- CommonMark and GitHub-style tables, task lists, strikethrough, autolinks, footnotes, definition lists, and alerts
- Inline, display, and fenced math rendered with RaTeX
- Selectable plain code blocks without startup syntax highlighting
- Standard Unicode emoji rendered with a bundled monochrome fallback font
- Local and remote PNG, JPEG, WebP, first-frame GIF, and SVG images
- A small File/Settings menu, per-window theme switching, Ctrl+mouse-wheel zoom, keyboard scrolling, Ctrl+O, and drag-and-drop
- A portable executable and a per-user installer for Windows 10 22H2 and Windows 11 x64

There is no editor, file watcher, search index, history database, updater, telemetry, or settings file.

## Usage

```text
FastMarkdownViewer.exe
FastMarkdownViewer.exe "C:\path with spaces\README.md"
FastMarkdownViewer.exe --version
```

The installer registers `.md` and `.markdown` under **Open with**. It does not and cannot silently replace your chosen Windows default application.

## Build from source

Prerequisites are Rust 1.95.0 and the Visual Studio 2022 C++ Build Tools with a Windows 10/11 SDK.

```powershell
rustup show
cargo build --locked --release --target x86_64-pc-windows-msvc
```

Release artifacts are built by GitHub Actions from annotated `vX.Y.Z` tags. See [CONTRIBUTING.md](CONTRIBUTING.md) for local checks, [docs/TESTING.md](docs/TESTING.md) for the permanent interaction gates, [docs/BENCHMARKING.md](docs/BENCHMARKING.md) for the performance protocol, and [docs/RELEASE-CHECKLIST.md](docs/RELEASE-CHECKLIST.md) for the current v0.1 ship decision.

## Privacy and security

Local Markdown is rendered without a browser engine; raw HTML is inert text. Remote images referenced by a document are fetched automatically and asynchronously, with scheme, redirect, response-size, and decoded-size limits. No cookies, credentials, referrer, persistent cache, analytics, or update request is used. Details are in [PRIVACY.md](PRIVACY.md) and security reports belong under [SECURITY.md](SECURITY.md).

## License

Licensed under either [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
