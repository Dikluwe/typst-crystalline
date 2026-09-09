#!/usr/bin/env python3
"""Test-only succession evidence from original tests, updated L0 and vanilla."""
import datetime, hashlib, json, pathlib, re, subprocess

def run(argv):
    at = datetime.datetime.now(datetime.timezone.utc).isoformat()
    p = subprocess.run(argv, capture_output=True, text=True, timeout=30)
    return dict(at=at, argv=argv, exit=p.returncode, stdout=p.stdout, stderr=p.stderr)

def digest(data): return hashlib.sha256(data).hexdigest()

source = pathlib.Path('01_core/src/compiler/eval/tests.rs').read_bytes()
prompt = pathlib.Path('00_nucleo/prompts/compiler/eval/tests.md').read_bytes()
receipt = dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(), scope='Post-candidate test-only succession, not product RED; no candidate measurement used', manifest_sha256=digest(pathlib.Path('00_nucleo/diagnosticos/p1325-test-succession-manifest.json').read_bytes()), head=run(['git','rev-parse','HEAD']), diffstat=run(['git','diff','HEAD','--stat']), source_sha256=digest(source), source_without_prompt_hash_sha256=digest(re.sub(rb'^//! @prompt-hash [0-9a-f]{8}\n',b'',source,flags=re.M)), prompt_normalized_sha256=digest(re.sub(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n',b'',prompt,flags=re.M)), vanilla_sha256=digest(pathlib.Path('/usr/local/bin/typst').read_bytes()), rows=[])
for expression, field in [('repr(type((:).missing))','missing'), ('repr(type((:).nope))','nope')]:
    start = expression.index('.' + field) + 1
    for profile,flags in [('default',[]),('html',['--features','html']),('a11y',['--features','a11y-extras']),('html+a11y',['--features','html,a11y-extras'])]:
        measured = run(['/usr/local/bin/typst','--color','never','eval','--format','json',*flags,expression])
        receipt['rows'].append(dict(profile=profile,expression=expression,field=field,language_field_range=[start,start+len(field)],**measured))
print(json.dumps(receipt,ensure_ascii=False,indent=2))
