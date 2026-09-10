# Torture-test companion assets

Generated with `python scripts/generate-torturetest-assets.py` (requires Pillow).
All images are original geometric fixtures; no upstream screenshots or binaries
are redistributed. Text examples are original. Only network.md fetches remote
images; the two root suites and other companions use local assets.

- quadrants PNG/JPEG/WebP: red/green above blue/yellow, 240 × 120 pixels.
- image space.png and café-日本語.png: identical PNG bytes, different paths.
- two-frame.gif: red then blue; FastMarkdownViewer only promises the red frame.
- viewbox.svg: 400 × 100, nonzero viewBox origin, blue rectangle/yellow circle.
- corrupt.png: deliberately invalid; missing-*.png and intentionally-missing.md
  deliberately do not exist.
- left/right same.md: independent files with the same basename; occupied.md is
  the destination collision control. Regenerate to reset manual rename changes.
- reload.md: edit a disposable copy, not the baseline fixture.
- stress.md: 160 paragraphs, long tokens, and a code payload above 256 KiB.
- unclosed-fence.md: intentionally has no closing fence, valid CommonMark at EOF.

Asset validation does not establish that the viewer passes the manual cases.
