const fs = require('fs');
const crypto = require('crypto');
const cp = require('child_process');
const exec = require('util').promisify(cp.execFile);
const path = require('path');
const root = '/repos/Antigravity/typst-crystalline';
process.chdir(root);
const dir = '00_nucleo/diagnosticos/';
const hash = x => crypto.createHash('sha256').update(x).digest('hex');
const read = p => fs.readFileSync(p);
const json = n => JSON.parse(read(dir + 'p1336-' + n + '.json'));
const manifest = json('manifest');
const baseline = json('baseline');
const prohibited = p => /(^|\/)00_nucleo\/(materialization|context)(\/|$)/.test(p);
const digest = p => prohibited(p) ? 'NOT_READ_RESTRICTED' : fs.existsSync(p) ? hash(read(p)) : 'MISSING';
const normative = read(manifest.prompt.path).toString().replace(/^Hash do Código:.*\n/m, '');
const changed = Object.entries(baseline.state.product_inventory).filter(([p,h]) => digest(p) !== h).map(([p]) => p);
const historical = Object.entries(baseline.historical_preserved).filter(([p]) => !prohibited(p));
const historyChanged = historical.filter(([p,h]) => digest(p) !== h).map(([p]) => p);
(async () => {
const inventories = (await exec('git', ['ls-files', '--cached', '--others', '--exclude-standard', '--', '00_nucleo/prompts', '01_core', '02_shell', '03_infra', '04_wiring', 'lab/typst-original', 'lab/surface-inventory', 'benches', 'Cargo.toml', 'Cargo.lock'], {encoding:'utf8'})).stdout.trim().split('\n');
const newFiles = inventories.filter(p => !(p in baseline.state.product_inventory));
const sourceText=read(manifest.source).toString();
const testStart=sourceText.indexOf('#[cfg(test)]\nmod p1336_tests {');
const testEnd=sourceText.indexOf('#[cfg(test)]\nmod p1326_tests {',testStart);
const reciprocal=hash(sourceText.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m,''));
const branch=(await exec('git',['branch','--show-current'],{encoding:'utf8'})).stdout.trim();
const staged=(await exec('git',['diff','--cached','--binary','--','.',':!00_nucleo/materialization',':!00_nucleo/context'],{encoding:'utf8'})).stdout;
const fullDiff=(await exec('git',['diff','HEAD','--binary','--','.',':!00_nucleo/materialization',':!00_nucleo/context'],{encoding:'utf8',maxBuffer:16*1024*1024})).stdout;
const offscope=d=>d.split(/(?=^diff --git )/m).filter(chunk=>!manifest.policy.allowed_product_changes.some(p=>chunk.startsWith('diff --git a/'+p+' b/'+p+'\n'))).join('');
const result = {
  at: new Date().toISOString(),
  manifest_sha256: hash(read(dir + 'p1336-manifest.json')),
  baseline_sha256: hash(read(dir + 'p1336-baseline.json')),
  head: (await exec('git',['rev-parse','HEAD'],{encoding:'utf8'})).stdout.trim(),
  branch,
  branch_unchanged: branch===baseline.state.branch,
  staged_unchanged: staged===baseline.state.staged,
  tracked_diff_outside_pair_unchanged: offscope(fullDiff)===offscope(baseline.state.diff),
  diff_stat: (await exec('git',['diff','HEAD','--stat','--','.',':!00_nucleo/materialization',':!00_nucleo/context'],{encoding:'utf8'})).stdout,
  source_sha256: digest(manifest.source),
  prompt_sha256: digest(manifest.prompt.path),
  normative_sha256: hash(normative),
  normative_frozen: normative === manifest.prompt.normative_text,
  rust_tests_sha256: testStart>=0&&testEnd>testStart?hash(sourceText.slice(testStart,testEnd)):null,
  reciprocal_hash_b: reciprocal.slice(0,8),
  reciprocal_metadata_valid: read(manifest.prompt.path).toString().match(/^Hash do Código: ([0-9a-f]{8})$/m)?.[1]===reciprocal.slice(0,8),
  product_changed_from_dirty_baseline: changed,
  unexpected_product_changes: changed.filter(p => !manifest.policy.allowed_product_changes.includes(p)),
  new_product_files: newFiles,
  historical_checked: historical.length,
  historical_changed: historyChanged,
  restricted_history_excluded: Object.keys(baseline.historical_preserved).filter(prohibited).length,
  lint_path: '/home/dikluwe/.cargo/bin/crystalline-lint',
  lint_sha256: digest('/home/dikluwe/.cargo/bin/crystalline-lint'),
  regime: 'A/B executado sem atestação de isolamento; sem refinement seal',
};
if (process.argv[2]) {
  if (!/^p1336-review-[a-z0-9-]+\.json$/.test(process.argv[2])) throw new Error('review output only');
  const output = path.join(dir, process.argv[2]);
  if(fs.existsSync(output)) throw new Error('preserve previous receipt');
  const patch = '*** Begin Patch\n*** Add File: '+output+'\n'+JSON.stringify(result,null,2).split('\n').map(s=>'+'+s).join('\n')+'\n*** End Patch\n';
  await new Promise((resolve,reject) => { const child=cp.spawn('apply_patch',[],{stdio:['pipe','pipe','pipe']});let err='';child.stderr.on('data',x=>err+=x);child.on('error',reject);child.on('close',code=>code===0?resolve():reject(new Error(err)));child.stdin.end(patch); });
}
console.log(JSON.stringify(result,null,2));
})().catch(e => { console.error(e.message); process.exitCode=1; });
