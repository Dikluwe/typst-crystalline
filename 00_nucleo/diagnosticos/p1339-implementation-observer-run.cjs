// Engineering runner: capture streams and provenance, emit an apply_patch
// receipt. It does not edit product, protected tests, or generate a verdict.
const fs = require('fs');
const crypto = require('crypto');
const cp = require('child_process');
const zlib = require('zlib');
const mode = process.argv[2];
const label = process.argv[3];
if (!['local', 'instrumented-compile'].includes(mode) || !/^[a-z0-9-]+$/.test(label || '')) throw new Error('explicit mode and safe receipt label required');
const root = '/repos/Antigravity/typst-crystalline';
const git = args => cp.spawnSync('git',args,{cwd:root,encoding:'utf8'}).stdout;
const sha = path => crypto.createHash('sha256').update(fs.readFileSync(path)).digest('hex');
const sourceFiles = () => git(['diff','HEAD','--name-only','--','01_core','03_infra']).trim().split('\n').filter(Boolean).filter(p=>fs.existsSync(p));
const pins = () => sourceFiles().map(path=>({path,sha256:sha(path)}));
const before = {utc:new Date().toISOString(),head:git(['rev-parse','HEAD']).trim(),diff_stat:git(['diff','HEAD','--stat']),source_pins:pins()};
const args = ['test','-p','typst-core'];
const env={...process.env};
const delta={};
if(mode==='local') {
  args.push('p1339_observer_local_tests','--lib','--release','--locked','--offline','--message-format','short');
  env.CARGO_TARGET_DIR=delta.CARGO_TARGET_DIR='/tmp/p1339-target.UD8gh7';
  if(env.RUSTFLAGS)throw new Error('ordinary engineering check requires absent RUSTFLAGS');
} else {
  args.push('--lib','--release','--locked','--offline','--no-run','--message-format','short');
  env.CARGO_TARGET_DIR=delta.CARGO_TARGET_DIR='/tmp/p1339-implementation-observer-instrumented.Y0Zc3f';
  env.RUSTFLAGS=delta.RUSTFLAGS='--cfg p1339_observation --check-cfg=cfg(p1339_observation)';
}
const start=Date.now();
const result=cp.spawnSync('cargo',args,{cwd:root,env,encoding:'utf8',maxBuffer:128*1024*1024});
const after={utc:new Date().toISOString(),source_pins:pins()};
const receipt={schema:'p1339-observer-engineering-run-v1',role:'implementation, not independent verifier',mode,argv:['cargo',...args],cwd:root,environment_delta:delta,before,after,wall_ms:Date.now()-start,status:result.status,signal:result.signal,error:result.error?.message??null,stdout:result.stdout,stderr_encoding:'gzip+base64 UTF-8, lossless',stderr_gzip_base64:zlib.gzipSync(Buffer.from(result.stderr,'utf8')).toString('base64'),stderr_sha256:crypto.createHash('sha256').update(result.stderr).digest('hex'),source_pins_unchanged:JSON.stringify(before.source_pins)===JSON.stringify(after.source_pins),runtime_credit:'Engineering check only. No phase-F/verdict credit.'};
const text=JSON.stringify(receipt,null,2)+'\n';
console.log('*** Begin Patch\n*** Add File: '+root+'/00_nucleo/diagnosticos/p1339-implementation-observer-'+label+'.json\n'+text.split('\n').map(line=>'+'+line).join('\n')+'\n*** End Patch');
