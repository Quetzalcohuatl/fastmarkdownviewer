# FastMarkdownViewer local fixes

This is `rusty-mermaid-diagrams` 0.2.0, vendored from the crates.io package corresponding to upstream commit `626883102df557a105763446e086f66e6caf0716` (`crates/diagrams`). The MIT license was retrieved from that same commit. Upstream: https://github.com/base58ed/rusty-mermaid.

This copy is patched only into the isolated Mermaid experiment. It is not yet a production viewer dependency. No upstream source cache was edited.

Local changes:

- `flowchart/parser.rs`: an opening shape delimiter commits to parsing a complete label; failures propagate instead of discarding malformed input. A node already assigned to a group retains that assignment when referenced from another group. Reject more than 32 subgraphs before recursive descent.
- `flowchart/parser_tests.rs`: change three upstream tests that explicitly expected lenient malformed-label fallback to require rejection. This is an intentional local behavior change.
- `mindmap/parser.rs`: recognize IDs preceding shaped labels, including `root((Markdown viewer))`, instead of printing the syntax literally.
- `lib.rs` and `common/error.rs`: reject flowcharts with over 512 vertices or 1,024 edges before entering the recursive layout engine; provide a readable resource-limit error. Add five regressions covering malformed labels, group membership and geometric containment, mindmap shape/text, oversized chains and nested subgraphs.

The graph limits mitigate the demonstrated stack overflow; they do not replace the layout algorithm with an iterative algorithm or prove safety for every allowed graph. Direct callers of `flowchart::bridge::layout` bypass the render entry point's size check. Future viewer integration should use the guarded render API and process isolation.

Validation: 1,004 library unit tests pass, including five new regression tests. The 37 external corpus cases produce the expected successful renders or errors, with no aborts/timeouts. Existing upstream unused-import/dead-code compiler warnings remain; they were not suppressed. Harness Clippy with warnings denied passes.

Run unit tests from the experiment workspace:

```powershell
cargo test --locked --manifest-path experiments/mermaid-spike/Cargo.toml -p rusty-mermaid-diagrams --lib
```
