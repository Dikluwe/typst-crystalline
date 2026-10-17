from pathlib import Path
import json
import re
import subprocess
import datetime
import hashlib

out = Path('00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16')
baseline = json.loads((out / 'baseline-result.json').read_text())
scope = json.loads((out / 'scope.json').read_text())
rows = json.loads((out / 'classification.json').read_text())
assert baseline['commands'][-1]['label'] == 'probe'
assert all(x['exit'] == (1 if x['label'] == 'architecture' else 0) for x in baseline['commands'])
assert json.loads((out / 'probe-baseline-result.json').read_text())['selected_e0004'] == 0
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest() == h for p,h in baseline['source_hashes'].items())
record = {'start': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'changes': [], 'files': {}}
for row in rows:
    path = Path(row['source'])
    text = path.read_text()
    pattern = r'^([ \t]*)(?:pub(?:\([^)]*\))? )?fn ' + row['function'] + r'\('
    matches = list(re.finditer(pattern, text, re.M))
    assert len(matches) == 1, row
    start = matches[0].start()
    opening = text.index('{', matches[0].end())
    balance = 1
    end = opening + 1
    while balance:
        balance += (text[end] == '{') - (text[end] == '}')
        end += 1
    body = text[start:end]
    prefix = 'Self' if row['source'] == '01_core/src/entities/value.rs' else 'Value'
    handled = set(re.findall(r'\b' + prefix + r'::([A-Z][A-Za-z0-9_]*)', body))
    assert handled <= set(scope['variants']) and handled
    complement = [name for name in scope['variants'] if name not in handled]
    def spelling(name):
        suffix = '' if name in ['None','Auto'] else '(_, _)' if name == 'LocatedContent' else '(_)'
        return prefix + '::' + name + suffix
    if row['function'] == 'is_none':
        old = 'matches!(self, Self::None)'
        assert body.count(old) == 1
        indent = matches[0][1] + '    '
        arm_indent = indent + '    '
        group = ('\n' + arm_indent + '| ').join(spelling(name) for name in complement)
        replacement = 'match self {\n' + arm_indent + 'Self::None => true,\n' + arm_indent + group + ' => false,\n' + indent + '}'
        after = body.replace(old, replacement, 1)
    else:
        wildcards = list(re.finditer(r'^([ \t]+)(?:_other|_)(?=\s*=>)', body, re.M))
        assert len(wildcards) == 1, row
        wildcard = wildcards[0]
        group = ('\n' + wildcard[1] + '| ').join(spelling(name) for name in complement)
        after = body[:wildcard.start()] + wildcard[1] + group + body[wildcard.end():]
    path.write_text(text[:start] + after + text[end:])
    record['changes'].append({'id':row['id'], 'source':row['source'], 'function':row['function'],
                              'before':body, 'after':after, 'handled':sorted(handled), 'complement':complement})
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
for path in scope['sources']:
    record['files'][path] = {'expected_after':Path(path).read_text(), 'before':baseline['files'][path]}
record['end'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'changed_functions':len(record['changes']), 'variants':len(scope['variants'])}))
