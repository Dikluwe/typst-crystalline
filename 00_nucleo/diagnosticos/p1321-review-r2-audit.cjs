// Independent read-only audit of R2; R1 artifacts are never rewritten.
// Run from repo root: node 00_nucleo/diagnosticos/p1321-review-r2-audit.cjs
const fs = require('node:fs'), crypto = require('node:crypto');
const root = '/repos/Antigravity/typst-crystalline', dir = `${root}/00_nucleo/diagnosticos`;
const read = p => fs.readFileSync(p.startsWith('/') ? p : `${root}/${p}`);
const sha = value => crypto.createHash('sha256').update(value).digest('hex');
const file = name => `${dir}/p1321-${name}.json`;
const json = name => JSON.parse(read(file(name)));
const report = { utc: new Date().toISOString(), regime: 'A/B without technical isolation attestation',
  checks: [], pending: [], failures: [], hashes: {}, receipts: {} };
const check = (ok, name) => { if (!ok) throw Error(name); report.checks.push(name); };
const pins = {
  'r2-baseline': '8ca670bb4ab656a29667abd67fc06bc4d227a3dcc0afa9330a5e9ad49537f065',
  'r2-unit-red': '13b0dc933bc5778fdd9c4d93c7eb8327296549b3b0169fec1483ed63e43723d5',
  'ab2-r2-freeze': '67c6b92d62018b1cc8673fe28e75d8c55fd28f1c4e15d15ab6a4bb535ecfc8dd',
  'r2-reseal': 'd2e3e398af4f29ce7519ffe555466633a1a9f1c7ed711ef972226dc0fb7d3db4',
  'ab2-candidate': 'd1699b5ae42327c7ba7254f5f1f2830453d0cd225fce8f79807cf329aa2a1a25',
};
for (const [name, hash] of Object.entries(pins)) check(sha(read(file(name))) === hash, `receipt pin ${name}`);
const freeze = json('ab2-r2-freeze'), red = json('r2-unit-red'), baseline = json('r2-baseline');
for (const [path, hash] of Object.entries({...freeze.inputs, ...freeze.binaries})) check(sha(read(path)) === hash, `frozen input ${path}`);
const pairs = [
  ['01_core/src/compiler/stdlib/loading.rs', '00_nucleo/prompts/compiler/stdlib/loading.md', 'f4ebb73fac0d64d412adbda475eaef65fb45e01ac2cf44a13c5b9d9aa408bf8f', 'bf6e830de31aab2b0cd113872e258949e46e6d54e3f041bcc21ed62cc43f57d4'],
  ['01_core/src/compiler/eval/call_dispatch.rs', '00_nucleo/prompts/compiler/eval/call_dispatch.md', '9b11c388cf50666beecad9a6c92fbebfb323cdf5edbb550f4be04a96e0c3e9d4', 'd11ca271808ccdda640fea58a3bfa0354816299a42c3736c72373c76859f5ebf'],
];
const sourceMarker = /^\/\/! @prompt-hash ([0-9a-f]{8})\n/gm;
const promptMarker = /^Hash do Código: ([0-9a-f]{8})\n/gm;
const tests = text => text.slice(text.indexOf('#[cfg(test)]'));
for (const [owner, l0, sourceHash, promptHash] of pairs) {
  const s = read(owner).toString(), p = read(l0).toString();
  check(sha(s) === sourceHash && sha(p) === promptHash, `reviewed pair ${owner}`);
  check([...s.matchAll(sourceMarker)].length === 1 && [...p.matchAll(promptMarker)].length === 1, `canonical metadata ${owner}`);
  const cleanS = s.replace(sourceMarker, ''), cleanP = p.replace(promptMarker, '');
  const B = sha(cleanS);
  check(B.slice(0, 8) === [...p.matchAll(promptMarker)][0][1], `independent B ${owner}`);
  const nuclei = [...p.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/\S+) sha256:([0-9a-f]{64})$/gm)];
  check(nuclei.length === 1, `single shared nucleus ${owner}`);
  const n = read(nuclei[0][1]);
  check(!/^\[\[depends\]\]/m.test(n.toString()), `dependency-free nucleus ${owner}`);
  const digest = crypto.createHash('sha256').update(n).update(Buffer.from([0])).update('TEKT-NUCLEUS-DEPS-V1').update(Buffer.from([0])).digest();
  check(digest.toString('hex') === nuclei[0][2], `effective nucleus pin ${owner}`);
  const path = Buffer.from(nuclei[0][1]), length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(path.length));
  const A = crypto.createHash('sha256').update(cleanP).update(Buffer.from([0])).update('TEKT-PROMPT-NUCLEI-V1').update(Buffer.from([0]))
    .update(length).update(path).update(Buffer.from([32])).update(digest).digest('hex');
  check(A.slice(0, 8) === [...s.matchAll(sourceMarker)][0][1], `independent effective A ${owner}`);
  check(sha(cleanP) === freeze.norms[`${root}/${l0}`].without_code_hash, `frozen norm ${owner}`);
  check(tests(s) === tests(red.before.files[owner].text), `all tests unchanged since RED R2 ${owner}`);
  check(red.before.files[owner].text.split('#[cfg(test)]')[0] === baseline.before.files[owner].text.split('#[cfg(test)]')[0], `RED productive prefix predates R2 ${owner}`);
  report.hashes[owner] = { source: sourceHash, l0: promptHash, normative: sha(cleanP), tests: sha(tests(s)), A, B, nucleus: digest.toString('hex') };
}
check(red.exit === 101 && /1 passed; 1 failed; 0 ignored/.test(red.stdout), 'genuine R2 RED');
check(sha(tests(read(pairs[0][0]).toString())) === '42219b7843fbcfb14b5241cd0d1b7b88be050d42a7b084c5820ed49cf59542fd', 'loading tests unchanged since original RED');
const cases = [...json('ab2-cases'), ...json('ab2-r2-cases').filter(x => !Object.hasOwn(freeze.exploratory_exclusions, x.id))];
const expected = [...json('ab2-expectations'), ...json('ab2-r2-expectations')];
check(cases.length === 129 && expected.length === 516, '129 cases in four profiles');
check(expected.every(x => !x.expected.unknown && Number.isInteger(x.expected.exit)), 'no accepted Unknown');
const expectationMap = new Map(expected.map(x => [`${x.id}|${x.profile}`, x.expected]));
const casesMap = new Map(cases.map(x => [x.id, x]));
check(expectationMap.size === expected.length && casesMap.size === cases.length, 'unique keys');
const profiles = {default: [], html: ['--features', 'html'], a11y: ['--features', 'a11y-extras'], 'html-a11y': ['--features', 'html,a11y-extras']};
const obs = r => JSON.stringify([r.exit, r.stdout, r.stderr, r.unknown || null]);
let focal;
for (const name of ['focal', 'full']) {
  if (!fs.existsSync(file(`ab2-r2-${name}`))) { report.pending.push(`A/B ${name}`); continue; }
  const run = json(`ab2-r2-${name}`);
  check(run.freeze === pins['ab2-r2-freeze'], `A/B ${name} freeze identity`);
  check(sha(read(run.binary)) === run.sha256, `A/B ${name} binary bytes`);
  if (name === 'focal') focal = run;
  else check(focal && focal.sha256 === run.sha256 && focal.state.utc <= run.state.utc && Object.values(focal.failures).every(x => x.length === 0), 'focal passed before full on same binary');
  report.binary = { path: run.binary, sha256: run.sha256 };
  for (const [order, rows] of Object.entries(run.runs)) {
    check(rows.length === (name === 'focal' ? 140 : 516), `${name}/${order} complete count`);
    const seen = new Set();
    for (const row of rows) {
      const key = `${row.id}|${row.profile}`, e = expectationMap.get(key);
      if (!e || seen.has(key)) throw Error(`unexpected duplicate ${key}`);
      seen.add(key);
      const argv = [run.binary, '--color=never', 'eval', ...profiles[row.profile], casesMap.get(row.id).expr];
      if (JSON.stringify(argv) !== JSON.stringify(row.argv)) throw Error(`changed argv ${key}`);
      if (obs(row) !== obs(e)) report.failures.push({name, order, key, actual: obs(row), expected: obs(e)});
    }
    check(run.failures[order].length === report.failures.filter(x => x.name === name && x.order === order).length, `${name}/${order} independent result agrees`);
  }
  check(JSON.stringify(Object.keys(run.runs).sort()) === JSON.stringify((name === 'focal' ? ['focal'] : ['normal','repeat','reverse']).sort()), `${name} required execution orders`);
  report.receipts[`ab2-r2-${name}`] = sha(read(file(`ab2-r2-${name}`)));
}
for (const name of ['build', 'unit-green', 'workspace-tests', 'fmt', 'diff-check', 'lint', 'lineage-final']) {
  const path = file(`r2-${name}`);
  if (!fs.existsSync(path)) { report.pending.push(name); continue; }
  const gate = JSON.parse(read(path));
  check(gate.exit === 0, `${name} exit zero`);
  for (const state of [gate.before, gate.after]) {
    check(state.head === 'd31047d7b8af7837c84adae4ded3d2ff50c62093', `${name} HEAD`);
    for (const [s,p,hs,hp] of pairs) check(state.files[s].sha256 === hs && state.files[p].sha256 === hp, `${name} exact pair ${s}`);
  }
  if (/tests|green/.test(name)) {
    const totals = [...gate.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored/g)]
      .reduce((a,m) => a.map((v,i)=>v+Number(m[i+1])), [0,0,0]);
    check(totals[0] > 0 && totals[1] === 0, `${name} successful test summary`);
    report[`${name}_totals`] = totals;
  }
  report.receipts[name] = { sha256: sha(read(path)), argv: gate.argv, utc: [gate.before.utc, gate.after.utc] };
}
report.status = report.failures.length ? 'FAIL_OBSERVABLE_CONTRACT' : report.pending.length ? 'PENDING' : 'PASS_REVIEWED_FRAGMENT_WITH_LIMITATIONS';
console.log(JSON.stringify(report, null, 2));
process.exitCode = report.failures.length ? 1 : report.pending.length ? 2 : 0;
