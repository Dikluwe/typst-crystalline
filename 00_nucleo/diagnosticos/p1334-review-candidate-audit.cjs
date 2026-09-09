const fs = require('fs');
const crypto = require('crypto');
const D = __dirname + '/';
const read = p => fs.readFileSync(p, 'utf8');
const sha = x => crypto.createHash('sha256').update(x).digest('hex');
const baseline = JSON.parse(read(D + 'p1334-baseline.json'));
const manifest = JSON.parse(read(D + 'p1334-manifest.json'));
const freeze = JSON.parse(read(D + 'p1334-ab-freeze-r1.json'));
const integration = JSON.parse(read(D + 'p1334-test-integration-r1.json'));
const findings = [];
const normalize = s => s.replace(/^\/\/! @prompt-hash .*\n/m, '').replace(/\n{3,}/g, '\n\n').trim();
function region(s, start, end) {
  const a = s.indexOf(start), b = s.indexOf(end, a);
  if (a < 0 || b < 0) throw new Error('Missing function boundary');
  return s.slice(a, b);
}
for (const kind of ['inputs', 'artifacts']) {
  for (const [path, expected] of Object.entries(freeze[kind])) {
    if (sha(fs.readFileSync(D + path)) !== expected) findings.push('changed frozen ' + path);
  }
}
const records = [];
for (const owner of manifest.owners) {
  const old = baseline.original_files[owner.source];
  const current = read(owner.source);
  let restored = current;
  for (const name of integration.owners[owner.source] || []) {
    const snippet = read(D + name).trimEnd();
    if (!restored.includes(snippet)) findings.push('frozen snippet absent ' + name);
    const matched = name.match(/p1334-ab-p(\d+)-successor\.rs/);
    if (matched) {
      const previous = matched[1] === '1333' ? 'p1333-ab-tests.rs' : `p1333-ab-p${matched[1]}-successor.rs`;
      restored = restored.replace(snippet, read(D + previous).trimEnd());
    } else restored = restored.replace(snippet, '');
  }
  if (owner.source.endsWith('/calc.rs')) {
    const start = 'pub(crate) fn calc_abs(', end = '/// P817-C/D — paridade vanilla';
    const first = region(current, start, end), prior = region(old, start, end);
    const unchangedPrefix = first.slice(0, first.indexOf('        [] => {')) === prior.slice(0, prior.indexOf('        [] => {'));
    if (!unchangedPrefix) findings.push('first-value branches changed');
    restored = restored.replace(region(restored, start, end), prior);
  } else if (owner.source.endsWith('/call_dispatch.rs')) {
    const start = 'pub(super) fn transport_native_call_span(', end = '/// Aplica uma função';
    restored = restored.replace(region(restored, start, end), region(old, start, end));
  } else restored = restored.replace('pub(crate) use crate::compiler::stdlib::calc::calc_abs;\n', '');
  const preserved = normalize(restored) === normalize(old);
  if (!preserved) findings.push('outside intended delta ' + owner.source);
  const prompt = read(owner.prompt), norm = prompt.replace(/^Hash do Código: [a-f0-9]{8}\n/m, '');
  if (sha(norm) !== owner.norm_sha256) findings.push('norm drift ' + owner.prompt);
  records.push({source: owner.source, sha256: sha(current), outside_intended_delta_preserved: preserved, prompt_norm_preserved: sha(norm) === owner.norm_sha256});
}
const allowed = new Set(manifest.owners.flatMap(o => [o.source, o.prompt]));
for (const [path, expected] of Object.entries(baseline.product_inventory)) {
  if (!allowed.has(path) && sha(fs.readFileSync(path)) !== expected) findings.push('out-of-scope file changed ' + path);
}
console.log(JSON.stringify({utc: new Date().toISOString(), head: baseline.head, working_tree: baseline.working_tree, baseline_sha256: sha(read(D + 'p1334-baseline.json')), manifest_sha256: sha(read(D + 'p1334-manifest.json')), freeze_sha256: sha(read(D + 'p1334-ab-freeze-r1.json')), records, findings}, null, 2));
if (findings.length) process.exitCode = 1;
