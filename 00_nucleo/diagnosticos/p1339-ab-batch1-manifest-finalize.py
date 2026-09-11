"""Add independent acceptances received during first manifest publication.
Same batch, identical cells/definitions/recipes; no semantic execution.
"""
import datetime, hashlib, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent
def pin(name,expected=None):
 p=D/name;sha=hashlib.sha256(p.read_bytes()).hexdigest()
 if expected and sha!=expected:raise ValueError(name+' drift')
 return {'path':str(p),'sha256':sha}
prior=pin('p1339-ab-batch1-manifest.json','fd8cb62f585419c5aef606d624affb74501b792e9337e5814518f3d3b604b14b')
data=json.loads(Path(prior['path']).read_text())
acceptances=[pin('p1339-verifier-batch1-definitions-final-review-r1.md','3fe4719397a08d23e56d76dda5b0297cbfa198353d6095492d2c906b96ea3502'),pin('p1339-verifier-test-instrumentation-acceptance-r1.md','129662ea12837856a1ae56edbf846f676145fb0fb49e8fb939e10a11ad489962')]
data['predecessor']=prior
data['publication_successor']={'reason':'Two independent acceptance pins arrived while the first metadata recorder was already hashing/publication-running. Preserve first publication and append these exact authoritative review inputs only.','same_batch':1,'semantic_processes_between_publications':0,'cases_changed':0,'expectations_changed':0,'commands_changed':0,'budget_reset':False,'published_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'recorder':pin(Path(__file__).name)}
data['independent_definition_acceptances']=acceptances
data['artifacts']+=acceptances+[prior,pin(Path(__file__).name)]
data['operational_manifest']='This r2 is the sole operational manifest for aggregate batch1; predecessor is immutable publication provenance, not another focal allowance.'
out=D/'p1339-ab-batch1-manifest-r2.json'
if out.exists():raise SystemExit('immutable output exists')
text=json.dumps(data,indent=2,ensure_ascii=True)+'\n'
patch='*** Begin Patch\n*** Add File: '+str(out)+'\n'+''.join('+'+s+'\n' for s in text.split('\n')[:-1])+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
print(pin(out.name))
