#!/usr/bin/env python3
"""Frozen independent P1305-r2 observable probes; no candidate source access."""
import argparse
import base64
import datetime
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIAG = ROOT / '00_nucleo/diagnosticos'
PROFILES = {'default': [], 'html': ['html'], 'a11y': ['a11y-extras'], 'html+a11y': ['html', 'a11y-extras']}
FIXTURES = {
    'ordinary/std.typ': '#let x = 7\n',
    'ordinary/global.typ': '#let x = 8\n',
    'ordinary/map.typ': '#let x = 9\n',
    'ordinary/unlisted-name.typ': '#let x = 10\n',
    'ordinary/holder.typ': '#let saved = std\n',
    'reexport/std.typ': '#import std: *\n',
    'routes/inner.typ': '#let observation = (repr(std), { let alias = std; repr(alias) }, { import std as renamed; (repr(renamed), renamed.calc.abs(-7)) })\n',
    'routes/outer.typ': '#import "inner.typ" as inner\n#let observation = inner.observation\n',
    'main.typ': '#metadata((repr(std), { let alias = std; repr(alias) }, { import std as renamed; (repr(renamed), renamed.calc.abs(-7)) })) <p1305-r2>\n',
}

def probes():
    out = []
    def add(key, expression, obligation='match-vanilla'):
        out.append({'id': key, 'expression': expression, 'obligation': obligation})
    for n in [0, 1, 39, 40, 41, 42, 81, 256]:
        add(f'array-repr-{n}', f'repr(range({n}))')
        add(f'array-data-{n}', f'{{ let a = range({n}); let r = repr(a); (a.len(), a, a.map(x => x * 2)) }}')
    add('array-strings', 'repr(range(42).map(x => "item-" + str(x)))')
    add('array-distinct-after-boundary', '{ let a = range(42); a.at(40) = -777; (repr(a), a.len(), a.at(39), a.at(40), a.at(41), a) }')
    add('nested-inner', 'repr((range(41), ("tail",)))')
    add('nested-outer', 'repr(range(81).map(x => (x, x + 1)))')
    add('nested-multiline', 'repr((range(18), range(41)))')
    add('escaped-strings', r'repr(("a\n\"b\\c", "tail"))')
    for width in [49, 50, 51]:
        add(f'ascii-body-{width}', 'repr(("' + ('a' * (width - 5)) + '", 0))')
    add('content-op', r'repr(math.op([a \# \*], limits: false))')
    add('fields-dict-41', 'repr((' + ', '.join(f'k{i}: {i}' for i in range(41)) + '))', 'preserve-baseline')
    add('fields-args-41', '{ let f(..args) = repr(args); f(..range(41)) }', 'preserve-baseline')
    add('content-sequence-41', 'repr(range(41).map(x => strong(str(x))).join())', 'preserve-baseline')
    add('named-modules', '(repr(std), repr(color.map), repr(calc), repr(sym), repr(pdf))')
    add('named-aliases', '{ let a = std; let b = a; let c = color.map; (repr(a), repr(b), repr(c), a.calc.abs(-7)) }')
    add('ordinary-collisions', '{ import "ordinary/std.typ" as a; import "ordinary/global.typ" as b; import "ordinary/map.typ" as c; import "ordinary/unlisted-name.typ" as d; let alias = a; (repr(a), repr(b), repr(c), repr(d), repr(alias), a.x, b.x, c.x, d.x) }')
    add('ordinary-bare', '{ import "ordinary/std.typ"; (repr(std), std.x) }')
    add('global-bare', '{ import std; (repr(std), std.calc.abs(-7)) }')
    add('global-alias-bare', '{ let renamed = std; import renamed; (repr(renamed), renamed.calc.abs(-8)) }')
    add('global-bare-unbound', '{ import std; global }', 'preserve-primary-diagnostic')
    add('global-alias-bare-unbound', '{ let renamed = std; import renamed; global }', 'preserve-primary-diagnostic')
    add('global-dynamic-unnamed', '{ import (std); global }', 'match-primary-diagnostic')
    add('global-field-bare', '{ import "ordinary/holder.typ" as holder; import holder.saved; (repr(saved), saved.calc.abs(-13)) }')
    add('global-field-bare-unbound', '{ import "ordinary/holder.typ" as holder; import holder.saved; global }', 'preserve-primary-diagnostic')
    add('global-dynamic-named', '{ import (std) as chosen; (repr(chosen), chosen.calc.abs(-14)) }')
    add('global-dynamic-items', '{ import (std): calc as c; c.abs(-15) }')
    add('global-dynamic-wildcard', '{ import (std): *; calc.abs(-16) }')
    add('global-dynamic-block', '{ import { std }; none }', 'match-primary-diagnostic')
    add('global-dynamic-call', '{ let f() = std; import f(); none }', 'match-primary-diagnostic')
    add('global-rename', '{ import std as renamed; (repr(renamed), renamed.calc.abs(-9)) }')
    add('global-items', '{ import std: calc as c, rgb as color-fn; (c.abs(-10), repr(type(color-fn))) }')
    add('global-wildcard', '{ import std: *; (calc.abs(-11), repr(type(rgb))) }')
    add('reexport-collision', '{ import "reexport/std.typ" as r; let alias = r; (repr(r), repr(alias), r.calc.abs(-12), repr(r.calc)) }')
    add('imported-route', '{ import "routes/inner.typ" as r; r.observation }')
    add('nested-import-route', '{ import "routes/outer.typ" as r; r.observation }')
    for path in ['hsl', 'hsv', 'linear_rgb', 'std.hsl', 'std.hsv', 'std.linear_rgb', 'calc.nope', 'sym.nope', 'color.map.nope', 'std.nope', 'pdf.data-cell', 'pdf.header-cell', 'pdf.table-summary']:
        add('sentinel-' + path, f'repr(type({path}))')
    add('sentinel-lookups', '(repr(type(std.rgb)), calc.abs(-7), repr(sym.arrow), repr(type(color.map.turbo)), repr(type(pdf.attach)), repr(type(pdf.artifact)))')
    return out

