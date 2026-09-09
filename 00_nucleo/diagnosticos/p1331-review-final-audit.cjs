const fs=require('fs'),path=require('path'),crypto=require('crypto'),assert=require('assert');
const D=__dirname,ROOT=path.resolve(D,'../..');
const read=n=>fs.readFileSync(path.join(D,n),'utf8'),json=n=>JSON.parse(read(n));
const sha=x=>crypto.createHash('sha256').update(x).digest('hex');
const hash=p=>sha(fs.readFileSync(p)),dh=n=>hash(path.join(D,n));
const manifest=json('p1331-manifest-r2.json'),mh=dh('p1331-manifest-r2.json');
assert.equal(mh,'6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9');
const baseline=json('p1331-baseline.json'),integration=json('p1331-test-integration-r2.json');
const green=json('p1331-unit-green-r2.json'),red=json('p1331-unit-red-r2.json');
const snapshot=green.after;
// Current git HEAD/index are checked independently in the shell: this host
// disallows Node child_process despite returning the subprocess output.
assert.equal(snapshot.head,manifest.head);
assert.equal(snapshot.staged,baseline.state.staged);
for(const [p,h]of Object.entries(snapshot.product_inventory))assert.equal(hash(path.join(ROOT,p)),h,p);
for(const [p,h]of Object.entries(baseline.state.product_inventory))if(![manifest.owner,manifest.prompt].includes(p))assert.equal(snapshot.product_inventory[p],h,p);
for(const [p,h]of Object.entries(baseline.historical_preserved))assert.equal(hash(p),h,p);
for(const [p,h]of Object.entries(integration.frozen))assert.equal(hash(p),h,p);
for(const [p,h]of Object.entries(manifest.frozen_r1))assert.equal(dh(p),h,p);
assert.equal(dh('p1330-closure.json'),manifest.previous_closure_sha256);
assert.equal(dh('p1331-baseline.json'),manifest.baseline_sha256);
const owner=fs.readFileSync(path.join(ROOT,manifest.owner),'utf8');
const prompt=fs.readFileSync(path.join(ROOT,manifest.prompt),'utf8');
const normCode=s=>s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m,'');
const A=sha(prompt.replace(/^Hash do Código: [0-9a-f]{8}\n/m,'')),B=sha(normCode(owner));
assert.equal(A,manifest.prompt_norm_sha256);
assert.equal(B,'8d7359a9585e354c370c032b573a7133d004806b3a3c84cd881f8665bb992521');
assert.equal(owner.match(/^\/\/! @prompt-hash ([0-9a-f]{8})$/m)[1],A.slice(0,8));
assert.equal(prompt.match(/^Hash do Código: ([0-9a-f]{8})$/m)[1],B.slice(0,8));
for(const n of ['p1331-ab-tests-r2.rs','p1331-ab-p1328-successor.rs','p1329-ab-tests-r1.rs','p1330-ab-tests.rs'])assert.equal(owner.split(read(n).trim()).length-1,1,n);
assert.equal(sha(normCode(owner.replace(read('p1331-ab-tests-r2.rs').trim(),read('p1331-ab-tests.rs').trim()))),'35e5681ac54d5207d71329d0da0560bfdeda83f0026030474f85e8c100244c7e');
const gateNames=['final-build-r2','unit-green-r2','workspace-tests-r2','final-fmt-r2','final-lint-r2','final-lineage-r2','final-lineage-lint-r2','final-diff-check-r2','cli-normal-r2','cli-repeat-r2','cli-reverse-r2'];
const gates={};
for(const n of gateNames){const r=json(`p1331-${n}.json`);assert.equal(r.exit,0,n);assert.equal(r.manifest_sha256,mh,n);assert.deepStrictEqual(r.before.product_inventory,snapshot.product_inventory,n);assert.deepStrictEqual(r.after.product_inventory,snapshot.product_inventory,n);assert.equal(r.before.head,manifest.head,n);assert.equal(r.after.head,manifest.head,n);gates[n]={sha256:dh(`p1331-${n}.json`),argv:r.argv,at:r.at,end:r.end,exit:r.exit};}
const fixture=json('p1331-fixture-path-r2.json');
assert.equal(fixture.exit,0);assert.equal(red.exit,101);
assert(fixture.stdout.includes('1 passed; 0 failed'));assert(red.stdout.includes('25 passed; 6 failed'));
assert(green.stdout.includes('31 passed; 0 failed'));
assert.deepStrictEqual(fixture.before.product_inventory,fixture.after.product_inventory);
assert.deepStrictEqual(fixture.after.product_inventory,red.before.product_inventory);
assert.deepStrictEqual(red.before.product_inventory,red.after.product_inventory);
assert.deepStrictEqual(red.before.product_inventory,integration.after.product_inventory);
assert(Date.parse(fixture.end)<Date.parse(red.at));assert(Date.parse(red.end)<Date.parse(green.at));
const testNames=s=>[...s.matchAll(/^test (\S+) \.\.\./gm)].map(x=>x[1]).sort();
assert.deepStrictEqual(testNames(red.stdout),testNames(green.stdout));
const workspace=json('p1331-workspace-tests-r2.json');
const totals=[...workspace.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)].reduce((a,m)=>a.map((n,i)=>n+Number(m[i+1])),[0,0,0]);
assert.deepStrictEqual(totals,[6717,0,3]);
const expected=json('p1331-ab-cli-expected-r2.json').expectations,oldExpected=json('p1330-ab-cli-expected.json').expectations;
assert.deepStrictEqual(expected,json('p1331-ab-cli-expected.json').expectations);
const key=x=>`${x.case}/${x.profile}`,map=xs=>new Map(xs.map(x=>[key(x),x]));
const em=map(expected);assert.equal(em.size,504);assert.equal(expected.length,504);
const om=map(oldExpected);assert.equal(om.size,332);
const changedHistory=[...om].filter(([k,v])=>JSON.stringify(v.expected)!==JSON.stringify(em.get(k).expected)).map(([k])=>k);
assert.equal(changedHistory.length,8);assert(changedHistory.every(k=>/^(string|symbol)\//.test(k)));
const observed=x=>({exit:x.exit,stdout:x.stdout,stderr:x.stderr});
const eq=(x,y)=>JSON.stringify(x)===JSON.stringify(y);
const candidateHash=hash(path.join(manifest.target,'release/typst'));
assert.equal(candidateHash,'a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47');
assert.equal(hash(manifest.binaries.baseline.path),manifest.binaries.baseline.sha256);
assert.equal(hash(manifest.binaries.vanilla.path),manifest.binaries.vanilla.sha256);
let firstRows;const cli={};
for(const order of ['normal','repeat','reverse']){
 const n=`p1331-ab-cli-${order}-r2.json`,r=json(n);
 assert.equal(r.manifest_sha256,mh);assert.equal(r.l0_norm_sha256,A);assert.equal(r.expected_sha256,dh('p1331-ab-cli-expected-r2.json'));
 assert.equal(r.binaries.CANDIDATE.sha256,candidateHash);assert.equal(r.binaries.BASE.sha256,manifest.binaries.baseline.sha256);assert.equal(r.binaries.VANILLA.sha256,manifest.binaries.vanilla.sha256);
 assert.equal(r.cases.length,504);assert.equal(map(r.cases).size,504);
 const counts={baselineFull:0,candidateFull:0,changed:0,regressions:0,correctedWithTraceDebt:0,remaining:0};
 for(const row of r.cases){const k=key(row),c=observed(row.results.CANDIDATE),b=observed(row.results.BASE),v=observed(row.results.VANILLA);assert.deepStrictEqual(c,em.get(k).expected,k);assert(row.candidate_matches_frozen_policy,k);counts.baselineFull+=eq(b,v);counts.candidateFull+=eq(c,v);counts.changed+=!eq(c,b);counts.regressions+=eq(b,v)&&!eq(c,v);counts.remaining+=!eq(c,v);if(!eq(c,b)&&!eq(c,v)){assert(['fallback-with-bound','fallback-with-nested','fallback-arguments-spread'].includes(row.case));assert.deepStrictEqual({...c,stderr:c.stderr.replace('while calling `calc.abs`','while calling `abs`')},v);counts.correctedWithTraceDebt++;}if(om.has(k))assert.deepStrictEqual(b,om.get(k).expected,k);}
 assert.deepStrictEqual(counts,{baselineFull:252,candidateFull:404,changed:164,regressions:0,correctedWithTraceDebt:12,remaining:100});
 const orderedKeys=r.cases.map(key);
 if(order!=='normal'){const firstMap=map(firstRows);for(const row of r.cases)for(const role of ['BASE','VANILLA','CANDIDATE'])assert.deepStrictEqual(observed(row.results[role]),observed(firstMap.get(key(row)).results[role]),`${order}/${key(row)}/${role}`);}
 if(order==='normal')firstRows=r.cases;
 else if(order==='repeat')assert.deepStrictEqual(orderedKeys,firstRows.map(key));
 else {const normalCases=[...new Set(firstRows.map(x=>x.case))];assert.deepStrictEqual([...new Set(r.cases.map(x=>x.case))],normalCases.reverse());}
 cli[order]={sha256:dh(n),utc:r.utc,observations:r.cases.length,...counts};
}
const results=n=>JSON.parse(json(n).stdout).runs.flatMap(x=>x.results),oldLint=results('p1330-final-lint.json'),newLint=results('p1331-final-lint-r2.json');
const signature=x=>JSON.stringify({rule:x.ruleId,level:x.level,message:x.message,uris:x.locations.map(l=>l.physicalLocation.artifactLocation.uri)});
const multiset=xs=>xs.reduce((m,x)=>(m.set(signature(x),(m.get(signature(x))||0)+1),m),new Map());
const omLint=multiset(oldLint),nmLint=multiset(newLint);let added=[];
for(const [k,n]of omLint)assert((nmLint.get(k)||0)>=n,k);
for(const [k,n]of nmLint)for(let z=omLint.get(k)||0;z<n;z++)added.push(JSON.parse(k));
assert.equal(added.length,1);assert.equal(added[0].rule,'V16');assert.equal(added[0].level,'note');assert(added[0].message.text.includes('other.type_name()'));
const lintCounts=newLint.reduce((a,r)=>(a[r.level]=(a[r.level]||0)+1,a),{error:0,warning:0,note:0});
assert.deepStrictEqual(lintCounts,{error:0,warning:240,note:1146});
assert.equal(dh('p1331-ab-receipt.md'),'d9a698acc2ca5f07df8400738199addad072320c7372508dcf78d3f835bdc5cd');
assert(read('p1331-ab-receipt.md').includes('Veredito: PASS no fragmento CLI congelado'));
console.log(JSON.stringify({at:new Date().toISOString(),head:manifest.head,working_tree:'uncommitted',manifest_sha256:mh,report_sha256:dh('p1331-final-report.md'),snapshot_provenance:'p1331-unit-green-r2.json after, identical to every final gate before/after',norm:A,owner_norm:B,candidate_sha256:candidateHash,all_inputs_and_history_intact:true,scope_c2_identical_to_reviewed_c1_except_fixture:true,red_green_same_31_tests:true,workspace:{passed:totals[0],failed:totals[1],ignored:totals[2]},historical:{migrated:changedHistory,preserved:324,new:172},cli,lint:{...lintCounts,removed:0,added},gates,pass:true},null,2));
