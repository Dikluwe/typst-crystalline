"""Paired four-publication CPU totals, using the recorded block order."""
import json
import random
import statistics
from pathlib import Path
import os

out = Path(os.environ['TEKT_RESULT_DIR'])
rows = [json.loads(line) for line in (out / 'paired.jsonl').read_text().splitlines()]
arms = ('off', 'doc', 'bytes', 'baseline', 'vanilla')
states = ('startup', 'comment', 'comment-2', 'visible')
assert len(rows) == 24 * len(arms)
blocks = []
for start in range(0, len(rows), len(arms)):
    block = {row['arm']: row for row in rows[start:start + len(arms)]}
    assert set(block) == set(arms)
    blocks.append(block)

def summary(values):
    rng = random.Random(20261122)
    replicates = sorted(statistics.median(rng.choices(values, k=len(values)))
                        for _ in range(10000))
    return {'median_ms': statistics.median(values),
            'bootstrap_ci95_ms': [replicates[249], replicates[9749]]}

totals = {arm: [sum(block[arm]['states'][state]['cpu_ns'] for state in states) / 1e6
                for block in blocks] for arm in arms}
result = {'blocks': len(blocks),
          'totals': {arm: summary(values) for arm, values in totals.items()},
          'paired_bytes_minus': {arm: summary([x-y for x,y in zip(totals['bytes'], totals[arm])])
                                 for arm in arms if arm != 'bytes'}}
(out / 'cycle.json').write_text(json.dumps(result, indent=2) + '\n')
