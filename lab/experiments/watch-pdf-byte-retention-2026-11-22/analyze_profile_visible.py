"""Attribute sampled visible CPU for the byte cache, using paired blocks."""
from collections import Counter
import importlib.util
import json
import os
from pathlib import Path
from random import Random
from statistics import median
import tarfile

out = Path(os.environ['TEKT_RESULT_DIR'])
root = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location(
    'phase_decoder', root / 'lab/experiments/physical-gap-review-2026-11-22/analyze.py')
analyzer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(analyzer)

def stats(values):
    rng = Random(681999)
    sims = sorted(median(rng.choices(values, k=len(values))) for _ in range(10000))
    return {'median': median(values), 'ci95': [sims[249], sims[9749]]}

rows = [json.loads(line) for line in (out / 'profile-visible.jsonl').read_text().splitlines()]
tables = {arm: analyzer.decoder.symbols(out / 'candidate.symbols') for arm in ('off', 'bytes')}
by_block = {}
leaves = {arm: Counter() for arm in tables}
with tarfile.open(out / 'profile-visible.tar.gz') as archive:
    for row in rows:
        stacks, timing = analyzer.decoder.stacks(archive, row, tables)
        scale = timing['main_ns'] / 1e6 / timing['samples']
        counts = Counter()
        for stack, n in stacks.items():
            counts['phase/' + analyzer.phase('tekt', stack)] += n
            if any('::pdf::byte_retention::' in frame for frame in stack):
                counts['byte_retention_scope'] += n
            if any('::pdf::watch_retention::' in frame for frame in stack):
                counts['document_retention_scope'] += n
            leaves[row['arm']][stack[0]] += n * scale
        measured = {name: n * scale for name, n in counts.items()}
        measured['cpu_ms'] = row['states']['visible']['cpu_ns'] / 1e6
        measured['samples'] = timing['samples']
        measured['truncated'] = timing['truncated']
        by_block.setdefault(row['block'], {})[row['arm']] = measured
assert len(by_block) == 24 and all(set(pair) == {'off', 'bytes'} for pair in by_block.values())
keys = sorted(set().union(*(r for pair in by_block.values() for r in pair.values())) - {'samples', 'truncated'})
summary = {'arms': {}, 'paired': {}, 'samples': {}, 'truncated': {}}
for arm in ('off', 'bytes'):
    summary['arms'][arm] = {key: stats([pair[arm].get(key, 0) for pair in by_block.values()]) for key in keys}
    summary['samples'][arm] = sum(pair[arm]['samples'] for pair in by_block.values())
    summary['truncated'][arm] = sum(pair[arm]['truncated'] for pair in by_block.values())
for key in keys:
    summary['paired'][key] = stats([pair['bytes'].get(key, 0) - pair['off'].get(key, 0)
                                    for pair in by_block.values()])
(out / 'profile-visible-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
(out / 'profile-visible-rows.json').write_text(json.dumps(by_block, indent=2) + '\n')
(out / 'profile-visible-leaves.json').write_text(json.dumps(
    {arm: leaves[arm].most_common(100) for arm in leaves}, indent=2) + '\n')
print(json.dumps(summary['paired'], indent=2))
