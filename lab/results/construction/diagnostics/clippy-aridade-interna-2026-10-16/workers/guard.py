from pathlib import Path
import json,gzip,re,subprocess,hashlib,datetime
out=Path('00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));records=[]
def span(s,marker):
 start=s.index(marker);opening=s.index('{',start);depth=1;at=opening+1
 while depth:depth+=int(s[at]=='{')-int(s[at]=='}');at+=1
 return start,opening,at
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,flags=re.M)
def fmt(s):return subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true','--emit','stdout'],input=s,capture_output=True,text=True,check=True).stdout
def bodyfmt(body):return fmt('fn guard() {\n'+body.strip()+'\n}\n')
for source in before['owners']:
 old=before['files'][source]['text'];new=Path(source).read_text();assert re.findall(r'\bpub fn [^{;]+',old)==re.findall(r'\bpub fn [^{;]+',new),source
 tests=lambda s:sorted(re.findall(r'#\[test\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))? )?fn (\w+)',s))
 assert tests(old)==tests(new),source;records.append(dict(source=source,externally_public_signatures_unchanged=True,test_names_unchanged=True))
for source in ['03_infra/src/export/render.rs','03_infra/src/export/svg/glyphs.rs']:
 old=before['files'][source]['text'];new=Path(source).read_text();a,b,c=span(old,'fn render_text_shaped(');x,y,z=span(new,'fn render_text_shaped(');assert bodyfmt(old[b+1:c-1])==bodyfmt(new[y+1:z-1]),source
 assert '_text:' not in new[x:y];records.append(dict(source=source,render_body_unchanged=True,unused_argument_removed=True))
source='03_infra/src/pipeline/pdf.rs';old=before['files'][source]['text'];new=Path(source).read_text();a,b,c=span(old,'fn compile_to_pdf_bytes_impl(');x,y,z=span(new,'fn compile_to_pdf_bytes_impl(')
new_body=new[y+1:z-1];new_body,n=re.subn(r'^\s*let PdfCompileRequest\s*\{[^}]*\}\s*=\s*request;','',new_body,count=1);assert n==1
assert bodyfmt(old[b+1:c-1])==bodyfmt(new_body)
fields=['full_error','document_id','stream_mode','pdf_tagged_override','features','pretty_override','pages_override','standard_override'];pattern=r'PdfCompileRequest\s*\{\s*'+''.join(re.escape(f)+r',\s*' for f in fields)+r'oracle:\s*(false|true),\s*\}'
flags=re.findall(pattern,new);assert flags==['false','false','true'],flags
records.append(dict(source=source,pdf_body_unchanged_after_unpack=True,direct_request_oracle_values=flags,request_fields=fields+['oracle']))
source='03_infra/src/export/gradients/mod.rs';old=before['files'][source]['text'];new=Path(source).read_text();start=old.index('pub(super) fn scan_all_gradients(');a,b,c=span(old[start:],'    fn walk(');old_walk=old[start:][a:c];start_new=new.index('pub(super) fn scan_all_gradients(');x,y,z=span(new[start_new:],'    fn walk(');new_walk=new[start_new:][x:z];body=new[start_new:][y+1:z-1]
body=re.sub(r'\bcollector\s*\.\s*(ptr_to_idx|refs|grad_objs)\b',lambda m:m[1],body);body=re.sub(r'\bcollector\s*\.\s*(next_id|counter)\b',lambda m:'*'+m[1],body)
body,n=re.subn(r'\bcollector\b','ptr_to_idx, refs, grad_objs, next_id, counter',body);assert n==4,n
old_body=old[start:][b+1:c-1];assert bodyfmt(old_body)==bodyfmt(body)
# Only the discovery function changes; resources traversal and all its tests stay intact.
a,b,c=span(old,'pub(super) fn scan_all_gradients(');x,y,z=span(new,'pub(super) fn scan_all_gradients(');assert fmt(strip(old[:a]+old[c:]))==fmt(strip(new[:x]+new[z:]))
assert re.search(r'next_id:\s*first_id,\s*counter:\s*1,',new)
records.append(dict(source=source,discovery_body_equivalent_under_state_field_projection=True,recursive_call_sites=4,resource_lookup_and_tests_unchanged=True))
source='03_infra/src/export/stream.rs';old=before['files'][source]['text'];new=Path(source).read_text();a,b,c=span(old,'    pub(crate) fn type1(');x,y,z=span(new,'    pub(crate) fn with_font_scenario(')
old_body=old[b+1:c-1].replace('font_scenario: FontScenario::Type1,','font_scenario,');assert bodyfmt(old_body)==bodyfmt(new[y+1:z-1]);assert 'fn cidfont(' not in new and 'fn multifont(' not in new
records.append(dict(source=source,context_field_defaults_unchanged=True,existing_scenario_enum_unchanged=old[old.index('pub(crate) enum FontScenario'):old.index('/// **P281** — agregador')]==new[new.index('pub(crate) enum FontScenario'):new.index('/// **P281** — agregador')]))
assert records[-1]['existing_scenario_enum_unchanged']
# Fixture adapters preserve all old expressions and every assertion outside calls.
source='03_infra/src/export/tests/mod.rs';old=before['files'][source]['text'];new=Path(source).read_text()
def argsplit(s):
 parts=[];start=0;depth=0
 for i,c in enumerate(s):
  if c in '([{':depth+=1
  elif c in ')]}':depth-=1
  elif c==',' and depth==0:
   if s[start:i].strip():parts.append(s[start:i].strip())
   start=i+1
 if s[start:].strip():parts.append(s[start:].strip())
 return parts
