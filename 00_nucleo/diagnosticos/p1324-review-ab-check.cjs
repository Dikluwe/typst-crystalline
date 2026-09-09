const fs=require('node:fs'),crypto=require('node:crypto');
const d='00_nucleo/diagnosticos/';
const sha=x=>crypto.createHash('sha256').update(x).digest('hex');
const read=n=>JSON.parse(fs.readFileSync(d+n));
const current=process.argv[2];
if(!current||!/^[a-z0-9-]+\.json$/.test(current))throw Error('candidate receipt filename required');
const freeze=read('p1324-ab-freeze-v3.json'),cases=read('p1324-ab-cases-v3.json');
const oracle=read('p1324-ab-oracle-v3.json'),baseline=read('p1324-ab-baseline-v3.json'),candidate=read(current);
const obs=r=>r.execution==='complete'&&[0,1].includes(r.exit)&&typeof r.stdout==='string'&&typeof r.stderr==='string'?JSON.stringify([r.exit,r.stdout,r.stderr]):null;
const key=r=>JSON.stringify([r.case,r.profile]);
const refs=x=>new Map(x.runs.map(r=>[key(r),r]));
const o=refs(oracle),b=refs(baseline),first=new Map(),def=new Map(cases.cases.map(c=>[c.id,c]));
const frozen=freeze.protected.map(x=>({...x,actual:sha(fs.readFileSync(x.path)),same:sha(fs.readFileSync(x.path))===x.sha256}));
const failures=[],orders={},rules={},unique=new Set();let stabilityChecks=0;
for(let r of candidate.runs){const id=key(r),c=def.get(r.case),ref=(c.rule==='vanilla'?o:b).get(id);unique.add(id);orders[r.order]=(orders[r.order]||0)+1;rules[c.rule]=(rules[c.rule]||0)+1;if(!obs(r)||!obs(ref)||obs(r)!==obs(ref)||r.obligation?.status!=='Preserved'||r.shape_ok!==true)failures.push({id,order:r.order,kind:'obligation'});if(first.has(id)){stabilityChecks++;if(obs(r)!==first.get(id)||r.stability?.status!=='Preserved')failures.push({id,order:r.order,kind:'stability'});}else first.set(id,obs(r));}
const out={at:new Date().toISOString(),candidate:current,candidateReceiptSha256:sha(fs.readFileSync(d+current)),binarySha256:candidate.binary_sha256,binaryCurrentlySame:sha(fs.readFileSync(candidate.binary))===candidate.binary_sha256,freezeSha256:sha(fs.readFileSync(d+'p1324-ab-freeze-v3.json')),frozen,processes:candidate.runs.length,unique:unique.size,orders,rules,stabilityChecks,failures,allFrozen:frozen.every(x=>x.same),manifestSame:candidate.manifest_sha256===sha(fs.readFileSync(d+'p1324-manifest.json')),scope:'Independent recomputation from all raw exit/stdout/stderr channels; candidate orders only, no claim baseline repeated'};
const path=d+'p1324-review-ab-final-audit.json';if(fs.existsSync(path))throw Error('append-only receipt');fs.writeFileSync(path,JSON.stringify(out,null,2)+'\n');console.log(JSON.stringify(out,null,2));
