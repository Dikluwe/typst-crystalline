const fs=require('node:fs'),crypto=require('node:crypto');
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const d='00_nucleo/diagnosticos/';
const load=n=>JSON.parse(fs.readFileSync(d+n));
const freeze=load('p1325-ab-freeze.json'), r=load('p1325-ab-candidate-cli.json'), e=load('p1325-ab-expectations.json');
const expected=new Map(e.expected.map(x=>[JSON.stringify([x.case,x.profile]),x]));
if(expected.size!==e.expected.length)throw Error('duplicate expectations');
const observed=new Map(), mismatches=[];
const obs=x=>JSON.stringify([x.exit,x.stdout,x.stderr]);
for(const row of r.rows){
 const key=JSON.stringify([row.case,row.profile]),k=JSON.stringify([row.order,row.product,row.case,row.profile]);
 if(observed.has(k))mismatches.push({reason:'duplicate',key:k});
 observed.set(k,row);
 if(!expected.has(key)||!expected.get(key)[row.product]||obs(row)!==obs(expected.get(key)[row.product]))mismatches.push({reason:'observable',key:k});
}
for(const order of ['normal','repeat','reverse'])for(const product of ['baseline','vanilla','candidate'])for(const row of e.expected){const k=JSON.stringify([order,product,row.case,row.profile]);if(!observed.has(k))mismatches.push({reason:'missing',key:k});}
const protectedChanged=Object.entries(freeze.protected).filter(([p,h])=>sha(fs.readFileSync(p))!==h).map(([p])=>p);
const p=fs.readFileSync(freeze.prompt.path,'utf8').replace(/^Hash do Código: [a-f0-9]{8}\n/m,'');
const binaries=Object.entries(r.binaries).map(([role,x])=>({role,...x,current:sha(fs.readFileSync(x.path))}));
const pinsGood=binaries.every(x=>x.current===x.sha256)&&r.binaries.baseline.sha256===freeze.baseline_binary_sha256&&r.binaries.vanilla.sha256===freeze.vanilla_binary_sha256&&r.binaries.candidate.sha256===load('p1325-final-build.json').binary.sha256;
const classes={};for(const x of e.expected)classes[x.classification]=(classes[x.classification]||0)+1;
const result={at:new Date().toISOString(),head:r.head,diffstat:r.diffstat,freezeSha256:sha(fs.readFileSync(d+'p1325-ab-freeze.json')),receiptSha256:sha(fs.readFileSync(d+'p1325-ab-candidate-cli.json')),expectationsSha256:sha(fs.readFileSync(d+'p1325-ab-expectations.json')),rows:r.rows.length,expectedRows:expected.size*9,classes,mismatches,protectedChanged,normativeMatches:sha(p)===freeze.prompt.normalized_sha256,binaries,pinsGood};
result.status=!mismatches.length&&!protectedChanged.length&&result.normativeMatches&&pinsGood?'PASS':'FAIL';
const out=d+'p1325-review-ab-final-audit.json';fs.writeFileSync(out,JSON.stringify(result,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify({output:out,sha256:sha(fs.readFileSync(out)),...result},null,2));
