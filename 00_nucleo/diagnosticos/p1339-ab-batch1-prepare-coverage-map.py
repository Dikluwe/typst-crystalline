"""Publish exact declaration/clause audit obligations, not a coverage verdict."""
import hashlib,json,pathlib,re,subprocess
D=pathlib.Path(__file__).resolve().parent
ROOT=D.parent.parent
def read(n):return json.loads((D/n).read_text())
def pin(n):return {'path':str(D/n),'sha256':hashlib.sha256((D/n).read_bytes()).hexdigest()}
def write(n,d):
 p=D/n;assert not p.exists();s=json.dumps(d,ensure_ascii=True,indent=2)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+'\n'.join('+'+x for x in s.split('\n')[:-1])+'\n*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True);print(json.dumps(pin(n)))

names=['p1339-mutant-closed-state-inventory-closure.json','p1339-mutant-closed-state-inventory-supplement.json','p1339-mutant-closed-state-inventory-resolution.json']
closure,supp,resolution=[read(n) for n in names]
decls={}
for name,data,key in [(names[0],closure,'declarations'),(names[1],supp,'declarations'),(names[2],resolution,'additional_qualified_declarations')]:
 for index,x in enumerate(data[key]):
  path=str((ROOT/x['path']).resolve()) if not x['path'].startswith('/') else x['path']
  uid=path+'::'+x['name']+'@'+str(x['first_line'])
  if uid not in decls:decls[uid]={**x,'path':path,'artifact':pin(name),'selector':f'{key}[{index}]','uid':uid}
  else:decls[uid].setdefault('alternate_nominal_sources',[]).append({'artifact':pin(name),'selector':f'{key}[{index}]'})

