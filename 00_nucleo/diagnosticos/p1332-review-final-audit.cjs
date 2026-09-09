// Read-only final audit. Missing evidence or a mismatch throws; no product runs.
require('./p1332-review-candidate-audit.cjs');
const fs=require('fs'),path=require('path'),crypto=require('crypto'),assert=require('assert');
const D=__dirname,ROOT=path.resolve(D,'../..');
const read=n=>fs.readFileSync(path.join(D,n),'utf8'),json=n=>JSON.parse(read(n));
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const dh=n=>hash(path.join(D,n));
const manifest=json('p1332-manifest.json'),mh=dh('p1332-manifest.json');
const baseline=json('p1332-baseline.json'),integration=json('p1332-test-integration.json');
const freeze=json('p1332-ab-freeze.json');
const green=json('p1332-unit-green.json'),red=json('p1332-unit-red.json');
const snapshot=green.after,owner='01_core/src/compiler/stdlib/calc.rs';
assert.equal(snapshot.head,baseline.head);
assert.equal(snapshot.staged,baseline.state.staged);
for(const [p,h]of Object.entries(snapshot.product_inventory)){
  assert(!p.startsWith('00_nucleo/materialization/')&&!p.startsWith('00_nucleo/context/'));
  assert.equal(hash(path.join(ROOT,p)),h,p);
}
for(const [p,h]of Object.entries(baseline.product_inventory))
  if(![owner,manifest.prompt].includes(p))assert.equal(snapshot.product_inventory[p],h,p);
for(const [p,h]of Object.entries(baseline.historical_preserved)){
  assert(p.startsWith(path.join(D,'p')),'historical allowlist');
  assert.equal(hash(p),h,p);
}
for(const [p,h]of Object.entries(integration.frozen))assert.equal(hash(p),h,p);
assert.equal(dh('p1331-closure.json'),manifest.previous_closure_sha256);
assert.equal(dh('p1332-baseline.json'),manifest.baseline_sha256);
const gateNames=['unit-green','final-build','workspace-tests','final-fmt','final-lint',
  'final-lineage','final-lineage-lint','final-diff-check','cli-normal','cli-repeat','cli-reverse'];
const gates={};
for(const name of gateNames){
  const file=`p1332-${name}.json`,r=json(file);
  assert.equal(r.exit,0,file);assert.equal(r.manifest_sha256,mh,file);
  for(const state of [r.before,r.after]){
    assert.deepStrictEqual(state.product_inventory,snapshot.product_inventory,file);
    assert.equal(state.head,baseline.head,file);
    assert.equal(state.staged,baseline.state.staged,file);
  }
  gates[name]={sha256:dh(file),argv:r.argv,at:r.at,end:r.end,exit:r.exit};
}
assert.equal(red.exit,101);
assert.deepStrictEqual(red.before.product_inventory,red.after.product_inventory);
assert.deepStrictEqual(red.before.product_inventory,integration.after.product_inventory);
assert(Date.parse(red.end)<Date.parse(json('p1332-code-reseal.json').at));
assert(Date.parse(json('p1332-code-reseal.json').at)<Date.parse(green.at));
const testNames=s=>[...s.matchAll(/^test (\S+) \.\.\./gm)].map(x=>x[1]).sort();
assert.deepStrictEqual(testNames(red.stdout),testNames(green.stdout));
assert.equal(testNames(green.stdout).length,35);
const workspace=json('p1332-workspace-tests.json');
const totals=[...workspace.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)]
  .reduce((a,m)=>a.map((n,i)=>n+Number(m[i+1])),[0,0,0]);
