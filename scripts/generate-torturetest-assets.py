"""Regenerate disposable manual-test assets. Requires Python 3 and Pillow.

Run from any working directory. Only writes this suite's named fixture files.
No network access or application execution is performed.
"""

from pathlib import Path

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "tests" / "fixtures" / "torturetest-assets"


def write(name: str, content: str) -> None:
    path = ASSETS / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8", newline="\n")


def main() -> None:
    ASSETS.mkdir(parents=True, exist_ok=True)
    picture = Image.new("RGB", (240, 120))
    draw = ImageDraw.Draw(picture)
    for bounds, color in [
        ((0, 0, 119, 59), "red"),
        ((120, 0, 239, 59), "green"),
        ((0, 60, 119, 119), "blue"),
        ((120, 60, 239, 119), "yellow"),
    ]:
        draw.rectangle(bounds, fill=color)
    for name in ("quadrants.png", "image space.png", "café-日本語.png"):
        picture.save(ASSETS / name)
    picture.save(ASSETS / "quadrants.jpg", quality=95)
    picture.save(ASSETS / "quadrants.webp", lossless=True)
    red = Image.new("RGB", (120, 60), "red")
    blue = Image.new("RGB", (120, 60), "blue")
    red.save(
        ASSETS / "two-frame.gif", save_all=True, append_images=[blue],
        duration=800, loop=0,
    )
    label = Image.new("RGB", (240, 60), "white")
    ImageDraw.Draw(label).text((20, 20), "PIXEL-ONLY-OTTER", fill="black")
    label.save(ASSETS / "text-label.png")
    write("corrupt.png", "Deliberately invalid PNG fixture, not image bytes.\n")
    write("viewbox.svg", '''<svg xmlns="http://www.w3.org/2000/svg" width="400" height="100" viewBox="10 20 400 100">
<rect x="10" y="20" width="400" height="100" fill="#2563eb"/>
<circle cx="60" cy="70" r="35" fill="#facc15"/>
</svg>
''')
    write("chapter one.md", """# Companion chapter

This file has no search beacon from the root document. Opening it must clear any
other tab's visual highlights while preserving each tab's own search state.

## Destination

CHAPTER-DESTINATION: a cross-document fragment should land here.

![Image relative to this chapter](quadrants.png)

[Return to torture test 2](../../../torturetest2.md#t2-15)
""")
    for side in ("left", "right"):
        write(f"{side}/same.md", f"""# {side.upper()} same-name document

Identity: {side.upper()}-DOCUMENT. A same.md file in the other folder is distinct.

![Image relative to this subfolder](../quadrants.png)

## Separate reading position

{side.upper()}-FIND-ANCHOR

[Root suite](../../../../torturetest2.md)
""" + "\n".join(f"\n{side.upper()} paragraph {i:03d}: preserve this tab's position.\n" for i in range(1, 41)))
        write(f"{side}/occupied.md", "# Existing rename destination\n\nDo not overwrite this collision fixture.\n")
    write("reload.md", "# Disposable reload sample\n\nRELOAD-VALUE-A\n" +
          "".join(f"\nReload paragraph {i:03d}: reading-position marker.\n" for i in range(1, 101)))
    token = "LONGTOKEN_" + "Q" * 1024
    fence = "".join(
        f'// row {i:05d}: ' + "abcdefghij" * 11 + "\n"
        for i in range(2400)
    )
    assert len(fence.encode("utf-8")) > 256 * 1024
    write("stress.md", """# Bounded stress sample

Manual fixture, not a benchmark. The fenced payload is intentionally over the
256 KiB highlighting cutoff. Keep every line readable without syntax coloring.

""" + "".join(
        f"Paragraph {i:03d} — This numbered paragraph must stay in order during resize, scrolling, zoom, and tab switching. "
        "The quick brown fox reads a Markdown document.\n\n"
        for i in range(1, 161)
    ) + f"## Long token\n\n{token}\n\n| Status | Token |\n|---|---|\n| Y | {token} |\n\n"
      + "## Large plain-fallback code\n\n```rust\n" + fence
      + "```\n\n## End\n\nSTRESS-END\n\n[Return](../../../torturetest2.md)\n")
    write("unclosed-fence.md", """# Unclosed fence at EOF

CommonMark permits an opening fence with no closing fence before end of file.
Everything after the next line is code, including the apparent heading.

```text
line one
line two
# Not an outline heading
UNCLOSED-FENCE-END
""")
    write("network.md", """# Optional remote-image exercise

Opening this file with automatic remote images enabled may make HTTPS requests
to raw.githubusercontent.com and example.invalid. The latter is a reserved
invalid domain and is deliberately expected to fail. A failure to reach the real
host is a network result, not by itself proof of a renderer defect.

1. Before opening this file, turn off Settings → Automatically load remote images.
2. Open it: both remote references should show individual Load image controls.
3. Click Load image on the first reference: it should load if the host is reachable.
4. Enable automatic images. The second reference should fail nonfatally with a
   useful placeholder; try Retry and confirm the document remains responsive.
5. Disable automatic images again. The already-loaded remote image should hide.
6. Repeat across a detached window to check the session-wide setting.
7. F5 reload and tab close/reopen should not show stale image state or crash.

Use a network inspector if asserting that a request did or did not occur; visual
placeholders alone cannot prove the absence of network requests. Requests already
in flight may finish after disabling the toggle, as documented in PRIVACY.md.

![Public GitHub image, network dependent](https://raw.githubusercontent.com/github/explore/main/topics/markdown/markdown.png)

![Deliberate DNS failure](https://example.invalid/torturetest2-missing.png)

NETWORK-END

[Return](../../../torturetest2.md)
""")
    write("README.md", """# Torture-test companion assets

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
""")
    print(f"Generated {len(list(ASSETS.rglob('*.*')))} companion files in {ASSETS}")


if __name__ == "__main__":
    main()
