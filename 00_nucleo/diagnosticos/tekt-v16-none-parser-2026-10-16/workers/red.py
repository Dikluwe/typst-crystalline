from pathlib import Path
import subprocess
import os
import json
import datetime
import gzip
import hashlib

out = Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
before = json.load(gzip.open(out/'before.json.gz'))
root = Path(before['source'])
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
record = {'start':datetime.datetime.now(datetime.timezone.utc).isoformat(), 'commands':[]}
env = dict(os.environ, CARGO_TARGET_DIR='/tmp/tekt-linter-none-target', CARGO_BUILD_JOBS='1')
for label, command, expected in [
    ('fmt',['rustfmt','--edition','2021','--config','skip_children=true','03_infra/rs_parser.rs'],0),
    ('seal',[before['old_binary']['binary'],'.','--checks','v1,v5,v15,v26','--fix-hashes'],0),
    ('tests',['cargo','test','--offline','--locked','--lib','v16_none_','--','--test-threads=1'],101),
]:
    print(json.dumps({'label':label,'command':command}),flush=True)
    with (out/('red-'+label+'.log')).open('w') as log:
        result=subprocess.run(command,cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
    item={'label':label,'command':command,'exit':result.returncode}
    record['commands'].append(item)
    (out/'red-result.json').write_text(json.dumps(record,indent=2)+'\n')
    assert result.returncode==expected,item
text=(out/'red-tests.log').read_text()
assert '3 passed; 6 failed;' in text,text[-3000:]
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              candidate_hashes={str(p.relative_to(root)):sha(p) for p in root.rglob('*') if p.is_file()},
              expected_failed=6, negative_controls_passed=3)
(out/'red-result.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'RED_verified':True,'failed':6,'negative_controls_passed':3}),flush=True)
