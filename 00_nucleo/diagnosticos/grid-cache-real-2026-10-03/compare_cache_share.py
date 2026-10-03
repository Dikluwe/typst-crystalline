"""A/B pareado entre binários Tekt antes e depois do reúso de métricas."""

import argparse
import hashlib
import json
import os
import pathlib
import statistics
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument('--before', type=pathlib.Path, required=True)
parser.add_argument('--after', type=pathlib.Path, required=True)
parser.add_argument('--output', type=pathlib.Path, required=True)
parser.add_argument('--fixtures', nargs='+', choices=('one_cell_row', 'distinct', 'same', 'rtl'))
parser.add_argument('--pairs', type=int, default=16)
args = parser.parse_args()
if args.pairs <= 0:
    parser.error('--pairs must be positive')

here = pathlib.Path(__file__).resolve().parent
root = here.parents[2]
fixtures = {
    'one_cell_row': root / '00_nucleo/diagnosticos/grid-time-2026-10-02/grid1.typ',
    'distinct': here / 'grid80-distinct.typ',
    'same': here / 'grid80-same.typ',
    'rtl': here / 'grid80-rtl.typ',
}
if args.fixtures:
    fixtures = {name: fixtures[name] for name in args.fixtures}
if 'rtl' in fixtures:
    letters = 'אבגדהוזחטיכלמנסעפצקרשת'
    lines = [
        '#set page(width: 500pt, height: 3500pt, margin: 30pt)',
        '#set text(font: "DejaVu Sans", lang: "he", dir: rtl, size: 10pt)',
        '#table(columns: 4, rows: 80,',
    ]
    for row in range(80):
        for col in range(4):
            lines.append(f'[{letters[(row + col) % len(letters)]}שלום{letters[(row + 2 * col) % len(letters)]}],')
    lines.append(')')
    fixtures['rtl'].write_text('\n'.join(lines) + '\n')
binaries = {'before': args.before.resolve(), 'after': args.after.resolve()}
scratch = pathlib.Path('/tmp/tekt-grid-cache-ab')
scratch.mkdir(exist_ok=True)
cpu = min(os.sched_getaffinity(0))
env = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*command):
    return subprocess.check_output(['git', *command], cwd=root, text=True)


def run(fixture, engine, index):
    pdf = scratch / f'{fixture}-{engine}-{index}.pdf'
    source = fixtures[fixture]
    command = [str(binaries[engine]), 'compile', str(source), str(pdf),
               '--root', str(source.parent), '--pdf-tagged=false', '--pretty=false']
    start = time.perf_counter_ns()
    process = subprocess.Popen(command, cwd=root, env=env, stdout=subprocess.DEVNULL,
                               stderr=subprocess.PIPE,
                               preexec_fn=lambda: os.sched_setaffinity(0, {cpu}))
    _, status, usage = os.wait4(process.pid, 0)
    stderr = process.stderr.read().decode(errors='replace')
    if status:
        raise RuntimeError(f'{command}: {status}: {stderr}')
    return {'fixture': fixture, 'engine': engine, 'index': index,
            'wall_ms': (time.perf_counter_ns() - start) / 1e6,
            'user_ms': usage.ru_utime * 1000,
            'system_ms': usage.ru_stime * 1000}


for fixture in fixtures:
    for warm in range(2):
        for engine in binaries:
            run(fixture, engine, f'w{warm}')

samples = []
for fixture in fixtures:
    for index in range(args.pairs):
        order = ('before', 'after') if index % 2 == 0 else ('after', 'before')
        for engine in order:
            samples.append(run(fixture, engine, index))

summary = {}
for fixture in fixtures:
    before = [item['wall_ms'] for item in samples
              if item['engine'] == 'before' and item['fixture'] == fixture]
    after = [item['wall_ms'] for item in samples
             if item['engine'] == 'after' and item['fixture'] == fixture]
    summary[fixture] = {
        'before_median_ms': statistics.median(before),
        'after_median_ms': statistics.median(after),
        'paired_after_minus_before_median_ms': statistics.median(
            a - b for a, b in zip(after, before)
        ),
        'before_range_ms': [min(before), max(before)],
        'after_range_ms': [min(after), max(after)],
    }

result = {
    'head': git('rev-parse', 'HEAD').strip(),
    'status': git('status', '--porcelain=v1', '--untracked-files=all'),
    'diff_stat': git('diff', 'HEAD', '--stat'),
    'utc': time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
    'cpu': cpu,
    'binary_sha256': {name: sha(path) for name, path in binaries.items()},
    'fixture_sha256': {name: sha(path) for name, path in fixtures.items()},
    'summary': summary,
    'samples': samples,
}
args.output.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(summary, indent=2))
