from pathlib import Path
import json
import re
import subprocess
import datetime
import hashlib

out = Path('00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16')
baseline = json.loads((out / 'baseline-result.json').read_text())
assert baseline['commands'][-1]['label'] == 'probe'
assert all(x['exit'] == (1 if x['label'] == 'architecture' else 0) for x in baseline['commands'])
assert json.loads((out / 'probe-baseline-result.json').read_text())['selected_e0004'] == 0
scope = json.loads((out / 'scope.json').read_text())
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest() == h for p, h in baseline['source_hashes'].items())
record = {'start': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'changes': [], 'files': {}}

def replace_function(source, name, old, new):
    path = Path(source)
    text = path.read_text()
    start = re.search(r'^(?:pub(?:\([^)]*\))? )?fn ' + name + r'\(', text, re.M)
    assert start, name
    end = re.search(r'^(?:pub(?:\([^)]*\))? )?fn |^#\[cfg\(test\)\]', text[start.end():], re.M)
    stop = start.end() + end.start() if end else len(text)
    body = text[start.start():stop]
    assert body.count(old) == 1, (name, body.count(old))
    changed = body.replace(old, new, 1)
    path.write_text(text[:start.start()] + changed + text[stop:])
    record['changes'].append({'source': source, 'function': name, 'before': body, 'after': changed})

root = '01_core/src/compiler/eval/selector_matching.rs'
textual = '01_core/src/compiler/eval/selector_matching/textual.rs'
replace_function(root, 'native_element_base', '        _ => None,', '''        Selector::NodeKind(_)
        | Selector::DynKind(_)
        | Selector::Text(_)
        | Selector::Regex(_)
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => None,''')
replace_function(textual, 'is_text_rule', '    matches!(selector, Selector::Text(_) | Selector::Regex(_))', '''    match selector {
        Selector::Text(_) | Selector::Regex(_) => true,
        Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => false,
    }''')
replace_function(textual, 'textual_selector_matches', '        _ => false,', '''        Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => false,''')
replace_function(textual, 'textual_match_ranges', '        _ => Vec::new(),', '''        Selector::Text(_)
        | Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => Vec::new(),''')
fallback = '''        Selector::Text(_)
        | Selector::NodeKind(_)
        | Selector::DynKind(_)
        | Selector::Regex(_)
        | Selector::Label(_) => false,
    }'''
replace_function('01_core/src/compiler/realize/bare_math.rs', 'captures_equation_rule',
                 '        _ => false,\n    }', fallback)
replace_function('01_core/src/compiler/realize/table_cells.rs', 'captures_rule',
                 '        _ => false,\n    }', fallback)
subprocess.run(['rustfmt', '--edition', '2021', '--config', 'skip_children=true', *scope['sources']], check=True)
for path in scope['sources']:
    record['files'][path] = {'expected_after': Path(path).read_text(), 'before': baseline['files'][path]}
record['end'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
(out / 'materialization.json').write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({'changed_functions': len(record['changes']), 'sources': scope['sources']}))
