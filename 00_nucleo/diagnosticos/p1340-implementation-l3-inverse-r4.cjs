// Mechanical inverse of R4 terminal-test hooks; not an independent verdict.
const fs = require('fs');
const crypto = require('crypto');
const sha = s => crypto.createHash('sha256').update(s).digest('hex');
const baseline = JSON.parse(fs.readFileSync('00_nucleo/diagnosticos/p1340-r4-pre-instrumentation-state.json'));
function withoutTerminalHooks(source) {
  const lines = source.split('\n');
  const output = [];
  let removed = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() !== '#[cfg(all(test, p1339_observation))]') {
      output.push(lines[i]);
      continue;
    }
    const module = lines[i + 1].includes('mod observation');
    let depth = 0;
    let complete = false;
    while (++i < lines.length) {
      // These fixed hooks contain no raw strings or comments with delimiters.
      const code = lines[i].replace(/"(?:\\.|[^"\\])*"/g, '""');
      for (const ch of code) {
        if ('({['.includes(ch)) depth++;
        else if (')}]'.includes(ch)) depth--;
      }
      if (depth === 0 && /[;} ]\s*$/.test(code) && /[;}]/.test(code)) {
        complete = true;
        break;
      }
    }
    if (!complete || depth !== 0) throw Error('Unrecognized cfg hook boundary');
    if (module && lines[i + 1] === '') i++;
    removed++;
  }
  return {source: output.join('\n'), removed};
}
const owners = ['03_infra/src/pipeline.rs', '03_infra/src/pipeline/context_stabilization.rs'].map(path => {
  const current = fs.readFileSync(path, 'utf8');
  const inverse = withoutTerminalHooks(current);
  const expected = baseline.before.modified[path] || baseline.before.new_sources[path];
  return {path, current_sha256: sha(current), inverse_sha256: sha(inverse.source),
    expected_sha256: expected, removed_cfg_units: inverse.removed,
    matches: sha(inverse.source) === expected};
});
console.log(JSON.stringify({baseline_receipt_sha256: sha(fs.readFileSync('00_nucleo/diagnosticos/p1340-r4-pre-instrumentation-state.json')),
  meaning: 'Exact source restriction witness for cfg-only terminal hooks; not F or equivalence', owners}, null, 2));
if (!owners.every(o => o.matches)) process.exitCode = 1;
