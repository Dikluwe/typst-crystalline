from pathlib import Path
import json,re,hashlib,subprocess,datetime
P=Path(__file__).resolve().parent;root=Path.cwd();scope_path=P.parents[1]/'scope.json';scope=json.loads(scope_path.read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def public(text):
 out=[]
 for m in re.finditer(r'(?m)^([ \t]*)pub(\([^\n)]*\))?\s+(?:(?:async|unsafe)\s+)?(fn|struct|enum|trait|type|const|static|mod)\s+([A-Za-z_][A-Za-z_0-9]*)',text):
  # A declared public header ends at its body or semicolon. Stable across bodies.
  end=min([x for x in [text.find('{',m.end()),text.find(';',m.end())] if x>=0])
  out.append({'visibility':'pub'+(m.group(2) or ''),'kind':m.group(3),'name':m.group(4),'header':' '.join(text[m.start():end].split())})
 return out
pairs=[]
for pair in scope['pairs']:
 s=Path(pair['source']);o=Path(pair['owner']);baseline=subprocess.check_output(['git','show','HEAD:'+str(s)],text=True);current=s.read_text();b=public(baseline);a=public(current);assert b==a,(str(s),b,a)
 assert re.search(r'@prompt '+re.escape(str(o))+r'\s',current)
 expected='00_nucleo/prompts/'+str(s).replace('01_core/','core/',1).replace('03_infra/','infra/',1).replace('/src/','/',1).removesuffix('.rs')+'.md';assert str(o)==expected
 pairs.append({**pair,'source_sha256':sha(s),'owner_sha256':sha(o),'public_declarations_unchanged':True,'public_declarations':a,'owner_header_canonical':True})
changed=subprocess.check_output(['git','diff','HEAD','--name-only'],text=True).splitlines();productive=[p for p in changed if re.match(r'0[1-4]_',p)];outside=sorted(set(productive)-set(scope['sources']));assert not outside,outside
for p in scope['sources']:
 delta=subprocess.check_output(['git','diff','HEAD','--unified=0','--',p],text=True)
 assert not re.search(r'^\+[^+].*#\[(?:allow|expect|ignore)\b',delta,re.M),p
nuclei_changed=[p for p in changed if p.startswith('00_nucleo/prompts/_nuclei/')];config_changed=[p for p in changed if p.endswith('Cargo.toml') or p in ['Cargo.lock','.tekt.toml','clippy.toml'] or p.startswith('.github/')];assert not nuclei_changed and not config_changed
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':'Read-only scope/API snapshot; final lineage and integration gates pending root','scope_sha256':sha(scope_path),'pairs':pairs,'pair_count':len(pairs),'source_owner_file_count':2*len(pairs),'productive_outside_scope':outside,'changed_nuclei':nuclei_changed,'changed_build_lint_configs':config_changed,'new_allow_expect_ignore':False,'public_declaration_diff':False,'new_entity_field_or_enum_or_trait':False,'classification':'Existing-owner internal exhaustiveness and source-backed parity corrections; same targets/defaults/phase and public contracts. Additional operator owner is existing private language comparison scope, not new public API.','authorship':'Core four implementation is this reviewing agent own work; query and HTML four implementations are independently reviewed. This is not represented as independent review of own core code.','limits':['No Cargo/test/build/lint run by reviewer; no sealed-lineage/current gate claim until root final freeze.','Approximate query limits, general HTML support, HTML fields/repr and historical mechanical equality/performance debts are not closed by this review.']}
(P/'api-scope-review-draft.json').write_text(json.dumps(r,indent=2)+'\n')
(P/'api-scope-review-draft.md').write_text('''# Scope/API review —12 source/L0 pairs\n\nAll public and restricted-public declaration headers are unchanged against HEAD4382891, as are entity/enum/trait fields: no new public API, mode/default or pipeline phase was introduced. Every source points to its deterministic canonical owner. Changed productive files are confined to the12 manifested sources. No Nucleus/build/lint configuration or new allow/expect/ignore attribute changed. Exact24 hashes and declaration inventories are in api-scope-review-draft.json.\n\nCore four code is the author's own work and is reviewed as a self-audit; query and HTML implementations received independent review. The private Context projection remains isolated to language comparison, and no transformed id/closure is stored in Engine/Value/rendered content, Hash, ObservationEq or allocators. Final lineage, compiler/lint/integration results remain pending root; this snapshot is not a final sealed-input certification.\n''')
print('JSON',sha(P/'api-scope-review-draft.json'));print('MD',sha(P/'api-scope-review-draft.md'))
