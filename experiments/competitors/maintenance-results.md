# Windows resource rerun — September 13, 2026

Five fresh processes per app and fixture, randomized together in one batch.
All 100 attempts are retained in [the raw CSV](maintenance-windows.csv).
See [the run record](maintenance-method.md) for build provenance, versions,
profiles, hardware, reproduction, and limitations. Parentheses show ranges.

## ordinary-5k.md

| App | Window observed / attempted | Working set MiB | Private memory MiB | Checkpoint CPU ms/s | Window proxy ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| FastMarkdownViewer 0.2.1 (published) | 5/5 | 127.6 (127.3–194.1) | 155.1 (154.2–206.3) | 0.0 (0.0–0.0) | 21.4 (19.5–230.5) |
| FastMarkdownViewer maintenance build (dda53cf) | 5/5 | 127.2 (127.0–127.5) | 154.7 (154.0–155.9) | 0.0 (0.0–0.0) | 21.2 (20.3–21.6) |
| aydiler/md-viewer 0.2.0 | 5/5 | 227.8 (227.5–238.9) | 314.6 (314.4–315.1) | 0.0 (0.0–0.0) | 39.1 (21.0–39.9) |
| MarkMello 0.4.0 | 5/5 | 137.5 (136.5–138.3) | 191.4 (190.9–191.8) | 0.0 (0.0–0.0) | 262.4 (262.2–324.8) |
| MD Preview 1.4.1 | 5/5 | 403.7 (403.6–409.6) | 229.0 (228.1–229.3) | 0.0 (0.0–15.5) | 36.6 (20.8–38.2) |
| MarkText 0.19.1 | 5/5 | 496.9 (492.8–511.2) | 382.2 (378.2–384.8) | 15.5 (0.0–15.5) | 483.4 (434.1–669.3) |
| mdview-zig 0.2.0 | 5/5 | 44.5 (44.4–45.4) | 60.9 (59.2–61.9) | 0.0 (0.0–15.5) | 20.5 (20.1–24.1) |
| Obsidian 1.13.7 | 5/5 | 462.0 (458.7–465.2) | 440.5 (436.2–445.4) | 0.0 (0.0–61.9) | 480.2 (421.9–770.3) |
| Typora 1.14.10 | 5/5 | 597.2 (594.4–602.8) | 466.9 (465.9–468.3) | 15.5 (0.0–30.9) | 345.2 (306.6–387.6) |
| VS Code 1.130.0 (Markdown preview) | 5/5 | 1944.7 (1884.4–1959.6) | 1638.3 (1555.2–1678.3) | 603.0 (216.6–1638.8) | 300.5 (281.4–334.5) |

## gfm-math-images-100k.md

| App | Window observed / attempted | Working set MiB | Private memory MiB | Checkpoint CPU ms/s | Window proxy ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| FastMarkdownViewer 0.2.1 (published) | 5/5 | 138.5 (138.4–138.9) | 168.2 (167.5–168.7) | 0.0 (0.0–0.0) | 21.3 (20.3–38.1) |
| FastMarkdownViewer maintenance build (dda53cf) | 5/5 | 138.7 (137.3–148.5) | 167.9 (166.0–169.0) | 0.0 (0.0–0.0) | 21.5 (20.7–227.8) |
| aydiler/md-viewer 0.2.0 | 5/5 | 257.7 (256.8–258.5) | 347.0 (345.1–354.2) | 0.0 (0.0–0.0) | 21.1 (20.5–41.5) |
| MarkMello 0.4.0 | 5/5 | 287.4 (285.9–288.2) | 345.6 (341.6–347.6) | 0.0 (0.0–0.0) | 254.9 (212.4–322.1) |
| MD Preview 1.4.1 | 5/5 | 560.3 (528.4–562.9) | 387.2 (353.3–388.8) | 108.3 (15.5–139.3) | 22.8 (20.6–51.6) |
| MarkText 0.19.1 | 5/5 | 695.2 (692.7–697.9) | 578.0 (577.6–584.3) | 0.0 (0.0–0.0) | 537.0 (519.3–1780.5) |
| mdview-zig 0.2.0 | 5/5 | 45.6 (45.5–45.6) | 61.6 (60.0–62.1) | 0.0 (0.0–0.0) | 35.6 (20.0–38.0) |
| Obsidian 1.13.7 | 5/5 | 813.7 (809.4–815.7) | 802.8 (799.4–812.2) | 0.0 (0.0–15.4) | 448.8 (412.3–503.6) |
| Typora 1.14.10 | 5/5 | 685.8 (676.9–709.4) | 556.0 (547.7–573.5) | 30.9 (0.0–61.9) | 356.0 (331.1–609.9) |
| VS Code 1.130.0 (Markdown preview) | 5/5 | 2230.8 (2093.4–2258.4) | 1901.0 (1790.1–1927.8) | 602.5 (309.3–1033.8) | 310.8 (280.7–569.4) |

## Audit

Executable SHA-256 values:

| App | SHA-256 |
| --- | --- |
| FastMarkdownViewer 0.2.1 (published) | `f10ddcf8f482b3c50ae1df905a04c24441ceb1fb02370742f8ca38a459529d66` |
| FastMarkdownViewer maintenance build (dda53cf) | `03f5bbbbf613105a32c1d681080c69ac52730fc60af90cb707bd7dcfbdd80970` |
| aydiler/md-viewer 0.2.0 | `c5fe8c07f3cd1678bd24c1b5ab55f5374932c0cba81deff5021b23270f30e44a` |
| MarkMello 0.4.0 | `01cd62372517971d003c37cd1191d9290107e8944de6a8a586157923937880fd` |
| MD Preview 1.4.1 | `d24eb9ea9b38f1e57f40d76eec919ae319494a4764ef1e64975e50f5a4b1b10f` |
| MarkText 0.19.1 | `f783de63f5bf170e7a2ab58b3755425ddf339a6628923664957121fc779ecdfa` |
| mdview-zig 0.2.0 | `29f2b042e99deccc584ce0f44fe553320bae7682727dc2c9de9977ae59bc77be` |
| Obsidian 1.13.7 | `a00f15ad21a289d4db67df1196d5d99ec5cf31a6972f7777fb9b41e20d86088e` |
| Typora 1.14.10 | `3a1807daf4f0f9e8d1257fac5f61429e9535debf230ffece8b7d931749108a57` |
| VS Code 1.130.0 (Markdown preview) | `c933b1301979abb102ded0b5c28c5c417fcb09411b15f5ff2484c8bb896cf9ef` |

Fixture SHA-256 values:

- `ordinary-5k.md`: `0b7ad3f74990c566da95984bd29019d7c5399c2ac2995e08b1dd6c01d315731d`
- `gfm-math-images-100k.md`: `771478d23dc1d5ab205db29e23c97f4eb2e28d7a6ecf37cf87298b647bc42594`

Non-success attempts: 0. A visible, live window does not establish that all content finished rendering.
