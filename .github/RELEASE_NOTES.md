FastMarkdownViewer v0.1.2 adds navigation between local documents and tab file actions, with no new viewer dependencies.

- Local `.md` and `.markdown` links open or activate a tab in the current window. Canonical paths reuse existing tabs; browser links still open in the browser.
- Heading links support `#section` and `other.md#section`, including practical Unicode lowercase slugs, duplicate suffixes, and explicit heading IDs. Missing files or headings show a nonfatal error.
- Right-click a tab for **Rename file…** or **Show in Explorer**. Rename changes the actual filename in the current folder, preserves Markdown extensions and tab state, rejects collisions, and reloads document paths/resource metadata.
- The native Mermaid feasibility spike and corpus are documented in `docs/MERMAID_EVALUATION.md`. Rendering remains deferred because of SVG text compatibility and size costs; Mermaid fences remain readable code.

Rename requires hard-link support (such as NTFS) for overwrite-safe operation. Unsupported filesystems and case-only collisions are rejected. Rename does not rewrite other documents' links; interruption between creating the new name and removing the old one can leave both names.

All 59 tests pass. The viewer remains read-only for document contents, with no editor, file watcher, updater, telemetry, or persistent settings database.

Download the portable EXE, portable ZIP, or per-user installer below. Builds remain unsigned; verify with `SHA256SUMS.txt` and GitHub provenance attestations. Existing limitations include incomplete bidirectional text layout, monochrome emoji, and limited native accessibility in detached child windows (use a separate launch for screen-reader access). Automatic remote images remain enabled by default and can be disabled in Settings; see `PRIVACY.md`.
