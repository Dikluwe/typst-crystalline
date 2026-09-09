// Independent postclosure read-only audit. Stdin: scoped NUL-separated git file inventory.
const fs=require('fs'),crypto=require('crypto'),path=require('path');
const R='/repos/Antigravity/typst-crystalline',D=R+'/00_nucleo/diagnosticos/';
const inputs={},violations=[],unknowns=[],counts={};let checks=0;
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
function read(p,pin=true){p=path.resolve(R,p);if(/00_nucleo\/(materialization|context)(\/|$)/.test(p))throw Error('restricted path');const b=fs.readFileSync(p);if(pin)inputs[p]=sha(b);return b;}
function ck(n,ok){checks++;if(!ok)violations.push(n);}
const json=n=>JSON.parse(read(D+n));
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function map(name,values){counts[name]=Object.keys(values).length;for(const[p,h]of Object.entries(values))ck(name+':'+p,sha(read(p,false))===h);}
const closure=json('p1338-closure.json'),final=json('p1338-review-final.json'),base=json('p1338-baseline.json'),m=json('p1338-manifest-r1.json');
ck('closure_pin',inputs[D+'p1338-closure.json']==='38bbe304bc2762bc986e1492859dbfcd7bb92e7a1aa53058f5ef8466ffddfda9');
ck('final_pin',inputs[D+'p1338-review-final.json']==='34b440ff8d68ef792ddb42d8e7cd243449b6705c5a8306e2e4bd6aa471f5a3c4');
ck('final_verdict',final.verdict==='PASS_SCOPED'&&!final.violations.length&&!final.unknowns.length);
ck('closure_after_final',Date.parse(closure.at)>Date.parse(final.at));
ck('closure_base_manifest',closure.baseline_sha256===inputs[D+'p1338-baseline.json']&&closure.manifest_sha256===inputs[D+'p1338-manifest-r1.json']);
map('final_inputs',final.inputs);
map('closure_artifacts',closure.artifacts);
map('historical',closure.historical_preserved);
map('prior_temporaries',closure.retained_prior_artifacts);
map('mutant_retained',closure.retained_mutant_artifacts);
map('prior_exports',closure.predecessor_active_exports_preserved);
map('product_state',closure.state.product_inventory);
ck('historical_map_exact',eq(closure.historical_preserved,base.historical_preserved));
ck('temporary_map_exact',eq(closure.retained_prior_artifacts,base.retained_prior_artifacts));
ck('changed_pair_only',eq(closure.changed_product_paths,[m.prompt.path,m.source].sort()));
const expectedChanges=Object.entries(base.state.product_inventory).filter(([p,h])=>closure.state.product_inventory[p]!==h).map(([p])=>p).sort();
ck('changed_pair_recomputed',eq(expectedChanges,closure.changed_product_paths));
const actual=[...new Set(fs.readFileSync(0,'utf8').split('\0').filter(Boolean))].sort();
ck('current_inventory_exact',eq(actual,Object.keys(closure.state.product_inventory).sort()));
const gitHead=read(R+'/.git/HEAD',false).toString().trim();
const currentHead=gitHead.startsWith('ref: ')?read(R+'/.git/'+gitHead.slice(5),false).toString().trim():gitHead;
ck('current_HEAD',currentHead===closure.state.head&&currentHead===base.state.head);
ck('branch_staged',closure.state.branch===base.state.branch&&closure.state.staged===base.state.staged&&closure.state.staged==='');
ck('no_stage_commit',closure.staged===false&&closure.committed===false);
ck('no_normative_change_no_global_claim',closure.normative_intention_changed_after_effective_freeze===false&&closure.global_language_parity_claim===false);
ck('regime_unchanged',closure.regime===m.regime);
ck('closure_lineage',closure.lineage.effective_a===final.lineage.A&&closure.lineage.code_b===final.lineage.B&&closure.lineage.recorded_a===final.lineage.A.slice(0,8)&&closure.lineage.recorded_b===final.lineage.B.slice(0,8));
ck('closure_workspace',eq(closure.workspace,final.workspace));
ck('closure_ab',eq(closure.ab_counts,final.ab.classes));
ck('four_mutants',closure.productive_mutants_rejected===4);
ck('closing_algorithm',closure.closure_algorithm_sha256===final.inputs[D+'p1338-close.py']);
for(const b of Object.values(closure.binaries))ck('binary:'+b.path,sha(read(b.path))===b.sha256);
ck('prior_closure_pin',sha(read(D+'p1337-closure.json'))===closure.previous_closure_sha256&&closure.previous_closure_sha256===base.previous_closure_sha256);
ck('step_metadata_only',typeof closure.step.path==='string'&&/^[0-9a-f]{64}$/.test(closure.step.sha256));
read(__filename);
console.log(JSON.stringify({at:new Date().toISOString(),phase:'postclosure',verdict:violations.length||unknowns.length?'HOLD':'PASS_SCOPED',violations,unknowns,checks,counts,head:currentHead,step:{...closure.step,verification:'Root-reported metadata only; reviewer did not read or hash the step.'},limitations:'No product gates or retained test binaries rerun. Closure, final inputs, inventories and retained artifacts verified read-only; all scoped final limitations remain.',inputs},null,2));
if(violations.length||unknowns.length)process.exitCode=1;
