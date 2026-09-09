// Independent read-only audit. Emits evidence to stdout; never edits judged inputs.
const fs = require('fs');
const crypto = require('crypto');
const path = require('path');
const ROOT = '/repos/Antigravity/typst-crystalline';
const D = ROOT + '/00_nucleo/diagnosticos/';
const inputs = {}, checks = [], violations = [];
const sha = x => crypto.createHash('sha256').update(x).digest('hex');
function read(p) {
  p = path.resolve(ROOT, p);
  if (/00_nucleo\/(materialization|context)(\/|$)/.test(p)) throw Error('forbidden read');
  const b = fs.readFileSync(p);
  if(p.includes('/p1337-') || p.endsWith('/field_access.rs') || p.endsWith('/field_access.md') || p.endsWith('/release/typst') || p==='/usr/local/bin/typst') inputs[p] = sha(b);
  return b;
}
function json(n) { return JSON.parse(read(D+n)); }
function check(name, ok, detail) { checks.push({name, ok, ...(detail === undefined ? {} : {detail})}); if(!ok) violations.push(name); }
const m=json('p1337-manifest-r1.json'), r0=json('p1337-manifest.json'), b=json('p1337-baseline.json');
check('manifest_pin', inputs[D+'p1337-manifest-r1.json']==='7c09595cde021b89fffa7c316b7706797ba3898aa5b6687cdfaf2deacb0e65ff');
check('baseline_pin',inputs[D+'p1337-baseline.json']===m.baseline_sha256);
check('manifest_capability_only', ['regime','baseline_sha256','measurement_sha256','prompt','source','source_sha256','pre_candidate_source','state','target','baseline_binary','vanilla','policy'].every(k=>JSON.stringify(m[k])===JSON.stringify(r0[k])) && Object.keys(m.roles).filter(k=>JSON.stringify(m.roles[k])!==JSON.stringify(r0.roles[k])).join()==='reviewer');
const f=json('p1337-tests-freeze.json'), af=json('p1337-attacks-freeze.json');
const successor=process.argv.includes('--r1')||process.argv.includes('--r2')?json('p1337-tests-freeze-r1.json'):null;
const formatted=process.argv.includes('--r2')?json('p1337-tests-freeze-r2.json'):null;
if(successor) {
  check('successor_original_freeze_pin', successor.predecessor_freeze_sha256===inputs[D+'p1337-tests-freeze.json']);
  check('successor_cli_freeze_unchanged',JSON.stringify(successor.protected)===JSON.stringify(f.protected)&&successor.baseline_runs_sha256===f.baseline_runs_sha256&&successor.expectations_sha256===f.expectations_sha256);
  for(const [n,h] of Object.entries(successor.protected_successors))check('successor_pin:'+n,sha(read(D+n))===h);
}
if(formatted) {
  check('format_successor_chain',formatted.predecessor_freeze_sha256===inputs[D+'p1337-tests-freeze-r1.json']);
  check('format_cli_unchanged',JSON.stringify(formatted.protected)===JSON.stringify(f.protected)&&formatted.baseline_runs_sha256===f.baseline_runs_sha256&&formatted.expectations_sha256===f.expectations_sha256);
  for(const [n,h] of Object.entries(formatted.protected_successors))check('format_successor_pin:'+n,sha(read(D+n))===h);
  const af2=json('p1337-attacks-freeze-r2.json');
  for(const [n,h]of Object.entries(af2.inputs))check('attacks_r2_pin:'+n,sha(read(path.isAbsolute(n)?n:D+n))===h);
  const red=json('p1337-unit-red-r1.json'), redPatch=json('p1337-tests-local-patch-r1.json');
  check('red_real_assertions',red.exit===101&&red.stdout.includes('test result: FAILED. 2 passed; 3 failed; 0 ignored')&&(red.stdout.match(/assertion `left == right` failed/g)||[]).length===3&&red.stderr.includes('Compiling typst-core v0.1.0 (/repos/Antigravity/typst-crystalline/01_core)')&&red.stderr.includes('Finished `release`'));
  check('red_source_stable',red.before.product_inventory[m.source]===redPatch.tests_only_after_sha256&&red.after.product_inventory[m.source]===redPatch.tests_only_after_sha256&&formatted.red_r1_sha256===inputs[D+'p1337-unit-red-r1.json']);
}
for(const [n,h] of Object.entries(f.protected)) check('test_pin:'+n,sha(read(D+n))===h);
for(const [p,h] of Object.entries(af.inputs)) check('attack_pin:'+p,sha(read(p))===h);
check('baseline_runs_pin',sha(read(D+'p1337-tests-baseline.json'))===f.baseline_runs_sha256);
check('expectations_pin',sha(read(D+'p1337-tests-expectations.json'))===f.expectations_sha256);
check('freezes_match_manifest',f.manifest_sha256===inputs[D+'p1337-manifest-r1.json']&&af.manifest_sha256===f.manifest_sha256);
const s=read(m.source).toString(), moduleText=read(D+(formatted?'p1337-tests-module-r2.rs.txt':successor?'p1337-tests-module-r1.rs.txt':'p1337-tests-module.rs.txt')).toString();
if(successor) {
  const original=read(D+'p1337-tests-module.rs.txt').toString();
  const r1Module=read(D+'p1337-tests-module-r1.rs.txt').toString();
  const addition=r1Module.match(/    #\[test\]\n    fn p1337_preserve_array_ast_message_and_total_span_debt\(\) \{[\s\S]*?\n    \}\n\n/);
  check('successor_only_array_witness_added',!!addition&&r1Module.replace(addition[0],'')===original);
  if(formatted) {
    const tokens=t=>t.match(/"(?:[^"\\]|\\.)*"|\/\/[^\n]*|\w+|[^\s]/g);
    const normalize=t=>tokens(t).filter((v,i,a)=>!(v===','&&a[i+1]===')'));
    check('format_token_sequence_preserved',JSON.stringify(normalize(r1Module))===JSON.stringify(normalize(moduleText)));
    check('format_literal_sequence_preserved',JSON.stringify(r1Module.match(/"(?:[^"\\]|\\.)*"/g))===JSON.stringify(moduleText.match(/"(?:[^"\\]|\\.)*"/g)));
    check('format_only_two_optional_commas',tokens(moduleText).length===tokens(r1Module).length+2);
    const redPatch=json('p1337-tests-local-patch-r1.json');
    check('format_all_other_source_bytes_preserved',sha(s.replace(moduleText,r1Module))===redPatch.tests_only_after_sha256);
  }
}
let restored=s.replace(moduleText+'\n','').replace('p1337_successor_bool_none_auto_lookup_names_and_ast_span','p1336_preserve_other_fallback_names_and_ast_span');
for(const [kind,oldName] of [['true','bool'],['none','none'],['auto','auto']]) restored=restored.replace(`("#(${kind}.nope)", "nope", "cannot access fields on type ${kind==='true'?'boolean':kind}")`,`("#(${kind}.nope)", "${kind}.nope", "cannot access fields on type ${oldName}")`);
check('tests_only_reconstructs_exact_baseline',restored===m.pre_candidate_source && sha(restored)===m.source_sha256);
check('single_exact_new_module',s.split(moduleText).length===2);
const stripB=x=>x.replace(/^Hash do Código: [a-f0-9]{8}\r?\n/m,'');
const l0=read(m.prompt.path).toString();
check('normative_prompt_unchanged',stripB(l0)===m.prompt.normative_text && sha(stripB(l0))===m.prompt.normative_sha256);
let protectedCount=0;
for(const [p,h] of Object.entries(b.state.product_inventory)) {
  if(m.policy.allowed_product_changes.includes(p)) continue;
  check('protected_product:'+p,sha(read(p))===h); protectedCount++;
}
let historicalCount=0,retainedCount=0;
for(const [p,h] of Object.entries(b.historical_preserved)) {check('historical:'+p,sha(read(p))===h); historicalCount++;}
for(const [p,h] of Object.entries(b.retained_prior_artifacts)) {check('retained:'+p,sha(read(p))===h); retainedCount++;}
for(const p of [m.vanilla,m.baseline_binary]) check('binary:'+p.path,sha(read(p.path))===p.sha256);
const runs=json('p1337-tests-baseline.json'), cases=json('p1337-tests-cases.json'), expectations=json('p1337-tests-expectations.json');
const signature=r=>JSON.stringify({exit:r.exit,stdout_base64:r.stdout_base64,stderr_base64:r.stderr_base64});
const keys=new Map(), declared=new Map(cases.map(c=>[c.id,c]));
for(const r of runs.rows) {
  const key=[r.id,r.profile,r.product,r.order].join('|');check('unique:'+key,!keys.has(key));keys.set(key,r);
  check('observed:'+key,!r.failure&&[0,1].includes(r.exit)&&r.observation==='Observed'&&['stdout','stderr'].every(ch=>Buffer.from(r[ch+'_base64'],'base64').toString('utf8')===r[ch])&&r.source===declared.get(r.id).expr&&sha(r.source)===r.source_sha256);
}
let classes={};
for(const c of cases) for(const profile of m.policy.profiles) {
  const get=(product,order)=>keys.get([c.id,profile,product,order].join('|'));
  for(const product of ['vanilla','baseline']) for(const order of m.policy.orders) check('order:'+c.id+profile+product+order,!!get(product,order)&&signature(get(product,order))===signature(get(product,'normal')));
  const e=expectations.find(e=>e.id===c.id&&e.profile===profile),base=get('baseline','normal'),vanilla=get('vanilla','normal');
  const kind=c.policy==='converge'?'CONVERGENCE_REQUIRED':signature(base)===signature(vanilla)?'PRESERVE_PARITY':'PRESERVE_EXISTING_DEBT';
  check('frozen_expectation:'+c.id+profile,!!e&&e.baseline_class===kind&&JSON.stringify(e.expected)===signature(c.policy==='converge'?vanilla:base)&&JSON.stringify(e.baseline)===signature(base)&&JSON.stringify(e.vanilla)===signature(vanilla));
  classes[kind]=(classes[kind]||0)+1;
}
check('matrix_cardinality',cases.length===45&&runs.rows.length===45*4*2*3&&expectations.length===45*4&&keys.size===runs.rows.length);
const result={at:new Date().toISOString(),status:violations.length?'BLOCKED':'PREFLIGHT_INTEGRITY_PASS_NOT_PATCH_GO',reviewer:'/root/p1319_review',context:'Retained prior independent review context, no P1337 implementation/test authorship; no technical isolation attestation or refinement seal',checks:checks.length,violations,unknowns:[],counts:{protectedCount,historicalCount,retainedCount,runs:runs.rows.length,classes},inputs};
console.log(JSON.stringify(result,null,2));
if(violations.length)process.exitCode=1;
