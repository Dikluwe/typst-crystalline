// Independent read-only audit. Run from repository root with node.
const fs = require('node:fs');
const crypto = require('node:crypto');
const cp = require('node:child_process');
const directory = '00_nucleo/diagnosticos/';
const load = name => JSON.parse(fs.readFileSync(directory + name, 'utf8'));
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const sha = path => hash(fs.readFileSync(path));
const obs = row => ({exit: row.exit, stdout: row.stdout, stderr: row.stderr});
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const key = (...xs) => xs.join('|');
const failures = [];
const requireCheck = (condition, message) => { if (!condition) failures.push(message); };
const measure = load('p1316-measurement.json');
const freeze = load('p1316-ab-freeze.json');
const baseline = load('p1316-ab-baseline-final.json');
const cases = load('p1316-ab-cases-r1.json').cases;
const previous = load('p1315-ab-candidate-runs.json');
for (const [path, digest] of Object.entries(freeze.inputs)) {
  requireCheck(sha(path) === digest, `Changed freeze input: ${path}`);
}
for (const [path, digest] of Object.entries(measure.prior_artifacts)) {
  requireCheck(sha(path) === digest, `Changed previous artifact: ${path}`);
}
const l0 = fs.readFileSync(freeze.l0.path, 'utf8');
requireCheck(hash(l0.replace(/^Hash do Código:.*\n/m, '')) === freeze.l0.normative_sha256,
  'Changed normative L0');
const rows = new Map(baseline.rows.map(r => [key(r.id, r.profile, r.product), r]));
const previousRows = new Map(previous.rows.filter(r => r.order === 'normal')
  .map(r => [key(r.id, r.profile), r]));
const expectations = new Map(freeze.expected.map(e => [key(e.id, e.profile), e]));
requireCheck(rows.size === baseline.rows.length, 'Duplicate baseline observations');
requireCheck(expectations.size === freeze.expected.length, 'Duplicate expectations');
for (const item of cases) for (const profile of ['default', 'html', 'a11y', 'html+a11y']) {
  const b = rows.get(key(item.id, profile, 'baseline'));
  const e = expectations.get(key(item.id, profile));
  requireCheck(Boolean(b && e), `Missing baseline/expectation: ${item.id}/${profile}`);
  if (!b || !e) continue;
  if (item.historical) {
    const p = previousRows.get(key(item.prior_id, profile));
    requireCheck(Boolean(p && p.cwd === b.cwd && p.argv[2] === b.argv[2]
      && same(obs(p), obs(b))), `Changed replay: ${item.id}/${profile}`);
  }
  const wanted = obs(b);
  if (item.kind === 'origin') {
    const vanilla = rows.get(key(item.id, profile, 'vanilla'));
    requireCheck(Boolean(vanilla && vanilla.stderr.startsWith('error: failed to parse CSV (')),
      `Wrong vanilla stratum: ${item.id}/${profile}`);
    if (!vanilla) continue;
    wanted.stderr = b.stderr.slice(0, b.stderr.indexOf('\n') + 1)
      + vanilla.stderr.slice(vanilla.stderr.indexOf('\n') + 1);
  }
  requireCheck(same(wanted, e.expected), `Wrong expectation: ${item.id}/${profile}`);
  requireCheck(e.expr === item.expr && e.cwd === item.cwd,
    `Wrong expected input: ${item.id}/${profile}`);
}
const sourcePath = '01_core/src/compiler/stdlib/loading.rs';
const source = fs.readFileSync(sourcePath, 'utf8');
const oldSource = measure.sources[sourcePath].text;
const stripLineage = text => text.replace(/^\/\/! @prompt-hash.*\n/m, '');
requireCheck(stripLineage(source.split('pub fn native_csv(')[0])
  === stripLineage(oldSource.split('pub fn native_csv(')[0]),
  'Production code before native_csv changed');
const previousTestMarker = '    #[test]\n    fn p1315_csv_unequal_lengths_use_record_ordinal';
requireCheck(source.includes(previousTestMarker) && oldSource.includes(previousTestMarker),
  'Previous tests marker missing');
requireCheck(source.slice(source.indexOf(previousTestMarker))
  === oldSource.slice(oldSource.indexOf(previousTestMarker)), 'Previous tests changed');
let candidateComparisons = null;
const candidatePath = directory + 'p1316-ab-candidate-runs.json';
if (fs.existsSync(candidatePath)) {
  const candidate = load('p1316-ab-candidate-runs.json');
  const unique = new Set();
  for (const row of candidate.rows) {
    const item = expectations.get(key(row.id, row.profile));
    requireCheck(Boolean(item && row.product === 'candidate' && row.argv[2] === item.expr
      && row.cwd === item.cwd && same(obs(row), item.expected)),
      `Candidate observation mismatch: ${row.id}/${row.profile}/${row.order}`);
    unique.add(key(row.id, row.profile, row.order));
  }
  requireCheck(candidate.rows.length === expectations.size * 3
    && unique.size === candidate.rows.length, 'Missing/duplicate candidate observations');
  for (const item of freeze.expected) for (const order of ['normal', 'repeat', 'reverse']) {
    requireCheck(unique.has(key(item.id, item.profile, order)), 'Missing required order');
  }
  candidateComparisons = candidate.rows.length;
}
console.log(JSON.stringify({
  utc: new Date().toISOString(),
  head: cp.execFileSync('git', ['rev-parse', 'HEAD'], {encoding: 'utf8'}).trim(),
  diff_stat: cp.execFileSync('git', ['diff', 'HEAD', '--stat'], {encoding: 'utf8'}),
  script_sha256: sha(__filename),
  freeze_sha256: sha(directory + 'p1316-ab-freeze.json'),
  source_sha256: sha(sourcePath), l0_raw_sha256: sha(freeze.l0.path),
  frozen_inputs_verified: Object.keys(freeze.inputs).length,
  previous_artifacts_verified: Object.keys(measure.prior_artifacts).length,
  cases: cases.length, historical_cases: cases.filter(c => c.historical).length,
  expectations: expectations.size,
  red_expectations: freeze.expected.filter(e => e.baseline_red).length,
  product_unchanged: oldSource.split('#[cfg(test)]')[0] === source.split('#[cfg(test)]')[0],
  candidate_comparisons: candidateComparisons,
  failures,
}, null, 2));
process.exitCode = failures.length ? 1 : 0;
