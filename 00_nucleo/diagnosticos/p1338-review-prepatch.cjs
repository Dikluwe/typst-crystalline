// Read-only independent freeze/RED auditor; publish stdout separately.
const fs=require('fs'),crypto=require('crypto'),path=require('path');
const R='/repos/Antigravity/typst-crystalline',D=R+'/00_nucleo/diagnosticos/';
const inputs={},violations=[],unknowns=[];let checks=0;
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
function read(p){p=path.resolve(R,p);if(/00_nucleo\/(materialization|context)(\/|$)/.test(p))throw Error('restricted');const b=fs.readFileSync(p);inputs[p]=sha(b);return b;}
const json=n=>JSON.parse(read(D+n));
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function ck(n,v){checks++;if(!v)violations.push(n);}
const sig=r=>({exit:r.exit,stdout_base64:r.stdout_base64,stderr_base64:r.stderr_base64});
const m=json('p1338-manifest-r1.json'),f=json('p1338-tests-freeze.json'),af=json('p1338-attacks-freeze.json'),link=json('p1338-attacks-freeze-tests.json');
ck('manifest_pin',inputs[D+'p1338-manifest-r1.json']==='14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843');
ck('test_freeze_pin',inputs[D+'p1338-tests-freeze.json']==='bd9f6a580251ba5dce783770657165354ab427736ceb4d2a4b87506958be04e3');
ck('attack_freeze_pin',inputs[D+'p1338-attacks-freeze.json']==='0f487a9d3825bd675b86f39002e99e6dfca1e918f4752db4c25859e3e034dcc0');
ck('attack_link_pin',inputs[D+'p1338-attacks-freeze-tests.json']==='b1f3db154085558185a33e50b2662ee9873549e53d3a1197983098561b7f641e');
for(const receipt of [f,af,link])ck('receipt_manifest',receipt.manifest_sha256===inputs[D+'p1338-manifest-r1.json']);
for(const map of [f.protected,af.inputs,link.inputs])for(const[n,h]of Object.entries(map))ck('freeze:'+n,sha(read(D+n))===h);
const mod=read(D+'p1338-tests-module.rs.txt').toString(),patch=json('p1338-tests-local-patch.json');
const oldName='p1337_preserve_array_ast_message_and_total_span_debt';
const oldCall='        check_ast(\n            "#((1, 2).missing)",\n            "(1, 2).missing",\n            "array does not contain field \\"missing\\"",\n        );';
const newCall='        check_ast("#((1, 2).missing)", "missing", "cannot access fields on type array");';
let expected=m.pre_candidate_source;
ck('module_anchor_unique',expected.split('#[cfg(test)]\nmod p1337_tests {').length===2);
ck('legacy_name_unique',expected.split(oldName).length===2);
ck('legacy_call_unique',expected.split(oldCall).length===2);
expected=expected.replace('#[cfg(test)]\nmod p1337_tests {',mod+'\n#[cfg(test)]\nmod p1337_tests {').replace(oldName,'p1338_successor_array_missing_message_and_field_span').replace(oldCall,newCall);
ck('source_reconstruction_pin',sha(expected)===patch.tests_only_after_sha256);
ck('source_reconstruction_current',read(m.source).toString()===expected);
ck('old_comment_preserved',expected.includes('// Array is an excluded baseline debt, not a P1337 parity correction.'));
ck('pre_tests_source_pin',patch.before_sha256===m.source_sha256);
ck('pre_freeze_fmt',patch.fmt.exit===0&&eq(patch.fmt.argv,['cargo','fmt','--all','--','--check']));
const l0=read(m.prompt.path).toString(),norm=l0.replace(/^Hash do Código: [0-9a-f]{8}\n/m,'');
ck('normative_L0_frozen',norm===m.prompt.normative_text&&sha(norm)===m.prompt.normative_sha256);
ck('module_six_tests',(mod.match(/#\[test\]/g)||[]).length===6&&mod.startsWith('#[cfg(test)]\nmod p1338_tests {'));
const discriminators={M1:'p1338_pure_missing_array_fields_message_and_received_span',M2:'p1338_ast_empty_heterogeneous_alias_unicode_multiline',M3:'p1338_pure_preserve_len_first_last_and_empty_none',M4:'p1338_preserve_other_categories_diagnostics'};
for(const[id,name]of Object.entries(discriminators))ck('selected_discriminator:'+id,mod.includes('fn '+name+'('));
ck('message_and_pure_span_assertions',mod.includes('assert_eq!(d.message, "cannot access fields on type array")')&&mod.includes('assert_eq!(d.span, span)'));
ck('AST_exact_byte_range_assertion',mod.includes('assert_eq!(world.source.span_byte_range(d.span), expected, "{text}")'));
ck('different_first_last_values',mod.includes('Value::Str("primeiro".into()), Value::Bool(false), Value::Int(83)'));
ck('Length_integral_span_control',mod.includes('("#((1pt).nope)", "(1pt).nope",'));
const base=json('p1338-tests-baseline.json'),cases=json('p1338-tests-cases.json'),ex=json('p1338-tests-expectations.json');
ck('baseline_freeze_pin',inputs[D+'p1338-tests-baseline.json']===f.baseline_runs_sha256);
ck('expectations_freeze_pin',inputs[D+'p1338-tests-expectations.json']===f.expectations_sha256);
ck('baseline_protected',eq(base.protected,f.protected));
const declarations=new Map(cases.map(c=>[c.id,c])),keys=new Map(),classes={};
ck('unique_cases',declarations.size===45&&cases.length===45);
for(const[p,b]of Object.entries(base.products)){ck('binary:'+p,sha(read(b.path))===b.sha256);ck('binary_manifest:'+p,eq(b,p==='vanilla'?m.vanilla:m.baseline_binary));}
for(const r of base.rows){const k=[r.id,r.profile,r.product,r.order].join('|');ck('unique:'+k,!keys.has(k));keys.set(k,r);const c=declarations.get(r.id);ck('source:'+k,!!c&&r.source===c.expr&&r.source_sha256===sha(c.expr)&&r.policy===c.policy);let observed=!r.failure&&[0,1].includes(r.exit)&&r.observation==='Observed';for(const ch of ['stdout','stderr']){try{observed&&=new TextDecoder('utf-8',{fatal:true}).decode(Buffer.from(r[ch+'_base64'],'base64'))===r[ch];}catch{observed=false;}}if(r.exit===0){try{JSON.parse(r.stdout);}catch{observed=false;}}else observed&&=!r.stdout&&r.stderr.includes('error: ')&&!r.stderr.includes('panicked at');ck('observed:'+k,observed);if(!observed)unknowns.push(k);ck('binary_row:'+k,r.binary_sha256===base.products[r.product]?.sha256&&r.argv[0]===base.products[r.product]?.path);ck('times:'+k,Date.parse(r.at)<=Date.parse(r.end)&&Date.parse(r.end)<=Date.parse(f.at));}
for(const c of cases)for(const p of m.policy.profiles)for(const product of ['baseline','vanilla'])for(const order of m.policy.orders){const k=[c.id,p,product,order].join('|'),r=keys.get(k),n=keys.get([c.id,p,product,'normal'].join('|'));ck('complete_stable:'+k,!!r&&!!n&&eq(sig(r),sig(n)));}
const eks=new Set();
for(const e of ex){const k=[e.id,e.profile].join('|');ck('expectation_unique:'+k,!eks.has(k));eks.add(k);const c=declarations.get(e.id),b=keys.get([e.id,e.profile,'baseline','normal'].join('|')),v=keys.get([e.id,e.profile,'vanilla','normal'].join('|'));const same=eq(sig(b),sig(v));const expectedClass=c.policy==='converge'?'CONVERGENCE_REQUIRED':same?'PRESERVE_PARITY':'PRESERVE_EXISTING_DEBT';ck('class:'+k,e.policy===c.policy&&e.baseline_class===expectedClass);ck('expected_channels:'+k,eq(e.baseline,sig(b))&&eq(e.vanilla,sig(v))&&eq(e.expected,sig(c.policy==='converge'?v:b)));if(c.policy==='converge')ck('genuine_baseline_delta:'+k,!same&&b.exit===1&&v.exit===1&&v.stderr.includes('cannot access fields on type array'));classes[expectedClass]=(classes[expectedClass]||0)+1;}
ck('matrix_cardinality',base.rows.length===1080&&keys.size===1080&&ex.length===180&&eks.size===180);
ck('classes',eq(classes,{CONVERGENCE_REQUIRED:40,PRESERVE_EXISTING_DEBT:40,PRESERVE_PARITY:100}));
ck('harness_controls',base.harness_tests.length===6&&base.harness_tests.every(t=>t.actual===t.expected));
read(D+'p1338-tests-operational-sequence.md');read(D+'p1338-tests-review-note.md');read(D+'p1338-review-manifest-r1.json');
let red=null;
if(fs.existsSync(D+'p1338-unit-red.json')){red=json('p1338-unit-red.json');ck('red_exit',red.exit===101);ck('red_filter',red.argv.includes('p1338_')&&red.argv.includes('--release'));ck('red_source_before_after',red.before.product_inventory[m.source]===sha(expected)&&red.after.product_inventory[m.source]===sha(expected));ck('red_after_freeze',Date.parse(red.at)>Date.parse(f.at)&&Date.parse(red.at)>Date.parse(link.at));ck('red_compiled_executed',red.stderr.includes('Finished `release`')&&red.stderr.includes('Running unittests')&&red.stdout.includes('running 7 tests')&&!/error\[E\d+\]/.test(red.stderr));ck('red_assertions',red.stdout.includes('test result: FAILED. 4 passed; 3 failed; 0 ignored')&&(red.stdout+red.stderr).includes('assertion `left == right` failed'));}
if(red){ck('red_receipt_pin',inputs[D+'p1338-unit-red.json']==='5ea6275f6f7f163a651fc043313a3020c8cd69ada0c77201479f84982a91e9ec');ck('red_three_assertion_failures',(red.stdout.match(/assertion `left == right` failed/g)||[]).length===3);ck('red_explicit_compilation',red.stderr.includes('Compiling typst-core v0.1.0 ('+R+'/01_core)'));}
read(__filename);
console.log(JSON.stringify({at:new Date().toISOString(),phase:'prepatch',verdict:violations.length||unknowns.length?'HOLD':red?'GO_PREPATCH_SCOPED':'FREEZE_VERIFIED_WAIT_RED',checks,violations,unknowns,inputs,source_tests_only_sha256:sha(expected),ab:{cases:cases.length,runs:base.rows.length,expectations:ex.length,classes},discriminators,red:red?{exit:red.exit,summary:red.stdout.match(/test result:.*/g),sha256:inputs[D+'p1338-unit-red.json']}:null,limitations:['No technical isolation attestation or refinement seal; prior review context retained.','Mutation discrimination is structural before C, not a kill claim; paired instrumental profile must compile and execute later.','Frozen debt preservation is not parity; no general Array/fields/LocatedContent/PDF/accessibility/warning claim.','Transient integration/removal before manifest R1 and legacy-call mechanical formatting are documented; only authorized sentinel name/message/anchor changed.']},null,2));
