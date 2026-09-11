"""Paired idle CPU check for FMV release binaries, separate from competitor data."""
import argparse
import csv
import hashlib
import json
import os
import pathlib
import random
import subprocess
import time

import psutil
from measure import snapshot, tree

parser = argparse.ArgumentParser()
parser.add_argument('config', type=pathlib.Path)
parser.add_argument('--output', required=True, type=pathlib.Path)
parser.add_argument('--runs', type=int, default=3)
parser.add_argument('--settle', type=float, default=10)
parser.add_argument('--duration', type=float, default=5)
args = parser.parse_args()
config = json.loads(args.config.read_text())
fixture = pathlib.Path(config['fixtures']) / 'ordinary-5k.md'
jobs = [(app, run) for app in config['apps'] for run in range(1, args.runs + 1)]
random.Random(20260911).shuffle(jobs)
fields = ['app', 'run', 'settle_s', 'sample_s', 'status', 'cpu_ms_per_second',
          'rss_mib', 'processes', 'exe_sha256', 'fixture_sha256']
with args.output.open('w', newline='') as out:
    writer = csv.DictWriter(out, fieldnames=fields, lineterminator='\n')
    writer.writeheader()
    for app, run in jobs:
        env = os.environ.copy()
        env.update(app.get('env', {}))
        command = [part.replace('{fixture}', str(fixture)) for part in app['command']]
        with open(command[0], 'rb') as binary:
            digest = hashlib.file_digest(binary, 'sha256').hexdigest()
        proc = subprocess.Popen(command, env=env, stdin=subprocess.DEVNULL,
                                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        root = psutil.Process(proc.pid)
        try:
            time.sleep(args.settle)
            before = snapshot(root)
            started = time.perf_counter()
            time.sleep(args.duration)
            after = snapshot(root)
            duration = time.perf_counter() - started
            row = dict(app=app['id'], run=run, settle_s=args.settle,
                       sample_s=round(duration, 4), status='alive' if proc.poll() is None else 'exited',
                       cpu_ms_per_second=round((after['cpu_s'] - before['cpu_s']) * 1000 / duration, 3),
                       rss_mib=round(after['rss_bytes'] / 2**20, 3), processes=after['processes'],
                       exe_sha256=digest, fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest())
            writer.writerow(row)
            out.flush()
            print(json.dumps(row), flush=True)
        finally:
            owned = tree(root)
            for child in reversed(owned):
                try:
                    child.terminate()
                except psutil.Error:
                    pass
            _, alive = psutil.wait_procs(owned, timeout=3)
            for child in alive:
                try:
                    child.kill()
                except psutil.Error:
                    pass
