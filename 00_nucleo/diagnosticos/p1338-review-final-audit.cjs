// Independent P1338 read-only final audit. Publish stdout separately with apply_patch.
const fs=require('fs'),crypto=require('crypto'),path=require('path');
const R='/repos/Antigravity/typst-crystalline',D=R+'/00_nucleo/diagnosticos/';
const inputs={},violations=[],unknowns=[];let checks=0;
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function read(p,pin=true){p=path.resolve(R,p);if(/00_nucleo\/(materialization|context)(\/|$)/.test(p))throw Error('restricted');const b=fs.readFileSync(p);if(pin)inputs[p]=sha(b);return b;}
const json=n=>JSON.parse(read(D+n));
function ck(n,v){checks++;if(!v)violations.push(n);}
const sig=r=>({exit:r.exit,stdout_base64:r.stdout_base64,stderr_base64:r.stderr_base64});
const m=json('p1338-manifest-r1.json'),b=json('p1338-baseline.json'),go=json('p1338-review-prepatch-go.json'),c=json('p1338-candidate.json');
ck('manifest_pin',inputs[D+'p1338-manifest-r1.json']==='14a36a87a116dee46464c32d358f3019702c201ea74809c90cc8bf81cdeeb843');
ck('baseline_pin',inputs[D+'p1338-baseline.json']===m.baseline_sha256);
ck('candidate_pin',inputs[D+'p1338-candidate.json']==='5e6a134b93d9b856333ba7399617a6fb4d44b650be7775833814dc249bc94cbd');
ck('go_scope',go.verdict==='GO_PREPATCH_SCOPED'&&!go.violations.length&&!go.unknowns.length);
ck('chronology',Date.parse(c.at)>Date.parse(go.at));
for(const[p,h]of Object.entries(go.inputs))if(![R+'/'+m.source,R+'/'+m.prompt.path].includes(p))ck('prepatch_input:'+p,sha(read(p))===h);
for(const name of ['red_review','attacks_tests_freeze'])ck('candidate_pin:'+name,sha(read(c[name].path))===c[name].sha256);
for(const name of ['tests_freeze','attacks_freeze'])ck('candidate_pin:'+name,sha(read(c[name+'_path']))===c[name+'_sha256']);
ck('candidate_red',sha(read(c.red_path))===c.red_sha256);
const mod=read(D+'p1338-tests-module.rs.txt').toString(),patch=json('p1338-tests-local-patch.json');
const oldCall='        check_ast(\n            "#((1, 2).missing)",\n            "(1, 2).missing",\n            "array does not contain field \\"missing\\"",\n        );';
let red=m.pre_candidate_source.replace('#[cfg(test)]\nmod p1337_tests {',mod+'\n#[cfg(test)]\nmod p1337_tests {').replace('p1337_preserve_array_ast_message_and_total_span_debt','p1338_successor_array_missing_message_and_field_span').replace(oldCall,'        check_ast("#((1, 2).missing)", "missing", "cannot access fields on type array");');
ck('reconstruct_RED',sha(red)===patch.tests_only_after_sha256&&sha(red)===go.source_tests_only_sha256);
const guard='            | Value::Auto\n',oldMessage='                format!("array does not contain field \\"{field}\\""),\n';
ck('unique_product_anchors',red.split(guard).length===2&&red.split(oldMessage).length===2);
const expected=red.replace(guard,guard+'            | Value::Array(_)\n').replace(oldMessage,'                "cannot access fields on type array",\n');
const src=read(m.source).toString(),l0=read(m.prompt.path).toString();
ck('exact_two_product_changes_all_tests_preserved',src===expected);
ck('candidate_bytes',src===c.source&&sha(src)===c.source_sha256);
const norm=l0.replace(/^Hash do Código: [0-9a-f]{8}\n/m,'');
ck('normative_L0_frozen',norm===m.prompt.normative_text&&sha(norm)===m.prompt.normative_sha256);
const refs=[...l0.matchAll(/^- (\S+) sha256:([a-f0-9]{64})$/gm)];
ck('one_known_nucleus',refs.length===1&&refs[0][1]==='00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml');
const nb=read(refs[0][1]);ck('leaf_nucleus',!nb.toString().includes('[[dependencies]]')&&!nb.toString().includes('[[depends]]'));
const nh=sha(Buffer.concat([nb,Buffer.from('\0TEKT-NUCLEUS-DEPS-V1\0')]));ck('effective_nucleus_pin',nh===refs[0][2]);
const frame=Buffer.alloc(8);frame.writeBigUInt64BE(BigInt(Buffer.byteLength(refs[0][1])));
const A=sha(Buffer.concat([Buffer.from(norm),Buffer.from('\0TEKT-PROMPT-NUCLEI-V1\0'),frame,Buffer.from(refs[0][1]+' '),Buffer.from(nh,'hex')]));
const B=sha(src.replace(/^\/\/! @prompt-hash [a-f0-9]{8}\n/m,''));
ck('independent_A',src.match(/^\/\/! @prompt-hash (\w+)$/m)[1]===A.slice(0,8));
ck('independent_B',l0.match(/^Hash do Código: (\w+)$/m)[1]===B.slice(0,8));
for(const p of ['/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs','/repos/Antigravity/tekt-linter/03_infra/nucleus.rs','/repos/Antigravity/tekt-linter/03_infra/prompt_io.rs'])read(p);
const reciprocal=json('p1338-lineage-reciprocal.json');
ck('reciprocal_metadata_only',reciprocal.before.recorded_a===reciprocal.after.recorded_a&&reciprocal.before.norm_sha256===reciprocal.after.norm_sha256&&reciprocal.after.effective_a===A&&reciprocal.after.code_b===B&&reciprocal.after.recorded_b===B.slice(0,8));
read(D+'p1338-reseal-final.json');
const preserved={product:0,historical:0,temporaries:0};
for(const[p,h]of Object.entries(b.state.product_inventory))if(!m.policy.allowed_product_changes.includes(p)){ck('product:'+p,sha(read(p,false))===h);preserved.product++;}
for(const [key,label]of [['historical_preserved','historical'],['retained_prior_artifacts','temporaries']])for(const[p,h]of Object.entries(b[key])){ck(label+':'+p,sha(read(p,false))===h);preserved[label]++;}
const gates={};
for(const name of ['unit-green','candidate-build','workspace-tests','fmt','diff-check','lineage-final','lint-general']){const g=json('p1338-'+name+'.json');gates[name]={exit:g.exit,argv:g.argv};ck('gate_exit:'+name,g.exit===0&&g.manifest_sha256===inputs[D+'p1338-manifest-r1.json']);for(const moment of ['before','after']){ck('gate_source:'+name+moment,g[moment].product_inventory[m.source]===sha(src));ck('gate_HEAD_stage:'+name+moment,g[moment].head===b.state.head&&g[moment].branch===b.state.branch&&g[moment].staged===b.state.staged);for(const[p,h]of Object.entries(b.state.product_inventory))if(!m.policy.allowed_product_changes.includes(p))ck('gate_preserve:'+name+moment+p,g[moment].product_inventory[p]===h);}}
ck('strict_lineage_command',gates['lineage-final'].argv.includes('v5,v15,v26')&&gates['lineage-final'].argv.includes('--fail-on')&&gates['lineage-final'].argv.includes('warning'));
const green=json('p1338-unit-green.json');ck('focal_GREEN',green.stdout.includes('test result: ok. 7 passed; 0 failed; 0 ignored'));
const workspace=json('p1338-workspace-tests.json');
const totals=[...workspace.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)].reduce((a,x)=>a.map((v,i)=>v+Number(x[i+1])),[0,0,0]);
ck('workspace_totals',eq(totals,[6749,0,3]));
ck('LocatedContent_antecedent_executed',workspace.stdout.includes('p1325_located_ast_preserves_whole_access_for_snapshot_and_legacy ... ok'));
const lintParts=n=>json(n).stdout.split(/(?=^(?:warning|info|error):)/m).filter(s=>/^(?:warning|info|error):/.test(s)).map(s=>s.replace(/(--> [^\n]+?):\d+(?::\d+)?/g,'$1:<line>').trim());
const oldLint=lintParts('p1337-lint-general.json'),newLint=lintParts('p1338-lint-general.json');
const multiset=a=>{const m=new Map();for(const x of a)m.set(x,(m.get(x)||0)+1);return m;};
const lm=multiset(oldLint),ln=multiset(newLint),removed=[],added=[];
for(const k of new Set([...lm.keys(),...ln.keys()])){const delta=(ln.get(k)||0)-(lm.get(k)||0);if(delta<0)removed.push({count:-delta,diagnostic:k});if(delta>0)added.push({count:delta,diagnostic:k});}
ck('lint_warning_multiset',eq(oldLint.filter(x=>x.startsWith('warning:')).sort(),newLint.filter(x=>x.startsWith('warning:')).sort()));
ck('lint_error_zero',newLint.filter(x=>x.startsWith('error:')).length===0);
ck('lint_entire_multiset_preserved',removed.length===0&&added.length===0);
const lint={counts:Object.fromEntries(['warning','info','error'].map(k=>[k,newLint.filter(x=>x.startsWith(k+':')).length])),removed,added,normalization:'Only reported source location line/column removed; paths/messages/snippets/multiplicity remain.'};
const runs=json('p1338-tests-candidate.json'),cases=json('p1338-tests-cases.json'),ex=json('p1338-tests-expectations.json'),comparison=json('p1338-tests-comparison.json');
const declared=new Map(cases.map(c=>[c.id,c])),expect=new Map(ex.map(e=>[[e.id,e.profile].join('|'),e])),keys=new Map(),classes={};
const binary=json('p1338-candidate-build.json').candidate_binary;ck('candidate_binary_receipt',eq(binary,runs.products.candidate));
for(const p of [binary,m.baseline_binary,m.vanilla])ck('binary:'+p.path,sha(read(p.path))===p.sha256);
for(const r of runs.rows){const k=[r.id,r.profile,r.order].join('|');ck('candidate_unique:'+k,!keys.has(k));keys.set(k,r);let observed=!r.failure&&[0,1].includes(r.exit)&&r.observation==='Observed';for(const ch of ['stdout','stderr']){try{observed&&=new TextDecoder('utf-8',{fatal:true}).decode(Buffer.from(r[ch+'_base64'],'base64'))===r[ch];}catch{observed=false;}}if(r.exit===0){try{JSON.parse(r.stdout);}catch{observed=false;}}else observed&&=!r.stdout&&r.stderr.includes('error: ')&&!r.stderr.includes('panicked at');ck('candidate_observed:'+k,observed);if(!observed)unknowns.push(k);ck('candidate_expression:'+k,r.source===declared.get(r.id)?.expr&&r.source_sha256===sha(r.source));ck('candidate_bin:'+k,r.binary_sha256===binary.sha256&&r.argv[0]===binary.path);const e=expect.get([r.id,r.profile].join('|'));ck('frozen_channels:'+k,!!e&&eq(sig(r),e.expected));if(e)classes[e.baseline_class]=(classes[e.baseline_class]||0)+1;}
for(const c of cases)for(const p of m.policy.profiles)for(const order of m.policy.orders){const k=[c.id,p,order].join('|'),r=keys.get(k),n=keys.get([c.id,p,'normal'].join('|'));ck('candidate_complete_stable:'+k,!!r&&!!n&&eq(sig(r),sig(n)));}
ck('candidate_cardinality',cases.length===45&&ex.length===180&&runs.rows.length===540&&keys.size===540);
ck('comparison_pin',comparison.candidate_runs_sha256===inputs[D+'p1338-tests-candidate.json']);
const comparisonKeys=new Set(comparison.rows.map(r=>[r.id,r.profile,r.order].join('|')));ck('comparison_complete',comparison.rows.length===540&&comparisonKeys.size===540&&comparison.rows.every(r=>r.verdict==='Satisfied'&&keys.has([r.id,r.profile,r.order].join('|'))));
ck('candidate_harness',runs.harness_tests.length===6&&runs.harness_tests.every(x=>x.actual===x.expected));
const summary=json('p1338-tests-summary.json');
ck('summary_pins',summary.candidate_receipt_sha256===inputs[D+'p1338-tests-candidate.json']&&summary.comparison_sha256===inputs[D+'p1338-tests-comparison.json']&&summary.build_receipt_sha256===inputs[D+'p1338-candidate-build.json']&&summary.freeze_sha256===inputs[D+'p1338-tests-freeze.json']);
ck('summary_counts',summary.candidate_runs===runs.rows.length&&summary.mandatory_unknowns===0&&eq(summary.comparison_classes,classes));
const attacks=json('p1338-attacks-results.json');ck('five_attack_rounds',attacks.complete===true&&eq(attacks.runs.map(x=>x.id),['C','M1','M2','M3','M4']));
const marker='        Value::Array(arr) => match field {',arrayGuard='            | Value::Array(_)\n';
const M1='        Value::Array(_) if !matches!(field, "len" | "first" | "last") => Err(vec![SourceDiagnostic::error(\n            span,\n            "array field access forbidden".to_string(),\n        )]),\n';
const first='            "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),\n';
const mutants={C:src,M1:src.replace(marker,M1+marker),M2:src.replace(arrayGuard,''),M3:src.replace(first,first.replace('arr.first()','arr.last()')),M4:src.replace(arrayGuard,arrayGuard+'            | Value::Length(_)\n')};
const witnesses={M1:'p1338_pure_missing_array_fields_message_and_received_span',M2:'p1338_ast_empty_heterogeneous_alias_unicode_multiline',M3:'p1338_pure_preserve_len_first_last_and_empty_none',M4:'p1338_preserve_other_categories_diagnostics'};
const rounds=[];
for(const a of attacks.runs){ck('individual_round:'+a.id,eq(a,json('p1338-attacks-run-'+a.id+'.json')));const sout=read(a.stdout_path).toString(),serr=read(a.stderr_path).toString(),body=read(a.path).toString();for(const[pk,hk]of [['path','source_sha256'],['patch_path','patch_sha256'],['preserved_executable','executable_sha256'],['stdout_path','stdout_sha256'],['stderr_path','stderr_sha256']])ck('round_pin:'+a.id+pk,sha(read(a[pk]))===a[hk]);ck('exact_mutation:'+a.id,body===mutants[a.id]);ck('tests_intact:'+a.id,body.includes(mod)&&a.tests_sha256===sha(mod+'\n'));ck('candidate_identity:'+a.id,a.candidate_sha256===sha(src)&&a.manifest_sha256===inputs[D+'p1338-manifest-r1.json']);ck('profile:'+a.id,a.profile_config==='profile.release.package.typst-core.opt-level=0'&&a.env.CARGO_BUILD_JOBS==='2'&&a.argv.includes('p1338_tests'));ck('fresh_compilation:'+a.id,a.valid_execution&&!a.execution_error&&a.compiled_typst_core&&a.mtime_fresh_ns>a.mtime_before_ns&&serr.includes('Compiling typst-core v0.1.0 ('+a.cwd+'/01_core)')&&serr.includes('Finished `release`')&&serr.includes('Running unittests')&&a.executed_tests===6&&sout.includes('running 6 tests'));ck('retained_copy:'+a.id,fs.statSync(a.preserved_executable).ino!==fs.statSync(a.original_executable).ino);ck('control_or_witness:'+a.id,a.id==='C'?a.exit===0&&sout.includes('test result: ok. 6 passed'):a.exit===101&&sout.includes(witnesses[a.id]+' ... FAILED')&&(sout+serr).includes('assertion `left == right` failed'));rounds.push({id:a.id,exit:a.exit,summary:sout.match(/test result:.*/g),executable_sha256:a.executable_sha256});}
ck('distinct_executables',new Set(attacks.runs.map(x=>x.executable_sha256)).size===5);
const attackFinal=json('p1338-attacks-final.json');
for(const[p,h]of Object.entries(attackFinal.inputs))ck('attack_final_pin:'+p,sha(read(D+p))===h);
ck('attack_final_scope_counts',attackFinal.control_passed&&attackFinal.valid_mutant_families===4&&attackFinal.semantic_rejections_observed===4&&attackFinal.unknowns===0);
const features={default:[],html:['html'],a11y:['a11y-extras'],'html+a11y':['html','a11y-extras']};
for(const r of runs.rows){const argv=[binary.path,'--color=never','eval',r.source,'--format','json'];if(features[r.profile]?.length)argv.push('--features',features[r.profile].join(','));ck('candidate_profile_argv:'+r.id+r.profile+r.order,eq(r.argv,argv));}
const report=read(D+'p1338-final-report.md'),closing=read(D+'p1338-close.py');
ck('report_pin',sha(report)==='f6efe9a4cdde90c3aea655ec29d4d3213431622ed9e6fc08f94630ba9bc6614e');
ck('closing_pin',sha(closing)==='4cbf2be2331509838b95a22c80c1c96ec6ca9d47ecf22b5796cecb64e3c574a6');
for(const n of ['p1338-record.py','p1336-record.py','p1334-lineage-lib.py'])read(D+n);
const inventory=json('p1338-review-product-inventory.json');ck('inventory_complete',inventory.exact&&inventory.added.length===0&&inventory.missing.length===0&&inventory.staged_diff_quiet_exit===0&&inventory.head===b.state.head);
read(__filename);
console.log(JSON.stringify({at:new Date().toISOString(),verdict:violations.length||unknowns.length?'BLOCKED':'PASS_SCOPED',checks,violations,unknowns,inputs,lineage:{A,B,nucleus:nh},preserved,gates,lint,ab:{runs:runs.rows.length,classes},workspace:{passed:totals[0],failed:totals[1],ignored:totals[2]},rounds,limitations:['A/B with separate authors, no technical isolation attestation or refinement seal; prior role contexts retained.','Four mutation families prove their discriminators only in the paired typst-core opt-level=0 profile; normal release gates are separate.','Frozen debt preservation is not convergence; no general Array/fields/LocatedContent/PDF/accessibility/warning claim.','External reverse-hash repair false-negative is compensated by independent complete A/B computation for this owner, not fixed in the linter.','Original pre-R1 transient integration/removal and documentary successor preserved; no step read by reviewer.']},null,2));
if(violations.length||unknowns.length)process.exitCode=1;
