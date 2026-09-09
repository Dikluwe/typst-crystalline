// Read-only postclosure verification. Does not execute product or closure scripts.
const fs=require('fs'),crypto=require('crypto'),path=require('path');
const R='/repos/Antigravity/typst-crystalline',D=R+'/00_nucleo/diagnosticos/';
const inputs={},violations=[],checks=[];
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
function read(p,pin=true){p=path.resolve(R,p);if(/00_nucleo\/(materialization|context)(\/|$)/.test(p))throw Error('forbidden read');const b=fs.readFileSync(p);if(pin)inputs[p]=sha(b);return b;}
const json=n=>JSON.parse(read(D+n));
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function ck(n,ok){checks.push({name:n,ok:!!ok});if(!ok)violations.push(n);}
const closure=json('p1337-closure.json'),final=json('p1337-review-final.json'),base=json('p1337-baseline.json'),m=json('p1337-manifest-r1.json');
ck('closure_pin',inputs[D+'p1337-closure.json']==='04d5c3cc470025fd10062861b4429873fa56f2c881a3c7c4bcea911fc5a6015e');
ck('final_verdict_pin',inputs[D+'p1337-review-final.json']==='abc90b9e1e8cc32585a1e101dfefbaf50ddea7e390f075af609eef74198e1242');
ck('final_clean',final.verdict==='PASS_SCOPED'&&!final.violations.length&&!final.unknowns.length);
ck('baseline_chain',closure.baseline_sha256===inputs[D+'p1337-baseline.json']&&closure.manifest_sha256===inputs[D+'p1337-manifest-r1.json']&&closure.previous_closure_sha256===base.previous_closure_sha256);
ck('scope_exact',eq(closure.changed_product_paths,[m.prompt.path,m.source].sort())&&closure.outcome==='P1337_SCOPED_BOOL_NONE_AUTO_DIAGNOSTIC_CORRECTION');
ck('no_expanded_claim',!closure.normative_intention_changed_after_freeze&&!closure.global_language_parity_claim&&!closure.staged&&!closure.committed);
ck('totals_match_review',eq(closure.workspace,final.workspace)&&eq(closure.ab_counts,final.ab.classes)&&closure.productive_mutants_rejected===5);
ck('lineage_match_review',closure.lineage.effective_a===final.lineage.A&&closure.lineage.code_b===final.lineage.B&&closure.lineage.recorded_a===final.lineage.A.slice(0,8)&&closure.lineage.recorded_b===final.lineage.B.slice(0,8));
ck('historical_inventory_unchanged',eq(closure.historical_preserved,base.historical_preserved)&&eq(closure.retained_prior_artifacts,base.retained_prior_artifacts));
let counts={};
for(const [label,map]of Object.entries({review_inputs:final.inputs,closure_artifacts:closure.artifacts,history:closure.historical_preserved,prior_temporaries:closure.retained_prior_artifacts,mutant_artifacts:closure.retained_mutant_artifacts,prior_exports:closure.predecessor_active_exports_preserved,product_state:closure.state.product_inventory})){
  counts[label]=Object.keys(map).length;
  for(const [p,h]of Object.entries(map))ck(label+':'+p,sha(read(p,false))===h);
}
for(const [k,v]of Object.entries(closure.binaries))ck('binary:'+k,sha(read(v.path))===v.sha256);
ck('algorithm_pin',sha(read(D+'p1337-close.py'))===closure.closure_algorithm_sha256&&closure.closure_algorithm_sha256===final.inputs[D+'p1337-close.py']);
ck('head_stage_recorded',closure.state.head===base.state.head&&closure.state.branch===base.state.branch&&closure.state.staged===base.state.staged);
ck('step_receipt_wellformed',typeof closure.step.path==='string'&&/^[a-f0-9]{64}$/.test(closure.step.sha256));
read(__filename);
console.log(JSON.stringify({at:new Date().toISOString(),phase:'postclosure',verdict:violations.length?'BLOCKED':'PASS_SCOPED',violations,unknowns:[],checks:checks.length,counts,head:closure.state.head,step:{...closure.step,verification:'Field recorded by root only; tactical file was not read or rehashed by reviewer.'},limitations:'No product gates rerun. Closure and final evidence verified read-only; A/B scope and all final limitations remain unchanged.',inputs},null,2));
if(violations.length)process.exitCode=1;