assert(totals[0]>=6717);assert.equal(totals[1],0);
const expected=json('p1332-ab-cli-expected.json'),raw=json('p1332-ab-cli-baseline.json');
const old=json('p1331-ab-cli-expected-r2.json');
const key=r=>`${r.case}/${r.profile}`,map=rs=>new Map(rs.map(r=>[key(r),r]));
const em=map(expected.expectations),rm=map(raw.cases),om=map(old.expectations);
assert.equal(em.size,616);assert.equal(rm.size,616);assert.equal(om.size,504);
assert.equal(expected.baseline_sha256,dh('p1332-ab-cli-baseline.json'));
const obs=x=>({exit:x.exit,stdout:x.stdout,stderr:x.stderr});
const eq=(x,y)=>JSON.stringify(x)===JSON.stringify(y);
let historicalChanged=0,newChanged=0;
for(const [k,r]of rm){
  const b=obs(r.results.BASE),e=em.get(k);
  if(om.has(k))assert.deepStrictEqual(b,om.get(k).expected,k);
  const wanted={...b,stderr:b.stderr.replaceAll('  while calling `calc.abs` at ','  while calling `abs` at ')};
  if(['name-gradient-linear','name-gradient-radial','name-gradient-conic','name-show'].includes(r.case)){
    const lines=wanted.stderr.split('\n');
    lines[0]=lines[0].replaceAll('Some("calc.abs")','Some("abs")').replaceAll("função 'calc.abs'","função 'abs'");
    wanted.stderr=lines.join('\n');
  }
  assert.deepStrictEqual(e.expected,wanted,k);
  if(!eq(b,wanted)){if(om.has(k))historicalChanged++;else newChanged++;}
  if(r.case==='name-math-import'){
    assert.equal(e.classification,'preserved-debt-imported-math-resolution');
    assert.deepStrictEqual(e.expected,b);assert(!eq(b,obs(r.results.VANILLA)));
  }
}
assert.equal(historicalChanged,84);assert.equal(newChanged,60);
const binaryHash=hash(path.join(manifest.target,'release/typst'));
assert.equal(json('p1332-final-build.json').binary.sha256,binaryHash);
for(const role of ['BASE','VANILLA'])assert.equal(hash(raw.binaries[role].path),raw.binaries[role].sha256);
let firstRows;const cli={};
for(const order of ['normal','repeat','reverse']){
  const file=`p1332-ab-cli-${order}.json`,r=json(file);
  assert.equal(r.manifest_sha256,mh);assert.equal(r.l0_norm_sha256,manifest.prompt_norm_sha256);
  assert.equal(r.expected_sha256,dh('p1332-ab-cli-expected.json'));
  assert.equal(r.runner_sha256,dh('p1332-ab-cli.py'));
  assert.equal(r.binaries.CANDIDATE.sha256,binaryHash);
  for(const role of ['BASE','VANILLA'])assert.equal(r.binaries[role].sha256,raw.binaries[role].sha256);
  assert.equal(r.cases.length,616);assert.equal(map(r.cases).size,616);
  const counts={baselineFull:0,candidateFull:0,changed:0,regressions:0,remaining:0,
    historicalBaselineFull:0,historicalCandidateFull:0,partialChanges:0};
  for(const row of r.cases){
    const k=key(row),c=obs(row.results.CANDIDATE),b=obs(row.results.BASE),v=obs(row.results.VANILLA);
    assert.deepStrictEqual(c,em.get(k).expected,k);assert.equal(row.classification,em.get(k).classification);
    assert(row.candidate_matches_frozen_policy,k);
    assert.deepStrictEqual(b,obs(rm.get(k).results.BASE),k);
    assert.deepStrictEqual(v,obs(rm.get(k).results.VANILLA),k);
    counts.baselineFull+=eq(b,v);counts.candidateFull+=eq(c,v);
    counts.changed+=!eq(c,b);counts.regressions+=eq(b,v)&&!eq(c,v);counts.remaining+=!eq(c,v);
    counts.partialChanges+=!eq(c,b)&&!eq(c,v);
    if(om.has(k)){counts.historicalBaselineFull+=eq(b,v);counts.historicalCandidateFull+=eq(c,v);}
  }
  assert.equal(counts.changed,144);assert.equal(counts.regressions,0);
  assert.equal(counts.partialChanges,68);
  assert.equal(counts.historicalBaselineFull,404);assert.equal(counts.historicalCandidateFull,452);
  if(order==='normal')firstRows=r.cases;
  else {
    const first=map(firstRows);
    for(const row of r.cases)for(const role of ['BASE','VANILLA','CANDIDATE'])
      assert.deepStrictEqual(obs(row.results[role]),obs(first.get(key(row)).results[role]));
    if(order==='repeat')assert.deepStrictEqual(r.cases.map(key),firstRows.map(key));
    else assert.deepStrictEqual([...new Set(r.cases.map(x=>x.case))],[...new Set(firstRows.map(x=>x.case))].reverse());
  }
  cli[order]={sha256:dh(file),utc:r.utc,observations:r.cases.length,...counts};
}
const lintResults=n=>JSON.parse(json(n).stdout).runs.flatMap(r=>r.results);
const sig=r=>JSON.stringify({rule:r.ruleId,level:r.level,message:r.message,uris:r.locations.map(l=>l.physicalLocation.artifactLocation.uri)});
const lint=lintResults('p1332-final-lint.json'),oldLint=lintResults('p1331-final-lint-r2.json');
assert.deepStrictEqual(lint.map(sig).sort(),oldLint.map(sig).sort());
const lintCounts=lint.reduce((a,r)=>(a[r.level]=(a[r.level]||0)+1,a),{error:0,warning:0,note:0});
assert.deepStrictEqual(lintCounts,{error:0,warning:240,note:1146});
const report=read('p1332-final-report.md'),ab=read('p1332-ab-receipt.md');
assert.equal(dh('p1332-final-report.md'),'991d0a37a368442e9719b99856d05514d32594eec24dc092e08273996bf37e6d');
assert.equal(dh('p1332-ab-receipt.md'),'7991337b964ef8f929c57ac76a90b37281899e87f0de4eab515e169d416b60ff');
assert(!report.includes('este rascunho ainda não constitui fechamento'));
assert(/PASS/.test(ab));
console.log(JSON.stringify({at:new Date().toISOString(),head:baseline.head,working_tree:'uncommitted',
  manifest_sha256:mh,freeze_sha256:dh('p1332-ab-freeze.json'),
  report_sha256:dh('p1332-final-report.md'),ab_receipt_sha256:dh('p1332-ab-receipt.md'),
  binary_sha256:binaryHash,eleven_gates_same_inventory:true,frozen_and_history_intact:true,
  same_35_red_green_tests:true,workspace:{passed:totals[0],failed:totals[1],ignored:totals[2]},
  historical:{cells:504,changed:historicalChanged,preserved:504-historicalChanged,new:112,newChanged},
  cli,lint:{...lintCounts,added:0,removed:0},gates,
  limits:['No isolation attestation or refinement seal','Only intrinsic abs name and explicit transitive effects',
    'Remaining guards/gradient/show/imported-math debt is not full parity','Step file not read by reviewer'],
  pass:true},null,2));
