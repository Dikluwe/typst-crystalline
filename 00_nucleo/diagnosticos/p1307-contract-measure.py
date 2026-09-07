#!/usr/bin/env python3
"""P1307 independent contract observations; no candidate implementation inputs."""
import argparse
import concurrent.futures
import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / '00_nucleo/diagnosticos/p1307-contract-measurement.json'
PROFILES = {'default': [], 'html': ['--features', 'html'], 'a11y': ['--features', 'a11y-extras'], 'html+a11y': ['--features', 'html,a11y-extras']}
FIXTURES = {'text.txt': 'olá\n', 'data.csv': 'z,a\n2,1\n', 'data.json': '{"z":2,"a":[true,null]}', 'data.yaml': 'z: 2\na: [true, null]\n', 'data.toml': 'z = 2\na = true\n', 'data.xml': '<r a="2">hi</r>', 'context.typ': '#set page(width: 100pt, height: 100pt)\n= Heading <probe>\n'}
VALUES = {
 'None': 'none', 'Bool': 'true', 'Int': '-42', 'Float': '1.25', 'Str': '"hello"',
 'Array': '(3, false, "x")', 'Dict': '(z: 2, a: 1)', 'Bytes': 'bytes((0, 65, 255))',
 'Symbol': 'sym.alpha', 'Content': '[hi]', 'Module': 'calc', 'Datetime': 'datetime(year: 2024, month: 2, day: 29)',
 'Func': 'calc.abs', 'Auto': 'auto', 'Length': '2pt', 'Relative': '2pt + 30%', 'Ratio': '25%',
 'Angle': '45deg', 'Color': 'rgb("#123456")', 'Stroke': 'stroke(paint: red, thickness: 2pt)',
 'Fraction': '2fr', 'Align': 'left + top', 'Gradient': 'gradient.linear(red, blue)',
 'Regex': 'regex("a+")', 'Tiling': 'tiling(size: (2pt, 2pt))[x]', 'Decimal': 'decimal("1.25")',
 'Duration': 'duration(seconds: 90)', 'Version': 'version(1, 2, 3)', 'Selector': 'heading.where(level: 1)',
 'Args': '((..xs) => xs)(1, a: 2)', 'State': 'state("probe", 0)', 'Counter': 'counter(heading)',
 'Label': '<probe>', 'Dir': 'ltr', 'Path': 'path("data.json")', 'Type': 'int',
}
EXTRA_VALUES = {
 'empty-string': '""', 'unicode': '"á漢😀"', 'escaping': r'"quote\" slash\\ tab\t cr\r nul\u{0} back\u{8} form\u{c}"',
 'multiline': '"a\\nb\\n"', 'multiline-real': r'"a\nb\n"', 'leading-trailing': r'"\na\n\n"',
 'yaml-strings': '("null", "true", "1", "1.0", "yes", "a: b", "#hash", "", "- x", " a ")',
 'array-empty': '()', 'dict-empty': '(:)', 'nested': '(z: (y: (1, 2), x: (a: "v")), a: ((q: 1), (q: 2)))',
 'none-field': '(z: 1, skip: none, a: 2)', 'none-array': '(1, none, 2)',
 'nan': 'float("NaN")', 'inf': 'float("inf")', 'negative-inf': 'float("-inf")', 'negative-zero': '-0.0',
 'float-limits': '(1e-300, 1e300, 1.0, 9007199254740991.0)', 'int-limits': '(-9223372036854775807 - 1, 9223372036854775807)',
 'content-empty': '[]', 'content-strong': '[*hi*]', 'content-sequence': '[a #text("b") c]',
 'symbol-modified': 'sym.arrow.r.double', 'bytes-empty': 'bytes(())',
}

