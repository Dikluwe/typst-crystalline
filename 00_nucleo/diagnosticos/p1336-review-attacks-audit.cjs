const fs=require('fs'),crypto=require('crypto');
const root='/repos/Antigravity/typst-crystalline/',D=root+'00_nucleo/diagnosticos/';
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const read=p=>JSON.parse(fs.readFileSync(p));
const prepared=read(D+'p1336-attacks-prepared.json');
const expected={C:[],M1:['p1336_pure_lookup_names_and_supplied_span','p1336_int_ast_identifier_span'],M2:['p1336_pure_lookup_names_and_supplied_span','p1336_str_ast_identifier_span'],M3:['p1336_int_ast_identifier_span'],M4:['p1336_str_ast_identifier_span'],M5:['p1336_preserve_other_fallback_names_and_ast_span'],M6:['p1336_preserve_int_str_methods_and_type_namespaces']};
const witness={M1:['cannot access fields on type int','cannot access fields on type integer'],M2:['cannot access fields on type str','cannot access fields on type string'],M3:['Some(2..13)','Some(6..13)'],M4:['Some(2..12)','Some(5..12)'],M5:['Some(7..11)','Some(2..11)'],M6:['cannot access fields on type integer or string']};
const errors=[],rows=[];
for(const id of Object.keys(expected)){
  const p=D+'p1336-attacks-run-'+id+(['C','M1'].includes(id)?'':'-r2')+'.json';
  const j=read(p),s=prepared.mutants.find(m=>m.id===id),out=fs.readFileSync(j.stdout_path,'utf8'),err=fs.readFileSync(j.stderr_path,'utf8');
  const requireThat=(yes,why)=>{if(!yes)errors.push(id+': '+why);};
  requireThat(j.manifest_sha256===hash(D+'p1336-manifest.json'),'manifest identity');
  requireThat(hash(s.path)===s.source_sha256&&j.source_sha256===s.source_sha256,'source identity');
  for(const ch of ['stdout','stderr'])requireThat(hash(j[ch+'_path'])===j[ch+'_sha256'],ch+' integrity');
  const src=fs.readFileSync(s.path,'utf8'),a=src.indexOf('#[cfg(test)]\nmod p1336_tests {'),b=src.indexOf('#[cfg(test)]\nmod p1326_tests {',a);
  const th=crypto.createHash('sha256').update(src.slice(a,b)).digest('hex');
  requireThat(th===prepared.tests_sha256&&j.tests_sha256===th,'test integrity');
  requireThat(j.exit===(id==='C'?0:101)&&j.valid_execution,'execution');
  const failures=[...out.matchAll(/^test .*::(p1336_\w+) \.\.\. FAILED$/gm)].map(m=>m[1]).sort();
  requireThat(JSON.stringify(failures)===JSON.stringify([...expected[id]].sort()),'discriminatory failed-test set');
  if(id!=='C'){
    requireThat(err.includes('Compiling typst-core v0.1.0 ('+j.cwd+'/01_core)'),'recompilation');
    requireThat(witness[id].every(w=>err.includes(w)),'specific witness');
  }
  rows.push({id,receipt:p,receipt_sha256:hash(p),source_path:s.path,source_sha256:s.source_sha256,tests_sha256:th,at_start:j.at_start,at_end:j.at_end,exit:j.exit,failed_tests:failures,witness:witness[id]||[],verdict:id==='C'?'Preserved':'Violated',valid:true});
}
console.log(JSON.stringify({at:new Date().toISOString(),manifest_sha256:hash(D+'p1336-manifest.json'),prepared_sha256:hash(D+'p1336-attacks-prepared.json'),candidate_source_sha256:prepared.candidate_sha256,rows,valid_mutant_families:6,rejected_mutant_families:6,mutation_score:errors.length?null:1.0,invalid_superseded_executions:['M2 R0','M3 R0','M4 R0','M5 R0','M6 R0'],errors},null,2));
process.exitCode=errors.length?1:0;
