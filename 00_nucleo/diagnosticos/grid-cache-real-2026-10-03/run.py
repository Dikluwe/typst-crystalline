"""Controle CLI pareado: 320 células distintas versus 320 textos idênticos."""

import hashlib
import json
import os
import pathlib
import statistics
import subprocess
import time

here = pathlib.Path(__file__).resolve().parent
root = here.parents[2]
output = pathlib.Path('/tmp/tekt-grid-cache-real')
output.mkdir(exist_ok=True)
engines = {
    'tekt': root / 'target/release/typst',
    'vanilla': root / 'lab/typst-original/target/release/typst',
}
fixtures = {
    'distinct': here / 'grid80-distinct.typ',
    'same': here / 'grid80-same.typ',
}
cpu = min(os.sched_getaffinity(0))
env = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1')


def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(['git', *args], cwd=root, text=True)


def run(fixture, engine, index):
    pdf = output / f'{fixture}-{engine}-{index}.pdf'
    command = [str(engines[engine]), 'compile', str(fixtures[fixture]), str(pdf),
               '--root', str(here), '--pdf-tagged=false', '--pretty=false']
    if engine == 'vanilla':
        command.extend(['--jobs', '1'])
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
            'user_ms': usage.ru_utime * 1000, 'system_ms': usage.ru_stime * 1000}


for fixture in fixtures:
    for warm in range(2):
        for engine in engines:
            run(fixture, engine, f'w{warm}')

samples = []
for engine in engines:
    for index in range(16):
        order = ('distinct', 'same') if index % 2 == 0 else ('same', 'distinct')
        for fixture in order:
            samples.append(run(fixture, engine, index))

summary = {}
for engine in engines:
    distinct = [item['wall_ms'] for item in samples
                if item['engine'] == engine and item['fixture'] == 'distinct']
    same = [item['wall_ms'] for item in samples
            if item['engine'] == engine and item['fixture'] == 'same']
    summary[engine] = {
        'distinct_median_ms': statistics.median(distinct),
        'same_median_ms': statistics.median(same),
        'paired_same_minus_distinct_median_ms': statistics.median(
            a - b for a, b in zip(same, distinct)
        ),
        'distinct_range_ms': [min(distinct), max(distinct)],
        'same_range_ms': [min(same), max(same)],
    }

result = {
    'head': git('rev-parse', 'HEAD').strip(),
    'status': git('status', '--porcelain=v1', '--untracked-files=all'),
    'diff_stat': git('diff', 'HEAD', '--stat'),
    'utc': time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
    'cpu': cpu,
    'binary_sha256': {name: sha(path) for name, path in engines.items()},
    'fixture_sha256': {name: sha(path) for name, path in fixtures.items()},
    'summary': summary,
    'samples': samples,
}
(output / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(summary, indent=2))
