// Engineering evidence only: invokes unchanged independent owner/style tests.
const fs = require('fs'), cp = require('child_process'), crypto = require('crypto');
const root = '/repos/Antigravity/typst-crystalline';
process.chdir(root);
const [label, sealPath] = process.argv.slice(2);
if (!/^p1339-[a-z0-9-]+\.json$/.test(label || '') ||
    !/^00_nucleo\/diagnosticos\/p1339-[a-z0-9-]+\.json$/.test(sealPath || '')) {
  throw Error('new receipt filename and existing independent Array seal path required');
}
const output = `00_nucleo/diagnosticos/${label}`;
if (fs.existsSync(output)) throw Error('Never overwrite evidence');
const sha = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const pin = path => ({path, sha256: sha(fs.readFileSync(path))});
const git = args => cp.execFileSync('git', args, {encoding: 'utf8'});
function snapshot() {
  const paths = git(['ls-files', '-z', '--cached', '--others', '--exclude-standard', '--',
    '01_core', '02_shell', '03_infra', '04_wiring', 'Cargo.toml', 'Cargo.lock', '.cargo', 'crystalline.toml'])
    .split('\0').filter(p => p && fs.existsSync(p) && fs.statSync(p).isFile());
  const seen = new Set();
  function include(path) {
    if (seen.has(path)) return;
    seen.add(path);
    if (!path.endsWith('.rs')) return;
    const text = fs.readFileSync(path, 'utf8');
    for (const m of text.matchAll(/"\/\.\.\/(00_nucleo\/diagnosticos\/[^"\n]+)"/g)) include(m[1]);
    for (const m of text.matchAll(/"(\/src\/[^"\n]+)"/g)) include('01_core' + m[1]);
  }
  include('01_core/src/compiler/eval/mod.rs');
  include('01_core/src/compiler/stdlib/primitives_constructors/array.rs');
  for (const path of [sealPath,
    '00_nucleo/diagnosticos/p1339-contract-array-owner-freeze-r1.json',
    '00_nucleo/diagnosticos/p1339-verifier-style-actual-chain-review-r2.json',
    '00_nucleo/diagnosticos/p1339-seal.json']) include(path);
  return {utc: new Date().toISOString(), head: git(['rev-parse', 'HEAD']).trim(),
    diff_stat: git(['diff', 'HEAD', '--stat']),
    build_inputs: [...new Set(paths)].sort().map(pin), includes: [...seen].sort().map(pin)};
}
const same = (a, b) => a.head === b.head &&
  JSON.stringify(a.build_inputs) === JSON.stringify(b.build_inputs) &&
  JSON.stringify(a.includes) === JSON.stringify(b.includes);
const stream = data => ({base64: (data || Buffer.alloc(0)).toString('base64'),
  sha256: sha(data || Buffer.alloc(0))});
const buildDelta = {CARGO_TARGET_DIR: '/tmp/p1339-implementation-observer-instrumented.Y0Zc3f',
  RUSTFLAGS: '--cfg p1339_observation --check-cfg=cfg(p1339_observation)'};
function run(argv, delta) {
  const env = {...process.env, ...delta};
  const keys = ['CARGO_TARGET_DIR', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTDOCFLAGS',
    'RUSTC', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'CARGO_BUILD_TARGET',
    'CARGO_BUILD_RUSTFLAGS', 'CARGO_INCREMENTAL', 'TYPST_COMMIT_SHA', 'RUST_MIN_STACK'];
  const start = new Date().toISOString();
  const result = cp.spawnSync(argv[0], argv.slice(1), {cwd: root, env,
    timeout: 1800000, maxBuffer: 128 * 1024 * 1024});
  return {argv, cwd: root, start, end: new Date().toISOString(), environment_delta: delta,
    relevant_environment: Object.fromEntries(keys.filter(k => k in env).map(k => [k, env[k]])),
    timeout_ms: 1800000, status: result.status, signal: result.signal,
    error: result.error?.message || null, stdout: stream(result.stdout), stderr: stream(result.stderr)};
}
const before = snapshot();
const compile = run(['cargo', 'test', '-p', 'typst-core', '--lib', '--release', '--locked',
  '--offline', '-j', '3', '--no-run', '--message-format', 'json-render-diagnostics'], buildDelta);
const afterCompile = snapshot();
const runtime = [];
if (compile.status === 0 && same(before, afterCompile)) {
  const artifacts = Buffer.from(compile.stdout.base64, 'base64').toString('utf8').split('\n')
    .flatMap(line => {try {return [JSON.parse(line)];} catch {return [];}})
    .filter(x => x.reason === 'compiler-artifact' && x.executable && x.profile.test && x.target.name === 'typst_core');
  const binaries = [...new Set(artifacts.map(x => x.executable))];
  if (binaries.length !== 1) throw Error('Expected exactly one actual core test executable');
  const binary = binaries[0];
  for (const test of [
    'compiler::stdlib::primitives_constructors::array::tests::p1339_array_frozen_owner_oracle',
    'compiler::eval::p1339_frozen_observation_binding::p1339_frozen_style_bound_matrix',
  ]) {
    const binaryBefore = pin(binary);
    const result = run([binary, test, '--exact', '--nocapture', '--test-threads=1'], {RUST_MIN_STACK: '33554432'});
    const transcript = Buffer.from(result.stdout.base64, 'base64').toString('utf8');
    const oneTestExecuted = /running 1 test\b/.test(transcript) && /test result: ok\. 1 passed; 0 failed; 0 ignored;/.test(transcript);
    runtime.push({...result, one_test_executed_and_passed: oneTestExecuted,
      binary_before: binaryBefore, binary_after: pin(binary)});
    if (result.status !== 0 || !oneTestExecuted) break;
  }
}
const after = snapshot();
const receipt = {schema: 'p1339-array-style-candidate-engineering-v1',
  role: 'implementer execution only; independent verdict required', runner: pin(__filename),
  seal: pin(sealPath), before, compile, after_compile: afterCompile, runtime, after,
  compile_inputs_unchanged: same(before, afterCompile), all_inputs_unchanged: same(before, after)};
const text = JSON.stringify(receipt, null, 2) + '\n';
cp.execFileSync('apply_patch', {input: `*** Begin Patch\n*** Add File: ${output}\n${text.trimEnd().split('\n').map(x => '+' + x).join('\n')}\n*** End Patch\n`});
console.log(JSON.stringify({receipt: output, sha256: sha(fs.readFileSync(output)),
  compile: compile.status, runtime: runtime.map(x => x.status), unchanged: receipt.all_inputs_unchanged}));
