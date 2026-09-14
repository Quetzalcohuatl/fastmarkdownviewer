# FastMarkdownViewer in 20 seconds

[Watch the demo](https://quetzalcohuatl.github.io/fastmarkdownviewer/#demo) · [Download MP4](https://quetzalcohuatl.github.io/fastmarkdownviewer/demo/fastmarkdownviewer-20s.mp4)

Open a file. Find what matters. Read the details.

## Transcript

| Time | Main caption | What you see |
|:--|:--|:--|
| 0–5 s | **Open a file. Start reading.** | A Markdown document with headings, an outline, a table, code, and a checklist. Clean Markdown; no editor to set up. |
| 5–10 s | **Find the line you need.** | The Find field contains `release`. Three matches are highlighted in context. |
| 10–16 s | **Code. Math. Diagrams.** | Highlighted Rust code, a rendered equation, and a Mermaid flowchart in the same document. |
| 16–20 s | **Try FastMarkdownViewer.** | Free, open source, and no account required. Windows, macOS, and Ubuntu; see installation instructions for requirements and signing status. |

## Format and accessibility

- Exactly 20 seconds; 1600 × 1000; 24 fps; H.264 MP4 with fast-start metadata.
- Silent, with large captions embedded in the video and an optional English WebVTT track.
- Native player controls; no autoplay, animation loop, JavaScript, or analytics.
- Plain-text transcript above and on the landing page. Timing is also available in `timeline.json`.

## Capture provenance

This is an **edited walkthrough of real renderer captures**, not a continuous
screen recording or a startup benchmark. No application UI was drawn or generated.
The source PNGs are unedited framebuffer captures; the video scales them and adds
captions, a progress line, and a closing card outside the application view.

Captured on Windows 11 at 150% display scale using the optimized `visual_check`
example and production `ViewerApp` at v0.2.2 (b5c4ed01).
The capture harness embeds the same renderer as the release; it is not the
published portable executable. The documents are synthetic fixtures included here.
Source hashes and asset hashes are in `provenance.json`.
Mermaid support covers a documented subset; this simple flowchart demonstrates
supported content. Reading/checklist content stays read-only.

## Reproduce

From the repository root on Windows, with Rust, Pillow, FFmpeg (libx264), and the
standard Segoe UI fonts installed:

```powershell
cargo build --locked --release --example visual_check
$env:FMV_VISUAL_THEME = 'Light'
target/release/examples/visual_check.exe docs/demo/project-notes.md docs/demo/reading.png
target/release/examples/visual_check.exe docs/demo/project-notes.md docs/demo/find.png release
target/release/examples/visual_check.exe docs/demo/technical-guide.md docs/demo/technical.png
Remove-Item Env:FMV_VISUAL_THEME
python scripts/build-demo.py --ffmpeg C:/path/to/ffmpeg.exe
```

Use the v0.2.2 application source when reproducing the published captures.
The compositor recreates the MP4, poster, and timing file; if captures or fixtures
change, refresh their hashes and source metadata in `provenance.json` too.
