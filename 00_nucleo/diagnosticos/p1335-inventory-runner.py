#!/usr/bin/env python3
"""P1335 author A: fresh enumeration, unilateral observations, frozen route union.

No candidate matrix, classification, or selection is an input to this program.
"""
import argparse
import concurrent.futures
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / '00_nucleo/diagnosticos'
TEMP = Path('/tmp/p1335-inventory.mZcVD2')
VANILLA_SHA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
PROFILES = {'default': [], 'html': ['html'], 'a11y': ['a11y-extras'], 'html+a11y': ['html', 'a11y-extras']}


CAPABILITIES = dict(executor='/root/p1335_inventory', inherited_context='Bounded inventory task; no P1335 candidate matrix/classification/selection.', permitted_read='P1335 plan, skill/references, ADRs, historical inventory/catalog/method, current source, root baseline/build/manifest, pinned binaries', forbidden_read='P1335 candidate matrices, classification, cohorts, selection', permitted_write=['00_nucleo/diagnosticos/p1335-inventory-*','00_nucleo/diagnosticos/p1335-probe-catalog.json',str(TEMP)], technical_limit='Shared tools/filesystem: procedural segregation, no isolation attestation')
READ_INPUTS = [ROOT/'00_nucleo/materialization/typst-passo-1335.md', Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md'), Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md'), Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md'), *[OUT/name for name in ['p1322-inventory-runner-r2.py','p1322-inventory-runner.py','p1322-inventory-incident.json','p1322-inventory-focal-validation.json','p1322-probe-catalog.json']], *[ROOT/p for p in ['00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md','00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md','lab/surface-inventory/src/main.rs','lab/surface-inventory/Cargo.toml','lab/surface-inventory/Cargo.lock','lab/surface-inventory/extra_seeds.json','lab/typst-original/crates/p1140-inventory/src/main.rs','lab/typst-original/crates/p1140-inventory/Cargo.toml','lab/typst-original/Cargo.toml','lab/typst-original/Cargo.lock']]]

def validate_product(args):
    assert sha(args.binary) == args.expected_sha
    if args.side == 'vanilla':
        assert args.expected_sha == VANILLA_SHA
    else:
        build = json.loads((OUT/'p1335-build.json').read_text())
        assert build['exit'] == 0, 'root build not complete'
        assert Path(build['candidate']['path']).resolve() == Path(args.binary).resolve()
        assert build['candidate']['sha256'] == args.expected_sha


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def pin(path):
    path = Path(path)
    return {'path': str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path), 'sha256': sha(path)}

def publish(name, payload):
    path = OUT / name
    if path.exists():
        raise RuntimeError(f'refusing overwrite: {path}')
    content = json.dumps(payload, ensure_ascii=False, indent=2) + '\n'
    patch = f'*** Begin Patch\n*** Add File: {path}\n' + ''.join('+' + line + '\n' for line in content.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)
    return pin(path)

def command(argv, env_delta=None, timeout=1200):
    begin, tick = utc(), time.monotonic_ns()
    env_delta = {'PYTHONDONTWRITEBYTECODE': '1', **(env_delta or {})}
    try:
        p = subprocess.run(argv, cwd=ROOT, env={**os.environ, **env_delta}, capture_output=True, text=True, timeout=timeout)
        code, out, err, unknown = p.returncode, p.stdout, p.stderr, 'signal' if p.returncode < 0 else None
    except subprocess.TimeoutExpired as e:
        code, out, err, unknown = None, e.stdout or '', e.stderr or '', 'timeout'
        out = out.decode() if isinstance(out, bytes) else out
        err = err.decode() if isinstance(err, bytes) else err
    return dict(argv=argv, cwd=str(ROOT), env_delta=env_delta, started_utc=begin, ended_utc=utc(), duration_ns=time.monotonic_ns()-tick, exit_code=code, stdout=out, stderr=err, unknown=unknown)

