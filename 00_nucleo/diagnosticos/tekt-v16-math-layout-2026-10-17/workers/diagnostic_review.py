from pathlib import Path
import collections, difflib, gzip, json

out = Path(__file__).resolve().parents[1]
b = json.load(gzip.open(out / 'before.json.gz'))
scope = json.load(open(out / 'scope.json'))
probe = json.load(gzip.open(out / 'probe-green-input.json.gz'))
before = json.load(open(out / 'baseline-architecture.log'))['runs'][0]['results']
after = json.load(open(out / 'validation-architecture.log'))['runs'][0]['results']
maps, positions = {}, {}
for source, ast in probe['AST'].items():
    maps[source] = {i + 1: j + 1 for block in difflib.SequenceMatcher(None, b['files'][source].splitlines(), Path(source).read_text().splitlines(), autojunk=False).get_matching_blocks() for i, j in zip(range(block.a, block.a + block.size), range(block.b, block.b + block.size))}
    positions[source] = {}
    assert len(ast['before']) == len(ast['after'])
    for a, z in zip(ast['before'], ast['after']):
        assert len(a['arms']) == len(z['arms'])
        for aa, zz in zip(a['arms'], z['arms']):
            positions[source][(aa['line'], aa['column'])] = (zz['line'], zz['column'])

def loc(r):
    p = r['locations'][0]['physicalLocation']
    return p['artifactLocation']['uri'].removeprefix('./'), p['region']['startLine'], p['region']['startColumn']

def key(r, line=None, column=None):
    x = json.loads(json.dumps(r))
    region = x['locations'][0]['physicalLocation']['region']
    if line is not None:
        region['startLine'] = line
        region['startColumn'] = column
    return json.dumps(x, sort_keys=True, ensure_ascii=False)

expected = []
unmapped = []
removed = []
removed_depth = []
rewritten_notes = []
optional_pattern = 'Some(Content::MathIdent(s)) | Some(Content::MathText(s))'
selected = {(r['source'], r['line']) for r in scope['targets'] if r['status'] == 'emitted-warning'}
for r in before:
    p, line, col = loc(r)
    if r['ruleId'] == 'V16' and (p, line) in selected:
        removed.append(r)
        continue
    if p.endswith('/math/layout/attach.rs') and r['ruleId'] == 'V20' and optional_pattern in r['message']['text']:
        removed_depth.append(r)
        continue
    if p.endswith('/math/layout/attach.rs') and r['ruleId'] == 'V19' and optional_pattern in r['message']['text']:
        rewritten_notes.append(r)
        r = json.loads(json.dumps(r))
        r['message']['text'] = r['message']['text'].replace(optional_pattern, 'Content::MathIdent(s) | Content::MathText(s)')
    if p not in maps:
        expected.append(key(r))
    elif (line, col) in positions[p]:
        n, c = positions[p][line, col]
        expected.append(key(r, n, c))
    elif line in maps[p]:
        expected.append(key(r, maps[p][line], col))
    else:
        unmapped.append(r)
missing = collections.Counter(expected) - collections.Counter(key(r) for r in after)
added = collections.Counter(key(r) for r in after) - collections.Counter(expected)
counts = lambda rs: dict(collections.Counter(r['ruleId'] + ':' + r['level'] for r in rs))
record = dict(before=counts(before), after=counts(after), removed_V16=removed, unmapped=unmapped, missing=[{'diagnostic': json.loads(k), 'count': n} for k, n in missing.items()], added=[{'diagnostic': json.loads(k), 'count': n} for k, n in added.items()])
(out / 'diagnostic-review.json').write_text(json.dumps(record, indent=2, ensure_ascii=False) + '\n')
print(json.dumps({k: record[k] for k in ['before', 'after']}))
print(json.dumps({'removed_V16': len(removed), 'unmapped': len(unmapped), 'missing': sum(missing.values()), 'added': sum(added.values())}))
assert not unmapped and not missing, record
assert len(removed) == 18 and len(removed_depth) == 1 and len(rewritten_notes) == 1
assert sum(added.values()) == 39
new_notes = [json.loads(k) for k, n in added.items() if n == 1]
assert len(new_notes) == 39
assert all(r['ruleId'] == 'V19' and r['level'] == 'note' for r in new_notes)
expected_positions = []
for target in probe['record']['targets']:
    ast = probe['AST'][target['source']]
    a = ast['before'][target['match_index']]
    z = ast['after'][target['match_index']]
    if target['id'] == 'partition-forwarding-auxiliary':
        arm = len(a['arms']) - 1
    else:
        arms = [i for i, a in enumerate(a['arms']) if a['pattern'] == '_']
        assert len(arms) == 1
        arm = arms[0]
    expected_positions.append((target['source'], z['arms'][arm]['line'], z['arms'][arm]['column']))
ordinary = [loc(r) for r in new_notes if loc(r)[0] in scope['existing_sources']]
assert collections.Counter(ordinary) == collections.Counter(expected_positions)
observer = [r for r in new_notes if loc(r)[0] not in scope['existing_sources']]
assert len(observer) == 2 and all(loc(r)[0] == scope['new_sources'][0] for r in observer)
assert record['after'] == {'V16:error': 152, 'V19:note': 1028, 'V20:note': 869}
delta = dict(before=record['before'], after=record['after'], removed_V16=removed, removed_V20=removed_depth, rewritten_V19=rewritten_notes, new_V19=new_notes, all_other_diagnostics_preserved=True, position_mapping='AST match/arm ordinals for existing consumers; exact unchanged line blocks for other locations. Only the specified Option projection rewrites a V19 message and removes its V20.')
(out / 'architecture-delta.json').write_text(json.dumps(delta, indent=2, ensure_ascii=False) + '\n')
remaining = [r for r in after if r['ruleId'] == 'V16']
assert not any(loc(r)[0] in scope['sources'] for r in remaining)
(out / 'remaining-v16.json').write_text(json.dumps(remaining, indent=2, ensure_ascii=False) + '\n')
print('DIFFERENTIAL PASS', flush=True)
