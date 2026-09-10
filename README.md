# FastMarkdownViewer

FastMarkdownViewer is a Windows-first, read-only Markdown viewer. Its job is deliberately narrow: double-click a Markdown file and see a rendered document quickly.

> **Stable:** v0.1.4 keeps the product deliberately small while hardening the core Windows workflow. Performance claims will be published only when they are backed by repeatable measurements.

## What v0.1 includes

- CommonMark and GitHub-style tables, task lists, strikethrough, autolinks, footnotes, definition lists, and alerts
- Inline, display, and fenced math rendered with RaTeX
- Selectable code blocks with lazy, background syntect highlighting
- Bundled monochrome emoji plus Windows font fallbacks for Japanese, Chinese, Korean, Arabic, Hebrew, Hindi/Devanagari, and Thai
- Local and remote PNG, JPEG, WebP, first-frame GIF, and SVG images
- Ctrl+F search with highlights, match counts, next/previous navigation, and optional case matching
- A resizable outline sidebar with a Settings toggle
- Multiple document tabs, per-tab scroll/search state, Ctrl+O, and multi-file drag-and-drop
- Drag tabs outside a window to move them into independent native windows
- File/Settings menus, session-wide themes, Ctrl+mouse-wheel zoom, and keyboard scrolling
- A portable executable and a per-user installer for Windows 10 22H2 and Windows 11 x64

There is no editor, file watcher, search index, history database, updater, telemetry, or settings file.

### Appearance

Settings → Color theme offers System, Light, Dark, Solarized Light, Solarized Dark, Quiet Light, Monokai, and Tomorrow Night Blue. These are lightweight built-in interpretations of familiar editor palettes, not imported VS Code themes or extension support. Code highlighting follows the chosen palette, including changes between two light or two dark themes.

Settings → Text font changes proportional text (including menus and headings); Code font independently changes code and inline code. Choices include Segoe UI, Arial, Calibri, Georgia, Times New Roman, Consolas, Courier New, Cascadia Code/Mono, JetBrains Mono, and Fira Code when their supported font filenames are installed in Windows or the user's Windows Fonts folder. Only available choices appear. Default restores the bundled family. Fonts load on selection, retain emoji/script fallbacks, and are never downloaded or redistributed. This is a curated font list, not an arbitrary installed-font browser; custom font files and editor ligature settings are not supported. Theme, font, and zoom changes apply to all windows in the current process and reset on restart. These appearance options were added in v0.1.4.

## Usage

```text
FastMarkdownViewer.exe
FastMarkdownViewer.exe "C:\path with spaces\README.md"
FastMarkdownViewer.exe --version
```

The installer registers `.md` and `.markdown` under **Open with**. It does not and cannot silently replace your chosen Windows default application.

### Reading shortcuts

| Shortcut | Action |
|:--|:--|
| Ctrl+O | Open a file in a tab; an already-open file activates its tab |
| F5 / Ctrl+R | Reload the current file, keeping the tab and reading position |
| Ctrl+W / middle-click tab | Close a tab |
| Ctrl+Tab / Ctrl+Shift+Tab | Next / previous tab |
| Ctrl+F | Show and focus Find / hide Find |
| Enter / Shift+Enter, F3 / Shift+F3 | Next / previous match (wraps around) |
| Escape | Close Find, or dismiss an error |
| Ctrl+H (also Ctrl+Shift+O) | Show / hide the outline sidebar |

Search operates on rendered text, including code and link captions, and can span inline formatting. `*` matches zero or more characters on the same line (for example, `hello*world`); `\*` finds a literal asterisk. Other punctuation is literal. Wildcards match as much of the line as possible. Image contents and rendered math are not searchable. The outline supports ATX and Setext headings, including duplicate titles. A failed open leaves existing tabs intact. The empty-window page lists reading shortcuts.

Drag a tab onto another tab to reorder it, or outside the window to move it into a new native window, keeping its document, scroll position, search, and caches. Escape cancels a drag. The tab's right-click menu also offers **Move to new window**. Closing the original window keeps detached windows alive; closing the last window exits. Theme and text size are shared within a process; each window has its own outline toggle. Moving tabs into an existing window is not supported yet. Separate command-line launches still start separate processes. Local Markdown links open or activate a tab in the current window.

Reload reads the file again and refreshes its image and rendering caches. If the file has become unreadable, the current document stays open with an error message.

Markdown links such as `[Chapter](docs/chapter.md)` open in this window and reuse an already-open canonical path. `[Section](#section)` scrolls within the current document; `[Section](other.md#section)` opens or activates the target tab and scrolls there. Heading slugs lowercase Unicode text, replace whitespace with hyphens, and remove punctuation other than hyphens/underscores. Repeated headings use `-1`, `-2`, etc.; explicit `{#id}` heading IDs also work. Percent-encoded paths/fragments are supported. Missing files or headings show a nonfatal error. This is practical slug support, not exact GitHub parity.

