"""Close artifact provenance after independent review; no product mutation."""
import importlib.util
import json
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1322-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)

def main():
    state=r.state();r.verify(state)
    base=r.baseline()
    def unrelated_status(text):
        return [line for line in text.splitlines() if '00_nucleo/diagnosticos/p1322-' not in line]
    assert unrelated_status(state['status'])==unrelated_status(base['state']['status']), 'Changes outside P1322 diagnostic namespace'
    checked={}
    for group in ['inputs','untracked_at_start']:
        for name,digest in base[group].items():
            path=Path(name)
            if not path.is_absolute():path=r.ROOT/path
            # This baseline group contains diagnostics, never an invitation
            # to enumerate or semantically read restricted history folders.
            assert r.sha(path)==digest,str(path)
            checked[str(path)]=digest
    assert r.sha(r.STEP)==base['step_sha256']
    gates={}
    for name in ['build','workspace-tests','lint','lineage','fmt','diff-check']:
        path=D/('p1322-'+name+'.json');gate=json.loads(path.read_text())
        assert gate['exit']==0,name
        r.verify(gate['before']);r.verify(gate['after'])
        gates[name]=dict(sha256=r.sha(path),exit=gate['exit'])
    report=D/'p1322-o-que-falta-para-paridade.md'
    assert 'aguardando o veredito' not in report.read_text()
    broken=[]
    for ref in re.findall(r'\]\(([^)]+)\)',report.read_text()):
        if '://' in ref:continue
        path=Path(ref.split('#')[0])
        if not path.is_absolute():path=D/path
        if not path.exists():broken.append(ref)
    assert not broken,broken
    inventory={str(p):r.sha(p) for p in sorted(D.glob('p1322-*')) if p.is_file()}
    inventory.update({str(p):r.sha(p) for p in sorted((D/'p1322-fixtures').rglob('*')) if p.is_file()})
    r.save('closure',dict(at=r.now(),state=state,manifest_sha256=r.sha(D/'p1322-manifest.json'),
        baseline_sha256=r.sha(D/'p1322-baseline.json'),protected_product_files=len(state['product_inventory']),
        historical_preserved=checked,step_sha256=r.sha(r.STEP),gates=gates,artifacts=inventory,
        report_sha256=r.sha(report),report_local_links_valid=True,
        limits='Operator provenance receipt, not a replacement for independent role-D verdict. No product/L0 changes, stage, commit, push or cleanup in P1322.'))

if __name__=='__main__':main()