def identity(side):
    scopes = ['lab/typst-original'] if side == 'vanilla' else ['00_nucleo/prompts', '01_core', '02_shell', '03_infra', '04_wiring', 'lab/surface-inventory', 'Cargo.toml', 'Cargo.lock']
    raw = subprocess.run(['git', 'ls-files', '-z', '--', *scopes], cwd=ROOT, capture_output=True, check=True).stdout
    files = {p.decode(): sha(ROOT / p.decode()) for p in raw.split(b'\0') if p}
    digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    return dict(scopes=scopes, tracked_files=len(files), sha256=digest, method='SHA256 canonical sorted compact JSON mapping tracked relative paths to current SHA256', files=files)

def common(args):
    return dict(author='/root/p1335_inventory', role='A', regime='executado sem atestação de isolamento', runner=pin(__file__), baseline_receipt=pin(ROOT / args.baseline), manifest=pin(ROOT / args.manifest), head=command(['git', 'rev-parse', 'HEAD'])['stdout'].strip(), source_state='working tree não commitado', diff_head_stat=command(['git', 'diff', 'HEAD', '--stat', '--', '.', ':(exclude)00_nucleo/materialization/**', ':(exclude)00_nucleo/context/**'])['stdout'], temporary_root=str(TEMP))

def build(args):
    side = args.side
    package = 'p1140-inventory' if side == 'vanilla' else 'surface-inventory-p1140'
    manifest = 'lab/typst-original/Cargo.toml' if side == 'vanilla' else 'lab/surface-inventory/Cargo.toml'
    assert (ROOT / args.baseline).exists() and (ROOT / args.manifest).exists()
    before = identity(side)
    target = TEMP / (side + '-target')
    cache = Path('/tmp/p1322-inventory.jkHC1q') / (side + '-target')
    assert not target.exists(), 'exclusive target must be new'
    cache_copy = command(['cp', '-a', '--reflink=auto', str(cache), str(target)])
    assert cache_copy['exit_code'] == 0
    old_executable = cache / 'release' / package
    new_executable = target / 'release' / package
    hardlinks = [str(p.relative_to(target)) for p in target.rglob('*') if p.is_file() and not p.is_symlink() and (cache / p.relative_to(target)).exists() and p.stat().st_ino == (cache / p.relative_to(target)).stat().st_ino and p.stat().st_dev == (cache / p.relative_to(target)).stat().st_dev]
    assert not hardlinks, hardlinks[:5]
    assert old_executable.stat().st_ino != new_executable.stat().st_ino
    receipt = command(['cargo', 'build', '--release', '--locked', '--offline', '--manifest-path', manifest, '-p', package], {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '2'}, timeout=2700)
    after = identity(side)
    result = dict(**common(args), side=side, receipt=receipt, source_identity_before=before, source_identity_after=after, source_unchanged=before==after, cache_copy=cache_copy, cache_original_enumerator=pin(old_executable), cache_not_hardlinked=True, cache_use='Build acceleration only; cargo rebuilds current source with --locked before measurement.', tmp_reason='/dev/shm is mounted read-only', environment=command(['rustc', '--version', '--verbose']))
    binary = target / 'release' / package
    result['enumerator'] = pin(binary) if binary.exists() else None
    publish(f'p1335-inventory-{side}-build.json', result)
    assert receipt['exit_code'] == 0 and before == after, 'build failed or source changed; receipt retained'
    print(side, 'enumerator ready', result['enumerator'], flush=True)

