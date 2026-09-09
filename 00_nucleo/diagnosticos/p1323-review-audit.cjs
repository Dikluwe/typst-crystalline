const fs = require('fs');
const crypto = require('crypto');
const cp = require('child_process');
const D = '00_nucleo/diagnosticos/';
const sha = value => crypto.createHash('sha256').update(value).digest('hex');
const read = path => fs.readFileSync(path, 'utf8');
const json = name => JSON.parse(read(D + name));
const baseline = json('p1323-baseline.json');
const frozen = json('p1323-ab-frozen-suite.json');
const allowed = ['00_nucleo/prompts/wiring.md', '04_wiring/src/main.rs'];
const changed = Object.entries(baseline.state.product_inventory).flatMap(([path, hash]) => {
  const actual = fs.existsSync(path) ? sha(fs.readFileSync(path)) : null;
  return actual === hash ? [] : [{path, baseline: hash, current: actual, allowed: allowed.includes(path)}];
});
const snippet = read(D + 'p1323-ab-unit.rs');
const owner = read('04_wiring/src/main.rs');
const integrated = '#[cfg(test)]\nmod tests {\n' + snippet.split('\n').filter((line, i, all) => i < all.length - 1 || line).map(line => line ? '    ' + line : '').join('\n') + '\n}\n\n';
const hasExactTest = owner.includes(integrated);
let restored = owner.replace(integrated, '');
restored = restored.replace(/const HTML_EXPERIMENTAL_WARNING: &str = concat!\([\s\S]*?\);\n\n/, '');
restored = restored.replace('eprint!("{HTML_EXPERIMENTAL_WARNING}");', 'eprintln!(\n                    "warning: html export is under active development and incomplete"\n                );');
restored = restored.replace(/^\/\/! @prompt-hash .*$/m, baseline.original_owner.match(/^\/\/! @prompt-hash .*$/m)[0]);
const prompt = read('00_nucleo/prompts/wiring.md');
const protectedRows = [...frozen.protected, ...frozen.obligation.nuclei].map(item => ({...item, intact: sha(fs.readFileSync(item.path)) === item.sha256}));
const tracked = cp.execFileSync('git', ['diff', 'HEAD', '--stat'], {encoding:'utf8'});
console.log(JSON.stringify({at:new Date().toISOString(),head:cp.execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),working_tree_diff_stat:tracked,
  baseline_sha256:sha(fs.readFileSync(D+'p1323-baseline.json')), frozen_suite_sha256:sha(fs.readFileSync(D+'p1323-ab-frozen-suite.json')),
  product_inventory_changed:changed, unauthorized_changes:changed.filter(row=>!row.allowed),
  exact_independent_test_integrated:hasExactTest, owner_restored_exact_baseline:restored===baseline.original_owner,
  owner_sha256:sha(owner),prompt_sha256:sha(prompt), normative_sha256:sha(prompt.replace(/^Hash do Código:.*\n/m,'')),
  normative_intact:sha(prompt.replace(/^Hash do Código:.*\n/m,''))===frozen.obligation.normative_sha256, protected_inputs:protectedRows},null,2));
