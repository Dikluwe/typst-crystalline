from pathlib import Path
import re,json,gzip,subprocess,datetime,hashlib
out=Path('00_nucleo/diagnosticos/clippy-excecoes-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));allowed={(p['rule'],p['reason']) for p in before['plans']};records=[];seen=[]
def normalize(s):
 s=re.sub(r'^//! @prompt-hash [^\n]*\n','',s,flags=re.M)
 return subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true','--emit','stdout'],input=s,capture_output=True,text=True,check=True).stdout
for source in before['owners']:
 old=before['files'][source]['text'];new=Path(source).read_text();removed=[]
 def remove(match):
  body=match[1];rule=re.match(r'\s*([\w:]+)',body).group(1);quoted=re.search(r'\breason\s*=\s*("(?:\\.|[^"\\])*")',body)
  if not quoted:return match[0]
  reason=json.loads(quoted[1])
  if (rule,reason) not in allowed:return match[0]
  removed.append(dict(rule=rule,reason=reason));seen.append((rule,reason));return ''
 stripped=re.sub(r'(?ms)^[ \t]*#\[expect\((.*?)\)\]\n',remove,new)
 assert normalize(stripped)==normalize(old),source
 old_fn=re.findall(r'pub(?:\([^)]*\))? fn [^{;]+',old);new_fn=re.findall(r'pub(?:\([^)]*\))? fn [^{;]+',new);assert old_fn==new_fn,source
 records.append(dict(source=source,added_expectations=removed,only_lint_attributes_and_lineage_header_changed=True,existing_signatures_bodies_cfg_tests_unchanged=True))
assert len(seen)==len(before['plans'])==49 and set(seen)==allowed
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert {p for p,h in before['all_source_hashes'].items() if sha(p)!=h}==set(before['files'])
(out/'code-guard.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),attributes=len(seen),files=records,current_hashes={p:sha(p) for p in before['files']},scope_owners_consumers_only=True),ensure_ascii=False,indent=2)+'\n')
print(json.dumps(dict(attributes=len(seen),consumers=len(records),body_signature_cfg_test_changes=0)))
