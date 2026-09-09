"""Fresh pre-candidate evidence; writes only new adversarial artifacts via apply_patch."""
import datetime, hashlib, json, subprocess, time
from pathlib import Path
ROOT = Path('/repos/Antigravity/typst-crystalline')
DIAG = ROOT / '00_nucleo/diagnosticos'
MANIFEST_SHA = 'b089feee37941c88de47154adbde550f568d65f076716cdde5ef69beb4483b17'
CASES = [
 ('bool-name', 'false.unavailable', 'vanilla'),
 ('bool-span', '{ let sinal = true;\n (\n sinal\n ).ausência }', 'vanilla'),
 ('none-span', '{ let vazio = none;\n (\n vazio\n ).ausência }', 'vanilla'),
 ('auto-span', '{ let escolha = auto;\n (\n escolha\n ).ausência }', 'vanilla'),
 ('array-boundary', '(3, 5).ausência', 'baseline'),
 ('bool-type-boundary', 'bool.ausência', 'baseline'),
 ('positive', '(not false, type(true), repr(none), repr(auto))', 'baseline'),
]
def sha(data): return hashlib.sha256(data).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def publish(path, value):
 assert not path.exists(), path
 body = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
 patch = '*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
 p = subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True)
 assert p.returncode == 0, p.stderr
 assert path.read_text() == body
if __name__ == '__main__':
 mpath=DIAG/'p1337-manifest.json'; assert sha(mpath.read_bytes()) == MANIFEST_SHA
 m=json.loads(mpath.read_text())
 result={'at_start':utc(),'manifest_sha256':MANIFEST_SHA,'script_sha256':sha(Path(__file__).read_bytes()),'cases':CASES,'observations':[],'provenance':[]}
 for command in [['git','rev-parse','HEAD'],['git','diff','HEAD','--stat']]:
  p=subprocess.run(command,cwd=ROOT,capture_output=True,text=True); result['provenance'].append({'argv':command,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 for role,identity in [('baseline',m['baseline_binary']),('vanilla',m['vanilla'])]:
  assert sha(Path(identity['path']).read_bytes()) == identity['sha256']
  for case,source,expected in CASES:
   argv=[identity['path'],'eval',source]; at=utc(); t=time.monotonic()
   p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
   result['observations'].append({'role':role,'binary_sha256':identity['sha256'],'case':case,'source':source,'expected_role':expected,'argv':argv,'at_start':at,'at_end':utc(),'seconds':time.monotonic()-t,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 result['at_end']=utc(); path=DIAG/'p1337-attacks-oracles.json'; publish(path,result)
 print(json.dumps({'path':str(path),'sha256':sha(path.read_bytes()),'observations':len(result['observations'])}))
