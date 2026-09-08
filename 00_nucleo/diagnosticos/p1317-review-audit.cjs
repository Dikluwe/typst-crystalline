// Independent read-only review; stdout is the audit receipt. Run at repository root.
const fs = require('node:fs');
const crypto = require('node:crypto');
const cp = require('node:child_process');
const D = '00_nucleo/diagnosticos/';
const load = name => JSON.parse(fs.readFileSync(D + name, 'utf8'));
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const sha = path => hash(fs.readFileSync(path));
const obs = r => ({exit: r.exit, stdout: r.stdout, stderr: r.stderr});
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const key = (...v) => v.join('|');
const failures = [];
const check = (condition, message) => { if (!condition) failures.push(message); };
const m = load('p1317-measurement.json');
const f = load('p1317-ab-freeze.json');
const b = load('p1317-ab-baseline-runs.json');
const cases = load('p1317-ab-cases.json').cases;
const red = load('p1317-unit-red.json');
const previous = load('p1316-ab-candidate-runs.json');
for (const [p, h] of Object.entries(f.inputs)) check(sha(p) === h, `Frozen input changed: ${p}`);
for (const [p, h] of Object.entries(m.prior_artifacts)) check(sha(p) === h, `Prior artifact changed: ${p}`);
for (const b of Object.values(f.binaries)) check(sha(b.path) === b.sha256, `Binary changed: ${b.path}`);
const sourcePath = '01_core/src/compiler/stdlib/loading.rs';
const source = fs.readFileSync(sourcePath, 'utf8');
const old = m.sources[sourcePath].text;
const l0 = fs.readFileSync(f.l0.path, 'utf8');
check(hash(l0.replace(/^Hash do Código: [^\r\n]+\r?\n/m, '')) === f.l0.normative_sha256, 'Normative L0 changed');
const branch = '            csv::ErrorKind::Utf8 { .. } => {\n                err("failed to parse CSV (file is not valid UTF-8)")\n            }\n';
check(source.split(branch).length === 2, 'Utf8 mapping is absent or duplicated');
const undo = source.replace(branch, '').replace(/^\/\/! @prompt-hash.*$/m, old.match(/^\/\/! @prompt-hash.*$/m)[0]);
check(hash(undo) === red.after.files[sourcePath], 'Candidate differs from RED beyond Utf8 mapping and lineage');
const prod = s => s.split('#[cfg(test)]\nmod tests {')[0];
check(prod(undo) === prod(old), 'Other production changed');
const marker = '    fn p1316_bytes_args(';
check(source.includes(marker) && source.slice(source.indexOf(marker)) === old.slice(old.indexOf(marker)), 'Old tests changed');
check(red.exit === 101 && red.stdout.includes('1 passed; 3 failed;'), 'Missing genuine RED');
check(same(red.before.files, red.after.files), 'RED inputs drifted');
const baselines = new Map(b.rows.map(r => [key(r.id, r.profile, r.product), r]));
const previousRows = new Map(previous.rows.filter(r => r.order === 'normal').map(r => [key(r.id, r.profile), r]));
const expected = new Map(f.expected.map(e => [key(e.id, e.profile), e]));
check(baselines.size === b.rows.length, 'Duplicate baseline');
check(expected.size === f.expected.length && expected.size === cases.length * 4, 'Duplicate or missing expectation');
for (const c of cases) for (const p of ['default', 'html', 'a11y', 'html+a11y']) {
  const row = baselines.get(key(c.id, p, 'baseline'));
  const e = expected.get(key(c.id, p));
  check(Boolean(row && e), `Missing baseline/expectation: ${c.id}/${p}`);
  if (!row || !e) continue;
  check(e.expr === c.expr && e.cwd === c.cwd, `Input drift: ${c.id}/${p}`);
  if (c.historical) {
    const prior = previousRows.get(key(c.prior_id, p));
    check(Boolean(prior && prior.argv[2] === row.argv[2] && prior.cwd === row.cwd && same(obs(prior), obs(row))), `Historical replay drift: ${c.id}/${p}`);
  }
  const want = obs(row);
  if (c.kind === 'utf8') {
    check(row.exit === 1 && row.stdout === '' && row.stderr.startsWith('error: failed to parse CSV (CSV parse error:') && row.stderr.split('\n')[0].includes('invalid utf-8'), `Wrong baseline stratum: ${c.id}/${p}`);
    const v = baselines.get(key(c.id, p, 'vanilla'));
    check(Boolean(v && v.stderr.startsWith(c.normative_only ? 'error: unexpected argument' : 'error: failed to parse CSV (file is not valid UTF-8')), `Wrong vanilla stratum: ${c.id}/${p}`);
    want.stderr = 'error: failed to parse CSV (file is not valid UTF-8)' + row.stderr.slice(row.stderr.indexOf('\n'));
  }
  check(same(want, e.expected), `Expected observation drift: ${c.id}/${p}`);
}
let candidateComparisons = null;
let candidateBinary = null;
if (fs.existsSync(D + 'p1317-ab-candidate-runs.json')) {
  const run = load('p1317-ab-candidate-runs.json');
  candidateBinary = run.binaries.candidate;
  check(sha(candidateBinary.path) === candidateBinary.sha256, 'Candidate binary changed');
  check(run.runner_sha256 === sha(D + 'p1317-ab-runner.py') && run.cases_sha256 === sha(D + 'p1317-ab-cases.json'), 'Candidate runner/cases drift');
  check(run.l0_normative_sha256 === f.l0.normative_sha256, 'Candidate L0 drift');
  const seen = new Set();
  for (const r of run.rows) {
    const e = expected.get(key(r.id, r.profile));
    check(Boolean(e && r.product === 'candidate' && r.argv[2] === e.expr && r.cwd === e.cwd && same(obs(r), e.expected)), `Candidate mismatch: ${r.id}/${r.profile}/${r.order}`);
    seen.add(key(r.id, r.profile, r.order));
  }
  check(seen.size === run.rows.length && run.rows.length === expected.size * 3, 'Missing or duplicated candidate observations');
  for (const e of f.expected) for (const o of ['normal', 'repeat', 'reverse']) check(seen.has(key(e.id, e.profile, o)), `Missing order: ${e.id}/${e.profile}/${o}`);
  candidateComparisons = run.rows.length;
}
const gates = {};
for (const name of ['unit-green', 'build', 'workspace-tests', 'lint', 'fmt', 'diff-check', 'lineage-final']) {
  const p = D + `p1317-${name}.json`;
  if (!fs.existsSync(p)) { gates[name] = 'pending'; continue; }
  const r = JSON.parse(fs.readFileSync(p));
  gates[name] = {sha256: sha(p), exit: r.exit, argv: r.argv};
  check(r.exit === 0, `Gate failed: ${name}`);
  if (r.after && r.after.files && name !== 'lineage-final') check(r.after.files[sourcePath] === sha(sourcePath), `Gate source differs: ${name}`);
}
console.log(JSON.stringify({
  utc: new Date().toISOString(),
  head: cp.execFileSync('git', ['rev-parse', 'HEAD'], {encoding: 'utf8'}).trim(),
  diff_stat: cp.execFileSync('git', ['diff', 'HEAD', '--stat'], {encoding: 'utf8'}),
  script_sha256: sha(__filename), freeze_sha256: sha(D + 'p1317-ab-freeze.json'),
  source_sha256: sha(sourcePath), l0_raw_sha256: sha(f.l0.path),
  frozen_inputs_verified: Object.keys(f.inputs).length,
  previous_artifacts_verified: Object.keys(m.prior_artifacts).length,
  cases: cases.length, historical_cases: cases.filter(c => c.historical).length,
  expectations: expected.size, red_expectations: f.expected.filter(e => e.baseline_red).length,
  candidate_comparisons: candidateComparisons, candidate_binary: candidateBinary,
  gates, failures,
}, null, 2));
process.exitCode = failures.length ? 1 : 0;
