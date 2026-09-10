FastMarkdownViewer v0.1.5 adds offline Mermaid diagrams without a browser engine or an external diagram service.

- Mermaid fences render above their original searchable, selectable, copyable source.
- Rendering uses patched Rusty in a hidden child process. Unsupported syntax, rendering failures, and oversized diagrams show an error with the source retained.
- Fixes include malformed flowchart labels, nested-subgraph membership, and shaped mindmap root labels. The previously crashing 2,000-edge chain is rejected before layout.
- All three Mermaid examples in torturetest3.md render. PlantUML, Graphviz, and active HTML/CSS rendering are not added.

This is a supported subset, not full Mermaid.js compatibility. Diagrams currently use a white canvas and Rusty's default palette. Limits include 64 KiB source, 512 flowchart vertices, 1,024 edges, 32 subgraphs, 4-megapixel images, and a 10-second timeout. Diagram caches are limited to 32 entries and 32 MiB per tab. Temporary rendering files are cleaned up after completion; abrupt application termination can leave them behind.

Validation: 78 automated tests passed locally, including executable rendering and error handling, plus actual-window visual checks. The preview EXE was about 19 MB, approximately 2.1 MB larger than v0.1.4.

Download the Windows x64 portable EXE, portable ZIP, or per-user installer below. Builds remain unsigned; verify SHA256SUMS.txt and GitHub provenance attestations. See PRIVACY.md for network and temporary-file behavior.
