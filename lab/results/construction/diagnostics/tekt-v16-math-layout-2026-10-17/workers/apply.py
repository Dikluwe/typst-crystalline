from pathlib import Path
import json,subprocess
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'))
subprocess.run(['python3',str(out/'workers/materialize.py')],check=True)
subprocess.run(['python3',str(out/'workers/seal_owned.py'),*scope['sources']],check=True)
subprocess.run(['python3',str(out/'workers/guard.py')],check=True)
