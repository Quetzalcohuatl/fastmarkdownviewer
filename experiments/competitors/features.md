# Feature comparison — 2026-09-11

**D** = documented by upstream (not necessarily verified in the tested binary).
**V** = observed in our small-fixture spot check. **P** = partial or narrower support.
**—** = explicitly absent/by design. **?** = not established; never infer absence
from an omitted README bullet. Upstream README snapshots may describe development
newer than the measured release. See [observations](observations.md) for differences.

## Direct desktop competitors

| Capability | FastMarkdownViewer | MarkMello | MarkText | aydiler/md-viewer | Marky | MD Preview | mdview-zig |
|---|---|---|---|---|---|---|---|
| Product | Read-only viewer | Viewer + optional editor | WYSIWYG/source editor | Read-only viewer | Read-only viewer/workspaces | Viewer + source editor | Minimal viewer |
| Repository license label | MIT/Apache-2.0 | GPL-3.0 | MIT | MIT | Apache-2.0 | MIT | GPL-3.0 |
| Windows release tested | Local optimized 0.1.5 | 0.4.0 | 0.19.1 stable | 0.2.0 | — | 1.4.1 | 0.2.0 |
| Linux release attempted | Experimental 0.1.5 | 0.4.0 | 0.19.1 stable | 0.2.0 | 0.1.3 | 1.4.1 | 0.4.0 (crash) |
| Rendering technology | Rust/egui/OpenGL | .NET/Avalonia | Electron | Rust/egui | Tauri/system WebView | Rust/system WebView | Zig/native APIs |
| Rendered tables | V | V | V | V | V | V | P: literal pipes in Windows test |
| Checked task lists | V | P: ordinary bullets in test | V | V | V | V | P: literal markers in test |
| Highlighted fenced code | V | Code block observed; highlighting ? | V | V | V | V | V |
| Inline math | V: RaTeX | P: absent in test | V: KaTeX | V: typst/mitex | D: KaTeX; source in Linux test | V: KaTeX | P: literal source |
| Display math | V | P: absent in test | D; fixture delimiter variant stayed source | V | D; fixture stayed source | V | P: literal source |
| Mermaid | P: Rusty subset, offline helper | D: Naiad native renderer | D | D: merman native renderer | D: Mermaid.js | D: Mermaid.js offline | ? |
| GitHub alerts | V | P: plain quote in test | P: marker stayed source | V | P: marker stayed source | V | P: marker stayed source |
| Footnotes | D | ? | ? | D | D | ? | ? |
| Definition lists | D | ? | ? | ? | ? | ? | ? |
| Raw HTML | —: inert text | ? | D: editing/rendering options | P: HTML tables explicitly documented | D: DOMPurify sanitization | ? | ? |
| YAML frontmatter | —: no metadata-card feature | ? | D | D: key/value table | ? | ? | ? |
| Images | D: local/remote raster + SVG | ? | D | D: local/remote + SVG | ? | D | ? |
| Find in document | V: rendered text, wildcard/case | D | V | D | ? | D/V: accessible Find control | ? |
| Heading outline | V | ? (minimap observed) | D | V | V | ? | ? |
| Multiple tabs | D/V | V | D | D/V | V | D/V | ? |
| Detached native windows | D | ? | D: multiple windows | ? | ? | ? | ? |
| File/folder browser | — | ? | D | D/V | D/V | —: tabs/recent files instead | —: minimal file viewer |
| Relative Markdown navigation | D | ? | ? | D | ? | D | ? |
| Back/forward document history | — | ? | ? | D | ? | ? | ? |
| Automatic file reload | —: manual F5/Ctrl+R | ? | ? | D | D | D | D |
| Persistent settings/session | —: session-wide appearance resets | D: appearance controls; persistence ? | D | D | D | D: restored tabs | D: scroll memory |
| Themes/zoom | D | D | D | D | D | D: zoom | P: dark theme |
| Select installed text/code fonts | D: curated platform lists | ? (size/spacing controls documented) | ? | ? | ? | ? | ? |
| Edit document contents | — | D | D | — | — | D: autosave | — |
| PDF/print/HTML export | — | ? | D: PDF/HTML | ? | ? | D: print/PDF | ? |
| Browser engine required | — | — | Bundled Chromium | — | System WebKit | System WebView | — |

