from pathlib import Path
import json,gzip,re,hashlib,sys
sys.path.insert(0,str(Path(__file__).resolve().parent));from rust_matches import block
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));allowed=set(scope['sources'])|set(scope['owners'].values());assert not [p for p,h in b['hashes'].items() if p not in allowed and hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h];protected_callbacks=[]
for source in ['01_core/src/compiler/stdlib/format.rs','01_core/src/compiler/stdlib/pdf.rs','01_core/src/compiler/stdlib/html/namespace.rs','01_core/src/compiler/stdlib/html/frame.rs']:
 old=b['files'][source];new=Path(source).read_text()
 for name in re.findall(r'^\s*(?:pub(?:\([^\n]+\))?\s+)?fn (native_\w+)\(',old,re.M):
  assert block(old,'fn '+name+'(')[2]==block(new,'fn '+name+'(')[2],(source,name);protected_callbacks.append(source+':'+name)
# Recover the previous constructors from the new payload literals and compare factory bodies.
def flatten(s):return re.sub(r'\s+','',re.sub(r',\s*\)',')',s))
def old_constructors(text):
 pattern=r'Func::native_format\(\s*FormatElementKind::(\w+),\s*NativeFunc\s*\{\s*name:\s*"([^"]+)",\s*call:\s*(\w+),\s*namespace:\s*Some\(([\s\S]*?)\),?\s*\},?\s*\)'
 witnesses=[]
 def replace(m):
  kind,name,callback,namespace=m.groups();assert kind.lower()==name;witnesses.append((kind,name,callback));return 'Func::native_with_namespace("'+name+'",'+callback+','+namespace+')'
 return re.sub(pattern,replace,text),witnesses
factories=[]
for source,name,expected in [('01_core/src/compiler/stdlib/format.rs','make_format_module',5),('01_core/src/compiler/stdlib/pdf.rs','make_pdf_module',1),('01_core/src/compiler/stdlib/html/namespace.rs','make_html_module',1)]:
 old=block(b['files'][source],'fn '+name+'(')[2];new=block(Path(source).read_text(),'fn '+name+'(')[2];recovered,witnesses=old_constructors(new);assert len(witnesses)==expected and flatten(old)==flatten(recovered),(source,witnesses);factories.append(dict(source=source,witnesses=witnesses,original_factory_body_preserved_except_carrier=True))
result=dict(public_contract='public-contract-after.json',original_callback_bodies_preserved=protected_callbacks,factories=factories,unrelated_sources_preserved=True,external_public_api_changes=False)
(out/'integrity-result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
