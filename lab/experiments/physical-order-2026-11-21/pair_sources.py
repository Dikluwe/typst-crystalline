"""Pair an isolated candidate with the published Tekt under one watch protocol."""
import json
import os
from pathlib import Path

import observe

out = Path(os.environ['TEKT_RESULT_DIR'])
work = Path(os.environ['TEKT_LAB_WORK'])
observe.EXES['published'] = Path(os.environ['TEKT_PUBLISHED_EXE'])
arms = ('tekt', 'published')
blocks = int(os.environ.get('TEKT_PAIRS', '20'))
witness = json.loads((out / 'witness.json').read_text())['tekt']
rows = []
with (out / 'paired-published.jsonl').open('w') as stream:
    for block in range(blocks):
        pair = {}
        for arm in (arms if block % 2 == 0 else arms[::-1]):
            for attempt in range(3):
                try:
                    area = 'paired-published' if attempt == 0 else f'paired-published/retry-{attempt}'
                    row = observe.run(block, arm, area=area)
                    break
                except (observe.IncompleteWatch, TimeoutError):
                    if attempt == 2:
                        raise
                    print('retry', block, arm, attempt + 1, flush=True)
            for name, _, _ in observe.STATES:
                assert row['states'][name]['pdf_key'] == witness[name]['key'], (block, arm, name)
            pair[arm] = row
            stream.write(json.dumps(row) + '\n')
            stream.flush()
        rows.append(pair)
        print(block + 1, '/', blocks, flush=True)
summary = {}
for name, _, _ in observe.STATES:
    candidate = [row['tekt']['states'][name]['cpu_ns']/1e6 for row in rows]
    published = [row['published']['states'][name]['cpu_ns']/1e6 for row in rows]
    summary[name] = {
        'candidate_ms': observe.statistics(candidate),
        'published_ms': observe.statistics(published),
        'paired_gap_ms': observe.statistics([a-b for a,b in zip(candidate, published)]),
        'paired_ratio': observe.statistics([a/b for a,b in zip(candidate, published)]),
    }
(out / 'paired-published.json').write_text(json.dumps(summary, indent=2) + '\n')
