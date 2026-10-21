"""Read-only independent seals/API audit against a named immutable reseal."""
from pathlib import Path
import argparse,json,gzip,hashlib,re,subprocess,datetime,tomllib
A=argparse.ArgumentParser();A.add_argument('--seal',type=Path,required=True);A.add_argument('--tag',required=True);a=A.parse_args()
P=Path(__file__).resolve().parent;R=Path.cwd();S=P.parents[1]/'scope.json';scope=json.loads(S.read_text());seal=json.load(gzip.open(a.seal,'rt'));assert seal['exit']==0 and seal['outside_changes']==[]
assert len(scope['pairs'])==13 and len(scope['sources'])==13 and len(scope['owners'])==13
assert len(set(scope['sources']+scope['owners']))==26 and set(scope['sources'])=={p['source'] for p in scope['pairs']} and set(scope['owners'])=={p['owner'] for p in scope['pairs']}
sha=lambda b:hashlib.sha256(b).hexdigest();digest=lambda p:sha(Path(p).read_bytes());utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
output=P/('api-scope-review-'+a.tag+'.json');assert not output.exists()
assert all(digest(R/p)==h for p,h in seal['after'].items()),'current inputs drift from named seal'
# Exact authorized metadata-line removal; never strip a matching body example.
def strip_meta(data,marker,source):
 out=[];count=0;active=True
 for line in data.splitlines(keepends=True):
  body=line.removesuffix(b'\n').removesuffix(b'\r')
  if active and body.startswith(marker):
   assert re.fullmatch(rb'[0-9a-f]{8}',body[len(marker):]);count+=1
  else:out.append(line)
  if source and not body.startswith(b'//!'):active=False
  elif not source and (not body or body.lstrip().startswith((b'```',b'~~~'))):active=False
 assert count==1
 return b''.join(out)
def framed(data,domain,deps):
 h=hashlib.sha256(data);h.update(b'\0'+domain+b'\0')
 for p,d in sorted(deps):
  q=p.encode();h.update(len(q).to_bytes(8,'big')+q+b' '+d)
 return h.digest()
cache={};active=set()
def nucleus(p):
 if p in cache:return cache[p]
 assert p not in active and len(active)<256;active.add(p);data=(R/p).read_bytes();d=tomllib.loads(data.decode());assert d['tekt']==1 and d['kind']=='nucleus';deps=[]
 for x in d.get('depends',[]):
  h=nucleus(x['path']);assert h.hex()==x['sha256'];deps.append((x['path'],h))
 active.remove(p);h=framed(data,b'TEKT-NUCLEUS-DEPS-V1',deps);cache[p]=h;return h
# All public declarations retain their header and type fields; function bodies excluded.
def surface(text):
 out=[]
 for m in re.finditer(r'(?m)^([ \t]*)pub(\([^\n)]*\))?\s+(?:(?:async|unsafe)\s+)?(fn|struct|enum|trait|type|const|static|mod)\s+([A-Za-z_][A-Za-z_0-9]*)',text):
  ends=[x for x in [text.find('{',m.end()),text.find(';',m.end())] if x>=0];end=min(ends)
  if m.group(3) in ['struct','enum','trait'] and text[end]=='{':
   n=1;i=end+1
   while n:
    if text[i]=='{':n+=1
    elif text[i]=='}':n-=1
    i+=1
   end=i
  out.append(' '.join(text[m.start():end].split()))
 return out
pairs=[]
for pair in scope['pairs']:
 s=R/pair['source'];o=R/pair['owner'];sb=s.read_bytes();ob=o.read_bytes();st=sb.decode();ot=ob.decode();declared=re.search(rb'(?m)^//! @prompt-hash ([0-9a-f]{8})$',sb).group(1).decode();reverse=re.search(rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})$',ob).group(1).decode()
 actual_code=sha(strip_meta(sb,b'//! @prompt-hash ',True))[:8];deps=[]
 refs=re.findall(r'(?m)^- (00_nucleo/prompts/_nuclei/\S+\.toml) sha256:([0-9a-f]{64})$',ot)
 assert refs==sorted(refs) and len(refs)==len(set(p for p,h in refs))
 for path,pin in refs:
  h=nucleus(path);assert h.hex()==pin;deps.append((path,h))
 clean=strip_meta(ob,'Hash do Código: '.encode(),False);actual_prompt=(framed(clean,b'TEKT-PROMPT-NUCLEI-V1',deps) if deps else hashlib.sha256(clean).digest()).hex()[:8]
 assert actual_code==reverse and actual_prompt==declared,(pair,actual_code,reverse,actual_prompt,declared)
 assert re.search(r'(?m)^//! @prompt '+re.escape(pair['owner'])+r'$',st)
 expected='00_nucleo/prompts/'+pair['source'].replace('01_core/','core/',1).replace('03_infra/','infra/',1).replace('/src/','/',1).removesuffix('.rs')+'.md';assert expected==pair['owner']
 base=subprocess.check_output(['git','show','HEAD:'+pair['source']],text=True);assert surface(base)==surface(st),pair['source']
 delta=subprocess.check_output(['git','diff','HEAD','--unified=0','--',pair['source']],text=True);assert not re.search(r'^\+[^+].*#\[(?:allow|expect|ignore)\b',delta,re.M)
 pairs.append({**pair,'source_sha256':sha(sb),'owner_sha256':sha(ob),'source_prompt_hash':declared,'computed_effective_prompt_hash':actual_prompt,'owner_code_hash':reverse,'computed_source_hash_without_header':actual_code,'direct_nucleus_pins':dict(refs),'canonical_owner':True,'public_declarations_and_type_fields_unchanged':True})
