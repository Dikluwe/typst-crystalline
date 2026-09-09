// Read-only reviewer audit. Run from repository root with node.
const fs = require('fs');
const crypto = require('crypto');
const d = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p, 'utf8');
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const baseline = JSON.parse(read(d + 'p1333-baseline.json'));
const integration = JSON.parse(read(d + 'p1333-test-integration.json'));
const freeze = JSON.parse(read(d + 'p1333-ab-freeze.json'));
const owner = '01_core/src/compiler/stdlib/calc.rs';
function strip(text, paths) {
  for (const path of paths) {
    const snippet = read(path);
    if (text.split(snippet).length !== 2) throw Error('Snippet not unique: ' + path);
    text = text.replace(snippet, '');
  }
  return text.replace(/^\/\/! @prompt-hash .*$/m, '//! @prompt-hash <lineage>').trimEnd();
}
function outsideAbs(text) {
  const start = text.indexOf('pub(crate) fn calc_abs(');
  const end = text.indexOf('/// P817-C/D', start);
  if (start < 0 || end < start) throw Error('Missing owner boundaries');
  return text.slice(0, start) + '<calc_abs>\n' + text.slice(end);
}
const old = strip(baseline.original_owner, Object.keys(integration.predecessors));
const current = strip(read(owner), integration.integrated_snippets.map(p => d + p));
const discrepancies = Object.entries(freeze.artifacts).filter(([p, expected]) => hash(d + p) !== expected);
const same = outsideAbs(old) === outsideAbs(current);
console.log(JSON.stringify({
  utc: new Date().toISOString(),
  baseline_sha256: hash(d + 'p1333-baseline.json'),
  manifest_sha256: hash(d + 'p1333-manifest.json'),
  full_freeze_sha256: hash(d + 'p1333-ab-freeze.json'),
  candidate_sha256: hash(owner),
  frozen_artifact_discrepancies: discrepancies,
  outside_calc_abs_equal_after_exact_snippet_removal_and_lineage_normalization: same,
}, null, 2));
if (!same || discrepancies.length) process.exitCode = 1;
