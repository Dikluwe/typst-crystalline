"""Ensaio baseline/candidato, com recibos e sem instrumentação produtiva."""
import argparse
import datetime
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = Path('/tmp/tekt-context-cache')
HERE = Path(__file__).resolve().parent
OUT.mkdir(exist_ok=True)
parser = argparse.ArgumentParser()
parser.add_argument('--candidate', type=Path)
args = parser.parse_args()
baseline = OUT / 'baseline-typst'
if not baseline.exists():
    shutil.copy2(ROOT / 'target/release/typst', baseline)
fixtures = {}
for name, source in json.loads((HERE / 'fixtures.json').read_text()).items():
    fixture = OUT / (name + '.typ')
    fixture.write_text(source)
    fixtures[name] = fixture
engines = {'baseline': baseline}
if args.candidate:
    engines['candidate'] = args.candidate.resolve()
env = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1', NO_COLOR='1')
cpu = min(os.sched_getaffinity(0))
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
git = lambda *options: subprocess.check_output(['git', *options], cwd=ROOT, text=True)
receipt = {'started': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'head': git('rev-parse', 'HEAD').strip(), 'status': git('status', '--short'), 'working_tree_hashes': {str(p): sha(p) for p in [*(ROOT / name for name in git('diff', '--name-only').splitlines() if name.endswith(('.rs', '.md', '.toml'))), ROOT / '01_core/src/compiler/realize/mod.rs', ROOT / '00_nucleo/prompts/compiler/realize/mod.md', ROOT / '00_nucleo/prompts/_nuclei/realization-order.toml', ROOT / '00_nucleo/prompts/_nuclei/compilation-metric-cache.toml', ROOT / '01_core/src/compiler/layout/realized.rs', ROOT / '00_nucleo/prompts/compiler/layout/realized.md']},
           'diff_stat': git('diff', 'HEAD', '--stat'), 'cpu': cpu,
           'environment': {key: env[key] for key in ('SOURCE_DATE_EPOCH', 'RAYON_NUM_THREADS', 'NO_COLOR')},
           'binaries': {name: {'path': str(path), 'sha256': sha(path)} for name, path in engines.items()},
           'fixtures': {name: {'source': path.read_text(), 'sha256': sha(path)} for name, path in fixtures.items()},
           'samples': []}
for name, fixture in fixtures.items():
    for index in range(-1, 9):
        order = list(engines)
        if index % 2:
            order.reverse()
        for engine in order:
            output = OUT / f'{name}-{engine}.pdf'
            usage = OUT / f'{name}-{engine}.usage'
            timing = OUT / f'{name}-{engine}.timings.json'
            command = ['/usr/bin/time', '-f', '%M %U %S', '-o', str(usage),
                       str(engines[engine]), 'compile', str(fixture), str(output),
                       '--root', str(fixture.parent), '--pdf-tagged=false', '--pretty=false',
                       '--timings-json', str(timing)]
            start = time.perf_counter_ns()
            run = subprocess.run(command, cwd=ROOT, env=env, capture_output=True,
                                 text=True, timeout=60,
                                 preexec_fn=lambda: os.sched_setaffinity(0, {cpu}))
            wall_ms = (time.perf_counter_ns() - start) / 1e6
            if run.returncode:
                raise RuntimeError((command, run.stderr))
            rss, user, system = usage.read_text().split()
            text = subprocess.check_output(['pdftotext', str(output), '-'], text=True)
            if index >= 0:
                receipt['samples'].append({'fixture': name, 'engine': engine, 'index': index,
                    'command': command, 'wall_ms': wall_ms, 'max_rss_kib': int(rss),
                    'cpu_seconds_coarse': float(user) + float(system), 'text': text,
                    'stderr': run.stderr, 'timings': json.loads(timing.read_text()),
                    'finished': datetime.datetime.now(datetime.timezone.utc).isoformat()})
receipt['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
receipt['summary'] = {name: {engine: {'wall_ms': statistics.median(
    x['wall_ms'] for x in receipt['samples'] if x['fixture'] == name and x['engine'] == engine),
    'max_rss_kib': statistics.median(x['max_rss_kib'] for x in receipt['samples']
    if x['fixture'] == name and x['engine'] == engine)} for engine in engines} for name in fixtures}
destination = HERE / ('bench-paired.json.gz' if args.candidate else 'bench-baseline.json.gz')
with gzip.open(destination, 'wt') as stream:
    json.dump(receipt, stream, ensure_ascii=False, indent=2)
print(json.dumps(receipt['summary'], indent=2))
