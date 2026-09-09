// Role C, read-only inspection. Emit one artifact as an apply_patch patch on stdout.
const fs = require('fs'), crypto = require('crypto');
const b = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p, 'utf8');
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const json = n => JSON.parse(read(b + n));
const pin = p => ({path:p, sha256:sha(p)});
const baseline=json('p1322-baseline.json'), manifest=json('p1322-manifest.json');
if (manifest.baseline_sha256 !== sha(b+'p1322-baseline.json')) throw Error('baseline drift');
const reports = Array.from({length:12},(_,i)=>b+'p'+(1310+i)+'-final-report.md').filter(p=>fs.existsSync(p));
reports.push(b+'p1313-implementation-report.md',b+'p1320-o-que-falta-para-paridade.md');
const prior=json('p1309-classification-sources.json');
const fullRead = new Set(['compiler/stdlib/loading','compiler/stdlib/calc','compiler/eval/call_dispatch','compiler/eval/bindings/field_access'].map(p=>'00_nucleo/prompts/'+p+'.md'));
const prompts=prior.prompts.map(p=>({path:p.path,sha256:sha(p.path),prior_sha256:p.sha256,identical_to_p1309:sha(p.path)===p.sha256,reading:fullRead.has(p.path)?'Full current L0 read by C':'Historical source analysis authenticated by identical current bytes; relevant claims inspected, no new architectural proposal',historical_source_receipt:pin(b+'p1309-classification-sources.json')}));
const extensions=structuredClone(prior.extension_normative_evidence);
for (const [path,e] of Object.entries(extensions)) {
  if(e[0].endsWith('math_style.md:76')) e[0]=e[0].replace(':76',':72');
  if(path==='replace')e[0]='00_nucleo/prompts/compiler/stdlib/text/case.md:33';
}
const provenance={at:new Date().toISOString(),head:baseline.state.head,working_tree:'non-committed',diff_stat:baseline.state.diff_stat,baseline:pin(b+'p1322-baseline.json'),manifest:pin(b+'p1322-manifest.json'),regime:'executado sem atestação técnica de isolamento'};
const oldDebt=json('p1309-certification-debt.json');
for(const x of oldDebt.provenance.inputs)if(sha(x.path)!==x.sha256)throw Error('historical mutation input drift '+x.path);
const debt={schema:'p1322-classification-certification-debt-v1',...provenance,universe:'certification_debt',prior:pin(b+'p1309-certification-debt.json'),reports_inspected:reports.map(pin),measurement_before_decision:{historical_families:oldDebt.families.length,historical_separate_p1308_mutants:oldDebt.p1308_separate_executed_families.length,product_mutants_executed_in_p1322:0,discharge_receipts_found_in_p1310_p1321_reports:0},families:oldDebt.families.map(f=>({...f,status:'STILL_PENDING_NO_DISCHARGE_RECEIPT',p1322_observation:'Reports P1310–P1321 supply A/B and review, no execution receipt for this causal productive mutant; no mutant run in P1322.',actual_source_mutants_executed_in_p1322:0})),p1308_separate_executed_families:oldDebt.p1308_separate_executed_families,decision:'Keep each named historical obligation pending; passing functional examples or auditor attacks cannot discharge productive mutants.',p1307_mutation_score:null,limitations:['No claim that attacks were never performed outside the inspected recorded history.','Original F01–F20 are counted once, with S01–S12/A01–A05 separately; P1308 M01–M06 have no explicit discharge map.','Tool reverse-hash defect is independent of both functional parity and productive mutation obligations.']};
const sources={schema:'p1322-classification-preparation-v1',...provenance,phase:'SOURCE_AND_HISTORY_PREPARATION_NO_CURRENT_RUNTIME_VERDICT',reports:reports.map(pin),inputs:['p1309-final-report.md','p1309-owner-ledger.tsv','p1309-selection.json','p1309-classification-sources.json','p1309-certification-debt.json'].map(n=>pin(b+n)),prompts,extension_normative_evidence:extensions,normative_unresolved:['calc.deg','calc.log10','calc.rad'],limitations:['Current runtime classification and selection await mandatory matrices/supplement.','Historical unchanged L0 source references do not grant approval for new architecture.','Full current loading/calc/call_dispatch/field_access read; no blanket claim of full human/model inspection of every historical owner.']};
const name=process.argv[2]||'sources';
const output=name==='debt'?debt:sources;
const path=b+'p1322-classification-'+(name==='debt'?'certification-debt':'sources')+'.json';
if(fs.existsSync(path))throw Error('Refuse to overwrite '+path);
const payload=JSON.stringify(output,null,2)+'\n';
process.stdout.write('*** Begin Patch\n*** Add File: '+path+'\n'+payload.trimEnd().split('\n').map(l=>'+'+l).join('\n')+'\n*** End Patch');
