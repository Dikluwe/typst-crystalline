"""Derive display metrics without mixing campaigns or sampled and nominal blocks."""
import json
from pathlib import Path
from statistics import median

experiment = Path(__file__).resolve().parent
out = experiment.parents[1] / 'results/construction/diagnostics' / experiment.name
results = json.loads((out / 'timing.json').read_text())
by_block = {}
for line in (out / 'measurements.jsonl').read_text().splitlines():
    row = json.loads(line)
    if not row['period_ns']:
        by_block.setdefault(row['block'], {})[row['arm']] = row['cpu_ns']
assert len(by_block) == 24 and all(set(b) == {'candidate', 'control', 'vanilla'}
                                   for b in by_block.values())
medians = {arm: item['median'] for arm, item in results['nominal'].items()}
candidate, vanilla = medians['candidate'], medians['vanilla']
summary = {
    'cpu_median_ms': medians,
    'candidate_minus_control_ms': results['paired']['control'],
    'candidate_minus_vanilla_ms': results['paired']['vanilla'],
    'paired_reduction_percent': median(
        100 * (1 - b['candidate'] / b['control']) for b in by_block.values()),
    'favorable_pairs': sum(b['candidate'] < b['control'] for b in by_block.values()),
    'nominal_pairs': len(by_block),
    'candidate_vanilla_ratio': candidate / vanilla,
    'further_reduction_percent_to_vanilla': 100 * (1 - vanilla / candidate),
    'reservation_median_ms': {
        arm: {key: item for key, item in results['phases'][arm].items()
              if key.startswith('reserve_')}
        for arm in ('candidate', 'control')},
    'reservation_paired_ms': {
        key: item for key, item in results['phase_paired_control'].items()
        if key.startswith('reserve_')},
}
(out / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2))