witnesses={
 'SW-closed-recursion':{
  'rule':'For this exact declaration and every listed variant/field unit, final verifier traces the actual private comparator branch and every child comparison; explicit exhaustive destructuring, no omitted fields/variant and no permissive wildcard.',
  'assertions':['Variant mismatch never Same','Each same-variant field is read and delegated to its complete typed witness; compare lengths/order/presence before children','Options distinguish None/Some(empty)/Some(explicit None); no fallback defaults or morphed Content','Result/SourceDiagnostic retains success/error, full ordered hints/traces and span fields','No public Value/Content PartialEq, Debug, repr, content hash or Func output is used as a generic substitute'],
  'required_candidate_evidence':['actual comparator entry used by context validator with callgraph','per-audit-unit exact path/line/symbol mapping','each delegated child helper and its complete nominal field coverage','compiler-exhaustive match/destructuring or equivalent complete source-backed field enumeration','real 71 relation cases plus public validator cases and scoped CLI conjunctions'],
 },
 'SW-ieee-leaf':{
  'rule':'Each f32/f64 leaf in this exact unit, including geometry/color/styling/relative wrappers, uses all IEEE bits for observation relation, preserving signed zero and reflexive identical NaN. No float leaf is hidden behind ordinary PartialEq.',
  'assertions':['same bits -> Same','different bits -> Different','public language equality/hash are not changed','wrapper variants and other data remain checked by SW-closed-recursion'],
  'required_candidate_evidence':['actual to_bits or equivalent bit-exact comparison per float leaf and callsite','relation-bits-same-nan,relation-bits-nan-payload,relation-bits-signed-zero actual tests','every float-containing transitive field mapped, not just Value::Float'],
 },
 'SW-immutable-identity':{
  'rule':'Fields retained inside the same immutable ordinary Func/Module/capture producer are accounted by causal whole-instance retention, not structural equality of newly recreated code/captures. Span and native/With metadata remain separately observable.',
  'assertions':['same retained immutable producer preserves all its fields and nested immutable objects','new ordinary closure is never Same solely because code/captures/repr match','Native conserves executable variant, fn pointer, name and namespace; With conserves function plus complete Args','Module identity includes its actual immutable inner object; fresh external CSL style equality is not assumed','no function is executed to decide relation'],
  'required_candidate_evidence':['actual owner immutability/creation/replacement paths for each listed field','F07 retained sibling and replaced capture/chain DTO evidence','private native/module/closure/With tests','all identity fast paths gated by retained causal generation, not mutable Arc address alone'],
 },
 'SW-resource-lifetime':{
  'rule':'This is an execution/resource/control carrier, not an immutable language value. Account each field by actual per-attempt construction, preservation, transactional isolation or explicit non-certification; do not compare it via pointer identity as a semantic leaf.',
  'assertions':['World,metrics,Location,target,features,producer,capture,entry chain remain logically same during valid reuse','effective request chain captured at each request, not final Engine.styles','route/AtomicUsize/constraint caches and sinks are never hidden observation storage or sameness proof','all mutation and lifecycle edges are source-accounted; no global registry/dyn service introduced','opaque or changed external service cannot certify positive reuse'],
  'required_candidate_evidence':['exact resource field def-use across eval and actual pipeline owners','typed lifecycle checker and same-context10/20/final30 case','SW-F08-shared-pagination-budget and SW-F09-sink-error-lineage','exact source-backed exclusions for legacy stub Engine and tracked macro control internals'],
 },
 'SW-opaque-service':{
  'rule':'Actual DynElement/PluginHost/ElementCtor service boundary has no unconditional structural equality proof. A mutable service pointer/token/name/dyn_eq is insufficient.',
  'assertions':['independently constructed real Dynamic callouts -> actual private Unproven','public same-callout metadata replacement never Ok(true)','no Unproven-to-language-warning path in result or sinks','Plugin host identity is not module u64 alone; optional plugin scope does not exclude a reached mandatory opaque value'],
  'required_candidate_evidence':['all actual opaque branches and causal provenance checks','real private and public opaque fixture-r2 plus all-channel predicate','no execution of service solely to infer equality'],
 },
 'SW-regex-derived-engine':{
  'rule':'Entity Regex has immutable language pattern and a derived compiled-engine/cache. Every listed external field is accounted through the derivation boundary, not recursively compared by Rust cache/Arc equality.',
  'assertions':['audit every constructor and clone/mutation path of entity Regex: compiled semantics derive from the exact retained pattern and fixed compiler options','audit every language observer: matching/repr/fields cannot observe mutable cache state or addresses','compare complete immutable language regex input(s); any unrecorded option/configuration invalidates this derivation witness','external Strategy/cache closures are not Typst Func and are not executed as an equality test','new same-pattern standard regex is supported and not automatically Unproven'],
  'required_candidate_evidence':['entity constructor and language-observer paths with exact source hashes/lines','external regex pattern/meta/RegexI/strat/info/pool boundary graph from resolution artifact','proof excluding every enumerated compiled/cache field from language observability without assuming Arc equality','actual relation-regex test and query/selector regex recursion callsites'],
 },
 'SW-decimal-closed':{
  'rule':'Decimal wrapper and external four-u32 representation are closed; account flags (sign/scale), hi, lo and mid without treating the type name as primitive proof.',
  'assertions':['all fields accounted directly or by a complete source-backed language-observable projection with normalization justified from constructor/observers','no hidden float/Func/Value field','same observation remains reflexive; supported Decimal never silently Unproven'],
  'required_candidate_evidence':['actual Decimal constructor/observer/equality normalization audit','actual comparator branch and all four representation fields or complete projection proof','no bare Eq derive used without this leaf audit'],
 },
 'SW-time-closed':{
  'rule':'Datetime None/Some date/time is distinguished; time::Date packed integer and time::Time bounded integers/one-variant padding are audited closed leaves in the pinned versions.',
  'assertions':['all datetime options and integer date/time values participate','both configured endian field layouts accounted','no hidden Value/Func/float behind leaf equality'],
  'required_candidate_evidence':['actual configured external declarations and leaf equality code paths','relation-datetime actual test','callsite from every containing field'],
 },
 'SW-uninhabited':{
  'rule':'Exact empty enum Covers has no constructible value in the audited configuration. Option<Covers> still distinguishes presence by exhaustive Rust matching; no unsafe fabricated Some or wildcard Same.',
  'assertions':['compiler-verified uninhabited declaration unchanged','every containing Option is explicitly accounted, not omitted'],
  'required_candidate_evidence':['actual type definition/configuration and containing field branches'],
 },
}
resource_names={'Engine','EvalContext','EvalTarget','FlowEvent','World','FontMetrics','Sink','Route','Validate','ImmutableConstraint','ConstraintEntry','EntryMap','Scopes','FxHashSet'}
identity_names={'Module','ModuleInner','ClosureRepr','ClosureParam','SyntaxNode','SyntaxText','LeafNode','InnerNode','ErrorNode','SyntaxError','SyntaxErrorKind','SyntaxKind','Capturer','IndependentStyle'}
opaque_names={'DynElement','PluginHost','ElementCtor','PluginFunc'}
for uid,x in decls.items():
 if x['name'] in resource_names:category='SW-resource-lifetime'
 elif x['name'] in opaque_names:category='SW-opaque-service'
 elif x['name'] in identity_names or (x['name']=='NodeKind' and 'syntax_node.rs' in x['path']):category='SW-immutable-identity'
 elif '/regex-' in x['path'] or '/regex-automata-' in x['path'] or x['path'].endswith('/entities/regex.rs'):category='SW-regex-derived-engine'
 elif x['name']=='Decimal':category='SW-decimal-closed'
 elif '/time-0.' in x['path'] or x['name']=='Datetime':category='SW-time-closed'
 elif x['name']=='Covers':category='SW-uninhabited'
 else:category='SW-closed-recursion'
 x['structural_witness']=category
 x['audit_units']=[]
 for row in x['declaration']:
  line=row['text'].strip()
  if line in ['{','}', '},',''];continue_dummy=False
  if line in ['{','}', '},','']:continue
  unit={'id':uid+':'+str(row['line']),'source_line':row['line'],'declaration_text':row['text'],
        'named_fields_or_parameters':re.findall(r'\b([A-Za-z_]\w*)\s*:(?!:)',row['text']),
        'witnesses':[category],
        'assertion':'Every field/variant/tuple position explicitly present on this exact declaration line must have the required candidate evidence; a source line containing several fields maps ALL of them, not one example.'}
  if re.search(r'\bf(?:32|64)\b',line) and category=='SW-closed-recursion':unit['witnesses'].append('SW-ieee-leaf')
  x['audit_units'].append(unit)
 x['phase']={'freeze':'C','verify':'F','preseal_runtime_credit':0,'status':'REQUIRED_WITNESS_NOT_YET_PROVEN'}

