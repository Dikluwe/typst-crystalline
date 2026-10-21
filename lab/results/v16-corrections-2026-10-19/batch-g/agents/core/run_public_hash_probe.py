"""No Cargo: measure public Eq/Debug/hash via a fresh real-root Cargo artifact."""
from pathlib import Path
import argparse,hashlib,json,subprocess,datetime
A=argparse.ArgumentParser();A.add_argument('--phase',choices=['before','candidate'],default='before');A.add_argument('--scope',type=Path);A.add_argument('--build-receipt',type=Path,required=True);A.add_argument('--rlib',type=Path,required=True);A.add_argument('--out',type=Path,required=True);a=A.parse_args()
R=Path.cwd().resolve();P=Path(__file__).resolve().parent;src=P/'public_hash_probe.rs';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
b=json.loads(a.build_receipt.read_text());assert b['exit']==0 and b['changed_inputs']==[];assert b['inputs'];assert all(sha(R/p)==d for p,d in b['inputs'].items())
rlib=a.rlib.resolve();deps=(R/'lab/target/product/debug/deps').resolve();assert rlib.parent==deps and rlib.name.startswith('libtypst_core-') and rlib.suffix=='.rlib'
info=rlib.with_name(rlib.stem.removeprefix('lib')+'.d');assert info.is_file();text=info.read_text();assert '01_core/src/lib.rs' in text;assert 'structural-' not in text
# An artifact built from a copied workspace must not stand in for the real root.
for token in text.replace('\\\n',' ').split():
 if token.endswith('01_core/src/lib.rs'):
  assert Path(token).resolve()==R/'01_core/src/lib.rs',token
hashowners=['01_core/src/entities/content/identity.rs','01_core/src/entities/content/rust_hash.rs','01_core/src/entities/content_hash.rs','00_nucleo/prompts/core/entities/content/identity.md','00_nucleo/prompts/core/entities/content/rust_hash.md','00_nucleo/prompts/core/entities/content_hash.md']
identity_pair={'01_core/src/entities/content/identity.rs','00_nucleo/prompts/core/entities/content/identity.md'}
phase_constraints=None
if a.phase=='candidate':
 scope_path=(a.scope or P.parent.parent/'scope.json').resolve();scope=json.loads(scope_path.read_text())
 assert len(scope['pairs'])==13 and scope['baseline_head']==subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
 declared={v for pair in scope['pairs'] for v in [pair['source'],pair['owner']]}
 assert len(declared)==26 and identity_pair<=declared
 assert all(p in b['inputs'] and sha(R/p)==b['inputs'][p] for p in declared)
 changed=set(subprocess.check_output(['git','diff','--name-only','HEAD'],text=True).splitlines());assert changed<=declared,sorted(changed-declared)
 assert scope['depth_scope_extension']['evidence_sha256']=='786b466c42e89682d021248a005ecda102a93011280e4f3851d9d897947a3664'
 before_path=P.parent.parent/'hash-before/receipt.json';before=json.loads(before_path.read_text())
 assert sha(before_path)==scope['depth_scope_extension']['evidence_sha256']
 assert sha(src)==before['source_sha256']=='8255fbf6dcda63bdc220d227a665e497f7560ccb8741bb5f2e32035db5d39cde'
 archived=P/'hash-probe-runner-before-candidate/run_public_hash_probe.py'
 assert sha(archived)==before['runner_sha256']=='9753520931a31df98f8350447871821c637559612fcb9bdd06bbe463032c5018'
 phase_constraints={'scope':str(scope_path),'scope_sha256':sha(scope_path),'declared_pairs':13,'changed_tracked_paths':sorted(changed),'allowed_changed_hash_owners':sorted(identity_pair),'baseline_runner':str(archived),'baseline_runner_sha256':sha(archived),'before_receipt':str(before_path),'before_receipt_sha256':sha(before_path)}
old={}
for p in hashowners:
 baseline=subprocess.check_output(['git','show','HEAD:'+p]);old[p]={'current_sha256':sha(R/p),'baseline_sha256':hashlib.sha256(baseline).hexdigest()}
 if a.phase=='before' or p not in identity_pair: assert old[p]['current_sha256']==old[p]['baseline_sha256'],p
 if a.phase=='candidate': assert p in b['inputs'] and old[p]['current_sha256']==b['inputs'][p],p
