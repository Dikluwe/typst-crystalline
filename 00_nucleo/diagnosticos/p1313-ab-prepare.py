"""Independent prepatch suite construction; writes exclusively via apply_patch."""
import json
import subprocess
from pathlib import Path

BASE = Path(__file__).resolve().parent
def add(path, text):
    assert not path.exists(), path
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + x + '\n' for x in text.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)

cases = []
def case(id, kind, expr, **kw):
    cases.append(dict(id=id, kind=kind, expr=expr, **kw))

old = json.loads((BASE / 'p1312-ab-cases.json').read_text())['cases']
for c in old:
    if c['kind'] in ('target', 'symbol') and c['id'] not in ('bytes', 'empty-bytes', 'stored-native'):
        expression = c['expr'].replace('read', 'csv').replace('encoding: none', 'delimiter: ";"').replace('encoding: "utf8"', 'row-type: array').replace('encoding: 5', 'delimiter: 5')
        case('cast-' + c['id'], 'cast' if c['kind'] == 'target' else 'symbol', expression)
    if not c['id'].startswith('csv-'):
        case('preserve-' + c['id'], 'control', c['expr'])

valid = {
    'rows': 'csv(bytes("a,b\\n1,2"))',
    'empty': 'csv(bytes(()))',
    'empty-dict': 'csv(bytes(""), row-type: dictionary)',
    'quoted': 'csv(bytes("\\"a,b\\",\\"c\\"\\"d\\"\\n\\"x\\ny\\",z"))',
    'unicode': 'csv(bytes("α,ação\\n東京,🦀"))',
    'delimiter': 'csv(bytes("a;b\\n1;2"), delimiter: ";")',
    'dictionary': 'csv(bytes("a,b\\n1,2\\n3,4"), row-type: dictionary)',
    'dict-delimiter': 'csv(bytes("a;b\\n1;2"), row-type: dictionary, delimiter: ";")',
    'header-only': 'csv(bytes("a,b"), row-type: dictionary)',
    'empty-fields': 'csv(bytes(",b\\n1,"))',
    'crlf': 'csv(bytes("a,b\\r\\n1,2\\r\\n"))',
    'with': 'csv.with(bytes("a;b\\n1;2"), delimiter: ";")(row-type: dictionary)',
    'nested-with': 'csv.with(delimiter: ";").with(bytes("a;b"))()',
    'args': 'csv(..arguments(row-type: dictionary, bytes("a,b\\n1,2")))',
    'sink': '{ let f(..a) = csv(..a); f(bytes("a,b")) }',
    'alias': '{ let f = csv; f(bytes("a,b")) }',
    'mapped': 'csv(..arguments(bytes("a,b")).map(x => x))',
    'path-looking': 'csv(bytes("missing.csv"))',
    'raw-nul': 'csv(bytes((97, 0, 44, 98)))',
    'data-file': 'csv(read("data.csv", encoding: none))',
}
for id, expr in valid.items():
    case('bytes-' + id, 'value', expr)

for id, expr in {
    'str': 'csv("data.csv")', 'path': 'csv(path("data.csv"))',
    'dictionary': 'csv("data.csv", row-type: dictionary)',
    'missing': 'csv()', 'missing-named': 'csv(delimiter: ";")',
    'source-named': 'csv(source: "data.csv")', 'path-named': 'csv(path: "data.csv")',
    'excess': 'csv("data.csv", 42)', 'unknown': 'csv("data.csv", strange: 2)',
    'unknown-before-cast': 'csv(strange: 2, false)', 'unknown-after-cast': 'csv(false, strange: 2)',
    'absent': 'csv("absent.csv")', 'option-before-io': 'csv("absent.csv", delimiter: "ab")',
    'duplicate-delimiter': 'csv.with(delimiter: "ab")("data.csv", delimiter: ",")',
    'duplicate-row': 'csv.with(row-type: 42)("data.csv", row-type: array)',
}.items():
    case('legacy-' + id, 'control', expr)

