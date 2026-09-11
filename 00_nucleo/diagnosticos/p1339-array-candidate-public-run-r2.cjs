// Mechanical candidate execution against independently frozen literal cells.
const fs = require('fs'), cp = require('child_process'), crypto = require('crypto');
const root = '/repos/Antigravity/typst-crystalline';
process.chdir(root);
const [mode, binary, seal, go] = process.argv.slice(2);
if (!['focal', 'expanded'].includes(mode) || ![binary, seal, go].every(x => x && fs.existsSync(x))) {
  throw Error('mode, actual candidate binary, independent seal and GO paths required');
}
const dir = '00_nucleo/diagnosticos/';
const output = `${dir}p1339-array-candidate-public-${mode}-r1.json`;
if (fs.existsSync(output)) throw Error('Never overwrite evidence');
const sha = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const pin = path => ({path, sha256: sha(fs.readFileSync(path))});
const refsPath = dir + 'p1339-contract-array-expanded-raw-r1.json';
const oraclesPath = dir + 'p1339-contract-array-expanded-oracles-r1.json';
if (pin(refsPath).sha256 !== '53b9ab9f4b5c3ee0bcffa2ea248a101aa85dc9a03539217cbea9b15202fdee8e' ||
    pin(oraclesPath).sha256 !== '35c043a82bbfd56b1d9e334355a0acacaed57039bb0c47b02826bdd888b183c0') {
  throw Error('Frozen reference drift');
}
const refs = JSON.parse(fs.readFileSync(refsPath));
const oracles = JSON.parse(fs.readFileSync(oraclesPath));
const cells = refs.runs.filter(x => x.binary === 'vanilla')
  .filter(x => mode === 'expanded' || (x.profile === 'default' && x.order === 'normal'))
  .map(ref => {
    const cell = oracles.cases.find(x => x.id === ref.id && x.profile === ref.profile && x.order === ref.order);
    if (!cell) throw Error('Missing frozen oracle for actual reference execution order');
    return cell;
  });
if (new Set(cells.map(x => JSON.stringify([x.id, x.profile, x.order]))).size !== cells.length) {
  throw Error('Duplicate cell identity');
}
if (cells.length !== (mode === 'expanded' ? 336 : 28)) throw Error('Incorrect cell count');
const git = args => cp.execFileSync('git', args, {encoding: 'utf8'});
function state() {
  const paths = git(['ls-files', '-z', '--cached', '--others', '--exclude-standard', '--',
    '01_core', '02_shell', '03_infra', '04_wiring', 'Cargo.toml', 'Cargo.lock', '.cargo', 'crystalline.toml'])
    .split('\0').filter(x => x && fs.existsSync(x) && fs.statSync(x).isFile());
  return {utc: new Date().toISOString(), head: git(['rev-parse', 'HEAD']).trim(),
    diff_stat: git(['diff', 'HEAD', '--stat']), product: [...new Set(paths)].sort().map(pin),
    inputs: [binary, seal, go, __filename, refsPath, oraclesPath].map(pin)};
}
const before = state(), rows = [];
const env = {...process.env, NO_COLOR: '1', PYTHONDONTWRITEBYTECODE: '1'};
for (const cell of cells) {
  const ref = refs.runs.find(x => x.binary === 'vanilla' && x.id === cell.id && x.profile === cell.profile && x.order === cell.order);
  if (!ref || sha(Buffer.from(cell.source)) !== cell.source_sha256 || ref.source !== cell.source ||
      ref.source_sha256 !== cell.source_sha256 || ref.argv.at(-1) !== cell.source) throw Error('Reference cell/source mismatch');
  const argv = [binary, ...ref.argv.slice(1)];
  const start = new Date().toISOString();
  const actual = cp.spawnSync(binary, argv.slice(1), {cwd: root, env, input: Buffer.alloc(0),
    timeout: 30000, maxBuffer: 16 * 1024 * 1024});
  const stdout = actual.stdout || Buffer.alloc(0), stderr = actual.stderr || Buffer.alloc(0);
  const transportValid = !actual.error && !actual.signal && [0, 1].includes(actual.status);
  const expected = cell.expected;
  const matches = expected === null ? null : transportValid && actual.status === expected.exit &&
    stdout.equals(Buffer.from(expected.stdout)) && stderr.equals(Buffer.from(expected.stderr));
  rows.push({id: cell.id, profile: cell.profile, order: cell.order, source: cell.source,
    source_sha256: cell.source_sha256, argv, cwd: root, stdin_base64: '', start,
    end: new Date().toISOString(), exit: actual.status, signal: actual.signal,
    error: actual.error?.message || null, timeout_ms: 30000,
    stdout_base64: stdout.toString('base64'), stderr_base64: stderr.toString('base64'),
    stdout_sha256: sha(stdout), stderr_sha256: sha(stderr), transport_valid: transportValid,
    expected, literal_match: matches, unknown: expected === null || !transportValid});
}
const after = state();
const unchanged = before.head === after.head && JSON.stringify(before.product) === JSON.stringify(after.product) &&
  JSON.stringify(before.inputs) === JSON.stringify(after.inputs);
const receipt = {schema: 'p1339-array-candidate-public-engineering-v1',
  role: 'implementer execution and literal comparison only; independent verdict required',
  mode, before, after, all_inputs_unchanged: unchanged,
  environment_overrides: {NO_COLOR: '1', PYTHONDONTWRITEBYTECODE: '1'},
  relevant_environment: Object.fromEntries(Object.entries(env).filter(([k]) =>
    ['PATH', 'LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', 'HOME', 'NO_COLOR', 'PYTHONDONTWRITEBYTECODE', 'SOURCE_DATE_EPOCH'].includes(k) || /^(TYPST_|FONTCONFIG_|XDG_)/.test(k))),
  rows, processes: rows.length, literal_matches: rows.filter(x => x.literal_match === true).length,
  literal_failures: rows.filter(x => x.literal_match === false).length,
  transport_failures: rows.filter(x => !x.transport_valid).length,
  exploratory_unknown: rows.filter(x => x.expected === null).length,
  global_P1339_PASS: false};
const text = JSON.stringify(receipt, null, 2) + '\n';
cp.execFileSync('apply_patch', {input: `*** Begin Patch\n*** Add File: ${output}\n${text.trimEnd().split('\n').map(x => '+' + x).join('\n')}\n*** End Patch\n`});
console.log(JSON.stringify({receipt: output, sha256: sha(fs.readFileSync(output)),
  matches: receipt.literal_matches, failures: receipt.literal_failures,
  transport_failures: receipt.transport_failures, exploratory_unknown: receipt.exploratory_unknown, unchanged}));
