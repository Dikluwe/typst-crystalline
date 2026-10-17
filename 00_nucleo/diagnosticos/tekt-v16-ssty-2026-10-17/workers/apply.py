from pathlib import Path
import json,gzip,datetime
from expected import expected,out
assert not json.load(open(out/'before-guard-result.json'))['pass_guard'];scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));assert all(Path(p).read_text()==b['files'][p] for p in scope['sources']);assert (out/'l0-record.json').exists()
for p,text in expected().items():Path(p).write_text(text)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources=scope['sources'],kind='Nominal exhaustive complements only; no behavioral change'),indent=2)+'\n');print('Two ssty matches made nominal')
