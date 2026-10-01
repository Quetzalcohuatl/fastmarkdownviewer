"""Measure helper completion and first fully written PNG tile, not GUI presentation."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import random
import subprocess
import time

parser = argparse.ArgumentParser(__doc__)
parser.add_argument('--baseline', type=Path, required=True)
parser.add_argument('--candidate', type=Path, required=True)
parser.add_argument('--work', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--runs', type=int, default=5)
args = parser.parse_args()
args.work.mkdir(parents=True, exist_ok=True)
fixtures = {
    'small': 'flowchart LR\n A[Open document] --> B[Read text] --> C[Load diagram]',
    'large': 'sequenceDiagram\n participant A as Reader\n participant B as Renderer\n'
             + (' A->>B: ' + 'wide label ' * 22 + '\n') * 70,
}
for name, source in fixtures.items():
    (args.work / f'{name}.mmd').write_text(source, encoding='utf-8')
appearance = dict(background=[39, 40, 34], surface=[52, 53, 47], text=[248, 248, 242],
                  muted=[176, 176, 155], accent=[166, 226, 46], selection=[73, 72, 62], font=None)
jobs = [(name, variant, run) for name in fixtures
        for variant in ['baseline', 'candidate-display', 'candidate-natural']
        for run in range(1, args.runs + 1)]
random.Random(20261001).shuffle(jobs)
# One excluded warmup per case, followed by the randomized fresh-process sample.
jobs = [(name, variant, 0) for name in fixtures
        for variant in ['baseline', 'candidate-display', 'candidate-natural']] + jobs
args.output.parent.mkdir(parents=True, exist_ok=True)


def ready_png(path):
    try:
        data = path.read_bytes()
        return data[:8] == b'\x89PNG\r\n\x1a\n' and data[-12:] == b'\0\0\0\0IEND\xaeB`\x82'
    except OSError:
        return False


with args.output.open('w', newline='', encoding='utf-8') as output:
    writer = csv.DictWriter(output, fieldnames=['fixture', 'variant', 'run', 'exit_code',
                                               'geometry_ms', 'first_png_ms', 'complete_ms',
                                               'width', 'height', 'binary_sha256', 'error'])
    writer.writeheader()
    for name, variant, run in jobs:
        work = args.work / f'{name}-{variant}-{run}'
        work.mkdir(exist_ok=False)
        tiled = variant != 'baseline'
        binary = (args.candidate if tiled else args.baseline).resolve()
        result = work / ('diagram.json' if tiled else 'diagram.png')
        png = work / 'tile-0-0.png' if tiled else result
        command = [str(binary), '--internal-mermaid', str((args.work / f'{name}.mmd').resolve()),
                   str(result.resolve())]
        if tiled:
            command += [json.dumps(appearance), json.dumps(dict(
                max_width=800 if variant == 'candidate-display' else 50000, pixel_scale=1))]
        first = geometry_time = None
        geometry = {}
        start = time.perf_counter()
        process = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                                   stderr=subprocess.DEVNULL)
        try:
            while True:
                now = (time.perf_counter() - start) * 1000
                if first is None and ready_png(png):
                    first = now
                if tiled and geometry_time is None:
                    try:
                        geometry = json.loads(result.read_text())
                        geometry_time = now
                    except (OSError, ValueError):
                        pass
                if process.poll() is not None:
                    break
                if now > 60000:
                    raise TimeoutError('Benchmark-only 60-second deadline exceeded')
                time.sleep(.005)
            complete = (time.perf_counter() - start) * 1000
        finally:
            if process.poll() is None:
                process.kill()
            process.wait()
        error = result.with_suffix('.error')
        if run:
            writer.writerow(dict(fixture=name, variant=variant, run=run, exit_code=process.returncode,
                                 geometry_ms=geometry_time, first_png_ms=first, complete_ms=complete,
                                 width=geometry.get('width'), height=geometry.get('height'),
                                 binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                                 error=error.read_text() if error.exists() else ''))
            output.flush()
print(f'Recorded {args.runs * 6} helper samples; first PNG polling granularity is approximately 5 ms.')
