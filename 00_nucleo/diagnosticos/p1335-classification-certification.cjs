const fs=require('fs'),crypto=require('crypto');
const b='00_nucleo/diagnosticos/',sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const pin=p=>({path:p,sha256:sha(p)}),read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const prior=read(b+'p1322-classification-certification-debt.json'),base=read(b+'p1335-baseline.json');
const references=new Map();
function inspect(x){if(!x||typeof x!=='object')return;if(x.path&&x.sha256&&x.path.startsWith(b)){const actual=sha(x.path);references.set(x.path,{path:x.path,expected_sha256:x.sha256,actual_sha256:actual,unchanged:actual===x.sha256});}for(const v of Object.values(x))if(typeof v==='object')inspect(v);}
inspect(prior);
const reports=[];for(let n=1323;n<=1334;n++)reports.push(pin(b+`p${n}-final-report.md`));
const data={schema:'p1335-classification-certification-debt-v1',at:new Date().toISOString(),
  baseline:pin(b+'p1335-baseline.json'),manifest:pin(b+'p1335-manifest.json'),head:base.state.head,working_tree:'non-committed',diff_stat:base.state.diff_stat,
  universe:'certification_debt',regime:'executado sem atestação técnica de isolamento',prior:pin(b+'p1322-classification-certification-debt.json'),reports_inspected:reports,
  references_checked:[...references.values()],families:prior.families.map(f=>({id:f.id,operation:f.operation,origin:f.origin,predecessor:f.predecessor,
    current_status:'STILL_PENDING_NO_PRODUCT_MUTANT_DISCHARGE_IN_P1335',evidence:'P1323–P1334 final reports declare A/B and no refinement seal/product mutation score. P1335 is audit only; its observer attacks do not execute this product mutant.',
    p1308_overlap:f.p1308_overlap,actual_source_mutants_executed_in_p1335:0})),
  p1308_separate_executed_families:prior.p1308_separate_executed_families,
  counts:{referenced_historical_families:prior.families.length,product_mutants_executed_in_p1335:0,discharge_receipts_in_reviewed_p1323_p1334_reports:0},
  policy:'Language divergence and missing adversarial certification are separate. No attack of copied auditor data discharges product debt; no absence of an attack proves a language error.',
  verdict:'NOT_SELF_APPROVED_ROLE_REVIEW_REQUIRED'};
if([...references.values()].some(r=>!r.unchanged))throw Error('Historical debt reference changed');
const path=b+'p1335-classification-certification-debt.json';if(fs.existsSync(path))throw Error('Refusing overwrite');
console.log('*** Begin Patch\n*** Add File: '+path+'\n'+JSON.stringify(data,null,2).split('\n').map(l=>'+'+l).join('\n')+'\n*** End Patch');
