from pathlib import Path
import json,subprocess,collections,datetime
out=Path('00_nucleo/diagnosticos/clippy-retencao-2026-10-16')
cmd=['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--']
for rule in ['dead_code','clippy::vec_box','clippy::too_many_arguments','unfulfilled_lint_expectations','clippy::mutable_key_type','clippy::single_range_in_vec_init','clippy::type_complexity','clippy::field_reassign_with_default','clippy::new_without_default','clippy::derivable_impls','clippy::map_entry','clippy::items_after_test_module','clippy::unnecessary_literal_unwrap']:cmd.extend(['-D',rule])
with (out/'strict-clippy.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
messages={}
for line in (out/'strict-clippy.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')!='compiler-message':continue
 m=x['message'];code=(m.get('code') or {}).get('code');spans=[s for s in m.get('spans',[]) if s['is_primary']]
 if code and spans:messages[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
counts=collections.Counter(k[0] for k in messages)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,exit=r.returncode,warnings=dict(counts))
(out/'strict-details.json').write_text(json.dumps(record,indent=2)+'\n')
expected=collections.Counter(x['code'] for x in json.loads((out/'clippy-inventory.json').read_text()))
assert r.returncode==0 and all(m['level']=='warning' for m in messages.values()) and counts==expected and not counts['clippy::too_many_arguments'],record
print(json.dumps(record),flush=True)
