// Read-only reviewer. Run: git show eb24cd657fc2333dc7ea5393f7cfebf8c7192d39:01_core/src/compiler/stdlib/loading.rs | node 00_nucleo/diagnosticos/p1313-review-check.cjs
const fs = require('fs');
const crypto = require('crypto');
const D = '00_nucleo/diagnosticos/';
const shaText = x => crypto.createHash('sha256').update(x).digest('hex');
const sha = p => shaText(fs.readFileSync(p));
const read = n => JSON.parse(fs.readFileSync(D + 'p1313-' + n + '.json', 'utf8'));
const eq = (a,b) => JSON.stringify(a) === JSON.stringify(b);
const obs = x => ({exit:x.exit,stdout:x.stdout,stderr:x.stderr});
const b = read('implementation-baseline'), f = read('ab-freeze');
const runs = read('ab-candidate-runs'), comparison = read('ab-comparison');
const build = read('build'), gates = read('gates'), workspace = read('workspace-tests');
const errors = [], checks = {};
const check = (name, value) => { checks[name] = !!value; if (!value) errors.push(name); };
const owner = '01_core/src/compiler/stdlib/loading.rs';
const l0 = '00_nucleo/prompts/compiler/stdlib/loading.md';
const original = fs.readFileSync(0,'utf8').split('\n');
const chunk = b.state.diff.split(/(?=^diff --git )/m).find(s => s.startsWith(`diff --git a/${owner} b/${owner}\n`));
let out=[], pos=0, active=false;
for (const line of chunk.split('\n')) {
  if (line.startsWith('@@')) { const m=line.match(/^@@ -(\d+)(?:,\d+)? \+/); const index=Number(m[1])-1; out.push(...original.slice(pos,index)); pos=index; active=true; continue; }
  if (!active) continue;
  if (line[0]===' ' || line[0]==='-') { if(original[pos]!==line.slice(1))throw Error('baseline context mismatch'); if(line[0]===' ')out.push(original[pos]); pos++; }
  else if(line[0]==='+')out.push(line.slice(1));
}
out.push(...original.slice(pos));
const prior=out.join('\n'), current=fs.readFileSync(owner,'utf8');
check('baseline_source_reconstructed_exactly',shaText(prior)===b.files[owner]);
const prod=s=>s.split('#[cfg(test)]')[0].replace(/^\/\/! @prompt-hash .*\n/m,'');
const part=(s,start,end)=>s.slice(s.indexOf(start),s.indexOf(end,s.indexOf(start)));
let left=prod(prior),right=prod(current);
left=left.replace(part(left,"fn arg_path<'a>",'/// Read accepts'),'CSV_HELPER\n');
right=right.replace(part(right,'fn arg_csv_source','/// Read accepts'),'CSV_HELPER\n');
left=left.replace('/// Read accepts PathOrStr; CSV retains its separately scoped legacy cast.','/// Read accepts PathOrStr; CSV has its separate DataSource cast.');
left=left.replace('/// `csv(path, delimiter:', '/// `csv(path | bytes, delimiter:');
left=left.replace('    let path = arg_path(args, "csv")?;', '    let source = arg_csv_source(args)?;');
left=left.replace('    let (_, data) = read_bytes(world, current_file, path, "csv")?;', '    if let Value::Bytes(data) = source {\n        return decode_csv(data.as_slice(), delimiter, row_type);\n    }\n    let (_, data) = read_bytes(world, current_file, source, "csv")?;');
check('production_only_declared_csv_delta',left===right);
const priorDrift=Object.entries(b.prior_artifacts).filter(([p,h])=>sha(p)!==h).map(([p])=>p);
check('prior_artifacts_intact',priorDrift.length===0);
const skipped=[],changed=[];
for(const [p,h] of Object.entries(b.files)) {
  if(p.startsWith('00_nucleo/context/')||p.startsWith('00_nucleo/materialization/')){skipped.push(p);continue;}
  if(sha(p)!==h)changed.push(p);
}
check('baseline_only_owner_and_l0_change',eq(changed.sort(),[owner,l0].sort()));
check('frozen_inputs_intact',Object.entries(f.inputs).every(([p,h])=>sha(p)===h));
const normative=fs.readFileSync(l0,'utf8').replace(/^Hash do Código: [0-9a-f]+\r?\n/m,'');
check('l0_normative_pin',shaText(normative)===f.l0.normative_sha256);
check('candidate_binary_pin',sha(build.candidate.path)===build.candidate.sha256 && eq(runs.binaries.candidate,build.candidate));
check('baseline_binaries_intact',Object.values(f.baseline_binaries).every(x=>sha(x.path)===x.sha256));
const expected = new Map(f.expected.map(x=>[x.id+'|'+x.profile,x.expected]));
const required=new Set([...expected.keys()].flatMap(k=>['normal','repeat','reverse'].map(o=>k+'|'+o)));
const actual=runs.runs.map(x=>x.id+'|'+x.profile+'|'+x.order);
check('ab_unique_complete',actual.length===required.size && new Set(actual).size===required.size && actual.every(k=>required.has(k)));
const abFailures=runs.runs.filter(x=>!eq(obs(x),expected.get(x.id+'|'+x.profile)));
check('ab_independent_recomparison',abFailures.length===0);
check('ab_comparison_receipt',comparison.status==='PASS' && comparison.failures.length===0 && comparison.unknown===0 && comparison.candidate_runs_sha256===sha(D+'p1313-ab-candidate-runs.json') && comparison.freeze_sha256===sha(D+'p1313-ab-freeze.json'));
const finalGates=['build','workspace-tests','lint','fmt','diff-check','lineage-preview-final'];
for(const name of finalGates){const g=read(name);check('final_gate_'+name,g.exit===0 && g.before.diff===build.before.diff && g.after.diff===build.after.diff);}
check('gate_receipt_inputs',Object.entries(gates.inputs).every(([p,h])=>sha(D+p)===h));
check('gates_all_true',gates.pass && Object.values(gates.checks).every(Boolean));
const replayCounts={};
for(const name of ['p1310-replay','p1311-replay','p1312-replay']){
  const d=read(name); check(name+'_identity',eq(d.candidate,build.candidate));
  check(name+'_checks',d.pass && Object.values(d.checks).every(Boolean));
  let preserved=0,changedCount=0,failures=0;
  for(const row of d.rows){const previous=row.p1312_observable||row.previous;const same=eq(row.observable,previous);if(same)preserved++;else changedCount++;
    const target=name==='p1312-replay'?row.expected:(same?previous:row.delta_expected);if(!eq(row.observable,target))failures++;}
  check(name+'_recomparison',failures===0);
  replayCounts[name]={preserved,changed:changedCount,failures};
}
const p1308=read('p1308-delta');check('p1308_delta',p1308.pass && Object.values(p1308.checks).every(Boolean) && eq(p1308.candidate,build.candidate));
const parseReplay=p=>JSON.parse(JSON.parse(fs.readFileSync(D+p,'utf8')).stdout);
const current1308=parseReplay('p1313-p1308-replay.json'),previous1308=parseReplay('p1312-p1308-replay.json'),vanilla1308=parseReplay('p1309-sentinels-p1308-vanilla.json');
const keyed=x=>new Map(x.rows.map(r=>[r.case+'|'+r.profile,r]));
const [cm,pm,vm]=[current1308,previous1308,vanilla1308].map(keyed);
const wanted1308=new Set(['p1307.decoder.csv','p1307.decoder.csv.wrong-type'].flatMap(c=>['default','html','a11y','html+a11y'].map(p=>c+'|'+p)));
const changed1308=[...cm.keys()].filter(k=>!eq(cm.get(k).observable,pm.get(k)?.observable));
check('p1308_independent_complete_keys',cm.size===current1308.rows.length && cm.size===pm.size && cm.size===vm.size && [...cm.keys()].every(k=>pm.has(k)&&vm.has(k)));
check('p1308_independent_exact_delta',changed1308.length===wanted1308.size && changed1308.every(k=>wanted1308.has(k)));
check('p1308_independent_vanilla',changed1308.every(k=>eq(cm.get(k).observable,vm.get(k).observable)));
const ws=[...workspace.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)].reduce((a,m)=>({passed:a.passed+Number(m[1]),failed:a.failed+Number(m[2]),ignored:a.ignored+Number(m[3])}),{passed:0,failed:0,ignored:0});
check('workspace_six_local_tests_final',workspace.stdout.split('\n').filter(x=>x.startsWith('test compiler::stdlib::loading::tests::p1313_')&&x.endsWith(' ... ok')).length===6);
const receiptNames=['implementation-baseline','ab-freeze','ab-candidate-runs','ab-comparison','gates',...finalGates,'unit-red-r1','unit-green','p1310-replay','p1311-replay','p1312-replay','p1308-delta'];
console.log(JSON.stringify({schema:'p1313-review-evidence-v1',issuer:'/root/p1313_review',utc:new Date().toISOString(),head:b.state.head,working_tree:'uncommitted',diff_stat:build.after.diff_stat,source_sha256:sha(owner),l0_sha256:sha(l0),candidate:build.candidate,checker_sha256:sha(__filename),inputs:Object.fromEntries(receiptNames.map(n=>['p1313-'+n+'.json',sha(D+'p1313-'+n+'.json')])),checks,errors,source_review:'CSV helper inspected separately; baseline reconstruction proves other production preserved.',prior_count:Object.keys(b.prior_artifacts).length,prior_drift:priorDrift,baseline_files_compared:Object.keys(b.files).length-skipped.length,restricted_paths_skipped:skipped.length,changed_files:changed,ab:{comparisons:runs.runs.length,failures:abFailures.length},workspace:ws,replays:replayCounts,p1308:p1308.counts_against_frozen_predecessor,regime:'A/B without technical isolation attestation',verdict:errors.length?'BLOCK':'PASS_SCOPED'},null,2));
process.exitCode=errors.length?1:0;
