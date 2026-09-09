"""Final current ledger/ranking audit and frozen R13/R14 data attacks."""
import collections,copy,importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(name):
    s=importlib.util.spec_from_file_location(name,D/(name+'.py'));m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
s=load('p1335-review-semantic');a=load('p1335-review-attacks');c=s.core
s.MINIMUM_OWNERS['primitive-instance-field-diagnostic']={s.FIELD}

def judge(selection):
    r=s.verify_selection(selection);cohorts={x['id']:x for x in selection['cohorts']}
    for name in ('primitive-instance-field-diagnostic','math-lexical-callee-resolution'):
        r.require(cohorts[name]['priority']==2,'CURRENT_CANONICAL_PRIORITY',name)
    for co in selection['cohorts']:
        r.require(co['path_count']==len(set(co['paths'])),'COHORT_PATH_COUNT',co['id'])
        if not co['eligible']:continue
        r.require(co['owner_set_complete'] and len(co['owner_prompts'])==co['owner_count'],'OWNER_COMPLETENESS',co['id'])
        for owner in co['owners']:
            txt=(c.ROOT/owner).read_text();r.require(any('@prompt '+p+'\n' in txt for p in co['owner_prompts']),'CAUSAL_OWNER_LINK',[co['id'],owner])
        r.require(set(co['paths'])<={x['path'] for x in co['witnesses']},'CAUSAL_PATH_WITNESS',co['id'])
    return r

def main():
    r=c.preflight();selection=c.read('p1335-classification-selection-r1.json');summary=c.read('p1335-classification-summary-r1.json');lineage=c.read('p1335-classification-source-lineage-r1.json')
    for obj in (selection,summary):
        for pin in obj['inputs']+[obj['classifier']]:r.require(c.digest(c.ROOT/pin['path'])==pin['sha256'],'CLASSIFIER_INPUT_PIN',pin['path'])
        r.require(obj['implementation_authorized']==False and obj['blockers']==[],'NO_IMPLEMENTATION_OR_BLOCKERS',None)
    for pin in lineage['additional_causal_sources']:r.require(c.digest(c.ROOT/pin['path'])==pin['sha256'],'CAUSAL_SOURCE_PIN',pin['path'])
    normal=c.read('p1335-matrix-normal.json')['results'];prior=c.read('p1322-matrix-normal.json')['results'];cat=c.read('p1335-probe-catalog.json')
    ledger=s.table('p1335-classification-owner-ledger-r1.tsv');trans=s.table('p1335-classification-transitions-r1.tsv');supp=c.read('p1335-sentinels-normal-r3.json')['rows']
    c.merge_review(r,s.verify_principal(ledger,cat,normal));c.merge_review(r,s.verify_transitions(trans,normal,prior));c.merge_review(r,judge(selection))
    counts=dict(collections.Counter(x['runtime_class'] for x in normal));closed=sum(x['runtime_class'] in c.MATCH for x in normal);p=summary['principal'];adj=p['adjusted'];er=c.read('p1335-review-extensions.json')
    extensions=set(er['accepted_documented_paths']);retained=[x for x in normal if x['path'] not in extensions]
    r.require(p['probes']==len(cat['probes']) and p['cells']==len(normal) and p['raw_counts']==counts and p['raw_closed_cells']==closed and p['raw_ratio']==closed/len(normal),'PRINCIPAL_EXACT_METRICS',None)
    r.require(set(adj['excluded_paths'])==extensions and adj['excluded_cells']==len(normal)-len(retained) and adj['denominator']==len(retained) and adj['numerator']==closed and adj['ratio']==closed/len(retained),'ADJUSTMENT_EXACT_METRICS',None)
    r.require(p['path_classes']==dict(collections.Counter(x['current_language_class'] for x in ledger)),'LEDGER_CLASS_COUNTS',None)
    r.require(summary['supplement']['cases']==len({x['id'] for x in supp}) and summary['supplement']['cells']==len(supp) and summary['supplement']['outside_principal_denominator'],'SUPPLEMENT_SEPARATE',None)
    r.require(summary['temporal']['new_public_paths']==len({x['id'] for x in normal}-{x['id'] for x in prior}) and summary['temporal']['principal_cell_transitions']==dict(collections.Counter(x['transition'] for x in trans)),'TEMPORAL_PRINCIPAL_ONLY',None)
    attacks=[];mutant=copy.deepcopy(selection)
    mutant['selected']=copy.deepcopy(next(x for x in mutant['cohorts'] if x['id']=='math-lexical-callee-resolution'))
    attacks.append(a.attack('R13',selection,mutant,judge,dict(replace='selected cohort with runner-up math; current priority and ranks retained')))
    mutant=copy.deepcopy(selection);co=next(x for x in mutant['cohorts'] if x['id']=='loader-data-source-missing')
    co['owners']=[x for x in co['owners'] if x!=s.DISPATCH];co['owner_prompts']=[x for x in co['owner_prompts'] if x!='00_nucleo/prompts/compiler/eval/call_dispatch.md'];co['owner_count']=len(co['owners']);co['rank_key'][1]=co['owner_count']
    attacks.append(a.attack('R14',selection,mutant,judge,dict(remove='necessary call_dispatch transport owner; owner_count and rank changed consistently, not a mere arithmetic mismatch')))
    reordered=copy.deepcopy(selection);reordered['cohorts'].reverse();c.merge_review(r,judge(reordered))
    return dict(at=c.utc(),role='D',checker_sha256=c.digest(__file__),semantic_checker_sha256=c.digest(D/'p1335-review-semantic.py'),plan_sha256=c.digest(D/'p1335-review-plan.md'),inputs=c.INPUTS,selected=selection['selected']['id'],rank_key=selection['selected']['rank_key'],principal=summary['principal'],attacks=attacks,valid=sum(x['valid'] for x in attacks),rejected=sum(x['valid'] and x['rejected'] for x in attacks),limits='Independent source lower bounds and current canonical priority are human-read premises; data attacks are not product mutants or isolation attestation. Temporal supplement 64 comparable versus12 relocated is handled by successor qualification.',**r.result())

if __name__=='__main__':print(json.dumps(main(),ensure_ascii=False,indent=2))
