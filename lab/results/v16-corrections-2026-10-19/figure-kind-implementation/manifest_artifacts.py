"""Bind final evidence and prepare an exact, NUL-separated local staging list."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess

ROOT=Path.cwd();P=Path(__file__).resolve().parent
assert (P/'closure-ledger.json').is_file() and (P/'README.md').is_file()
assert (P/'diagnostic-history/provenance.json').is_file()
scope=json.loads((P/'scope.json').read_text())
expected=set(scope['sources'])|set(scope['owners'])|{'00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md'}
actual=set(subprocess.check_output(['git','diff','HEAD','--name-only'],text=True).splitlines())|set(subprocess.check_output(['git','ls-files','--others','--exclude-standard'],text=True).splitlines())
assert actual==expected, {'outside':sorted(actual-expected),'missing':sorted(expected-actual)}
manifest=P/'artifact-manifest.json'
assert not manifest.exists()
files=sorted(f for f in P.rglob('*') if f.is_file() and '__pycache__' not in f.parts and f.suffix!='.pyc' and f!=manifest)
entries=[{'path':str(f.relative_to(ROOT)),'bytes':f.stat().st_size,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()} for f in files]
manifest.write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope_count':len(scope['pairs']),'productive_and_current_diagnostic_files':sorted(expected),'artifact_count':len(entries),'excluded_self':str(manifest.relative_to(ROOT)),'exclusions':['__pycache__','*.pyc'],'artifacts':entries},indent=2)+'\n')
stage=sorted(expected|{str(f.relative_to(ROOT)) for f in files}|{str(manifest.relative_to(ROOT))})
Path('/tmp/tekt-figure-final-paths.nul').write_bytes(b'\0'.join(s.encode() for s in stage)+b'\0')
Path('/tmp/tekt-figure-final-commit.txt').write_text('Preserve typed Figure identity and realize reference supplements\n\nRetain Figure/Grid nominal kinds, lexical occurrence fields and semantic child order. Prepare actual caption/reference supplements in the approved contextual session, preserving aliases, counters, source diagnostics and hidden-child pruning. Restore known math callback styles and require contextual Location for modern counter.final reads. Keep current English L0 specifications separate from construction history.\n\nValidated twelve integrated gates, eight real enum-extension witnesses, 84 frozen policies and 40 required reference cases. Pinned differential lint falls from 20 to 18 V16 errors; preserved parity gaps remain recorded. No suppressions or weaker contracts.\n')
print('Exact staging list:',len(stage),'files; artifacts:',len(entries),'bytes:',sum(e['bytes'] for e in entries))
