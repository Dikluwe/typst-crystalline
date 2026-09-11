const fs = require('fs');
const crypto = require('crypto');
const cp = require('child_process');
const path = require('path');
const root = '/repos/Antigravity/typst-crystalline';
const [label, ...argv] = process.argv.slice(2);
if (!/^[a-z0-9-]+$/.test(label || '') || !argv.length) throw Error('label and command required');
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const git = (...a) => cp.execFileSync('git', a, {cwd:root, encoding:'utf8'});
const paths = [
  '01_core/src/compiler/stdlib/foundations/float.rs',
  '01_core/src/compiler/stdlib/primitives_constructors/version.rs',
  '01_core/src/compiler/stdlib/primitives_constructors.rs',
  '01_core/src/compiler/stdlib/mod.rs',
];
function state() {
  return {utc:new Date().toISOString(), head:git('rev-parse','HEAD').trim(),
    diff_stat:git('diff','HEAD','--stat'), status:git('status','--short'),
    source_sha256:Object.fromEntries(paths.map(p=>[p,sha(path.join(root,p))]))};
}
const before = state();
const start = Date.now();
const run = cp.spawnSync(argv[0], argv.slice(1), {cwd:root, encoding:'utf8', maxBuffer:128*1024*1024});
const after = state();
const receipt = {executor:'/root/p1339_remaining_l0', role:'implementation-local-check',
  regime:'executado sem atestacao de isolamento', label, argv,
  target_dir:process.env.CARGO_TARGET_DIR || null, before, after,
  elapsed_ms:Date.now()-start, exit:run.status, signal:run.signal,
  error:run.error?.message, stdout:run.stdout, stderr:run.stderr,
  sources_changed_during_run:JSON.stringify(before.source_sha256)!==JSON.stringify(after.source_sha256),
  authority_sha256:sha(path.join(root,'00_nucleo/diagnosticos/p1339-implementation-authorities.json')),
  seal_sha256:sha(path.join(root,'00_nucleo/diagnosticos/p1339-seal.json')),
  no_claims:['Not independent verification','No final P1339 verdict']};
const out = path.join(root,`00_nucleo/diagnosticos/p1339-implementation-numerics-${label}.json`);
fs.writeFileSync(out, JSON.stringify(receipt,null,2)+'\n', {flag:'wx'});
process.stdout.write((run.stdout || '')+(run.stderr || ''));
console.log(JSON.stringify({receipt:out,sha256:sha(out),exit:run.status,elapsed_ms:receipt.elapsed_ms}));
process.exitCode = run.status ?? 1;