def cases():
    result = []
    def add(key, expression, category, context=False):
        result.append({'id': key, 'expression': expression, 'expression_sha256': sha(expression.encode()), 'numbered_source': '\n'.join(f'{n+1}: {line}' for n,line in enumerate(expression.splitlines())), 'category': category, 'context': context})
    for name, value in VALUES.items():
        add('construct-' + name, f'(repr(type({value})), repr({value}))', 'constructibility')
    for fmt in ('json', 'toml', 'yaml'):
        for name, value in {**VALUES, **EXTRA_VALUES}.items():
            arg = f'(value: {value})' if fmt == 'toml' else value
            add(f'{fmt}-value-{name}', f'{fmt}.encode({arg})', 'serialization')
        for name, value in {'Location': 'query(<probe>).first().location()', 'LocatedContent': 'query(<probe>).first()'}.items():
            arg = f'(value: {value})' if fmt == 'toml' else value
            add(f'{fmt}-value-{name}', f'{fmt}.encode({arg})', 'serialization-context', True)
        v = '(z: (3, 2), a: (b: 1))'
        add(fmt+'-identity', f'(repr(type({fmt})), repr({fmt}), repr(type({fmt}.encode)), repr({fmt}.encode))', 'namespace')
        for label, call in {
            'default': f'{fmt}.encode({v})',
            'missing': f'{fmt}.encode()', 'excess': f'{fmt}.encode({v}, 2)',
            'unexpected-named': f'{fmt}.encode({v}, nope: true)', 'named-value': f'{fmt}.encode(value: {v})',
            'pretty-true': f'{fmt}.encode({v}, pretty: true)', 'pretty-false': f'{fmt}.encode({v}, pretty: false)',
            'pretty-wrong': f'{fmt}.encode({v}, pretty: "no")',
            'parent-with-identity': f'(repr(type({fmt}.with().encode)), repr({fmt}.with().encode))',
            'parent-with-bound': f'{fmt}.with(bytes("bad")).encode({v})',
            'parent-with-unexpected': f'{fmt}.with(nope: 9).encode({v})',
            'encoder-with-empty': f'{fmt}.encode.with()({v})',
            'encoder-with-bound': f'{fmt}.encode.with({v})()',
            'encoder-with-named': f'{fmt}.encode.with(pretty: false)({v})',
            'encoder-with-named-override': f'{fmt}.encode.with(pretty: false)({v}, pretty: true)',
            'encoder-with-repr': f'repr({fmt}.encode.with({v}))',
            'encoder-with-identity': f'(repr(type({fmt}.encode.with())), repr({fmt}.encode.with()))',
            'encoder-with-missing': f'{fmt}.encode.with()()',
            'encoder-with-excess': f'{fmt}.encode.with({v})(2)',
            'encoder-with-unexpected': f'{fmt}.encode.with(nope: 1)({v})',
            'unknown-member': f'{fmt}.nope',
            'roundtrip': f'{fmt}(bytes({fmt}.encode({v})))',
        }.items(): add(fmt+'-'+label, call, 'signature-with')
    for name, expr in {'nondict-int': '1', 'nondict-array': '(1,2)', 'nondict-none': 'none', 'none-only': '(x: none)', 'array-none': '(x: (none,))', 'heterogeneous': '(x: (1, "a", true))', 'empty': '(:)', 'ordered-tables': '(z: (q: 1), a: 2, y: (v: 3), b: 4)'}.items():
        add('toml-top-'+name, f'toml.encode({expr})', 'toml-domain')
    add('toml-multiline-error', '{\n  let marker = 1;\n  toml.encode((x: (1, none)))\n}', 'diagnostic-range')
    for name, value in {'Location': 'query(<probe>).first().location()', 'LocatedContent': 'query(<probe>).first()'}.items():
        add('construct-'+name, f'(repr(type({value})), repr({value}))', 'constructibility-context', True)
    for fmt in ('read', 'csv', 'json', 'yaml', 'toml', 'cbor', 'xml'):
        add(fmt+'-decoder-identity', f'(repr(type({fmt})), repr({fmt}))', 'preserve-decoder')
        add(fmt+'-decoder-missing', f'{fmt}()', 'preserve-decoder')
        filename = {'read': 'text.txt', 'csv': 'data.csv', 'cbor': 'data.cbor'}.get(fmt, 'data.'+fmt)
        add(fmt+'-decoder', f'{fmt}("{filename}")', 'preserve-decoder')
        add(fmt+'-decoder-with', f'{fmt}.with("{filename}")()', 'preserve-decoder')
    for fmt in ('csv', 'xml', 'read'):
        add(fmt+'-no-encode', fmt+'.encode', 'expected-absence')
        add(fmt+'-with-no-encode', fmt+'.with().encode', 'expected-absence')
    for key, expr in {'read-encoding': 'read("text.txt", encoding: "latin1")', 'csv-delimiter': 'csv("data.csv", delimiter: "ab")', 'json-malformed': 'json(bytes("{bad"))', 'toml-malformed': 'toml(bytes("x = ["))', 'yaml-malformed': 'yaml(bytes("x: ["))', 'xml-malformed': 'xml(bytes("<r>"))', 'cbor-malformed': 'cbor(bytes((255,)))'}.items():
        add(key, expr, 'preserve-decoder-diagnostic')
    for key, expr in {'identity': '(repr(type(cbor.encode)), repr(cbor.encode))', 'value': 'cbor.encode((z: 2, a: (none, bytes((0, 255)))))', 'roundtrip': 'cbor(cbor.encode((z: 2, a: (none, bytes((0, 255))))))', 'symbol': 'cbor.encode(sym.alpha)', 'content': 'cbor.encode([hi])', 'missing': 'cbor.encode()', 'excess': 'cbor.encode(1, 2)', 'named': 'cbor.encode(1, pretty: true)', 'parent-with': 'cbor.with().encode(1)', 'encoder-with': 'cbor.encode.with(1)()'}.items():
        add('cbor-'+key, expr, 'preserve-cbor')
    return result

