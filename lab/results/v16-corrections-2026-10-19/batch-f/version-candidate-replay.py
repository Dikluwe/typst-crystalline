from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
root=Path.cwd(); out=root/'lab/results/v16-corrections-2026-10-19/batch-f'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
state_path=out/'version-green-run/state.json.gz'
state=json.loads(gzip.decompress(state_path.read_bytes()))
assert all(sha(root/p)==h for p,h in state['inputs'].items())
assert json.loads((out/'version-green-run/freeze.json').read_text())['changed']==[]
commands=out/'version-green-run/commands.json'
assert [r['exit'] for r in json.loads(commands.read_text())]==[0,0]
product=root/'lab/target/product/debug/typst'
receipt=out/'version-candidate-build-receipt.json'
assert not receipt.exists()
receipt.write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':state['head'],'state':{'path':str(state_path),'sha256':sha(state_path)},'commands':{'path':str(commands),'sha256':sha(commands)},'product':{'path':str(product),'sha256':sha(product)}},indent=2)+'\n')
subprocess.run(['python3',str(out/'agents/adapters/replay.py'),'--output',str(out/'agents/adapters/version-candidate-replay'),'--validation-state',str(state_path),'--build-receipt',str(receipt)],check=True)