def calls(s,pattern):
 result=[]
 for m in re.finditer(pattern,s):
  opening=m.end()-1;depth=1;end=opening+1
  while depth:depth+=int(s[end]=='(')-int(s[end]==')');end+=1
  result.append((m.start(),end,argsplit(s[opening+1:end-1]),m))
 return result
olds=calls(old,r'PageContext::(cidfont|multifont)\(');news=calls(new,r'PageContext::with_font_scenario\(');assert len(olds)==len(news)==5
canon=lambda s:re.sub(r'\s+','',s)
for previous,current in zip(olds,news):
 old_args=previous[2];new_args=current[2];kind=previous[3][1];assert len(new_args)==6
 assert [canon(v) for v in old_args[:4]]==[canon(v) for v in new_args[:4]] and canon(old_args[-1])==canon(new_args[-1])
 fields=['char_to_gid','glyph_mapping','glyph_to_nominal','bitmap'] if kind=='cidfont' else ['fonts','per_font_char_to_gid','per_font_glyph_mapping','per_font_glyph_to_nominal','per_font_glyph_reverse','per_font_bitmap']
 scenario=new_args[4];variant='Cidfont' if kind=='cidfont' else 'Multifont';assert scenario.startswith('FontScenario::'+variant)
 values={}
 for field in argsplit(scenario[scenario.index('{')+1:scenario.rfind('}')]):
  key,sep,value=field.partition(':');values[key.strip()]=value.strip() if sep else key.strip()
 expected=dict(zip(fields,old_args[4:-1]))
 if kind=='multifont':expected['bindings']='Some(crate::export::font_binding::LookupCache::default())'
 assert {k:canon(v) for k,v in values.items()}=={k:canon(v) for k,v in expected.items()}
def mask(s,found):
 for start,end,args,m in reversed(found):s=s[:start]+'PageContext::fixture_constructor()'+s[end:]
 return s
masked_new=mask(new,news).replace('use crate::export::stream::FontScenario;\n','');assert fmt(strip(mask(old,olds)))==fmt(strip(masked_new))
records.append(dict(source=source,fixture_calls_migrated=5,resource_argument_order_and_scenario_field_values_preserved=True,all_code_fixtures_and_assertions_outside_constructor_calls_unchanged=True))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert {p for p,h in before['all_source_hashes'].items() if sha(p)!=h}==set(before['files'])
(out/'code-guard.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),records=records,scope_owners_consumers_only=True,current_hashes={p:sha(p) for p in before['files']}),ensure_ascii=False,indent=2)+'\n')
print(json.dumps(records))