# Named new carrier units come from L0, never inferred from candidate source.
new=[
 {'id':'L0.Selector.Element.function','l0':'00_nucleo/prompts/entities/selector.md','line':268,'declaration':'function: Func','witnesses':['SW-closed-recursion','SW-immutable-identity'],'tests':['relation-selector-retained','relation-selector-field-value','relation-selector-empty-bare']},
 {'id':'L0.Selector.Element.fields','l0':'00_nucleo/prompts/entities/selector.md','line':269,'declaration':'fields: EcoVec<(EcoString,Value)>','witnesses':['SW-closed-recursion'],'tests':['relation-selector-field-order','relation-selector-field-type','relation-selector-field-value']},
 {'id':'L0.ShowSelector.NativeElement','l0':'00_nucleo/prompts/entities/show.md','line':217,'declaration':'NativeElement(Func)','witnesses':['SW-closed-recursion','SW-immutable-identity'],'tests':'all18 selector show-witness CLI plus SW-F01-carrier-routing'},
 {'id':'L0.ElementPayload.NativeElement','l0':'00_nucleo/prompts/entities/element_payload.md','line':324,'declaration':'NativeElement unit, no new ElementKind','witnesses':['SW-closed-recursion'],'tests':'W02 filtered occurrence CLI plus SW-F02-occurrence-tags'},
]

