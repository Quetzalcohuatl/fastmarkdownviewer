"""Fresh-process CPU layout and tab lifecycle measurements, excluding GPU/presentation."""
import argparse
import csv
import hashlib
import random
import subprocess
from pathlib import Path

parser = argparse.ArgumentParser(__doc__)
parser.add_argument('--baseline', type=Path, required=True)
parser.add_argument('--candidate', type=Path, required=True)
parser.add_argument('--fixtures', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--runs', type=int, default=5)
args = parser.parse_args()
jobs = [(variant, fixture, run) for variant in ['baseline', 'candidate']
        for fixture in ['ordinary-5k.md', 'gfm-math-images-100k.md', 'stress-2m.md', 'images.md']
        for run in range(1, args.runs + 1)]
random.Random(20261001).shuffle(jobs)
args.output.parent.mkdir(parents=True, exist_ok=True)
with args.output.open('w', newline='', encoding='utf-8') as output:
    writer = csv.DictWriter(output, fieldnames=['variant', 'fixture', 'run', 'binary_sha256',
                                               'fixture_sha256', 'scroll_median_ms', 'scroll_p95_ms',
                                               'final_scroll_offset'])
    writer.writeheader()
    for variant, fixture, run in jobs:
        binary = getattr(args, variant).resolve()
        path = (args.fixtures / fixture).resolve()
        result = subprocess.run([str(binary), str(path)], input='\n\n\n', capture_output=True,
                                text=True, timeout=120, check=True)
        values = next(line.removeprefix('SCROLL ') for line in result.stdout.splitlines()
                      if line.startswith('SCROLL ')).split(',')
        assert float(values[2]) > 0, 'Workload did not scroll'
        writer.writerow(dict(variant=variant, fixture=fixture, run=run,
                             binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                             fixture_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                             scroll_median_ms=values[0], scroll_p95_ms=values[1],
                             final_scroll_offset=values[2]))
        output.flush()
print(f'Recorded {len(jobs)} fresh-process layout runs.')
