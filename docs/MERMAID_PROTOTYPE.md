# Native Mermaid prototype — 2026-09-10

## Decision

Native Mermaid remains compatible with the viewer's lightweight direction. The prototype now rasterizes labels successfully without a browser, Node, or a rendering service. It is **not production support**: strict parsing accepted an intentionally malformed line, and a realistic workflow exposed awkward edge routing. Keep the experiment isolated until these fidelity gaps and application integration are addressed.

The application manifest, application lockfile, published v0.1.4 executable, and ordinary-document rendering are unchanged. This experiment cannot establish integrated viewer startup, scrolling, memory, or final executable size.

## Implementation

The isolated crate uses pinned `mermaid-rs-renderer 0.3.1`, defaults disabled, with its strict parsing entry point. An opt-in `svg-text` feature enables resvg/usvg text shaping. It loads four explicit Windows text/code font files rather than asking usvg to scan all installed fonts. Large supplemental CJK fonts load only when relevant script ranges appear. Font files are not redistributed. Missing supplemental fonts can still leave missing glyphs on other machines.

The helper rejects input over 64 KiB, SVG output over 10 MiB, and raster surfaces over 16 megapixels. These are prototype limits, not a comprehensive resource sandbox: input is read before the length check; SVG layout and intermediate allocation happen before the output/pixel checks. The PowerShell runner launches each case separately with a ten-second deadline. A production helper needs an OS memory limit and robust cancellation as well as pre-layout graph limits. An in-process worker cannot isolate aborting panics under our release profile.

## Measurements

Windows x64, Rust 1.95.0, release optimization/LTO/strip settings matching the viewer. All sizes are standalone harness sizes, not incremental viewer measurements.

| Executable | Bytes | MiB |
|---|---:|---:|
| Current strict-parser helper without SVG text | 5,008,384 | 4.78 |
| Current helper with SVG text and conditional fonts | 5,825,536 | 5.56 |
| Original SVG-only baseline from September 8 | 1,298,432 | 1.24 |

SVG text costs about 0.78 MiB against the current no-text helper. The total is about 4.32 MiB above the original baseline; the baseline's harness code predates this change. Actual integrated size will depend on shared dependencies and integration choices. Final text-helper SHA-256: `077EC2B2A3FAC75BBAB5C994E077D2BC05E5CE25AD1F5E4A8489B5CAC8CC9F70`.

Three fresh-process runs per fixture, after prior runs had warmed filesystem/font caches:

| Fixture | Median pipeline time |
|---|---:|
| Class | 15.0 ms |
| Basic flowchart | 27.4 ms |
| Realistic sequence with loop/alternatives | 18.4 ms |
| Multilingual labels | 25.1 ms |
| 200-edge chain | 158.4 ms |
| Workflow with subgraph and branches | 243.5 ms |

Pipeline timing includes input reading, Mermaid rendering, explicit font loading, SVG parsing and rasterization. It excludes process launch and output file encoding/writes. A first-ever earlier render took about 743 ms; the table is not a cold-start guarantee.

The sampled peak working set was approximately 8–14 MiB for the small ordinary valid cases, 23 MiB for the workflow, 30 MiB for the chain, and 53 MiB for the multilingual case. These are sampled lower bounds for standalone helper processes, not viewer memory deltas or reliable full-process peak measurements. Loading every supplemental font for every diagram previously put even ordinary cases around 50–60 MiB; conditional loading avoids that cost in ordinary diagrams.

Raw evidence: [three-run measurements](../experiments/mermaid-spike/results-text-2026-09-10.csv), [limit checks](../experiments/mermaid-spike/results-limits-2026-09-10.csv).

## Validation and remaining work

- Six original diagram types returned SVG with labels. PNG inspection covered flowchart, sequence, class, state, ER, and pie, plus multiline labels, the larger workflow and multilingual text. Labels are visible; mixed Arabic/Hebrew was visually inspected for presence only, not linguistically validated for order/shaping correctness.
- The original invalid header and unfinished edge were rejected. Oversized input was rejected. A 2,000-edge chain was rejected by the pixel limit after layout. No case aborted or hit the deadline. This is a small corpus, not a safety guarantee.
- `invalid-trailing.mmd` unexpectedly succeeds: the renderer creates a node named `totally` while omitting the rest of the malformed statement. A successful renderer return must not be treated as evidence of complete parsing.
- The workflow is readable but has long wrapping edges and labels far from their branches. We have not established GitHub-equivalent layout or syntax support.
- Optimized build, formatting and Clippy with warnings denied passed for the text variant. The production app was not modified, so its full test suite was not rerun.

Next integration gate: resolve or explicitly constrain parser fidelity, add real-world corpus comparisons against Mermaid.js, and measure a lazy helper integration in the viewer. Cache completed renders, retain accessible source fallback, and ensure helper failures cannot close the viewer. No release or support claim should be based solely on this experiment.