Right-click a tab for **Rename file…** or **Show in Explorer**. Rename changes the actual filename in its current folder; omitting the extension preserves `.md`/`.markdown`, and other extensions, invalid names, and collisions are rejected. Tab order, active selection, and reading/search state are retained; internal path, title, and resource base are reloaded. It does not rewrite links in other documents. To prevent overwriting even a concurrently created destination, rename uses exclusive hard-link creation followed by removing the original name; this requires a filesystem with hard-link support (such as NTFS). Unsupported filesystems fail with an error, leaving the original in place. A process interruption between those operations can leave both names. Case-only renames on case-insensitive filesystems are rejected as collisions.

**Show in Explorer** opens the parent folder and selects the file. A portable platform abstraction also provides Finder selection on macOS and parent-folder opening elsewhere; Windows is the supported and tested target. Reading never edits document contents; filename changes happen only through the explicit rename action.

Mermaid remains a readable code block. See [the native renderer evaluation](docs/MERMAID_EVALUATION.md) for the corpus, measurements, and reasons for deferring rendering.

**Settings → Automatically load remote images** defaults to on. Turn it off to hide remote images and show individual **Load image** buttons. This setting applies to every window in the current session and resets on the next launch. Requests already running may finish. Ordinary links still open only when clicked.

The current eframe backend does not initialize native accessibility for dynamically created child windows. For screen-reader access, open the file through a separate application launch instead of detaching its tab.

Syntax definitions initialize on a worker only when a language-tagged code block becomes visible. Results are cached by content, language, theme, and font size. Unknown languages remain plain text; blocks over 256 KiB skip highlighting. Highlight caches are bounded to 128 entries and approximately 8 MiB per tab.

East Asian, Indic (Nirmala UI), and Thai/Lao (Leelawadee UI) system fonts load on demand. Glyph coverage depends on installed Windows fonts; no system fonts are redistributed or downloaded. Hindi/Devanagari and Thai samples are covered on the tested Windows installation. The renderer already uses HarfRust shaping, but full Unicode bidirectional paragraph layout remains limited, so mixed Arabic/Hebrew and left-to-right text needs further renderer work. Coverage is not a guarantee of correct layout for every language. Myanmar, Khmer, Tibetan, Ethiopic, and historic scripts have no dedicated fallback configured and remain unverified. Emoji are monochrome.

Ordinary prose, including long unbroken words, wraps within a reading column capped at 960 logical pixels. Tables give longer columns more room, wrap text and long tokens, and scroll horizontally when their minimum widths exceed the available space. **Settings → Word wrap** is enabled by default for prose, table cells, and code, and is shared across windows in the current session. Disable it to retain long lines and scroll horizontally. Code blocks retain their own horizontal scroll area when needed. Scrollbars stay visible while content overflows and reserve space beside it. The document also has a horizontal scrollbar as a fallback for objects that cannot fit. Nested blockquotes and inline styles preserve their surrounding structure and formatting.

Definition lists are an extension, not part of core CommonMark. Write a term on one line followed by `: Definition` on the next; multiple definitions and indented continuation paragraphs are supported. Failed images display an **Image unavailable** placeholder with alt text, error details on hover, and a **Retry** button.

## Build from source

Prerequisites are Rust 1.95.0 and the Visual Studio 2022 C++ Build Tools with a Windows 10/11 SDK.

```powershell
rustup show
cargo build --locked --release --target x86_64-pc-windows-msvc
```

Release artifacts are built by GitHub Actions from annotated `vX.Y.Z` tags. See [CONTRIBUTING.md](CONTRIBUTING.md) for local checks, [docs/TESTING.md](docs/TESTING.md) for the permanent interaction gates, [docs/BENCHMARKING.md](docs/BENCHMARKING.md) for the performance protocol, and [docs/RELEASE-CHECKLIST.md](docs/RELEASE-CHECKLIST.md) for the current v0.1 ship decision.

## Privacy and security

Local Markdown is rendered without a browser engine; raw HTML is inert text. Remote images referenced by a document load asynchronously when enabled, with scheme, redirect, response-size, and decoded-size limits. Image workers and memory caches are bounded and start only when needed. No cookies, credentials, referrer, persistent cache, analytics, or update request is used. Details are in [PRIVACY.md](PRIVACY.md) and security reports belong under [SECURITY.md](SECURITY.md).

## License

Licensed under either [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
