"""No Cargo: measure public Eq/Debug/hash via a fresh real-root Cargo artifact."""
from pathlib import Path
import argparse,hashlib,json,subprocess,datetime
A=argparse.ArgumentParser();A.add_argument('--build-receipt',type=Path,required=True);A.add_argument('--rlib',type=Path,required=True);A.add_argument('--out',type=Path,required=True);a=A.parse_args()
R=Path.cwd().resolve();P=Path(__file__).resolve().parent;src=P/'public_hash_probe.rs';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
b=json.loads(a.build_receipt.read_text());assert b['exit']==0 and b['changed_inputs']==[];assert b['inputs'];assert all(sha(R/p)==d for p,d in b['inputs'].items())
rlib=a.rlib.resolve();deps=(R/'lab/target/product/debug/deps').resolve();assert rlib.parent==deps and rlib.name.startswith('libtypst_core-') and rlib.suffix=='.rlib'
info=rlib.with_name(rlib.stem.removeprefix('lib')+'.d');assert info.is_file();text=info.read_text();assert '01_core/src/lib.rs' in text;assert 'structural-' not in text
# An artifact built from a copied workspace must not stand in for the real root.
for token in text.replace('\\\n',' ').split():
 if token.endswith('01_core/src/lib.rs'):
  assert Path(token).resolve()==R/'01_core/src/lib.rs',token
hashowners=['01_core/src/entities/content/identity.rs','01_core/src/entities/content/rust_hash.rs','01_core/src/entities/content_hash.rs','00_nucleo/prompts/core/entities/content/identity.md','00_nucleo/prompts/core/entities/content/rust_hash.md','00_nucleo/prompts/core/entities/content_hash.md']
old={}
for p in hashowners:
 baseline=subprocess.check_output(['git','show','HEAD:'+p]);old[p]={'current_sha256':sha(R/p),'baseline_sha256':hashlib.sha256(baseline).hexdigest()};assert old[p]['current_sha256']==old[p]['baseline_sha256']
assert not a.out.exists();a.out.mkdir(parents=True)
r={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'build_receipt':str(a.build_receipt.resolve()),'build_receipt_sha256':sha(a.build_receipt),'source_sha256':sha(src),'runner_sha256':sha(__file__),'inputs':b['inputs'],'artifact':{'rlib':str(rlib),'rlib_sha256':sha(rlib),'dep_info':str(info),'dep_info_sha256':sha(info),'real_root':str(R)},'hash_owners_unchanged_against_head':old,'rustc_version':subprocess.check_output(['rustc','-Vv'],text=True),'note':'Measurement only; compiler/runtime failures are not semantic RED. Any Eq=true unequal hash remains a contract violation, no invented exemption. Depth1 leaf Source is canonical; deeper repeated raw carriers are public representation controls, not a frontend reachability claim.'}
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
(a.out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');print('compile',r['compile']['exit']);print('run',r.get('run',{}).get('exit'));print(json.dumps(r.get('summary',{}),indent=2));print('receipt',sha(a.out/'receipt.json'))
