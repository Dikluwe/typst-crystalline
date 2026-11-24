"""Measure the visible fallback with and without intervening cache hits."""
import json
import os
from pathlib import Path
import statistics

import run

observe = run.observe
original = observe.STATES
out = Path(os.environ['TEKT_RESULT_DIR'])
rows = []
for sequence, states in [('direct', (original[0], original[-1])),
                         ('after_comments', original)]:
    observe.STATES = states
    for block in range(12):
        order = ('off', 'bytes') if block % 2 == 0 else ('bytes', 'off')
        pair = {}
        for arm in order:
            row = observe.run(block, arm, area='visible-ablation-' + sequence)
            pair[arm] = row['states']['visible']['cpu_ns'] / 1e6
        pair['difference_ms'] = pair['bytes'] - pair['off']
        pair['sequence'] = sequence
        pair['block'] = block
        rows.append(pair)
        print(sequence, block + 1, pair['difference_ms'], flush=True)
observe.STATES = original
result = {sequence: {'median_difference_ms': statistics.median(
    row['difference_ms'] for row in rows if row['sequence'] == sequence)}
    for sequence in ('direct', 'after_comments')}
(out / 'visible-ablation.json').write_text(json.dumps({'summary': result, 'rows': rows}, indent=2) + '\n')