changed=subprocess.check_output(['git','diff','HEAD','--name-only'],text=True).splitlines();productive=[p for p in changed if re.match(r'0[1-4]_',p)];allowed=set(scope['sources']+scope['owners']);assert set(productive)==set(scope['sources']);normative=[p for p in changed if p.startswith('00_nucleo/prompts/')];assert set(normative)==set(scope['owners']);assert set(changed)==allowed,changed
assert not subprocess.check_output(['git','diff','HEAD','--','Cargo.toml','Cargo.lock','clippy.toml','.tekt.toml','crystalline.toml','00_nucleo/prompts/_nuclei'],text=True)
reviews={}
for relative in ['agents/html/core-implementation-review.json','agents/contexts/hash-depth-plan-independent-review.json','agents/contexts/hash-depth-implementation-independent-review.json','agents/core/query-implementation-review.json','agents/core/html-core-contract-review.json']:
 review=S.parent/relative;assert review.is_file();reviews[relative]={'sha256':digest(review),'evidence_role':'Independent core review by a different agent' if '/html/core-' in relative or '/contexts/hash-' in relative else 'Independent HTML/query review by core author; earlier scoped snapshot, not a later-build certificate'}
state={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':'PASS API/scope/independent forward+reverse seal audit for this exact named snapshot; executable/full gate closure is separate','seal':str(a.seal),'seal_sha256':digest(a.seal),'scope_sha256':digest(S),'pair_count':len(pairs),'source_owner_file_count':2*len(pairs),'pairs':pairs,'all_frozen_inputs_match_named_seal':len(seal['after']),'unique_effective_nuclei':{p:h.hex() for p,h in sorted(cache.items())},'unique_effective_nucleus_count':len(cache),'public_signature_entity_trait_enum_default_phase_change':False,'new_allow_expect_ignore_or_config_change':False,'outside_productive_or_normative_changes':[],'author_roles':'Core implementation author self-audit is explicitly disclosed; HTML4/query implementation reviewed independently. Parent/other agent independent core review and actual tests remain separate evidence.','proof_method':f'Independent Python SHA256 metadata stripping and domain-framed recursive Nucleus hashing from pinned linter source algorithm; no repair/linter/Cargo invocation. Baseline-to-current full public header and type-field equality; deterministic owner/header paths; exact {2*len(pairs)}-file change set.', 'independent_source_reviews':reviews, 'debug_depth_refinement':{'source':'01_core/src/entities/content/identity.rs','owner':'00_nucleo/prompts/core/entities/content/identity.md','classification':scope['depth_scope_extension']['classification'],'measurement_receipt_sha256':scope['depth_scope_extension']['evidence_sha256'],'limits':'The old mechanical output-preservation clause is superseded only where four invisible carriers consumed visible depth and violated established transparency/equal-value hashing; visible MAX_DEPTH8 and all nominal formatting, algorithms, causal identities and public signatures remain unchanged. No blanket numerical-hash or debug-output preservation claim; no new default or phase.','refuted_hypothesis':scope['depth_scope_extension']['not_evidence_of_a_debt']} ,'hash_algorithm_evidence':{p:digest(p) for p in ['/repos/Antigravity/tekt-linter/03_infra/nucleus.rs','/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs','/repos/Antigravity/tekt-linter/03_infra/prompt_io.rs']},'limits':['No complete language/equality/HTML/query parity claim.','Documented approximate projection, HTML fields/repr/unsupported semantic kinds, historical mechanical false cases and bare-Math performance target remain explicitly bounded.','No Cargo, formatter, reseal or source/L0 change executed by reviewer.','Any later source/L0 mutation supersedes this snapshot review without changing its historical evidence.']}
assert all(digest(R/p)==h for p,h in seal['after'].items()),'post-review drift'
output.write_text(json.dumps(state,indent=2)+'\n');md=output.with_suffix('.md');md.write_text(f"""# Scope/API audit — {len(pairs)} sealed pairs

PASS for the exact named seal and hashes bound in the JSON companion. All {2*len(pairs)} source/L0 files match the recorded frozen snapshot. Both directions of all {len(pairs)} seals and every effective Nucleus pin were independently recalculated. Canonical owner paths/headers are correct. Public and restricted-public signatures and entity/type fields remain unchanged; no new suppression/configuration or out-of-scope productive/normative file changed. No public API, default, compatibility expansion or pipeline phase was added.

The core implementation author performed this self-audit transparently. Independent core implementation and carrier-depth plan/implementation reviews by the other agents are linked and hashed in the JSON; HTML and query also received independent source reviews. Those scoped historical source reviews and this metadata/API audit do not replace final compiler/lint tests, actual frozen-corpus replay or the source-bound hash probe.

The identity refinement explicitly supersedes only old mechanical Debug-output preservation where Source/ShowRecipe/Located/Realized consumed visible depth and violated established transparency and equal-value hashing. Borrowed projection precedes the same MAX_DEPTH8; nominal formatting, hashing algorithms, causal/observational identities, defaults and phases remain unchanged. The public hash contract already permits numeric results to follow separately specified Debug corrections; equal values must still hash equally. The actual nested Float control refuted its suggested mismatch and is not reported as an observed gap. Finite tests do not establish a universal Hash law or full language parity.

Known approximation/equality/HTML/representation/performance limits remain. No source/L0 mutation, Cargo, formatting or resealing was performed by this audit. Any later mutation supersedes this snapshot rather than retroactively certifying it.
""");print(output,digest(output));print(md,digest(md));print('nuclei',len(cache),'pairs',len(pairs))
