from pathlib import Path
import json,gzip,re,subprocess,hashlib,datetime
out=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));records=[]
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,flags=re.M)
def fmt(s):
 r=subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true','--emit','stdout'],input=strip(s),capture_output=True,text=True,check=True);return r.stdout
for p in before['plans']:
 old=before['files'][p]['text'];new=Path(p).read_text()
 assert re.findall(r'pub(?:\([^)]*\))? fn [^{;]+',old)==re.findall(r'pub(?:\([^)]*\))? fn [^{;]+',new),p
 names=lambda s:sorted(re.findall(r'#\[(?:test|ignore)[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))? )?fn (\w+)',s))
 assert names(old)==names(new),p
 record=dict(source=p,public_function_signatures_unchanged=True,test_names_preserved=True)
 if p.startswith('04_wiring/tests/'):
  reverse=lambda m:f'assert!({m[1]}.success(), "stderr:\\n{{}}", {m[1]}.stderr);'
  reverted,count=re.subn(r'assert!\(\s*(\w+)\.success\(\),\s*"stdout:\\n\{\}\\nstderr:\\n\{\}",\s*\1\.stdout,\s*\1\.stderr,?\s*\);',reverse,new)
  assert count>0,p
  assert fmt(reverted)==fmt(old),p
  record.update(only_failure_details_changed=True,stdout_diagnostics=count)
 if p in ['03_infra/src/pipeline/html.rs','03_infra/src/pipeline/svg.rs']:
  mod='p1465_tests' if '/html.' in p else 'p1462_tests';anchor=f'#[cfg(test)]\nmod {mod} {{'
  def detach(s):
   start=s.index(anchor);end=s.index('\n}\n',start)+3
   return s[:start]+s[end:],s[start:end]
  rest_old,tests_old=detach(old);rest_new,tests_new=detach(new)
  assert tests_old==tests_new and fmt(rest_old)==fmt(rest_new),p
  record.update(test_module_body_unchanged=True,productive_helpers_unchanged=True)
 if p=='03_infra/src/watch.rs':
  reverted=new.replace('    thread: Option<thread::JoinHandle<()>>,','    thread: Option<thread::JoinHandle<()>>,\n    live: bool,',1).replace('Ok(Self { addr, state, stop, thread: Some(thread) })','Ok(Self { addr, state, stop, thread: Some(thread), live })',1)
  assert fmt(old)==fmt(reverted)
  record['only_private_duplicate_bool_removed']=True
 if p=='03_infra/src/plugin_host/tests.rs':
  assert fmt(old.replace('const I64: u8 = 0x7e;\n',''))==fmt(new)
  record['fixture_bytes_and_tests_unchanged']=True
 records.append(record)
assert {p for p,h in before['all_source_hashes'].items() if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h}==set(before['files'])
(out/'contract-test-guard.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files=records,changed_owners_and_consumers_only=True),indent=2)+'\n')
print(json.dumps(records))
