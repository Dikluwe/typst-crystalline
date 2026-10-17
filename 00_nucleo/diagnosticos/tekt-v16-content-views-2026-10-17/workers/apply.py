from pathlib import Path
import json,gzip,datetime
from expected import out,expected
r=json.load(open(out/'red-result.json'));assert r['functional_red'] and not r['compiler_error'] and r['sources_unchanged'];red=json.load(gzip.open(out/'red-state.json.gz'));assert all(Path(p).read_text()==s for p,s in red['files'].items())
for p,s in expected().items():Path(p).write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),scope='Six nominal complements and transparent styled-equation carrier classification only'),indent=2)+'\n');print('Content projections implemented')
