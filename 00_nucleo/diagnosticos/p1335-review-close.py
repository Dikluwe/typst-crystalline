"""Assemble independent review evidence; stdout only, no judged mutations."""
import importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('c',D/'p1335-review-check-r1.py');c=importlib.util.module_from_spec(s);s.loader.exec_module(c)

def main():
    r,preserved=c.preservation_review();components={};pins={}
    names=['inventory','gates','runtime-final','supplement-final-r3','transversal-final-r4','boundaries','preservation-ledger-r1','extensions','certification','selection-r1','final-ledger-r1','preservation-final']
    for name in names:
        p=D/('p1335-review-'+name+'.json');data=c.read(p.name);pins[str(p)]=c.digest(p)
        r.require(data['verdict']=='Preserved' and not data.get('violations') and not data.get('unknowns'),'REVIEW_COMPONENT',name)
        for path,pin in data.get('inputs',{}).items():
            if isinstance(pin,str):r.require(Path(path).is_file() and c.digest(path)==pin,'COMPONENT_INPUT_PIN',[name,path])
        components[name]=dict(path=str(p),sha256=c.digest(p),verdict=data['verdict'],checks=data.get('checks'))
    attacks=[]
    for name in ['attacks-catalog','attacks-runtime','principal-partial','selection-r1']:
        p=D/('p1335-review-'+name+'.json');data=c.read(p.name);pins[str(p)]=c.digest(p);attacks.extend(data['attacks'])
    ids=[x['id'] for x in attacks]
    r.require(set(ids)=={f'R{i:02}' for i in range(1,15)} and len(ids)==len(set(ids)),'FROZEN_ATTACK_COVERAGE',ids)
    for a in attacks:r.require(a['valid'] and a['rejected'] and a['observed']['verdict']=='Violated','FROZEN_ATTACK_REJECTED',a['id'])
    for name in ['p1335-close.py','p1335-o-que-falta-para-paridade.md','p1335-classification-summary-r1.json','p1335-classification-selection-r1.json','p1335-classification-functional-reconciliation-r1.json','p1335-classification-source-lineage-r1.json','p1335-classification-recommendation.md','p1335-review-final.md','p1335-review-plan.md','p1335-transversal-retention-incident.json','p1335-aggregate-r1.json','p1335-transversal-metrics.json','p1335-visual-qa.json']:
        pins[str(D/name)]=c.digest(D/name)
    pins[str(Path(__file__))]=c.digest(__file__)
    for p in sorted(D.glob('p1335-review-*.py')):pins[str(p)]=c.digest(p)
    return dict(at=c.utc(),role='D',regime='executado sem atestação técnica de isolamento',manifest_sha256=c.digest(D/'p1335-manifest.json'),baseline_sha256=c.digest(D/'p1335-baseline.json'),head=c.read('p1335-baseline.json')['state']['head'],working_tree='uncommitted exact sixteen tracked files/diff in pinned baseline',inputs=pins,components=components,attacks=dict(planned=14,executed=len(attacks),valid=sum(a['valid'] for a in attacks),rejected=sum(a['valid'] and a['rejected'] for a in attacks),ids=sorted(ids),scope='Data-auditor attacks only; not product mutation score'),preservation=preserved,selected='primitive-instance-field-diagnostic',rank=[2,1,-2,1,'primitive-instance-field-diagnostic'],implementation_authorized=False,production_mutants_executed=0,certification_debt_discharged=False,global_language_parity_claim=False,closure_algorithm_reviewed=True,operator_closure_execution_pending=True,limits=['Finite scanner/pre-filter does not establish unlimited enumeration.','Eight R0 PDFs overwritten and unrecoverable; no R0 PDF retention claim.','Current raw CLI absence requires exact policy; old Unknown receipts retained.','Visual/export QA remains sampled, not global.','64 comparable functional closures and12 relocated current closures remain distinct.','Shared filesystem/tools provide procedural separation, not technical isolation.'],**r.result())

if __name__=='__main__':print(json.dumps(main(),ensure_ascii=False,indent=2))
