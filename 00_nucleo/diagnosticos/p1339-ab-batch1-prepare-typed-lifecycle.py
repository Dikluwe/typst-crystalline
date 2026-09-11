"""Type the batch-one prospective observation contract; no product run."""
import copy, hashlib, json, pathlib, re, struct, subprocess
D=pathlib.Path(__file__).resolve().parent
def pin(n):
 p=D/n;return {'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
def write(n,d):
 p=D/n;assert not p.exists()
 s=json.dumps(d,ensure_ascii=True,indent=2)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+'\n'.join('+'+x for x in s.split('\n')[:-1])+'\n*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True);print(json.dumps(pin(n)))
old=json.loads((D/'p1339-ab-batch1-retention-lifecycle-fixtures.json').read_text())
d=copy.deepcopy(old)
d['predecessor']=pin('p1339-ab-batch1-retention-lifecycle-fixtures.json')
d['schema']='p1339-independent-retention-lifecycle-fixtures-v2'
d['predicate_program']=pin('p1339-ab-batch1-lifecycle-predicate.py')
d['predicate_precedence']='The typed predicates, checker program and structural-witness assertions below are executable/structural obligations. Earlier prose remains rationale, not a license for adapter-selected expected outcomes.'
d['review_input']=pin('p1339-verifier-batch1-component-review-r1.md')
d['dto_schema']={
 'format':'JSON objects, exact required keys and types; additional real provenance keys allowed, missing/unknown keys never silently defaulted',
 'types':{
  'Identity':'nonempty string backed by actual immutable generation/object identity or audited reference equality; never a fixture-assigned success token',
  'OptionalIdentity':'Identity | null',
  'Attempt':'integer 0..5, 0 only denotes actual ordinary discovery',
  'Span':'{file_id: string, source_sha256: 64-hex-string, byte_start: nonnegative integer, byte_end: nonnegative integer, raw_span: string}',
  'Diagnostic':'{severity: Error|Warning, span: Span, message: string, hints: [string], trace: [{kind: Call|Show|Import|Include, payload: string|null, span: Span}]}',
  'Result':'{kind: Ok, value: typed JSON value} | {kind: Err, diagnostics: [Diagnostic]}',
  'CounterValue':'[nonnegative integer]',
  'MetadataValue':'integer | string | boolean | null | [MetadataValue] | {Func: Identity} | {Length: {abs_pt_bits: 16-hex-string, em_bits:16-hex-string}}; other runtime variants must be reported as an explicit binding gap, not stringify/repr',
  'Resources':'{producer_id:Identity,capture_id:Identity,context_location:Identity,entry_chain_id:Identity,world_id:Identity,metrics_id:Identity,target:paged,features:[html|a11y-extras]}',
  'CounterEvent':'{key_id:string,action:{kind:Set,values:CounterValue}|{kind:Step,level:positive integer}|{kind:Func,func_id:Identity},location:Identity,span:Span,producer_generation_id:Identity}',
  'RequestReplay':'{request_id:Identity,chain_id:Identity,resources:Resources,operation:string,typed_arguments:JSON,result:Result}',
 },
 'event_common':{'sequence':'consecutive nonnegative integer, actual hook order','kind':'one of exact event variants below'},
 'event_variants':{
  'DiscoveryCompleted':{'source_context_offset':'nonnegative byte offset','generation_id':'Identity','selected':'boolean','result':'Result','output_id':'OptionalIdentity','sink_id':'Identity'},
  'AttemptStarted':{'attempt':'Attempt >=1','read_snapshot_id':'Identity','observational_stores_empty':'boolean','counter_values':'object key_id -> CounterValue'},
  'BodyStarted':{'attempt':'Attempt','source_context_offset':'nonnegative byte offset','execution_id':'Identity','generation_id':'Identity','parent_generation_id':'OptionalIdentity','resources':'Resources'},
  'RequestRecorded':{'execution_id':'Identity','generation_id':'Identity','request_id':'Identity','operation':'string from actual closed operation variant','typed_arguments':'lossless typed JSON','span':'Span','chain_id':'Identity','resources':'Resources','result':'Result'},
  'BodyCompleted':{'attempt':'Attempt','execution_id':'Identity','generation_id':'Identity','result':'Result','output_id':'OptionalIdentity','func_ids':'[Identity]','sink_id':'Identity','final_chain_id':'Identity'},
  'CandidateBuilt':{'attempt':'Attempt >=1','document_id':'Identity','content_id':'Identity','snapshot_id':'Identity','positions_complete':'boolean','page_store_complete':'boolean','counter_values':'object key_id -> CounterValue','metadata_values':'[MetadataValue]','counter_events':'[CounterEvent]','current_generation_ids':'[Identity]'},
  'ValidationCompleted':{'attempt':'Attempt >=1','generation_id':'Identity','candidate_snapshot_id':'Identity','result':'Result whose Ok value is boolean','replayed_requests':'[RequestReplay]','sink_id':'Identity'},
  'ContributionRetained':{'attempt':'Attempt >=1','generation_id':'Identity','original_execution_id':'Identity','output_id':'OptionalIdentity','func_ids':'[Identity]','sink_id':'Identity','resources':'Resources','request_ids':'[Identity]'},
  'GenerationInvalidated':{'attempt':'Attempt','generation_id':'Identity','reason':'actual closed reason as string','descendant_ids':'[Identity]','actual_descendant_ids_before_replacement':'[Identity] observed from pre-replacement real traversal, not copied from expected list'},
  'SinkDisposition':{'sink_id':'Identity','origin_generation_id':'OptionalIdentity','origin_phase':'validation|discovery_selected|discovery_legacy|selected_body|final_nonconvergence','disposition':'published|discarded','diagnostics':'[Diagnostic]'},
  'FunctionInvoked':{'func_id':'Identity','origin':'body|counter_fold|display_callback|projection|comparator_category_test|other_actual_origin'},
  'CompilationReturned':{'attempt':'Attempt','document_id':'OptionalIdentity','result':'Result','metadata_values':'[MetadataValue]','current_output_ids':'[Identity]','current_func_ids':'[Identity]','published_sink_ids':'[Identity]','export_permitted':'boolean'},
 },
 'observation_envelope':{'case_id':'exact frozen id','execution':'Observed|Unknown','binding_audit':'accepted_real_product_path only after independent verifier audit; otherwise explicit pending/rejected','events':'[event_variant]','binary_sha256':'64 hex','compiled_configuration':'object','port_source_pins':'array of exact actual productive binding paths and hashes'},
 'identity_caveat':'Pointer addresses may serve only where verifier proves immutable causal lifetime and generation provenance. Mutable world/service pointer alone never proves sameness.',
}

def length(n):return {'Length':{'abs_pt_bits':struct.pack('>d',float(n)).hex(),'em_bits':'0000000000000000'}}
for c in d['cases']:
 key=c['id'].removeprefix('lifecycle-')
 c['context_coordinates']=[{'index':i,'byte_offset':len(c['source'][:m.start()].encode()),'token':'#context'} for i,m in enumerate(re.finditer(r'#context\b',c['source']))]
 c['explicit_projections']={
  'contexts':'Only actual context generations whose Source spans originate at the listed context_coordinates, including their real descendant edges.',
  'metadata':'Actual current MetadataStore entries belonging to this source, in document order. No metadata assertion wrapper or added context is used.',
  'counter_keys':{'c':'heading.where()'},
  'projection_call_context':'Separate fresh EvalContext/Engine for observation only, same actual snapshot/resources and complete counter owner. Do not register observation-probe calls in product read records or publish their sinks. Probe counter keys are declared below, never all keys.',
  'events':'All listed DTO transitions for this source across ordinary discovery, actual selected attempts and validations, with no speculative extra transitions.',
 }
 t={}
 if key in ['increasing_set','oscillating_set']:
  reads=[0,1,2,3,4] if key=='increasing_set' else [0,1,0,1,0]
  produced=[1,2,3,4,5] if key=='increasing_set' else [1,0,1,0,1]
  t={'attempts':[1,2,3,4,5],'counter_key_id':'c','counter_values_read':[[v] for v in reads],'counter_values_produced':[[v] for v in produced],'metadata_by_attempt':[[[v]] for v in reads],'final_metadata':[[reads[-1]]],'manual_set_events_per_snapshot':1}
 elif key=='retained-sibling-growth':
  c['explicit_projections']['counter_keys']={'stable':'heading.where(level: 9)','changing':'heading.where(level: 1)'}
  t={'attempts':[1,2,3,4,5],'counter_key_id':'changing','counter_values_read':[[v] for v in range(5)],'counter_values_produced':[[v] for v in range(1,6)],'retained_context_index':0,'manual_set_events_per_snapshot':1}
 elif key in ['replaced-capture-descendant','replaced-chain-descendant']:
  c['explicit_projections']['counter_keys']={'stable':'heading.where(level: 9)','changing':'heading.where(level: 1)'}
  vals=list(range(5)) if key=='replaced-capture-descendant' else [length(v) for v in range(10,15)]
  t={'attempts':[1,2,3,4,5],'counter_key_id':'changing','counter_values_read':[[v] for v in range(5)],'counter_values_produced':[[v] for v in range(1,6)],'metadata_by_attempt':[[v] for v in vals],'final_metadata':[vals[-1]],'replaced_child_context_index':1,'manual_set_events_per_snapshot':1}
 elif key=='request-chain-points':t={'final_metadata':[length(10),length(20)]}
 elif key in ['nested_producer','fresh_callback_stable']:t={'final_metadata':[[12]]}
 elif key=='nan_key_stable_zero':
  c['explicit_projections']['counter_keys']={'c':'heading.where(level: float.nan)'}
  t={'attempts':[1],'final_metadata':[[0]]}
 elif key=='provisional-assert-final-panic':t={'final_error_message':'panicked with: P1339_FINAL_PANIC','forbidden_final_messages':['assertion failed']}
 elif key=='legacy-sibling-error':t={'final_error_message':'panicked with: P1339_LEGACY_SIBLING','legacy_error_unselected':True}
 elif key=='selected-stable-error':t={'final_error_message':'label `<missing>` does not exist in the document','stable_error_validation':True}
 else:raise AssertionError(key)
 c['typed_predicates']=t
 c['prose_status']='Retained predecessor rationale. Machine checks use typed_predicates + pinned predicate program; additional structural obligations are explicitly enumerated in structural_witnesses.'

d['structural_witnesses']=[
 {'id':'SW-F08-shared-pagination-budget','phase':'F','mandatory':True,
  'owners':['03_infra/src/pipeline.rs','01_core/src/compiler/eval/mod.rs'],
  'input':'Actual candidate source and compiled paginated callgraph, frozen L0 infra/pipeline.md paragraphs naming Ak/Ik and shared relayout ceiling.',
  'required_evidence':{
   'budget_storage':'Exact path/line/symbol for the single selected-attempt budget/counter and its owner lifetime.',
   'initializations':'Exhaustive source-backed list of writes initializing/reinitializing that budget; each with path/line and containing function.',
   'retry_edges':'Exhaustive edges that retry after contextual invalidation, page count, PageStore, positions, or nested discovery; exact caller/callee/callsite and budget passed/read/written.',
   'cycles':'Enumerate every actual callgraph cycle/SCC reachable from paginated compile through those retry edges; list storage identity read/updated by each.',
   'return_dataflow':'Exact def-use chain from Dk created with I(k-1) to returned document; final history I0..I5; no I5 projection/replay output exported.',
  },
  'assertions':[
   {'op':'every','subject':'initializations','predicate':'dominates first A1 and is outside all retry cycles; no reset on layout/position/nested edge'},
   {'op':'every','subject':'retry_edges','predicate':'selected semantic attempt budget storage is identical across contextual and pagination transitions'},
   {'op':'every','subject':'cycles','predicate':'no path can complete A6 or bypass the shared ceiling by recursively starting another private five-attempt loop'},
   {'op':'equals','subject':'at_ceiling.return_document_def','expected':'D5 created reading I4'},
   {'op':'equals','subject':'at_ceiling.diagnostic_snapshot_order','expected':['I0','I1','I2','I3','I4','I5']},
  ],'failure_policy':'Missing edge/initialization or unsupported proof is a mandatory final failure, not a prose coverage claim. The actual increasing/oscillating DTO tests are conjunctive; this witness is not inferred from their page count.'},
 {'id':'SW-F09-sink-error-lineage','phase':'F','mandatory':True,'owners':['03_infra/src/pipeline.rs','01_core/src/compiler/eval/mod.rs'],
  'required_evidence':'Exhaustive actual sink creation, replay sink disposal, replaced-generation disposal, retained publication and Err-to-return paths with def-use for complete SourceDiagnostic vectors.',
  'assertions':['No discarded validation or invalidated-generation sink reaches exported publication','Each retained sink publication edge is reached once','Pending final Err dominates export rejection without sorting or textual reclassification','No Unproven branch constructs vanilla nonconvergence warning'],
  'conjunctive_runtime':'All twelve actual DTO scenarios and full public diagnostic oracles; missing real transitions fail.'},
]
write('p1339-ab-batch1-retention-lifecycle-typed.json',d)

style={k:copy.deepcopy(v) for k,v in old.items() if k not in ['cases','dto_schema','global_predicates','translation_limits']}
style['schema']='p1339-same-context-request-chain-fixture-v1'
style['style_signatures']=pin('p1339-mutant-closed-state-style-signatures.md')
style['id']='api-same-record-request-chains-10-20-final30'
style['matrix_ids']=['F05-projections','F07-retention','F09-errors-sinks']
style['setup']={'snapshot':'TagIntrospector::empty()','current_location':'one Locator::next retained throughout','ctx':'one actual EvalContext::new, in_context=true,target=paged,profile features','engine':'one Engine, immutable memory World, FixedMetrics, default chain, distinct product and validation sinks','setup_code':'let c = counter(heading.where())'}
style['operations']=[
 {'op':'set_engine_chain','chain_id':'chain10','construction':'default_chain.push(StyleDelta { size: Some(10.0), ..StyleDelta::empty() })'},
 {'op':'eval_expr','source_id':'request10.typ','source':'c.final()','same_ctx':True,'expected_result':{'kind':'Ok','value':[0]}},
 {'op':'set_engine_chain','chain_id':'chain20','construction':'default_chain.push(StyleDelta { size: Some(20.0), ..StyleDelta::empty() })'},
 {'op':'eval_expr','source_id':'request20.typ','source':'c.final()','same_ctx':True,'expected_result':{'kind':'Ok','value':[0]}},
 {'op':'set_engine_chain','chain_id':'chain30','construction':'default_chain.push(StyleDelta { size: Some(30.0), ..StyleDelta::empty() })'},
 {'op':'call_actual_has_filtered_counter_reads','expected':True},
 {'op':'call_actual_context_reads_valid_for','candidate':'clone of same actual empty snapshot','same_ctx':True,'expected_result':{'kind':'Ok','value':True}},
]
style['dto']={'recorded_requests':'ordered [{actual_request_id:Identity,actual_ctx_id:Identity,source_id:string,chain_identity:Identity,chain_size_pt_bits:hex16,operation:CounterFinal,result:{kind:Ok,value:[0]}}]',
 'replayed_requests':'ordered [{actual_request_id:Identity,actual_ctx_id:Identity,chain_identity:Identity,chain_size_pt_bits:hex16,operation:CounterFinal,result:{kind:Ok,value:[0]}}]',
 'engine_chain_at_validation':'{chain_identity:Identity,chain_size_pt_bits:hex16}','selection':'boolean','validation':'Result','validation_sink_publications':'integer'}
style['typed_predicates']=[
 {'op':'length_equal','path':'recorded_requests','expected':2},
 {'op':'length_equal','path':'replayed_requests','expected':2},
 {'op':'project_equal','path':'recorded_requests','field':'source_id','expected':['request10.typ','request20.typ']},
 {'op':'project_equal','path':'recorded_requests','field':'chain_size_pt_bits','expected':[struct.pack('>d',10.0).hex(),struct.pack('>d',20.0).hex()]},
 {'op':'project_equal','path':'replayed_requests','field':'chain_size_pt_bits','expected':[struct.pack('>d',10.0).hex(),struct.pack('>d',20.0).hex()]},
 {'op':'pointwise_identity_equal','left':'recorded_requests','right':'replayed_requests','fields':['actual_request_id','actual_ctx_id','chain_identity']},
 {'op':'all_same_identity','paths':['recorded_requests[*].actual_ctx_id','replayed_requests[*].actual_ctx_id']},
 {'op':'equal','path':'engine_chain_at_validation.chain_size_pt_bits','expected':struct.pack('>d',30.0).hex()},
 {'op':'equal','path':'selection','expected':True},
 {'op':'equal','path':'validation','expected':{'kind':'Ok','value':True}},
 {'op':'equal','path':'validation_sink_publications','expected':0},
]
style['limits']='The existing direct eval_expr fixture sequence constructs inputs to one real record; it does not emulate a pipeline cycle. Hooks observe actual request capture/replay. They cannot fill chains from fixture labels or final Engine.styles.'
write('p1339-ab-batch1-same-context-style-fixture.json',style)