def sha(data): return hashlib.sha256(data).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args): return subprocess.run(['git', *args], cwd=ROOT, capture_output=True, text=True, check=True).stdout
def state(): return {'at': now(), 'head': git('rev-parse','HEAD').strip(), 'status': git('status','--short','--untracked-files=all'), 'diff_stat': git('diff','HEAD','--stat'), 'diff': git('diff','HEAD')}

def diagnostics(stderr, expression):
    messages = re.findall(r'^(error|warning): (.*)$', stderr, re.M)
    hints = re.findall(r'^\s*= (?:hint|help): (.*)$', stderr, re.M)
    anchors = []
    rows = expression.splitlines(keepends=True)
    for match in re.finditer(r'┌─ ([^\n]+):(\d+):(\d+)\n(?:[^\n]*\n){1,3}?[^\n]*│ ([ \t]*)(\^+)\n', stderr):
        filename, line, col, _, carets = match.groups()
        line, col = int(line), int(col)
        if filename == '<input-expression>' and 0 < line <= len(rows):
            start = len(''.join(rows[:line-1]).encode()) + len(rows[line-1][:col].encode())
            end = start + len(rows[line-1][col:col+len(carets)].encode())
            anchors.append({'file': filename, 'line': line, 'column': col, 'half_open_utf8': [start,end], 'text': expression.encode()[start:end].decode()})
        else: anchors.append({'file': filename, 'line': line, 'column': col, 'unresolved': True})
    return {'messages': messages, 'hints': hints, 'anchors': anchors, 'primary_count': sum(m[0]=='error' for m in messages), 'lateral_count': sum(m[0]!='error' for m in messages)}

