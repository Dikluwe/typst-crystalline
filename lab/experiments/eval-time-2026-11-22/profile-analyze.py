"""Compare sampled evaluation CPU and nested scopes in paired Tekt runs."""
import importlib.util
import json
from pathlib import Path
import statistics
import sys
import tarfile

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / 'physical-order-2026-11-21'))
import observe

source = here.parent / 'physical-gap-review-2026-11-22/analyze.py'
spec = importlib.util.spec_from_file_location('phase_decoder', source)
analyzer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(analyzer)

out = Path(__import__('os').environ['TEKT_RESULT_DIR'])
rows = [json.loads(line) for line in (out / 'profile-paired.jsonl').read_text().splitlines()]
tables = {arm: analyzer.decoder.symbols(out / f'{arm}.symbols')
          for arm in ('tekt', 'published')}
by_block = {}
with tarfile.open(out / 'profile-paired.tar.gz') as archive:
    for row in rows:
        stacks, timing = analyzer.decoder.stacks(archive, row, tables)
        scale = timing['main_ns'] / 1e6 / timing['samples']
        counts = {'eval': 0, 'group': 0, 'collapse': 0}
        for stack, count in stacks.items():
            if analyzer.phase('tekt', stack) != 'eval':
                continue
            counts['eval'] += count
            if any('TextualSnapshot::same_group' in frame for frame in stack):
                counts['group'] += count
            if any('StyleChain::collapse' in frame for frame in stack):
                counts['collapse'] += count
        measured = {key + '_ms': count * scale for key, count in counts.items()}
        measured.update(cpu_ms=row['cpu_ns'] / 1e6,
                        samples=timing['samples'], truncated=timing['truncated'],
                        period_ns=row['period_ns'])
        by_block.setdefault(row['block'], {})[row['arm']] = measured
assert len(by_block) == 24 and all(set(pair) == {'tekt', 'published'} for pair in by_block.values())
summary = {'pairs': len(by_block), 'arms': {}, 'paired': {}}
for key in ('eval_ms', 'group_ms', 'collapse_ms', 'cpu_ms'):
    for arm in ('tekt', 'published'):
        values = [pair[arm][key] for pair in by_block.values()]
        summary['arms'].setdefault(arm, {})[key] = observe.statistics(values)
    gaps = [pair['tekt'][key] - pair['published'][key] for pair in by_block.values()]
    summary['paired'][key] = observe.statistics(gaps)
summary['samples'] = {arm: sum(pair[arm]['samples'] for pair in by_block.values())
                      for arm in ('tekt', 'published')}
summary['truncated'] = {arm: sum(pair[arm]['truncated'] for pair in by_block.values())
                        for arm in ('tekt', 'published')}
(out / 'profile-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
(out / 'profile-rows.json').write_text(json.dumps(by_block, indent=2) + '\n')
print(json.dumps(summary, indent=2))