assert not a.out.exists();a.out.mkdir(parents=True)
r={'utc_start':utc(),'phase':a.phase,'phase_constraints':phase_constraints,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'build_receipt':str(a.build_receipt.resolve()),'build_receipt_sha256':sha(a.build_receipt),'source_sha256':sha(src),'runner_sha256':sha(__file__),'inputs':b['inputs'],'artifact':{'rlib':str(rlib),'rlib_sha256':sha(rlib),'dep_info':str(info),'dep_info_sha256':sha(info),'real_root':str(R)},'hash_owners_against_head':old,'rustc_version':subprocess.check_output(['rustc','-Vv'],text=True),'note':'Measurement only; compiler/runtime failures are not semantic RED. Any Eq=true unequal hash remains a contract violation, no invented exemption. Depth1 leaf Source is canonical; deeper repeated raw carriers are public representation controls, not a frontend reachability claim.'}
argv=['rustc','--edition=2021',str(src),'-L','dependency='+str(deps),'--extern','typst_core='+str(rlib),'-o',str(a.out/'probe')];run=subprocess.run(argv,capture_output=True,text=True);r['compile']={'argv':argv,'exit':run.returncode,'stdout':run.stdout,'stderr':run.stderr}
if run.returncode==0:
 argv=[str((a.out/'probe').resolve())];run=subprocess.run(argv,capture_output=True,text=True,timeout=30);r['run']={'argv':argv,'exit':run.returncode,'stdout':run.stdout,'stderr':run.stderr};r['probe_sha256']=sha(a.out/'probe')
 if run.returncode==0:
  rows=[json.loads(s) for s in run.stdout.splitlines()];r['rows']=rows;assert len(rows)==366
  # Fresh library smoke: old always-false Title would fail this explicit control.
  r['fresh_corrected_eq_smoke']=all(x['equal'] for x in rows if x['family'] in ['Title','Frame','Html','Context'] and x['depth']==0);assert r['fresh_corrected_eq_smoke']
  r['violations']={'public_hash':[x for x in rows if x['equal'] and not x['public_hash_equal']],'std_hash':[x for x in rows if x['equal'] and not x['std_hash_equal']]}
  r['summary']={}
  for family in ['Strong','Sequence','Title','Frame','Html','Context','Space','HtmlAttrsOrder','NestedValueFloat']:
   selected=[x for x in rows if x['family']==family];r['summary'][family]={'rows':len(selected),'equal':sum(x['equal'] for x in selected),'public_hash_violations':sum(x['equal'] and not x['public_hash_equal'] for x in selected),'std_hash_violations':sum(x['equal'] and not x['std_hash_equal'] for x in selected)}
r['utc_end']=utc();r['changed_inputs']=[p for p,d in b['inputs'].items() if sha(R/p)!=d];r['artifact_unchanged']=sha(rlib)==r['artifact']['rlib_sha256'] and sha(info)==r['artifact']['dep_info_sha256'];assert not r['changed_inputs'] and r['artifact_unchanged']
r['probe_source_unchanged']=sha(src)==r['source_sha256'];r['runner_unchanged']=sha(__file__)==r['runner_sha256'];assert r['probe_source_unchanged'] and r['runner_unchanged']
if phase_constraints is not None:
 r['scope_unchanged']=sha(phase_constraints['scope'])==phase_constraints['scope_sha256'];assert r['scope_unchanged']
r['candidate_acceptance']=None if a.phase=='before' else {'finite_rows':len(r.get('rows',[])),'public_hash_violations':len(r.get('violations',{}).get('public_hash',[])),'std_hash_violations':len(r.get('violations',{}).get('std_hash',[])),'pass':r['compile']['exit']==0 and r.get('run',{}).get('exit')==0 and len(r.get('rows',[]))==366 and not any(r.get('violations',{}).values()),'scope':'Exactly the unchanged 366-row public probe; not a universal Content/Value Hash-law proof. The measured NestedValueFloat control refuted the proposed negative-zero gap.'}
(a.out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');print('compile',r['compile']['exit']);print('run',r.get('run',{}).get('exit'));print(json.dumps(r.get('summary',{}),indent=2));print('receipt',sha(a.out/'receipt.json'))

if a.phase=='candidate' and not r['candidate_acceptance']['pass']: raise SystemExit(1)
