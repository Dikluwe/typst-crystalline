// Read-only independent checker. Run: node 00_nucleo/diagnosticos/p1321-review-audit.cjs
// Optional arguments are final gate receipt paths. JSON is emitted to stdout.
const fs = require('node:fs');
const crypto = require('node:crypto');
const root = '/repos/Antigravity/typst-crystalline';
const dir = `${root}/00_nucleo/diagnosticos`;
const owner = '01_core/src/compiler/stdlib/loading.rs';
const l0 = '00_nucleo/prompts/compiler/stdlib/loading.md';
const read = p => fs.readFileSync(p.startsWith('/') ? p : `${root}/${p}`);
const sha = data => crypto.createHash('sha256').update(data).digest('hex');
const json = name => JSON.parse(read(`${dir}/p1321-${name}.json`));
const report = { utc: new Date().toISOString(), checks: [], receipts: {}, pending: [], failures: [],
  regime: 'A/B executed without technical isolation attestation' };
const check = (condition, name) => { if (!condition) throw Error(name); report.checks.push(name); };
const pins = {
  baseline: '4b4de0feeb4c1df1b9007de81f11f6a4d1ae90b25cdc1c9ca1eadaa211e58a96',
  measurement: 'f13c4ea33fe8d8f91a5ef443d3183d1cbb3f77d9ac6726c0e4c5e23c54fbef05',
  'unit-red': 'd750796583662024a5aac54f07bf601c87c9ea50dec145ae0b0282d7c758e8d8',
  'ab2-freeze': 'b90c53cc2d982160dce2faca2f4afb6910d07b06e321b8bf8090bc51ead0d6ca',
  reseal: '4f6a14bdd0a185e18dc9d08f9b49a5441bea964e1d368fbc577713b4125a0e7a',
};
for (const [name, digest] of Object.entries(pins)) {
  check(sha(read(`${dir}/p1321-${name}.json`)) === digest, `frozen receipt ${name}`);
}
const source = read(owner).toString(), prompt = read(l0).toString();
const sourceMarker = /^\/\/! @prompt-hash ([0-9a-f]{8})\n/gm;
const promptMarker = /^Hash do Código: ([0-9a-f]{8})\n/gm;
check([...source.matchAll(sourceMarker)].length === 1, 'one canonical A header');
check([...prompt.matchAll(promptMarker)].length === 1, 'one canonical B header');
const cleanSource = source.replace(sourceMarker, ''), cleanPrompt = prompt.replace(promptMarker, '');
check(sha(source) === '8024109818bfa83a8cf62f01dd1962b1e3dc01100e22d09eb6333b623292faa6', 'reviewed candidate source');
check(sha(prompt) === '878571e48bb3dd48eb573e3e0af7eba35ce83bde3f827a8338cfbd0d1a33bf1a', 'reviewed candidate L0');
check([...prompt.matchAll(promptMarker)][0][1] === sha(cleanSource).slice(0, 8), 'reverse B independently recomputed');
const nuclei = [...prompt.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/\S+) sha256:([0-9a-f]{64})$/gm)];
check(nuclei.length === 1, 'one expected nucleus');
const nucleusBytes = read(nuclei[0][1]);
check(!/^\[\[depends\]\]/m.test(nucleusBytes.toString()), 'nucleus dependency-free');
const digest = crypto.createHash('sha256').update(nucleusBytes).update(Buffer.from([0]))
  .update('TEKT-NUCLEUS-DEPS-V1').update(Buffer.from([0])).digest();
check(digest.toString('hex') === nuclei[0][2], 'effective nucleus pin independently recomputed');
const pathBytes = Buffer.from(nuclei[0][1]), size = Buffer.alloc(8);
size.writeBigUInt64BE(BigInt(pathBytes.length));
const a = crypto.createHash('sha256').update(cleanPrompt).update(Buffer.from([0]))
  .update('TEKT-PROMPT-NUCLEI-V1').update(Buffer.from([0])).update(size)
  .update(pathBytes).update(Buffer.from([32])).update(digest).digest('hex');
