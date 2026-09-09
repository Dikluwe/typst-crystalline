// Read-only independent audit. Run from repo root:
// git show d31047d7b8af7837c84adae4ded3d2ff50c62093:01_core/src/compiler/stdlib/loading.rs | node 00_nucleo/diagnosticos/p1319-review-audit.cjs
// JSON goes to stdout. Exit 2 means required final receipts are still absent.
const fs = require('node:fs');
const crypto = require('node:crypto');
const root = '/repos/Antigravity/typst-crystalline';
const directory = `${root}/00_nucleo/diagnosticos`;
const owner = '01_core/src/compiler/stdlib/loading.rs';
const l0 = '00_nucleo/prompts/compiler/stdlib/loading.md';
const sha = data => crypto.createHash('sha256').update(data).digest('hex');
const read = path => fs.readFileSync(path.startsWith('/') ? path : `${root}/${path}`);
const json = name => JSON.parse(read(`${directory}/p1319-${name}.json`));
const assert = (value, message) => { if (!value) throw Error(message); };
const report = { schema: 'p1319-review-audit-v1', utc: new Date().toISOString(),
  regime: 'A/B without technical isolation attestation', checks: [], pending: [], receipts: {} };
const check = (value, message) => { assert(value, message); report.checks.push(message); };
const source = read(owner), prompt = read(l0), text = source.toString(), ptext = prompt.toString();
const sourceMarker = /^\/\/! @prompt-hash ([0-9a-f]{8})\n/gm;
const promptMarker = /^Hash do Código: ([0-9a-f]{8})\n/gm;
check([...text.matchAll(sourceMarker)].length === 1, 'one source metadata line');
check([...ptext.matchAll(promptMarker)].length === 1, 'one prompt metadata line');
const cleanedSource = text.replace(sourceMarker, '');
const cleanedPrompt = ptext.replace(promptMarker, '');
const frozen = json('ab-freeze');
check(sha(read(`${directory}/p1319-ab-freeze.json`)) === '61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f', 'freeze hash');
check(sha(cleanedPrompt) === frozen.l0.normative_sha256, 'frozen normative L0 unchanged');
check([...ptext.matchAll(promptMarker)][0][1] === sha(cleanedSource).slice(0, 8), 'reverse code hash B');
check(sha(source) === '15078b5441b514a200c54039589521cb4755b99a99a68b5a37983c247f759800', 'reviewed r2 source');
check(sha(prompt) === '1bd9633f0cdfb5a1ef7ca0265b55ac7ea8ad5b28c41035a2aaaa3572edff0dd1', 'reviewed r2 L0 bytes');

// Independently reproduce effective prompt A, including the one dependency-free nucleus.
const pins = [...ptext.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/[^\s]+) sha256:([0-9a-f]{64})$/gm)];
check(pins.length === 1, 'expected nucleus graph cardinality');
const nucleus = read(pins[0][1]);
check(!/^\[\[depends\]\]/m.test(nucleus.toString()), 'nucleus has no transitive dependencies');
const digest = crypto.createHash('sha256').update(nucleus).update(Buffer.from([0]))
  .update('TEKT-NUCLEUS-DEPS-V1').update(Buffer.from([0])).digest();
check(digest.toString('hex') === pins[0][2], 'effective nucleus pin');
const pathBytes = Buffer.from(pins[0][1]), size = Buffer.alloc(8);
size.writeBigUInt64BE(BigInt(pathBytes.length));
const a = crypto.createHash('sha256').update(cleanedPrompt).update(Buffer.from([0]))
  .update('TEKT-PROMPT-NUCLEI-V1').update(Buffer.from([0])).update(size)
  .update(pathBytes).update(Buffer.from([32])).update(digest).digest('hex');
check(a.slice(0, 8) === [...text.matchAll(sourceMarker)][0][1], 'forward effective prompt hash A');
report.hashes = { source: sha(source), prompt: sha(prompt), normative: sha(cleanedPrompt),
  source_without_header: sha(cleanedSource), effective_prompt: a, nucleus: digest.toString('hex') };

// Rebuild RED source from immutable baseline bytes on stdin and recorded unified diff.
const baseline = fs.readFileSync(0, 'utf8');
check(sha(baseline) === '870f861651c35f44afd5f3f68548bb0364463b2ca3d4d2087b5460bdbc986f5a', 'baseline source stdin identity');
const red = json('unit-red');
const segment = red.before.diff.split(`diff --git a/${owner} b/${owner}\n`)[1];
assert(segment, 'RED source delta missing');
const patch = segment.split('\ndiff --git ')[0].split('\n'), lines = baseline.split('\n');
let at = 0, result = [];
for (let i = 0; i < patch.length; i++) {
  const hunk = patch[i].match(/^@@ -(\d+)(?:,\d+)? \+\d+(?:,\d+)? @@/);
  if (!hunk) continue;
  const start = Number(hunk[1]) - 1;
  result.push(...lines.slice(at, start)); at = start;
  for (i++; i < patch.length && !patch[i].startsWith('@@'); i++) {
    const line = patch[i];
    if (line.startsWith(' ')) { assert(lines[at] === line.slice(1), 'RED context mismatch'); result.push(lines[at++]); }
    else if (line.startsWith('-')) { assert(lines[at] === line.slice(1), 'RED deletion mismatch'); at++; }
    else if (line.startsWith('+')) result.push(line.slice(1));
    else assert(line === '' || line.startsWith('\\'), 'unsupported RED patch record');
  }
  i--;
}
result.push(...lines.slice(at));
const redSource = result.join('\n'), tests = value => value.slice(value.indexOf('#[cfg(test)]'));
check(sha(redSource) === red.before.files[owner], 'RED full source reconstructed');
check(tests(redSource) === tests(text), 'entire test module unchanged since RED');
check(red.exit === 101 && /65 passed; 4 failed; 0 ignored/.test(red.stdout), 'genuine RED receipt');
report.hashes.test_module = sha(tests(text));

