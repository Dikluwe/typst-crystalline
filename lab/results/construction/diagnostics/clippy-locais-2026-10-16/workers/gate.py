from pathlib import Path
import json,subprocess,collections,sys,datetime
out=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16');mode=sys.argv[1]
rules=['clippy::derivable_impls','clippy::map_entry','clippy::items_after_test_module','clippy::unnecessary_literal_unwrap']
if mode=='red-tests':rules=['clippy::items_after_test_module','clippy::unnecessary_literal_unwrap']
cmd=['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--','-A','warnings']
for rule in rules+['clippy::new_without_default','clippy::field_reassign_with_default','clippy::type_complexity','unfulfilled_lint_expectations']:cmd.extend(['-D',rule])
with (out/(mode+'-clippy.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
messages={}
for line in (out/(mode+'-clippy.log')).read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')!='compiler-message':continue
 m=x['message'];spans=[s for s in m.get('spans',[]) if s['is_primary']];code=(m.get('code') or {}).get('code')
 if m['level']=='error' and code and spans:messages[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
counts=collections.Counter(k[0] for k in messages)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,exit=r.returncode,errors=dict(counts),mode=mode)
(out/(mode+'-details.json')).write_text(json.dumps(record,indent=2)+'\n')
if mode.startswith('red'):
 assert r.returncode==101
 expected={'clippy::derivable_impls':3,'clippy::map_entry':1} if mode=='red' else {'clippy::items_after_test_module':2,'clippy::unnecessary_literal_unwrap':1}
 assert counts==expected,counts
else:assert r.returncode==0 and not counts,record
print(json.dumps(record),flush=True)
