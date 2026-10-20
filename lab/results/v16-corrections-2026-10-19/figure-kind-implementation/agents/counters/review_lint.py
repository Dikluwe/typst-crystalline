#!/usr/bin/env python3
"""Evidence-only exact SARIF differential; never invokes lint or mutates sources."""
import argparse
from collections import Counter, defaultdict
from datetime import datetime, timezone
import gzip
import hashlib
import json
from pathlib import Path
import subprocess


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    raw = Path(path).read_bytes()
    return json.loads(gzip.decompress(raw) if str(path).endswith('.gz') else raw)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False)


def results(path):
    return [r for run in read(path)['runs'] for r in run.get('results', [])]


def source(result):
    return result.get('locations', [{}])[0].get('physicalLocation', {}).get('artifactLocation', {}).get('uri', '').removeprefix('./')


def key(result):
    return (source(result), result['ruleId'], result.get('level', ''), result['message']['text'])


def counts(items):
    return dict(sorted(Counter(r['ruleId'] + ':' + r.get('level', '') for r in items).items()))


def exact_delta(old, new):
    a, b = Counter(map(canonical, old)), Counter(map(canonical, new))
    digest = lambda c: hashlib.sha256(canonical(sorted(c.elements())).encode()).hexdigest()
    return {'baseline_occurrences': len(old), 'final_occurrences': len(new),
            'removed': [json.loads(s) for s in (a-b).elements()],
            'added': [json.loads(s) for s in (b-a).elements()],
            'baseline_multiset_sha256': digest(a), 'final_multiset_sha256': digest(b)}


def inventory(delta, items):
    locations = defaultdict(list)
    for r in items:
        locations[key(r)].append(r.get('locations', []))
    return [dict(source=k[0], rule=k[1], level=k[2], message=k[3], delta_occurrences=n,
                 all_snapshot_occurrence_locations=locations[k]) for k, n in sorted(delta.items())]


p = argparse.ArgumentParser()
p.add_argument('--baseline', required=True)
p.add_argument('--candidate', required=True)
p.add_argument('--scope', required=True)
p.add_argument('--state', required=True)
p.add_argument('--previous')
p.add_argument('--output', required=True, help='Output prefix; existing files are never overwritten')
a = p.parse_args()
output = Path(a.output)
for suffix in ['.json', '.md']:
    if output.with_suffix(suffix).exists():
        p.error('Refusing to overwrite historical review: ' + str(output.with_suffix(suffix)))
scope = read(a.scope)
paths = {pair[k] for pair in scope['pairs'] for k in ['source', 'owner']}
old, new = results(a.baseline), results(a.candidate)
inside_old, inside_new = [r for r in old if source(r) in paths], [r for r in new if source(r) in paths]
outside = exact_delta([r for r in old if source(r) not in paths], [r for r in new if source(r) not in paths])
old_keys, new_keys = Counter(map(key, inside_old)), Counter(map(key, inside_new))
removed, added = inventory(old_keys-new_keys, inside_old), inventory(new_keys-old_keys, inside_new)
state = read(a.state)
changed = [path for path in sorted(paths) if path in state['inputs'] and (not Path(path).exists() or sha(path) != state['inputs'][path])]
report = {'review_utc': datetime.now(timezone.utc).isoformat(),
          'review_state': 'Evidence-only exact diagnostic comparison; no source/L0 writes, lint execution, Cargo or suppression.',
          'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'state': 'uncommitted working tree',
          'diff_head_stat_at_review': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'scope': {'path': a.scope, 'sha256': sha(a.scope), 'pairs': len(scope['pairs'])},
          'baseline': {'path': a.baseline, 'sha256': sha(a.baseline), 'counts': counts(old)},
          'candidate': {'path': a.candidate, 'sha256': sha(a.candidate), 'counts': counts(new),
                        'state_path': a.state, 'state_sha256': sha(a.state),
                        'utc_start': state['utc_start'], 'utc_end': state['utc_end'],
                        'changed_inputs_during_lint': state['changed_inputs'],
                        'linter_sha256': state['linter_sha256'], 'command': state['command']},
          'outside_scope_complete_objects': outside,
          'inside_scope_comparison': 'Exact source/rule/level/full-message multiset; locations retained separately; no loss of repeated multiplicity.',
          'removed': removed, 'added': added,
          'current_scope_inputs_differing_from_snapshot': changed,
          'manual_reconciliation_required': True,
          'limits': 'Grouped-alternative runtime completeness is not certified; structural probes and measured language controls remain separate. Notes are inventoried, not suppressed. No global zero-lint or F closure assertion.'}
if a.previous:
    previous = results(a.previous)
    report['previous_complete_objects'] = {'path': a.previous, 'sha256': sha(a.previous), **exact_delta(previous, new)}
    prev_keys = Counter(map(key, previous))
    report['previous_message_inventory'] = {'removed': inventory(prev_keys-Counter(map(key, new)), previous), 'added': inventory(Counter(map(key, new))-prev_keys, new)}
output.with_suffix('.json').write_text(json.dumps(report, indent=2, ensure_ascii=False)+'\n')
lines = ['# Diagnostic differential inventory', '',
         f"Evidence-only draft for `{a.candidate}` with {len(scope['pairs'])} manifested pairs. Manual causal reconciliation is required before a final conclusion.", '',
         '| Snapshot | Counts |', '|---|---|', f"| Original baseline | {counts(old)} |", f"| Candidate | {counts(new)} |", '',
         f"Outside scope, exact complete diagnostic objects: {len(outside['removed'])} removed, {len(outside['added'])} added. Snapshot changed inputs: {state['changed_inputs']}; current scope mismatches: {changed}.", '']
for title, entries in [('Removed message occurrences', removed), ('Added message occurrences', added)]:
    lines += ['## '+title, '']
    by_source = defaultdict(Counter)
    for item in entries:
        by_source[item['source']][item['rule']] += item['delta_occurrences']
    lines += [f'- `{path}`: '+', '.join(f'{rule} × {n}' for rule, n in sorted(totals.items())) for path, totals in sorted(by_source.items())]
    lines += ['']
lines += ['The companion JSON retains every full message, multiplicity and location, exact outside-scope objects and hashes. V19 runtime alternative coverage and V20 nested identity notes remain visible; no test can establish every alternative merely by executing a shared arm. No completion or clean-global-lint claim is made.']
output.with_suffix('.md').write_text('\n'.join(lines)+'\n')
print(json.dumps({'scope_pairs':len(scope['pairs']), 'counts':counts(new), 'outside_removed':len(outside['removed']), 'outside_added':len(outside['added']), 'removed':dict(Counter({r:sum(x['delta_occurrences'] for x in removed if x['rule']==r) for r in {x['rule'] for x in removed}})), 'added':dict(Counter({r:sum(x['delta_occurrences'] for x in added if x['rule']==r) for r in {x['rule'] for x in added}})), 'output':str(output)}))
