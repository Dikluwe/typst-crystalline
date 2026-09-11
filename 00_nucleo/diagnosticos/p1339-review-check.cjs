// Read-only audit of canonical P1339 receipts. Prints a receipt for apply_patch.
const fs = require('fs');
const crypto = require('crypto');
const root = '/repos/Antigravity/typst-crystalline';
const d = '00_nucleo/diagnosticos/';
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const read = n => JSON.parse(fs.readFileSync(d + 'p1339-' + n + '.json'));
const inputs = [
  '/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md',
  '/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md',
  '/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md',
  '00_nucleo/materialization/typst-passo-1339.md',
  ...['0107-paridade-linguagem-nao-mecanica','0108-disciplina-anti-deriva','0127-gate-l0-paragem-vs-fluxo','0129-nucleos-tekt-l0-compartilhado'].map(n=>'00_nucleo/adr/typst-adr-'+n+'.md'),
  ...['a0','authority-manifest','probe-manifest','vanilla-runs','crystalline-before-runs','comparison-before'].map(n=>d+'p1339-'+n+'.json'),
  d+'p1339-probe.py', d+'p1339-a0-recorder.py',
  ...['layout/angle','foundations/float','foundations/func','foundations/version'].map(n=>'lab/typst-original/crates/typst-library/src/'+n+'.rs'),
];
const a=read('a0'), m=read('probe-manifest'), v=read('vanilla-runs'), c=read('crystalline-before-runs'), b=read('comparison-before'), authority=read('authority-manifest');
const issues=[];
const check=(ok,reason)=>{if(!ok) issues.push(reason);};
const key=r=>JSON.stringify([r.id,r.order,r.profile]);
const obs=r=>JSON.stringify([r.execution,r.exit,r.stdout,r.stderr]);
const counts={Preserved:0,Violated:0,Unknown:0};
const byCandidate=new Map(c.rows.map(r=>[key(r),r]));
const byComparison=new Map(b.comparisons.map(r=>[key(r),r]));
const instability=[];
check(a.script_sha256===sha(d+'p1339-a0-recorder.py'),'A0 recorder hash');
check(m.runner_sha256===sha(d+'p1339-probe.py'),'probe runner hash');
check(authority.predecessor.sha256===sha(d+'p1339-a0.json'),'authority predecessor');
check(authority.step.sha256===sha(authority.step.path),'authority step');
for(const [name,h] of Object.entries(b.inputs)) check(h===sha(d+'p1339-'+name+'.json'),'comparison input '+name);
for(const [name,r] of [['vanilla',v],['crystalline',c]]) {
  check(r.a0_sha256===sha(d+'p1339-a0.json'),name+' A0 hash');
  check(r.authority_manifest_sha256===sha(d+'p1339-authority-manifest.json'),name+' authority hash');
  check(r.probe_manifest_sha256===sha(d+'p1339-probe-manifest.json'),name+' manifest hash');
  check(r.runner_sha256===sha(d+'p1339-probe.py'),name+' runner hash');
  check(r.binary.sha256===sha(r.binary.path),name+' binary hash');
  check(r.rows.length===m.cases.length*Object.keys(m.profiles).length*m.orders.length,name+' row coverage');
  check(new Set(r.rows.map(key)).size===r.rows.length,name+' duplicate keys');
  const rows=new Map(r.rows.map(x=>[key(x),x]));
  for(const case_ of m.cases) for(const profile of Object.keys(m.profiles)) {
    const normal=rows.get(key({id:case_.id,order:'normal',profile}));
    const reverse=rows.get(key({id:case_.id,order:'reverse',profile}));
    check(Boolean(normal&&reverse),name+' missing '+case_.id);
    if(obs(normal)!==obs(reverse)) instability.push([name,case_.id,profile]);
    for(const row of [normal,reverse]) check(JSON.stringify(row.argv)===JSON.stringify([r.binary.path,'eval','--format','json',...m.profiles[profile],case_.expression]),name+' argv '+case_.id);
  }
}
for(const r of v.rows) {
  const other=byCandidate.get(key(r));
  const classification=r.execution==='Unknown'||other.execution==='Unknown'?'Unknown':obs(r)===obs(other)?'Preserved':'Violated';
  counts[classification]++;
  check(byComparison.get(key(r)).classification===classification,'comparison '+key(r));
}
check(JSON.stringify(counts)===JSON.stringify(b.counts),'reported counts');
check(instability.length===b.instability.length,'reported instability');
check(v.end<=c.start,'vanilla before crystalline');
check(a.end<=authority.at&&authority.at<=m.at&&m.at<=v.start,'causal timestamps');
const receipt={
  at:new Date().toISOString(),executor:'/root/p1339_preflight_review',regime:'executado sem atestação de isolamento',
  context:'fresh task only; shared filesystem; no candidate supplied or read',
  head:a.head,
  git_provenance:'HEAD and empty tracked diff independently checked by shell at 2026-09-09T21:55:25Z; initial full status in A0; new diagnostic files are untracked.',
  inputs:Object.fromEntries(inputs.map(p=>[p,sha(p)])),
  binary_hashes:Object.fromEntries([v,c].map(r=>[r.binary.path,sha(r.binary.path)])),
  observations:{cases:m.cases.length,rows_per_binary:v.rows.length,comparisons:b.comparisons.length,counts,instability,issues,vanilla_start:v.start,vanilla_end:v.end,crystalline_start:c.start,crystalline_end:c.end},
  limitations:['Initial skill, step and authority were read before first hash snapshot; no claim of pre-read freeze for these inputs.','A0 product inventory and antecedent claims inspected as receipt, not independently recertified against P1338.','Sources hashed locally; upstream a51e02804 identity inherited from repository authority, not independently rebuilt.','No source writes, L0, contract, oracles, candidate implementation or probe reruns by reviewer.','This verifies receipt coherence, not full A.1 or product equivalence.'],
  checker_sha256:sha(__filename),
};
console.log(JSON.stringify(receipt,null,2));
