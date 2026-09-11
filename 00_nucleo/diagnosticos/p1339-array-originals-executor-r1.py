"""Unchanged original70 programs; direct frozen execute, explicit successor integrity.
Preparation only until independently accepted successor seal and candidate GO.
The ancestral checked_inputs and every oracle remain untouched, not monkeypatched.
"""
import argparse, concurrent.futures, hashlib, importlib.util, json, re, subprocess, tempfile, time
from pathlib import Path
D=Path(__file__).resolve().parent;ROOT=D.parents[1]
def read(p):return json.loads(Path(p).read_text())
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def pin(p):return {'path':str(Path(p).resolve()),'sha256':sha(p)}
def module(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def save(path,value):
 assert path.parent==D and path.name.startswith('p1339-array-originals-') and not path.exists()
 b=json.dumps(value,ensure_ascii=True,indent=2)+'\n';patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+l+'\n' for l in b.split('\n')[:-1])+'*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
def main():
 p=argparse.ArgumentParser()
 for x in ['manifest','manifest-sha256','successor-seal','successor-seal-sha256','go','go-sha256','binary','binary-sha256']:p.add_argument('--'+x,required=True)
 p.add_argument('--mode',choices=['focal','expanded'],required=True);a=p.parse_args()
 assert sha(a.manifest)==a.manifest_sha256;m=read(a.manifest)
 for item in m['inputs']:assert pin(item['path'])==item,('prepared input drift',item['path'])
 assert m['executor']==pin(__file__)
 assert sha(a.successor_seal)==a.successor_seal_sha256 and a.successor_seal_sha256!=m['ancestor_seal']['sha256']
 assert sha(a.go)==a.go_sha256 and sha(a.binary)==a.binary_sha256
 # The independent verifier/root supplies these accepted immutable identities.
 # This transport records them; it does not invent a successor verdict schema.
 authority=[pin(a.manifest),pin(a.successor_seal),pin(a.go),pin(a.binary)]
 ancestor=read(m['ancestor_seal']['path']);supp=read(m['supplement']['path']);plan=read(m['plan']['path']);selection=read(m['selection']['path'])
 frozen=module('p1339_original_frozen_execute',m['frozen_executor']['path'])
 def integrity():
  started=frozen.utc();tick=time.monotonic();checks=[]
  norms={str(Path(x['path']).resolve()):x['normative_sha256'] for x in ancestor['l0']}
  for path,item in supp['l0'].items():norms[str((ROOT/path).resolve())]=item['normative_sha256']
  assert len(supp['l0'])==4 and len(norms)==32
  def exact(item):
   actual=sha(item['path']);return {'path':item['path'],'expected':item['sha256'],'actual':actual,'valid':actual==item['sha256']}
  unchanged=[x for x in ancestor['immutable_inputs'] if str(Path(x['path']).resolve()) not in norms]
  with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:checks.extend(pool.map(exact,unchanged))
  for item in [*m['inputs'],*authority]:checks.append(exact(item))
  for path,wanted in norms.items():
   data=Path(path).read_bytes();lines=data.splitlines(keepends=True);end=next((i for i,l in enumerate(lines) if l in (b'\n',b'\r\n')),len(lines))
   removed=[i for i,l in enumerate(lines[:end]) if re.fullmatch(rb'Hash do C\xc3\xb3digo: [0-9a-f]{8}\r?\n',l)]
   assert len(removed)==1,('invalid canonical L0 metadata',path)
   actual=hashlib.sha256(b''.join(l for i,l in enumerate(lines) if i not in removed)).hexdigest()
   checks.append({'path':path,'mode':'normative_only_canonical_Hash_do_Codigo_excluded','raw_sha256':sha(path),'expected':wanted,'actual':actual,'valid':actual==wanted})
  for c in plan['cases']:
   if c.get('source_path'):checks.append(exact({'path':c['source_path'],'sha256':c['source_sha256']}))
  result={'start_utc':started,'end_utc':frozen.utc(),'seconds':time.monotonic()-tick,'checks':checks,'valid':all(x['valid'] for x in checks)}
  return result
 ids=selection['ids'];assert len(ids)==len(set(ids))==70
 cases=[c for c in plan['cases'] if c['id'] in ids];assert [c['id'] for c in cases]==selection['plan_order_ids']
 byid={c['id']:c for c in cases};cells=selection['focal_cells'] if a.mode=='focal' else selection['cells'];assert len(cells)==(70 if a.mode=='focal' else 840)
 rawpath=D/('p1339-array-originals-'+a.mode+'-runs-r1.json');judgedpath=D/('p1339-array-originals-'+a.mode+'-judgment-r1.json')
 assert not rawpath.exists() and not judgedpath.exists()
 before=frozen.state();pre=integrity();assert pre['valid'],'successor protected input drift before execution'
 temp=Path(tempfile.mkdtemp(prefix='p1339-array-originals-'));tick=time.monotonic()
 tasks=[(byid[x['case_id']],a.binary,'candidate',x['profile'],x['order'],temp,i,a.binary_sha256) for i,x in enumerate(cells)]
 print('Starting unchanged original programs:',len(tasks),'candidate processes',flush=True)
 with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(lambda task:frozen.execute(*task),tasks))
 seconds=time.monotonic()-tick;post=integrity()
 raw={'schema':'p1339-array-originals-execution-v1','phase':'candidate','authority_manifest_sha256':frozen.MANIFEST_SHA,'plan_sha256':m['plan']['sha256'],'runner_sha256':sha(frozen.__file__),'binaries':{'candidate':pin(a.binary)},'before':before,'after':frozen.state(),'seal':pin(a.successor_seal),'ancestor_seal':m['ancestor_seal'],'manifest':pin(a.manifest),'GO':pin(a.go),'selection':m['selection'],'adapter':pin(__file__),'processes':len(rows),'wall_seconds':seconds,'sum_child_seconds':sum(r['seconds'] for r in rows),'rows':rows,'integrity_before':pre,'integrity_after':post,'regime':'executado sem atestacao de isolamento'}
 save(rawpath,raw)
 assert post['valid'],'successor protected input drift after execution; raw retained'
 predicate=module('p1339_original_sealed_predicate',m['predicate']['path']);positive=read(m['positive']['path']);opaque=read(m['opaque']['path']);focal=read(m['focal_reference']['path']);derived=read(m['derived']['path'])
 assert derived['raw']==m['focal_reference']
 if a.mode=='expanded':judged=predicate.check(raw,'candidate',ids,focal,derived)
 else:
  source_by_id={x['id']:x['source_sha256'] for x in selection['case_proofs']};judged=[]
  for row,cell in zip(rows,cells):
   case=byid[cell['case_id']]
   assert {k:row['id'] if k=='case_id' else row[k] for k in cell}==cell
   assert row['source_sha256']==source_by_id[row['id']] and row['binary_sha256']==a.binary_sha256
   assert row['argv']==[a.binary,'eval','--format','json',case['expression']]
   assert row['cwd']==str(ROOT) and row['mode']=='eval' and row['env']['NO_COLOR']=='1' and row['env']['TERM']=='dumb'
   x=predicate.evaluate(row,'candidate',positive,opaque,focal,derived);x.update({k:row[k] for k in ['product','profile','order']});judged.append(x)
 failures=[x for x in judged if x['classification']!='Preserved' or x.get('differences')]
 receipt={'raw':pin(rawpath),'manifest':pin(a.manifest),'predicate':m['predicate'],'source_and_oracle_changes':0,'mode':a.mode,'cells':len(rows),'classified':len(judged),'classifications':judged,'failures':failures,'Unknown':sum(r['execution']!='Observed' for r in rows),'PASS':not failures and len(judged)==len(rows) and all(r['execution']=='Observed' for r in rows),'review':'PENDING_INDEPENDENT_REVIEW; no global/future/internal/Array-isolated-RED claim','regime':'executado sem atestacao de isolamento'}
 save(judgedpath,receipt);print(json.dumps({'raw':pin(rawpath),'judgment':pin(judgedpath),'PASS':receipt['PASS'],'cells':len(rows),'failures':len(failures),'Unknown':receipt['Unknown'],'wall_seconds':seconds}),flush=True)
if __name__=='__main__':main()
