"""Summarize repeated runner results and create a local visual comparison gallery."""
import csv
import html
import json
import statistics
from pathlib import Path

root = Path(__file__).resolve().parents[2]
out = root / "target" / "mermaid-comparison"
variants = ["mmdr-text", "rusty-text", "selkie-text", "merman-text"]
rows = []
for run in range(1, 4):
    with (out / str(run) / "results.csv").open(newline="", encoding="utf-8-sig") as file:
        for row in csv.DictReader(file):
            row["run"] = run
            rows.append(row)
destination = Path(__file__).with_name("results-comparison-2026-09-10.csv")
with destination.open("w", newline="", encoding="utf-8") as file:
    writer = csv.DictWriter(file, fieldnames=list(rows[0]))
    writer.writeheader()
    writer.writerows(rows)

summary = []
for variant in variants:
    for fixture in sorted({r["fixture"] for r in rows}):
        cases = [r for r in rows if r["renderer"] == variant and r["fixture"] == fixture]
        times = []
        for r in cases:
            values = dict(part.split("=", 1) for part in r["output"].split() if "=" in part)
            if "pipeline_ms" in values:
                times.append(float(values["pipeline_ms"]))
        summary.append(dict(renderer=variant, fixture=fixture,
                            statuses=[r["status"] for r in cases],
                            median_ms=statistics.median(times) if times else None,
                            sampled_peak_mib=max(int(r["sampled_peak_bytes"]) for r in cases)/1024**2))
(out / "summary.json").write_text(json.dumps(summary, indent=2), encoding="utf-8")

parts = ['<!doctype html><meta charset="utf-8"><title>Native Mermaid comparison</title>',
         '<style>body{font:16px system-ui;margin:24px;background:#eee;color:#222} '
         '.grid{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:12px}'
         'article{background:white;padding:12px;overflow:auto;border:1px solid #bbb}'
         'img{max-width:100%;max-height:650px;object-fit:contain;background:white}'
         'pre{white-space:pre-wrap;font-size:12px}h2{margin-top:40px}</style>',
         '<h1>Native Mermaid comparison</h1><p>Same input and SVG rasterizer. '
         'Click an image for full resolution. Successful output is not a compatibility pass. '
         'Times are medians of three fresh-process runs and exclude launch and output encoding.</p>']
for fixture in sorted({r["fixture"] for r in rows}):
    parts += [f'<h2>{html.escape(fixture)}</h2><div class="grid">']
    for variant in variants:
        row = next(r for r in rows if r["renderer"] == variant and r["fixture"] == fixture and r["run"] == 1)
        stats = next(s for s in summary if s["renderer"] == variant and s["fixture"] == fixture)
        parts += [f'<article><h3>{variant}</h3><p>{html.escape(str(stats["statuses"]))} · '
                  f'{stats["median_ms"]} ms</p>']
        png = f'1/{variant}-{Path(fixture).stem}.png'
        if row["status"] == "ok" and (out / png).exists():
            parts += [f'<a href="{png}"><img src="{png}" alt="{html.escape(fixture)}"></a>']
        else:
            parts += [f'<pre>{html.escape(row["error"])}</pre>']
        parts += ['</article>']
    parts += ['</div>']
(out / "index.html").write_text("\n".join(parts), encoding="utf-8")
print(json.dumps(summary, indent=2))
