const fs = require('fs');
const crypto = require('crypto');
const D = '00_nucleo/diagnosticos/';
const hash = value => crypto.createHash('sha256').update(value).digest('hex');
const read = path => {
  if (/00_nucleo\/(?:materialization|context)\//.test(path)) throw Error('restricted input');
  return fs.readFileSync(path);
};
const json = name => JSON.parse(read(D + name));
const digest = path => hash(read(path));
const assert = (condition, label) => { if (!condition) throw Error(label); };
const baseline = json('p1330-baseline.json');
const manifest = json('p1330-manifest-r2.json');
const manifestHash = digest(D + 'p1330-manifest-r2.json');
assert(digest(D + 'p1330-baseline.json') === manifest.baseline_sha256, 'baseline');
assert(digest(D + 'p1329-closure.json') === manifest.previous_closure_sha256, 'predecessor');
const names = ['final-build', 'final-unit-green', 'workspace-tests', 'final-fmt-resealed',
  'final-lint', 'final-lineage', 'final-lineage-lint', 'final-diff-check',
  'cli-normal', 'cli-repeat', 'cli-reverse'];
const current = Object.fromEntries(Object.keys(baseline.state.product_inventory)
  .map(path => [path, digest(path)]));
const gates = names.map(name => {
  const path = D + 'p1330-' + name + '.json';
  const receipt = JSON.parse(read(path));
  for (const phase of ['before', 'after']) {
    assert(JSON.stringify(receipt[phase].product_inventory) === JSON.stringify(current), name + '/' + phase);
  }
  assert(receipt.exit === 0 && receipt.manifest_sha256 === manifestHash, name);
  return { name, path, sha256: digest(path), at: receipt.at, end: receipt.end, exit: receipt.exit };
});
const changes = Object.keys(current).filter(path => current[path] !== baseline.state.product_inventory[path]);
assert(JSON.stringify(changes.sort()) === JSON.stringify([manifest.prompt, manifest.owner].sort()), 'scope');
const integration = json('p1330-test-integration.json');
for (const [path, value] of Object.entries(integration.frozen)) assert(digest(path) === value, 'freeze ' + path);
for (const [path, value] of Object.entries(baseline.historical_preserved)) assert(digest(path) === value, 'history ' + path);
const src = read(manifest.owner).toString();
const prompt = read(manifest.prompt).toString();
const codeNorm = s => s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m, '');
const promptNorm = s => s.replace(/^Hash do Código: [0-9a-f]{8}\n/m, '');
const hashA = hash(promptNorm(prompt));
const hashB = hash(codeNorm(src));
assert(hashA === manifest.prompt_norm_sha256 && src.includes('@prompt-hash ' + hashA.slice(0, 8)), 'hash A');
assert(prompt.includes('Hash do Código: ' + hashB.slice(0, 8)), 'hash B');
const old = read(D + 'p1329-ab-p1328-successor.rs').toString().trim();
const successor = read(D + 'p1330-ab-p1328-successor.rs').toString().trim();
const snippet = read(D + 'p1330-ab-tests.rs').toString().trim();
const intArm = src.slice(src.indexOf('        [Value::Int(i)] =>'), src.indexOf('        [Value::Float(f)] =>'));
const expectedSource = (baseline.original_owner.replace(old, successor).trimEnd() + '\n\n' + snippet + '\n')
  .replace('        [Value::Int(i)] => Ok(Value::Int(i.saturating_abs())),\n', intArm);
assert(codeNorm(expectedSource) === codeNorm(src), 'exact candidate scope');
const frozen = json('p1330-ab-cli-expected.json');
const key = row => row.case + '/' + row.profile;
const observed = result => ({exit: result.exit, stdout: result.stdout, stderr: result.stderr});
const expected = new Map(frozen.expectations.map(row => [key(row), row.expected]));
assert(expected.size === frozen.expectations.length, 'unique expectations');
const binaryHash = digest(manifest.target + '/release/typst');
const cli = ['normal', 'repeat', 'reverse'].map(order => {
  const name = 'p1330-ab-cli-' + order + '.json';
  const data = json(name);
  assert(data.binaries.CANDIDATE.sha256 === binaryHash && data.manifest_sha256 === manifestHash, 'cli provenance');
  assert(data.l0_norm_sha256 === hashA && data.expected_sha256 === digest(D + 'p1330-ab-cli-expected.json'), 'cli freeze');
  const seen = new Set();
  for (const row of data.cases) {
    const k = key(row);
    assert(!seen.has(k), 'cli duplicate'); seen.add(k);
    assert(JSON.stringify(observed(row.results.CANDIDATE)) === JSON.stringify(expected.get(k)), 'cli mismatch ' + k);
  }
  assert(seen.size === expected.size, 'cli missing');
  return {order, observations:seen.size, sha256:digest(D + name)};
});
const workspace = json('p1330-workspace-tests.json');
const summaries = [...workspace.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)]
  .map(m => ({passed:+m[1], failed:+m[2], ignored:+m[3]}));
console.log(JSON.stringify({at:new Date().toISOString(), verdict:'PASS_AUDIT', head:baseline.state.head,
  working_tree:'uncommitted', diff_stat:workspace.before.diff_stat, manifest_sha256:manifestHash,
  current_product_inventory_count:Object.keys(current).length, current_product_inventory_sha256:hash(JSON.stringify(current)), product_changes:changes, hash_a:hashA, hash_b:hashB,
  owner_sha256:digest(manifest.owner), prompt_sha256:digest(manifest.prompt), binary_sha256:binaryHash,
  gates, cli, workspace_summaries:summaries, historical_files_verified:Object.keys(baseline.historical_preserved).length,
  frozen_artifacts_verified:Object.keys(integration.frozen).length,
  limits:['No isolated sandbox attestation or refinement seal', 'Step file not read by reviewer', 'Only recorded overflow scope']}, null, 2));
