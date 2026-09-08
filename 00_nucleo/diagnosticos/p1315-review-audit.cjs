// Read-only independent receipt audit. Run from the repository root.
const fs = require('fs');
const crypto = require('crypto');
const cp = require('child_process');
const prefix = '00_nucleo/diagnosticos/p1315-';
const read = p => JSON.parse(fs.readFileSync(p));
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const errors = [];
const check = (condition, label) => { if (!condition) errors.push(label); };
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const obs = r => ({ exit: r.exit, stdout: r.stdout, stderr: r.stderr });
const freezePath = prefix + 'ab-freeze.json';
const freeze = read(freezePath);
check(sha(freezePath) === 'c2cd0c90e4ea09082980b7dba5ed960dc8c9415bc4930fbeb6c68c2566ddac4f', 'freeze identity');
for (const [p, hash] of Object.entries(freeze.inputs)) check(sha(p) === hash, 'frozen input: ' + p);
for (const [name, binary] of Object.entries(freeze.binaries)) check(sha(binary.path) === binary.sha256, 'frozen binary: ' + name);
const l0 = fs.readFileSync(freeze.l0.path, 'utf8');
const metadata = l0.match(/^Hash do Código: [0-9a-f]+\r?\n/gm) || [];
check(metadata.length === 1, 'single L0 metadata line');
const normative = crypto.createHash('sha256').update(l0.replace(/^Hash do Código: [0-9a-f]+\r?\n/m, '')).digest('hex');
check(normative === freeze.l0.normative_sha256, 'normative L0');
const gates = {};
for (const name of ['unit-green', 'build', 'workspace-tests', 'lint', 'lineage-final', 'fmt', 'diff-check']) {
  const path = prefix + name + '.json';
  const r = read(path);
  check(r.exit === 0, 'gate exit: ' + name);
  check(r.before.head === 'bc8213f36b7a29b4fdc30cfc74ddc23586117c64' && r.after.head === r.before.head, 'gate HEAD: ' + name);
  check(same(r.before.files, r.after.files), 'gate source drift: ' + name);
  for (const [p, hash] of Object.entries(r.before.files)) check(sha(p) === hash, 'gate current source: ' + name + '/' + p);
  check(sha(prefix + 'record.py') === r.recorder_sha256, 'gate recorder: ' + name);
  const results = [...r.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/g)];
  const count = results.reduce((a, m) => a.map((n, i) => n + Number(m[i + 1])), [0, 0, 0]);
  gates[name] = { sha256: sha(path), argv: r.argv, exit: r.exit, utc: [r.before.utc, r.after.utc] };
  if (results.length) gates[name].tests = { passed: count[0], failed: count[1], ignored: count[2], summaries: results.length };
  if (name === 'lint') gates[name].diagnostics = { errors: (r.stdout.match(/^error:/gm) || []).length, warnings: (r.stdout.match(/^warning:/gm) || []).length, infos: (r.stdout.match(/^info:/gm) || []).length };
  if (name === 'lineage-final') check(r.stdout === 'Nothing to fix\n', 'lineage dry-run');
}
const candidate = { path: '/dev/shm/p1315-target.V6TWEF/release/typst', sha256: '6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1' };
check(sha(candidate.path) === candidate.sha256, 'candidate binary');
const runPath = prefix + 'ab-candidate-runs.json';
const comparisonPath = prefix + 'ab-comparison.json';
let ab = { pending: true };
if (fs.existsSync(runPath) && fs.existsSync(comparisonPath)) {
  const r = read(runPath), c = read(comparisonPath);
  check(r.cases_sha256 === sha(prefix + 'ab-cases.json') && r.runner_sha256 === sha(prefix + 'ab-runner.py'), 'A/B collection inputs');
  check(r.l0_normative_sha256 === normative, 'A/B collection L0');
  check(same(r.binaries.candidate, candidate), 'A/B candidate identity');
  const expected = new Map(freeze.expected.map(e => [[e.id, e.profile].join('|'), e]));
  check(expected.size === freeze.expected.length, 'duplicate frozen expectation');
  const profiles = { default: [], html: ['--features', 'html'], a11y: ['--features', 'a11y-extras'], 'html+a11y': ['--features', 'html,a11y-extras'] };
  const seen = new Set();
  for (const row of r.rows) {
    const key = [row.id, row.profile, row.order].join('|');
    check(!seen.has(key), 'duplicate observation: ' + key); seen.add(key);
    const e = expected.get([row.id, row.profile].join('|'));
    check(!!e, 'unexpected observation: ' + key);
    check(['normal', 'repeat', 'reverse'].includes(row.order), 'unexpected order: ' + key);
    if (!e) continue;
    check(row.product === 'candidate' && row.cwd === e.cwd, 'observation route: ' + key);
    check(same(row.argv, [candidate.path, 'eval', e.expr, ...profiles[row.profile]]), 'full argv/profile: ' + key);
    check(same(obs(row), e.expected), 'observation output: ' + key);
  }
  check(seen.size === expected.size * 3, 'complete observation count');
  check(c.freeze_sha256 === sha(freezePath) && c.runs_sha256 === sha(runPath), 'comparison receipt identity');
  check(c.status === 'PASS' && c.unknown === 0 && c.failures.length === 0 && c.comparisons === seen.size, 'comparison receipt result');
  ab = { pending: false, runs_sha256: sha(runPath), comparison_sha256: sha(comparisonPath), comparisons: seen.size, independently_recounted_failures: errors.filter(e => e.startsWith('observation output')).length };
} else errors.push('A/B artifacts pending');
console.log(JSON.stringify({ utc: new Date().toISOString(), head: cp.execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(), diff_stat: cp.execFileSync('git', ['diff', 'HEAD', '--stat'], { encoding: 'utf8' }), audit_script_sha256: sha(__filename), freeze_sha256: sha(freezePath), l0_normative_sha256: normative, candidate, gates, ab, errors }, null, 2));
process.exitCode = errors.length ? 1 : 0;
