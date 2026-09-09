// Read historical public data only; print an immutable apply_patch successor.
const fs = require('fs'), crypto = require('crypto');
const b = '00_nucleo/diagnosticos/';
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const pin = n => ({path:b+n,sha256:sha(b+n)});
const read = n => JSON.parse(fs.readFileSync(b+n,'utf8'));
const profiles=['default','html','a11y','html+a11y'];
const cases=[];
function add(step,id,expression,category,source,format='json',fixtures={}) {
  cases.push({id:`${step}.${id}`,historical_step:step,historical_id:id,expression,route:'eval',format,profiles,
    origin:pin(source),historical_category:category,fixtures,
    mandatory_channels:['exit','stdout','stderr'],historical_expectation_is_not_current_parity:true});
}
const p24='p1324-ab-baseline-v3.json';
for(const r of read(p24).runs.filter(r=>r.profile==='default'))
  add('p1324',r.case,r.argv[r.argv.indexOf('--format')+2],'scope-and-boundary',p24);
const p25='p1325-ab-candidate-cli.json';
for(const [id,category,expression] of read(p25).cases)add('p1325',id,expression,category,p25);
const p26='p1326-ab-baseline.json';
for(const [id,expression,category] of read(p26).cases)add('p1326',id,expression,category,p26);
const p27='p1327-ab-cli-candidate-c2.json',x27=read(p27);
for(const [id,expression,category] of x27.cases)add('p1327',id,expression,category,p27,'json',x27.fixtures);
const p27r2='p1327-ab-r2-candidate.json';
for(const [id,expression,format,category] of read(p27r2).cases)add('p1327r2',id,expression,category,p27r2,format);
// These public cases close gaps outside the P1334 cumulative abs corpus.
const payload={schema:'p1335-classification-required-sentinels-v1',at:new Date().toISOString(),executor:'/root/p1335_classifier',
  policy:'All cases supplement only. Deduplicate on exact expression, format, fixture bytes, route and effective profile while retaining every historical origin. Copy fixtures under p1335 namespace; compare current bilateral full channels. Historical output at moved paths requires location control, never broad normalization.',
  cases,
  html_required:{source:pin('p1323-ab-frozen-suite-r2.json'),covered_by_root_transversal:true,
    requirement:'Compile/legacy HTML enabled and disabled; error+warning ordering; three hints and paragraph termination; default/pdf/svg/png/query/eval no HTML warning. Preserve raw channels alongside DOM/PDF projections.'},
  historical_final_sources:['p1324-ab-candidate-v3.json','p1325-ab-candidate-cli.json','p1326-ab-final.json',p27,p27r2].map(pin),
  status:'FROZEN_REQUIREMENT_BEFORE_P1335_CURRENT_RESULTS_NOT_A_VERDICT'};
const history=[];
for(let n=1323;n<=1334;n++){
 const closure=read(`p${n}-closure.json`),report=`p${n}-final-report.md`;
 if(closure.report_sha256!==sha(b+report))throw Error('Historical report pin mismatch '+n);
 history.push({step:`P${n}`,report:pin(report),closure:pin(`p${n}-closure.json`),historical_at:closure.at,
   historical_head:closure.state.head,manifest_sha256:closure.manifest_sha256,binary_sha256:closure.binary_sha256||null,
   previous_closure_sha256:closure.previous_closure_sha256||null,limits:closure.limits,current_decision:'PENDING_FRESH_SENTINELS'});
}
const ancestorNames=['p1322-o-que-falta-para-paridade.md','p1322-probe-catalog.json','p1322-classification-owner-ledger.tsv','p1322-classification-selection-r2.json','p1322-classification-final.cjs','p1322-classification-selection-r2.cjs','p1322-classification-query-hint-addendum-r2.md','p1322-classification-certification-debt.json','p1322-closure.json'];
const inputs={schema:'p1335-classification-historical-inputs-v1',at:new Date().toISOString(),historical_only:true,ancestors:ancestorNames.map(pin),history};
let patch='*** Begin Patch\n';
for(const [name,data] of Object.entries({'required-sentinels.json':payload,'historical-inputs.json':inputs})){
 const path=b+'p1335-classification-'+name;if(fs.existsSync(path))throw Error('Refusing overwrite '+path);
 patch+='*** Add File: '+path+'\n'+JSON.stringify(data,null,2).split('\n').map(x=>'+'+x).join('\n')+'\n';
}
console.log(patch+'*** End Patch');
