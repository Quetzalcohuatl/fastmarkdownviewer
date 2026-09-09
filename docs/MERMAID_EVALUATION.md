# Mermaid feasibility — 2026-09-08

**Decision: defer production Mermaid rendering.** Fenced Mermaid remains readable as code. Neither evaluated renderer is a clean fit for the existing SVG pipeline. No Mermaid dependency was added to the application manifest or lockfile.

| | mermaid-rs-renderer 0.3.1 | merman 0.7.0 |
|:--|:--|:--|
| Configuration tested | Defaults off; SVG library only | Defaults off; `render` only; resvg-safe SVG method |
| Basic syntax tested | Flowchart, sequence, class, state, ER, pie: all returned SVG | Same six: all returned SVG |
| Unique normal/build package entries in isolated tree | 64 | 212 |
| Optimized spike EXE | 5,009,408 bytes | 10,720,768 bytes |
| Increase over resvg-only harness | 3,710,976 bytes (3.54 MiB) | 9,422,336 bytes (8.99 MiB) |
| Top-level license | MIT | MIT OR Apache-2.0 |
| Registry activity at evaluation | Updated July 6, 2026 | Updated September 2, 2026 |
| Invalid header / unfinished flowchart | Both returned readable errors | Both returned readable errors |

The baseline harness is 1,298,432 bytes with 33 package entries. Counts deduplicate package/version lines from `cargo tree -e normal,build --prefix none`, including the harness, using its pinned lockfile. These are not incremental production dependency counts. Sizes use the app's optimization/LTO/strip/panic settings on Windows x64, Rust 1.95.0; actual viewer impact could differ because of shared code.

[mmdr's API](https://docs.rs/mermaid-rs-renderer/0.3.1/mermaid_rs_renderer/) documents additional chart types and optional CLI/PNG features. [Merman's API](https://docs.rs/merman/0.7.0/merman/render/struct.HeadlessRenderer.html) exposes strict parsing and a resvg-safe conversion. Both are active pre-1.0 projects, not established drop-in Mermaid.js equivalents. Primary sources: [mmdr repository](https://github.com/1jehuang/mermaid-rs-renderer), [Merman repository](https://github.com/Latias94/merman), [mmdr registry](https://crates.io/crates/mermaid-rs-renderer), [Merman registry](https://crates.io/crates/merman). Update times describe crate records, not necessarily publication dates of the selected versions.

Their declared licenses fit the project's dual-license direction. Merman's transitive metadata also includes MPL-2.0 packages and deprecated `serde_yaml`; production adoption would need a full cargo-deny/notice review. This spike is not that review. Rendering uses no browser, Chromium, WebView, Node, or network. Cargo downloads during evaluation are separate from diagram rendering.

## Output quality and safety

Both candidates produced parseable SVG for six deliberately small common inputs. Both rejected the two malformed cases with exit code 2; neither aborted or hit the ten-second timeout. This does **not** establish panic freedom, bounded memory, or safety for arbitrary large graphs. The app uses `panic = "abort"`, so a worker thread alone would not isolate a renderer panic. Future integration needs input/graph/output limits and broader malformed-input coverage.

Flowchart raster inspection found recognizable nodes and edges but **no text labels in either result**, including Merman's resvg-safe output. The SVGs contain text; the shipping resvg 0.45.1 feature set disables SVG text handling. The harness uses that same feature set. Enabling text support or converting labels to paths requires a deliberate font, glyph, shaping, size, and startup strategy. This is the immediate integration blocker, not a claim that either library inherently generates bad SVG. Other diagram types were parsed/rasterized but not exhaustively visually assessed.

Single fresh-process render observations ranged from roughly 14–784 ms for mmdr and 1–4 ms for Merman. The first mmdr sample was a large outlier; its source includes lazy system-font loading. These single observations are diagnostics, not a speed ranking. [Raw results](../experiments/mermaid-spike/results.csv) retain errors and timings.

## Reproduce and reconsider

[The isolated harness and eight-file corpus](../experiments/mermaid-spike/README.md) are outside the application workspace and have their own lockfile. Each diagram runs in a separate process with a ten-second deadline. SVG and PNG output stays under ignored `target/`.

Reconsider mmdr first after demonstrating and measuring an acceptable SVG text solution. Any integration should initialize only for visible Mermaid fences, use bounded background workers and caches, reuse the existing image pipeline, and preserve readable source/error fallback. Do not enable CLI, PNG, GUI, PDF, or math stacks merely to obtain SVG.
