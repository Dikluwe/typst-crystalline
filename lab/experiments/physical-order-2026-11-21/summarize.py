"""Summarize only complete paired blocks, retaining interrupted raw logs."""
import json
import os
from pathlib import Path
from statistics import median

from observe import STATES, statistics

out = Path(os.environ['TEKT_RESULT_DIR'])
pairs = {}
for line in (out / 'nominal.jsonl').read_text().splitlines():
    row = json.loads(line)
    pairs.setdefault(row['block'], {})[row['arm']] = row
complete = {key: pair for key, pair in pairs.items() if set(pair) == {'tekt', 'vanilla'}}
result = {'complete_blocks': len(complete), 'partial_blocks': sorted(set(pairs)-set(complete)), 'states': {}}
for name, _, _ in STATES:
    tekt = [pair['tekt']['states'][name]['cpu_ns']/1e6 for pair in complete.values()]
    vanilla = [pair['vanilla']['states'][name]['cpu_ns']/1e6 for pair in complete.values()]
    result['states'][name] = {
        'tekt_ms': statistics(tekt),
        'vanilla_ms': statistics(vanilla),
        'paired_gap_ms': statistics([a-b for a,b in zip(tekt, vanilla)]),
        'paired_ratio': statistics([a/b for a,b in zip(tekt, vanilla)]),
    }
(out / 'timing-complete-pairs.json').write_text(json.dumps(result, indent=2) + '\n')
print(result['complete_blocks'], result['partial_blocks'])
