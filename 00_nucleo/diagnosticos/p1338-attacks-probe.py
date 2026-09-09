"""Fresh P1338 pre-C adversarial witnesses, preserving both diagnostic debts and parity."""
import datetime, hashlib, json, subprocess, time
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline'); D=ROOT/'00_nucleo/diagnosticos'
MANIFEST_SHA='8c51d637e9b248628d4bfe32b684a00f201b3139b84868b1f6cb351bb6290e4b'
CASES=[
 ('array-name','(7, "z", none).indisponível','vanilla'),
 ('array-empty','().ausência','vanilla'),
 ('array-span','{ let lista = (true, auto);\n (\n lista\n ).campo-ausente }','vanilla'),
 ('length-boundary','(17pt).ausência','baseline'),
 ('array-first-value','(13, 27).first','baseline'),
 ('array-first-empty','().first','baseline'),
 ('array-len-value','(13, 27).len','baseline'),
 ('array-methods','((13, 27).len(), (13, 27).first(), array.len((13, 27)))','baseline'),
 ('array-type-boundary','array.ausência','baseline'),
]
def sha(data): return hashlib.sha256(data).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def publish(path,value):
 assert not path.exists()
 body=json.dumps(value,ensure_ascii=False,indent=2)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+s+'\n' for s in body.splitlines())+'*** End Patch\n'
 p=subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True); assert p.returncode==0,p.stderr
 assert path.read_text()==body
if __name__=='__main__':
 mpath=D/'p1338-manifest.json'; assert sha(mpath.read_bytes())==MANIFEST_SHA; m=json.loads(mpath.read_text())
 r={'at_start':utc(),'manifest_sha256':MANIFEST_SHA,'baseline_sha256':m['baseline_sha256'],'script_sha256':sha(Path(__file__).read_bytes()),'cases':CASES,'observations':[],'provenance':[]}
 for argv in [['git','rev-parse','HEAD'],['git','diff','HEAD','--stat']]:
  p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True); r['provenance'].append({'argv':argv,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 for role,identity in [('baseline',m['baseline_binary']),('vanilla',m['vanilla'])]:
  assert sha(Path(identity['path']).read_bytes())==identity['sha256']
  for name,source,expected in CASES:
   argv=[identity['path'],'eval',source]; at=utc(); t=time.monotonic(); p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
   r['observations'].append({'role':role,'binary_sha256':identity['sha256'],'case':name,'source':source,'expected_role':expected,'argv':argv,'at_start':at,'at_end':utc(),'seconds':time.monotonic()-t,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 r['at_end']=utc(); path=D/'p1338-attacks-oracles.json'; publish(path,r); print(json.dumps({'path':str(path),'sha256':sha(path.read_bytes()),'runs':len(r['observations'])}))
