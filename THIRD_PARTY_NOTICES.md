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
