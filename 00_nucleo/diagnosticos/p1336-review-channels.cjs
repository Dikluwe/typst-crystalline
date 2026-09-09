const fs=require('fs'), crypto=require('crypto');
const D='/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/';
const digest=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const read=n=>JSON.parse(fs.readFileSync(D+'p1336-tests-'+n+'.json'));
const cases=read('cases'), baseline=read('baseline'), candidate=read('candidate'), freeze=read('freeze'), expectations=read('expectations');
const sig=r=>JSON.stringify([r.exit,r.stdout_base64,r.stderr_base64]);
const errors=[];
const requireThat=(yes,why)=>{if(!yes)errors.push(why);};
for(const [name,h]of Object.entries(freeze.protected))requireThat(digest(D+name)===h,'freeze '+name);
requireThat(digest(D+'p1336-tests-baseline.json')===freeze.baseline_runs_sha256,'baseline hash');
requireThat(digest(D+'p1336-tests-expectations.json')===freeze.expectations_sha256,'expectations hash');
const expected=new Map(expectations.map(e=>[e.id+'|'+e.profile,e]));
const counts={};
for(const [phase,j]of [['baseline',baseline],['candidate',candidate]]){
  const seen=new Set(),normal=new Map(j.rows.filter(r=>r.order==='normal').map(r=>[[r.id,r.profile,r.product].join('|'),sig(r)]));
  for(const r of j.rows){
    const k=[r.id,r.profile,r.product,r.order].join('|');requireThat(!seen.has(k),'duplicate '+k);seen.add(k);
    requireThat(r.observation==='Observed'&&!r.failure,'unknown '+k);
    const c=cases.find(c=>c.id===r.id);requireThat(!!c&&c.expr===r.source,'source '+k);
    for(const ch of ['stdout','stderr'])requireThat(Buffer.from(r[ch+'_base64'],'base64').toString('utf8')===r[ch],'channel '+k+ch);
    requireThat(normal.get([r.id,r.profile,r.product].join('|'))===sig(r),'order '+k);
    requireThat(j.products[r.product]?.sha256===r.binary_sha256,'binary '+k);
    if(phase==='candidate'){
      const e=expected.get(r.id+'|'+r.profile);requireThat(e&&sig(r)===sig(e.expected),'candidate expectation '+k);
      requireThat(e&&sig(e.expected)===sig(e.policy==='converge'?e.vanilla:e.baseline),'policy '+k);
    }
    const ck=[phase,r.profile,r.order].join('|');counts[ck]=(counts[ck]||0)+1;
  }
  for(const c of cases)for(const p of ['default','html','a11y','html+a11y'])for(const o of ['normal','repeat','reverse'])for(const product of Object.keys(j.products))requireThat(seen.has([c.id,p,product,o].join('|')),'missing tuple '+[phase,c.id,p,product,o].join('|'));
}
console.log(JSON.stringify({at:new Date().toISOString(),manifest_sha256:digest(D+'p1336-manifest.json'),freeze_sha256:digest(D+'p1336-tests-freeze.json'),baseline_sha256:digest(D+'p1336-tests-baseline.json'),candidate_sha256:digest(D+'p1336-tests-candidate.json'),case_count:cases.length,counts,errors},null,2));
process.exitCode=errors.length?1:0;
