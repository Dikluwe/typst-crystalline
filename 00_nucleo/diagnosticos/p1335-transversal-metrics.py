"""Descriptive metrics of retained transversal channels, separate from inventory."""
import collections, importlib.util, json, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('receipt', D/'p1335-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
before = r.state(); r.verify(before)
source = D/'p1335-transversal-r3.json'
data = json.loads(source.read_text())
overlay = json.loads((D/'p1335-cli-capability-overlay.json').read_text())
known = {(x['id'], x['phase'], x['side']) for x in overlay['rows'] if x['successor_state'] == 'PUBLIC_CLI_ABSENT'}
groups = []
for group in data['matrix']:
    rows = group['results']
    groups.append(dict(phase=group['phase'], profile=group['profile'], cases=len(rows),
        selected_projection=dict(collections.Counter(x['estado'] for x in rows)),
        raw_channels=dict(collections.Counter(x.get('channel_state', 'NOT_EXECUTED_FEATURE_GATE') for x in rows))))
warning_counts = {}
for phase in ('normal', 'repeat', 'reverse'):
    rows = [x for x in data['warnings'] if x['phase'] == phase]
    count = collections.Counter()
    for row in rows:
        c = row['observations']['crystalline']
        count['C_CONTRACT_PRESERVED' if c['complete'] and c['warning_contract'] else 'C_CONTRACT_VIOLATED'] += 1
        if not row['bilateral']:
            count['C_ONLY_CONTROL_NO_BILATERAL_CREDIT'] += 1
            continue
        v = row['observations']['vanilla']
        if (row['id'], phase, 'vanilla') in known:
            count['PUBLIC_CLI_ABSENT'] += 1
        elif not v['complete'] or not c['complete']:
            count['Unknown'] += 1
        else:
            equal = all(v[k] == c[k] for k in ('exit_code','stdout','stderr'))
            count['RAW_CHANNELS_EQUAL' if equal else 'RAW_CHANNELS_DIFFERENT'] += 1
    warning_counts[phase] = dict(count)
normal = [g for g in groups if g['phase'] == 'normal']
extra_counts = {phase:dict(collections.Counter(x['runtime_class'] for x in data['extra'] if x['phase']==phase)) for phase in ('normal','repeat','reverse')}
after = r.state(); r.verify(after)
r.save('transversal-metrics', dict(at=r.now(), before=before, after=after,
    inputs={str(p):r.sha(p) for p in [source, D/'p1335-cli-capability-overlay.json', D/'p1335-review-transversal-final-r4.json', Path(__file__), D/'p1335-manifest.json']},
    groups=groups, warning_counts=warning_counts, extra_counts=extra_counts,
    location_cells=len(data['location']), matrix_cells_per_order=sum(x['cases'] for x in normal),
    selected_projection_per_order=dict(sum((collections.Counter(g['selected_projection']) for g in normal),collections.Counter())),
    raw_channels_per_order=dict(sum((collections.Counter(g['raw_channels']) for g in normal),collections.Counter())),
    limits='All counts supplemental. Selected geometry/text/DOM/metadata projections do not replace raw channels. Rejected vanilla CLI has no export parity claim. C-only controls have no bilateral denominator. PDF bytes are not language equivalence.'))
