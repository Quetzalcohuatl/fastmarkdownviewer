# Measured desktop resources

Five samples per cell unless the count says otherwise. See [method/environment](environment.md) and [feature coverage](features.md). Window proxy is NOT first-content latency. Memory is process-tree RSS/working set at the fixed checkpoint; shared pages may be counted repeatedly. CPU is ms per second (1000 = one core). Live at checkpoint means a window and surviving process tree (plus an Electron renderer where checked), not successful whole-document rendering.

## Windows

| Fixture | App | Live at checkpoint / attempted | Window proxy median ms (range) | RSS median MiB | Private median MiB | CPU median ms/s |
|---|---|---:|---:|---:|---:|---:|
| gfm-math-images-100k.md | aydiler | 5/5 | 20.9 (19–25) | 257.1 | 352.7 | 0.0 |
| gfm-math-images-100k.md | fmv | 5/5 | 18.7 (18–25) | 167.2 | 195.5 | 1005.7 |
| gfm-math-images-100k.md | markmello | 5/5 | 233.9 (189–254) | 288.3 | 345.6 | 0.0 |
| gfm-math-images-100k.md | marktext | 5/5 | 412.0 (398–533) | 636.9 | 463.5 | 0.0 |
| gfm-math-images-100k.md | md-preview | 5/5 | 19.3 (18–22) | 628.9 | 451.9 | 93.0 |
| gfm-math-images-100k.md | mdview-zig | 5/5 | 19.3 (19–22) | 46.8 | 62.0 | 0.0 |
| gfm-math-images-100k.md | obsidian | 5/5 | 391.3 (391–397) | 794.8 | 784.7 | 0.0 |
| gfm-math-images-100k.md | typora | 5/5 | 281.3 (273–814) | 742.8 | 606.7 | 15.5 |
| gfm-math-images-100k.md | vscode-preview | 5/5 | 291.6 (289–303) | 2311.9 | 1988.2 | 1843.0 |
| images.md | aydiler | 5/5 | 23.3 (20–37) | 214.7 | 281.7 | 0.0 |
| images.md | fmv | 5/5 | 21.3 (20–36) | 91.4 | 203.9 | 991.2 |
| images.md | markmello | 5/5 | 238.7 (231–244) | 118.6 | 154.1 | 0.0 |
| images.md | marktext | 5/5 | 498.0 (467–525) | 464.9 | 293.0 | 0.0 |
| images.md | md-preview | 5/5 | 19.9 (19–37) | 378.2 | 188.0 | 15.5 |
| images.md | mdview-zig | 5/5 | 20.1 (19–22) | 45.0 | 61.0 | 0.0 |
| images.md | obsidian | 5/5 | 487.5 (399–501) | 419.2 | 363.8 | 0.0 |
| images.md | typora | 5/5 | 358.6 (328–373) | 600.3 | 453.9 | 15.5 |
| images.md | vscode-preview | 5/5 | 370.5 (345–424) | 1906.2 | 1594.7 | 1671.3 |
| ordinary-5k.md | aydiler | 5/5 | 19.1 (18–22) | 228.0 | 315.2 | 0.0 |
| ordinary-5k.md | fmv | 5/5 | 18.7 (18–21) | 134.8 | 161.0 | 961.3 |
| ordinary-5k.md | markmello | 5/5 | 195.7 (189–201) | 137.1 | 177.1 | 0.0 |
| ordinary-5k.md | marktext | 5/5 | 422.3 (395–480) | 477.8 | 305.0 | 0.0 |
| ordinary-5k.md | md-preview | 5/5 | 18.9 (19–20) | 417.5 | 236.4 | 15.5 |
| ordinary-5k.md | mdview-zig | 5/5 | 18.7 (19–23) | 45.6 | 60.5 | 0.0 |
| ordinary-5k.md | obsidian | 5/5 | 389.6 (372–424) | 467.3 | 439.4 | 15.5 |
| ordinary-5k.md | typora | 5/5 | 286.1 (270–312) | 607.6 | 470.5 | 15.5 |
| ordinary-5k.md | vscode-preview | 5/5 | 304.7 (287–307) | 1916.3 | 1582.1 | 1393.0 |
| stress-2m.md | aydiler | 5/5 | 18.9 (19–21) | 986.8 | 1087.9 | 0.0 |
| stress-2m.md | fmv | 5/5 | 18.6 (18–305) | 642.2 | 701.9 | 1005.1 |
| stress-2m.md | markmello | 5/5 | 261.7 (201–276) | 812.5 | 806.4 | 1019.0 |
| stress-2m.md | marktext (blank pilot) | 5/5 | 500.7 (376–534) | 496.7 | 328.5 | 1004.5 |
| stress-2m.md | md-preview | 0/5 | — | — | — | — |
| stress-2m.md | mdview-zig | 5/5 | 19.6 (18–21) | 51.4 | 68.7 | 0.0 |
| stress-2m.md | obsidian | 0/5 | — | — | — | — |
| stress-2m.md | typora (file rejected; excluded) | 5/5 | — | — | — | — |
| stress-2m.md | vscode-preview | 5/5 | 302.9 (288–305) | 3879.7 | 3533.5 | 3399.8 |

