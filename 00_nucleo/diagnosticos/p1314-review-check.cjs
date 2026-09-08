// Read-only review; run in the host namespace containing the recorded RAM files.
const fs = require('fs');
const cp = require('child_process');
const crypto = require('crypto');
const assert = require('assert/strict');
const D = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p, 'utf8');
const json = n => JSON.parse(read(D + n));
const digest = data => crypto.createHash('sha256').update(data).digest('hex');
const sha = p => digest(fs.readFileSync(p));
const git = (...args) => cp.execFileSync('git', args, {encoding:'utf8'});
const safe = p => { assert(!/(^|\/)00_nucleo\/(materialization|context)\//.test(p), p); return p; };
const B = json('p1314-baseline.json');
const F = json('p1314-ab-freeze.json');
const C = json('p1314-ab-cases.json').cases;
const L = F.l0.path;
const O = '01_core/src/compiler/stdlib/loading.rs';
const receipt = {at:new Date().toISOString(),head:git('rev-parse','HEAD').trim(),diff_stat:git('diff','HEAD','--stat'),index_diff:git('diff','--cached','--stat'),hashes:{},checks:{},pending:[]};
assert.equal(receipt.head,B.state.head);
assert.equal(receipt.index_diff,'');
for (const [p,h] of Object.entries(F.inputs)) assert.equal(sha(safe(p)),h,p);
for (const [name,b] of Object.entries(F.baseline_binaries)) assert.equal(sha(b.path),b.sha256,name);
assert.equal(digest(read(L).replace(/^Hash do Código: [0-9a-f]+\r?\n/m,'')),F.l0.normative_sha256);
receipt.checks.frozen_pins = Object.keys(F.inputs).length;
const changes=[];
for (const [p,h] of Object.entries(B.files)) if (sha(safe(p))!==h) changes.push(p);
assert.deepEqual(changes.sort(),[L,O].sort());
for (const [p,h] of Object.entries(B.prior_artifacts)) assert.equal(sha(safe(p)),h,p);
receipt.checks.baseline_files={count:Object.keys(B.files).length,changed:changes};
receipt.checks.prior_artifacts=Object.keys(B.prior_artifacts).length;
// Reconstruct the actual uncommitted P1313 owner using the captured baseline diff.
const headText=git('show',B.state.head+':'+O);
const marker='diff --git a/'+O+' b/'+O+'\n';
assert(B.state.diff.includes(marker));
const patch=B.state.diff.split(marker)[1].split('\ndiff --git ')[0];
const original=headText.split('\n'); original.pop();
let position=0, output=[], inHunk=false;
for (const line of patch.split('\n')) {
  const h=line.match(/^@@ -(\d+)(?:,\d+)? \+\d+(?:,\d+)? @@/);
  if(h){const target=Number(h[1])-1; output.push(...original.slice(position,target));position=target;inHunk=true;continue;}
  if(!inHunk || line==='' || line.startsWith('\\ No newline')) continue;
  if(line[0]===' ' || line[0]==='-'){assert.equal(original[position],line.slice(1),'baseline patch context');position++;}
  if(line[0]===' ' || line[0]==='+')output.push(line.slice(1));
}
output.push(...original.slice(position));
const baselineOwner=output.join('\n')+'\n';
assert.equal(digest(baselineOwner),B.files[O]);
const currentOwner=read(O);
const oldTests='    #[test]\n    fn p1313_csv_bytes_values_without_world_access()';
assert.equal(currentOwner.slice(currentOwner.indexOf(oldTests)),baselineOwner.slice(baselineOwner.indexOf(oldTests)));
const stripHeader=s=>s.replace(/^\/\/! @prompt-hash .*$/m,'');
assert.equal(stripHeader(currentOwner.split('/// Validate every causal occurrence;')[0]),stripHeader(baselineOwner.split('/// `csv(path | bytes, delimiter:')[0]));
receipt.checks.owner_reconstructed=true;
receipt.checks.prior_tests_and_non_csv_production_intact=true;
for (const p of [L,O,D+'p1314-baseline.json',D+'p1314-ab-freeze.json',D+'p1314-review-precontract.md',D+'p1314-review-prepatch.md'])receipt.hashes[p]=sha(p);
const observation=r=>JSON.stringify({exit:r.exit,stdout:r.stdout,stderr:r.stderr});
const measurement=json('p1314-ab-baseline-final.json');
const historical=json('p1313-ab-candidate-runs.json');
assert.equal(new Set(C.map(c=>c.id)).size,C.length);
assert.equal(F.expected.length,4*C.length);
for(const e of F.expected){
 const rows=measurement.runs.filter(r=>r.id===e.id&&r.profile===e.profile&&r.product===(e.kind==='control'?'baseline':'vanilla'));
 assert.equal(rows.length,1); const r=rows[0];assert.equal(r.argv[2],e.expr);
 const o={exit:r.exit,stdout:r.stdout,stderr:r.stderr};
 if(e.kind==='symbol')o.stderr='error: expected string, found symbol\n'+o.stderr.split('\n').slice(1).join('\n');
 assert.equal(observation(o),observation(e.expected),e.id);
 if(e.historical_p1313){
  const b=measurement.runs.find(r=>r.id===e.id&&r.profile===e.profile&&r.product==='baseline');
  const p=historical.runs.find(r=>r.id===e.id&&r.profile===e.profile&&r.order==='normal');
  assert.equal(b.cwd,p.cwd);assert.equal(observation(b),observation(p));
 }
}
receipt.checks.expected_recomputed=F.expected.length;
const gates=['unit-red','unit-green','build','workspace-tests','lint','fmt','diff-check','lineage-preview-final'];
for(const name of gates){
 const p=D+'p1314-'+name+'.json'; if(!fs.existsSync(p)){receipt.pending.push(name);continue;}
 const r=JSON.parse(read(p));receipt.hashes[p]=sha(p);assert.equal(r.exit,name==='unit-red'?101:0,name);
 if(name!=='unit-red')for(const key of ['before','after']){
  assert.equal(r[key].head,B.state.head);assert.equal(r[key].diff,git('diff','HEAD'),name+' '+key+' source identity');
 }
 if(name==='unit-red') assert(r.stdout.includes('2 passed; 4 failed;'));
 if(name==='unit-green')assert(r.stdout.includes('6 passed; 0 failed;'));
 if(name==='workspace-tests'){
  const summaries=[...r.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/g)];assert(summaries.length);
  receipt.checks.workspace=summaries.reduce((a,m)=>({passed:a.passed+Number(m[1]),failed:a.failed+Number(m[2]),ignored:a.ignored+Number(m[3])}),{passed:0,failed:0,ignored:0});
 }
 if(name==='lint')receipt.checks.lint={errors:(r.stdout.match(/^error:/gm)||[]).length,warnings:(r.stdout.match(/^warning:/gm)||[]).length,infos:(r.stdout.match(/^info:/gm)||[]).length};
 if(name==='lineage-preview-final')assert.equal(r.stdout.trim(),'Nothing to fix');
 if(name==='build'){assert.equal(sha(r.candidate.path),r.candidate.sha256);receipt.candidate=r.candidate;}
}
const candidateFile=D+'p1314-ab-candidate-runs.json';
if(fs.existsSync(candidateFile)){
 const R=JSON.parse(read(candidateFile));const required=new Set(F.expected.flatMap(e=>['normal','repeat','reverse'].map(o=>[e.id,e.profile,o].join('|'))));
 assert.equal(R.runs.length,required.size);const observed=new Set();let historicalChanges=0;
 for(const r of R.runs){const k=[r.id,r.profile,r.order].join('|');assert(required.has(k));assert(!observed.has(k));observed.add(k);const e=F.expected.find(e=>e.id===r.id&&e.profile===r.profile);assert.equal(r.argv[2],e.expr);assert.equal(r.cwd,e.cwd);assert.equal(observation(r),observation(e.expected),k);assert.equal(r.product,'candidate');if(e.historical_p1313&&e.baseline_red)historicalChanges++;}
 receipt.hashes[candidateFile]=sha(candidateFile);receipt.checks.ab={comparisons:R.runs.length,unknown:0,historicalChanges};
}else receipt.pending.push('ab-candidate-runs');
const key=r=>[r.case||r.id,r.profile].join('|');
const map=rows=>{const m=new Map(rows.map(r=>[key(r),r]));assert.equal(m.size,rows.length);return m;};
receipt.checks.replays={};
for(const cohort of ['p1310','p1311','p1312','p1313']){
 const name='p1314-'+cohort+'-replay.json',path=D+name;
 if(!fs.existsSync(path)){receipt.pending.push(cohort+'-replay');continue;}
 const R=json(name);receipt.hashes[path]=sha(path);
 for(const [p,h]of Object.entries(R.inputs))assert.equal(sha(D+p),h,p);
 assert.equal(R.candidate.sha256,receipt.candidate.sha256);
 const prior=cohort==='p1313'?map(historical.runs.filter(r=>r.order==='normal')):map(json('p1313-'+cohort+'-replay.json').rows);
 const actual=map(R.rows);assert.deepEqual([...actual.keys()].sort(),[...prior.keys()].sort());
 let changed=0;
 const allowed=new Set();
 for(const [k,r]of actual){
  const before=cohort==='p1313'?prior.get(k):prior.get(k).observable;
  let expected=before;
  if(cohort==='p1312'&&r.case==='csv-delimiter'){
   expected=json('p1312-ab-baseline-sealed.json').runs.find(x=>x.id===r.case&&x.profile===r.profile&&x.product==='vanilla');allowed.add(k);
  }
  if(cohort==='p1313'&&F.historical_deltas.includes(r.case)){
   expected=F.expected.find(e=>e.id===r.case&&e.profile===r.profile).expected;allowed.add(k);
  }
  assert.equal(observation(r.observable),observation(expected),cohort+' '+k);
  if(observation(r.observable)!==observation(before)){changed++;assert(allowed.has(k));}
  if(r.stdout_base64!==undefined)assert.equal(Buffer.from(r.stdout_base64,'base64').toString(),r.observable.stdout);
  if(r.stderr_base64!==undefined)assert.equal(Buffer.from(r.stderr_base64,'base64').toString(),r.observable.stderr);
 }
 assert.equal(changed,allowed.size);
 receipt.checks.replays[cohort]={preserved:actual.size-changed,changed,unknown:0};
}
const p1308=D+'p1314-p1308-replay.json';
if(fs.existsSync(p1308)){
 const R=JSON.parse(read(p1308));assert.equal(R.exit,0);receipt.hashes[p1308]=sha(p1308);
 const current=JSON.parse(R.stdout),previous=JSON.parse(json('p1313-p1308-replay.json').stdout);
 const a=map(current.rows),b=map(previous.rows);assert.deepEqual([...a.keys()].sort(),[...b.keys()].sort());
 for(const[k,r]of a){assert.deepEqual(r.observable,b.get(k).observable,k);assert.equal(r.verdict,b.get(k).verdict);}
 assert.equal(current.binary_sha256,receipt.candidate.sha256);assert.equal(current.oracle_sha256,previous.oracle_sha256);
 receipt.checks.replays.p1308={preserved:a.size,changed:0,counts_against_historical_oracle:current.counts};
}else receipt.pending.push('p1308-replay');
receipt.hashes[D+'p1314-review-check.cjs']=sha(D+'p1314-review-check.cjs');
console.log(JSON.stringify(receipt,null,2));
