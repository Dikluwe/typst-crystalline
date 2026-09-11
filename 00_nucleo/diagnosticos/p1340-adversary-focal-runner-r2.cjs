// Executes the real pinned baseline harness; records evidence, never a verdict.
const fs = require('fs');
const crypto = require('crypto');
const cp = require('child_process');
const mode = process.argv[2];
if (!['control', 'NM01', 'NM02', 'control-repeat'].includes(mode)) throw Error('explicit frozen mode required');
const root = '/repos/Antigravity/typst-crystalline';
const cwd = '/tmp/p1340-adversary-baseline.IqcW7q';
const target = '/tmp/p1340-adversary-target.DvNoJ7';
const sha = x => crypto.createHash('sha256').update(x).digest('hex');
const fileSha = x => sha(fs.readFileSync(x));
const pins = {
  '00_nucleo/diagnosticos/p1340-adversary-focal-inputs-r2.json':'d4c5dfc90fdef655fc14c873c502acfeb85e5d89506751fdc04d1c4cb2fbcbca',
  '00_nucleo/diagnosticos/p1340-adversary-focal-harness-r2.rs':'40560192cf55bbde4dacd20f49c2d13f4ed51a75993ad52c3bde5553402525a1',
  '00_nucleo/diagnosticos/p1339-mutant-closed-state-api-harness.rs':'09c467410cb0f4e37d11dca47c15f4913271f74e7b1400457513d76cd9c373c0',
  '00_nucleo/diagnosticos/p1339-ab-closed-api-fixtures-r1.json':'dda8618c9ede3640fab05d6cd4278f6c766c3410e4a646561f4bd32436f20dd5'
};
for (const [path, expected] of Object.entries(pins)) {
  if(fileSha(root+'/'+path)!==expected || fileSha(cwd+'/'+path)!==expected) throw Error('input drift '+path);
}
const args=['test','-p','typst-core','p1340_adversary_focal_actual_apis','--lib','--locked','--offline','--message-format=short','--','--nocapture'];
const delta={CARGO_TARGET_DIR:target,CARGO_PROFILE_DEV_DEBUG:'0',CARGO_PROFILE_TEST_DEBUG:'0'};
if(process.env.RUSTFLAGS) throw Error('unexpected RUSTFLAGS');
const sourcePath='01_core/src/compiler/eval/mod.rs';
const sourceBefore=fileSha(cwd+'/'+sourcePath);
const source=fs.readFileSync(cwd+'/'+sourcePath,'utf8');
const nm01=source.includes('/* P1340_NM01 */ ObservationRelation::Same');
const nm02=source.includes('/* P1340_NM02 */ ctx.introspector = self.introspector.clone();');
if(nm01!==(mode==='NM01') || nm02!==(mode==='NM02')) throw Error('requested mode does not match actual mutation markers');
const start=new Date().toISOString(), clock=Date.now();
const result=cp.spawnSync('cargo',args,{cwd,env:{...process.env,...delta},encoding:'utf8',maxBuffer:128*1024*1024});
const rows=(result.stdout||'').split('\n').filter(x=>x.startsWith('P1340_ADVERSARY ')).map(x=>JSON.parse(x.slice('P1340_ADVERSARY '.length)));
const receipt={schema:'p1340-adversary-raw-run-r2',executor:'/root/p1340_adversary',mode,baseline_sha256:'0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a',baseline_head:'2f42d64253547734564513a1159ee6b584c1c4b4',baseline_state:'working tree recorded by baseline before.diff_stat and frozen patch',cwd,target,argv:['cargo',...args],environment_delta:delta,input_pins:pins,source_path:sourcePath,source_sha256:sourceBefore,source_unchanged:sourceBefore===fileSha(cwd+'/'+sourcePath),cargo_toml_sha256:fileSha(cwd+'/Cargo.toml'),cargo_lock_sha256:fileSha(cwd+'/Cargo.lock'),runner_sha256:fileSha(__filename),start,end:new Date().toISOString(),wall_ms:Date.now()-clock,status:result.status,signal:result.signal,error:result.error?.message??null,stdout:result.stdout,stderr:result.stderr,rows,execution_scope:'Existing L1 operations only; zero L3 terminal execution credit; no verdict or mutation score'};
const text=JSON.stringify(receipt,null,2)+'\n';
console.log('*** Begin Patch\n*** Add File: '+root+'/00_nucleo/diagnosticos/p1340-adversary-focal-'+mode.toLowerCase()+'-raw-r2.json\n'+text.split('\n').map(x=>'+'+x).join('\n')+'\n*** End Patch');