extra={
 'SW-F01-carrier-routing':{
  'required_evidence':['Actual Selector::Element field declarations exactly match L0 function/orderedfields','All actual where construction, clone, and/or composition branches preserve real variant, function and complete fields','ShowSelector conversion uses NativeElement(Func) preserving identity; every affected exhaustive match listed','ElementPayload::NativeElement is unit and ElementKind baseline enum is unchanged'],
  'assertions':['No name/AST/dyn fallback as identity','No empty Element collapsed to bare Kind','All baseline variants/fields preserved','Real compiled private Selector pairs and18 public show-witnesses required conjunctively'],
 },
 'SW-F02-occurrence-tags':{
  'required_evidence':['Exact original Content traversal and layout traversal paths for Strong/Emph, including empty body','Every call allocating Location and entering/leaving tags for those variants, plus parent/order edges','Exact Tag::End content and snapshot Some field extraction including body,label,default delta300','All Styled/render-only wrapper paths showing no spurious NativeElement own occurrence','Full actual public filtered occurrence cases query ordered carriers, not raw count alone'],
  'assertions':['One own Location per actual Strong/Emph occurrence including empty/reused/nested, no content-hash alias','Introspection/layout follow same allocation traversal and align Tag boundaries','Some complete fields never use fallback; None preserves legacy path','Unsupported explicit/set producers preserve their isolated baseline controls'],
 },
 'SW-F03-counter-event-fold':{
  'required_evidence':['Actual ordered CounterUpdate Set/Step/Func producers and complete metadata fields; all event constructors/callsites enumerated','Actual demand-time complete-log fold helper and prefix slicing def-use','Isolated callback evaluation context, lexical resources and original SourceDiagnostic traces','Actual lifecycle fresh_callback and full public runtime interleaved/prefix-before-error/undemanded callback cases'],
  'assertions':['Fold complete demanded log before returning prefix','No eager callback execution for undemanded key','Language key equality remains distinct from observation relation and nonreflexive forNaN','No duplicate old-generation update on retry','Static and bound calls reach same real owner'],
 },
 'SW-F10-ownership-purity':{
  'required_evidence':['All31 L0-to-consumer1:1 ownership/lineage pins','Exact compiled callgraph for dispatch/semantic owners, all new branches','Strict crystalline-lint zero violations, full workspace build/tests/fmt/diff checks','Global4718x4 normal/repeat/reverse baseline-candidate preservation handled by root, with pinned raw receipts'],
  'assertions':['L1 has no I/O/global mutable state/unauthorized registry or dyn extension','No business formula copied into glue or infra','Only approved3 public APIs and approved carriers changed; no new module','No protected oracle/baseline/contract mutation'],
 },
}

artifact_names=['p1339-ab-batch1-cli-oracles.json','p1339-ab-batch1-cli-plan.json','p1339-ab-private-relation-fixtures-r1.json','p1339-ab-closed-api-fixtures-r1.json','p1339-ab-batch1-projection-fixtures-r2.json','p1339-ab-batch1-public-opaque-fixture-r2.json','p1339-ab-batch1-retention-lifecycle-typed-r2.json','p1339-ab-batch1-same-context-style-fixture.json']
artifacts={n:pin(n) for n in artifact_names}
contract=read('p1339-contract-r3.json')
matrix_refs={
 'F01-carrier':{'fixtures':['relation-selector-field-order','relation-selector-field-type','relation-selector-empty-bare','relation-located-none-some'],'witnesses':['SW-F01-carrier-routing']},
 'F02-occurrence':{'fixtures':['all13 w02 derived equal/different CLI pairs and original14baseline ancestors'],'witnesses':['SW-F02-occurrence-tags']},
 'F03-counter':{'fixtures':['where-counter-runtime-* CLI','lifecycle-fresh_callback_stable','lifecycle-increasing_set'],'witnesses':['SW-F03-counter-event-fold']},
 'F04-selection':{'fixtures':['api-new-unselected','api-construct-not-demand','api-update-not-demand','api-unreached-demand','api-empty-filtered-demand','api-demand-before-label-error','api-state-legacy-before','lifecycle-legacy-sibling-error'],'witnesses':['SW-resource-lifetime']},
 'F05-projections':{'fixtures':['all26 original closed API cases','all8 additional projection cases','api-same-record-request-chains-10-20-final30'],'witnesses':['SW-closed-recursion','SW-resource-lifetime']},
 'F06-closed-comparator':{'fixtures':['all71 real private relation cases','api-query-independent-dynamic-noncertification'],'witnesses':list(witnesses)},
 'F07-retention':{'fixtures':['lifecycle-retained-sibling-growth','lifecycle-replaced-capture-descendant','lifecycle-replaced-chain-descendant','api-same-record-request-chains-10-20-final30'],'witnesses':['SW-immutable-identity','SW-resource-lifetime']},
 'F08-attempts':{'fixtures':['lifecycle-increasing_set','lifecycle-oscillating_set','lifecycle-nested_producer','lifecycle-provisional-assert-final-panic'],'witnesses':['SW-F08-shared-pagination-budget']},
 'F09-errors-sinks':{'fixtures':['lifecycle-provisional-assert-final-panic','lifecycle-selected-stable-error','lifecycle-legacy-sibling-error','api-state-display-value-before-callback-error','api-query-independent-dynamic-noncertification','all public full diagnostic CLI predicates'],'witnesses':['SW-F09-sink-error-lineage','SW-opaque-service']},
 'F10-architecture':{'fixtures':['independent20mutants including structuralM12 and mandatory_unknownM20','full root workspace gates and global preservation'],'witnesses':['SW-F10-ownership-purity']},
}
clause_map=[]
for m in contract['final_internal_matrices']:
 for i,clause in enumerate(m['requires']):
  clause_map.append({'matrix_id':m['id'],'clause_index':i,'clause':clause,'obligations':m['obligations'],**matrix_refs[m['id']], 'phase':'F','preseal_state':'NOT_EXECUTED_PRESEAL','preseal_credit':0,'mandatory':True,'closure':'All listed concrete evidence plus exact per-clause structural witness; no runtime credit from merely publishing this map.'})