def pin(path):
    data = pathlib.Path(path).read_bytes()
    return {'path': str(path), 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)}

def run(argv, cwd, side, profile, phase, probe):
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic_ns()
    try:
        p = subprocess.run(argv, cwd=cwd, capture_output=True, timeout=90)
        code, stdout, stderr, complete = p.returncode, p.stdout, p.stderr, True
    except subprocess.TimeoutExpired as e:
        code, stdout, stderr, complete = None, e.stdout or b'', e.stderr or b'', False
    row = {'id': probe['id'], 'expression': probe.get('expression'), 'obligation': probe.get('obligation'), 'side': side, 'profile': profile, 'phase': phase, 'argv': argv, 'cwd': str(cwd), 'started_at': started, 'duration_ns': time.monotonic_ns() - clock, 'exit_code': code, 'complete': complete}
    for name, raw in [('stdout', stdout), ('stderr', stderr)]:
        row[name] = raw.decode('utf-8', errors='replace')
        row[name + '_base64'] = base64.b64encode(raw).decode()
        row[name + '_sha256'] = hashlib.sha256(raw).hexdigest()
    try:
        row['value'] = json.loads(stdout) if code == 0 else None
    except ValueError:
        row['value'] = None
    return row

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--crystalline', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--mode', choices=['focal', 'matrix', 'refinement'], default='focal')
    parser.add_argument('--only', help='comma-separated probe IDs for bounded calibration')
    parser.add_argument('--supplement', action='store_true', help='Append disjoint focal observations before sealing; retain original evidence')
    args = parser.parse_args()
    if args.mode != 'focal':
        run_auxiliary(args)
        return
    output = pathlib.Path(args.output)
    previous = json.loads(output.read_text()) if args.supplement else None
    previous_pin = pin(output) if args.supplement else None
    if output.exists() and not args.supplement:
        raise SystemExit('Refusing to replace existing measurement')
    tmp = pathlib.Path(tempfile.mkdtemp(prefix='p1305-r2-oracle-'))
    for relative, source in FIXTURES.items():
        path = tmp / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(source)
    binaries = {'vanilla': '/usr/local/bin/typst', 'crystalline': args.crystalline}
    selected = [p for p in probes() if not args.only or p['id'] in args.only.split(',')]
    records = []
    jobs = [(p, profile) for p in selected for profile in PROFILES]
    for phase, order, sides in [('normal', jobs, list(binaries)), ('inverted', list(reversed(jobs)), list(reversed(binaries)))]:
        for p, profile in order:
            for side in sides:
                argv = [binaries[side], 'eval', p['expression'], '--format', 'json']
                if PROFILES[profile]:
                    argv += ['--features', ','.join(PROFILES[profile])]
                records.append(run(argv, tmp, side, profile, phase, p))
    if not args.only:
        for side in binaries:
            argv = [binaries[side], 'query', 'main.typ', '<p1305-r2>', '--field', 'value', '--one']
            records.append(run(argv, tmp, side, 'default', 'document', {'id': 'document-route'}))
    repeats = []
    for p, profile in jobs:
        for side in binaries:
            rows = [r for r in records if (r['id'], r['profile'], r['side']) == (p['id'], profile, side)]
            repeats.append({'id': p['id'], 'profile': profile, 'side': side, 'stable': all(rows[0][k] == rows[1][k] for k in ['exit_code', 'stdout_sha256', 'stderr_sha256', 'complete'])})
    def git(*a):
        return subprocess.check_output(['git', *a], cwd=ROOT).decode()
    data = {'schema': 'p1305-r2-oracle-measurement-v1', 'preflight_manifest': pin(DIAG / 'p1305-r2-preflight-manifest.json'), 'runner': pin(__file__), 'binary_pins': {s: pin(p) for s, p in binaries.items()}, 'baseline_source': {'head': git('rev-parse', 'HEAD').strip(), 'status': git('status', '--short'), 'diff_stat': git('diff', 'HEAD', '--stat'), 'diff_sha256': hashlib.sha256(git('diff', 'HEAD', '--binary').encode()).hexdigest()}, 'fixtures': [{'relative_path': p, 'content': s, **pin(tmp / p)} for p, s in FIXTURES.items()], 'profiles': PROFILES, 'probes': selected, 'runs': records, 'repetition': repeats, 'existing_color_channels': pin(DIAG / 'p1304-repr-refinement.json'), 'policy': 'No unmeasured case or invalid execution counts as success. preserve-baseline cases retain any documented preexisting vanilla divergence. Document query flags are unavailable; all four profile document routes are covered by independent Rust tests.'}
    if previous is not None:
        if {p['id'] for p in previous['probes']} & {p['id'] for p in selected}:
            raise SystemExit('Supplement must contain only new probe IDs')
        data['supplements'] = [previous]
        data['supplement_predecessor'] = previous_pin
        data['probes'] = previous['probes'] + data['probes']
        data['runs'] = previous['runs'] + data['runs']
        data['repetition'] = previous['repetition'] + data['repetition']
    with output.open('w' if args.supplement else 'x') as stream:
        json.dump(data, stream, indent=2, ensure_ascii=False)
        stream.write('\n')
    print(json.dumps({'output': str(output), 'runs': len(records), 'stable': all(r['stable'] for r in repeats), 'normal_errors': [{'id': r['id'], 'side': r['side'], 'profile': r['profile'], 'stderr': r['stderr']} for r in records if r['phase'] == 'normal' and r['exit_code'] and not r['id'].startswith('sentinel-')]}), flush=True)

