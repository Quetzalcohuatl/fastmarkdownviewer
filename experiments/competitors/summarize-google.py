"""Audit and summarize the September 14 discovery-driven Windows cohort.

Run from any directory after measure.py completes; keeps every attempt and
reports failures separately instead of turning them into zero-cost successes.
"""
import collections
import csv
import json
from pathlib import Path
import statistics

DIRECTORY = Path(__file__).resolve().parent
APPS = {
    'fmv-0.2.1': 'FastMarkdownViewer 0.2.1',
    'aydiler': 'aydiler/md-viewer 0.2.0',
    'marklite': 'MarkLite 1.1.1 (unverified content)',
    'markmello': 'MarkMello 0.4.0',
    'markpad': 'Markpad 2.7.6',
    'marktext': 'MarkText 0.19.1',
    'md-preview': 'MD Preview 1.4.1',
    'mdview-zig': 'mdview-zig 0.2.0',
    'moji': 'Moji 1.0.7',
    'obsidian': 'Obsidian 1.13.7',
    'typora': 'Typora 1.14.10',
    'vscode-preview': 'VS Code 1.130.0, Markdown preview',
}
FIXTURES = {
    'ordinary-5k.md': '0b7ad3f74990c566da95984bd29019d7c5399c2ac2995e08b1dd6c01d315731d',
    'gfm-math-images-100k.md': '771478d23dc1d5ab205db29e23c97f4eb2e28d7a6ecf37cf87298b647bc42594',
}
NEW_HASHES = {
    'marklite': '98a5cda2c1c4b884c8c94c5b8a5a1d1749df1cecb58c2bcf1e211e73cbde4fa5',
    'markpad': '8cbde9e19bc01fad952f7f3b3276ebc003ac963483334f64f9f1c95be4b146b7',
    'moji': '38af3fbe00fe2a1f9c39dd0e681a3195e4ba26f32b1cb250385393bead04bcb5',
}


def summarize():
    with (DIRECTORY / 'google-windows.csv').open(newline='') as stream:
        rows = list(csv.DictReader(stream))
    with (DIRECTORY / 'maintenance-windows.csv').open(newline='') as stream:
        expected_hashes = {r['app']: r['exe_sha256'] for r in csv.DictReader(stream)}
    expected_hashes.update(NEW_HASHES)
    assert len(rows) == len(APPS) * len(FIXTURES) * 5, f'Incomplete batch: {len(rows)}/120'
    groups = collections.defaultdict(list)
    for row in rows:
        assert row['app'] in APPS and row['fixture'] in FIXTURES, row
        assert row['exe_sha256'] == expected_hashes[row['app']], row
        assert row['fixture_sha256'] == FIXTURES[row['fixture']], row
        assert float(row['sample_duration_s']) > 0, row
        groups[row['app'], row['fixture']].append(row)
    for app in APPS:
        for fixture in FIXTURES:
            assert sorted(int(r['run']) for r in groups[app, fixture]) == [1, 2, 3, 4, 5]

    def metric(samples, field):
        values = [float(r[field]) for r in samples if r[field] != '']
        return f'{statistics.median(values):.1f} ({min(values):.1f}–{max(values):.1f})' if values else '—'

    lines = [
        '# Expanded Windows resource results — September 14, 2026', '',
        'Five fresh launches per app/document, all 12 entries randomized in one batch.',
        'All **120 attempts** are retained in [the raw CSV](google-windows.csv).',
        '[Method, hardware and reproduction](google-method.md);',
        '[selection and unmeasured products](google-discovery.md);',
        '[rendering observations](google-observations.md); [features](google-features.md).', '',
        'Values are **median (minimum–maximum)**. Window discovery is not first-content',
        'latency. Checkpoint CPU is not settled idle or battery use. MiB = 2²⁰ bytes.', '',
        '**MarkLite is not ranked:** its window appeared, but the document area remained',
        'blank in the pilot, including a 30-second follow-up with default environment.',
        'Its ten window-only samples stay visible here for audit, not as a reading-performance',
        'verdict. The README ranks eleven apps (FMV plus ten competitors).', '',
    ]
    for fixture in FIXTURES:
        lines += [f'## {fixture}', '',
                  '| App | Window observed / attempts | Working set MiB | Private memory MiB | Checkpoint CPU ms/s | Window proxy ms |',
                  '| --- | ---: | ---: | ---: | ---: | ---: |']
        for app, label in APPS.items():
            samples = groups[app, fixture]
            valid = [r for r in samples if r['status'] == 'window-observed']
            cells = [metric(valid, f) for f in ['checkpoint_rss_mib', 'checkpoint_private_mib', 'cpu_ms_per_second', 'window_proxy_ms']]
            lines.append(f'| {label} | {len(valid)}/{len(samples)} | ' + ' | '.join(cells) + ' |')
        lines.append('')

    downloads = json.loads((DIRECTORY / 'google-downloads.json').read_text())
    lines += ['## Download and distributed app sizes', '',
              'Selected newly acquired products, with the published FMV portable binary as baseline.',
              'These are file-byte counts, not total installed footprints. Compression and packaging',
              'differ; shared runtimes and system libraries are excluded. In particular, Markpad',
              'requires WebView2 in addition to its smaller executable.', '',
              '| App | Download MiB | Distributed app files MiB | Packaging |',
              '| --- | ---: | ---: | --- |',
              '| FMV 0.2.1 | 18.2 | 18.2 | Portable executable, 19,061,248 bytes |']
    for item in downloads:
        lines.append(f"| {item['app']} {item['version']} | {item['bytes']/2**20:.1f} | {item['distributed_app_bytes']/2**20:.1f} | {item['external_runtime_note']} |")
    lines += ['', '[Exact download hashes and byte counts](google-downloads.json).', '',
              '## Audit', '', '| App | Executable SHA-256 |', '| --- | --- |']
    for app, label in APPS.items():
        lines.append(f'| {label} | `{expected_hashes[app]}` |')
    lines += ['', 'Fixture SHA-256 values:', '']
    for fixture, digest in FIXTURES.items():
        lines.append(f'- `{fixture}`: `{digest}`')
    failures = [r for r in rows if r['status'] != 'window-observed']
    lines += ['', f'Launch-status failures: **{len(failures)}**. Separately, **10 MarkLite attempts** lack a validated reading setup.', '']
    if failures:
        lines += ['| App | Fixture | Run | Status |', '| --- | --- | ---: | --- |']
        lines += [f"| {r['app']} | {r['fixture']} | {r['run']} | {r['status']} |" for r in failures]
        lines.append('')
    lines += ['A visible live window does not establish complete rendering or equivalent feature coverage.', '']
    (DIRECTORY / 'google-results.md').write_text('\n'.join(lines), encoding='utf-8')
    print(f'Audited {len(rows)} attempts; {len(failures)} non-success attempts.')


if __name__ == '__main__':
    summarize()
