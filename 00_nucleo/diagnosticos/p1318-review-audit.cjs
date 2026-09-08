const fs = require('fs');
const cp = require('child_process');
const crypto = require('crypto');
const D = '00_nucleo/diagnosticos/';
const P = '01_core/src/compiler/stdlib/loading.rs';
const read = p => JSON.parse(fs.readFileSync(p));
const sha = s => crypto.createHash('sha256').update(s).digest('hex');
const red = read(D + 'p1318-unit-red.json');
const orig = cp.execFileSync('git', ['show', red.before.head + ':' + P], {encoding:'utf8'}).split('\n');
const patch = red.before.diff.split('diff --git a/' + P + ' b/' + P + '\n')[1].split('\n');
let result = [], at = 0, active = false;
for (const line of patch) {
  const h = /^@@ -(\d+)(?:,\d+)? \+\d+(?:,\d+)? @@/.exec(line);
  if (h) { const start = Number(h[1]) - 1; result.push(...orig.slice(at,start)); at=start; active=true; continue; }
  if (!active) continue;
  if (line.startsWith(' ')) { if (orig[at] !== line.slice(1)) throw Error('context'); result.push(orig[at++]); }
  else if (line.startsWith('-')) { if (orig[at] !== line.slice(1)) throw Error('deletion'); at++; }
  else if (line.startsWith('+')) result.push(line.slice(1));
}
result.push(...orig.slice(at));
const reconstructed = result.join('\n');
const now = fs.readFileSync(P,'utf8');
const baseline = read(D+'p1318-measurement.json');
const base = baseline.sources[P].text;
const clean = s => s.replace(/^.*@prompt-hash.*\n/m,'').replace(/^.*@prompt.*\n/m,'');
const beforeDecode = s => s.slice(0,s.indexOf('pub fn decode_csv('));
const otherNatives = s => s.slice(s.indexOf('fn arg_csv_source('),s.indexOf('pub fn native_csv('));
const freeze = read(D+'p1318-ab-freeze.json');
const l0 = fs.readFileSync(freeze.l0.path,'utf8');
const receipt = {
  utc: new Date().toISOString(),
  head: cp.execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),
  diff_stat: cp.execFileSync('git',['diff','HEAD','--stat'],{encoding:'utf8'}),
  source_sha256: sha(now),
  l0_sha256: sha(l0),
  red_source_reconstructed_sha256: sha(reconstructed),
  red_source_hash_matches: sha(reconstructed) === red.before.files[P],
  entire_tests_identical_red_candidate: reconstructed.split('#[cfg(test)]')[1] === now.split('#[cfg(test)]')[1],
  prior_to_decode_unchanged_except_lineage: clean(beforeDecode(base)) === clean(beforeDecode(now)),
  other_native_functions_unchanged: otherNatives(base) === otherNatives(now),
  normative_l0_unchanged: sha(l0.replace(/^Hash do Código: [^\r\n]+\r?\n/m,'')) === freeze.l0.normative_sha256,
  freeze_input_mismatches: Object.entries(freeze.inputs).filter(([p,h])=>sha(fs.readFileSync(p))!==h).map(([p])=>p),
  prior_artifact_mismatches: Object.entries(baseline.prior_artifacts).filter(([p,h])=>sha(fs.readFileSync(p))!==h).map(([p])=>p),
};
const runsPath = D+'p1318-ab-candidate-runs.json';
if (fs.existsSync(runsPath)) {
  const runs = read(runsPath), comparison = read(D+'p1318-ab-comparison.json');
  const expected = new Map(freeze.expected.map(e=>[[e.id,e.profile].join('|'),e]));
  const required = new Set(freeze.expected.flatMap(e=>['normal','repeat','reverse'].map(o=>[e.id,e.profile,o].join('|'))));
  const seen = new Set(), failures = [];
  for (const r of runs.rows) {
    const key = [r.id,r.profile,r.order].join('|'), e = expected.get([r.id,r.profile].join('|'));
    if (seen.has(key) || !required.has(key)) failures.push(key+' identity');
    seen.add(key);
    if (!e || r.product!=='candidate' || r.argv[2]!==e.expr || r.cwd!==e.cwd || ['exit','stdout','stderr'].some(k=>r[k]!==e.expected[k])) failures.push(key+' observation');
  }
  receipt.ab = {freeze_sha256:sha(fs.readFileSync(D+'p1318-ab-freeze.json')),runs_sha256:sha(fs.readFileSync(runsPath)),comparison_sha256:sha(fs.readFileSync(D+'p1318-ab-comparison.json')),rows:runs.rows.length,required:required.size,missing:[...required].filter(k=>!seen.has(k)),failures,comparison_status:comparison.status,reported_unknown:comparison.unknown,utc:[runs.before.utc,runs.after.utc],binary:runs.binaries.candidate,binary_sha256_now:sha(fs.readFileSync(runs.binaries.candidate.path))};
  receipt.gates = {};
  for (const name of ['unit-red','unit-green','build','workspace-tests','lineage-final','lint','fmt','diff-check']) {
    const p=D+'p1318-'+name+'.json', g=read(p);
    receipt.gates[name]={sha256:sha(fs.readFileSync(p)),argv:g.argv,exit:g.exit,utc:[g.before.utc,g.after.utc],source_and_l0_stable:JSON.stringify(g.before.files)===JSON.stringify(g.after.files),files:g.after.files};
    if (name==='workspace-tests') receipt.gates[name].passed_failed_ignored=[...g.stdout.matchAll(/test result: .*?(\d+) passed; (\d+) failed; (\d+) ignored/g)].reduce((a,m)=>a.map((n,i)=>n+Number(m[i+1])),[0,0,0]);
    if (name==='lint') receipt.gates[name].error_warning_info=['error','warning','info'].map(k=>(g.stdout.match(new RegExp('^'+k+':','gm'))||[]).length);
  }
}
console.log(JSON.stringify(receipt,null,2));
