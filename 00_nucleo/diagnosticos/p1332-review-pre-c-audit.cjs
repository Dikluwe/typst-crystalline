const fs = require('fs');
const crypto = require('crypto');
const assert = require('assert');
const d = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p, 'utf8');
const json = p => JSON.parse(read(d + p));
const sha = s => crypto.createHash('sha256').update(s).digest('hex');
const freeze = json('p1332-ab-freeze.json');
const integration = json('p1332-test-integration.json');
const baseline = json('p1332-baseline.json');
assert.equal(sha(read(d + 'p1332-ab-freeze.json')), '7e2e2350cefbf5316131cd8ff9ccc0baf53be0efb36d3a0a4ffe059990c21719');
assert.equal(sha(read(d + 'p1332-test-integration.json')), 'e77f14a829afa0e69dfe1dfb0df3315cd1bc62d3eb4c8f048576b6d2ad74f2c6');
for (const [file, hash] of Object.entries({...freeze.artifacts, ...freeze.inputs})) {
  assert.equal(sha(read(d + file)), hash, file);
}
for (const [file, hash] of Object.entries({...integration.frozen, ...integration.predecessors})) {
  assert.equal(sha(read(file)), hash, file);
}
const manifest = sha(read(d + 'p1332-manifest.json'));
assert.equal(freeze.manifest_sha256, manifest);
assert.equal(integration.manifest_sha256, manifest);
assert.equal(sha(read('00_nucleo/prompts/compiler/stdlib/calc.md').replace(/^Hash do Código: [0-9a-f]{8}\n/m, '')), freeze.l0_norm_sha256);
let restored = read('01_core/src/compiler/stdlib/calc.rs');
let lineChanges = 0;
for (const entry of freeze.migration_ledger) {
  const before = read(d + entry.source).split('\n');
  const after = read(d + entry.successor).split('\n');
  assert.equal(before.length, after.length);
  const changes = before.flatMap((line, i) => line === after[i] ? [] : [{line:i+1,before:line,after:after[i]}]);
  assert.deepEqual(changes, entry.changes);
  for (const change of changes) assert.equal(change.before.replaceAll('"calc.abs"', '"abs"'), change.after);
  lineChanges += changes.length;
  const snippet = read(d + entry.successor).trim();
  assert.equal(restored.split(snippet).length - 1, 1);
  restored = restored.replace(snippet, read(d + entry.source).trim());
}
const addition = read(d + 'p1332-ab-tests.rs').trim();
assert.equal(restored.split(addition).length - 1, 1);
restored = restored.replace(addition, '').trimEnd() + '\n';
const normSource = s => s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m, '');
assert.equal(normSource(restored), normSource(baseline.original_owner));
assert.equal(lineChanges, 7);
const raw = json('p1332-ab-cli-baseline.json');
const expected = json('p1332-ab-cli-expected.json');
const old = json('p1331-ab-cli-expected-r2.json');
const key = r => r.case + '/' + r.profile;
const map = rows => new Map(rows.map(r => [key(r), r]));
const rawMap = map(raw.cases), expectedMap = map(expected.expectations), oldMap = map(old.expectations);
assert.equal(rawMap.size, 616);
assert.equal(expectedMap.size, 616);
assert.equal(oldMap.size, 504);
assert.equal(expected.baseline_sha256, sha(read(d + 'p1332-ab-cli-baseline.json')));
assert.equal(expected.historical_expectations_sha256, sha(read(d + 'p1331-ab-cli-expected-r2.json')));
const obs = r => ({exit:r.exit, stdout:r.stdout, stderr:r.stderr});
let historicalChanged = 0, newChanged = 0, transitive = 0, importedMath = 0;
for (const [k, row] of rawMap) {
  const e = expectedMap.get(k);
  assert.ok(e, k);
  const actual = obs(row.results.BASE);
  if (oldMap.has(k)) assert.deepEqual(actual, oldMap.get(k).expected, k);
  const wanted = {...actual};
  wanted.stderr = wanted.stderr.replaceAll('  while calling `calc.abs` at ', '  while calling `abs` at ');
  if (['name-gradient-linear','name-gradient-radial','name-gradient-conic','name-show'].includes(row.case)) {
    const lines = wanted.stderr.split('\n');
    lines[0] = lines[0].replaceAll('Some("calc.abs")', 'Some("abs")').replaceAll("função 'calc.abs'", "função 'abs'");
    wanted.stderr = lines.join('\n');
    transitive++;
  }
  assert.deepEqual(e.expected, wanted, k);
  if (JSON.stringify(wanted) !== JSON.stringify(actual)) {
    if (oldMap.has(k)) historicalChanged++; else newChanged++;
  }
  if (row.case === 'name-math-import') {
    assert.equal(e.classification, 'preserved-debt-imported-math-resolution');
    assert.equal(actual.exit, 0);
    assert.equal(row.results.VANILLA.exit, 1);
    assert.deepEqual(e.expected, actual);
    importedMath++;
  }
}
assert.equal(historicalChanged, 84);
assert.equal(newChanged, 60);
assert.equal(transitive, 16);
assert.equal(importedMath, 4);
console.log(JSON.stringify({
  at:new Date().toISOString(), manifest_sha256:manifest,
  freeze_sha256:sha(read(d+'p1332-ab-freeze.json')),
  integration_sha256:sha(read(d+'p1332-test-integration.json')),
  input_hashes_match:true, snippets_exact:true, runtime_pre_c:true,
  historical_lines:lineChanges, cells:rawMap.size,
  historical_cells:oldMap.size, historicalChanged, newChanged,
  transitive, importedMath,
}, null, 2));
