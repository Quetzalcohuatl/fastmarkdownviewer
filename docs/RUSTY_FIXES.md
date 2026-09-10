# Rusty Mermaid local fixes — 2026-09-10

The known everyday defects are fixed in a local copy of `rusty-mermaid-diagrams` 0.2.0. The known oversized-chain crash is prevented by rejecting the diagram before layout. These changes are integrated into FastMarkdownViewer v0.1.5.

| Problem | Behavior after the fix |
|---|---|
| Missing bracket or closing quote silently became a bare node | Returns a parse error; no misleading partial image |
| Later reference moved a node out of its nested group | Keeps the original group; the regression also checks the node's rendered bounds lie inside it |
| Mindmap root displayed `root((Markdown viewer))` | Displays `Markdown viewer` with the requested circle shape |
| 2,000-edge chain overflowed the stack | Returns a resource-limit error before recursive layout |

The flowchart policy allows up to 512 vertices, 1,024 edges and 32 subgraphs. The subgraph limit is checked before recursive parsing. Chains and fans with 500 edges still render. These limits are conservative application policy, not a proof that the underlying recursive algorithms cannot fail; a production integration should retain a helper process and cancellation.

## Validation

- All **1,004 library unit tests passed**. Five new regressions cover the fixes. Three upstream tests intentionally asserted permissive malformed-label behavior; they were updated to match the requested strict behavior.
- All **37 corpus executions** had the expected result: valid cases rendered, malformed and oversized cases returned errors; no aborts or timeouts. [Main corpus](../experiments/mermaid-spike/results-fixed-full-2026-09-10.csv), [focused corpus](../experiments/mermaid-spike/results-fixed-focused-2026-09-10.csv).
- Inspected the corrected nested-subgraph and mindmap PNGs, confirming the repaired grouping and labels visually.
- Optimized helper build succeeded; the helper is **3,478,016 bytes**, essentially unchanged from the prior 3,480,064-byte build. Extra size against the 1,299,968-byte baseline remains about **2.08 MiB**. This is not a final viewer binary measurement.
- Harness Clippy with warnings denied passed. The vendored upstream library still has existing unused-import/dead-code warnings. Its whole upstream integration/visual-golden suite was not run; the full unit suite and our external corpora were run.

The [vendored README](../experiments/mermaid-spike/vendor/rusty-mermaid-diagrams/README.md) records provenance, the intentional behavior changes, and the limits.

## Viewer integration

Mermaid code fences render a diagram above their original searchable, selectable source. Other code fences are unchanged. Unsupported or malformed input shows a readable error while retaining the source. Diagrams use a white canvas with Rusty's default palette, including when the viewer uses a dark theme.

The same EXE starts a hidden rendering child process, with one render at a time and a 10-second timeout. A renderer crash does not run on the viewer's UI thread. Source is limited to 64 KiB, generated SVG to 10 MiB, and output images to 4 megapixels. Each tab caches at most 32 diagrams and 32 MiB of image pixels until reload or close. These are application limits, not a hard cap on worker memory. Temporary files are cleaned after the worker completes or is killed.

The executable integration tests exercise valid PNG output, malformed syntax, excessive input, and the previously crashing 2,000-edge chain. A renderer callback regression checks that Mermaid source stays in document navigation and that ordinary code is retained. Actual window screenshots cover diagrams and error fallback with Solarized Light.