def enumerate_side(args):
    validate_product(args)
    build_path = OUT / f'p1335-inventory-{args.side}-build.json'
    build_data = json.loads(build_path.read_text())
    assert identity(args.side) == build_data['source_identity_after']
    binary = build_data['enumerator']['path']
    assert sha(binary) == build_data['enumerator']['sha256']
    for profile in ['default', 'html']:
        argv = [binary, '/dev/stdout']
        if args.side == 'vanilla':
            argv.extend([profile, args.binary])
        else:
            argv.extend([str(OUT / f'p1335-inventory-vanilla-structural-input-{profile}.json'), profile, 'lab/surface-inventory/extra_seeds.json', args.binary])
        receipt = command(argv, timeout=900)
        assert receipt['exit_code'] == 0, receipt
        payload = json.loads(receipt['stdout'])
        if args.side == 'vanilla':
            publish(f'p1335-inventory-vanilla-structural-input-{profile}.json', payload)
        assert payload['product_sha256'] == args.expected_sha
        # Enumerators expose finite Symbol variants as public metadata. Expand them
        # into executable language paths; do not manufacture modifier permutations.
        expanded = {}
        for route, entry in list(payload['entries'].items()):
            for modifiers, value in (entry.get('symbol') or {}).get('variants', []):
                if modifiers:
                    path = route + '.' + modifiers
                    if path not in payload['entries']:
                        expanded[path] = dict(present=True, availability='declared_variant_requires_cli_confirmation', kind='symbol', params=None, source='runtime:symbol-variants', owner_path=route, owner_kind='symbol', slot_kind='symbol_variant', access_form='symbol_modifier', structural_verified=False, symbol=None, declared_value=value)
        payload['entries'].update(expanded)
        payload['entries'] = dict(sorted(payload['entries'].items()))
        payload['p1335_provenance'] = dict(**common(args), product=pin(args.binary), build=pin(build_path), receipt=receipt, source_identity_sha256=build_data['source_identity_after']['sha256'], finite_variant_routes_added=len(expanded), source_unchanged=identity(args.side)==build_data['source_identity_after'])
        assert payload['p1335_provenance']['source_unchanged']
        publish(f'p1335-inventory-{args.side}-{profile}{args.suffix}.json', payload)
        print(args.side, profile, len(payload['entries']), flush=True)

def focal(args):
    validate_product(args)
    controls = [('sym.arrow.r.double', 'VALUE'), ('emoji.arrow.r.filled', 'VALUE'), ('sym.arrow.__p1335_nonexistent__', 'DIAGNOSTIC')]
    checks, executions = [], []
    for profile in ['default', 'html']:
        source = OUT / f'p1335-inventory-{args.side}-{profile}.json'
        data = json.loads(source.read_text())
        raw = json.loads(data['p1335_provenance']['receipt']['stdout'])['entries']
        declared = {route+'.'+mods for route,entry in raw.items() for mods,value in (entry.get('symbol') or {}).get('variants', []) if mods}
        union = set(raw) | declared
        exact = union == set(data['entries'])
        assert exact
        fake = set(data['entries']) | {'sym.arrow.__p1335_fiction__.r.double'}
        checks.append(dict(input=pin(source), raw=len(raw), declared_variants=len(declared-set(raw)), union=len(union), exact_declared_union=exact, fictional_expansion_attack_rejected=union != fake))
        for route, expected in controls:
            features = [] if profile == 'default' else ['--features','html']
            receipt = command([args.binary, 'eval', f'repr((type({route}), repr({route})))', '--format','json', *features], timeout=30)
            actual = 'EXECUTION_UNKNOWN' if receipt['unknown'] else 'VALUE' if receipt['exit_code'] == 0 else 'DIAGNOSTIC' if receipt['exit_code'] == 1 and receipt['stderr'].startswith('error:') else 'EXECUTION_UNKNOWN'
            executions.append(dict(side=args.side, profile=profile, route=route, expected=expected, actual=actual, passed=actual==expected, execution=receipt))
    passed = all(x['passed'] for x in executions)
    publish(f'p1335-inventory-focal-validation-{args.side}.json', dict(**common(args), structural_checks=checks, cli_controls=executions, passed=passed, product=pin(args.binary), historical_incident=pin(OUT/'p1322-inventory-incident.json'), scope='Declared finite union and CLI control projection only; no parity verdict.'))
    assert passed

