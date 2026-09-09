const fs = require('fs');
const crypto = require('crypto');
const d = '00_nucleo/diagnosticos/';
const read = p => JSON.parse(fs.readFileSync(p));
const sha = x => crypto.createHash('sha256').update(x).digest('hex');
const hashFile = p => sha(fs.readFileSync(p));
const assert = (p, msg) => { if (!p) throw new Error(msg); };
const m = read(d + 'p1327-r2-manifest.json');
const b = read(d + 'p1327-baseline.json');
const b2 = read(d + 'p1327-r2-baseline.json');
const strip = s => s.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m, '');
const provenance = read(d+'p1327-r2-workspace-tests.json').after;
const result = {at: new Date().toISOString(), head: provenance.head, diff_stat: provenance.diff_stat, provenance_receipt_sha256:hashFile(d+'p1327-r2-workspace-tests.json'), manifest_sha256: hashFile(d+'p1327-r2-manifest.json'), lineage: [], artifacts: {}, gates: {}, cli: []};
for (let i=0;i<m.owners.length;i+=2) {
  const owner=m.owners[i], p=m.owners[i+1], s=fs.readFileSync(owner,'utf8'), prompt=fs.readFileSync(p,'utf8');
  const norm=prompt.replace(/^Hash do Código: [0-9a-f]{8}\n/m,'');
  assert(sha(norm)===m.prompts[p],'norm '+p);
  let payload=Buffer.from(norm+'\0TEKT-PROMPT-NUCLEI-V1\0');
  const pins=[...prompt.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/\S+) sha256:([0-9a-f]{64})$/gm)].sort((a,b)=>a[1].localeCompare(b[1]));
  for(const [,path,h] of pins){const nucleus=fs.readFileSync(path);assert(!nucleus.includes('[[depends]]'),'unhandled deps');const pin=sha(Buffer.concat([nucleus,Buffer.from('\0TEKT-NUCLEUS-DEPS-V1\0')]));assert(pin===h,'pin');const bytes=Buffer.from(path),len=Buffer.alloc(8);len.writeBigUInt64BE(BigInt(bytes.length));payload=Buffer.concat([payload,len,bytes,Buffer.from([32]),Buffer.from(pin,'hex')]);}
  const a=sha(payload),z=sha(strip(s));
  assert(s.match(/^\/\/! @prompt-hash (.*)$/m)[1]===a.slice(0,8),'A');
  assert(prompt.match(/^Hash do Código: (.*)$/m)[1]===z.slice(0,8),'B');
  result.lineage.push({owner,prompt:p,hash_a:a,hash_b:z});
}
const cur=fs.readFileSync('04_wiring/src/main.rs','utf8');
const mask=s=>strip(s).replace(/fn run_eval\([\s\S]*?(?=\nfn run_query\()/,'<run_eval>');
assert(mask(cur)===mask(b2.original_wiring),'outside run_eval changed');
const r1=read(d+'p1327-ab-r1-format.json'), legacy=read(d+'p1327-ab-legacy-replacements.json');
let tests=b.originals['01_core/src/compiler/eval/tests.rs'];
for(const r of r1.canonical_legacy_successors){assert(tests.split(r.old).length===2,'old block');tests=tests.replace(r.old,r.new);}
tests=tests.replace(legacy.append_before_unique_anchor,fs.readFileSync(legacy.snippet,'utf8')+'\n'+legacy.append_before_unique_anchor);
assert(strip(tests)===strip(fs.readFileSync('01_core/src/compiler/eval/tests.rs','utf8')),'tests changed');
for(const [p,h] of Object.entries(read(d+'p1327-ab-freeze.json').hashes)){if(!m.prompts[p])assert(hashFile(p)===h,'frozen '+p);}
for(const p of ['p1327-manifest.json','p1327-baseline.json','p1327-ab-freeze.json','p1327-ab-r1-format.json','p1327-ab-r2-oracle.json','p1327-ab-r2-runner.py','p1327-ab-r2-measure.json','p1327-ab-cli-candidate.json'])result.artifacts[p]=hashFile(d+p);
for(const [candidate,oracle] of [['p1327-ab-cli-candidate-c2.json','p1327-ab-cli-oracle.json'],['p1327-ab-r2-candidate.json','p1327-ab-r2-oracle.json']]){
  const x=read(d+candidate),o=read(d+oracle),e=new Map(o.expected.map(r=>[r.profile+'/'+r.id,r]));
  const obs=r=>JSON.stringify([r.exit,r.stdout,r.stderr]);
  const keys=new Set();for(const r of x.runs){const key=r.profile+'/'+r.id;assert(obs(r)===obs(e.get(key)),'CLI differs');assert(r.verdict==='Preserved','wrong verdict');assert(!keys.has(r.order+'/'+key),'duplicate');keys.add(r.order+'/'+key);}
  assert(x.runs.length===e.size*3,'missing CLI');assert(x.candidate_sha256===hashFile('/tmp/p1327-target.k9Mq0s/release/typst'),'binary');
  result.cli.push({candidate,sha256:hashFile(d+candidate),runs:x.runs.length,binary_sha256:x.candidate_sha256});
}
for(const n of ['final-build','unit-green','final-fmt','final-lineage','final-lineage-lint','final-lint','final-diff-check','workspace-tests']){const p=d+'p1327-r2-'+n+'.json';const x=read(p);assert(x.exit===0,'gate '+n);result.gates[n]={sha256:hashFile(p),at:x.at,end:x.end,argv:x.argv,exit:x.exit,summaries:(x.stdout||'').split('\n').filter(l=>l.includes('test result:')||l.includes('violations'))};}
const warnings=x=>(x.stdout+x.stderr).split('\n').filter(l=>l.startsWith('warning:'));
const oldLint=read(d+'p1326-final-lint.json'),newLint=read(d+'p1327-r2-final-lint.json');
assert(JSON.stringify(warnings(oldLint))===JSON.stringify(warnings(newLint)),'lint warning drift');
result.lint_warning_preservation={baseline_sha256:hashFile(d+'p1326-final-lint.json'),before:warnings(oldLint).length,after:warnings(newLint).length,ordered_messages_identical:true};
const gate=read(d+'p1327-r2-workspace-tests.json');
const changed=Object.keys(b.state.product_inventory).filter(p=>b.state.product_inventory[p]!==gate.after.product_inventory[p]);
assert(changed.every(p=>m.owners.includes(p)),'outside owners');
assert(Object.keys(gate.after.product_inventory).every(p=>p in b.state.product_inventory),'new product');
result.changed_product_paths=changed;result.pass=true;
console.log(JSON.stringify(result,null,2));