for (const [path, digest] of Object.entries(frozen.inputs)) check(sha(read(path)) === digest, `frozen input ${path}`);
report.freeze_counts = { cases: new Set(frozen.expected.map(e => e.id)).size,
  expectations: frozen.expected.length, red: frozen.expected.filter(e => e.baseline_red).length,
  vanilla: frozen.expected.filter(e => e.full_vanilla_parity).length,
  normative: frozen.expected.filter(e => e.normative_only).length };
check(new Set(frozen.expected.map(e => `${e.id}|${e.profile}`)).size === frozen.expected.length, 'unique freeze keys');

for (const name of ['build-r2', 'fmt-r2', 'diff-check-r2', 'lint-r2', 'lineage-final-r2',
  'lineage-verified-r2', 'unit-green-r2', 'workspace-tests-r2']) {
  const path = `${directory}/p1319-${name}.json`;
  if (!fs.existsSync(path)) { report.pending.push(name); continue; }
  const gate = JSON.parse(read(path));
  report.receipts[name] = { sha256: sha(read(path)), exit: gate.exit,
    argv: gate.argv, before_utc: gate.before.utc, after_utc: gate.after.utc };
  check(gate.exit === 0, `${name} exit zero`);
  for (const state of [gate.before, gate.after]) {
    check(state.files[owner] === sha(source) && state.files[l0] === sha(prompt), `${name} tested final source/L0`);
    check(state.head === 'd31047d7b8af7837c84adae4ded3d2ff50c62093', `${name} baseline HEAD`);
  }
  if (name === 'lineage-verified-r2') check(gate.bidirectional_pass === true, 'explicit reverse check passed');
  if (name === 'unit-green-r2') check(/69 passed; 0 failed; 0 ignored/.test(gate.stdout), 'all loading tests GREEN');
  if (name === 'workspace-tests-r2') {
    const totals = [...gate.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)]
      .reduce((a, m) => a.map((v, i) => v + Number(m[i + 1])), [0, 0, 0]);
    check(totals[0] > 0 && totals[1] === 0, 'workspace test summaries');
    report.workspace_totals = { passed: totals[0], failed: totals[1], ignored: totals[2] };
  }
}

// Candidate A/B paths may be passed explicitly after the test authority publishes them.
const [runsPath, comparisonPath] = process.argv.slice(2);
if (!runsPath || !comparisonPath) report.pending.push('final A/B runs and comparison paths');
else {
  const runs = JSON.parse(read(runsPath)), comparison = JSON.parse(read(comparisonPath));
  report.receipts.ab_runs = sha(read(runsPath)); report.receipts.ab_comparison = sha(read(comparisonPath));
  const expected = new Map(frozen.expected.map(e => [`${e.id}|${e.profile}`, e]));
  const keys = new Set();
  check(runs.l0_normative_sha256 === frozen.l0.normative_sha256, 'candidate A/B normative hash');
  check(runs.binaries.candidate.sha256 === '37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd', 'released candidate binary identity');
  check(sha(read(runs.binaries.candidate.path)) === runs.binaries.candidate.sha256, 'candidate binary bytes unchanged');
  const observe = r => JSON.stringify([r.exit, r.stdout, r.stderr]);
  for (const row of runs.rows) {
    const key = `${row.id}|${row.profile}|${row.order}`, e = expected.get(`${row.id}|${row.profile}`);
    assert(!keys.has(key) && e, 'unknown/duplicate candidate key'); keys.add(key);
    assert(['normal', 'repeat', 'reverse'].includes(row.order), 'unknown execution order');
    assert(row.product === 'candidate' && row.argv[2] === e.expr && row.cwd === e.cwd, 'candidate expression/cwd identity');
    assert(observe(row) === observe(e.expected), `candidate mismatch ${key}`);
  }
  check(keys.size === frozen.expected.length * 3, 'all A/B orders complete and exact');
  check(comparison.status === 'PASS' && comparison.unknown === 0 && comparison.failures.length === 0, 'test authority comparison PASS');
  check(comparison.freeze_sha256 === sha(read(`${directory}/p1319-ab-freeze.json`)) &&
    comparison.measurement_sha256 === sha(read(runsPath)), 'comparison binds exact freeze and runs');
  report.ab_comparisons = keys.size;
}
report.status = report.pending.length ? 'PENDING' : 'PASS_WITH_DOCUMENTED_LIMITATIONS';
console.log(JSON.stringify(report, null, 2));
process.exitCode = report.pending.length ? 2 : 0;
