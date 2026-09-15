"""Randomized CPU reading comparison of named, prebuilt compiler profiles.

Uses the existing reading_benchmark protocol; no startup or GPU measurements.
Run only after all builds have finished.
"""
import argparse
import csv
import hashlib
from pathlib import Path
import random
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary', action='append', required=True, metavar='NAME=PATH')
parser.add_argument('--fixtures', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--repetitions', type=int, default=15)
parser.add_argument('--seed', type=int, default=20260917)
args = parser.parse_args()
if args.repetitions < 1:
    parser.error('--repetitions must be positive')
binaries = {}
for item in args.binary:
    name, separator, filename = item.partition('=')
    if not separator or not name or name in binaries:
        parser.error('--binary needs a unique NAME=PATH')
    path = Path(filename).resolve(strict=True)
    binaries[name] = (path, hashlib.sha256(path.read_bytes()).hexdigest())
fixtures = {}
for filename in ['ordinary-5k.md', 'gfm-math-images-100k.md']:
    path = (args.fixtures / filename).resolve(strict=True)
    fixtures[filename] = (path, hashlib.sha256(path.read_bytes()).hexdigest())
order = [(name, fixture, repetition)
         for name in binaries for fixture in fixtures
         for repetition in range(1, args.repetitions + 1)]
random.Random(args.seed).shuffle(order)
args.output.parent.mkdir(parents=True, exist_ok=True)
with args.output.open('w', newline='', encoding='utf-8') as output:
    writer = csv.DictWriter(output, fieldnames=[
        'variant', 'fixture', 'repetition', 'binary_sha256', 'fixture_sha256',
        'scroll_median_ms', 'scroll_p95_ms', 'final_scroll_offset'])
    writer.writeheader()
    for name, fixture, repetition in order:
        binary, binary_hash = binaries[name]
        path, fixture_hash = fixtures[fixture]
        result = subprocess.run([str(binary), str(path)], input='\n\n\n',
                                capture_output=True, text=True, timeout=30, check=True)
        values = next(line.removeprefix('SCROLL ') for line in result.stdout.splitlines()
                      if line.startswith('SCROLL ')).split(',')
        writer.writerow({
            'variant': name, 'fixture': fixture, 'repetition': repetition,
            'binary_sha256': binary_hash, 'fixture_sha256': fixture_hash,
            'scroll_median_ms': values[0], 'scroll_p95_ms': values[1],
            'final_scroll_offset': values[2]})
        output.flush()
print(f'Recorded {len(order)} runs in {args.output}')
