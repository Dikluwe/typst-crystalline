// Independent audit; reads judged material and writes only a new review receipt.
const fs = require('node:fs'), cp = require('node:child_process'), crypto = require('node:crypto');
const hash = b => crypto.createHash('sha256').update(b).digest();
const hex = b => hash(b).toString('hex');
const baseDir = '00_nucleo/diagnosticos/';
const phase = process.argv[2];
if (!phase || !/^[a-z0-9-]+$/.test(phase)) throw Error('explicit phase required');
const out = baseDir + `p1324-review-${phase}-audit.json`;
if (fs.existsSync(out)) throw Error('review receipts are append-only');
const read = p => fs.readFileSync(p);
const run = (cmd,args) => {let r=cp.spawnSync(cmd,args,{encoding:'utf8',maxBuffer:20e6});return {cmd,args,exit:r.status,stdout:r.stdout,stderr:r.stderr};};
const pre = JSON.parse(read(baseDir+'p1324-review-preflight.json'));
const manifestBytes=read(baseDir+'p1324-manifest.json'), manifest=JSON.parse(manifestBytes);
const sourcePath=manifest.owner,promptPath=manifest.prompt.path;
const source=read(sourcePath).toString(),prompt=read(promptPath).toString();
const normative=prompt.replace(/^Hash do Código: [0-9a-f]{8}\n/m,'');
const cleanSource=source.replace(/^\/\/! @prompt-hash [0-9a-f]{8}\n/m,'');
const refs=[...prompt.matchAll(/^- (00_nucleo\/prompts\/_nuclei\/[^ ]+) sha256:([0-9a-f]{64})$/gm)];
const deps=refs.map(([,path,pin])=>{const bytes=read(path);if(bytes.toString().includes('[[dependencies]]'))throw Error('recursive nucleus requires explicit audit');const digest=hash(Buffer.concat([bytes,Buffer.from('\0TEKT-NUCLEUS-DEPS-V1\0')]));return {path,pin,digest,raw:hex(bytes),match:digest.toString('hex')===pin};});
let pieces=[Buffer.from(normative)];
if(deps.length){pieces.push(Buffer.from('\0TEKT-PROMPT-NUCLEI-V1\0'));for(const dep of deps.sort((a,b)=>a.path.localeCompare(b.path))){let len=Buffer.alloc(8);len.writeBigUInt64BE(BigInt(Buffer.byteLength(dep.path)));pieces.push(len,Buffer.from(dep.path),Buffer.from(' '),dep.digest);}}
const expectedHeader=hex(Buffer.concat(pieces)).slice(0,8),actualHeader=source.match(/^\/\/! @prompt-hash ([a-f0-9]{8})$/m)[1];
const expectedCode=hex(cleanSource).slice(0,8),actualCode=prompt.match(/^Hash do Código: ([a-f0-9]{8})$/m)[1];
const protectedFiles=Object.entries(pre.hashes).filter(([p])=>!p.startsWith('/')&&p!==sourcePath&&p!==promptPath);
const preserved=protectedFiles.map(([path,before])=>({path,before,after:hex(read(path)),same:before===hex(read(path))}));
const receipts={};
for(const name of fs.readdirSync(baseDir).filter(x=>/^p1324-.*\.json$/.test(x)&&!x.startsWith('p1324-review-'))){const bytes=read(baseDir+name),body=JSON.parse(bytes);receipts[name]={sha256:hex(bytes),keys:Object.keys(body),at:body.at,end:body.end,exit:body.exit,manifest_sha256:body.manifest_sha256};}
const result={phase,at:new Date().toISOString(),head:run('git',['rev-parse','HEAD']),diffStat:run('git',['diff','HEAD','--stat']),diffOwner:run('git',['diff','HEAD','--',sourcePath,promptPath]),sourceSha256:hex(source),promptSha256:hex(prompt),manifestSha256:hex(manifestBytes),normativeSha256:hex(normative),normativeFrozen:hex(normative)===manifest.prompt.normative_sha256,lineage:{expectedHeader,actualHeader,headerMatches:expectedHeader===actualHeader,expectedCode,actualCode,reciprocalMatches:expectedCode===actualCode,deps:deps.map(d=>({...d,digest:d.digest.toString('hex')}))},preserved,receipts};
fs.writeFileSync(out,JSON.stringify(result,null,2)+'\n');
console.log(JSON.stringify({output:out,sha256:hex(read(out)),phase,at:result.at,normativeFrozen:result.normativeFrozen,lineage:result.lineage,preserved:preserved.every(x=>x.same),receipts},null,2));