Typora explicitly rejected the stress file in a separate pilot; its raw readings describe an error screen and are excluded. Rows marked “blank pilot” retain process-resource measurements, but the separate large-file spot check showed no document content; do not treat them as completed-render memory or speed results. See [observations](observations.md).

## Linux

| Fixture | App | Live at checkpoint / attempted | Window proxy median ms (range) | RSS median MiB | Private median MiB | CPU median ms/s |
|---|---|---:|---:|---:|---:|---:|
| gfm-math-images-100k.md | aydiler | 5/5 | 299.4 (288–353) | 339.5 | 329.9 | 0.0 |
| gfm-math-images-100k.md | fmv | 5/5 | 118.1 (110–125) | 192.8 | 183.9 | 1473.7 |
| gfm-math-images-100k.md | markmello | 5/5 | 280.8 (265–331) | 361.3 | 356.0 | 0.0 |
| gfm-math-images-100k.md | marktext | 5/5 | 622.2 (546–1009) | 929.1 | 203.8 | 9.9 |
| gfm-math-images-100k.md | marky | 5/5 | 83.1 (68–94) | 790.0 | 502.0 | 839.2 |
| gfm-math-images-100k.md | md-preview | 5/5 | 71.4 (58–117) | 738.0 | 452.8 | 722.9 |
| gfm-math-images-100k.md | mdview-zig | 0/5 | — | — | — | — |
| gfm-math-images-100k.md | obsidian (keyring setup; excluded) | 5/5 | — | — | — | — |
| gfm-math-images-100k.md | typora | 5/5 | 298.2 (281–803) | 881.4 | 422.0 | 0.0 |
| gfm-math-images-100k.md | vscode-preview | 5/5 | 719.2 (448–930) | 2249.9 | 571.5 | 117.7 |
| images.md | aydiler | 5/5 | 370.1 (317–407) | 307.5 | 299.4 | 0.0 |
| images.md | fmv | 5/5 | 139.8 (111–170) | 165.5 | 158.4 | 2128.4 |
| images.md | markmello | 5/5 | 298.4 (278–450) | 224.3 | 218.8 | 0.0 |
| images.md | marktext | 5/5 | 608.1 (575–720) | 744.2 | 204.8 | 9.9 |
| images.md | marky | 5/5 | 106.0 (64–121) | 668.6 | 377.9 | 815.4 |
| images.md | md-preview | 5/5 | 100.9 (90–110) | 560.5 | 277.4 | 880.2 |
| images.md | mdview-zig | 0/5 | — | — | — | — |
| images.md | obsidian (keyring setup; excluded) | 5/5 | — | — | — | — |
| images.md | typora | 5/5 | 351.2 (330–410) | 954.9 | 342.4 | 19.5 |
| images.md | vscode-preview | 5/5 | 525.0 (473–543) | 2029.4 | 514.7 | 19.5 |
| ordinary-5k.md | aydiler | 5/5 | 330.4 (278–407) | 295.1 | 287.5 | 0.0 |
| ordinary-5k.md | fmv | 5/5 | 121.8 (94–144) | 180.9 | 173.6 | 2180.1 |
| ordinary-5k.md | markmello | 5/5 | 307.0 (257–325) | 232.1 | 227.1 | 0.0 |
| ordinary-5k.md | marktext | 5/5 | 543.0 (471–895) | 740.6 | 206.1 | 9.9 |
| ordinary-5k.md | marky | 5/5 | 97.2 (74–107) | 699.7 | 411.2 | 771.6 |
| ordinary-5k.md | md-preview | 5/5 | 88.0 (71–169) | 602.0 | 319.5 | 654.2 |
| ordinary-5k.md | mdview-zig | 0/5 | — | — | — | — |
| ordinary-5k.md | obsidian (keyring setup; excluded) | 5/5 | — | — | — | — |
| ordinary-5k.md | typora | 5/5 | 628.0 (328–800) | 800.0 | 338.9 | 9.9 |
| ordinary-5k.md | vscode-preview | 5/5 | 475.2 (419–538) | 2063.0 | 535.3 | 137.0 |
| stress-2m.md | aydiler | 5/5 | 1247.0 (1142–1441) | 1070.5 | 1044.8 | 0.0 |
| stress-2m.md | fmv | 5/5 | 394.6 (349–447) | 427.3 | 366.7 | 1024.6 |
| stress-2m.md | markmello (blank pilot) | 5/5 | 306.7 (270–842) | 1123.9 | 1118.8 | 992.1 |
| stress-2m.md | marktext (blank pilot) | 5/5 | 1037.9 (542–1933) | 1305.1 | 204.4 | 9.9 |
| stress-2m.md | marky | 5/5 | 72.5 (57–115) | 1460.6 | 1172.0 | 1263.0 |
| stress-2m.md | md-preview | 5/5 | 75.4 (62–141) | 1245.7 | 967.0 | 1075.7 |
| stress-2m.md | mdview-zig | 0/5 | — | — | — | — |
| stress-2m.md | obsidian (keyring setup; excluded) | 0/5 | — | — | — | — |
| stress-2m.md | typora (file rejected; excluded) | 5/5 | — | — | — | — |
| stress-2m.md | vscode-preview (blank pilot) | 5/5 | 437.4 (415–529) | 2549.6 | 1089.5 | 684.3 |

