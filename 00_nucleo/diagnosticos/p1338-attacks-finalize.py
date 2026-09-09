"""Validate completed P1338 instrumental receipts and publish an immutable aggregate."""
import json, re, runpy, subprocess
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline'); D=ROOT/'00_nucleo/diagnosticos'
x=runpy.run_path(str(D/'p1338-attacks-runner.py')); sha=x['sha']; publish=x['publish']
prepared=json.loads((D/'p1338-attacks-prepared.json').read_text())
aggregate=json.loads((D/'p1338-attacks-results.json').read_text())
assert aggregate['complete'], 'Incomplete runs cannot certify complete discrimination'
families={'M1':'wrong Array absent message','M2':'wrong Array AST span','M3':'pure Array first replaced by last','M4':'Length scope extrapolation'}
expected={'M1':'p1338_pure_missing_array_fields_message_and_received_span','M2':'p1338_ast_empty_heterogeneous_alias_unicode_multiline','M3':'p1338_pure_preserve_len_first_last_and_empty_none','M4':'p1338_preserve_other_categories_diagnostics'}
records=[]
for ident in ['C',*families]:
 path=D/f'p1338-attacks-run-{ident}.json'; r=json.loads(path.read_text())
 out=Path(r['stdout_path']).read_text(); err=Path(r['stderr_path']).read_text()
 checks={'source_hash':sha(Path(r['path']).read_bytes())==r['source_sha256'],'stdout_hash':sha(out.encode())==r['stdout_sha256'],'stderr_hash':sha(err.encode())==r['stderr_sha256'],'binary_hash':sha(Path(r['preserved_executable']).read_bytes())==r['executable_sha256'],'compiled_workspace':r['compiled_typst_core'],'executed_six_tests':r['executed_tests']==6,'fresh_timestamp':r['mtime_fresh_ns']>r['mtime_before_ns'],'tests_match_prepared':r['tests_sha256']==prepared['tests_sha256'],'binary_copy_distinct_inode':Path(r['preserved_executable']).stat().st_ino!=Path(r['original_executable']).stat().st_ino}
 if ident=='C': checks['control_passed']=r['exit']==0 and 'test result: ok. 6 passed; 0 failed;' in out
 else:
  checks['semantic_test_failure']=r['exit']==101 and 'assertion `left == right` failed' in err
  checks['family_witness_failed']=any(expected[ident] in s and s.endswith('... FAILED') for s in out.splitlines())
  checks['different_from_control_binary']=r['executable_sha256']!=aggregate['runs'][0]['executable_sha256']
 assert all(checks.values()),(ident,checks)
 records.append({'id':ident,'family':families.get(ident,'paired C control'),'receipt_path':str(path),'receipt_sha256':sha(path.read_bytes()),'checks':checks,'failed_tests':[s for s in out.splitlines() if s.endswith('... FAILED')],'assertion_excerpts':re.findall(r'thread [\s\S]*?(?=\n\n|\Z)',err),'source_sha256':r['source_sha256'],'binary_path':r['preserved_executable'],'binary_sha256':r['executable_sha256'],'seconds':r['seconds']})
names=['p1338-manifest-r1.json','p1338-candidate.json','p1338-attacks-plan.md','p1338-attacks-freeze.json','p1338-attacks-freeze-tests.json','p1338-attacks-tests-link.md','p1338-tests-freeze.json','p1338-tests-module.rs.txt','p1338-attacks-oracles.json','p1338-attacks-prepared.json','p1338-attacks-results.json','p1338-attacks-runner.py']
integrity={}
for frozen in ['p1338-attacks-freeze.json','p1338-attacks-freeze-tests.json']:
 f=json.loads((D/frozen).read_text()); integrity[frozen]=all(sha((D/n).read_bytes())==digest for n,digest in f['inputs'].items())
assert all(integrity.values())
assert sha((ROOT/'01_core/src/compiler/eval/bindings/field_access.rs').read_bytes())==prepared['candidate_sha256']
versions=[]
for command in [['cargo','--version'],['rustc','-Vv']]:
 p=subprocess.run(command,capture_output=True,text=True); versions.append({'argv':command,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
r={'at':x['utc'](),'manifest_sha256':prepared['manifest_sha256'],'status':'four compiled semantic mutant failures; pending independent reviewer verdict','regime':'A/B without technical isolation attestation/refinement seal; previous adversary context retained','scope':'Array absent lookup diagnostics and specified preservation boundaries only; no global parity','profile':'release --config profile.release.package.typst-core.opt-level=0, dependencies release, jobs2, paired C; normal release gates separate','inputs':{n:sha((D/n).read_bytes()) for n in names},'input_integrity':integrity,'main_candidate_unchanged':True,'control_passed':True,'valid_mutant_families':4,'semantic_rejections_observed':4,'mutation_score_observed':1.0,'unknowns':0,'rounds':records,'cargo_processes':5,'cargo_seconds':sum(a['seconds'] for a in records),'versions':versions,'provenance':aggregate['runs'][0]['provenance'],'notes':['Plan, runner, oracles and tests frozen before C; no post-C family or oracle revision.','Each actual executable copied before next link; per-round preserved executable, not mutable target/deps path, is canonical.','Pure first witness is direct helper preservation, independent of AST pre-dispatch; Length preservation remains debt, not parity credit.']}
publish(D/'p1338-attacks-final.json',r)
print(json.dumps({'path':str(D/'p1338-attacks-final.json'),'sha256':sha((D/'p1338-attacks-final.json').read_bytes()),'mutants':4,'unknowns':0,'cargo_seconds':r['cargo_seconds']}))
