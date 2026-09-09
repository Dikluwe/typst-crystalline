// Role C. Read frozen bilateral data; emit fresh diagnostic artifacts through apply_patch.
const fs=require('fs'),crypto=require('crypto'),cp=require('child_process');
const b='00_nucleo/diagnosticos/',root=process.cwd();
const read=p=>fs.readFileSync(p,'utf8'), sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const json=n=>JSON.parse(read(b+n)), pin=n=>({path:b+n,sha256:sha(b+n)});
const tab=p=>{const lines=read(p).trimEnd().split('\n'),ks=lines.shift().split('\t');return lines.map(l=>Object.fromEntries(l.split('\t').map((v,i)=>[ks[i],v])));};
const allmatch=xs=>xs.every(x=>x.startsWith('MATCH_'));
const key=r=>r.id+'\u0000'+r.profile;
const group=xs=>{const m=new Map();for(const r of xs){if(!m.has(r.id))m.set(r.id,[]);m.get(r.id).push(r);}return m;};
const count=xs=>xs.reduce((a,x)=>(a[x]=(a[x]||0)+1,a),{});
const same=(a,z)=>JSON.stringify(a)===JSON.stringify(z);
const stable=r=>({runtime_class:r.runtime_class,...Object.fromEntries(['vanilla','crystalline'].map(s=>[s,Object.fromEntries(['exit_code','stdout_base64','stderr_base64','complete','reason_code','source_sha256','binary_sha256','features'].map(k=>[k,r[s][k]]))]))});
const classify=(v,c)=>!v.complete||!c.complete?'EXECUTION_UNKNOWN':v.exit_code===0&&c.exit_code===0?(v.stdout_base64!==c.stdout_base64?'DIFFERENT_VALUE':v.stderr_base64===c.stderr_base64?'MATCH_VALUE':'DIFFERENT_DIAGNOSTIC'):v.exit_code===0?'VANILLA_ONLY':c.exit_code===0?'CRYSTALLINE_ONLY':['exit_code','stdout_base64','stderr_base64'].every(k=>v[k]===c[k])?'MATCH_DIAGNOSTIC':'DIFFERENT_DIAGNOSTIC';
const base=json('p1322-baseline.json'),catalog=json('p1322-probe-catalog.json');
const matrices=['normal','repeat','reverse'].map(x=>json('p1322-matrix-'+x+'.json'));
const mat=matrices[0],expectedKeys=new Set(catalog.probes.flatMap(p=>Object.keys(catalog.profiles).map(profile=>key({id:p.id,profile}))));
const blockers=[],inputNames=['p1322-baseline.json','p1322-manifest.json','p1322-probe-catalog.json','p1322-runtime-freeze.json','p1322-classification-sources.json','p1322-classification-certification-debt.json',...['normal','repeat','reverse'].map(x=>'p1322-matrix-'+x+'.json')];
function checkRuns(runs,expected,label){
 const ref=new Map(runs[0].map(r=>[key(r),r]));
 for(const [i,rows] of runs.entries()){
  const keys=new Set(rows.map(key));if(rows.length!==keys.size||keys.size!==expected.size||[...expected].some(k=>!keys.has(k)))blockers.push(label+' incomplete/duplicate keys '+i);
  for(const r of rows){if(classify(r.vanilla,r.crystalline)!==r.runtime_class)blockers.push(label+' invalid classification '+key(r));if(r.runtime_class==='EXECUTION_UNKNOWN')blockers.push(label+' Unknown '+key(r));if(i&&ref.has(key(r))&&!same(stable(r),stable(ref.get(key(r)))))blockers.push(label+' unstable '+key(r));}
 }
}
checkRuns(matrices.map(m=>m.results),expectedKeys,'principal');
for(const m of matrices){if(m.catalog_sha256!==sha(b+'p1322-probe-catalog.json')||m.manifest_sha256!==sha(b+'p1322-manifest.json'))blockers.push('principal pin drift');}
const previous=json('p1309-matrix-normal.json'),oldRuntime=new Map(previous.results.map(r=>[key(r),r]));
const historicalPrincipalCount=new Set(previous.results.map(r=>r.id)).size;
const oldLedger=tab(b+'p1309-owner-ledger.tsv'),oldById=new Map(oldLedger.map(r=>[r.probe_id,r]));
const oldSelection=json('p1309-selection.json'),oldCohorts=new Map(oldSelection.cohorts.map(c=>[c.id,c]));
const prep=json('p1322-classification-sources.json');
const rust=Object.keys(base.state.product_inventory).filter(p=>/^(01_core|02_shell|03_infra|04_wiring)\//.test(p)&&p.endsWith('.rs'));
const owners=new Map();for(const p of rust)for(const m of read(p).matchAll(/^\s*\/\/[!/]?\s*@prompt\s+(\S+)\s*$/gm)){const a=owners.get(m[1])||[];a.push(p);owners.set(m[1],a);}
const metadata=new Map();function meta(p){if(!metadata.has(p)){const cs=owners.get(p)||[];if(cs.length!==1)throw Error('Owner not 1:1 '+p);metadata.set(p,{path:p,sha256:sha(p),consumers:cs,consumer_sha256:sha(cs[0]),consumer_prompt_hash:read(cs[0]).match(/@prompt-hash\s+(\S+)/)?.[1],declared_code_hash:read(p).match(/Hash do Código:\s*(\S+)/)?.[1],nucleus_pins:[...read(p).matchAll(/(00_nucleo\/prompts\/_nuclei\/\S+\.toml)\s+sha256:([a-f0-9]{64})/g)].map(m=>({path:m[1],effective_sha256:m[2],raw_sha256:sha(m[1])}))});}return metadata.get(p);}
const op=s=>'00_nucleo/prompts/'+s+'.md';
const continuous='ADR0127_INTERNAL_PARITY_L0_FIRST_RED_GREEN_REVALIDATE',publicGate='ADR0127_PUBLIC_OR_COMPATIBILITY_GATE_REQUIRES_FUTURE_CONCRETE_L0';
function refresh(s){if(!s)return s;
 if(s.startsWith('01_core/src/compiler/eval/bindings/field_access.rs:')){const n=Number(s.split(':')[1]);if(n===96)return s.replace(':96',':264');if(n===103)return s.replace(':103',':271');if(n===263)return s.replace(':263',':420');if(n===269)return s.replace(':269',':437');}
 if(s.startsWith('01_core/src/compiler/stdlib/loading.rs:')){const n=Number(s.split(':')[1]);const map={1047:1197,1013:1145,1175:1406};if(map[n])return s.slice(0,s.lastIndexOf(':')+1)+map[n];}
 return s;
}
const cohorts=new Map();
function cohort(id,spec,witness){
 if(!cohorts.has(id)){const c={id,eligible:true,priority:spec.priority,owner_prompts:spec.owner_prompts,causal_evidence:spec.causal_evidence.map(refresh),gate_class:spec.gate_class||continuous,regression_surface:spec.regression_surface||{rank:null,scope:'Not demonstrated',evidence:[]},causal_hypothesis:spec.causal_hypothesis||spec.regression_surface?.scope,paths:[],witnesses:[],canonical_route_demonstrated:true};
 c.owners=c.owner_prompts.map(p=>meta(p).consumers[0]).sort();c.owner_count=c.owners.length;cohorts.set(id,c);}
 const c=cohorts.get(id);if(witness){if(!c.paths.includes(witness.path))c.paths.push(witness.path);c.witnesses.push(witness);}return c;
}
function inheritedCohort(id,witness){const original=oldCohorts.get(id);if(!original)throw Error('Unknown historical cohort '+id);const spec=structuredClone(original);if(id.startsWith('missing-')&&spec.priority===6)spec.regression_surface={rank:null,scope:'Exact future signature/carrier and regression risk not yet specified; historical numeric rank is not evidence of low risk.',evidence:spec.causal_evidence};return cohort(id,spec,witness);}
const ledger=[],transitions=[];
function defaultOwner(path){if(path==='sym'||path.startsWith('sym.'))return [op('compiler/stdlib/sym'),'01_core/src/compiler/stdlib/sym.rs:638','lab/typst-original/crates/typst-library/src/lib.rs:374'];if(path==='emoji'||path.startsWith('emoji.'))return[op('compiler/stdlib/emoji'),'01_core/src/compiler/stdlib/emoji.rs:47','lab/typst-original/crates/typst-library/src/lib.rs:374'];if(path==='math'||path.startsWith('math.'))return[op('compiler/stdlib/structural/math'),'01_core/src/compiler/stdlib/structural/math.rs:1140','lab/typst-original/crates/typst-library/src/math/mod.rs:98'];return[op('compiler/eval'),'01_core/src/compiler/eval/mod.rs:1560','lab/typst-original/crates/typst-library/src/lib.rs:374'];}
const principalById=group(mat.results);
for(const p of catalog.probes){
 const rs=principalById.get(p.id)||[],prev=oldById.get(p.id),classes=rs.map(r=>r.runtime_class),types=new Set(classes),path=p.path;
 let [prompt,csrc,vsrc]=prev?[prev.owner_prompt,refresh(prev.crystalline_owner_file_line),prev.vanilla_source_file_line]:defaultOwner(path);
 let languageClass=allmatch(classes)?'CLOSED_MEASURED_LOOKUP_REPR_ONLY':types.has('EXECUTION_UNKNOWN')?'EXECUTION_UNKNOWN':types.has('VANILLA_ONLY')?'MISSING_LANGUAGE_MEMBER':types.has('DIFFERENT_DIAGNOSTIC')?'DIAGNOSTIC_DIVERGENCE':types.has('DIFFERENT_VALUE')?'WRONG_PUBLIC_VALUE_REPR':'UNRESOLVED_INTENT';
 let evidence=prev?.normative_evidence||prompt+':1',intent='Only the frozen lookup/kind/repr/diagnostic expression is measured; no claim about unmeasured function calls.',hypothesis=prev?.causal_hypothesis||'Fresh public path lookup, including finite Symbol modifiers; owner of binding discovery is distinct from complete callable semantics.',cid='';
 if(path==='html'&&allmatch(classes))languageClass='EXPECTED_FEATURE_DISABLED';
 if(types.has('CRYSTALLINE_ONLY')&&prep.extension_normative_evidence[path]){[evidence,intent]=prep.extension_normative_evidence[path];languageClass=path.startsWith('calc.')?'UNRESOLVED_INTENT':'DOCUMENTED_PRODUCT_EXTENSION';}
 if(!allmatch(classes)&&prev?.cohort_id){cid=prev.cohort_id;inheritedCohort(cid,{id:p.id,path,profile:rs.find(r=>!r.runtime_class.startsWith('MATCH'))?.profile||'default',universe:'principal',measurement_ref:b+'p1322-matrix-normal.json#'+p.id});}
 // Fresh modifier rows require separate source-level inspection, never automatic old closure.
 if(!allmatch(classes)&&!cid&&!['DOCUMENTED_PRODUCT_EXTENSION','UNRESOLVED_INTENT','EXECUTION_UNKNOWN'].includes(languageClass)){
  if((path.startsWith('sym.quote.')||path.startsWith('math.quote.'))&&types.has('DIFFERENT_VALUE')){prompt=op('entities/symbol');csrc='01_core/src/entities/symbol.rs:121';vsrc='lab/typst-original/crates/typst-library/src/foundations/symbol.rs:367';cid='symbol-repr-quote-escaping';hypothesis='The same string escaping helper applies escape_debug then escapes quotes again; selected quote variants exercise the public repr defect.';inheritedCohort(cid,{id:p.id,path,profile:rs[0].profile,universe:'principal',measurement_ref:b+'p1322-matrix-normal.json#'+p.id});}
  else if(path.startsWith('math.join.')){cid='math-symbol-binding-warning';prompt=op('compiler/eval/bindings/field_access');csrc='01_core/src/compiler/eval/bindings/field_access.rs:264';vsrc='lab/typst-original/crates/typst-eval/src/code.rs:353';hypothesis='Missing warning happens at the math.join ancestor, before modifier lookup; the existing top-level warning guard only accepts module sym.';inheritedCohort(cid,{id:p.id,path,profile:rs[0].profile,universe:'principal',measurement_ref:b+'p1322-matrix-normal.json#'+p.id});}
  else if(/^(sym|math)\.(gt\.tri|lt\.tri|tack\..*double)/.test(path)&&types.has('DIFFERENT_DIAGNOSTIC')){languageClass='DOCUMENTED_INTENTIONAL_DIAGNOSTIC_DIVERGENCE';prompt=op('entities/symbol');csrc='01_core/src/entities/symbol.rs:83';vsrc='lab/typst-original/crates/typst-library/src/foundations/symbol.rs:141';evidence=op('compiler/stdlib/sym')+':90';intent='Current L0 explicitly preserves omitted variant deprecation messages while requiring modifier values. This is not language diagnostic parity and is not an extension credit.';hypothesis='SymbolVariant lacks deprecation-message/state carrier; modified returns only an optional symbol. Vanilla emits warning from variant metadata on first matching deprecated modifier.';}
  else {cid='unresolved-new-path-'+path;cohort(cid,{priority:7,owner_prompts:[prompt],causal_evidence:[csrc,vsrc,evidence],gate_class:'UNRESOLVED_CAUSAL_SCOPE_NO_IMPLEMENTATION_AUTHORITY',regression_surface:{rank:null,scope:'Fresh path requires source-specific causal resolution',evidence:[]}}, {id:p.id,path,profile:rs[0]?.profile,universe:'principal',measurement_ref:b+'p1322-matrix-normal.json#'+p.id}).eligible=false;}
 }
 const time=[];for(const r of rs){const old=oldRuntime.get(key(r)),fixtureComparable=old?old.expression===r.expression:false;let transition=!old?'ADDED_FRESH':old.runtime_class===r.runtime_class?'UNCHANGED_RUNTIME_CLASS':r.runtime_class.startsWith('MATCH')&&!old.runtime_class.startsWith('MATCH')?'CLOSED_NOW':old.runtime_class.startsWith('MATCH')&&!r.runtime_class.startsWith('MATCH')?'REGRESSION_CANDIDATE':'CHANGED_NONMATCH';if(transition==='REGRESSION_CANDIDATE')blockers.push('Unisolated regression '+key(r));time.push(transition);transitions.push({probe_id:p.id,path,profile:r.profile,previous_runtime_class:old?.runtime_class||'NOT_IN_P1309',current_runtime_class:r.runtime_class,transition,fixture_comparable:fixtureComparable,universe:'principal',measurement_ref:b+'p1322-matrix-normal.json#'+p.id,previous_measurement_ref:old?b+'p1309-matrix-normal.json#'+p.id:''});}
 const md=meta(prompt);ledger.push({path,probe_id:p.id,profiles:rs.map(r=>r.profile).join(','),runtime_class_by_profile:rs.map(r=>r.profile+':'+r.runtime_class).join(';'),previous_class:prev?.current_language_class||'NOT_IN_P1309',transition:[...new Set(time)].join(','),current_language_class:languageClass,crystalline_owner_file_line:csrc,vanilla_source_file_line:vsrc,owner_prompt:prompt,owner_prompt_sha256:md.sha256,consumer_unique:md.consumers[0],consumer_prompt_hash:md.consumer_prompt_hash,language_observable:p.expression,normative_intent:intent,normative_evidence:evidence,causal_hypothesis:hypothesis,refutation:'Same source/profile has equal complete observable, or cited current source identifies a different cause/required owner. MATCH does not certify calls absent from the corpus.',gate_class:cid?cohorts.get(cid).gate_class:'NO_CHANGE_AUTHORIZED',universe:'principal',canonical_route:path,cohort_id:cid,measurement_ref:b+'p1322-matrix-normal.json#'+p.id});
}

const supplementalRuns=['normal','repeat','reverse'].map(x=>json('p1322-sentinels-'+x+'.json'));
inputNames.push('p1322-sentinels-cases.json','p1322-sentinels-freeze.json',...['normal','repeat','reverse'].map(x=>'p1322-sentinels-'+x+'.json'));
const supp=supplementalRuns[0].rows;
const suppCases=json('p1322-sentinels-cases.json');
const suppExpected=new Set(suppCases.cases.flatMap(c=>Object.keys(c.observations||suppCases.profiles).map(profile=>key({id:c.id,profile}))));
checkRuns(supplementalRuns.map(m=>m.rows),suppExpected,'supplement');
const field=op('compiler/eval/bindings/field_access'),loading=op('compiler/stdlib/loading'),dispatch=op('compiler/eval/call_dispatch');
function newSpec(priority,owner_prompts,causal_evidence,hypothesis,rank=null){return{priority,owner_prompts,causal_evidence,causal_hypothesis:hypothesis,regression_surface:{rank,scope:hypothesis,evidence:causal_evidence}};}
const specs={
 'namespace-function-missing-field':newSpec(3,[field],['01_core/src/compiler/eval/bindings/field_access.rs:181','01_core/src/compiler/eval/bindings/field_access.rs:271','01_core/src/compiler/eval/bindings/field_access.rs:420','01_core/src/entities/func.rs:291','01_core/src/entities/func.rs:355','lab/typst-original/crates/typst-library/src/foundations/func.rs:291','lab/typst-original/crates/typst-eval/src/code.rs:347',field+':170'],'Native/NativeWithEngine namespace Some missing-field branch omits public function name and uses legacy quote style; span chooser excludes this category. Existing Func name/With traversal and AST field span suffice in this one owner. Restrict to Some missing lookup; retain present lookup, Native None, Closure/Plugin/Element, Module/Dict/Type/Content/Float behavior.',1),
 'closure-missing-field-diagnostic':newSpec(3,[field],['01_core/src/compiler/eval/bindings/field_access.rs:181','01_core/src/compiler/eval/bindings/field_access.rs:427','lab/typst-original/crates/typst-library/src/foundations/func.rs:297',field+':170'],'Namespace None for closure/With uses generic type-function message and aggregate field span. Existing FuncRepr categories and field span identify this separate category.',1),
 'nonmodule-field-span':newSpec(3,[field],['01_core/src/compiler/eval/bindings/field_access.rs:271','lab/typst-original/crates/typst-eval/src/code.rs:347',field+':170'],'For Dict, Content and non-is-nan Float missing fields, identical primary messages already exist; owner chooses access.span instead of existing field span. No lookup behavior change is needed.',1),
 'type-field-diagnostic':newSpec(3,[field],['01_core/src/compiler/eval/bindings/field_access.rs:437','lab/typst-original/crates/typst-library/src/foundations/ty.rs:1',field+':170'],'Type field fallback uses internal int name and legacy quotes plus aggregate span. Full formatter/category scope not demonstrated.'),
 'csv-symbol-source':newSpec(4,[loading],['01_core/src/compiler/stdlib/loading.rs:1406','01_core/src/compiler/stdlib/loading.rs:1197','lab/typst-original/crates/typst-library/src/loading/mod.rs:54',loading+':1'],'Symbol is rejected before DataSource coercion, changing successful parse and source/options precedence. Existing Symbol has a character but source provenance/general cast policy scope is unresolved.'),
 'csv-symbol-delimiter':newSpec(4,[loading],['01_core/src/compiler/stdlib/loading.rs:1360','lab/typst-original/crates/typst-library/src/loading/csv.rs:103',loading+':1'],'Delimiter accepts Str only; Symbol coercion changes successful delimiter parse and invalid-ASCII diagnostic. Existing character carrier is present; exact future cast scope must be specified.'),
 'csv-valid-file-error-anchor':newSpec(3,[loading],['01_core/src/compiler/stdlib/loading.rs:948','01_core/src/compiler/stdlib/loading.rs:955','01_core/src/compiler/stdlib/foundations/path.rs:28','03_infra/src/world.rs:743','lab/typst-original/crates/typst-library/src/loading/csv.rs:138',loading+':1'],'CSV valid UTF-8 external-file parse error has correct text but lacks external source excerpt. Current read_path returns bytes without FileId; the complete required owner/contract set is unresolved. Not a one-owner cheap diagnostic.'),
 'loader-io-envelope':newSpec(3,[loading],['01_core/src/compiler/stdlib/loading.rs:1188','03_infra/src/world.rs:743','lab/typst-original/crates/typst-library/src/loading/read.rs:24',loading+':1'],'Loading wraps failure in Portuguese/detached span; physical searched-path spelling also differs in World. Complete external-I/O envelope requires more than the diagnosed loading owner; full set unresolved.'),
 'closure-missing-call-span':newSpec(3,[dispatch,op('compiler/eval/closures')],['01_core/src/compiler/eval/closures.rs:88','01_core/src/compiler/eval/call_dispatch.rs:455','lab/typst-original/crates/typst-library/src/foundations/args.rs:173'],'Closure missing positional uses Args.span, but native-only aggregate transport deliberately excludes closures. Complete closure origin/capture risk is not demonstrated.'),
 'math-direct-csv-lookup':newSpec(7,[op('compiler/eval/math')],['01_core/src/compiler/eval/math.rs:1','lab/typst-original/crates/typst-eval/src/math.rs:1'],'Direct math identifier resolves global csv before canonical math-namespace rejection. Alias/With reach CSV; this is a lookup cause, not native CSV missing-source.'),
 'eval-import-path-boundary':newSpec(7,[op('compiler/eval/modules')],['01_core/src/compiler/eval/modules.rs:152','03_infra/src/world.rs:723'],'Eval-import source has a path outside the current sandbox root. Compile captured-only succeeds, so captured/eval and closure/file-origin causes cannot be collapsed; complete owner set unresolved.'),
 'detached-source-origin':newSpec(7,[loading],['01_core/src/compiler/stdlib/foundations/path.rs:15','01_core/src/compiler/stdlib/loading.rs:1406','lab/typst-original/crates/typst-library/src/loading/mod.rs:54'],'Detached spread string is resolved against current file despite vanilla forbidding filesystem access from detached provenance. Carrier/World boundary unresolved.'),
 'array-bytes-constructor':newSpec(5,[dispatch],['01_core/src/compiler/eval/call_dispatch.rs:1695','lab/typst-original/crates/typst-library/src/foundations/array.rs:164'],'Type array reaches no-constructor fallback despite Array/Bytes values existing. Constructor contract/implementation owner scope not demonstrated.'),
};
function mapping(r){const id=r.id,e=r.expression;
 if(/^p1311\.(assert|json-missing|yaml-missing|toml-missing|cbor-missing|json-with-missing|table-missing)$/.test(id))return['namespace-function-missing-field',e.split('.')[0]];
 if(/^p1311\.(closure|closure-with|named-closure|named-closure-nested-with)$/.test(id))return['closure-missing-field-diagnostic','closure.missing-field'];
 if(id==='p1306.dictionary'||/^p1311\.(dictionary|content|float-other)$/.test(id))return['nonmodule-field-span',id.includes('content')?'content.missing-field':id.includes('float')?'float.missing-field':'dictionary.missing-field'];
 if(id==='p1311.type')return['type-field-diagnostic','int.missing-field'];
 if(e==='cbor.encode()')return['cbor-argument-missing','cbor.encode'];
 if(id.startsWith('p1307.cbor.arg.')||/^causal-cbor\.[0-8]$/.test(id))return['cbor-argument-validation','cbor.encode'];
 if(e==='read()')return['loader-path-missing','read'];
 if(/^(json|yaml|toml|cbor|xml)\(\)$/.test(e))return['loader-data-source-missing',e.slice(0,-2)];
 if(/cbor.*(Symbol|Content)|cbor-(decoded|bytes)/.test(id))return['cbor-symbol-content-fallback','cbor.encode'];
 if(id==='r4.args.sink-before-positional')return['closure-nonterminal-sink-binding','closure.nonterminal-sink'];
 if(id.startsWith('math.control.'))return['calc-abs-content-diagnostic','calc.abs'];
 if(id==='p1306.imported-error')return['module-import-error-trace','module.import-error-trace'];
 if(id.startsWith('p1306.global-bare'))return['module-bare-import-warning','module.bare-import-warning'];
 if(id==='p1307.decoder.xml'||id==='p1311.xml-success'||id==='p1321.protected_xml')return['xml-node-namespace','xml'];
 if(id==='p1321.preserve_path_bad'||id.startsWith('p1320.csv-unequal-'))return['csv-valid-file-error-anchor','csv'];
 if(id==='p1321.preserve_io'||id.startsWith('p1320.csv-missing-file')||id==='causal-cbor.10')return['loader-io-envelope',id==='causal-cbor.10'?'read':'csv'];
 if(id.includes('symbol_source')||id==='p1321.protected_source'||id==='p1320.csv-symbol-source')return['csv-symbol-source','csv'];
 if(id.includes('symbol_delimiter')||id==='p1321.protected_delimiter'||id==='p1320.csv-symbol-delimiter')return['csv-symbol-delimiter','csv'];
 if(id.startsWith('p1321.r2.r2_user_'))return['closure-missing-call-span','closure.missing-argument'];
 if(id==='p1321.r2.r2_math_csv_direct')return['math-direct-csv-lookup','math.csv'];
 if(id==='p1320.csv-captured-eval'||id==='p1320.csv-closure-eval')return['eval-import-path-boundary','eval.import'];
 if(id==='p1320.csv-detached-good')return['detached-source-origin','csv'];
 if(id==='p1320.bytes-roundtrip')return['array-bytes-constructor','array'];
 return null;
}
const suppLedger=[];
for(const [id,rs] of group(supp)){
 const r=rs[0],raw=rs.map(x=>x.runtime_class),projected=rs.map(x=>x.language_projection),closed=rs.every(x=>x.runtime_class.startsWith('MATCH')||x.language_projection==='Preserved');
 let cid='',c=null;
 if(!closed){const mapped=mapping(r);if(!mapped)throw Error('Unmapped supplemental divergence '+id);cid=mapped[0];const witness={id,path:mapped[1],profile:r.profile,universe:'supplement',measurement_ref:b+'p1322-sentinels-normal.json#'+id};c=specs[cid]?cohort(cid,specs[cid],witness):inheritedCohort(cid,witness);}
 suppLedger.push({probe_id:id,path:c?.paths.find(x=>x===mapping(r)?.[1])||id,profiles:rs.map(x=>x.profile).join(','),runtime_class_by_profile:rs.map(x=>x.profile+':'+x.runtime_class).join(';'),language_projection_by_profile:rs.map(x=>x.profile+':'+(x.language_projection||'NOT_USED')).join(';'),current_language_class:closed?(raw.every(x=>x.startsWith('MATCH'))?'CLOSED_MEASURED_FUNCTIONAL_SENTINEL':'MATCH_PROJECTED_LANGUAGE_RAW_TRANSPORT_DIFFERENCE'):raw.includes('EXECUTION_UNKNOWN')?'EXECUTION_UNKNOWN':/cbor-(decoded|bytes)/.test(id)?'DIFFERENT_PROJECTED_VALUE':'OPEN_LANGUAGE_DIVERGENCE',owner_prompt:c?.owner_prompts.join(';')||'',crystalline_owner_file_line:c?.causal_evidence.filter(x=>!x.startsWith('lab/')&&!x.startsWith('00_')).join(';')||'',vanilla_source_file_line:c?.causal_evidence.filter(x=>x.startsWith('lab/')).join(';')||'',language_observable:r.expression,causal_hypothesis:c?.causal_hypothesis||'Only this complete bilateral assertion/projection is closed; historical expected preservation is distinct from parity.',refutation:'Equal complete language observation on same literal source/profile refutes this divergence; a different required owner refutes proposed causal scope.',gate_class:c?.gate_class||'NO_CHANGE_AUTHORIZED',universe:'supplement',cohort_id:cid,measurement_ref:b+'p1322-sentinels-normal.json#'+id});
}

const transversalLedger=[];
if(fs.existsSync(b+'p1322-transversal-r2.json')){
 const tx=json('p1322-transversal-r2.json');
 for(const run of tx.matrix.filter(x=>x.phase==='normal'))for(const r of run.results){let cid='',language=r.estado==='MATCH'?'CLOSED_MEASURED_TRANSVERSAL':r.estado==='DISABLED_BY_PROFILE'?'EXPECTED_FEATURE_GATED':r.estado;
  if(r.estado==='DIFFERENCE'&&r.id==='P1137-I-001'){
   cid='query-cli-feature-option';language='MISSING_PUBLIC_CLI_CAPABILITY';
   cohort(cid,{...newSpec(6,[op('shell/cli'),op('wiring')],['02_shell/src/cli.rs:313','02_shell/src/cli.rs:411','04_wiring/src/main.rs:512','lab/typst-original/crates/typst-cli/src/args.rs:185',op('shell/cli')+':929'],'QueryArgs lacks --features and public QueryIntent lacks feature transport. L0 P1165/P1288 only commit Compile/Eval. Future query public contract and L4/L3 transport require explicit ADR0127 gate; full downstream owners/risk unresolved.'),gate_class:publicGate},{id:r.id,path:'CLI.query.--features',profile:run.profile,universe:'transversal',measurement_ref:b+'p1322-transversal-r2.json#'+r.id});
  }else if(r.estado==='DIFFERENCE'&&/^P1138-S-00[123]$/.test(r.id)){
   cid='external-diagnostic-path-display';language='ENVIRONMENT_BOUND_DIAGNOSTIC_SPELLING';
   cohort(cid,newSpec(7,[op('wiring')],['04_wiring/src/main.rs:633','lab/typst-original/crates/typst-cli/src/world.rs:156'],'Outside cwd, strip_prefix fallback displays absolute path; vanilla pathdiff creates relative ../ path. Internal copies match, so no comparable P1320 regression. Same file identity is established but current normative intent for this diagnostic spelling is unresolved, not assumed mechanical closure.'),{id:r.id,path:'diagnostic.external-path-display',profile:run.profile,universe:'transversal',measurement_ref:b+'p1322-transversal-r2.json#'+r.id});
  }else if(r.estado==='DIFFERENCE'&&/^P1138-X-00[23]$/.test(r.id))language='MECHANICAL_RENDER_OR_FONT_RESOURCE_DIFFERENCE';
  else if(!['MATCH','DISABLED_BY_PROFILE'].includes(r.estado))blockers.push('Unclassified transversal '+r.id+' '+r.estado);
  transversalLedger.push({probe_id:r.id,profile:run.profile,current_language_class:language,raw_state:r.estado,cohort_id:cid,universe:'transversal',measurement_ref:b+'p1322-transversal-r2.json#'+r.id});
 }
}
const txClosure=json('p1322-transversal.json');
for(const r of txClosure.closures.filter(r=>r.phase==='normal')){
 const cid=r.id==='closure-only'?'closure-source-file-origin':'';
 if(cid)cohort(cid,newSpec(7,[op('compiler/eval/closures'),loading],['01_core/src/compiler/eval/closures.rs:160','01_core/src/compiler/eval/closures.rs:169','01_core/src/compiler/stdlib/foundations/path.rs:15','lab/typst-original/crates/typst-eval/src/call.rs:638'],'Imported closure body resolves its CSV string against caller engine file; captured-only document succeeds but closure-only reads root data.csv rather than sub/data.csv. Full source-origin carrier/owner/public-contract scope remains unresolved.'),{id:r.id,path:'closure.source-file-origin',profile:r.profile,universe:'transversal',measurement_ref:b+'p1322-transversal.json#'+r.id});
 transversalLedger.push({probe_id:r.id,profile:r.profile,current_language_class:cid?'OPEN_LANGUAGE_SOURCE_ORIGIN':'CLOSED_DOCUMENT_ASSERTIONS_ONLY',raw_state:r.state,cohort_id:cid,universe:'transversal',measurement_ref:b+'p1322-transversal.json#'+r.id});
}
// Re-decide scope and eligibility now. Discovery/glue ownership is never a complete repair owner set.
for(const c of cohorts.values()){
 c.paths.sort();c.path_count=c.paths.length;
 c.causal_evidence=[...new Set(c.causal_evidence)];
 c.current_redecision='Fresh P1322 bilateral witnesses establish this observed divergence; current owner/source and explicit L0 scope delimit cause. Historical priority alone is not authority.';
 c.refutation='A same-source/profile complete equal observation refutes divergence; missing native category/origin or any further productive consumer needed for the complete message/value/reflection refutes the stated owner set.';
 c.owner_set_complete=c.regression_surface.rank!==null&&!c.id.startsWith('missing-');
 c.canonical_route_demonstrated=c.witnesses.length>0;
 if(c.id.startsWith('missing-')&&c.priority===6){c.owner_set_complete=false;c.regression_surface.rank=null;c.regression_surface.scope='Current lookup proves absence only. Historical glue owner is a discovery owner; complete future public signature/entity/phase/owners and risk are Unknown.';c.gate_class=publicGate;}
 if(c.regression_surface.rank===null)c.eligible=false;
 if(!c.owner_set_complete)c.eligible=false;
 if(['missing-math-style-exposure','missing-float-constants'].includes(c.id)){c.eligible=false;c.owner_set_complete=false;c.regression_surface={rank:null,scope:'Existing carrier is suggested by current neighboring bindings, but complete future callable contract and owner scope were not proven by lookup. Not a cheap repair by shared glue.',evidence:c.causal_evidence};}
 if(c.id==='math-symbol-binding-warning'){c.regression_surface.scope='Only math.join ancestor warning is missing. Four current paths (top binding and three modifier continuations) share the same sym-only module guard; existing warning carrier and unchanged sym counterparts bound regression to one guarded warning route.';}
 if(c.id==='cbor-argument-validation'){
  c.owner_prompts.push(op('compiler/eval'));c.owners.push(meta(op('compiler/eval')).consumers[0]);c.owners.sort();c.owner_count=c.owners.length;
  c.causal_evidence.push('01_core/src/compiler/eval/mod.rs:1836','01_core/src/compiler/eval/call_dispatch.rs:1648');
  c.regression_surface={rank:null,scope:'Direct arity/named formatting is loading-local, but With/Args/sink expose qualified cbor.encode trace identity from eval registration. Complete indirect observable cannot be advertised as one-owner validation; required repair interaction remains Unknown.',evidence:c.causal_evidence};c.owner_set_complete=false;c.eligible=false;
 }
 c.rank_key=[c.priority,c.owner_count,-c.path_count,c.regression_surface.rank,c.id];
}
const sourceFiles=[...new Set([...cohorts.values()].flatMap(c=>c.causal_evidence).concat(ledger.flatMap(r=>[r.crystalline_owner_file_line,r.vanilla_source_file_line,r.normative_evidence])).filter(Boolean).map(s=>s.split(':')[0]).filter(s=>fs.existsSync(s)))].sort();
const sourcePins=sourceFiles.map(path=>({path,sha256:sha(path)}));
const closureClaims=[
 ['P1310','Five invalid DataSource casts, public type and original offending span',r=>/^p1307\.decoder\.(json|yaml|toml|cbor|xml)\.wrong-type$/.test(r.id),'Symbol/missing/named/excess/I/O remain separate.'],
 ['P1311','Native namespace None field identity and field span',r=>/^p1311\.(csv-|read-|xml-)/.test(r.id)&&r.id!=='p1311.xml-success','Some namespace and user closures explicitly preserved as open boundaries; absent encode is bilateral diagnostic, not missing functionality.'],
 ['P1312','Read invalid path cast with original argument span',r=>r.family==='read-cast','Read missing and I/O are not closed; CSV is not Path-only.'],
 ['P1313','CSV Bytes success and invalid DataSource cast',r=>['p1321.valid_array','p1321.cast_int','p1321.cast_bool','p1321.cast_content'].includes(r.id),'Symbol source remains rejected; functional implementation report is historical authority, not initial proposal.'],
 ['P1314','CSV validates each option occurrence and retains origin',r=>/^p1321\.(with_(delimiter|row|cross)|args_dup|sink_dup|option_)/.test(r.id),'Syntactic duplicate and Symbol are separate strata.'],
 ['P1315','CSV unequal record ordinal',r=>['p1321.preserve_unequal','p1321.preserve_multiline','p1321.preserve_crlf'].includes(r.id),'Ordinal alone does not close the complete diagnostic; the current complete Bytes diagnostics match.'],
 ['P1316','CSV Bytes parser error original argument span',r=>r.id==='p1321.preserve_unequal','UTF-8-valid file external excerpts remain open.'],
 ['P1317','CSV UTF-8 public cause and first-error precedence',r=>['p1321.preserve_utf8','p1320.csv-binary-bytes'].includes(r.id),'Later invalid byte/earlier fields also rechecked by transversal boundary extras.'],
 ['P1318','CSV Bytes textual line/column including multiline/CRLF',r=>['p1321.preserve_multiline','p1321.preserve_crlf','p1321.preserve_utf8'].includes(r.id),'Unicode and later invalid byte are separately measured in frozen boundary extras.'],
 ['P1319','CSV invalid-buffer file virtual path and caller origin',r=>/^p1320\.csv-binary-file-(str|path)$/.test(r.id),'Valid UTF-8 file excerpt, I/O and closure Str origin are not closed.'],
 ['P1320','Focal audit lacunas remeasured, not a historical global denominator',r=>r.family==='p1320-lacunas','Current open focal rows remain explicit; csv.encode is bilateral absence, not new encoder.'],
 ['P1321','CSV source/options/remaining/read precedence and missing aggregate spans',r=>/^p1321\.(missing_|cast_|remaining_|precedence_|alias_|args_|sink_|with_|map_|spread_|syntax_duplicate|valid_)/.test(r.id),'Native identity stays restricted: user closures, read/json/cbor missing, direct math csv, Symbol, I/O/external-origin remain scoped debt.'],
].map(([step,claim,predicate,boundary])=>{const rows=supp.filter(predicate);return{step,historical_claim:claim,measurement:pin('p1322-sentinels-normal.json'),witness_ids:[...new Set(rows.map(r=>r.id))],cells:rows.length,raw_counts:count(rows.map(r=>r.runtime_class)),current_decision:step==='P1320'?'FOCAL_OPEN_AND_CLOSED_ROWS_RECONCILED':rows.length&&rows.every(r=>r.runtime_class.startsWith('MATCH')||r.language_projection==='Preserved')?'CLOSED_ONLY_STATED_FRAGMENT':'NOT_CLOSED',boundary};});
for(const c of closureClaims)if(c.step!=='P1320'&&c.current_decision!=='CLOSED_ONLY_STATED_FRAGMENT')blockers.push('Mandatory historical fragment not revalidated '+c.step);
const transversal=json('p1322-transversal.json');inputNames.push('p1322-transversal.json','p1322-transversal-freeze.json');
const extras=['normal','repeat','reverse'].map(phase=>transversal.extra.filter(r=>r.phase===phase));checkRuns(extras,new Set(extras[0].map(key)),'boundary-extra');
const extraSummary={counts:count(extras[0].map(r=>r.runtime_class)),cells:extras[0].length,scope:'Boundary CSV multiline/Unicode/late byte invalid and JSON invalid source alias/With; see complete expressions and raw receipts.'};
const closureBoundary={measurement:pin('p1322-transversal.json'),rows:transversal.closures.map(r=>({id:r.id,profile:r.profile,phase:r.phase,state:r.state,vanilla_exit:r.vanilla.exit,crystalline_exit:r.crystalline.exit})),interpretation:'Captured-only document assertions pass. Closure-only fails by reading root data.csv rather than module sub/data.csv. Eval imported source rejects sandbox boundary earlier; these are not one cause and not P1321 native missing-source debt.',evidence:['01_core/src/compiler/eval/closures.rs:160','01_core/src/compiler/eval/closures.rs:169','01_core/src/compiler/stdlib/foundations/path.rs:15','03_infra/src/world.rs:723'],owner_set_complete:false,risk:'Unknown',gate_class:publicGate};
// Generic transversal profile provenance has an independently identified R0 adapter defect.
// A fresh repaired receipt must be explicitly pinned below before this audit can recommend.
const transversalProfileReceiptName='p1322-transversal-r2.json';
let transversalProfileSummary=null;
if(!transversalProfileReceiptName||!fs.existsSync(b+transversalProfileReceiptName))blockers.push('Mandatory transversal four-profile successor receipt not yet supplied');
else {
 inputNames.push(transversalProfileReceiptName,'p1322-transversal-r2-freeze.json');const tx=json(transversalProfileReceiptName),seen=new Set(),normal=new Map(),statuses=[];
 function features(argv){const xs=[];for(let i=0;i<argv.length;i++){if(argv[i]==='--features')xs.push(...argv[++i].split(','));else if(argv[i].startsWith('--features='))xs.push(...argv[i].slice(11).split(','));}return [...new Set(xs)].sort();}
 for(const run of tx.matrix)for(const r of run.results){const k=r.id+'\0'+run.profile,phaseKey=k+'\0'+run.phase;if(seen.has(phaseKey))blockers.push('Transversal duplicate '+phaseKey);seen.add(phaseKey);statuses.push(r.estado);if(r.estado==='UNKNOWN')blockers.push('Transversal Unknown '+phaseKey);for(const s of ['oracle','crystalline'])if(r[s]?.command&&!same(features(r[s].command),[...catalog.profiles[run.profile]].sort()))blockers.push('Transversal feature mismatch '+phaseKey+' '+s);if(run.phase==='normal')normal.set(k,[r.estado,r.classe]);else if(!same(normal.get(k),[r.estado,r.classe]))blockers.push('Transversal unstable language state '+phaseKey);}
 if(seen.size!==20*4*3)blockers.push('Transversal missing expected 240 cells');
 if(tx.location.length!==3*4*3||tx.location.some(r=>r.estado!=='MATCH'))blockers.push('Location contrast missing/not MATCH');
 transversalProfileSummary={receipt:pin(transversalProfileReceiptName),cells:seen.size,states:count(statuses),location_contrast_cells:tx.location.length,interpretation:'External-source spelling differences remain environment-bound observations: source path differs only by relative/absolute display, while internal copies match. They are not comparable P1320 regressions. Raster subpixel and font resource names remain mechanical, with language geometry/text checked separately.',closure_boundary:closureBoundary};
}
const rankCompare=(a,z)=>a.priority-z.priority||a.owner_count-z.owner_count||z.path_count-a.path_count||(a.regression_surface.rank??Infinity)-(z.regression_surface.rank??Infinity)||a.id.localeCompare(z.id);
const ranked=[...cohorts.values()].filter(c=>c.eligible).sort(rankCompare),selected=blockers.length?null:ranked[0]||null;
for(const c of cohorts.values())c.loss_reason=!c.eligible?'INELIGIBLE: complete causal owner/risk not demonstrated':c===selected?'SELECTED':!selected?'BLOCKED_PENDING_MANDATORY_EVIDENCE':'Loses lexicographic rank '+JSON.stringify(c.rank_key)+' to '+JSON.stringify(selected.rank_key);
const rawCounts=count(mat.results.map(r=>r.runtime_class)),closedCells=(rawCounts.MATCH_VALUE||0)+(rawCounts.MATCH_DIAGNOSTIC||0),extensions=ledger.filter(r=>r.current_language_class==='DOCUMENTED_PRODUCT_EXTENSION'),excludedExtensionIds=new Set(extensions.map(r=>r.probe_id)),adjustedRows=mat.results.filter(r=>!excludedExtensionIds.has(r.id));
const common={schema:'p1322-classification-v1',at:new Date().toISOString(),head:base.state.head,working_tree:base.state.status,diff_stat:base.state.diff_stat,regime:'executado sem atestação técnica de isolamento',inputs:[...new Set(inputNames)].map(pin),classifier:pin('p1322-classification-final.cjs'),verdict:'NOT_SELF_APPROVED_ROLE_D_REQUIRED'};
const summary={...common,principal:{probes:catalog.probes.length,cells:mat.results.length,raw_counts:rawCounts,raw_closed_cells:closedCells,raw_ratio:closedCells/mat.results.length,path_classes:count(ledger.map(r=>r.current_language_class)),unknown:rawCounts.EXECUTION_UNKNOWN||0,three_orders_checked:true},adjusted:{policy:'Exclude only L0-documented product-extension rows, not unresolved calc extras and not documented variant-warning divergences.',excluded_paths:extensions.map(r=>r.path),cells:adjustedRows.length,raw_counts:count(adjustedRows.map(r=>r.runtime_class)),closed_ratio:adjustedRows.filter(r=>r.runtime_class.startsWith('MATCH')).length/adjustedRows.length},temporal:{previous_probe_count:historicalPrincipalCount,current_historical_probe_count:ledger.filter(r=>oldById.has(r.probe_id)).length,cell_transitions:count(transitions.map(r=>r.transition)),added_paths:catalog.probes.length-historicalPrincipalCount},supplement:{cases:suppLedger.length,cells:supp.length,raw_counts:count(supp.map(r=>r.runtime_class)),language_projection:count(supp.map(r=>r.language_projection||'NOT_USED')),language_classes:count(suppLedger.map(r=>r.current_language_class)),outside_principal_denominator:true},extraSummary,transversalProfileSummary,blockers};
const selection={...common,rule:'priority, fewer complete causal owners, more actual same-cause paths, smaller demonstrated regression surface, lexical id; Unknown is never low risk.',selected:selected?{id:selected.id,priority:selected.priority,owner_prompts:selected.owner_prompts,owners:selected.owners,paths:selected.paths,rank_key:selected.rank_key,gate_class:selected.gate_class}:null,ranked_eligible:ranked.map(c=>c.id),cohorts:[...cohorts.values()].sort(rankCompare),blockers,implementation_authorized:false};
function tsv(rows){const ks=[...new Set(rows.flatMap(r=>Object.keys(r)))];return ks.join('\t')+'\n'+rows.map(r=>ks.map(k=>String(r[k]??'').replace(/[\t\r\n]/g,' ')).join('\t')).join('\n')+'\n';}
const outputs={
 'owner-ledger.tsv':tsv(ledger),'supplemental-ledger.tsv':tsv(suppLedger),'transversal-ledger.tsv':tsv(transversalLedger),'transitions.tsv':tsv(transitions),'summary.json':summary,'selection.json':selection,'source-lineage.json':{...common,owner_metadata:[...metadata.values()],sources:sourcePins},'functional-reconciliation.json':{...common,closureClaims,extraSummary,closureBoundary},
};
outputs['cohorts.md']='# P1322 — coortes causais atuais\n\nSem autoveredito; aguardam revisão D. Paths de controles/alias repetidos não multiplicam causas. Owner set incompleto e risco Unknown não são reparos baratos.\n\n| Coorte | Prioridade | Owners conhecidos | Completo | Paths | Risco | Elegível |\n| --- | --- | --- | --- | --- | --- | --- |\n'+[...cohorts.values()].sort(rankCompare).map(c=>`| ${c.id} | ${c.priority} | ${c.owner_count} | ${c.owner_set_complete} | ${c.path_count} | ${c.regression_surface.rank??'Unknown'} | ${c.eligible} |`).join('\n')+'\n';
const lines=['*** Begin Patch'];for(const [name,data] of Object.entries(outputs)){const path=b+'p1322-classification-'+name;const content=typeof data==='string'?data:JSON.stringify(data,null,2)+'\n';if(fs.existsSync(path)){if(!process.argv.includes('--refresh-own-artifacts'))throw Error('Refusing overwrite '+path);lines.push('*** Update File: '+path,'@@',...read(path).trimEnd().split('\n').map(x=>'-'+x),...content.trimEnd().split('\n').map(x=>'+'+x));}else lines.push('*** Add File: '+path,...content.trimEnd().split('\n').map(x=>'+'+x));}lines.push('*** End Patch');console.log(lines.join('\n'));
