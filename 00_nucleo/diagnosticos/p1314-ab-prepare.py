"""Pre-candidate contract construction from L0 and historical public observations."""
import json
from pathlib import Path
import subprocess

BASE = Path(__file__).resolve().parent
def add(path, raw):
    assert not path.exists(), path
    patch = '*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in raw.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)

old = json.loads((BASE/'p1313-ab-cases.json').read_text())['cases']
delta_ids = {'legacy-option-before-io', 'legacy-duplicate-delimiter', 'legacy-duplicate-row'}
delta_ids.update('bytes-legacy-'+s for s in ['delimiter-empty','delimiter-long','delimiter-unicode','delimiter-int','row-string','row-type','row-bool','option-priority','duplicate-delimiter','duplicate-row'])
cases = [dict(id=c['id'], kind='target' if c['id'] in delta_ids else 'control', expr=c['expr'], historical_p1313=True) for c in old]
def case(id, expr, kind='target'):
    cases.append(dict(id='options-'+id,kind=kind,expr=expr,historical_p1313=False))
for name, values in [('delimiter', ['""','"ab"','"é"','42','1.5','false','none','auto','(a: 1)','(1, 2)','x => x','csv','str','bytes(";")','[x]','2pt','red','<x>']), ('row-type',['"array"','42','1.5','false','none','auto','(a: 1)','(1, 2)','x => x','csv','str','bytes("a")','[x]','2pt','red','<x>','sym.alpha'])]:
    for i,v in enumerate(values): case(name+'-type-'+str(i), 'csv(bytes("a,b"), '+name+': '+v+')')
for name, bad, good in [('delimiter','"ab"','","'),('row-type','42','array')]:
    for suffix,expr in {
        'alias': '{ let f = csv; f(bytes("a,b"), NAME: BAD) }',
        'with': '{ let f = csv.with(NAME: BAD); f(bytes("a,b")) }',
        'nested-with': 'csv.with(NAME: BAD).with(bytes("a,b"))()',
        'first-bad-last-good': 'csv.with(NAME: BAD)(bytes("a,b"), NAME: GOOD)',
        'first-good-last-bad': 'csv.with(NAME: GOOD)(bytes("a,b"), NAME: BAD)',
        'bad-middle': 'csv.with(NAME: GOOD).with(NAME: BAD)(bytes("a,b"), NAME: GOOD)',
        'args': '{ let a = arguments(NAME: BAD); csv(bytes("a,b"), ..a) }',
        'args-overridden': '{ let a = arguments(NAME: BAD); csv(bytes("a,b"), ..a, NAME: GOOD) }',
        'dict-spread': 'csv(bytes("a,b"), ..(NAME: BAD))',
        'sink': '{ let f(..a) = csv(..a); f(bytes("a,b"), NAME: BAD) }',
        'filter': 'csv(..arguments(bytes("a,b"), NAME: BAD).filter((..a) => true))',
        'map': 'csv(bytes("a,b"), ..arguments(NAME: BAD).map((..a) => BAD))',
        'multiline': 'csv(\n  row-type: array,\n  bytes("a,b"),\n  delimiter: "ab",\n)' if name=='delimiter' else 'csv(\n  delimiter: ",",\n  bytes("a,b"),\n  row-type: 42,\n)',
        'absent-str': 'csv("absent.csv", NAME: BAD)',
        'absent-path': 'csv(path("absent.csv"), NAME: BAD)',
        'before-decode': 'csv(bytes("a,b\\n1"), NAME: BAD)',
    }.items(): case(name+'-'+suffix,expr.replace('NAME',name).replace('BAD',bad).replace('GOOD',good))
for id,expr in {
    'delimiter-last-valid': 'csv.with(delimiter: ",")(bytes("a;b\\n1;2"), delimiter: ";")',
    'row-last-valid': 'csv.with(row-type: array)(bytes("a,b\\n1,2"), row-type: dictionary)',
    'row-last-array': 'csv.with(row-type: dictionary)(bytes("a,b\\n1,2"), row-type: array)',
    'both-last-valid': 'csv.with(row-type: array, delimiter: ",").with(row-type: dictionary, delimiter: ";")(bytes("a;b\\n1;2"))',
    'delimiter-before-row': 'csv.with(row-type: 42).with(delimiter: "ab")(bytes("a,b"))',
    'all-delimiter-before-row': 'csv.with(row-type: 42, delimiter: ",")(bytes("a,b"), delimiter: "ab")',
    'first-delimiter-invalid': 'csv.with(delimiter: "é").with(delimiter: "ab")(bytes("a,b"))',
    'first-row-invalid': 'csv.with(row-type: 42).with(row-type: str)(bytes("a,b"))',
}.items(): case(id,expr, 'value' if 'last-' in id else 'target')
for id,expr in {
    'symbol': 'csv(bytes("a,b"), delimiter: sym.alpha)',
    'symbol-with': 'csv.with(delimiter: sym.alpha)(bytes("a,b"), delimiter: ",")',
    'symbol-args': '{ let a = arguments(delimiter: sym.alpha); csv(bytes("a,b"), ..a) }',
}.items(): case(id,expr,'symbol')
for id,expr in {
    'unknown-before-options': 'csv(bytes("a,b"), unknown: 1, delimiter: "ab")',
    'unknown-before-source': 'csv(42, unknown: 1, delimiter: "ab")',
    'source-before-options': 'csv(42, delimiter: "ab", row-type: 42)',
    'missing-before-options': 'csv(delimiter: "ab", row-type: 42)',
    'excess-valid': 'csv(bytes("a,b"), 42, delimiter: ",")',
    'duplicate-syntax-delimiter': 'csv(bytes("a,b"), delimiter: "ab", delimiter: ",")',
    'duplicate-syntax-row': 'csv(bytes("a,b"), row-type: 42, row-type: array)',
    'utf8-invalid': 'csv(bytes((255, 44, 98)))',
    'utf8-invalid-dict': 'csv(bytes((255, 44, 98)), row-type: dictionary)',
    'parse-unequal-with': 'csv.with(delimiter: ";")(bytes("a;b\\n1"))',
    'read-options-preserved': 'read.with(encoding: 42)("data.txt", encoding: none)',
}.items(): case(id,expr,'control')
assert len({c['id'] for c in cases})==len(cases)
add(BASE/'p1314-ab-cases.json',json.dumps(dict(schema='p1314-ab-cases-v1',cases=cases),ensure_ascii=False,indent=2)+'\n')
for name,raw in {'data.txt':'ação α\n','data.csv':'a,b\n1,2\n','unequal.csv':'a,b\n1\n','single.csv':'a,b\n','paths.typ':'#let rooted = path("data.txt")\n'}.items(): add(Path('/tmp/p1314-ab-fixtures')/name,raw)
runner=(BASE/'p1313-ab-runner.py').read_text().replace('p1313','p1314').replace('/dev/shm/p1312-target.B8uLa9/release/typst','/dev/shm/p1313-target.keFg93/release/typst').replace('446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0','cefb4b485cc25ae98871cfbd7a76925d0333d6d26906dc7f7426868e899285ce')
runner=runner.replace('expression = case.get("surrogate", case["expr"]) if product == "baseline" and case["kind"] == "legacy-bytes" else case["expr"]','expression = case["expr"]')
add(BASE/'p1314-ab-runner.py',runner)
print(json.dumps(dict(cases=len(cases),historical=len(old),delta_ids=sorted(delta_ids))))
