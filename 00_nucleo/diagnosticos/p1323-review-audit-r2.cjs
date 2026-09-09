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
const git = args => { try { return cp.execFileSync('git', args, {encoding:'utf8'}); } catch (e) { if(e.code === 'EPERM' && e.status === 0 && typeof e.stdout === 'string') return e.stdout; throw e; } };
const tracked = git(['diff', 'HEAD', '--stat']);
const audit = {at:new Date().toISOString(),head:git(['rev-parse','HEAD']).trim(),working_tree_diff_stat:tracked,
  baseline_sha256:sha(fs.readFileSync(D+'p1323-baseline.json')), frozen_suite_sha256:sha(fs.readFileSync(D+'p1323-ab-frozen-suite.json')),
  product_inventory_changed:changed, unauthorized_changes:changed.filter(row=>!row.allowed),
  exact_independent_test_integrated:hasExactTest, owner_restored_exact_baseline:restored===baseline.original_owner,
  owner_sha256:sha(owner),prompt_sha256:sha(prompt), normative_sha256:sha(prompt.replace(/^Hash do Código:.*\n/m,'')),
  normative_intact:sha(prompt.replace(/^Hash do Código:.*\n/m,''))===frozen.obligation.normative_sha256, protected_inputs:protectedRows};


const framed = (data, domain, deps) => {
  const h = crypto.createHash('sha256').update(data).update(Buffer.from([0])).update(domain).update(Buffer.from([0]));
  for (const [p,d] of deps.sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])))) {
    const n=Buffer.alloc(8); n.writeBigUInt64BE(BigInt(Buffer.byteLength(p)));
    h.update(n).update(p).update(Buffer.from([32])).update(d);
  }
  return h.digest();
};
const refs = [...prompt.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/[^ ]+) sha256:([a-f0-9]{64})$/gm)];
const deps = refs.map(([,p,pin])=>{
  const data=fs.readFileSync(p);
  if(data.includes(Buffer.from('[[depends]]'))) throw Error('Unsupported nucleus dependency');
  const digest=framed(data,'TEKT-NUCLEUS-DEPS-V1',[]);
  if(digest.toString('hex')!==pin) throw Error('Bad nucleus pin');
  return [p,digest];
});
const sourceHash=sha(owner.replace(/^\/\/! @prompt-hash [a-f0-9]{8}\n/m,''));
const promptHash=framed(prompt.replace(/^Hash do Código: [a-f0-9]{8}\n/m,''),'TEKT-PROMPT-NUCLEI-V1',deps).toString('hex').slice(0,8);
audit.lineage={source_hash:sourceHash,code_metadata:prompt.match(/Hash do Código: ([a-f0-9]{8})/)[1],prompt_effective:promptHash,source_header:owner.match(/@prompt-hash ([a-f0-9]{8})/)[1],nucleus_pins_valid:true};
audit.lineage.valid=audit.lineage.code_metadata===sourceHash.slice(0,8)&&audit.lineage.source_header===promptHash;
const r2=json('p1323-ab-process-receipt-r2.json'), f2=json('p1323-ab-frozen-suite-r2.json');
const original=json('p1323-ab-process-receipt.json');
const ids=bytes=>{
  const s=bytes.toString('latin1'), matches=[...s.matchAll(/<xmpMM:InstanceID>([A-Za-z0-9+/]{22}==)<\/xmpMM:InstanceID>/g)];
  if(matches.length!==1||s.split('<xmpMM:InstanceID>').length!==2) throw Error('Unsupported PDF instance');
  const m=matches[0],v=m[1], decoded=Buffer.from(v,'base64');
  if(decoded.length!==16||decoded.toString('base64')!==v)throw Error('Bad base64');
  const start=m.index+'<xmpMM:InstanceID>'.length, out=Buffer.from(bytes);
  out.fill(0,start,start+v.length);
  return sha(out);
};
audit.ab={cases_unchanged:JSON.stringify(f2.cases)===JSON.stringify(frozen.cases),warning_unchanged:f2.warning_base64===frozen.warning_base64,protected_unchanged:JSON.stringify(f2.protected)===JSON.stringify(frozen.protected),failures:[],artifact_failures:[],orders_stable:r2.order_checks.every(x=>x.stable),rounds:r2.rounds.map(r=>({order:r.order,cases:r.rows.length})),original_pdf_failure_preserved:original.rounds.every(r=>r.rows.find(x=>x.case.id==='preserve-pdf').checks.preservation==='Violated')};
for (const r of r2.rounds)for(const row of r.rows) {
  if(row.checks.warning!=='Preserved'||row.checks.preservation!=='Preserved')audit.ab.failures.push([r.order,row.case.id,row.checks]);
  for(const obs of Object.values(row.observations))if(obs.artifact&&sha(fs.readFileSync(obs.artifact.path))!==obs.artifact.sha256)audit.ab.artifact_failures.push(obs.artifact.path);
  if(row.case.format==='pdf') {
    const a=row.observations.baseline.artifact,b=row.observations.candidate.artifact;
    if(!a||!b||ids(fs.readFileSync(a.path))!==ids(fs.readFileSync(b.path)))audit.ab.artifact_failures.push(r.order+' PDF independent comparison');
  }
}
audit.gates={};
for(const p of ['p1323-candidate-build.json','p1323-unit-green.json','p1323-workspace-tests.json','p1323-fmt.json','p1323-final-lint.json','p1323-final-lineage-r2.json','p1323-final-diff-check-r2.json']) {
  const j=json(p);audit.gates[p]={sha256:sha(fs.readFileSync(D+p)),at:j.at,argv:j.argv,exit:j.exit};
}
audit.input_pins={};
for(const p of ['p1323-manifest.json','p1323-baseline.json','p1323-obligation-freeze.json','p1323-ab-frozen-suite.json','p1323-ab-frozen-suite-r2.json','p1323-ab-cli.py','p1323-ab-cli-r2.py','p1323-ab-process-receipt.json','p1323-ab-process-receipt-r2.json','p1323-ab-pdf-focal-receipt.json','p1323-unit-red.json','p1323-unit-green.json','p1323-seam-preservation.json','p1323-reciprocal-correction.json'])audit.input_pins[p]=sha(fs.readFileSync(D+p));
console.log(JSON.stringify(audit,null,2));

