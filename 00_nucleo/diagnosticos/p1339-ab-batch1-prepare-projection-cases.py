"""Close declared F05 projection cases using published real carrier signatures."""
import copy,hashlib,json,pathlib,subprocess
D=pathlib.Path(__file__).resolve().parent
def pin(n):return {'path':str(D/n),'sha256':hashlib.sha256((D/n).read_bytes()).hexdigest()}
old=json.loads((D/'p1339-ab-closed-api-fixtures-r1.json').read_text())
base=next(x for x in old['cases'] if x['id']=='api-empty-filtered-demand')
cases=[]
def case(id,body,snaps,expected=None):
 c=copy.deepcopy(base);c.update(id=id,body_code=body,snapshots=snaps,body_snapshot='base',validate_candidate_ids=[x['id'] for x in snaps if x['id']!='base'],history_ids=['base','same'],value_bindings=[])
 c['matrix_ids']=['F05-projections','F09-errors-sinks']
 c['predicates']={'body':expected or {'result':'Ok'},'has_filtered_counter_reads':True,'validation':{x['id']:{'result':'Ok','value':x['id']=='same'} for x in snaps if x['id']!='base'},'nonconvergence_diagnostics':{'result':'Ok','length':0}}
 cases.append(c);return c
empty=[{'id':'base','origin':'empty'},{'id':'same','origin':'clone_of','clone_of':'base'}, {'id':'changed','origin':'clone_of','clone_of':'base','overlays':[{'kind':'state_init','key':'s','expression':'1','location':'entry'}]}]
case('api-state-method-init-fallback','{ let v = state("s", 7).final(); counter(heading.where()).final(); v }',copy.deepcopy(empty),{'result':'Ok','value_type':'Int','value':7})
case('api-state-global-none-fallback','{ let v = state_final("s"); counter(heading.where()).final(); v }',copy.deepcopy(empty),{'result':'Ok','value_type':'None'})
c=case('api-state-display-value-before-callback-error','{ state("s", 7).display(x => { counter(heading.where()).final(); panic("P1339_DISPLAY_BODY") }) }',copy.deepcopy(empty),{'result':'Err','message':'panicked with: P1339_DISPLAY_BODY','span_required':True})
c['actual_effect_predicates']={'display_callback_invocations_during_body':1,'display_callback_invocations_during_validation':0,'display_callback_invocations_during_diagnostics':0,'recorded_operations_in_order':['state_display_value','counter_final'],'counter_request_selector_contains_element':True,'retained_body_error_not_replaced_by_validation_success':True}
c['history_ids']=['base','same']
snaps=[{'id':'base','origin':'empty'},{'id':'same','origin':'clone_of','clone_of':'base'},{'id':'resolved','origin':'clone_of','clone_of':'base','overlays':[{'kind':'label','label':'<missing>','location':'entry'}]}]
case('api-state-at-missing-label-resolution','{ counter(heading.where()).final(); state("s", 7).at(<missing>) }',snaps,{'result':'Err','message':'label `<missing>` does not exist in the document','span_required':True})
snaps=[{'id':'base','origin':'content_expression','content_expression':'[#metadata(1)<node>]','introspection_entrypoint':'pure'},{'id':'same','origin':'clone_of','clone_of':'base'},{'id':'changed_content_same_location','origin':'clone_of','clone_of':'base','overlays':[{'kind':'element','location':'label:node','content_expression':'metadata(2)','fields':[['value','2'],['label','<node>']]}]},{'id':'none','origin':'empty'}]
c=case('api-locate-first-not-full-carrier','{ let v = locate(metadata); counter(heading.where()).final(); v }',snaps,{'result':'Ok','value_type':'Location'})
c['predicates']['validation']['changed_content_same_location']['value']=True
c=case('api-locate-none-to-first','{ let v = locate(metadata); counter(heading.where()).final(); v }',[{'id':'base','origin':'empty'},{'id':'same','origin':'clone_of','clone_of':'base'},{'id':'first','origin':'content_expression','content_expression':'[#metadata(1)<node>]','introspection_entrypoint':'pure'}],{'result':'Ok','value_type':'None'})
c=case('api-here-location-only','{ let v = here(); counter(heading.where()).final(); v }',[{'id':'base','origin':'empty'},{'id':'same','origin':'clone_of','clone_of':'base'},{'id':'changed_positions','origin':'clone_of','clone_of':'base','overlays':[{'kind':'position','location':'current','page':2,'x_pt':50,'y_pt':60},{'kind':'pages','total_pages':2}]}],{'result':'Ok','value_type':'Location'})
c['predicates']['validation']['changed_positions']['value']=True
snaps=[{'id':'base','origin':'empty','overlays':[{'kind':'position','location':'current','page':1,'x_pt':0,'y_pt':0},{'kind':'pages_raw','total_pages':1,'numberings':[{'kind':'Func','binding':'f'}],'supplements':['[]']}]},{'id':'same','origin':'clone_of','clone_of':'base'},{'id':'new_function','origin':'clone_of','clone_of':'base','overlays':[{'kind':'pages_raw','total_pages':1,'numberings':[{'kind':'Func','binding':'g'}],'supplements':['[]']}]},{'id':'pattern','origin':'clone_of','clone_of':'base','overlays':[{'kind':'pages_raw','total_pages':1,'numberings':[{'kind':'Pattern','value':'1'}],'supplements':['[]']}]},{'id':'none','origin':'clone_of','clone_of':'base','overlays':[{'kind':'pages_raw','total_pages':1,'numberings':[None],'supplements':['[]']}]}]
c=case('api-page-numbering-raw-function','{ counter(heading.where()).final(); here().page-numbering() }',snaps,{'result':'Ok','value_type':'Func'})
c['value_bindings']=[{'name':'f','expression':'n => panic("P1339_NUMBERING_NOT_CALLED")'},{'name':'g','expression':'n => panic("P1339_NUMBERING_NOT_CALLED")'}]
c['actual_effect_predicates']={'numbering_callback_invocations_body':0,'numbering_callback_invocations_validation':0,'numbering_callback_invocations_diagnostics':0,'same_binding_returned_from_body':'f'}
c['overlay_translation']='pages_raw calls existing PageStore::from_runtime(NonZeroUsize, Vec<Option<Numbering>>, Vec<Content>) and inject_pages. Func uses existing Value binding without re-evaluation; no new public API.'
d={k:v for k,v in old.items() if k!='cases'}
d.update(schema='p1339-batch1-additional-projection-fixtures-v1',batch=1,predecessor=pin('p1339-ab-closed-api-fixtures-r1.json'),contract=pin('p1339-contract-r3.json'),budget=pin('p1339-budget-redesign-r1.json'),cases=cases,status='Same aggregated batch1 component; no product process; future APIs NOT_EXECUTED_PRESEAL')
d['additional_effect_port']='Passive hooks at actual display/numbering callback invocation and RequestRecorded boundaries. Typed counts above are mandatory; callback must not be invoked merely to compare/probe its category.'
p=D/'p1339-ab-batch1-projection-fixtures.json';assert not p.exists();s=json.dumps(d,ensure_ascii=True,indent=2)+'\n'
patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+'\n'.join('+'+x for x in s.split('\n')[:-1])+'\n*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
print(json.dumps(pin(p.name)))
