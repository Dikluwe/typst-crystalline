// Role C read-only input inspection; emits artifacts to stdout for apply_patch.
const fs = require('fs');
const crypto = require('crypto');
const base = '00_nucleo/diagnosticos/';
const read = p => fs.readFileSync(p, 'utf8');
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const planPath = base + 'p1307-r6-mutation-plan.json';
const originalPath = base + 'p1307-mutant-plan.json';
const p8Path = base + 'p1308-mutation-ledger.json';
const verifierPath = base + 'p1308-verification-r2-final.json';
const plan = JSON.parse(read(planPath));
const original = JSON.parse(read(originalPath));
const p8 = JSON.parse(read(p8Path));
const originals = new Map(original.families.map(r => [r.id, r]));
const at = new Date().toISOString();
const provenance = {
  head: 'eb24cd657fc2333dc7ea5393f7cfebf8c7192d39',
  measured_at_utc: at,
  tracked_diff_stat: '',
  source_state_receipt:'git rev-parse HEAD and git diff HEAD --stat executed separately immediately before artifact generation; stdout baseline HEAD and empty tracked diff.',
  inputs: [planPath, originalPath, p8Path, verifierPath].map(path => ({path,sha256:hash(path)})),
};
const families = [...plan.families,...plan.additional_families].map(r => ({
  ...r,
  origin: {path:planPath,sha256:hash(planPath),json_pointer:`/${plan.families.includes(r)?'families':'additional_families'}/${(plan.families.includes(r)?plan.families:plan.additional_families).indexOf(r)}`},
  predecessor: r.base_id ? {path:originalPath,sha256:hash(originalPath),base_id:r.base_id,defect:originals.get(r.base_id).defect} : null,
  status:'STILL_PENDING',
  actual_source_mutants_executed_in_p1309:0,
  supersession:'R6 refines instrumentation anchors of original F01-F20; it does not discharge their obligations. S01-S12 and A01-A05 remain distinct causal defects.',
  p1308_overlap: r.id==='F17'||r.id==='A04'||r.id==='A05' ? 'M02/M04/M05 exercise diagnostic transport nearby, but do not execute this named encoder/Args defect; no discharge.' : r.id==='S11' ? 'M06 alters Args formatting, not captured Heading repr; sharing formatter owner is not supersession.' : r.id==='A01' ? 'M03/M04 touch filter vocabulary/origin, not duplicate occurrence preservation; no discharge.' : 'No identical causal mutant in the six P1308 families.',
  reason:'The predecessor verifier explicitly states all 37 historical P1307 families were not executed or discharged. This audit executes no productive mutants.',
}));
const debt = {
  schema:'p1309-certification-debt-v1', executor:'/root/p1309_classifier',
  regime:'executado sem atestação de isolamento técnico', provenance,
  universe:'certification_debt', family_count:families.length,
  status_counts:{STILL_PENDING:families.length}, families,
  duplicate_mapping:{original_twenty:'Original integer IDs1..20 map bijectively to R6 F01..F20; count once, not twice.',additional:'S01..S12 and A01..A05 introduce17 distinct obligations; shared owner/witness does not imply duplicate cause.',superseded_families:[]},
  p1308_separate_executed_families:p8.rows.map(r=>({id:r.id,status:'EXECUTED_KILLED',origin:{path:p8Path,sha256:hash(p8Path)},receipt:r.receipt,source_change:r.source_change,witnesses:r.witnesses,causal_review:r.causal_review,discharges_p1307:[]})),
  p1307_mutation_score:null,
  recommendation:'Preparar passo adversarial próprio para as37 famílias pendentes, com candidato/controle GREEN pinados, mutação causal compilável, testemunha e restauração; não executar neste P1309.',
  claim_limit:'Functional encoder equality in a fresh corpus neither proves nor removes this certification debt. No language divergence is inferred from pending attacks. P1308 historical six-mutant score is not a P1307 score.',
};
process.stdout.write(JSON.stringify({[base+'p1309-certification-debt.json']:JSON.stringify(debt,null,2)+'\n'}));
