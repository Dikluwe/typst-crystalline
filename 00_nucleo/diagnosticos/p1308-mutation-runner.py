#!/usr/bin/env python3
"""Execute one already-applied P1308 source mutant in an exact disposable copy.

Consumes SPEC JSON; emits a full raw receipt on stdout, never edits sources.
SPEC: mode control|mutant, family P1308-M01..06, candidate_root, target_dir,
freeze {path,sha256} with source_inventory, lease {path,sha256}, tests [suffix],
changes [{path,before_sha256,after_sha256,function,defect}], patch {path,sha256},
green_control {path,sha256}. The latter three are required only for mutants.
Lease: granted_by /root, exclusive and no_other_builds true, candidate_root,
target_dir, expires_at UTC ISO. Root must grant the build window explicitly.
No source failure is called a killed mutant until separate causal review.
"""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
HELPER = HERE / 'p1307-r6-mutation-runner.py'
assert hashlib.sha256(HELPER.read_bytes()).hexdigest() == 'bd91767023faa8194b24d11dd929eaf13d2ad2eed74174d2ff38b64465ccf60d'
helper = runpy.run_path(str(HELPER))
sha, read, pinned, now, inventory, execute = [helper[k] for k in ('sha', 'read', 'pinned', 'now', 'inventory', 'execute')]
PLAN = HERE / 'p1308-mutation-plan.json'
MANIFEST = HERE / 'p1308-manifest.json'