def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--baseline'); ap.add_argument('--focal', action='store_true'); args = ap.parse_args()
    prior = json.loads(OUT.read_text()) if OUT.exists() else None
    before = state(); began = now(); workspace = pathlib.Path(tempfile.mkdtemp(prefix='p1307-contract-', dir='/tmp'))
    for path, content in FIXTURES.items(): (workspace/path).write_text(content)
    (workspace/'data.cbor').write_bytes(bytes.fromhex('a2617a026161f5'))
    binaries = {'vanilla': '/usr/local/bin/typst'}
    if args.baseline: binaries['baseline'] = args.baseline
    if not args.focal and not args.baseline: raise SystemExit('--baseline required for full measurement')
    if args.baseline:
        pinned = json.loads((ROOT/'00_nucleo/diagnosticos/p1307-baseline.json').read_text())['binaries']
        assert sha(pathlib.Path(args.baseline).read_bytes()) == pinned['crystalline']['sha256']
        assert sha(pathlib.Path(binaries['vanilla']).read_bytes()) == pinned['vanilla']['sha256']
    records = []
    corpus = cases()
    for order in (['focal'] if args.focal else ['normal','repeat','reverse']):
        work = [(profile, flags, case, product, binary) for profile,flags in PROFILES.items() for case in corpus for product,binary in binaries.items()]
        if args.focal: work = [item for item in work if item[0]=='default']
        if order == 'reverse': work.reverse()
        def run(item):
            profile, flags, case, product, binary = item
            argv = [binary, 'eval', '--format', 'json', *flags]
            if case['context']: argv += ['--in', 'context.typ']
            argv += [case['expression']]
            start=now(); clock=time.monotonic()
            try:
                r=subprocess.run(argv,cwd=workspace,capture_output=True,text=True,timeout=20)
                row={'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
                if r.returncode == 0:
                    try: row.update(classification='value', value=json.loads(r.stdout))
                    except json.JSONDecodeError: row['classification']='Unknown'
                elif r.returncode == 1 and r.stderr.startswith('error:'):
                    row['classification']='diagnostic'; row['diagnostic']=diagnostics(r.stderr,case['expression'])
                else: row['classification']='Unknown'
            except subprocess.TimeoutExpired as err: row={'classification':'Unknown','reason':'timeout','stdout':str(err.stdout),'stderr':str(err.stderr)}
            row.update(order=order,profile=profile,case=case['id'],product=product,argv=argv,cwd=str(workspace),started=start,finished=now(),seconds=time.monotonic()-clock)
            return row
        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: records.extend(pool.map(run,work))
    payload={'protocol':'tekt-materializacao-segregada/full/1','role':'independent contract author','phase':'P1 observations only','attestation':'executado sem atestacao de isolamento tecnico','started':began,'finished':now(),'state_before':before,'state_after':state(),'preflight_sha256':sha((ROOT/'00_nucleo/diagnosticos/p1307-preflight.json').read_bytes()),'script_sha256':sha(pathlib.Path(__file__).read_bytes()),'baseline_artifact_sha256':sha((ROOT/'00_nucleo/diagnosticos/p1307-baseline.json').read_bytes()) if (ROOT/'00_nucleo/diagnosticos/p1307-baseline.json').exists() else None,'binaries':{k:{'path':v,'sha256':sha(pathlib.Path(v).read_bytes())} for k,v in binaries.items()},'environment':{k:os.environ.get(k) for k in ['LANG','LC_ALL','TYPST_FEATURES','TYPST_ROOT','SOURCE_DATE_EPOCH']},'fixtures':{**{k:{'source':v,'sha256':sha(v.encode())} for k,v in FIXTURES.items()},'data.cbor':{'hex':'a2617a026161f5','sha256':sha(bytes.fromhex('a2617a026161f5'))}},'cases':corpus,'profiles':PROFILES,'dispatch':'4 workers; submissions normal/repeat/integrally reversed; map retains submission order','runs':records,'prior_attempt':prior,'sources':{p:sha((ROOT/p).read_bytes()) for p in ['01_core/src/entities/value.rs','lab/typst-original/crates/typst-library/src/foundations/value.rs','lab/typst-original/crates/typst-library/src/foundations/bytes.rs','lab/typst-original/crates/typst-library/src/foundations/symbol.rs','lab/typst-original/crates/typst-library/src/foundations/content/mod.rs','lab/typst-original/crates/typst-library/src/loading/json.rs','lab/typst-original/crates/typst-library/src/loading/toml.rs','lab/typst-original/crates/typst-library/src/loading/yaml.rs']}}
    if prior is not None:
        payload['adapter_revision'] = {'revision': 1, 'hypothesis': 'CLI eval diagnostic columns are zero-based; resolve UTF-8 source offsets without subtracting one.', 'prior_sha256': sha(OUT.read_bytes()), 'affected': 'derived half-open ranges only; raw outputs unchanged', 'focal_controls': ['json-missing','toml-top-array-none','toml-multiline-error'], 'cost': 'one focal pass before full matrix'}
    payload['infrastructure_attempts'] = [{'at': '2026-09-07T16:06:00+00:00', 'time_precision': 'minute approximation; original tool receipt is causal record', 'command': ['python3','00_nucleo/diagnosticos/p1307-contract-measure.py','--baseline','/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst'], 'environment': 'default exec sandbox, private /dev/shm', 'exit': 1, 'error': "FileNotFoundError: [Errno 2] No such file or directory: '/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst'", 'classification': 'instrumentation invalid; no product conclusion', 'resolution': 'read-only host tmpfs access via reviewed require_escalated execution; baseline SHA checked before corpus', 'prior_measurement_preserved': True}]
    OUT.write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'path':str(OUT),'sha256':sha(OUT.read_bytes()),'cases':len(corpus),'runs':len(records),'unknown':sum(r['classification']=='Unknown' for r in records)}))
    if args.focal:
        for r in records:
            if r['classification']!='value': print(r['case'],r['classification'],repr(r['stderr'][:250]))

if __name__ == '__main__': main()