def observe(args):
    validate_product(args)
    assert json.loads((OUT / f'p1335-inventory-focal-validation-{args.side}.json').read_text())['passed']
    for profile in ['default', 'html']:
        source = OUT / f'p1335-inventory-{args.side}-{profile}{args.suffix}.json'
        payload = json.loads(source.read_text())
        features = [] if profile == 'default' else ['--features', 'html']
        begin, tick = utc(), time.monotonic_ns()
        def one(item):
            route, entry = item
            expression = f'repr((type({route}), repr({route})))'
            receipt = command([args.binary, 'eval', expression, '--format', 'json', *features], timeout=30)
            status = 'EXECUTION_UNKNOWN' if receipt['unknown'] else ('VALUE' if receipt['exit_code'] == 0 else 'DIAGNOSTIC' if receipt['exit_code'] == 1 and receipt['stderr'].startswith('error:') else 'EXECUTION_UNKNOWN')
            decoded = None
            if status == 'VALUE':
                try:
                    decoded = json.loads(receipt['stdout'])
                    if not isinstance(decoded, str) or not decoded.startswith('(') or not decoded.endswith(')'):
                        raise ValueError('required public type/repr tuple absent')
                except (ValueError, TypeError) as e:
                    status, receipt['unknown'] = 'EXECUTION_UNKNOWN', str(e)
            parts = route.split('.')
            return route, dict(name=parts[-1], namespace='.'.join(parts[:-1]), ancestors=['.'.join(parts[:i]) for i in range(1, len(parts))], expression=expression, expression_sha256=hashlib.sha256(expression.encode()).hexdigest(), kind_from_structural_inventory=entry['kind'], type_and_repr=decoded, observation=status, execution=receipt)
        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
            entries = dict(pool.map(one, sorted(payload['entries'].items())))
        unknowns = [p for p, e in entries.items() if e['observation'] == 'EXECUTION_UNKNOWN']
        result = dict(**common(args), schema_version='p1335-inventory-language-observations-v1', side=args.side, profile=profile, features=features, product=pin(args.binary), structural_input=pin(source), started_utc=begin, ended_utc=utc(), duration_ns=time.monotonic_ns()-tick, max_concurrency=8, processes=len(entries), unknowns=unknowns, entries=entries)
        validate_product(args)
        publish(f'p1335-inventory-observables-{args.side}-{profile}{args.suffix}.json', result)
        print(args.side, profile, 'observed', len(entries), 'unknown', len(unknowns), flush=True)

