"""Check retained dataset completeness and identity; does not validate rendering."""
import collections
import csv
import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent
FILES = {'ordinary-5k.md', 'gfm-math-images-100k.md', 'stress-2m.md', 'images.md'}
EXPECTED = {
    'windows.csv': ({'fmv', 'aydiler', 'markmello', 'marktext', 'md-preview', 'mdview-zig'}, 5),
    'windows-anchors.csv': ({'vscode-preview', 'obsidian', 'typora'}, 5),
    'linux.csv': ({'fmv', 'aydiler', 'markmello', 'marktext', 'marky', 'md-preview', 'mdview-zig', 'vscode-preview', 'obsidian', 'typora'}, 5),
    'windows-cli.csv': ({'glow', 'mdcat'}, 10),
    'linux-cli.csv': ({'glow', 'mdcat'}, 10),
}

def main():
    report = {'scope': 'Dataset completeness/identity only; not content correctness', 'datasets': {}}
    fixture_hashes = collections.defaultdict(set)
    for name, (apps, runs) in EXPECTED.items():
        path = ROOT / name
        rows = list(csv.DictReader(path.open()))
        identities = [(r['app'], r['fixture'], int(r['run'])) for r in rows]
        expected = {(a, f, n) for a in apps for f in FILES for n in range(1, runs + 1)}
        assert len(identities) == len(set(identities)), f'{name}: duplicate samples'
        assert set(identities) == expected, f'{name}: incomplete or unexpected sample grid'
        binaries = collections.defaultdict(set)
        for row in rows:
            for column in ['exe_sha256', 'fixture_sha256']:
                assert re.fullmatch('[0-9a-f]{64}', row[column]), f'{name}: malformed SHA-256'
            fixture_hashes[row['fixture']].add(row['fixture_sha256'])
            binaries[row['app']].add(row['exe_sha256'])
        assert all(len(values) == 1 for values in binaries.values()), f'{name}: binary changed within batch'
        status_key = 'exit_code' if '-cli' in name else 'status'
        if status_key == 'exit_code':
            assert all(r.get('content_verified') == 'True' for r in rows), f'{name}: missing fixture heading in output'
        report['datasets'][name] = {
            'samples': len(rows), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
            'status_counts': dict(collections.Counter(r[status_key] for r in rows)),
            'executable_sha256': {app: next(iter(values)) for app, values in sorted(binaries.items())},
            'negative_cpu_deltas_excluded_from_medians': sum(float(r.get('cpu_ms_per_second') or 0) < 0 for r in rows),
        }
        if status_key == 'status':
            report['datasets'][name]['live_at_checkpoint'] = sum(r['status'] == 'window-observed' and int(r['checkpoint_processes']) > 0 for r in rows)
        if name == 'linux.csv':
            report['datasets'][name]['configuration_exclusion'] = '20 Obsidian attempts retained but excluded from summarized resources because the pilot displayed keyring setup'
        if name in ['windows-anchors.csv', 'linux.csv']:
            report['datasets'][name]['document_rejection_exclusion'] = '5 Typora stress attempts retained but excluded from summarized resources because the pilot explicitly rejected the file as too large'
    assert all(len(values) == 1 for values in fixture_hashes.values()), 'Fixtures differ across platforms'
    report['fixture_sha256'] = {name: next(iter(values)) for name, values in sorted(fixture_hashes.items())}
    manifest=json.loads((ROOT/'fixture-manifest.json').read_text())
    assert len(manifest)==28 and sum(name.endswith('.svg') for name in manifest)==24
    assert all(manifest[name]['sha256']==digest for name,digest in report['fixture_sha256'].items()), 'Dataset differs from pinned fixture manifest'
    for platform in ['windows', 'linux']:
        path = ROOT / f'{platform}-late-cpu.csv'
        rows = list(csv.DictReader(path.open()))
        assert len(rows) == 2 and {r['app'] for r in rows} == {'fmv', 'aydiler'}
        assert all(int(r['processes']) > 0 and float(r['sample_s']) >= 5 for r in rows)
        report['datasets'][path.name] = {'samples': 2, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
    (ROOT / 'audit.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Verified 380 desktop attempts, 160 CLI samples and 4 diagnostic samples; consistent fixture and per-batch binary hashes.')

if __name__ == '__main__':
    main()
