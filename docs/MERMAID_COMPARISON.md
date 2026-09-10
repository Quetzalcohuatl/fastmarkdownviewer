# Native Mermaid comparison — 2026-09-10

**Follow-up:** the [local Rusty fixes](RUSTY_FIXES.md) repair the malformed-label, grouping and mindmap defects and reject oversized flowcharts before layout. The comparison below preserves the original unpatched results.

## Recommendation

**Rusty Mermaid 0.2.0 is the best candidate for a small, fast experimental integration. Merman 0.7.0 performed best on our malformed-input checks and inspected label layout, at a much larger size.** Neither result establishes full GitHub compatibility. No renderer was added to the production application.

The new candidates were actually compiled and run, alongside our earlier candidates. Every variant uses the same resvg 0.45.1 text-enabled rasterizer, explicit Windows fonts, script-dependent CJK fonts, and input/output/pixel limits. This comparison includes working labels, not just SVG generation.

## Size and speed

Windows x64, AMD Ryzen 9 9950X, Rust 1.95.0; optimized build with the viewer's opt-level 3, thin LTO, one codegen unit, aborting panics, and stripped symbols. Defaults are disabled on renderer dependencies; only the SVG/render features are enabled. Rebuilt SVG-only baseline: 1,299,968 bytes (1.24 MiB).

| Renderer | Full helper bytes | Extra over baseline | Basic flowchart | Larger workflow |
|---|---:|---:|---:|---:|
| **Rusty Mermaid 0.2.0** | **3,480,064** | **2.08 MiB** | **2.08 ms** | **7.24 ms** |
| mermaid-rs-renderer 0.3.1 | 5,825,536 | 4.32 MiB | 28.10 ms | 248.14 ms |
| Selkie 0.3.0 | 7,874,560 | 6.27 MiB | 4.85 ms | 9.42 ms |
| Merman 0.7.0 | 11,593,216 | 9.82 MiB | 4.57 ms | 10.58 ms |

Times are medians of three fresh-process runs after earlier builds/runs had warmed filesystem caches. They include input reading, parsing/layout, font loading, SVG parsing, and rasterization; they exclude process launch and SVG/PNG output writes/encoding. They are not cold-start or scrolling benchmarks. One first comparison run overlapped a Clippy check; two further runs were performed without build work. Fixed renderer order and the small corpus also limit inference.

These are **isolated executable deltas**, not measured additions to the viewer. Shared dependencies, integration, caching, process isolation, and drawing choices will change production costs. Rusty Mermaid saves 2,345,472 bytes (about 2.24 MiB) against the current mmdr text helper, approximately halving the incremental harness cost.

Sampled peak working sets for the workflow were 19.4 MiB (Rusty), 23.1 MiB (mmdr), 23.0 MiB (Selkie), and 23.2 MiB (Merman). The multilingual case reached roughly 49–53 MiB across candidates because of supplemental fonts. Five-millisecond polling misses short-lived allocations and sometimes entire runs: these figures are lower-bound diagnostics, not accurate peak memory or app memory deltas.

## Correctness and stress results

Twenty fixtures per renderer, three repetitions: **240 process executions**. The corpus has fourteen ordinary valid cases, two long-chain stress cases, three intentionally malformed inputs, and one oversized-input case. Successful exits are not counted as visual or semantic passes.

| Check | Rusty | mmdr | Selkie | Merman |
|---|---|---|---|---|
| Unknown diagram header | Rejects | Rejects | Rejects | Rejects |
| Unclosed node label / unfinished edge | **Accepts and renders only A** | Rejects | Rejects | Rejects |
| Invalid trailing statement | Rejects | **Accepts partial content** | **Accepts** | Rejects |
| Input over 64 KiB | Harness rejects | Harness rejects | Harness rejects | Harness rejects |
| 200-edge chain | Renders | Renders | Renders | Renders |
| 2,000-edge chain | **Stack overflow abort** | Harness pixel limit | **Layout panic abort** | Harness pixel limit |

The two aborts reproduced in all three repetitions. Selkie reports a recursion-depth assertion in its network-simplex layout. No case hit the runner's ten-second deadline. Pixel-limit rejection happens after layout; it does not guarantee bounded intermediate work. A future integration must isolate crashes and bound graph work before rendering. A background thread alone cannot contain an abort or stack overflow.

Visual inspection findings:

- **Rusty:** readable basic and realistic flowcharts, sequence diagrams and multilingual labels. Long labels expand the image rather than wrapping like mmdr/Merman. The mindmap renders the root syntax literally as `root((Markdown viewer))`, so a successful SVG return does not mean syntax fidelity. Custom node styling appeared in the styled sample. The malformed flowchart becomes a misleading single-node diagram.
- **mmdr:** readable labels and useful wrapping, but the larger workflow uses awkward wrapping edges with labels far from their branches. It silently drops part of the malformed trailing statement.
- **Selkie:** multiline/long-label sample clips the long label; multilingual labels overflow their boxes. It is larger than mmdr and did not offer a compelling quality advantage in the inspected samples.
- **Merman:** inspected workflow, long-label and multilingual samples are readable; rejects all three malformed fixtures. This is the strongest observed combination of error handling and label layout, but also the largest helper.

The local HTML gallery puts transparent PNGs on white backgrounds. Transparency shown as black by some image previews is not itself a renderer defect. We have not performed pixel/semantic comparison against Mermaid.js, a broad compatibility corpus, full linguistic validation of mixed Arabic/Hebrew, dark-theme integration, or viewer accessibility/selection tests.

## Focused follow-up: ordinary syntax and smaller graphs

An additional 17-fixture corpus was run once each through Rusty and Merman (34 process executions). The fixtures cover chains and fans of 25, 50, 100, 250 and 500 edges, a cycle, a README-like flowchart, nested subgraphs, an aliased/autonumbered sequence, and three small malformed inputs. Both rendered all fourteen valid cases without aborting or timing out. This means successful output, not full fidelity. Rusty's 500-edge chain took about 81 ms; its 500-edge fan took about 505 ms, dominated by rasterization. These are single observations, not repeat-run medians or a universal safe size threshold.

Visual inspection of the README-like and sequence samples found readable output. **The nested-subgraph sample places Parse outside Reader even though the source places it inside**, while still inside App. Rusty also accepts unclosed quoted/bracketed labels and renders a single A node; Merman rejects those inputs. Both reject the unfinished edge.

This narrows the conclusion: the 2,000-edge crash alone should not disqualify Rusty for ordinary reading. Small-input parser and grouping defects are more relevant release blockers. Proceeding with a constrained experimental integration is reasonable, but shipping should wait for these defects to be fixed or the affected syntax to be explicitly rejected with source fallback. No measured prevalence of 2,000-edge Markdown diagrams is available.

Reproduce with `generate-focused.ps1`, then `run.ps1 -Variants rusty-text,merman-text -CorpusDirectory experiments/mermaid-spike/focused-corpus -OutputDirectory target/mermaid-focused`. [Raw focused results](../experiments/mermaid-spike/results-focused-2026-09-10.csv).

## Other approaches (source inspection)

[Ferrite](https://github.com/OlaProeis/Ferrite) was inspected at commit `3ba085c561670342d72c560efbf6b0b92b5c0b46`. Its renderer lives inside `src/markdown/mermaid`, imports egui and application diagnostics, and targets egui 0.34 while our viewer uses 0.36. It was **not benchmarked** or extracted. Reusing it is a porting project, not currently a standalone dependency replacement. With Rusty's smaller working SVG path available, extracting an editor subsystem is not the first choice for this comparison.

Rusty's renderer-independent Scene still offers a possible future direct-egui backend. This comparison uses its supplied SVG backend, so no unmeasured direct-drawing savings are included. [Beautiful Mermaid](https://github.com/lukilabs/beautiful-mermaid) is TypeScript and was not included in the native Rust benchmark; its runtime integration would be a different experiment.

## Evidence and reproduction

- [All measured executions](../experiments/mermaid-spike/results-comparison-2026-09-10.csv)
- [Executable sizes and SHA-256 hashes](../experiments/mermaid-spike/binaries-comparison-2026-09-10.json)
- [Harness and commands](../experiments/mermaid-spike/README.md)
- Local gallery: `target/mermaid-comparison/index.html` (generated, not committed)
- Primary library sources: [Rusty Mermaid](https://github.com/base58ed/rusty-mermaid), [Selkie](https://github.com/btucker/selkie), [mmdr](https://github.com/1jehuang/mermaid-rs-renderer), [Merman](https://github.com/Latias94/merman)

Release builds succeeded for all four text variants and the baseline. Clippy with warnings denied passed for the newly added Rusty and Selkie variants; formatting and whitespace checks passed. The application manifest/lockfile and published release are unchanged, so the production test suite was not rerun. No upstream issue or release was published.
