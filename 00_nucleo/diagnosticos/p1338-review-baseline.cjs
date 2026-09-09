// Independent read-only P1338 baseline audit. Writes only JSON to stdout.
const fs = require('fs');
const crypto = require('crypto');
const path = require('path');
const root = '/repos/Antigravity/typst-crystalline';
const dir = path.join(root, '00_nucleo/diagnosticos');
const violations = [], counts = {}, inputs = {};
let checks = 0;
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
function safe(p) {
  const absolute = path.resolve(root, p);
  if (/\/00_nucleo\/(materialization|context)(\/|$)/.test(absolute)) throw Error('restricted input');
  return absolute;
}
function hash(p) { return sha(fs.readFileSync(safe(p))); }
function check(ok, label) { checks++; if (!ok) violations.push(label); }
function load(name, expected) {
  const p = path.join(dir, name), b = fs.readFileSync(p), h = sha(b);
  inputs[p] = h; if (expected) check(h === expected, `${name}: pin`);
  return JSON.parse(b);
}
function verifyMap(name, map) {
  counts[name] = Object.keys(map).length;
  for (const [p, h] of Object.entries(map)) {
    try { check(hash(p) === h, `${name}: ${p}`); }
    catch (e) { check(false, `${name}: ${p}: ${e.message}`); }
  }
}
const baseline = load('p1338-baseline.json', '6d85aece9e323950a8f722b11e87eb125b4a342a7404f969f60ea69595790b3c');
const closure = load('p1337-closure.json', baseline.previous_closure_sha256);
const post = load('p1337-review-postclosure.json', baseline.previous_review_sha256);
const manifest = load('p1338-manifest.json', '8c51d637e9b248628d4bfe32b684a00f201b3139b84868b1f6cb351bb6290e4b');
const measurement = load('p1338-measurement.json', manifest.measurement_sha256);
check(post.verdict === 'PASS_SCOPED' && post.violations.length === 0 && post.unknowns.length === 0, 'antecedent postclosure verdict');
check(baseline.state.head === closure.state.head && baseline.state.head === post.head, 'HEAD chain');
check(baseline.state.staged === '' && baseline.state.staged === closure.state.staged, 'staged unchanged');
check(JSON.stringify(baseline.state.product_inventory) === JSON.stringify(closure.state.product_inventory), 'product inventory baseline equals closure');
verifyMap('baseline_history', baseline.historical_preserved);
verifyMap('baseline_temporaries', baseline.retained_prior_artifacts);
for (const name of ['artifacts', 'historical_preserved', 'retained_mutant_artifacts', 'retained_prior_artifacts', 'predecessor_active_exports_preserved']) verifyMap('closure_' + name, closure[name]);
verifyMap('postclosure_inputs', post.inputs);
for (const [p, text] of Object.entries(baseline.snapshots)) check(sha(text) === closure.state.product_inventory[p], `snapshot closure: ${p}`);
for (const key of ['predecessor', 'vanilla']) { const b = baseline[key]; check(hash(b.path) === b.sha256, key + ' binary'); inputs[b.path] = b.sha256; }
check(baseline.predecessor.sha256 === closure.binaries.candidate.sha256, 'predecessor candidate chain');
check(baseline.vanilla.upstream === 'a51e02804', 'ratified upstream');
const source = '01_core/src/compiler/eval/bindings/field_access.rs';
const stripHeader = s => s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m, '');
check(stripHeader(manifest.pre_candidate_source) === stripHeader(baseline.snapshots[source]), 'manifest product and all tests unchanged except header');
check(sha(manifest.pre_candidate_source) === manifest.source_sha256, 'manifest source hash');
check(manifest.baseline_sha256 === inputs[path.join(dir, 'p1338-baseline.json')], 'manifest baseline pin');
check(measurement.baseline_sha256 === manifest.baseline_sha256, 'measurement baseline pin');
check(JSON.stringify(measurement.before.product_inventory) === JSON.stringify(baseline.state.product_inventory), 'measurement before inventory');
check(JSON.stringify(measurement.after.product_inventory) === JSON.stringify(baseline.state.product_inventory), 'measurement after inventory');
counts.measurement_cases = measurement.rows.length;
counts.measurement_processes = 0;
for (const row of measurement.rows) {
  check(sha(row.source) === row.source_sha256, 'measurement source ' + row.source);
  for (const [kind, run] of Object.entries(row.observations)) {
    counts.measurement_processes++;
    check(run.binary_sha256 === baseline[kind === 'baseline' ? 'predecessor' : 'vanilla'].sha256, 'measurement binary ' + row.source + kind);
    check([0,1].includes(run.exit), 'measurement valid exit ' + row.source + kind);
    for (const channel of ['stdout', 'stderr']) check(Buffer.from(run[channel]).equals(Buffer.from(run[channel + '_base64'], 'base64')), 'measurement channel ' + row.source + kind + channel);
    check(Date.parse(run.at) <= Date.parse(run.end), 'measurement timestamp ' + row.source + kind);
  }
}
inputs[__filename] = hash(__filename);
console.log(JSON.stringify({at: new Date().toISOString(), phase: 'baseline-and-initial-scope', verdict: violations.length ? 'HOLD' : 'BASELINE_CHAIN_VERIFIED_NOT_GO', checks, counts, violations, unknowns: [], inputs, limitations: ['No implementation GO: independent freezes and genuine RED still required.', 'Historical reviewer context retained, no test or product authorship; A/B without technical isolation attestation or refinement seal.', 'No step contents or hash read; no product gates rerun.', 'Current evolving L0/source are not asserted equal to baseline; frozen baseline and manifest snapshots are the pre-C targets.']}, null, 2));
