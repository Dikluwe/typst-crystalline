"""Archive GDB clone counts and exact binary/fixture/status provenance."""
import datetime, hashlib, json, re, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
logs={
 'plain':D/'clone-logs/item6-gdb-final-plain.log',
 'array_20000':D/'clone-logs/item6-gdb-final-array20000.log',
 'array_20000_no_len':D/'clone-logs/item6-gdb-final-array20000-no-len.log',
 'dict_pair':D/'clone-logs/item6-gdb-final-dict-pair.log'}
counts={name:{k:int(v) for k,v in re.findall(r'([A-Z_]*VALUE_CLONES)=(\d+)',p.read_text())} for name,p in logs.items()}
assert counts['plain']['VALUE_CLONES']==1336
assert counts['array_20000']['VALUE_CLONES']==41350
assert counts['array_20000_no_len']['VALUE_CLONES']==1348
assert counts['dict_pair']['DICT_WITH_LEN_VALUE_CLONES']==19360
assert counts['dict_pair']['DICT_NO_LEN_VALUE_CLONES']==15358
status=(D/'clone-git-state-at-start.txt').read_bytes()
state={'utc_capture':datetime.datetime.now(datetime.timezone.utc).isoformat(),
       'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
       'working_tree':'uncommitted',
       'status_at_measurement_sha256':hashlib.sha256(status).hexdigest(),
       'status_unchanged_during_measurement':status==Path('/tmp/item6-gdb-state-at-end.txt').read_bytes(),
       'debug_binary_sha256':sha(R/'target/debug/typst'),
       'gdb_script_sha256':sha(D/'clone-count.gdb'),
       'gdb_dict_pair_script_sha256':sha(D/'clone-dict-two.gdb'),
       'fixtures':{str(p.relative_to(R)):sha(p) for p in sorted([*(D/'fixtures').glob('*.typ'),*(D/'probe-fixtures').glob('*.typ')])},
       'logs':{k:{'sha256':sha(v),'modified_utc':datetime.datetime.fromtimestamp(v.stat().st_mtime,datetime.timezone.utc).isoformat(),'counts':counts[k]} for k,v in logs.items()},
       'method':'GDB breakpoint on <typst_core::entities::value::Value as core::clone::Clone>::clone, command increments counter then continues; debug CLI, same source and fixtures, counts not timings.'}
(D/'clone-results.json').write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(counts,indent=2))
