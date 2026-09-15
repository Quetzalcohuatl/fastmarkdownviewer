"""Compare CPU-side reading work; does not measure startup or GPU presentation."""
import argparse
import csv
import hashlib
import random
import subprocess
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--baseline', type=Path, required=True)
parser.add_argument('--candidate', type=Path, required=True)
parser.add_argument('--fixtures', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--repetitions', type=int, default=5)
parser.add_argument('--seed', type=int, default=20260915)
args = parser.parse_args()
assert args.repetitions > 0
order = [(variant, fixture, repetition)
         for variant in ['baseline', 'candidate']
         for fixture in ['ordinary-5k.md', 'gfm-math-images-100k.md']
         for repetition in range(1, args.repetitions + 1)]
random.Random(args.seed).shuffle(order)
args.output.parent.mkdir(parents=True, exist_ok=True)
with args.output.open('w', newline='', encoding='utf-8') as output:
    writer = csv.DictWriter(output, fieldnames=[
        'variant', 'fixture', 'repetition', 'binary_sha256', 'fixture_sha256',
        'scroll_median_ms', 'scroll_p95_ms', 'final_scroll_offset'])
    writer.writeheader()
    for variant, fixture, repetition in order:
        binary = getattr(args, variant).resolve()
        path = (args.fixtures / fixture).resolve()
        result = subprocess.run([str(binary), str(path)], input='\n\n\n',
                                capture_output=True, text=True, timeout=30, check=True)
        values = next(line.removeprefix('SCROLL ') for line in result.stdout.splitlines()
                      if line.startswith('SCROLL ')).split(',')
        writer.writerow({
            'variant': variant, 'fixture': fixture, 'repetition': repetition,
            'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
            'fixture_sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
            'scroll_median_ms': values[0], 'scroll_p95_ms': values[1],
            'final_scroll_offset': values[2]})
        output.flush()
print(f'Recorded {len(order)} runs in {args.output}')
