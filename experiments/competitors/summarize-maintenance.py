"""Audit and summarize the September 13 Windows maintenance rerun.

Run from the repository root after measure.py completes all 100 attempts.
No samples or failures are silently discarded.
"""
import collections
import csv
from pathlib import Path
import statistics

DIRECTORY = Path(__file__).resolve().parent
APPS = {
    'fmv-0.2.1': 'FastMarkdownViewer 0.2.1 (published)',
    'fmv': 'FastMarkdownViewer maintenance build (dda53cf)',
    'aydiler': 'aydiler/md-viewer 0.2.0',
    'markmello': 'MarkMello 0.4.0',
    'md-preview': 'MD Preview 1.4.1',
    'marktext': 'MarkText 0.19.1',
    'mdview-zig': 'mdview-zig 0.2.0',
    'obsidian': 'Obsidian 1.13.7',
    'typora': 'Typora 1.14.10',
    'vscode-preview': 'VS Code 1.130.0 (Markdown preview)',
}
FIXTURES = ['ordinary-5k.md', 'gfm-math-images-100k.md']


def summarize():
    with (DIRECTORY / 'maintenance-windows.csv').open(newline='') as stream:
        rows = list(csv.DictReader(stream))
    groups = collections.defaultdict(list)
    for row in rows:
        assert row['app'] in APPS and row['fixture'] in FIXTURES, row
        groups[row['app'], row['fixture']].append(row)
    assert len(rows) == 100, f'Expected 100 attempts, found {len(rows)}'
    for app in APPS:
        assert len({r['exe_sha256'] for r in rows if r['app'] == app}) == 1
        for fixture in FIXTURES:
            assert sorted(int(r['run']) for r in groups[app, fixture]) == [1, 2, 3, 4, 5]
    for fixture in FIXTURES:
        assert len({r['fixture_sha256'] for r in rows if r['fixture'] == fixture}) == 1

    def metric(samples, field):
        values = [float(r[field]) for r in samples]
        return f'{statistics.median(values):.1f} ({min(values):.1f}–{max(values):.1f})' if values else '—'

    lines = [
        '# Windows resource rerun — September 13, 2026', '',
        'Five fresh processes per app and fixture, randomized together in one batch.',
        'All 100 attempts are retained in [the raw CSV](maintenance-windows.csv).',
        'See [the run record](maintenance-method.md) for build provenance, versions,',
        'profiles, hardware, reproduction, and limitations. Parentheses show ranges.', '',
    ]
    for fixture in FIXTURES:
        lines += [f'## {fixture}', '',
                  '| App | Window observed / attempted | Working set MiB | Private memory MiB | Checkpoint CPU ms/s | Window proxy ms |',
                  '| --- | ---: | ---: | ---: | ---: | ---: |']
        for app, label in APPS.items():
            samples = groups[app, fixture]
            valid = [r for r in samples if r['status'] == 'window-observed']
            cells = [metric(valid, f) for f in ['checkpoint_rss_mib', 'checkpoint_private_mib', 'cpu_ms_per_second', 'window_proxy_ms']]
            lines.append(f'| {label} | {len(valid)}/{len(samples)} | ' + ' | '.join(cells) + ' |')
        lines.append('')
    lines += ['## Audit', '', 'Executable SHA-256 values:', '', '| App | SHA-256 |', '| --- | --- |']
    for app, label in APPS.items():
        digest = next(r['exe_sha256'] for r in rows if r['app'] == app)
        lines.append(f'| {label} | `{digest}` |')
    lines += ['', 'Fixture SHA-256 values:', '']
    for fixture in FIXTURES:
        digest = next(r['fixture_sha256'] for r in rows if r['fixture'] == fixture)
        lines.append(f'- `{fixture}`: `{digest}`')
    failures = [r for r in rows if r['status'] != 'window-observed']
    lines += ['', f'Non-success attempts: {len(failures)}. A visible, live window does not establish that all content finished rendering.', '']
    (DIRECTORY / 'maintenance-results.md').write_text('\n'.join(lines), encoding='utf-8')
    print(f'Audited {len(rows)} attempts; {len(failures)} non-success attempts.')


if __name__ == '__main__':
    summarize()
