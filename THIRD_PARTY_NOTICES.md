# Third-party notices

FastMarkdownViewer is built from open-source Rust crates. Exact package names, versions, checksums, and dependency edges are recorded in `Cargo.lock`.

Major direct components include:

- eframe, egui, and egui_extras — MIT OR Apache-2.0
- egui_commonmark and egui_commonmark_backend — MIT OR Apache-2.0. Both 0.25.0 crates are vendored; patch records are in their `FASTMARKDOWNVIEWER-PATCHES.md` files and original licenses are preserved.
- pulldown-cmark — MIT
- syntect — MIT (embedded syntax definitions and themes retain their upstream notices)
- regex — MIT OR Apache-2.0
- RaTeX crates — MIT
- image — MIT OR Apache-2.0
- url — MIT OR Apache-2.0
- ureq — MIT OR Apache-2.0
- rfd — MIT
- open — MIT
- Noto Emoji 3.002 — SIL Open Font License 1.1. The bundled monochrome variable
  font is from Google Fonts commit
  `b979dba422e445492b0eb9951ac52ee0b4d648c3`; its license is preserved at
  `assets/fonts/OFL-NotoEmoji.txt`.

The transitive graph also includes components under the Boost Software License
1.0 and root-certificate data under CDLA-Permissive-2.0. egui's embedded default
fonts include files under the SIL Open Font License 1.1 and Ubuntu Font Licence
1.0. These are permissive licenses and are explicitly accepted by `deny.toml`.

As of the v0.1 dependency lock, RaTeX's font stack transitively uses
`ttf-parser` 0.25.1. RustSec advisory RUSTSEC-2026-0192 marks that crate as
unmaintained but identifies no vulnerability and provides no safe upgrade. The
warning is explicitly tracked in `deny.toml`; it must be revisited whenever the
RaTeX stack is updated.

The packaged notices include the complete project MIT and Apache-2.0 license texts. `cargo deny check licenses` is the release-time source of truth for the complete transitive dependency set.

No Tinta source code is included or adapted. Tinta informed product-positioning research only.

Windows fallback fonts are read from the local operating system and are not redistributed.

Syntect 5.3 uses bincode 1.3.3 for its bundled syntax/theme dumps. RustSec
RUSTSEC-2025-0141 marks bincode as unmaintained, with no safe upgrade and no
vulnerability listed. The application only deserializes syntect's embedded
trusted dumps, not document-supplied binary data. This maintenance exception is
tracked explicitly in `deny.toml` and must be revisited when syntect changes.

## Rusty Mermaid

Rusty Mermaid 0.2.0 and its component crates are MIT licensed. The diagrams component is locally patched; its provenance and changes are recorded in experiments/mermaid-spike/vendor/rusty-mermaid-diagrams/README.md.

MIT License

Copyright (c) 2026 Nabeel Ali Memon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Rusty Mermaid SVG text rendering enables usvg 0.45.1's rustybuzz 0.20.1 dependency. RUSTSEC-2026-0206 marks rustybuzz as unmaintained, with no vulnerability or safe upgrade listed. This maintenance exception is tracked in deny.toml; revisit it when the shared resvg/usvg dependency can migrate to harfrust. Mermaid rendering runs in an isolated child process with a timeout.