life=read('p1339-ab-batch1-retention-lifecycle-typed-r2.json')
prose_map=[]
for c in life['cases']:
 for i,p in enumerate(c['predicates']):
  prose_map.append({'case_id':c['id'],'predicate_index':i,'original_predicate':p,
   'typed_assertions':{'artifact':life['predicate_program'],'selector':'check(case, observed) plus predecessor.check(case, observed)','case_expected_selector':f'cases[id={c["id"]}].typed_predicates'},
   'conjunctive_structural_witnesses':[x for x in ['SW-immutable-identity','SW-resource-lifetime','SW-F03-counter-event-fold','SW-F08-shared-pagination-budget','SW-F09-sink-error-lineage'] if (x!='SW-F03-counter-event-fold' or c['id'] in ['lifecycle-fresh_callback_stable','lifecycle-nan_key_stable_zero','lifecycle-nested_producer'])],
   'no_discharge_by_prose':'Verifier must map each stated proposition to exact checker assertion line or actual per-case structural proof before seal; unspecified or merely generic claims remain missing, not accepted.'})

payloads=[]
for row in resolution['direct_payload_map']:
 r=dict(row);r['witness']='SW-opaque-service' if (r['owner'],r['variant'])==('Content','Dynamic') else 'SW-closed-recursion'
 if r['owner']=='Value' and r['variant'] in ['Func','Module']:r['additional_witness']='SW-immutable-identity'
 if r['owner']=='Value' and r['variant']=='Regex':r['additional_witness']='SW-regex-derived-engine'
 r['phase']='F';r['status']='REQUIRED_NOT_PROVEN';payloads.append(r)
require_counts={x:sum(r['owner']==x for r in payloads) for x in ['Value','Content']}
assert require_counts=={'Value':38,'Content':98}
write('p1339-ab-batch1-coverage-map.json',{
 'schema':'p1339-independent-field-clause-map-v1','author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento','batch':1,
 'status':'Candidate map for structural review before aggregate manifest; NOT proof of completed coverage',
 'authority_manifest':pin('p1339-authority-manifest-r2.json'),'contract':pin('p1339-contract-r3.json'),'l0_freeze':pin('p1339-l0-freeze.json'),'budget':pin('p1339-budget-redesign-r1.json'),
 'inventories':{n:pin(n) for n in names},'fixture_artifacts':artifacts,'direct_variant_counts':require_counts,
 'direct_variant_map':payloads,'declaration_field_units':list(decls.values()),'approved_new_carrier_units':new,
 'structural_witness_definitions':{**witnesses,**extra,**{x['id']:x for x in life['structural_witnesses']}},
 'F_clause_map':clause_map,'original_lifecycle_predicate_map':prose_map,
 'W_obligation_map':[{ 'id':'W'+str(i).zfill(2),'F_clauses':[{'matrix':r['matrix_id'],'index':r['clause_index']} for r in clause_map if 'W'+str(i).zfill(2) in r['obligations']], 'CLI_oracle_selector':f'oracles with obligation_ids containing W{i:02d}', 'mandatory':True} for i in range(1,11)],
 'homonym_resolution':supp['qualified_ambiguities'],'aliases':resolution['qualified_aliases'],
 'phase_ledger':{'C_existing_public':'actual pinned vanilla/baseline CLI and adversary20mutants after manifest; no future runtime credit','F_closed_and_architecture':'all units/clauses mandatory real execution and source proof, no NotDue/Unknown waiver'},
 'closure_gate':['All exact fields and variants must have nonempty accepted source-backed evidence; uncovered unit blocks seal/final as appropriate','No generic field wildcard grants success; declarations are audit targets, not expected Same','Missing actual supported relation category is Unproven and blocks mandatory case, never excluded','Full lifecycle144 and style12 raw cells plus exact core API/private case matrices required','Independent verifier decides witness sufficiency, not oracle author or adapter'],
})