Sources: [FastMarkdownViewer](../../README.md),
[MarkMello](https://github.com/dartdavros/MarkMello),
[MarkText](https://github.com/marktext/marktext),
[MarkText diagrams](https://marktext.me/docs/markdown-syntax),
[aydiler/md-viewer](https://github.com/aydiler/md-viewer),
[Marky](https://github.com/GRVYDEV/marky),
[MD Preview](https://github.com/vorojar/md-preview),
[mdview-zig](https://github.com/nathannncurtis/mdview-zig).

“Mermaid supported” does not mean equal diagram coverage. Native Rusty, merman and
Naiad implementations must not be represented as full Mermaid.js compatibility.
Likewise, table support does not establish colspan, alignment, wrapping, or huge
table performance. Read the observed exceptions before treating a checkbox as parity.

## Familiar reference applications

These have broader scopes than a file viewer. “V” below refers to the small Windows
fixture; feature support beyond that is documented, not exhaustively tested.

| Capability | FMV | VS Code 1.130.0 preview | Obsidian 1.13.7 reading view | Typora 1.14.10 Windows / 1.14.9 Linux |
|---|---|---|---|---|
| Scope | Read-only viewer | IDE/editor + preview | Note editor/vault | Markdown editor |
| License model | MIT/Apache-2.0 | Open-source Code base; Microsoft distribution terms | Proprietary | Proprietary; trial used |
| Headings/tables/code | V | V | V | V |
| Task checkboxes | V | Fixture markers remained text | V, interactive tasks | V |
| Math | V | V | V | D; disabled/unrecognized in tested default fixture |
| Mermaid | Native subset | Extensions can add diagram rendering | D | D |
| Alerts/callouts | V | Marker remained source in fixture | V; richer callout syntax | V |
| Footnotes/frontmatter | Footnotes; no metadata cards | Extension/parser dependent | D: footnotes/properties | D |
| Wikilinks/embeds/backlinks | — | Extensions can add workflows | D: native vault features | ? |
| Folder/workspace search | — | D | D | D |
| Editor / source changes | — | D | D | D |
| PDF/export | — | Extensions/external tools | D: PDF | D: PDF and additional formats |
| Custom themes/extensions | Built-in themes only | D | D | Custom CSS themes; no equivalent plugin ecosystem claimed |
| Persistent preferences | — | D | D | D |

Sources: [VS Code Markdown](https://code.visualstudio.com/docs/languages/markdown),
[Obsidian syntax](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown),
[Obsidian backlinks](https://help.obsidian.md/Plugins/Backlinks),
[Obsidian PDF export release notes](https://obsidian.md/changelog/2026-01-12-desktop-v1.11.4/),
[Typora features](https://typora.io/), and the local pilot observations.

VS Code is configured to skip its optional first-run introduction. A minimal local
extension calls the built-in preview command on startup; no third-party Markdown
renderer is installed. Restricted Mode remains enabled. The Windows pilot retained
source and preview tabs; the Linux pilot showed the preview tab. Default application
services are included in the process tree.
Obsidian uses a new vault containing only the selected fixture and image assets,
with a saved reading-mode workspace. A pilot vault containing all stress files
crashed its renderer while indexing; those pilot readings were discarded.
Typora's trial/profile is preserved across samples; no license/trial state is reset.

## Wider open-source research set

These projects belong in the competitive landscape but do not all belong in the
same native desktop timing table. Platform entries describe available/documented
paths, not our certification of support.

| Project | Platform/category | Notable documented support compared with FMV | Measurement scope |
|---|---|---|---|
| [leaanthony/mdview](https://github.com/leaanthony/mdview) | Windows/Linux/macOS; Go/Wails WebView | Minimal single-file viewer, six persistent themes, goldmark rendering | Source-install project; no GitHub release binary in inventory; feature research |
| [beleon/mdview](https://codefloe.com/beleon/mdview) | Native Wayland/C++; hosted on CodeFloe | Find, live reload, outline, text selection, links, images, code highlighting, zoom | Feature research from the [author's description](https://blog.leonbecker.de/mdview-and-the-missing-middle-between-less-and-electron/); no X11 fallback, outside this X11 benchmark |
| [simov/markdown-viewer](https://github.com/simov/markdown-viewer) | Browser extension | Selectable parsers, custom CSS, 30+ themes, automatic reload, outline, MathJax, Mermaid, emoji shortcodes, settings sync | Feature research; browser + extension costs cannot be compared to extension size alone |
| [Glow](https://github.com/charmbracelet/glow) | Windows/Linux/macOS terminal | CLI rendering, file discovery, terminal reading UI; limited by terminal display model | Separate CLI conversion measurement; not a GUI launch result |
| [mdcat](https://github.com/swsnr/mdcat) | Windows/Linux/macOS terminal | CommonMark, syntect, terminal hyperlinks, images in compatible terminals | Separate CLI conversion measurement; terminal rendering not included |
| [Frogmouth](https://github.com/Textualize/frogmouth) | Windows/Linux/macOS terminal; Python/Textual | Local/URL documents, history, bookmarks, table of contents | Feature research; interactive TUI not measured with the GUI harness |
| [Grip](https://github.com/joeyespo/grip) | Python server + browser | GitHub-like preview, linked documents, live reload, HTML export | Feature research; normal rendering uses GitHub API, so network/cache state matters |
| [ekino/MarkdownViewer](https://github.com/ekino/MarkdownViewer) | macOS-focused; Windows 0.11.1 tested | Folder browser, outline, multiple windows, persisted folder, PDF export, KaTeX, Mermaid, alerts, footnotes, image lightbox | Windows CLI launch displayed welcome page instead of fixture; excluded from performance ranking |
| [newuni/md-viewer](https://github.com/newuni/md-viewer) | macOS native + WebView fallback | Finder Quick Look, live reload, HTML export, render tiers/Fast Mode, Find, outline, paste preview | No Windows/Linux package; Mermaid fallback uses jsDelivr, unlike FMV's offline helper |
| [rajatarya/mdviewer](https://github.com/rajatarya/mdviewer) | macOS Tauri | Wikilink labels, foldable callouts, frontmatter cards, KaTeX, Mermaid; optional WeasyPrint PDF export | No Windows/Linux release asset in inventory |
| [trsdn/mdviewer](https://github.com/trsdn/mdviewer) | macOS Swift/WebView; Full/Lite editions | Folder navigator, image inspection, print, themes, alerts/footnotes; Full adds offline Mermaid and YAML cards | No Windows/Linux release asset in inventory |

## Product implications

FMV already offers a substantial read-only feature set: math, code highlighting,
images, search, outline, tabs and detached windows without a browser engine. Its
clearest functional gaps against nearby viewers are **automatic reload, persistent
preferences/session restoration, folder navigation, printing/export, frontmatter
presentation and broader Mermaid compatibility**. Editing is a product-scope choice,
not a missing viewer requirement.

Do not infer that FMV wins performance because it uses Rust or has a smaller binary.
The measured CPU, memory and compatibility results must carry that conclusion.