def run_auxiliary(args):
    """Final mechanical execution only, after productive implementation exists."""
    output = pathlib.Path(args.output)
    if output.exists():
        raise SystemExit('Refusing to replace existing measurement')
    binaries = {'vanilla': '/usr/local/bin/typst', 'crystalline': args.crystalline}
    pins = {side: pin(path) for side, path in binaries.items()}
    if pins['vanilla']['sha256'] != '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8':
        raise SystemExit('Ratified vanilla identity changed')
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    if args.mode == 'matrix':
        original = DIAG / 'p1304-run-matrix.py'
        if pin(original)['sha256'] != '6c9f60029d2e7cb422d73afcc98c9970dbde9b04723046459f70aec065f82ebc':
            raise SystemExit('Historical matrix runner identity changed')
        spec = importlib.util.spec_from_file_location('p1304_matrix', original)
        matrix = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(matrix)
        catalog = json.loads((DIAG / 'p1304-probe-catalog.json').read_text())
        if catalog['probes'] != matrix.build_catalog()['probes']:
            raise SystemExit('Canonical corpus changed')
        before = matrix.source_state()
        data = matrix.execute(catalog, pins, 90)
        data.update({'source_before': before, 'source_after': matrix.source_state(), 'catalog': pin(DIAG / 'p1304-probe-catalog.json'), 'mechanical_runner': pin(original)})
    else:
        historical_path = DIAG / 'p1304-repr-refinement.json'
        historical = json.loads(historical_path.read_text())
        records = []
        jobs = [(m['path'], profile) for m in historical['map_measurements'] for profile in PROFILES]
        if len(jobs) != 60:
            raise SystemExit('Expected exactly fifteen maps in four profiles')
        for phase, ordered, sides in [('normal', jobs, list(binaries)), ('inverted', list(reversed(jobs)), list(reversed(binaries)))]:
            for path, profile in ordered:
                expression = f'(kind: repr(type({path})), n: {path}.len(), rgba: {path}.map(c => c.components().map(x => float(x))), hex: {path}.map(c => c.to-hex()), repr: repr({path}))'
                probe = {'id': path, 'expression': expression, 'obligation': 'repr-match-and-full-data-preserved'}
                for side in sides:
                    argv = [binaries[side], 'eval', expression, '--format', 'json']
                    if PROFILES[profile]:
                        argv += ['--features', ','.join(PROFILES[profile])]
                    row = run(argv, ROOT, side, profile, phase, probe)
                    value = row['value']
                    try:
                        if row['exit_code'] != 0 or value['kind'] != 'array' or value['n'] != len(value['rgba']) or value['n'] != len(value['hex']):
                            raise ValueError('incomplete structured array')
                        tokens = []
                        for rgba, color in zip(value['rgba'], value['hex']):
                            raw = color.removeprefix('#').lower()
                            if len(raw) == 6 and len(rgba) == 4 and rgba[3] == 1.0:
                                raw += 'ff'
                            if len(raw) != 8 or any(c not in '0123456789abcdef' for c in raw):
                                raise ValueError('unknown color token')
                            tokens.append('0x' + raw)
                        rgba8 = ','.join(tokens)
                        components = json.dumps(value['rgba'], separators=(',', ':'), ensure_ascii=False)
                        row['digests'] = {'rgba8_sha256': hashlib.sha256(rgba8.encode()).hexdigest(), 'full_components_sha256': hashlib.sha256(components.encode()).hexdigest(), 'cardinality': value['n'], 'indices': {str(i): value['rgba'][i] for i in [39, 40, value['n'] - 2, value['n'] - 1] if 0 <= i < value['n']}}
                        previous = next(m for m in historical['map_measurements'] if m['path'] == path)['profiles'][profile][side]
                        row['historical_data_preserved'] = row['digests']['full_components_sha256'] == previous['full_components_sha256'] and row['digests']['cardinality'] == previous['cardinality']
                    except (TypeError, KeyError, ValueError) as error:
                        row['normalization_unknown'] = str(error)
                    records.append(row)
        data = {'runs': records, 'historical': pin(historical_path), 'canonical_digest_policy': historical['canonical_digest_policy'], 'baseline_digests': [{ 'path': m['path'], 'rgba8_sha256': m['l0_expected_rgba8_sha256']} for m in historical['map_measurements']]}
    data.update({'schema': 'p1305-r2-' + args.mode + '-v1', 'runner': pin(__file__), 'manifest': pin(DIAG / 'p1305-r2-manifest.json'), 'started_at': started, 'finished_at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'binaries': pins, 'regime': 'executado sem atestação de isolamento técnico', 'unknown_policy': 'Any incomplete or unparseable mandatory result blocks. JSON null alone is not parse failure.'})
    with output.open('x') as stream:
        json.dump(data, stream, indent=2, ensure_ascii=False)
        stream.write('\n')
    print(json.dumps({'output': str(output), 'sha256': pin(output)['sha256'], 'mode': args.mode}), flush=True)

if __name__ == '__main__':
    main()
