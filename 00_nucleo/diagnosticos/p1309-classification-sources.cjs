// Role C. Read source and lineage only; emit a preparation receipt for apply_patch.
const fs=require('fs'), crypto=require('crypto');
const read=p=>fs.readFileSync(p,'utf8');
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const base='00_nucleo/diagnosticos/';
const lines=read(base+'p1304-owner-ledger.tsv').trimEnd().split('\n');
const keys=lines.shift().split('\t');
const historical=lines.map(l=>Object.fromEntries(l.split('\t').map((x,i)=>[keys[i],x])));
const extraPrompts=['compiler/stdlib/structural','compiler/stdlib/structural/markup','compiler/stdlib/emoji','compiler/stdlib/sys','entities/version','entities/symbol','compiler/eval/modules','compiler/eval/call_dispatch','compiler/eval/closures'].map(p=>'00_nucleo/prompts/'+p+'.md');
const promptNames=[...new Set([...historical.map(r=>r.owner_prompt),...extraPrompts])].sort();
function walk(p){return fs.readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+'/'+e.name):[p+'/'+e.name]);}
const rust=['01_core','02_shell','03_infra','04_wiring'].flatMap(walk).filter(p=>p.endsWith('.rs'));
const ownership=new Map();
for(const p of rust){const matches=[...read(p).matchAll(/^\s*\/\/[!/]?\s*@prompt\s+(\S+)\s*$/gm)];for(const m of matches){const a=ownership.get(m[1])||[];a.push(p);ownership.set(m[1],a);}}
const prompts=promptNames.map(path=>{const text=read(path),consumers=ownership.get(path)||[];return {path,sha256:sha(path),read_mode:'FULL_TEXT_READ_BY_ROLE_C',line_count:text.split('\n').length-1,declared_code_hash:text.match(/Hash do Código:\s*(\S+)/)?.[1]||null,consumers,ownership_1_to_1:consumers.length===1,consumer_sha256:consumers.length===1?sha(consumers[0]):null,consumer_prompt_hash:consumers.length===1?read(consumers[0]).match(/@prompt-hash\s+(\S+)/)?.[1]||null:null,nucleus_pins:[...text.matchAll(/(00_nucleo\/prompts\/_nuclei\/[^\s]+\.toml)\s+sha256:([0-9a-f]{64})/g)].map(m=>({path:m[1],declared_effective_sha256:m[2],raw_file_sha256:sha(m[1]),integrity:'Requires global crystalline-lint V26; effective hash is not raw file SHA256.'}))};});
const norm={
  asset:['00_nucleo/prompts/compiler/stdlib/structural/document.md:10','Explicit crystalline extension and native signature.'],
  'sym.registered':['00_nucleo/prompts/compiler/stdlib/sym.md:94','Explicit historical extension retained without parity credit.'],
  'math.registered':['00_nucleo/prompts/compiler/stdlib/structural/math.md:90','Explicit inherited extension retained without parity credit.'],
  replace:['00_nucleo/prompts/compiler/stdlib/text/case.md:26','Public native replace signature and global acceptance examples.'],
};
for(const r of historical.filter(r=>r.current_language_class==='INTENTIONAL_PRODUCT_EXTENSION')){
  if(norm[r.path])continue;
  if(r.path.startsWith('calc.'))norm[r.path]=['00_nucleo/prompts/compiler/stdlib/calc.md:14','No authorizing claim for this path in the fully read owner or exact-name search across prompts/ADRs. Rust compatibility comment alone is not L0 authority; normative intent unresolved.'];
  else if(r.path.startsWith('counter_'))norm[r.path]=['00_nucleo/prompts/compiler/stdlib/counter.md:213','Explicit global natives retained for historical compatibility.'];
  else if(r.path.startsWith('state_'))norm[r.path]=['00_nucleo/prompts/compiler/stdlib/state.md:142','Explicit global natives retained for historical compatibility.'];
  else if(/^(grid|table)_/.test(r.path))norm[r.path]=['00_nucleo/prompts/compiler/eval.md:194','Six flat aliases explicitly preserve their historical names.'];
  else if(['lof','lot'].includes(r.path))norm[r.path]=['00_nucleo/prompts/compiler/stdlib/structural/outline.md:9','Explicit extension aliases without vanilla homologues.'];
  else if(r.owner_prompt.endsWith('math_style.md'))norm[r.path]=['00_nucleo/prompts/compiler/stdlib/math_style.md:76','Explicit registration of each style native in root scope; later claims preserve globals and shared native.'];
  else norm[r.path]=['00_nucleo/prompts/compiler/stdlib/structural.md:14','Explicit global structural natives, with owner signatures; underover also explicitly intentional at structural.md:642.'];
}
const receipt={schema:'p1309-classification-sources-v1',executor:'/root/p1309_classifier',at:new Date().toISOString(),head:'eb24cd657fc2333dc7ea5393f7cfebf8c7192d39',tracked_diff_stat:'',regime:'executado sem atestação de isolamento técnico',phase:'SOURCE_PREPARATION_NOT_RUNTIME_CLASSIFICATION',context_inherited:'Parent task scope, repository rules, exact P1309 step path; no candidate patch, no fresh matrix received.',write_allowlist:['p1309-owner-ledger.tsv','p1309-transition-ledger.tsv','p1309-selection.json','p1309-classification-*','p1309-certification-debt.json'],inputs:[base+'p1304-owner-ledger.tsv',base+'p1304-feature-matrix.json',base+'p1304-decision-report.md'].map(path=>({path,sha256:sha(path)})),prompts,extension_normative_evidence:norm,owner_correction:[{path:'raw.line',historical_owner:'00_nucleo/prompts/compiler/stdlib/text.md',measured_evidence:'00_nucleo/prompts/compiler/stdlib/text.md:78; 00_nucleo/prompts/compiler/stdlib/structural/markup.md:26; 01_core/src/compiler/stdlib/structural/markup.rs:78',current_native_owner:'00_nucleo/prompts/compiler/stdlib/structural/markup.md',note:'Text hub explicitly delegates raw ownership; future raw.line implementation still requires new exact L0 contract.'}],limitations:['No runtime final classification or selection until frozen full matrix is stable in canonical/repeated/reversed order.','Lineage metadata measures unique headers, not sufficient global V15/V26 proof.','Historical source line anchors will be refreshed against product at runtime classification.','Normative silence in calc is unresolved intent; no fabricated public-contract contradiction or automatic removal.']};
process.stdout.write(JSON.stringify({[base+'p1309-classification-sources.json']:JSON.stringify(receipt,null,2)+'\n'}));
