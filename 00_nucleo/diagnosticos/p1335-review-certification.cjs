const fs=require('fs'),crypto=require('crypto');
const base='00_nucleo/diagnosticos/',inputs={},errors=[];
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
function read(p){inputs[p]=sha(p);return JSON.parse(fs.readFileSync(p));}
const current=read(base+'p1335-classification-certification-debt.json');
const prior=read(base+'p1322-classification-certification-debt.json');
const ids=new Set(current.families.map(x=>x.id));
if(ids.size!==current.families.length || current.families.length!==prior.families.length)errors.push('FAMILY_COVERAGE');
for(const old of prior.families){
  const x=current.families.find(x=>x.id===old.id);
  if(!x){errors.push('MISSING_FAMILY:'+old.id);continue;}
  for(const k of ['operation','origin','predecessor','p1308_overlap'])if(JSON.stringify(x[k])!==JSON.stringify(old[k]))errors.push('FAMILY_CHANGED:'+old.id+':'+k);
  if(x.actual_source_mutants_executed_in_p1335!==0 || x.current_status!=='STILL_PENDING_NO_PRODUCT_MUTANT_DISCHARGE_IN_P1335')errors.push('DEBT_UNJUSTIFIABLY_DISCHARGED:'+old.id);
}
for(const p of current.references_checked){
  if(!p.path.startsWith(base))throw Error('unexpected reference scope');
  inputs[p.path]=sha(p.path);
  if(p.actual_sha256!==inputs[p.path] || p.expected_sha256!==inputs[p.path] || p.unchanged!==true)errors.push('REFERENCE_PIN:'+p.path);
}
const reportAnchors=[];
for(const p of current.reports_inspected){
  inputs[p.path]=sha(p.path);
  if(inputs[p.path]!==p.sha256)errors.push('REPORT_PIN:'+p.path);
  const lines=fs.readFileSync(p.path,'utf8').split('\n');
  const anchors=lines.flatMap((text,i)=>/sem.{0,20}selo|sem.{0,20}atest|não.{0,20}mutantes|sem.{0,20}mutation/.test(text)?[{line:i+1,text}]:[]);
  if(!anchors.length)errors.push('REPORT_REGIME_UNCLEAR:'+p.path);
  reportAnchors.push({path:p.path,anchors});
}
if(current.counts.referenced_historical_families!==ids.size || current.counts.product_mutants_executed_in_p1335!==0 || current.counts.discharge_receipts_in_reviewed_p1323_p1334_reports!==0)errors.push('COUNTS');
if(JSON.stringify(current.p1308_separate_executed_families)!==JSON.stringify(prior.p1308_separate_executed_families))errors.push('P1308_SEPARATION');
console.log(JSON.stringify({at:new Date().toISOString(),role:'D',manifest_sha256:sha(base+'p1335-manifest.json'),checker_sha256:sha(__filename),inputs,verdict:errors.length?'Violated':'Preserved',violations:errors,reportAnchors,counts:{inherited_families:ids.size,current_product_mutants:0},scope:'Revalidated identities and explicit continued debt; not a current product mutation score or a proof of product semantics.'},null,2));
