// Owner-local engineering tests. No oracle edits or independent verdict.
const fs=require('fs'),cp=require('child_process'),c=require('crypto'),z=require('zlib');
const label=process.argv[2];if(!/^[a-z0-9-]+$/.test(label||''))throw Error('label required');
const root='/repos/Antigravity/typst-crystalline';process.chdir(root);
const sha=x=>c.createHash('sha256').update(x).digest('hex');
const pin=p=>({path:p,sha256:sha(fs.readFileSync(p))});
const encode=x=>({encoding:'gzip+base64 UTF-8 lossless',sha256:sha(x),data:z.gzipSync(Buffer.from(x)).toString('base64')});
const git=args=>cp.spawnSync('git',args,{encoding:'utf8'}).stdout;
function snapshot(){
 const sources=cp.spawnSync('rg',['--files','01_core','-g','*.rs','-g','Cargo.toml'],{encoding:'utf8'}).stdout.trim().split('\n');
 const paths=new Set([...sources,'Cargo.toml','Cargo.lock','00_nucleo/diagnosticos/p1340-verifier-seal-r3.json','00_nucleo/diagnosticos/p1340-test-binding-authority.json','00_nucleo/diagnosticos/p1340-test-binding-delegation.json','00_nucleo/diagnosticos/p1340-contract-correction-r3.json','00_nucleo/diagnosticos/p1340-contract-correction-predicate-r3.py','00_nucleo/prompts/compiler/eval.md','00_nucleo/prompts/entities/style_chain.md']);
 const seen=new Set();
 function visit(path){if(seen.has(path))return;seen.add(path);paths.add(path);if(!path.endsWith('.rs'))return;const text=fs.readFileSync(path,'utf8');for(const m of text.matchAll(/"\/\.\.\/(00_nucleo\/diagnosticos\/[^"\n]+)"/g))visit(m[1]);}
 visit('01_core/src/compiler/eval/mod.rs');
 const pins=[...paths].sort().map(pin);
 return {utc:new Date().toISOString(),head:git(['rev-parse','HEAD']).trim(),diff_stat:git(['diff','HEAD','--stat']),inputs:encode(JSON.stringify(pins)),input_count:pins.length,owners:[pin('01_core/src/compiler/eval/mod.rs'),pin('01_core/src/entities/style_chain.rs')]};
}
const before=snapshot();
const envDelta={CARGO_TARGET_DIR:'/tmp/p1339-implementation-observer-instrumented.Y0Zc3f',RUSTFLAGS:'--cfg p1339_observation --check-cfg=cfg(p1339_observation)'};
const args=['test','-p','typst-core','--lib','--release','--locked','--offline','-j','2','--message-format','json-render-diagnostics','p1340_binding_actual_relation_prefix_and_lossless_payloads','--','--nocapture'];
const start=Date.now();
const result=cp.spawnSync('cargo',args,{env:{...process.env,...envDelta},encoding:'utf8',maxBuffer:128*1024*1024});
const after=snapshot();
const artifacts=(result.stdout||'').split('\n').flatMap(line=>{try{return[JSON.parse(line)]}catch{return[]}}).filter(x=>x.reason==='compiler-artifact'&&x.executable&&x.profile.test&&x.target.name==='typst_core').map(x=>pin(x.executable));
const inverse=cp.spawnSync('node',['00_nucleo/diagnosticos/p1340-implementation-l1-bindings-inverse-check.cjs'],{encoding:'utf8'});
const receipt={schema:'p1340-l1-binding-engineering-run-v1',role:'implementation; no independent F verdict',manifest_sha256:'4cca936b0b50739a097a8299ae7eddfa328b864a19200525f12176207b1377bf',seal:pin('00_nucleo/diagnosticos/p1340-verifier-seal-r3.json'),runner:pin('00_nucleo/diagnosticos/p1340-implementation-l1-bindings-run.cjs'),argv:['cargo',...args],cwd:root,environment_delta:envDelta,before,after,wall_ms:Date.now()-start,status:result.status,signal:result.signal,stdout:encode(result.stdout||''),stderr:encode(result.stderr||''),error:result.error?.message??null,inputs_unchanged:before.inputs.sha256===after.inputs.sha256,actual_artifacts_after:artifacts,inverse_check:{argv:['node','00_nucleo/diagnosticos/p1340-implementation-l1-bindings-inverse-check.cjs'],status:inverse.status,stdout:inverse.stdout,stderr:inverse.stderr},limits:'Binary SHA collected after cargo test, not a fabricated before-runtime pin. Inputs include every current core Rust source, manifests and transitive observer includes; not external compiler/cache attestation.'};
const text=JSON.stringify(receipt,null,2);
console.log('*** Begin Patch\n*** Add File: '+root+'/00_nucleo/diagnosticos/p1340-implementation-l1-bindings-'+label+'.json\n'+text.split('\n').map(x=>'+'+x).join('\n')+'\n*** End Patch');
