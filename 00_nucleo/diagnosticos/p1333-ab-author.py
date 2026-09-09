#!/usr/bin/env python3
"""Author-time historical migration, never imports or executes candidate code."""
import pathlib
import subprocess
import difflib
import json

D = pathlib.Path(__file__).resolve().parent

def add(name, content):
    path = D / name
    assert name.startswith('p1333-ab-') and not path.exists()
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n'
    patch += ''.join('+' + line + '\n' for line in content.splitlines())
    patch += '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)

def replace_once(text, before, after):
    assert text.count(before) == 1, (before, text.count(before))
    return text.replace(before, after)

ledger = []
for step in range(1328, 1333):
    src = f'p1332-ab-p{step}-successor.rs' if step < 1332 else 'p1332-ab-tests.rs'
    dst = f'p1333-ab-p{step}-successor.rs'
    before = (D / src).read_text()
    after = before
    if step == 1328:
        after = replace_once(after, '"argumento nomeado inesperado: \'unexpected\'",', 'CONTENT_ERROR,')
        after = replace_once(after, '&format!("calc.abs() requer 1 argumento, recebeu {count}"),', '&if count == 0 { format!("calc.abs() requer 1 argumento, recebeu {count}") } else { CONTENT_ERROR.into() },')
    elif step == 1329:
        after = replace_once(after, '"argumento nomeado inesperado: \'bad\'",', 'if matches!(&value, Value::Length(l) if l.abs.to_pt() != 0.0 && l.em != 0.0) { MIXED_ERROR } else { "argumento nomeado inesperado: \'bad\'" },')
        after = replace_once(after, '"calc.abs() requer 1 argumento, recebeu 2",', 'if matches!(&value, Value::Length(l) if l.abs.to_pt() != 0.0 && l.em != 0.0) { MIXED_ERROR } else { "calc.abs() requer 1 argumento, recebeu 2" },')
    elif step == 1330:
        after = replace_once(after, '"argumento nomeado inesperado: \'bad\'",', 'if count == 0 { "argumento nomeado inesperado: \'bad\'" } else { OVERFLOW },')
        after = replace_once(after, '&format!("calc.abs() requer 1 argumento, recebeu {count}"),', '&if count == 0 { format!("calc.abs() requer 1 argumento, recebeu {count}") } else { OVERFLOW.into() },')
    elif step == 1331:
        after = replace_once(after, 'for (value, _) in family_values(features)', 'for (value, kind) in family_values(features)')
        after = replace_once(after, '"argumento nomeado inesperado: \'bad\'",', '&if count == 0 { "argumento nomeado inesperado: \'bad\'".into() } else { rejection(kind) },')
        after = replace_once(after, '&format!("calc.abs() requer 1 argumento, recebeu {count}"),', '&if count == 0 { format!("calc.abs() requer 1 argumento, recebeu {count}") } else { rejection(kind) },')
    else:
        after = replace_once(after, '"calc.abs() requer 1 argumento, recebeu 2",', '"expected integer, float, length, angle, ratio, fraction, or decimal, found boolean",')
        after = replace_once(after, '"argumento nomeado inesperado: \'bad\'",', '"expected integer, float, length, angle, ratio, fraction, or decimal, found boolean",')
        after = replace_once(after, 'traced_error(text, None, message, call, "abs", false);', 'traced_error(text, if text.contains("with(false,") { Some("false") } else { None }, message, call, "abs", false);')
    add(dst, after)
    ledger.append({'source': src, 'successor': dst, 'diff': ''.join(difflib.unified_diff(before.splitlines(True), after.splitlines(True), fromfile=src, tofile=dst))})
add('p1333-ab-migration-ledger.json', json.dumps({'policy': 'Only first-invalid guard expectations change; test names and all remaining prior coverage preserved. P1332 traced guards migrate primary origin from detached to false.', 'native': ledger}, indent=2, ensure_ascii=False) + '\n')
