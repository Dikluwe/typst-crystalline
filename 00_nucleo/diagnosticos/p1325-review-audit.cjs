// Read-only independent audit of judged inputs; append-only review output.
const fs=require('node:fs'), cp=require('node:child_process'), crypto=require('node:crypto');
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const read=p=>fs.readFileSync(p);
const d='00_nucleo/diagnosticos/';
const phase=process.argv[2];
if(!/^[a-z0-9-]+$/.test(phase||'')) throw Error('phase required');
const output=d+`p1325-review-${phase}-audit.json`;
if(fs.existsSync(output)) throw Error('append-only receipt exists');
const base=JSON.parse(read(d+'p1325-baseline.json'));
const manifest=JSON.parse(read(d+'p1325-manifest.json'));
const prompt=read(manifest.prompt).toString(), source=read(manifest.owner).toString();
const normative=prompt.replace(/^Hash do Código: [a-f0-9]{8}\n/m,'');
const clean=s=>s.replace(/^\/\/! @prompt-hash [a-f0-9]{8}\n/m,'');
const deps=[...prompt.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/[^ ]+) sha256:([a-f0-9]{64})$/gm)].map(([,path,pin])=>{
  const bytes=read(path); if(bytes.toString().includes('[[dependencies]]')) throw Error('recursive nucleus requires deeper audit');
  const effective=sha(Buffer.concat([bytes,Buffer.from('\0TEKT-NUCLEUS-DEPS-V1\0')]));
  return {path,pin,effective,match:pin===effective};
});
const pieces=[Buffer.from(normative)];
if(deps.length){pieces.push(Buffer.from('\0TEKT-PROMPT-NUCLEI-V1\0'));for(const x of deps.sort((a,b)=>a.path.localeCompare(b.path))){const n=Buffer.alloc(8);n.writeBigUInt64BE(BigInt(Buffer.byteLength(x.path)));pieces.push(n,Buffer.from(x.path),Buffer.from(' '),Buffer.from(x.effective,'hex'));}}
const expectedHeader=sha(Buffer.concat(pieces)).slice(0,8), actualHeader=source.match(/^\/\/! @prompt-hash ([a-f0-9]{8})$/m)[1];
const expectedCode=sha(clean(source)).slice(0,8),actualCode=prompt.match(/^Hash do Código: ([a-f0-9]{8})$/m)[1];
const preserved=Object.entries(base.state.product_inventory).filter(([p])=>p!==manifest.prompt&&p!==manifest.owner).map(([path,before])=>({path,before,after:sha(read(path))}));
const run=args=>{const r=cp.spawnSync('git',args,{encoding:'utf8',maxBuffer:30e6});return {args,exit:r.status,stdout:r.stdout,stderr:r.stderr};};
const result={phase,at:new Date().toISOString(),baselineSha256:sha(read(d+'p1325-baseline.json')),manifestSha256:sha(read(d+'p1325-manifest.json')),sourceSha256:sha(source),promptSha256:sha(prompt),normativeSha256:sha(normative),normativeFrozen:sha(normative)===manifest.prompt_norm_sha256,sourceEqualsBaseline:source===base.original_owner,cleanSourceEqualsBaseline:clean(source)===clean(base.original_owner),lineage:{expectedHeader,actualHeader,headerMatches:expectedHeader===actualHeader,expectedCode,actualCode,reciprocalMatches:expectedCode===actualCode,deps},preserved:preserved.every(x=>x.before===x.after),changedProtected:preserved.filter(x=>x.before!==x.after),head:run(['rev-parse','HEAD']),diffStat:run(['diff','HEAD','--stat'])};
fs.writeFileSync(output,JSON.stringify(result,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({output,sha256:sha(read(output)),...result},null,2));
