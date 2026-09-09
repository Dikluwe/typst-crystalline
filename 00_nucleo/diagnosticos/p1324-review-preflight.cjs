const fs = require('node:fs');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
const read = p => fs.readFileSync(p);
const run = (bin, argv) => { const t = new Date().toISOString(); const r = cp.spawnSync(bin, argv, {encoding:'utf8', env:{...process.env,NO_COLOR:'1'},maxBuffer:8e6}); return {at:t,bin,argv,exit:r.status,stdout:r.stdout,stderr:r.stderr,error:r.error?.message}; };
const owner = '01_core/src/compiler/eval/bindings/field_access.rs';
const l0 = '00_nucleo/prompts/compiler/eval/bindings/field_access.md';
const nucleus = '00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml';
const protectedFiles = ['00_nucleo/prompts/compiler/eval/call_dispatch.md','00_nucleo/prompts/compiler/stdlib/loading.md','00_nucleo/prompts/wiring.md','01_core/src/compiler/eval/call_dispatch.rs','01_core/src/compiler/stdlib/loading.rs','04_wiring/src/main.rs'];
const result = {at:new Date().toISOString(),head:run('git',['rev-parse','HEAD']).stdout.trim(),diffStat:run('git',['diff','HEAD','--stat']).stdout,diff:run('git',['diff','HEAD','--',...protectedFiles]).stdout,hashes:Object.fromEntries([...protectedFiles,owner,l0,nucleus].map(p=>[p,sha(read(p))])),ownerText:read(owner).toString(),l0Text:read(l0).toString(),runs:[]};
for (const bin of ['/tmp/p1323-target.9NrOxY/release/typst','/usr/local/bin/typst']) {
  result.hashes[bin] = sha(read(bin));
  for(const expr of ['assert.missing','json.missing','yaml.missing','toml.missing','cbor.missing','table.missing','json.with().missing','{ let renamed = json; renamed.missing }','table.missing(1)','type(json.encode)','csv.encode','{ let json(x) = x; json.missing }']) result.runs.push(run(bin,['eval',expr]));
}
result.end = new Date().toISOString();
fs.writeFileSync('00_nucleo/diagnosticos/p1324-review-preflight.json',JSON.stringify(result,null,2)+'\n');
console.log(JSON.stringify({at:result.at,head:result.head,runs:result.runs.map(r=>({bin:r.bin,expr:r.argv[1],exit:r.exit,stdout:r.stdout,stderr:r.stderr})),hashes:result.hashes},null,2));
