#!/usr/bin/env python3
"""B-only migration of frozen public snippets, without productive-code access."""
import pathlib, subprocess, difflib, json
D = pathlib.Path(__file__).resolve().parent
def add(name, text):
    p = D / name
    assert name.startswith('p1334-ab-') and not p.exists()
    subprocess.run(['apply_patch'], input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in text.splitlines())+'*** End Patch\n', text=True, check=True, capture_output=True)
ledger=[]
for n in range(1328,1334):
    src=f'p1333-ab-p{n}-successor.rs' if n<1333 else 'p1333-ab-tests.rs'
    before=(D/src).read_text(); after=before
    after=after.replace('format!("calc.abs() requer 1 argumento, recebeu {count}")','String::from("missing argument: value")')
    after=after.replace('"calc.abs() requer 1 argumento, recebeu 0"','"missing argument: value"')
    after=after.replace('"calc.abs() requer 1 argumento, recebeu 2"','"unexpected argument"')
    if n in [1330,1331]:
        after=after.replace('"argumento nomeado inesperado: \'bad\'"','"missing argument: value"')
    if n==1329:
        after=after.replace('"argumento nomeado inesperado: \'bad\'"','"unexpected argument"')
    if n==1332:
        after=after.replace('} else { None }, message, call, "abs", false);','} else { Some(call) }, message, call, "abs", false);')
        after=after.replace('            assert_eq!(d.trace.len(), 1, "{text}: {d:?}");','            if message == "missing argument: value" {\n                assert!(d.trace.is_empty(), "{text}: {d:?}");\n                continue;\n            }\n            assert_eq!(d.trace.len(), 1, "{text}: {d:?}");')
    if n==1333:
        a=after.index('    fn p1333_native_valid_first_never_substitutes_a_later_failure()')
        b=after.index('    #[test]',a)
        part=after[a:b]
        # None synthesis orders remaining positional before named.
        old='"argumento nomeado inesperado: \'bad\'"'
        assert part.count(old)==3
        part=part.replace(old,'"unexpected argument"',1).replace(old,'"unexpected argument: bad"',1).replace(old,'"missing argument: value"',1)
        # Historical missing fixture had contradictory Some metadata. Restore None
        # for this normative guard; first-invalid robustness fixture is retained.
        begin=part.index('            args.occurrences = Some(vec![ArgOccurrence {')
        end=part.index('            bare_diagnostic(',begin)
        part=part[:begin]+'            // P1334: coherent synthetic missing fixture (historical Some was stale).\n'+part[end:]
        part=part.replace('"missing argument: value",\n                Span::detached(),','"missing argument: value",\n                aggregate,')
        after=after[:a]+part+after[b:]
    dst=f'p1334-ab-p{n}-successor.rs'
    add(dst,after)
    ledger.append({'source':src,'successor':dst,'diff':''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile=src,tofile=dst))})
add('p1334-ab-migration-ledger.json',json.dumps({'policy':'Only P1334 missing/surplus guards and resulting anchors/traces migrate. All prior first-invalid coverage stays; historical inconsistent Some fixtures are local robustness outside causal parity. One stale missing fixture becomes coherent None, explicitly shown in diff.','native':ledger},indent=2)+'\n')
# Reuse only public test scaffolding, not old fixtures, for the new coherent suite.
old=(D/'p1333-ab-tests.rs').read_text()
prefix=old[:old.index('    fn public_error(')].replace('mod p1333_tests','mod p1334_tests')
new=(D/'p1334-ab-new-native.inc').read_text()
add('p1334-ab-tests.rs',prefix+new+'\n}\n')
# Public old dispatcher tests remain installed separately; new module starts
# with their identity-preservation scaffold, renamed to distinguish its scope.
old=(D/'p1334-public-dispatch-tests.rs').read_text()
old=old.replace('mod tests {','mod p1334_dispatch_tests {').replace('p1321_csv_transport','p1334_abs_transport')
old=old.replace('let native = Func::native("alias-not-csv", crate::compiler::stdlib::native_csv);','let module = crate::compiler::stdlib::make_calc_module();\n        let Value::Func(native) = module.scope().get("abs").unwrap().clone() else { panic!() };')
old=old.replace('Func::native("csv",','Func::native("abs",')
old=old.replace('native_json, native_json_encode, native_panic, native_read,','native_csv, native_json, native_json_encode, native_panic, native_read,')
old=old.replace('Func::native("alias", native_json_encode),','Func::native("alias", native_csv),\n            Func::native("alias", native_json_encode),')
add('p1334-ab-dispatch-tests.rs',old)