for id, expr, surrogate in [
    ('unequal', 'csv(bytes("a,b\\n1"))', 'csv("unequal.csv")'),
    ('unequal-dict', 'csv(bytes("a,b\\n1"), row-type: dictionary)', 'csv("unequal.csv", row-type: dictionary)'),
    ('delimiter-empty', 'csv(bytes("a,b"), delimiter: "")', 'csv("data.csv", delimiter: "")'),
    ('delimiter-long', 'csv(bytes("a,b"), delimiter: "ab")', 'csv("data.csv", delimiter: "ab")'),
    ('delimiter-unicode', 'csv(bytes("a,b"), delimiter: "é")', 'csv("data.csv", delimiter: "é")'),
    ('delimiter-int', 'csv(bytes("a,b"), delimiter: 42)', 'csv("data.csv", delimiter: 42)'),
    ('row-string', 'csv(bytes("a,b"), row-type: "dictionary")', 'csv("data.csv", row-type: "dictionary")'),
    ('row-type', 'csv(bytes("a,b"), row-type: str)', 'csv("data.csv", row-type: str)'),
    ('row-bool', 'csv(bytes("a,b"), row-type: false)', 'csv("data.csv", row-type: false)'),
    ('option-priority', 'csv(bytes("a,b"), row-type: false, delimiter: "ab")', 'csv("data.csv", row-type: false, delimiter: "ab")'),
    ('unknown', 'csv(bytes("a,b"), strange: 2)', 'csv("data.csv", strange: 2)'),
    ('excess', 'csv(bytes("a,b"), 42)', 'csv("data.csv", 42)'),
    ('duplicate-delimiter', 'csv.with(delimiter: "ab")(bytes("a,b"), delimiter: ",")', 'csv.with(delimiter: "ab")("data.csv", delimiter: ",")'),
    ('duplicate-row', 'csv.with(row-type: 42)(bytes("a,b"), row-type: array)', 'csv.with(row-type: 42)("data.csv", row-type: array)'),
]:
    case('bytes-legacy-' + id, 'legacy-bytes', expr, surrogate=surrogate)

add(BASE / 'p1313-ab-cases.json', json.dumps({'schema': 'p1313-ab-cases-v1', 'cases': cases}, ensure_ascii=False, indent=2) + '\n')
for name, raw in {'data.txt': 'ação α\n', 'data.csv': 'a,b\n1,2\n', 'unequal.csv': 'a,b\n1\n', 'paths.typ': '#let rooted = path("data.txt")\n'}.items():
    add(Path('/tmp/p1313-ab-fixtures') / name, raw)
runner = (BASE / 'p1312-ab-runner.py').read_text().replace('p1312', 'p1313').replace('/dev/shm/p1311-target.JE8Cyy/release/typst', '/dev/shm/p1312-target.B8uLa9/release/typst').replace('4bbced9ec793fb84daa560e0d965958b33e1eb2f5ecd4bfd3bc30b5900bb09fb', '446c3ccfa5ef7f5eaf6031543f4d28566dac6eb53dca842ec7eb16acb084fbf0').replace('"paths.typ")', '"paths.typ", "unequal.csv")')
runner = runner.replace('for path, digest in binaries.values():\n        assert sha(path)', 'for path, digest in binaries.values():\n        assert sha(path)', 1)
runner = runner.replace('for product, (binary, digest) in binaries.items():', 'for product, (binary, digest) in binaries.items():')
runner = runner.replace('argv = [binary, "eval", case["expr"], *flags]', 'expression = case.get("surrogate", case["expr"]) if product == "baseline" and case["kind"] == "legacy-bytes" else case["expr"]\n                    argv = [binary, "eval", expression, *flags]')
add(BASE / 'p1313-ab-runner.py', runner)
print(json.dumps({'cases': len(cases)}))
