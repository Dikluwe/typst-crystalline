const fs = require('fs');
const crypto = require('crypto');
const assert = require('assert');
const d = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p);
const sha = s => crypto.createHash('sha256').update(s).digest('hex');
const m = JSON.parse(read(d + 'p1332-manifest.json'));
const b = JSON.parse(read(d + 'p1332-baseline.json'));
assert.equal(sha(read(d + 'p1332-manifest.json')), 'b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084');
for (const [file, key] of [
  ['p1332-baseline.json', 'baseline_sha256'],
  ['p1332-baseline-public.json', 'public_baseline_sha256'],
  ['p1332-name-consumers-public.json', 'name_consumers_sha256'],
  ['p1331-closure.json', 'previous_closure_sha256'],
]) assert.equal(sha(read(d + file)), m[key]);
const p = read(m.prompt).toString();
const s = read('01_core/src/compiler/stdlib/calc.rs').toString();
const norm = p.replace(/^Hash do Código: [0-9a-f]{8}\n/m, '');
assert.equal(sha(norm), m.prompt_norm_sha256);
const canonicalSource = text => text.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m, '');
assert.equal(canonicalSource(s), canonicalSource(b.original_owner));
const old = p.replace(/## P1332 — nome intrínseco da função abs\n[\s\S]*?(?=## Subset)/, '');
assert.equal(old, b.original_prompt);
const changed = Object.entries(b.product_inventory)
  .filter(([file, hash]) => sha(read(file)) !== hash)
  .map(([file]) => file);
assert.deepEqual(changed.sort(), [m.prompt, '01_core/src/compiler/stdlib/calc.rs'].sort());
console.log(JSON.stringify({
  at: new Date().toISOString(),
  manifest_sha256: sha(read(d + 'p1332-manifest.json')),
  prompt_norm_sha256: sha(norm),
  all_bindings: true,
  owner_only_header: true,
  l0_only_addition: true,
  changed,
}, null, 2));
