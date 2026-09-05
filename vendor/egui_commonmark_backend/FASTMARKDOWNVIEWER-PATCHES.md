# FastMarkdownViewer patches

This directory contains `egui_commonmark_backend` 0.25.0 under its original `MIT OR Apache-2.0` license.

The local patch is intentionally narrow:

1. pass collected image alt text to `egui::Image`, which displays it on failure and exposes it to accessibility;
2. keep plain code lines unwrapped inside a horizontal scroll area.

No syntax-highlighting feature is enabled. Replace this directory with an upstream release after equivalent changes are released. Until an upstream repository and pull request URL exist, this file is the exact patch record; do not claim the patch has been submitted.
