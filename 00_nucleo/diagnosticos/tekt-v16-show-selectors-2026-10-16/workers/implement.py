from pathlib import Path
import json
import subprocess
import datetime

out = Path('00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16')
scope = json.loads((out / 'scope.json').read_text())
record = {'commands': []}
for label, command in [
    ('materialize', ['python3', str(out / 'workers/materialize.py')]),
    ('seal', ['python3', str(out / 'workers/seal_owned.py'), *scope['sources']]),
    ('guard', ['python3', str(out / 'workers/guard.py')]),
]:
    item = {'label': label, 'command': command, 'start': datetime.datetime.now(datetime.timezone.utc).isoformat()}
    with (out / ('implementation-' + label + '.log')).open('w') as log:
        r = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    item.update(exit=r.returncode, end=datetime.datetime.now(datetime.timezone.utc).isoformat())
    record['commands'].append(item)
    (out / 'implementation-result.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(item), flush=True)
    assert r.returncode == 0
