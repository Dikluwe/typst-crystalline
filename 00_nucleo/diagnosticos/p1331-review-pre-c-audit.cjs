const fs = require('fs');
const crypto = require('crypto');
const assert = require('assert');
const path = require('path');
const d = __dirname;
const root = path.resolve(d, '../..');
const read = n => fs.readFileSync(path.join(d,n), 'utf8');
const json = n => JSON.parse(read(n));
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const baseline = json('p1331-baseline.json');
const integration = json('p1331-test-integration.json');
const manifestHash = sha(path.join(d,'p1331-manifest.json'));
assert.equal(manifestHash,'837bb832ce13f0686a438f672c6fe001224738a15b1a369efcc6f7533b7db141');
assert.equal(integration.manifest_sha256,manifestHash);
for(const [p,h] of Object.entries(integration.frozen)) assert.equal(sha(p),h,p);
for(const [p,h] of Object.entries(baseline.historical_preserved)) assert.equal(sha(p),h,p);
const normalize = s => s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m,'');
const owner = fs.readFileSync(path.join(root,'01_core/src/compiler/stdlib/calc.rs'),'utf8');
const old = read('p1330-ab-p1328-successor.rs').trim();
const successor = read('p1331-ab-p1328-successor.rs').trim();
const fresh = read('p1331-ab-tests.rs').trim();
assert.equal(baseline.original_owner.split(old).length-1,1);
const wanted = baseline.original_owner.replace(old,successor).trimEnd()+'\n\n'+fresh+'\n';
assert.equal(normalize(owner),normalize(wanted),'exact pre-C owner');
for(const n of ['p1329-ab-tests-r1.rs','p1330-ab-tests.rs']) assert.equal(owner.split(read(n).trim()).length-1,1,n);
const prior=json('p1330-ab-cli-expected.json').expectations;
const expected=json('p1331-ab-cli-expected.json').expectations;
const rows=json('p1331-ab-cli-baseline.json').cases;
const key = x => `${x.case}/${x.profile}`;
const mapped = xs => new Map(xs.map(x=>[key(x),x]));
const pm=mapped(prior), em=mapped(expected), rm=mapped(rows);
assert.equal(pm.size,prior.length); assert.equal(em.size,expected.length); assert.equal(rm.size,rows.length);
assert.equal(prior.length,332); assert.equal(expected.length,504); assert.equal(rows.length,504);
const observed = x => ({exit:x.exit,stdout:x.stdout,stderr:x.stderr});
const changed=[];
for(const [k,p] of pm){
  assert(em.has(k)&&rm.has(k),k);
  assert.deepStrictEqual(observed(rm.get(k).results.BASE),p.expected,k);
  if(JSON.stringify(em.get(k).expected)!==JSON.stringify(p.expected)){
    assert(['string','symbol'].includes(p.case),k);
    assert.deepStrictEqual(em.get(k).expected,observed(rm.get(k).results.VANILLA),k);
    changed.push(k);
  }
}
assert.equal(changed.length,8);
const result={at:new Date().toISOString(),head:baseline.state.head,working_tree:'uncommitted',
  provenance:'p1331-test-integration.json before/after full state; p1331-baseline.json baseline state',
  manifest_sha256:manifestHash,integration_sha256:sha(path.join(d,'p1331-test-integration.json')),
  freeze_sha256:sha(path.join(d,'p1331-ab-freeze.md')),runtime_exact_baseline:true,
  old_p1329_p1330_snippets_exact:true,frozen_and_historical_intact:true,
  historical_cells:prior.length,migrated_cells:changed,preserved_cells:prior.length-changed.length,
  new_cells:expected.length-prior.length,pass:true};
console.log(JSON.stringify(result,null,2));
