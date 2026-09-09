// Concise view of fresh supplemental gaps. Read-only; no classification verdict.
const fs=require('fs'),b='00_nucleo/diagnosticos/';
const read=n=>JSON.parse(fs.readFileSync(b+n));
const name=process.argv[2]||'p1335-sentinels-normal-r3.json',data=read(name),rows=data.rows||data.extra||[];
const groups=new Map();for(const r of rows){if(!groups.has(r.id))groups.set(r.id,[]);groups.get(r.id).push(r);}
console.log(JSON.stringify({source:name,counts:data.counts,open:[...groups].filter(([id,rs])=>rs.some(r=>!r.runtime_class.startsWith('MATCH')&&r.language_projection!=='Preserved')).map(([id,rs])=>({id,expression:rs[0].expression,profiles:rs.map(r=>r.profile+':'+r.runtime_class+':'+(r.language_projection||'')),vanilla:{exit:rs[0].vanilla.exit_code,stdout:rs[0].vanilla.stdout,stderr:rs[0].vanilla.stderr},crystalline:{exit:rs[0].crystalline.exit_code,stdout:rs[0].crystalline.stdout,stderr:rs[0].crystalline.stderr}}))},null,2));