check(a.slice(0, 8) === [...source.matchAll(sourceMarker)][0][1], 'forward effective A independently recomputed');
const freeze = json('ab2-freeze'), red = json('unit-red'), baseline = json('baseline');
check(sha(cleanPrompt) === freeze.norm.without_code_hash, 'normative freeze unchanged');
const tests = text => text.slice(text.indexOf('#[cfg(test)]'));
check(tests(source) === tests(red.before.files[owner].text), 'complete tests unchanged since RED');
check(sha(tests(source)) === '42219b7843fbcfb14b5241cd0d1b7b88be050d42a7b084c5820ed49cf59542fd', 'RED test module identity');
check(red.before.files[owner].text.split('#[cfg(test)]')[0] === baseline.before.files[owner].text.split('#[cfg(test)]')[0], 'RED productive prefix equals P1319');
check(red.exit === 101 && /66 passed; 6 failed; 0 ignored/.test(red.stdout), 'genuine compiled RED');
for (const [path, digest] of Object.entries({...freeze.inputs, ...freeze.binaries})) {
  check(sha(read(path)) === digest, `protected input ${path}`);
}
check(sha(read(`${dir}/p1319-ab-freeze.json`)) === '61d7c5005e889ae0857230f27a4ca8db5ecc38734abfe8699418459c37a8908f', 'historical P1319 freeze retained');
const olderFreeze = JSON.parse(read(`${dir}/p1319-ab-freeze.json`));
for (const [path, digest] of Object.entries(olderFreeze.inputs)) check(sha(read(path)) === digest, `historical protected input ${path}`);
report.hashes = { source: sha(source), prompt: sha(prompt), normative: sha(cleanPrompt),
  tests: sha(tests(source)), B: sha(cleanSource), A: a, nucleus: digest.toString('hex') };
const cases = json('ab2-cases'), expected = json('ab2-expectations');
check(cases.length === 110 && expected.length === 440, 'frozen corpus cardinality');
check(expected.every(x => !x.expected.unknown && Number.isInteger(x.expected.exit)), 'no Unknown accepted');
check(new Set(expected.map(x => `${x.id}|${x.profile}`)).size === expected.length, 'unique expected keys');
const observe = r => JSON.stringify([r.exit, r.stdout, r.stderr, r.unknown || null]);
const candidatePath = `${dir}/p1321-ab2-candidate.json`;
if (!fs.existsSync(candidatePath)) report.pending.push('A/B candidate');
else {
  const candidate = JSON.parse(read(candidatePath));
  check(sha(read(candidate.candidate)) === candidate.sha256, 'tested binary unchanged');
  report.binary = { path: candidate.candidate, sha256: candidate.sha256 };
  report.receipts.ab = sha(read(candidatePath));
  const expectedMap = new Map(expected.map(x => [`${x.id}|${x.profile}`, x]));
  const casesMap = new Map(cases.map(x => [x.id, x]));
  for (const order of ['normal', 'repeat', 'reverse']) {
    const rows = candidate.runs[order], keys = new Set();
    check(rows.length === expected.length, `${order} complete cardinality`);
    for (const row of rows) {
      const key = `${row.id}|${row.profile}`, e = expectedMap.get(key);
      if (!e || keys.has(key)) throw Error(`unknown/duplicate candidate row ${key}`);
      keys.add(key);
      if (row.argv[0] !== candidate.candidate || row.argv.at(-1) !== casesMap.get(row.id).expr) throw Error(`changed input ${key}`);
      if (observe(row) !== observe(e.expected)) report.failures.push({order, key, actual: row.stderr, expected: e.expected.stderr});
    }
    check(candidate.failures[order].length === report.failures.filter(x => x.order === order).length, `${order} independently confirmed failure count`);
  }
  report.ab_comparisons = expected.length * 3;
}
for (const path of process.argv.slice(2)) {
  const gate = JSON.parse(read(path));
  check(gate.exit === 0, `gate exit zero ${path}`);
  for (const state of [gate.before, gate.after]) {
    check(state.head === 'd31047d7b8af7837c84adae4ded3d2ff50c62093', `gate HEAD ${path}`);
    check(state.files[owner].sha256 === sha(source) && state.files[l0].sha256 === sha(prompt), `gate candidate identity ${path}`);
  }
  report.receipts[path] = { sha256: sha(read(path)), argv: gate.argv,
    before_utc: gate.before.utc, after_utc: gate.after.utc };
  if (/unit-green/.test(path)) check(/72 passed; 0 failed; 0 ignored/.test(gate.stdout), 'all 72 loading tests GREEN');
  if (/workspace-tests/.test(path)) {
    const sums = [...gate.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)]
      .reduce((a, x) => a.map((v, i) => v + Number(x[i + 1])), [0, 0, 0]);
    check(sums[0] > 0 && sums[1] === 0, 'workspace successful summaries');
    report.workspace_totals = sums;
  }
}
report.status = report.failures.length ? 'FAIL_OBSERVABLE_CONTRACT' : report.pending.length ? 'PENDING_FINAL_GATES' : 'PASS_FOR_REVIEWED_INPUTS_WITH_LIMITATIONS';
console.log(JSON.stringify(report, null, 2));
process.exitCode = report.failures.length ? 1 : report.pending.length ? 2 : 0;
