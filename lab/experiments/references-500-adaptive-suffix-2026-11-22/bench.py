"""Pair the same laboratory ELF in three modes against the pinned vanilla."""
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from statistics import median
import sys

here = Path(__file__).resolve().parent
root = here.parents[2]
sys.path.insert(0, str(here.parent / 'references-500-visible-eval-residual-2026-11-21'))
from watch import EXES, run

out = Path(os.environ['TEKT_LAB_TIMING_OUT'])
EXES['candidate'] = here / 'candidate.elf'
EXES['vanilla'] = root / 'lab/typst-original/target/release/typst'
arms = ('off', 'fixed', 'adaptive', 'vanilla')
rng = Random(20261122)
os.sched_setaffinity(0, {2})
order = list(permutations(arms))
rng.shuffle(order)
series = []
with (out / 'measurements.jsonl').open('w') as log:
    for block, group in enumerate(order):
        rows = {}
        for index, mode in enumerate(group):
            engine = 'vanilla' if mode == 'vanilla' else 'candidate'
            options = None if mode == 'vanilla' else {
                'TEKT_LAB_SUFFIX_MODE': mode,
                'TEKT_LAB_MEMO_PROTECT': '1',
                'TEKT_LAB_MEMO_BYTES': str(8 * 1024 * 1024),
            }
            row = run(4 * block + index, engine, 0, overrides=options)
            rows[mode] = row['cpu_ns'] / 1e6
            print(json.dumps(row | {'block': block, 'mode': mode}), file=log, flush=True)
        series.append(rows)
        print(block + 1, '/ 24', flush=True)

def summary(samples):
    draws = sorted(median(rng.choices(samples, k=len(samples))) for _ in range(10000))
    return {'median': median(samples), 'ci95': [draws[249], draws[9749]]}

result = {
    'cpu_ms': {arm: summary([row[arm] for row in series]) for arm in arms},
    'adaptive_minus_ms': {arm: summary([row['adaptive'] - row[arm] for row in series])
                          for arm in arms if arm != 'adaptive'},
    'adaptive_over_vanilla': summary([row['adaptive'] / row['vanilla'] for row in series]),
    'saving_vs_off_percent': summary([100 * (1 - row['adaptive'] / row['off']) for row in series]),
}
(out / 'timing.json').write_text(json.dumps(result, indent=2) + '\n')
