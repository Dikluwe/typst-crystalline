"""Aggregate immutable completed rounds; does not alter oracles, source or receipts."""
import json, re, runpy, subprocess
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline'); D=ROOT/'00_nucleo/diagnosticos'
x=runpy.run_path(str(D/'p1337-attacks-runner.py')); sha=x['sha']; publish=x['publish']
prepared=json.loads((D/'p1337-attacks-prepared.json').read_text())
aggregate=json.loads((D/'p1337-attacks-results.json').read_text())
assert aggregate['complete'], 'Incomplete runs are not a complete mutation result'
families={'M1':'wrong Bool name','M2':'wrong Bool AST span','M3':'wrong None AST span','M4':'wrong Auto AST span','M5':'Array scope extrapolation'}
expected={'M1':'p1337_pure_all_singletons_bool_values_names_and_supplied_spans','M2':'p1337_ast_variants_aliases_parentheses_unicode_and_multiline','M3':'p1337_ast_variants_aliases_parentheses_unicode_and_multiline','M4':'p1337_ast_variants_aliases_parentheses_unicode_and_multiline','M5':'p1337_preserve_array_ast_message_and_total_span_debt'}
records=[]
for ident in ['C',*families]:
 path=D/f'p1337-attacks-run-{ident}.json'; r=json.loads(path.read_text())
 out=Path(r['stdout_path']).read_text(); err=Path(r['stderr_path']).read_text()
 checks={'source_hash':sha(Path(r['path']).read_bytes())==r['source_sha256'],'stdout_hash':sha(out.encode())==r['stdout_sha256'],'stderr_hash':sha(err.encode())==r['stderr_sha256'],'binary_hash':sha(Path(r['preserved_executable']).read_bytes())==r['executable_sha256'],'compiled_workspace':r['compiled_typst_core'],'executed_four_tests':r['executed_tests']==4,'fresh_timestamp':r['mtime_fresh_ns']>r['mtime_before_ns'],'tests_match_prepared':r['tests_sha256']==prepared['tests_sha256'],'binary_copy_distinct_inode':Path(r['preserved_executable']).stat().st_ino!=Path(r['original_executable']).stat().st_ino}
 if ident=='C': checks['control_passed']=r['exit']==0 and 'test result: ok. 4 passed; 0 failed;' in out
 else:
  checks['semantic_test_failure']=r['exit']==101 and 'assertion `left == right` failed' in err
  checks['family_witness_failed']=any(expected[ident] in s and s.endswith('... FAILED') for s in out.splitlines())
  checks['different_from_control_binary']=r['executable_sha256']!=aggregate['runs'][0]['executable_sha256']
 assert all(checks.values()),(ident,checks)
 failures=[s for s in out.splitlines() if s.endswith('... FAILED')]
 excerpts=re.findall(r'thread [\s\S]*?(?=\n\n|\Z)',err)
 records.append({'id':ident,'family':families.get(ident,'paired C control'),'receipt_path':str(path),'receipt_sha256':sha(path.read_bytes()),'checks':checks,'failed_tests':failures,'assertion_excerpts':excerpts,'source_sha256':r['source_sha256'],'binary_path':r['preserved_executable'],'binary_sha256':r['executable_sha256'],'seconds':r['seconds']})
frozen=['p1337-manifest-r1.json','p1337-candidate.json','p1337-attacks-plan.md','p1337-attacks-plan-addendum-r1.md','p1337-attacks-freeze-r2.json','p1337-tests-freeze-r2.json','p1337-tests-module-r2.rs.txt','p1337-attacks-oracles.json','p1337-attacks-prepared.json','p1337-attacks-results.json','p1337-attacks-runner.py']
versions=[]
for command in [['cargo','--version'],['rustc','-Vv']]:
 p=subprocess.run(command,capture_output=True,text=True); versions.append({'argv':command,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
result={'at':x['utc'](),'manifest_sha256':prepared['manifest_sha256'],'status':'five compiled semantic mutant failures; pending independent reviewer verdict','regime':'A/B executed without technical isolation attestation; no refinement seal','scope':'Bool/None/Auto field lookup diagnostic fragment plus preservation sentinel Array; no global parity claim','profile':'release with --config profile.release.package.typst-core.opt-level=0; dependencies release; same C and mutants; normal release gates separate','inputs':{n:sha((D/n).read_bytes()) for n in frozen},'control_passed':True,'valid_mutant_families':5,'semantic_rejections_observed':5,'mutation_score_observed':1.0,'unknowns':0,'rounds':records,'cargo_processes':6,'cargo_seconds':sum(r['seconds'] for r in records),'versions':versions,'provenance':aggregate['runs'][0]['provenance'],'notes':['Pre-C observability revision added independent Array sentinel; mechanical formatting succession retained its meaning. No post-C family/test/oracle revisions.','Each round source, patch, channels and executable preserved before next link. Original target executable path may now refer to latest mutant; preserved per-round executable is canonical.','C source hash and all main-tree tests remained protected; final normative release proof belongs to operator.']}
publish(D/'p1337-attacks-final.json',result)
print(json.dumps({'path':str(D/'p1337-attacks-final.json'),'sha256':sha((D/'p1337-attacks-final.json').read_bytes()),'mutants':5,'unknowns':0,'cargo_seconds':result['cargo_seconds']}))
