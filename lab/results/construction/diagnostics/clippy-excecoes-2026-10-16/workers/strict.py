from pathlib import Path
import json,gzip,subprocess,collections,re,datetime
out=Path('00_nucleo/diagnosticos/clippy-excecoes-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'))
cmd=['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--','-W','clippy::too_many_arguments']
for rule in ['unfulfilled_lint_expectations','clippy::mutable_key_type','clippy::single_range_in_vec_init','clippy::type_complexity','clippy::field_reassign_with_default','clippy::new_without_default','clippy::derivable_impls','clippy::map_entry','clippy::items_after_test_module','clippy::unnecessary_literal_unwrap']:cmd.extend(['-D',rule])
with (out/'strict-clippy.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
messages={}
for line in (out/'strict-clippy.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')!='compiler-message':continue
 m=x['message'];code=(m.get('code') or {}).get('code');spans=[s for s in m.get('spans',[]) if s['is_primary']]
 if code and spans:messages[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
counts=collections.Counter(k[0] for k in messages);remaining=[]
for (code,path,offset),m in messages.items():
 if code!='clippy::too_many_arguments':
  assert m['level']=='warning',(code,path,m['message'])
  continue
 s=m['spans'];span=next(s for s in s if s['is_primary']);fragment=Path(path).read_bytes()[span['byte_start']:span['byte_end']].decode();name=re.search(r'\bfn (\w+)',fragment).group(1)
 remaining.append((path,name))
expected={(x['path'],x['function']) for x in before['pending_corrections']}
expected_counts=collections.Counter(x['code'] for x in json.loads((out/'clippy-inventory.json').read_text()));assert counts==expected_counts,(counts,expected_counts)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),command=cmd,exit=r.returncode,warnings=dict(counts),remaining_functions=remaining,expected_remaining_functions=sorted(expected))
(out/'strict-details.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0 and set(remaining)==expected and len(remaining)==11,record
print(json.dumps(record),flush=True)
