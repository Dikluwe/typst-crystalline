// Freeze historical final outputs without reading P1335 execution results.
const fs=require('fs'),crypto=require('crypto'),b='00_nucleo/diagnosticos/';
const read=n=>JSON.parse(fs.readFileSync(b+n));
const pin=n=>({path:b+n,sha256:crypto.createHash('sha256').update(fs.readFileSync(b+n)).digest('hex')});
const suite=read('p1335-sentinels-cases-r2.json'), byId=new Map(suite.cases.map(x=>[x.id,x]));
const aliases=new Map(suite.aliases.map(x=>[x.id,x.canonical]));
const rows=[], sources=[];
const triple=r=>({exit:r.exit,stdout:r.stdout,stderr:r.stderr});
function add(step,file,id,profile,expected,argv=null,expression=null,format=null,classification=null){
 const originalId=step+'.'+id,canonical=aliases.get(originalId)||originalId,c=byId.get(canonical);
 if(!c)throw Error('Missing current case '+originalId);
 const argFormat=argv&&argv.includes('--format')?argv[argv.indexOf('--format')+1]:format;
 const histExpr=expression||(argv&&argv.includes('--format')?argv[argv.indexOf('--format')+2]:null);
 const moved=!!(histExpr&&histExpr!==c.expression)||/p1334-ab-fixtures|p1327-ab-fixtures/.test(expected.stderr)||c.id.includes('cross-source')||(/p1327/.test(step)&&/import\s+"/.test(c.expression));
 if(!['exit','stdout','stderr'].every(k=>expected[k]!==undefined))throw Error('Incomplete channels '+originalId);
 rows.push({historical_id:originalId,canonical_id:canonical,profile,feature_flags:suite.profiles[profile],route:c.route,format:c.format,current_expression:c.expression,current_source_sha256:c.source_sha256,current_cwd:c.cwd,historical_expression:histExpr,historical_format:argFormat,historical_argv:argv,origin:pin(file),historical_classification:classification,expected_final_literal:triple(expected),needs_location_control:moved,comparison:moved?'PENDING_EXACT_LOCATION_CONTROL_NO_RAW_NORMALIZATION':'EXACT_EXIT_STDOUT_STDERR',interpretation:'Historical preservation only; parity assessed independently. Later authorized correction is a measured transition, not automatically regression.'});
}
for(const [step,file] of [['p1324','p1324-ab-candidate-v3.json'],['p1327','p1327-ab-cli-candidate-c2.json'],['p1327r2','p1327-ab-r2-candidate.json']]){
 sources.push(pin(file));for(const r of read(file).runs.filter(r=>r.order==='normal'))add(step,file,r.case||r.id,r.profile,r,r.argv,null,null,r.class||r.obligation?.status);
}
{
 const file='p1325-ab-candidate-cli.json';sources.push(pin(file));for(const r of read(file).rows.filter(r=>r.order==='normal'&&r.product==='candidate'))add('p1325',file,r.case,r.profile,r,r.argv,null,null,r.category);
}
{
 const file='p1326-ab-final.json';sources.push(pin(file));for(const r of read(file).records.filter(r=>r.order==='normal'))add('p1326',file,r.id,r.profile,r.candidate.output,r.candidate.argv,null,null,r.policy);
}
for(const [file,baseline] of [['p1334-ab-cli-expected.json','p1334-ab-cli-baseline.json'],['p1334-ab-cross-expected.json','p1334-ab-cross-baseline.json']]){
 sources.push(pin(file),pin(baseline));const x=read(baseline),rs=x.runs||x.rows||x.records||[];
 for(const r of read(file).expectations){const old=rs.find(z=>(z.case||z.id)===r.case&&z.profile===r.profile);add('p1334',file,r.case,r.profile,r.expected,old?.argv,null,'repr',r.classification);}
}
const keys=rows.map(x=>x.historical_id+'|'+x.profile);if(new Set(keys).size!==keys.length)throw Error('Duplicate expectation');
const result={schema:'p1335-classification-preservation-freeze-v1',at:new Date().toISOString(),role:'classification',not_verdict:true,input:pin('p1335-sentinels-cases-r2.json'),sources,policy:'Frozen before full P1335 sentinel results. Full final candidate observations P1324-P1327, frozen expected maps P1334. Historical outputs are not current vanilla parity. Moved source/fixtures require an explicit location control; no raw normalization. Profiles and format remain independent axes. Alias origins preserved. No P1335 result file read.',rows};
const out=b+'p1335-classification-preservation-freeze.json';if(fs.existsSync(out))throw Error('Refusing overwrite');
console.log('*** Begin Patch\n*** Add File: '+out+'\n'+JSON.stringify(result,null,2).split('\n').map(s=>'+'+s).join('\n')+'\n*** End Patch');
