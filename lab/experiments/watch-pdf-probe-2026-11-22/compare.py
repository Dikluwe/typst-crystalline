"""Pair exact zlib cache probe against the frozen byte-cache ELF."""
import json
import os
from pathlib import Path
from random import Random
import statistics
import sys

here = Path(__file__).resolve().parent
root = here.parents[2]
sys.path.insert(0, str(root / 'lab/experiments/watch-pdf-byte-retention-2026-11-22'))
import run

observe = run.observe
work = Path(os.environ['TEKT_LAB_WORK'])
out = Path(os.environ['TEKT_RESULT_DIR'])
old = root / 'lab/experiments/watch-pdf-byte-retention-2026-11-22/candidate-binary'
new = here / 'candidate-binary'
observe.EXES = {'old': old, 'probe': new}
observe.ARMS = tuple(observe.EXES)

def environment(case):
    env = run.old_env(case)
    env['TEKT_LAB_WATCH_RETAIN_BYTES'] = '1'
    return env
observe.env = environment

def summary(values):
    rng = Random(20261122)
    sims = sorted(statistics.median(rng.choices(values, k=len(values))) for _ in range(10000))
    return {'median_ms': statistics.median(values), 'ci95_ms': [sims[249], sims[9749]]}

observe.validate()
valid = json.loads((out / 'validation.json').read_text())
for state in valid['old']:
    assert valid['old'][state]['navigation'] == valid['probe'][state]['navigation']
    assert valid['old'][state]['pages'] == valid['probe'][state]['pages']
    assert valid['old'][state]['links'] == valid['probe'][state]['links']

rows = []
for block in range(24):
    order = ('old', 'probe') if block % 2 == 0 else ('probe', 'old')
    pair = {}
    for arm in order:
        pair[arm] = observe.run(block, arm, area='paired')
    rows.append(pair)
    print(block + 1, '/ 24', flush=True)
with (out / 'paired.jsonl').open('w') as stream:
    for pair in rows:
        for arm in ('old', 'probe'):
            stream.write(json.dumps(pair[arm]) + '\n')
states = tuple(row[0] for row in observe.STATES)
result = {}
for state in states:
    old_ms = [pair['old']['states'][state]['cpu_ns'] / 1e6 for pair in rows]
    probe_ms = [pair['probe']['states'][state]['cpu_ns'] / 1e6 for pair in rows]
    result[state] = {'old': summary(old_ms), 'probe': summary(probe_ms),
                     'paired_probe_minus_old': summary([p-o for p,o in zip(probe_ms, old_ms)])}
cycle_old = [sum(pair['old']['states'][state]['cpu_ns'] for state in states) / 1e6 for pair in rows]
cycle_probe = [sum(pair['probe']['states'][state]['cpu_ns'] for state in states) / 1e6 for pair in rows]
result['cycle'] = {'old': summary(cycle_old), 'probe': summary(cycle_probe),
                   'paired_probe_minus_old': summary([p-o for p,o in zip(cycle_probe, cycle_old)])}
(out / 'timing.json').write_text(json.dumps(result, indent=2) + '\n')
