# Current feature comparison — September 14, 2026

This compares products found through [three Google result pages](google-discovery.md),
plus the earlier Windows reference apps. **D** means documented by the linked
upstream source; **V** means observed in our fixture spot check; **P** means
partial/narrower support; **—** means explicitly absent or outside the product's
stated purpose; **?** means not established, not unsupported. A documented
feature is not a conformance test. Current upstream pages can describe work
newer than the pinned performance binaries.

[Pinned GitHub documentation revisions](google-sources.json) record the source
snapshots used for the repository-backed products; website and Store links were
checked on the date above.

## Desktop reading and writing

| Product and source | Platforms advertised / tested here | Tables + tasks | Math | Mermaid | Find / outline | Edit contents | Export | Reload external edits |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [FMV 0.2.1](../../README.md) | Windows, Mac, Linux / Windows resources | V | V | P: Rusty subset | V / V | — | — | Manual |
| [Markpad](https://github.com/sftwrdotdev/Markpad) | Windows, Mac, Linux / Windows 2.7.6 | V | V | D | D / D | D | D: HTML, print/PDF | D: automatic |
| [Moji](https://github.com/alexishida/Moji) | Windows, Mac, Linux / Windows 1.0.7 | V | V | D | D / V | D | D: HTML, PDF, PNG | ? |
| [MarkLite](https://marklite.app/) | Windows, Linux / Windows 1.1.1 | ? | ? | ? | D: editor / ? | D | ? | ? |
| [aydiler/md-viewer](https://github.com/aydiler/md-viewer) | Linux-focused; Windows 0.2.0 tested | V | V | D: merman | D / V | — | ? | D: automatic |
| [MarkText](https://github.com/marktext/marktext) | Windows, Mac, Linux / Windows 0.19.1 | V | P: tested display delimiter variant stayed source | D | V / D | D | D: HTML, PDF | ? |
| [Typora](https://typora.io/) | Windows, Mac, Linux / Windows 1.14.10 | D | D | D | D / D | D | D: PDF, HTML and more | ? |
| [Marky](https://github.com/GRVYDEV/marky) | Mac, Linux / no current Windows score | V in earlier Linux test | D; source in earlier fixture | D | D / V in earlier test | — | ? | D: automatic |
| [Nimbalyst](https://github.com/nimbalyst/nimbalyst) | Windows, Mac, Linux / setup pilot only | ? | ? | D | ? / ? | D | ? | ? |
| [Simple Markdown Viewer](https://apps.microsoft.com/detail/9p1l338qvhjw) | Windows / unmeasured | ? | D: KaTeX | D | D / D | D: split editor | D: HTML, PDF, print | D: automatic |
| [Markdown Viewer Free](https://apps.microsoft.com/detail/9p9sdhx8tqvq) | Windows / unverified launch pilot | ? | ? | ? | ? / ? | ? | ? | ? |
| [iA Writer](https://ia.net/writer) | Windows, Mac, iPhone/iPad / unmeasured | ? | ? | ? | D / D | D | D: PDF/templates | ? |
| [MacMD Viewer](https://macmdviewer.com/) | Mac / unmeasured | D: tables; tasks ? | ? | D | ? / D | — | D: print/PDF | D: automatic |
| [MD-Viewer](https://apps.apple.com/us/app/md-viewer/id6752493034?mt=12) | Mac / unmeasured | ? | ? | ? | ? / ? | —: marketed for reading | D: PDF | ? |
| [Markoff](https://thoughtbot.com/blog/markoff-free-markdown-previewer) | Mac / unmeasured | ? | ? | ? | ? / ? | —: opens external editor | ? | D: automatic |

MarkLite's concise website does not establish its full Markdown dialect. Unknown
cells should not be read as FMV victories. Similarly, a product's general
Markdown claim is not proof that it passes every GFM extension. The Store listing
for Markdown Viewer Free promises offline basic Markdown rendering; that alone
does not establish advanced math, diagrams, or navigation features.

## Workflow differences that matter

| Capability | FMV | Alternatives offering it |
| --- | --- | --- |
| Read without an editor/workspace | D: open a file directly | Also Marky, aydiler, MacMD Viewer, MD-Viewer, Markoff; Markpad/Moji have reading modes |
| Native renderer without a browser engine | D: Rust/egui | aydiler is also Rust/egui; MarkLite bundles Flutter; mdview-zig and MarkMello also avoid a browser engine |
| Tabs and saved sessions | D: released in 0.2.0 | D: Markpad, aydiler, MarkText; Moji documents tabs, recent files and untitled-draft recovery, not equivalent restoration of every saved tab |
| Drag a tab into a new native window | D | Markpad documents multiple windows; equivalent drag behavior was not tested |
| Automatic file watching | —: F5/Ctrl+R | D: Markpad, aydiler, Marky, Simple Markdown Viewer, MacMD Viewer, Markoff |
| Folder/workspace navigation | — | D: aydiler, Marky, MarkText, Nimbalyst; Obsidian and VS Code also provide it |
| Export/print | — | D: Markpad, Moji, MarkText, Typora, Simple Markdown Viewer, iA Writer, MacMD Viewer, MD-Viewer |
| Localized interface | —: English labels | D: Markpad lists 26 languages; Moji lists multiple locales; iA Writer documents Windows translations; MD-Viewer lists 52 languages |
| Non-Latin document/input glyphs | P: tested scripts, system fonts required; bidirectional layout limited | Markpad/Moji render Japanese in our fixture; broad language correctness was not tested across competitors |
| Finder Quick Look extension | — | D: MacMD Viewer |
| Raw HTML rendering | —: inert text by design | D: Marky sanitizes HTML; aydiler supports HTML tables; browser-based viewers offer broader HTML handling |
| Full Mermaid.js compatibility | —: native subset with readable source fallback | Moji, Markpad, Marky and several web viewers use Mermaid.js; this is not a guarantee every version supports every diagram |
| Free, open-source desktop app | D: MIT/Apache-2.0 | Also Markpad (BSD-3-Clause), Moji/MarkLite/MarkText/aydiler/Nimbalyst (MIT), Marky (Apache-2.0), Markoff (open source) |

Multilingual **glyph coverage**, translated **UI labels**, and correct Unicode
**bidirectional layout** are three different capabilities. FMV's recent font
regressions and fixes do not establish feature parity with localized apps.

## Browser and mobile alternatives

These are feature comparisons, not Windows resource measurements. A browser
that is already running can change the incremental cost of opening a document.

| Product / primary source | Environment | Documented strengths | Benchmark status |
| --- | --- | --- | --- |
| [Markdown Reader](https://md-reader.github.io/) | Chrome, Edge, Firefox extension | Local/URL files, automatic refresh, outline, code highlighting, math and Mermaid; Pro offering | Needs isolated browser baseline |
| [Markdown Viewer (simov)](https://github.com/simov/markdown-viewer) | Browser extension | Local/remote files, themes, code highlighting, math, Mermaid, outline, auto-reload | Needs isolated browser baseline |
| [MarkView](https://getmarkview.com/) | Chromium extension | Local/URL files, math, Mermaid, outline, file browser, auto-refresh, 15 UI languages | Needs isolated browser baseline |
| [Markdown Live Preview](https://markdownlivepreview.com/) | Web editor | Source/preview, synchronized scroll, dark mode, PDF export control | Web UI inspected; no resource ranking |
| [Markdown Viewer (pages.dev)](https://markdownviewer.pages.dev/) | Web editor | File import, edit/split/preview, find/replace, HTML/PDF/PNG exports, translated UI, sharing | Web UI inspected; no resource ranking |
| [StackEdit](https://stackedit.io/) | Web editor | Offline writing, synchronized preview, math, diagrams, cloud sync/publishing, collaboration | No browser resource ranking |
| [Simple Markdown](https://play.google.com/store/apps/details?id=com.wbrawner.simplemarkdown&hl=en_US) | Android editor | Open-source editing and sharing on mobile | No matched Android device test |
| [Quill](https://quilljs.com/) | Rich-text editor library | Embeddable editor with an API | Not a standalone Markdown viewer |

## Earlier reference apps retained

The [original feature matrix](features.md) and [rendering observations](observations.md)
cover MarkMello, MD Preview, mdview-zig, Obsidian and VS Code. Their pinned Windows
binaries remain in the new resource batch; they are not claimed to be direct
Google results. In particular, mdview-zig's Windows build used less memory while
leaving table pipes, task markers, math and alerts as source. MarkMello's tested
math was absent and tasks appeared as ordinary bullets. FMV's old “no saved
settings” entry in that historical matrix does **not** describe 0.2.1.

## Reading the comparison fairly

FMV combines a small standalone Windows download, low measured resource use,
technical-document rendering, and a focused read-only workflow. It is not the
most feature-rich editor, the smallest viewer by every metric, or an established
first-content speed champion. Resource usage and rendering coverage should be
read together; missing capabilities are potential product work, not grounds to
erase the competitor's better metric.
