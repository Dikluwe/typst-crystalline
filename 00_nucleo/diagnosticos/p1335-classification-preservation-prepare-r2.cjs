// Metadata-only successor: final expected channels are unchanged.
const fs=require('fs'),crypto=require('crypto'),b='00_nucleo/diagnosticos/';
const read=n=>JSON.parse(fs.readFileSync(b+n));
const pin=n=>({path:b+n,sha256:crypto.createHash('sha256').update(fs.readFileSync(b+n)).digest('hex')});
const x=read('p1335-classification-preservation-freeze.json');
x.predecessor=pin('p1335-classification-preservation-freeze.json');x.at=new Date().toISOString();x.schema='p1335-classification-preservation-freeze-v2';
x.revision='P1334 source/argv populated from baseline cases.results.BASE; frozen final expectation bytes unchanged; no P1335 full sentinel results read.';
const src=['p1334-ab-cli-baseline.json','p1334-ab-cross-baseline.json'].flatMap(file=>read(file).cases.map(c=>({...c,file})));
for(const row of x.rows.filter(r=>r.historical_id.startsWith('p1334.'))){
 const old=src.find(r=>'p1334.'+r.case===row.historical_id&&r.profile===row.profile);if(!old)throw Error(row.historical_id);
 row.historical_expression=old.expression;row.historical_argv=old.results.BASE.argv;row.historical_source_origin=pin(old.file);
 row.needs_location_control=old.expression!==row.current_expression||row.needs_location_control;
 row.comparison=row.needs_location_control?'PENDING_EXACT_LOCATION_CONTROL_NO_RAW_NORMALIZATION':'EXACT_EXIT_STDOUT_STDERR';
}
const out=b+'p1335-classification-preservation-freeze-r2.json';if(fs.existsSync(out))throw Error('No overwrite');
console.log('*** Begin Patch\n*** Add File: '+out+'\n'+JSON.stringify(x,null,2).split('\n').map(s=>'+'+s).join('\n')+'\n*** End Patch');
