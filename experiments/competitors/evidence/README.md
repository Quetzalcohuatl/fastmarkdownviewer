# Linux small-document pilot screenshots

These screenshots verify a separate pilot launch, not every timed sample or full
document completion. All content is the synthetic `ordinary-5k.md` fixture. The
Xfce desktop uses software graphics in the VM described in [environment.md](../environment.md).

| App | Screenshot | Observed limitation |
|---|---|---|
| FastMarkdownViewer | [View](linux-fmv.png) | Small-fixture spot check only |
| aydiler/md-viewer | [View](linux-aydiler.png) | Small-fixture spot check only |
| MarkMello | [View](linux-markmello.png) | Math absent; task items ordinary bullets |
| Marky | [View](linux-marky.png) | Math and alert marker remained source |
| MarkText | [View](linux-marktext.png) | This display-math delimiter variant and alert marker remained source |
| MD Preview | [View](linux-md-preview.png) | Small-fixture spot check only |
| Obsidian | [View](linux-obsidian.png) | Keyring setup over reading view; excluded from Linux resource comparison |
| Typora | [View](linux-typora.png) | Math remained source with default configuration |
| VS Code preview | [View](linux-vscode-preview.png) | Task and alert markers remained source |

mdview-zig crashed; a desktop-only image is not useful rendering evidence.

## Stress-document pilots

Linux captures were requested approximately 12 seconds after launching each pilot
harness. They are separate from timed samples and show only the top viewport.

| App | Linux screenshot | Observation |
|---|---|---|
| FMV | [View](linux-fmv-stress.png) | Content, math and highlighted code visible |
| aydiler | [View](linux-aydiler-stress.png) | Content, math and highlighted code visible |
| MarkMello | [View](linux-markmello-stress.png) | Window frame with unpainted content |
| Marky | [View](linux-marky-stress.png) | Content visible; math literal and code plain |
| MD Preview | [View](linux-md-preview-stress.png) | Content visible; math and code formatting incomplete |
| MarkText | [View](linux-marktext-stress.png) | Blank content at capture |
| Typora | [View](linux-typora-stress.png) | Explicit file-size rejection |
| VS Code preview | [View](linux-vscode-preview-stress.png) | Blank preview at capture |

Windows Typora also explicitly rejected the file: [screenshot](windows-typora-stress.jpg).
Blank captures are time-bounded observations, not proof of permanent failure.

## Portable image fixture

After replacing machine-specific links and rerunning the image measurements, Linux
pilots displayed the relative SVG image in [FMV](images-fmv.png) and
[aydiler](images-aydiler.png). These captures verify the visible image, not that
all 24 images were decoded at the resource checkpoint.
