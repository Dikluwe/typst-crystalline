const fs=require('node:fs'), crypto=require('node:crypto'), cp=require('node:child_process');
const sha=x=>crypto.createHash('sha256').update(x).digest('hex');
const read=p=>fs.readFileSync(p),d='00_nucleo/diagnosticos/';
const b=JSON.parse(read(d+'p1325-baseline.json')),m=JSON.parse(read(d+'p1325-test-succession-manifest.json')),f=JSON.parse(read(d+'p1325-ab-legacy-freeze.json'));
const normCode=s=>s.replace(/^\/\/! @prompt-hash [a-f0-9]{8}\n/m,'');
const pairs=[['01_core/src/compiler/eval/bindings/field_access.rs','00_nucleo/prompts/compiler/eval/bindings/field_access.md'],['01_core/src/compiler/eval/tests.rs','00_nucleo/prompts/compiler/eval/tests.md']];
const lineage=pairs.map(([sourcePath,promptPath])=>{
 const s=read(sourcePath).toString(),p=read(promptPath).toString(),norm=p.replace(/^Hash do Código: [a-f0-9]{8}\n/m,'');
 const deps=[...p.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/[^ ]+) sha256:([a-f0-9]{64})$/gm)].map(([,path,pin])=>{const bytes=read(path);if(bytes.toString().includes('[[depends]]'))throw Error('recursive nucleus not supported here');return {path,pin,effective:sha(Buffer.concat([bytes,Buffer.from('\0TEKT-NUCLEUS-DEPS-V1\0')]))};});
 const pieces=[Buffer.from(norm),Buffer.from('\0TEKT-PROMPT-NUCLEI-V1\0')];
 for(const x of deps.sort((a,b)=>a.path.localeCompare(b.path))){const len=Buffer.alloc(8);len.writeBigUInt64BE(BigInt(Buffer.byteLength(x.path)));pieces.push(len,Buffer.from(x.path),Buffer.from(' '),Buffer.from(x.effective,'hex'));}
 const hashA=sha(Buffer.concat(pieces)),hashB=sha(normCode(s));
 return {sourcePath,promptPath,rawSource:sha(s),rawPrompt:sha(p),normative:sha(norm),hashA,hashB,header:s.match(/^\/\/! @prompt-hash ([a-f0-9]{8})$/m)[1],reciprocal:p.match(/^Hash do Código: ([a-f0-9]{8})$/m)[1],deps};
});
const allowed=pairs.flat();
const changes=Object.entries(b.state.product_inventory).filter(([p])=>!allowed.includes(p)).filter(([p,h])=>sha(read(p))!==h).map(([p])=>p);
const af=JSON.parse(read(d+'p1325-ab-freeze.json'));
const protectedChanged=Object.entries({...af.protected,...f.protected,...m.frozen_runtime_pair}).filter(([p,h])=>sha(read(p))!==h).map(([p])=>p);
const currentTests=read(f.source).toString();
const run=args=>{const r=cp.spawnSync('git',args,{encoding:'utf8',maxBuffer:3e6});return {args,exit:r.status,stdout:r.stdout,stderr:r.stderr};};
const result={at:new Date().toISOString(),head:run(['rev-parse','HEAD']),diffstat:run(['diff','HEAD','--stat']),staged:run(['diff','--cached']),lineage,changes,protectedChanged,testDeltaMatchesFrozen:sha(normCode(currentTests))===f.expected_updated_without_prompt_hash_sha256,legacyNormativeMatches:lineage[1].normative===m.prompt_norm_sha256,runtimeNormativeMatches:lineage[0].normative===af.prompt.normalized_sha256,baselineSha256:sha(read(d+'p1325-baseline.json')),successionManifestSha256:sha(read(d+'p1325-test-succession-manifest.json')),legacyFreezeSha256:sha(read(d+'p1325-ab-legacy-freeze.json')),abFreezeSha256:sha(read(d+'p1325-ab-freeze.json'))};
result.status=!changes.length&&!protectedChanged.length&&result.testDeltaMatchesFrozen&&result.legacyNormativeMatches&&result.runtimeNormativeMatches&&lineage.every(x=>x.header===x.hashA.slice(0,8)&&x.reciprocal===x.hashB.slice(0,8)&&x.deps.every(n=>n.pin===n.effective))?'PASS':'FAIL';
const out=d+'p1325-review-succession-final-audit.json';fs.writeFileSync(out,JSON.stringify(result,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify({output:out,sha256:sha(read(out)),...result},null,2));
