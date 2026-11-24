from datetime import datetime, timezone
from pathlib import Path
import hashlib
import json
import subprocess
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
out = root / 'lab/results/construction/diagnostics' / experiment.name
source = Path(sys.argv[1])
manifest = out.parent / 'reserve-locations-borrowed-2026-11-21/candidate-source.sha256'
expected = json.loads((out/'changed-files.json').read_text())
observed = {}
lines = manifest.read_text().splitlines()
for line in lines:
    sha, name = line.split(maxsplit=1)
    name = name.removeprefix('./')
    actual = hashlib.sha256((source/name).read_bytes()).hexdigest()
    if actual != sha:
        observed[name] = actual
assert observed == expected
patch = []
for name, path in [('metrics','font_metrics'),('fallback','font_metrics/fallback'),('session','shaper/complete_session')]:
    for ext, prefix in [('rs','03_infra/src'),('md','00_nucleo/prompts/infra')]:
        filename = f'{prefix}/{path}.{ext}'
        result = subprocess.run(['diff','-u','--label','a/'+filename,'--label','b/'+filename,
                                 str(out/f'before-{name}.{ext}'),str(out/f'candidate-{name}.{ext}')],
                                stdout=subprocess.PIPE, check=False)
        assert result.returncode == 1
        patch.append(result.stdout)
(out/'candidate.patch').write_bytes(b''.join(patch))
(out/'freeze.json').write_text(json.dumps({
    'utc': datetime.now(timezone.utc).isoformat(),
    'head': (out/'head.txt').read_text().strip(),
    'files_verified': len(lines),
    'delta': observed,
},indent=2)+'\n')
