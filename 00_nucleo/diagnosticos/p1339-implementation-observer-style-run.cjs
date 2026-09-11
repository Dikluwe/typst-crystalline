// Bounded engineering runner, not an independent verdict. Emits apply_patch only.
const fs=require('fs'),cp=require('child_process'),crypto=require('crypto'),z=require('zlib');
const root='/repos/Antigravity/typst-crystalline';
const label=process.argv[2];
if(!/^[a-z0-9-]+$/.test(label||''))throw Error('receipt label required');
process.chdir(root);
const sha=x=>crypto.createHash('sha256').update(x).digest('hex');
const pin=p=>({path:p,sha256:sha(fs.readFileSync(p))});
const git=args=>cp.spawnSync('git',args,{encoding:'utf8'}).stdout;
const encode=x=>({encoding:'gzip+base64 UTF-8, lossless',sha256:sha(x),data:z.gzipSync(Buffer.from(x)).toString('base64')});
function includePins(){
  const seen=new Set();
  function visit(p){
    if(seen.has(p))return;seen.add(p);
    if(!p.endsWith('.rs'))return;
    const text=fs.readFileSync(p,'utf8');
    for(const m of text.matchAll(/"\/\.\.\/(00_nucleo\/diagnosticos\/[^"\n]+)"/g))visit(m[1]);
    // The only other observer include is its own source used for real line provenance.
    for(const m of text.matchAll(/"(\/src\/[^"\n]+)"/g))visit('01_core'+m[1]);
  }
  visit('01_core/src/compiler/eval/mod.rs');
  for(const path of ['00_nucleo/diagnosticos/p1339-verifier-seal-F-style-supersession-r1.json','00_nucleo/diagnosticos/p1339-verifier-seal-F-style-supersession-r2.json','00_nucleo/diagnosticos/p1339-seal.json'])visit(path);
  for(const path of ['00_nucleo/diagnosticos/p1339-style-observation-owner-extension.md','00_nucleo/diagnosticos/p1339-style-observation-owner-extension-pins.json','00_nucleo/diagnosticos/p1339-verifier-style-r3-engineering-review-r1.json','01_core/src/entities/style_chain.rs','00_nucleo/prompts/entities/style_chain.md','00_nucleo/diagnosticos/p1339-implementation-observer-authority.json'])visit(path);
  return [...seen].sort().map(pin);
}
function snapshot(){
  const paths=git(['ls-files','--','01_core','02_shell','03_infra','04_wiring','Cargo.toml','Cargo.lock','.cargo']).trim().split('\n').filter(p=>p&&fs.existsSync(p)&&fs.statSync(p).isFile());
  const source=paths.map(pin);
  return {utc:new Date().toISOString(),head:git(['rev-parse','HEAD']).trim(),diff_stat:git(['diff','HEAD','--stat']),includes:includePins(),all_tracked_build_inputs:encode(JSON.stringify(source)),input_count:source.length};
}
const before=snapshot();
const delta={CARGO_TARGET_DIR:'/tmp/p1339-implementation-observer-instrumented.Y0Zc3f',RUSTFLAGS:'--cfg p1339_observation --check-cfg=cfg(p1339_observation)'};
const args=['test','-p','typst-core','--lib','--release','--locked','--offline','--no-run','--message-format','json-render-diagnostics'];
const start=Date.now();
const compile=cp.spawnSync('cargo',args,{cwd:root,env:{...process.env,...delta},encoding:'utf8',maxBuffer:128*1024*1024});
const afterCompile=snapshot();
const same=(a,b)=>JSON.stringify(a.includes)===JSON.stringify(b.includes)&&a.all_tracked_build_inputs.sha256===b.all_tracked_build_inputs.sha256;
const receipt={schema:'p1339-observer-style-engineering-v1',role:'implementer; no independent verification or final F credit',authority_manifest_sha256:'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',runner:pin('00_nucleo/diagnosticos/p1339-implementation-observer-style-run.cjs'),before,compile:{argv:['cargo',...args],cwd:root,environment_delta:delta,status:compile.status,signal:compile.signal,stdout:encode(compile.stdout||''),stderr:encode(compile.stderr||''),error:compile.error?.message??null},after_compile:afterCompile,compile_inputs_unchanged:same(before,afterCompile),runtime:null};
if(compile.status===0&&same(before,afterCompile)){
  const artifacts=compile.stdout.split('\n').filter(Boolean).flatMap(line=>{try{return [JSON.parse(line)]}catch{return []}}).filter(x=>x.reason==='compiler-artifact'&&x.executable&&x.profile.test&&x.target.name==='typst_core');
  const bins=[...new Set(artifacts.map(x=>x.executable))];
  if(bins.length!==1)throw Error('expected one actual core test executable, found '+bins.length);
  const binary=pin(bins[0]);
  const runtimeArgs=['compiler::eval::p1339_frozen_observation_binding::p1339_frozen_style_bound_matrix','--exact','--nocapture','--test-threads=1'];
  const runStart=new Date().toISOString();
  const run=cp.spawnSync(bins[0],runtimeArgs,{cwd:root,env:process.env,encoding:'utf8',maxBuffer:128*1024*1024});
  receipt.runtime={argv:[bins[0],...runtimeArgs],cwd:root,start_utc:runStart,end_utc:new Date().toISOString(),binary_before:binary,binary_after:pin(bins[0]),status:run.status,signal:run.signal,stdout:encode(run.stdout||''),stderr:encode(run.stderr||''),error:run.error?.message??null};
}
receipt.after=snapshot();receipt.all_inputs_unchanged=same(before,receipt.after);receipt.wall_ms=Date.now()-start;
const text=JSON.stringify(receipt,null,2)+'\n';
console.log('*** Begin Patch\n*** Add File: '+root+'/00_nucleo/diagnosticos/p1339-implementation-observer-'+label+'.json\n'+text.trimEnd().split('\n').map(x=>'+'+x).join('\n')+'\n*** End Patch');
