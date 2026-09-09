// Independent read-only final audit. Publication is done separately with apply_patch.
const fs=require('fs'),crypto=require('crypto'),path=require('path');
const R='/repos/Antigravity/typst-crystalline',D=R+'/00_nucleo/diagnosticos/';
const inputs={},violations=[],unknowns=[],checks=[];
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
function read(p,pin=true){p=path.resolve(R,p);if(/00_nucleo\/(materialization|context)(\/|$)/.test(p))throw Error('forbidden');let b=fs.readFileSync(p);if(pin)inputs[p]=sha(b);return b;}
const json=n=>JSON.parse(read(D+n));
function ck(name,ok){checks.push({name,ok:!!ok});if(!ok)violations.push(name);}
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const sig=r=>({exit:r.exit,stdout_base64:r.stdout_base64,stderr_base64:r.stderr_base64});
const m=json('p1337-manifest-r1.json'),base=json('p1337-baseline.json'),go=json('p1337-review-prepatch-go.json'),candidate=json('p1337-candidate.json');
ck('manifest_pin',inputs[D+'p1337-manifest-r1.json']==='7c09595cde021b89fffa7c316b7706797ba3898aa5b6687cdfaf2deacb0e65ff');
ck('baseline_pin',inputs[D+'p1337-baseline.json']===m.baseline_sha256);
for(const [p,h]of Object.entries(go.inputs))if(![R+'/'+m.source,R+'/'+m.prompt.path].includes(p))ck('prepatch_pin:'+p,sha(read(p))===h);
for(const p of ['red_review','red_mechanical_transport'])ck('candidate_'+p,sha(read(candidate[p].path))===candidate[p].sha256);
for(const k of ['tests_freeze','attacks_freeze'])ck('candidate_'+k,sha(read(candidate[k+'_path']))===candidate[k+'_sha256']&&candidate[k+'_path'].endsWith('-r2.json'));
const mod=read(D+'p1337-tests-module-r2.rs.txt').toString();
let red=m.pre_candidate_source.replace('#[cfg(test)]\nmod p1336_tests {',mod+'\n#[cfg(test)]\nmod p1336_tests {').replace('p1336_preserve_other_fallback_names_and_ast_span','p1337_successor_bool_none_auto_lookup_names_and_ast_span');
for(const [kind,old]of [['true','bool'],['none','none'],['auto','auto']])red=red.replace(`("#(${kind}.nope)", "${kind}.nope", "cannot access fields on type ${old}")`,`("#(${kind}.nope)", "nope", "cannot access fields on type ${kind==='true'?'boolean':kind}")`);
ck('reconstruct_formatted_tests_only',sha(red)===json('p1337-tests-local-patch-r2.json').tests_only_after_sha256);
const guard='            | Value::Str(_)\n';
const oldArm='        other @ (Value::Int(_) | Value::Str(_)) => Err(vec![SourceDiagnostic::error(\n            span,\n            format!("cannot access fields on type {}", vanilla_type_name(&other)),\n        )]),';
const newArm='        other @ (Value::Bool(_) | Value::Int(_) | Value::Str(_)) => {\n            Err(vec![SourceDiagnostic::error(\n                span,\n                format!("cannot access fields on type {}", vanilla_type_name(&other)),\n            )])\n        }';
ck('productive_anchor_unique',red.split(guard).length===2&&red.split(oldArm).length===2);
const expected=red.replace(guard,guard+'            | Value::Bool(_)\n            | Value::None\n            | Value::Auto\n').replace(oldArm,newArm);
const src=read(m.source).toString(),l0=read(m.prompt.path).toString();
ck('only_reviewed_product_delta_all_tests_exact',src===expected);
ck('candidate_source_pin',sha(src)===candidate.source_sha256&&src===candidate.source);
const cleanL0=l0.replace(/^Hash do Código: [a-f0-9]{8}\n/m,'');
ck('normative_L0_unchanged',cleanL0===m.prompt.normative_text&&sha(cleanL0)===m.prompt.normative_sha256);
const refs=[...l0.matchAll(/^- (\S+) sha256:([a-f0-9]{64})$/gm)];
ck('one_reviewed_nucleus',refs.length===1&&refs[0][1]==='00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml');
const nb=read(refs[0][1]);ck('nucleus_leaf',!nb.toString().includes('[[dependencies]]'));
const nh=sha(Buffer.concat([nb,Buffer.from('\0TEKT-NUCLEUS-DEPS-V1\0')]));ck('nucleus_effective_pin',refs[0][2]===nh);
const frame=Buffer.alloc(8);frame.writeBigUInt64BE(BigInt(Buffer.byteLength(refs[0][1])));
const A=sha(Buffer.concat([Buffer.from(cleanL0),Buffer.from('\0TEKT-PROMPT-NUCLEI-V1\0'),frame,Buffer.from(refs[0][1]+' '),Buffer.from(nh,'hex')]));
const B=sha(src.replace(/^\/\/! @prompt-hash [a-f0-9]{8}\n/m,''));
ck('independent_forward_A',src.match(/^\/\/! @prompt-hash (\w+)$/m)[1]===A.slice(0,8));
ck('independent_reverse_B',l0.match(/^Hash do Código: (\w+)$/m)[1]===B.slice(0,8));
const reciprocal=json('p1337-lineage-reciprocal.json');
ck('reciprocal_metadata_only',reciprocal.before.recorded_a===reciprocal.after.recorded_a&&reciprocal.before.norm_sha256===reciprocal.after.norm_sha256&&reciprocal.after.effective_a===A&&reciprocal.after.code_b===B&&reciprocal.after.recorded_b===B.slice(0,8));
for(const p of ['/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs','/repos/Antigravity/tekt-linter/03_infra/nucleus.rs','/repos/Antigravity/tekt-linter/03_infra/prompt_io.rs'])read(p);
let preserved={product:0,historical:0,temporary:0};
for(const [p,h]of Object.entries(base.state.product_inventory))if(!m.policy.allowed_product_changes.includes(p)){ck('product:'+p,sha(read(p,false))===h);preserved.product++;}
for(const [p,h]of Object.entries(base.historical_preserved)){ck('history:'+p,sha(read(p,false))===h);preserved.historical++;}
for(const [p,h]of Object.entries(base.retained_prior_artifacts)){ck('temporary:'+p,sha(read(p,false))===h);preserved.temporary++;}
const gateNames=['p1337-unit-green.json','p1337-candidate-build.json','p1337-workspace-tests.json','p1337-fmt.json','p1337-diff-check.json','p1337-lint-general.json','p1337-lineage-final.json'];
const gates={};
for(const n of gateNames){const r=json(n);gates[n]={exit:r.exit,argv:r.argv,summary:(r.stdout||'').match(/test result:.*|.*errors.*warnings.*/g)};ck('gate_exit:'+n,r.exit===0);if(r.before?.product_inventory)ck('gate_source_stable:'+n,r.before.product_inventory[m.source]===sha(src)&&r.after.product_inventory[m.source]===sha(src));}
ck('strict_lineage_gate',gates['p1337-lineage-final.json'].argv.includes('v5,v15,v26')&&gates['p1337-lineage-final.json'].argv.includes('--fail-on')&&gates['p1337-lineage-final.json'].argv.includes('warning'));
const lintParts=n=>json(n).stdout.split(/(?=^(?:warning|info|error):)/m).filter(s=>/^(?:warning|info|error):/.test(s)).map(s=>s.replace(/(--> [^\n]+?):\d+(?::\d+)?/g,'$1:<line>').trim());
const lintOld=lintParts('p1336-lint-final.json'),lintNew=lintParts('p1337-lint-general.json');
const multiset=a=>{const m=new Map();for(const x of a)m.set(x,(m.get(x)||0)+1);return m;};
const lo=multiset(lintOld),ln=multiset(lintNew),lintRemoved=[],lintAdded=[];
for(const k of new Set([...lo.keys(),...ln.keys()])){const diff=(ln.get(k)||0)-(lo.get(k)||0);if(diff<0)lintRemoved.push({count:-diff,diagnostic:k});if(diff>0)lintAdded.push({count:diff,diagnostic:k});}
ck('warnings_multiset_preserved',eq(lintOld.filter(x=>x.startsWith('warning:')).sort(),lintNew.filter(x=>x.startsWith('warning:')).sort()));
ck('only_two_expected_info_rewordings',lintRemoved.length===2&&lintAdded.length===2&&lintRemoved.every(x=>x.count===1&&x.diagnostic.startsWith('info:')&&x.diagnostic.includes('other @ (Value::Int(_) | Value::Str(_))'))&&lintAdded.every(x=>x.count===1&&x.diagnostic.startsWith('info:')&&x.diagnostic.includes('other @ (Value::Bool(_) | Value::Int(_) | Value::Str(_))')));
const lint={counts:Object.fromEntries(['warning','info','error'].map(k=>[k,lintNew.filter(x=>x.startsWith(k+':')).length])),normalization:'Only location line/column numbers removed; severity, message, snippet, path and multiplicity preserved.',removed:lintRemoved,added:lintAdded};
const invalidLint=json('p1337-lint-strict.json');ck('invalid_lint_not_used_as_gate',invalidLint.exit===2);
const runs=json('p1337-tests-candidate.json'),old=json('p1337-tests-baseline.json'),ex=json('p1337-tests-expectations.json'),cases=json('p1337-tests-cases.json'),comparison=json('p1337-tests-comparison.json');
const decl=new Map(cases.map(c=>[c.id,c])),expect=new Map(ex.map(e=>[[e.id,e.profile].join('|'),e])),keys=new Map();let classes={};
for(const p of [m.vanilla,m.baseline_binary,runs.products.candidate])ck('binary:'+p.path,sha(read(p.path))===p.sha256);
for(const r of runs.rows){let k=[r.id,r.profile,r.order].join('|');ck('candidate_unique:'+k,!keys.has(k));keys.set(k,r);let good=!r.failure&&[0,1].includes(r.exit)&&r.observation==='Observed';for(const ch of ['stdout','stderr']){try{good&&=new TextDecoder('utf-8',{fatal:true}).decode(Buffer.from(r[ch+'_base64'],'base64'))===r[ch];}catch{good=false;}}if(r.exit===0){try{JSON.parse(r.stdout);}catch{good=false;}}else good&&=!r.stdout&&r.stderr.includes('error: ')&&!r.stderr.includes('panicked at');ck('observed:'+k,good);if(!good)unknowns.push(k);ck('candidate_source_expression:'+k,r.source===decl.get(r.id)?.expr&&sha(r.source)===r.source_sha256);ck('candidate_binary_row:'+k,r.binary_sha256===runs.products.candidate.sha256&&r.argv[0]===runs.products.candidate.path);const e=expect.get([r.id,r.profile].join('|'));ck('exact_frozen_channels:'+k,!!e&&eq(sig(r),e.expected));classes[e.baseline_class]=(classes[e.baseline_class]||0)+1;}
for(const c of cases)for(const profile of m.policy.profiles)for(const order of m.policy.orders){let k=[c.id,profile,order].join('|'),normal=keys.get([c.id,profile,'normal'].join('|'));ck('stable_order:'+k,keys.has(k)&&normal&&eq(sig(keys.get(k)),sig(normal)));}
ck('candidate_cardinality',cases.length===45&&ex.length===180&&old.rows.length===1080&&runs.rows.length===540&&keys.size===540);
ck('comparison_recomputed',comparison.rows.length===540&&comparison.rows.every(r=>r.verdict==='Satisfied'&&keys.has([r.id,r.profile,r.order].join('|'))));
ck('comparison_pin',comparison.candidate_runs_sha256===inputs[D+'p1337-tests-candidate.json']);
const attacks=json('p1337-attacks-results.json');ck('attack_rounds_complete',attacks.complete===true&&attacks.runs.length===6);let rounds=[];
for(const a of attacks.runs){const rr=json('p1337-attacks-run-'+a.id+'.json');ck('round_receipt:'+a.id,eq(a,rr));const sout=read(a.stdout_path).toString(),serr=read(a.stderr_path).toString(),source=read(a.path).toString();ck('round_hashes:'+a.id,sha(sout)===a.stdout_sha256&&sha(serr)===a.stderr_sha256&&sha(source)===a.source_sha256&&sha(read(a.preserved_executable))===a.executable_sha256);ck('round_profile:'+a.id,a.profile_config==='profile.release.package.typst-core.opt-level=0'&&a.env.CARGO_BUILD_JOBS==='2');ck('round_fresh_compile:'+a.id,a.valid_execution&&a.compiled_typst_core&&a.mtime_fresh_ns>=a.mtime_before_ns&&serr.includes('Compiling typst-core v0.1.0 ('+a.cwd+'/01_core)')&&serr.includes('Finished `release`')&&serr.includes('Running unittests')&&/running [1-9]\d* tests/.test(sout));ck('round_tests_intact:'+a.id,source.includes(mod)&&a.candidate_sha256===sha(src));ck('round_control_or_assertion:'+a.id,a.id==='C'?a.exit===0:a.exit===101&&sout.includes('FAILED')&&(sout+serr).includes('assertion `left == right` failed'));rounds.push({id:a.id,exit:a.exit,executable_sha256:a.executable_sha256,summary:sout.match(/test result:.*/g)});}
const report=read(D+'p1337-final-report.md'),closing=read(D+'p1337-close.py');
const attackFinal=json('p1337-attacks-final.json');
for(const [p,h]of Object.entries(attackFinal.inputs))ck('attacks_final_pin:'+p,sha(read(D+p))===h);
ck('attacks_final_totals',attackFinal.control_passed&&attackFinal.valid_mutant_families===5&&attackFinal.semantic_rejections_observed===5&&attackFinal.unknowns===0);
ck('distinct_retained_executables',new Set(attacks.runs.map(a=>a.executable_sha256)).size===6);
const m1='        Value::Bool(_) => Err(vec![SourceDiagnostic::error(\n            span,\n            "cannot access fields on type bool".to_string(),\n        )]),\n';
const expectedMutants={C:src,M1:src.replace(newArm,m1+newArm),M2:src.replace('            | Value::Bool(_)\n',''),M3:src.replace('            | Value::None\n',''),M4:src.replace('            | Value::Auto\n',''),M5:src.replace(guard,guard+'            | Value::Array(_)\n')};
for(const a of attacks.runs){ck('exact_frozen_mutant_family:'+a.id,read(a.path).toString()===expectedMutants[a.id]);ck('mutant_patch_hash:'+a.id,sha(read(a.patch_path))===a.patch_sha256);ck('mutant_strict_freshness:'+a.id,a.mtime_fresh_ns>a.mtime_before_ns&&a.executed_tests===4&&a.tests_sha256===sha(mod+'\n'));ck('retained_not_hardlinked:'+a.id,fs.statSync(a.preserved_executable).ino!==fs.statSync(a.original_executable).ino);}
const summary=json('p1337-tests-summary-r1.json');
ck('effective_summary_pins',summary.candidate_receipt_sha256===inputs[D+'p1337-tests-candidate.json']&&summary.comparison_sha256===inputs[D+'p1337-tests-comparison.json']&&summary.effective_freeze_sha256===inputs[D+'p1337-tests-freeze-r2.json']);
const failedSummary=read(D+summary.supersedes_invalid_summary.file);let parseFailed=false;try{JSON.parse(failedSummary);}catch{parseFailed=true;}
ck('invalid_summary_retained_not_authoritative',parseFailed&&sha(failedSummary)===summary.supersedes_invalid_summary.sha256);
const replay=json('p1337-review-witness-direct.json');read(D+'p1337-review-witness-replay.json');
ck('independent_direct_replay',replay.runs.length===6&&replay.runs.every(x=>x.exit_code===(x.id==='C'?0:101)&&!x.session_id&&x.output.includes('test result:')&&(x.id==='C'||x.output.includes('assertion `left == right` failed')))&&Object.values(replay.binaries).every(x=>x.actual===x.expected&&sha(read(x.path))===x.actual));
read(__filename);read(D+'p1337-review-fmt-transition.md');
const inventory=json('p1337-review-product-inventory.json');ck('product_inventory_exact',inventory.exact&&inventory.added.length===0&&inventory.missing.length===0&&inventory.baseline_sha256===m.baseline_sha256&&inventory.head===base.state.head&&inventory.staged_diff_quiet_exit===0);
const workspaceTotals=[...json('p1337-workspace-tests.json').stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)].reduce((a,x)=>a.map((v,i)=>v+Number(x[i+1])),[0,0,0]);
ck('workspace_report_sum',eq(workspaceTotals,[6743,0,3]));
console.log(JSON.stringify({
  at:new Date().toISOString(),verdict:violations.length||unknowns.length?'BLOCKED':'PASS_SCOPED',
  violations,unknowns,checks:checks.length,lineage:{A,B,nucleus:nh},preserved,gates,lint,
  ab:{runs:runs.rows.length,classes},workspace:{passed:workspaceTotals[0],failed:workspaceTotals[1],ignored:workspaceTotals[2]},rounds,
  instrumentation_history:[
    'Initial strict lint invocation failed argument parsing; corrected strict invocation is the acceptance gate.',
    'Initial truncated tests-summary.json remains invalid and pinned; compact summary-r1 references intact authoritative runs/comparison.',
    'Optional Node spawnSync replay returned EPERM alongside test output and was not clean acceptance evidence. Six direct exec_command replays then completed with intact retained binary hashes and expected semantic outcomes.'
  ],
  limitations:[
    'A/B without technical isolation attestation or refinement seal; reviewer retained prior review context',
    'RED-r1 transported to R2 by audited canonical rustfmt, not claimed as execution of R2 bytes',
    'Instrumental opt-level=0 mutation profile is paired discrimination, not normal release equivalence',
    'Existing debts preserved, not counted as parity; no general fields/layout/PDF/accessibility/LocatedContent/warning coverage claim',
    'Reverse linter repair false-negative remains external; independent full A/B computation compensates this pair only',
    'Parser-invalid initial strict-lint invocation retained as instrumentation history, never counted as gate; corrected V5/V15/V26 invocation passed'
  ],inputs
},null,2));
if(violations.length||unknowns.length)process.exitCode=1;
