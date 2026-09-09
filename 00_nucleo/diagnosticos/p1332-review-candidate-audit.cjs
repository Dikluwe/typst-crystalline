const fs = require('fs'), crypto = require('crypto'), assert = require('assert');
const d = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p, 'utf8');
const json = p => JSON.parse(read(d + p));
const sha = s => crypto.createHash('sha256').update(s).digest('hex');
const freeze = json('p1332-ab-freeze.json');
const baseline = json('p1332-baseline.json');
const reseal = json('p1332-code-reseal.json');
const green = json('p1332-unit-green.json');
assert.equal(sha(read(d + 'p1332-code-reseal.json')), '4728524d1deb5ecb625734540cbd953bebb57d33804b7c3aaacf9ff88f46b0d9');
for (const [f,h] of Object.entries({...freeze.artifacts,...freeze.inputs})) assert.equal(sha(read(d+f)),h,f);
const owner = read('01_core/src/compiler/stdlib/calc.rs');
const norm = s => s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m,'');
assert.equal(sha(norm(owner)), '0f671ea97c0408df0bddeead179cf44a8787cef780c3fbe8bd25b4c7f21a9cec');
assert.equal(sha(norm(owner)),reseal.hash_b);
let restored = owner;
for (const {source,successor} of freeze.migration_ledger) {
  const snippet=read(d+successor).trim();
  assert.equal(restored.split(snippet).length-1,1);
  restored=restored.replace(snippet,read(d+source).trim());
}
const addition=read(d+'p1332-ab-tests.rs').trim();
assert.equal(restored.split(addition).length-1,1);
restored=restored.replace(addition,'').trimEnd()+'\n';
const candidate='dict.insert("abs".into(), Value::Func(Func::native("abs", calc_abs)));';
const previous='dict.insert("abs".into(), Value::Func(Func::native("calc.abs", calc_abs)));';
assert.equal(restored.split(candidate).length-1,1);
assert.equal(norm(restored.replace(candidate,previous)),norm(baseline.original_owner));
assert.equal(green.exit,0);
assert.ok(green.stdout.includes('35 passed; 0 failed;'));
assert.ok(green.stderr.includes('Finished `release` profile'));
assert.equal(green.manifest_sha256,sha(read(d+'p1332-manifest.json')));
assert.deepEqual(green.before.product_inventory,green.after.product_inventory);
assert.deepEqual(green.before.product_inventory,reseal.after.product_inventory);
assert.equal(sha(read(d+'p1332-unit-red.json')),reseal.red_sha256);
assert.equal(sha(read(d+'p1332-review-red.md')),reseal.red_review_sha256);
const gates={};
for (const suffix of ['fmt','lineage','lineage-lint','diff-check','lint']) {
  const filename='p1332-final-'+suffix+'.json', receipt=json(filename);
  assert.equal(receipt.exit,0,filename);
  if(receipt.before) assert.deepEqual(receipt.before.product_inventory,green.before.product_inventory,filename);
  if(receipt.after) assert.deepEqual(receipt.after.product_inventory,green.before.product_inventory,filename);
  gates[suffix]=sha(read(d+filename));
}
console.log(JSON.stringify({at:new Date().toISOString(),manifest_sha256:green.manifest_sha256,
  candidate_hash:sha(norm(owner)),green_sha256:sha(read(d+'p1332-unit-green.json')),
  green_start:green.at,green_end:green.end,production_delta:'only abs intrinsic registration',
  preserved_frozen_artifacts:true,gates},null,2));