def run(spec):
    rec = dict(schema='p1308-real-source-mutation-v1', started=now(),
               regime='executado sem atestacao de isolamento tecnico',
               spec=spec, commands=[], status='Unknown', source_mutant_killed=False,
               runner_sha256=sha(__file__), helper_sha256=sha(HELPER),
               manifest_sha256=sha(MANIFEST), plan_sha256=sha(PLAN))
    try:
        assert sha(MANIFEST) == 'ee8df0e15cf50ed0903472c219dd2aefbeb787a11ebb18f12d2b15f8dd754ca1'
        manifest = read(MANIFEST)
        for ref in manifest['inputs']:
            assert sha(ROOT / ref['path']) == ref['sha256'], ref['path']
        family = next(f for f in read(PLAN)['families'] if f['id'] == spec['family'])
        root, target = Path(spec['candidate_root']).resolve(), Path(spec['target_dir']).resolve()
        assert root != ROOT and not root.is_relative_to(ROOT) and not ROOT.is_relative_to(root)
        assert root.name not in ('tmp', 'shm') and root.is_absolute()
        freeze = pinned(spec['freeze'])
        expected = freeze['source_inventory']
        before = inventory(root, expected)
        changes = {p for p in before if before[p] != expected[p]}
        assert spec['mode'] in ('control', 'mutant')
        if spec['mode'] == 'control':
            assert not changes and not spec.get('changes')
        else:
            assert changes == {family['owner']} == {c['path'] for c in spec['changes']}
            for c in spec['changes']:
                assert c['before_sha256'] == expected[c['path']]
                assert c['after_sha256'] == before[c['path']]
                assert c['function'] and c['defect']
            assert sha(spec['patch']['path']) == spec['patch']['sha256']
            rec['concrete_patch'] = Path(spec['patch']['path']).read_text()
            control = pinned(spec['green_control'])
            assert control['status'] == 'GREEN_CONTROL'
            assert control['spec']['freeze'] == spec['freeze']
        lease = pinned(spec['lease'])
        assert lease['granted_by'] == '/root' and lease['exclusive'] and lease['no_other_builds']
        assert lease['candidate_root'] == str(root) and lease['target_dir'] == str(target)
        remaining = (dt.datetime.fromisoformat(lease['expires_at']) - dt.datetime.now(dt.timezone.utc)).total_seconds()
        assert remaining > 0
        env = {k:v for k,v in os.environ.items() if not k.startswith(('TYPST_', 'CRYSTALLINE_'))}
        settings = {f'CARGO_PROFILE_{p}_{k}': v for p in ('DEV', 'TEST')
                    for k,v in {'DEBUG':'0','INCREMENTAL':'true','LTO':'false','CODEGEN_UNITS':'256'}.items()}
        env.update(settings, CARGO_TARGET_DIR=str(target), NO_COLOR='1', TERM='dumb')
        rec['environment'] = {k:env.get(k) for k in ('PATH','RUSTFLAGS','RUSTUP_TOOLCHAIN','CARGO_BUILD_JOBS','CARGO_TARGET_DIR',*settings)}
        if spec['mode'] == 'mutant':
            assert control['environment'] == rec['environment'], 'Same-profile control required'
            assert control['spec'].get('route', 'rust') == spec.get('route', 'rust')
        if spec.get('route', 'rust') == 'cli':
            public = spec['public_cases']
            assert public and len(public) == len(set(public))
            assert set(public) <= set(family['public_witnesses'] + family['boundary_controls'])
            rec.update(source_before=before, lease=lease)
            with open('/tmp/p1308-mutation-build.lock', 'a') as lock:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
                build = execute(['cargo','build','-p','typst-wiring','--bin','typst','--message-format=json'],root,env,min(remaining,900))
                rec['commands'].append(build)
                assert not build['timed_out'] and build['returncode'] == 0, 'Compilation failure is not a kill'
                artifacts = []
                for line in build['stdout'].splitlines():
                    try:
                        item = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    if item.get('reason') == 'compiler-artifact' and item.get('executable') and item['target']['name'] == 'typst':
                        artifacts.append(item['executable'])
                assert len(set(artifacts)) == 1
                binary = Path(artifacts[0]).resolve()
                assert binary.is_relative_to(target)
                rec.update(binary=str(binary),binary_sha256=sha(binary))
                pattern = '^(?:'+'|'.join(re.escape(x) for x in public)+')$'
                row = execute([sys.executable,str(HERE/'p1308-contract-replay.py'),'--binary',str(binary),'--case',pattern,'--profile','all'],root,env,300)
                rec['commands'].append(row)
                assert row['returncode'] == 0 and not row['timed_out']
                observed = json.loads(row['stdout'])
                assert observed['binary_sha256'] == rec['binary_sha256']
                assert len(observed['rows']) == len(public)*4
                assert {(r['case'],r['profile']) for r in observed['rows']} == {(c,p) for c in public for p in ('default','html','a11y','combined')}
                rec['public_observations'] = observed
                witnesses = {r['case']+'/'+r['profile']:r['verdict'] for r in observed['rows']}
                assert set(witnesses.values()) <= {'Preserved','Violated'}
            rec['witnesses'] = witnesses
            if spec['mode'] == 'control':
                rec['status'] = 'GREEN_CONTROL' if set(witnesses.values()) == {'Preserved'} else 'CONTROL_NOT_GREEN'
            else:
                assert all(control['witnesses'].get(k) == 'Preserved' for k in witnesses)
                rec['status'] = 'COMPILED_WITNESS_FAILURE_PENDING_CAUSAL_REVIEW' if 'Violated' in witnesses.values() else 'SURVIVED_FOCAL'
            rec['source_after'] = inventory(root,expected)
            assert rec['source_after'] == before
            rec['finished'] = now()
            return rec
        assert spec.get('route', 'rust') == 'rust'
        tests = spec['tests']
        allowed = set(re.findall(r'fn (p1308_\w+)\(', ''.join((HERE / p).read_text() for p in ('p1308-tests.patch','p1308-tests-supplement.patch'))))
        assert tests and len(tests) == len(set(tests)) and set(tests) <= allowed
        rec.update(source_before=before, lease=lease)
        with open('/tmp/p1308-mutation-build.lock', 'a') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            build = execute(['cargo','test','-p','typst-core','--lib','--no-run','--message-format=json'], root, env, min(remaining, 900))
            rec['commands'].append(build)
            assert not build['timed_out'] and build['returncode'] == 0, 'Compilation failure is not a kill'
            artifacts = []
            for line in build['stdout'].splitlines():
                try:
                    item = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if item.get('reason') == 'compiler-artifact' and item.get('executable') and item['target']['name'] == 'typst_core' and item['profile']['test']:
                    artifacts.append(item['executable'])
            assert len(set(artifacts)) == 1
            binary = Path(artifacts[0]).resolve()
            assert binary.is_relative_to(target)
            rec.update(binary=str(binary), binary_sha256=sha(binary))
            listing = execute([str(binary),'--list'],root,env,30)
            rec['commands'].append(listing)
            assert listing['returncode'] == 0
            names = [s.removesuffix(': test') for s in listing['stdout'].splitlines() if s.endswith(': test')]
            witnesses = {}
            for suffix in tests:
                names_for_test = [n for n in names if n.endswith('::'+suffix) and 'p1308_independent' in n]
                assert len(names_for_test) == 1
                row = execute([str(binary),names_for_test[0],'--exact','--test-threads=1','--nocapture'],root,env,60)
                rec['commands'].append(row)
                assert not row['timed_out'] and row['returncode'] in (0,101)
                assert re.search(r'test result: (?:ok|FAILED)\. (?:1 passed; 0 failed|0 passed; 1 failed);',row['stdout'])
                witnesses[suffix] = 'Preserved' if row['returncode'] == 0 else 'Violated'
        rec['witnesses'] = witnesses
        if spec['mode'] == 'control':
            rec['status'] = 'GREEN_CONTROL' if set(witnesses.values()) == {'Preserved'} else 'CONTROL_NOT_GREEN'
        else:
            assert all(control['witnesses'].get(k) == 'Preserved' for k in witnesses)
            rec['status'] = 'COMPILED_WITNESS_FAILURE_PENDING_CAUSAL_REVIEW' if 'Violated' in witnesses.values() else 'SURVIVED_FOCAL'
        rec['source_after'] = inventory(root,expected)
        assert rec['source_after'] == before
    except (AssertionError, KeyError, ValueError, OSError, StopIteration) as exc:
        rec.update(status='Unknown',reason=type(exc).__name__+': '+str(exc))
    rec['finished'] = now()
    return rec


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('spec')
    args = parser.parse_args()
    print(json.dumps(run(read(args.spec)),ensure_ascii=False,indent=2))