Typora explicitly rejected the stress file in a separate pilot; its raw readings describe an error screen and are excluded. Rows marked “blank pilot” retain process-resource measurements, but the separate large-file spot check showed no document content; do not treat them as completed-render memory or speed results. See [observations](observations.md).

Obsidian Linux resource samples remain in the raw CSV but are excluded above: a keyring-creation prompt appeared over the reading view in the pilot. No credentials were entered or keyring settings changed. This is an environment/setup limitation, not an app rendering failure.

## Windows CLI conversion (separate workload)

Output captured through a pipe; no terminal painting, browser or GUI measured. Success requires exit code zero and the expected fixture heading in output after stripping ANSI sequences. These timings must not be ranked against desktop window proxies.

| Fixture | App | Successful/attempted | Process-to-exit median ms | Range ms |
|---|---|---:|---:|---:|
| gfm-math-images-100k.md | glow | 10/10 | 257.1 | 233.0–270.4 |
| gfm-math-images-100k.md | mdcat | 10/10 | 55.5 | 52.1–64.3 |
| images.md | glow | 10/10 | 52.4 | 43.4–56.2 |
| images.md | mdcat | 10/10 | 11.6 | 9.5–16.8 |
| ordinary-5k.md | glow | 10/10 | 63.8 | 54.8–84.1 |
| ordinary-5k.md | mdcat | 10/10 | 40.9 | 35.9–58.3 |
| stress-2m.md | glow | 10/10 | 5305.3 | 5150.9–5382.6 |
| stress-2m.md | mdcat | 10/10 | 374.0 | 362.5–409.7 |

## Linux CLI conversion (separate workload)

Output captured through a pipe; no terminal painting, browser or GUI measured. Success requires exit code zero and the expected fixture heading in output after stripping ANSI sequences. These timings must not be ranked against desktop window proxies.

| Fixture | App | Successful/attempted | Process-to-exit median ms | Range ms |
|---|---|---:|---:|---:|
| gfm-math-images-100k.md | glow | 10/10 | 234.8 | 219.1–275.6 |
| gfm-math-images-100k.md | mdcat | 10/10 | 43.7 | 38.3–61.2 |
| images.md | glow | 10/10 | 17.1 | 12.8–20.5 |
| images.md | mdcat | 10/10 | 3.9 | 3.3–13.3 |
| ordinary-5k.md | glow | 10/10 | 29.4 | 23.5–34.6 |
| ordinary-5k.md | mdcat | 10/10 | 28.2 | 24.8–59.1 |
| stress-2m.md | glow | 10/10 | 5102.8 | 4946.8–5452.2 |
| stress-2m.md | mdcat | 10/10 | 360.8 | 340.2–420.8 |