def catalog(args):
    history_path = OUT / 'p1322-probe-catalog.json'
    assert sha(history_path) == 'a9853e4c54663a258a9dd706c1c974930579c65e335b283243b10eae19b4889a'
    historical = json.loads(history_path.read_text())
    old = historical['probes']
    assert len({p['id'] for p in old}) == len(old)
    records, inputs, unknowns = {}, [], []
    for side in ['vanilla', 'crystalline']:
        for profile in ['default', 'html']:
            suffix = ''
            path = OUT / f'p1335-inventory-{side}-{profile}{suffix}.json'
            obs_path = OUT / f'p1335-inventory-observables-{side}-{profile}{suffix}.json'
            inv, obs = json.loads(path.read_text()), json.loads(obs_path.read_text())
            assert obs['structural_input'] == pin(path)
            assert set(inv['entries']) == set(obs['entries'])
            inputs.append(dict(**pin(path), side=side, profile=profile, entries=len(inv['entries']), language_observations=pin(obs_path)))
            for route, entry in inv['entries'].items():
                observed = obs['entries'][route]
                records.setdefault(route, []).append(dict(side=side, profile=profile, kind=entry['kind'], owner_path=entry['owner_path'], owner_kind=entry['owner_kind'], access_form=entry['access_form'], present=observed['observation']=='VALUE', observation=observed['observation']))
                if entry['availability'] == 'unknown' or 'truncated' in route or observed['observation'] == 'EXECUTION_UNKNOWN':
                    unknowns.append(dict(path=route, side=side, profile=profile, reason='enumeration_or_cli_unknown'))
    old_paths = {p['path'] for p in old}
    controls = {'csv.encode', 'xml.encode', 'read.encode'}
    new = [dict(id='p1335-path-'+path, path=path, expression=f'repr((type({path}), repr({path})))', profiles=list(PROFILES), origins=['p1335:fresh-structural-union' if path in records else 'p1335:negative-control'], roles=['fresh_surface_route' if path in records else 'explicit_negative_control']) for path in sorted((set(records)|controls)-old_paths)]
    probes = old + new
    assert len({p['id'] for p in probes}) == len(probes)
    result = dict(**common(args), schema_version='p1335-probe-catalog-v1', created_at=utc(), profiles=PROFILES, counts=dict(probes=len(probes), historical_probes=len(old), added_probes=len(new), fresh_union_paths=len(records)), historical_input=pin(history_path), inventory_inputs=inputs, preservation='Every P1322 probe retained verbatim in historical order; successor IDs only for newly discovered paths. Catalog identity does not claim behavioral closure.', canonical_execution_order='Sort by probe id, then profiles default/html/a11y/html+a11y; reverse the complete keyed job list.', probes=probes, unknowns=unknowns)
    catalog_pin = publish('p1335-probe-catalog.json', result)
    ancestors = []
    for route in sorted({p['path'] for p in probes}):
        parts = route.split('.')
        chain = ['.'.join(parts[:i]) for i in range(1, len(parts))]
        for side in ['vanilla', 'crystalline']:
            for profile in ['default', 'html']:
                missing = [a for a in chain if not any(e['side']==side and e['profile']==profile and e['present'] for e in records.get(a, []))]
                if missing:
                    ancestors.append(dict(path=route, side=side, profile=profile, ancestors=chain, absent_or_unmeasured_ancestors=missing))
    reconciliation = dict(**common(args), schema_version='p1335-inventory-reconciliation-v1', created_at=utc(), catalog=catalog_pin, historical_catalog=pin(history_path), added=[dict(probe_id=p['id'],path=p['path'],reason='newly enumerated public route; catalog discovery does not establish temporal regression' if p['path'] in records else 'explicit negative control') for p in new], removed=[], renamed=[], split=[], merged=[], unchanged=[dict(probe_id=p['id'],path=p['path'],verbatim=True,currently_enumerated=p['path'] in records) for p in old], route_records=records, blocked_or_unmeasured_ancestors=ancestors, unknowns=unknowns, limits=['Finite named routes and declared Symbol modifier paths, not the infinite space of expressions or modifier permutations.', 'Structural scopes stop at depth 8; truncation becomes Unknown.', 'Crystalline runtime enum traverses module/function namespaces; vanilla hints and explicit seeds extend candidate lookup.', 'Kind/repr observations confirm route access only, not full function semantics, aliases, argument validation, rendering or encoder behavior.', 'Features inferred only from observed default/html presence; a11y feature distinctions belong to the bilateral operator.', 'Catalog identity is not a semantic rename/split/merge decision. All historical IDs retained, including inaccessible ancestors.'], negative_controls=sorted(controls), selection_access='none')
    publish('p1335-inventory-reconciliation.json', reconciliation)
    publish('p1335-inventory-author-receipt.json', dict(**common(args), catalog=catalog_pin, reconciliation=pin(OUT/'p1335-inventory-reconciliation.json'), counts=result['counts'], unknowns=unknowns, capabilities=CAPABILITIES, readable_inputs=[pin(p) for p in READ_INPUTS], scope='Inventário e preservação de IDs; sem classificação, seleção ou veredito independente.'))
    print(json.dumps({'catalog':catalog_pin,'counts':result['counts'],'unknowns':len(unknowns)}), flush=True)

def main():
    p = argparse.ArgumentParser()
    p.add_argument('mode', choices=['build', 'enumerate', 'focal', 'observe', 'catalog'])
    p.add_argument('--side', choices=['vanilla', 'crystalline'])
    p.add_argument('--binary')
    p.add_argument('--expected-sha')
    p.add_argument('--suffix', default='')
    p.add_argument('--baseline', default='00_nucleo/diagnosticos/p1335-baseline.json')
    p.add_argument('--manifest', default='00_nucleo/diagnosticos/p1335-manifest.json')
    args = p.parse_args()
    {'build': build, 'enumerate': enumerate_side, 'focal': focal, 'observe': observe, 'catalog': catalog}[args.mode](args)

if __name__ == '__main__':
    main()
