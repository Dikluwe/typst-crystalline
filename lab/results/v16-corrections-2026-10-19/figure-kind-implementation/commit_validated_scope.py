"""Commit only the exact manifested, independently validated local scope."""
from pathlib import Path
import hashlib
import importlib.util
import json
import re
import subprocess

ROOT=Path.cwd();P=Path(__file__).resolve().parent
git=lambda *args:subprocess.check_output(['git',*args],text=True).strip()
sha=lambda path:hashlib.sha256(path.read_bytes()).hexdigest()
ledger=json.loads((P/'closure-ledger.json').read_text())
manifest=json.loads((P/'artifact-manifest.json').read_text())
assert git('branch','--show-current')==ledger['branch']=='codex/tekt-mature-layout'
assert git('rev-parse','HEAD')==ledger['baseline_head']
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
build=json.loads((P/'candidate-build-4/receipt.json').read_text())
assert build['inputs']==v.frozen_inputs()
for pair in ledger['source_owner_pairs']:
    assert sha(ROOT/pair['source'])==pair['source_sha256']
    assert sha(ROOT/pair['owner'])==pair['owner_sha256']
for artifact in manifest['artifacts']:
    assert sha(ROOT/artifact['path'])==artifact['sha256'],artifact['path']
expected={p.decode() for p in Path('/tmp/tekt-figure-final-paths.nul').read_bytes().split(b'\0') if p}
staged=set(git('diff','--cached','--name-only').splitlines())
assert not staged or staged==expected,sorted(staged-expected)
subprocess.run(['git','add','-f','--pathspec-from-file=/tmp/tekt-figure-final-paths.nul','--pathspec-file-nul'],check=True)
assert set(git('diff','--cached','--name-only').splitlines())==expected
whitespace=subprocess.run(['git','diff','--cached','--check'],capture_output=True,text=True)
Path('/tmp/tekt-figure-staged-whitespace.txt').write_text(whitespace.stdout+whitespace.stderr)
if whitespace.returncode:
    # Raw failed-run stdout is historical evidence; changing its spaces would
    # invalidate its byte identity. Preserve and identify every such warning.
    warnings=re.findall(r'^(.+?):\d+: (.+)$',whitespace.stdout,re.M)
    assert warnings and not whitespace.stderr
    raw={a['path'] for a in manifest['artifacts'] if a['path'].endswith('.log')}
    assert all(path in raw and message in ('trailing whitespace.','new blank line at EOF.') for path,message in warnings),warnings
subprocess.run(['git','diff','--cached','--check','--',*manifest['productive_and_current_diagnostic_files']],check=True)
subprocess.run(['git','commit','-F','/tmp/tekt-figure-final-commit.txt'],check=True)
commit=git('rev-parse','HEAD')
assert git('rev-parse','HEAD^')==ledger['baseline_head']
assert not git('status','--porcelain')
assert build['inputs']==v.frozen_inputs()
for pair in ledger['source_owner_pairs']:
    assert sha(ROOT/pair['source'])==pair['source_sha256']
    assert sha(ROOT/pair['owner'])==pair['owner_sha256']
result={'commit':commit,'parent':ledger['baseline_head'],'branch':ledger['branch'],'files':len(expected),'clean':True,'validated_frozen_inputs_unchanged':True,'binary_rebuilt_after_commit':False,'artifact_manifest_sha256':sha(P/'artifact-manifest.json')}
Path('/tmp/tekt-figure-postcommit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
