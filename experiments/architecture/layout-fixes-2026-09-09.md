# Rendering fixes: exploratory comparison

Compared v0.1.2 (`40ac653`) with the September 9 rendering fixes on the same Windows x64 machine, using Rust 1.95.0 and locked, optimized Glow builds. Three fresh processes per variant and fixture, randomized by `compare-reader.ps1`, each measuring 30 scrolling frames through the production headless egui surface. Reported values are medians of the three per-process medians. These are CPU layout measurements, not GPU presentation or startup timings.

| Fixture | v0.1.2 | Rendering fixes |
| --- | ---: | ---: |
| ordinary-5k.md | 0.78 ms | 0.75 ms |
| gfm-math-images-100k.md | 10.38 ms | 10.37 ms |

The small sample suggests no material regression; it does not establish a speedup. Raw layout and tab-lifecycle memory samples are in [the CSV](layout-fixes-2026-09-09.csv). Native startup was not measured (`-Runs 0 -LayoutRuns 3`). Generate the fixtures with `fixtures/generate.ps1`; use the existing reader comparison harness and independently built `reading_benchmark` examples for the two source revisions. When sharing a Cargo target directory between source snapshots, clean the three local packages before rebuilding to avoid stale path artifacts.

The optimized executable increased from 16,805,376 to 16,821,760 bytes: 16 KiB, approximately 0.10%. No Cargo dependencies or bundled fonts were added. The new fixture was also inspected through real framebuffer captures for tables, nested quotes, definitions, inline formatting, missing images, and Hindi/Thai samples; those captures are correctness checks rather than benchmarks.

## Follow-up: document overflow and solid scrollbars

Repeated the same three-process CPU-layout protocol after adding the document horizontal overflow fallback, solid nested scrollbars, and code-frame bounds. Medians were 0.73 → 0.75 ms for the 5 KiB fixture and 10.63 → 10.84 ms for the 100 KiB fixture (baseline v0.1.2 → final candidate). These roughly 2% differences are exploratory, not a demonstrated regression or speedup. Raw samples: [overflow CSV](overflow-fixes-2026-09-09.csv). The final executable is 16,822,784 bytes, 17 KiB above v0.1.2. No dependencies were added.

## Follow-up: table alignment, code frames, and optional wrapping

With wrapping enabled by default, the same three-process comparison measured 0.86 → 0.82 ms for the 5 KiB fixture and 10.09 → 10.19 ms for the 100 KiB fixture (v0.1.2 → candidate). This is a small exploratory sample; no speedup is claimed. [Raw samples](wrap-fixes-2026-09-09.csv). The executable is 16,820,224 bytes. The user-supplied torture-test document was inspected with both wrap settings; focused tests now cover table alignment, full-height code backgrounds, explicit block boundaries, and the actual Settings toggle.
