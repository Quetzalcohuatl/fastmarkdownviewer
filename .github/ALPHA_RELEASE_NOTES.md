FastMarkdownViewer is an early Windows x64 alpha for testing the core
double-click, open, render, and scroll experience.

Known alpha limitations:

- The executable and installer are unsigned, so Windows SmartScreen may warn.
- Large documents currently use full-document layout while safe viewport
  virtualization is being developed.
- Remote images make asynchronous HTTP(S) requests under the policy documented
  in `PRIVACY.md`.
- Windows ARM64, macOS, and Linux packages are not included yet.
- There is no updater, file watcher, editor, or persistent settings database.

Please report crashes and rendering problems through GitHub Issues. Verify
downloads with `SHA256SUMS.txt` and the GitHub build-provenance attestation.
