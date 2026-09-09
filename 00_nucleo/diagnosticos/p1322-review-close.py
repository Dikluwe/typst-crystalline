"""Pin the final operator report only after independent review and final handoff."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
report=D/'p1322-o-que-falta-para-paridade.md'
assert len(sys.argv)==2 and sha(report)==sys.argv[1], 'Final report pin required'
names=['runtime-final','supplement','transversal-r2','gates','preservation-final',
       'attacks-catalog','attacks-runtime','attacks-semantic-r1','semantic-final',
       'query-exit2-controls','channels','selection-r2']
pins={}
input_checks=[]
receipts={}
for name in names:
    p=D/f'p1322-review-{name}.json';pins[str(p)]=sha(p)
    data=json.loads(p.read_text());receipts[name]=data
    if not name.startswith('attacks-'):
        assert data['verdict']=='Preserved', name
    for p,h in data.get('inputs',{}).items():
        assert Path(p).is_file() and sha(p)==h, (name,p)
        input_checks.append(dict(receipt=name,path=p,sha256=h))
attacks={}
for name in ['attacks-catalog','attacks-runtime','attacks-semantic-r1']:
    for attack in receipts[name]['attacks']:
        assert attack['id'] not in attacks
        attacks[attack['id']]=attack
assert len(attacks)==13 and all(a['valid'] and a['rejected'] for a in attacks.values())
assert receipts['selection-r2']['attack']['valid'] and receipts['selection-r2']['attack']['rejected']
for name in ['p1322-review-plan.md','p1322-review-check.py','p1322-review-semantic.py',
             'p1322-review-attacks.py','p1322-review-channels.py','p1322-review-selection-r2.py',
             'p1322-review-close.py','p1322-review-channels-objection.md',
             'p1322-classification-freeze.md','p1322-classification-selection-r2.json',
             'p1322-classification-query-hint-addendum.md','p1322-classification-query-hint-addendum-r2.md',
             'p1322-transversal-channels.json','p1322-review-final.md',report.name]:
    p=D/name;pins[str(p)]=sha(p)
result=dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),role='D',
    verdict='AUDIT_PRESERVED_WITH_DECLARED_LIMITS',
    isolation='executado sem atestação técnica de isolamento',
    authority='Independent reviewer only; no product/L0 edits; no implementation authorization; not global language parity',
    final_report=dict(path=str(report),sha256=sha(report)),
    selected=receipts['selection-r2']['selected'],
    unique_planned_attacks=dict(valid=13,rejected=13,survivors=0,ids=sorted(attacks)),
    repeated_D12_current_universe=receipts['selection-r2']['attack'],
    counts=dict(principal_probes=4718,principal_cells_per_order=18872,historical_probes=2182,new_probes=2536,
        raw_equal=18220,adjusted_denominator=18716,supplement_cases=816,supplement_cells_per_order=3186,
        principal_execution_unknown=0,supplement_execution_unknown=0,actual_historical_match_regressions=0,
        transversal_projected_match_raw_delta_per_order=17,historical_mutation_families_not_discharged=37),
    checks_by_receipt={k:v.get('checks') for k,v in receipts.items() if v.get('checks') is not None},
    validated_input_pins=input_checks,pins=pins,
    limits=['MATCH is only the declared typed observable; raw-channel overlay remains authoritative for warnings/hints.',
        'Old selection is superseded by the R2 causal universe. HTML L0 contradiction outranks namespace diagnostics.',
        'Generic/incomplete exit2 remains Unknown; only the complete observed query --features rejection is typed CLI absence.',
        'D06 opacity and D09 historical regression are deliberate copied-data controls, not observed product failures.',
        'No product mutants run; 13/13 does not discharge 37 historical certification families.',
        'Initial C summary rewrite was not fully archived; final frozen artifacts and focal reruns are reproducible.',
        'Source completeness of future missing capabilities/complex query formatter remains Unknown; no low-risk inference.',
        'Temporary files and prior evidence retained; no stage/commit/push.'])
dest=D/'p1322-review-final.json';assert not dest.exists()
payload=json.dumps(result,ensure_ascii=False,indent=2)+'\n'
subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(dest)+'\n'+''.join('+'+line+'\n' for line in payload.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
print(json.dumps(dict(verdict=result['verdict'],receipt_sha256=sha(dest),report=result['final_report'],input_pins_checked=len(input_checks)),indent=2))
