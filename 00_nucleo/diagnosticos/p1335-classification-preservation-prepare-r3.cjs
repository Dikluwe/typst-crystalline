// Historical argv source position varies with feature flags; definitions are canonical.
const fs=require('fs'),crypto=require('crypto'),b='00_nucleo/diagnosticos/';
const read=n=>JSON.parse(fs.readFileSync(b+n));
const pin=n=>({path:b+n,sha256:crypto.createHash('sha256').update(fs.readFileSync(b+n)).digest('hex')});
const x=read('p1335-classification-preservation-freeze-r2.json'),defs=new Map(read('p1335-classification-required-sentinels.json').cases.map(c=>[c.id,c]));
x.predecessor=pin('p1335-classification-preservation-freeze-r2.json');x.at=new Date().toISOString();x.schema='p1335-classification-preservation-freeze-v3';
x.revision='Correct feature-profile historical expression metadata from frozen public case definitions: --features occurs after --format value and before expression. No final expected channel changed; no P1335 full sentinel result read.';
for(const row of x.rows.filter(r=>!r.historical_id.startsWith('p1334.'))){
 const c=defs.get(row.historical_id);if(!c)throw Error(row.historical_id);
 if(row.historical_argv.at(-1)!==c.expression)throw Error('Historical argv definition mismatch '+row.historical_id);
 row.historical_expression=c.expression;row.historical_source_origin=c.origin;
 row.needs_location_control=c.expression!==row.current_expression||/p1334-ab-fixtures|p1327-ab-fixtures/.test(row.expected_final_literal.stderr)||(/p1327/.test(row.historical_id)&&/import\s+"/.test(c.expression));
 row.comparison=row.needs_location_control?'PENDING_EXACT_LOCATION_CONTROL_NO_RAW_NORMALIZATION':'EXACT_EXIT_STDOUT_STDERR';
}
const out=b+'p1335-classification-preservation-freeze-r3.json';if(fs.existsSync(out))throw Error('No overwrite');
console.log('*** Begin Patch\n*** Add File: '+out+'\n'+JSON.stringify(x,null,2).split('\n').map(s=>'+'+s).join('\n')+'\n*** End Patch');
