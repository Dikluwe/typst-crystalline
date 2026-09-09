"""Seal the read-only audit only after independent final approval.

No product execution, hash repair, cleanup, staging or commit is performed.
The prior R0 temporary PDF retention failure is explicit, never waived away.
"""
import importlib.util, json, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('receipt', D/'p1335-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)

def read(name):
    return json.loads((D/('p1335-'+name+'.json')).read_text())

def check_pin(path, expected):
    p = Path(path)
    if not p.is_absolute(): p = r.ROOT/p
    assert p.is_file(), str(p)
    assert r.sha(p) == expected, str(p)

def check_inputs(data):
    inputs = data.get('inputs', {})
    if isinstance(inputs, dict):
        for p, h in inputs.items():
            if isinstance(h, str) and len(h) == 64: check_pin(p, h)
    elif isinstance(inputs, list):
        for item in inputs:
            if isinstance(item, dict) and 'path' in item and 'sha256' in item:
                check_pin(item['path'], item['sha256'])

def main():
    before = r.state(); r.verify(before)
    baseline = r.baseline()
    assert before['branch'] == baseline['state']['branch']
    check_pin(r.STEP, baseline['step_sha256'])
    for p, h in baseline['historical_preserved'].items(): check_pin(p, h)
    for p, h in baseline['inputs'].items(): check_pin(p, h)
    manifest = read('manifest'); check_inputs(manifest)
    check_pin(D/'p1335-baseline.json', manifest['baseline_sha256'])
    check_pin(manifest['step']['path'], manifest['step']['sha256'])
    gates = ['build','workspace-tests','fmt','lint','lineage','diff-check']
    for name in gates:
        data = read(name); assert data['exit'] == 0, name
        r.verify(data['before']); r.verify(data['after'])
        assert data['manifest_sha256'] == r.sha(D/'p1335-manifest.json')
    bins = dict(vanilla=baseline['vanilla'], crystalline=read('build')['candidate'])
    for binary in bins.values(): check_pin(binary['path'], binary['sha256'])
    frozen = ['runtime-freeze','sentinels-freeze-r3','transversal-freeze-r3',
              'math-controls-freeze','math-controls-freeze-r1','location-control-freeze',
              'classification-preservation-freeze-r3']
    for name in frozen: check_inputs(read(name))
    aggregate = read('aggregate-r1')
    for category, expected in [('principal',18872),('supplement',4422)]:
        a = aggregate[category]
        assert a['pairs'] == expected and a['stable'] and not any(a['deltas'].values())
        assert all('UNKNOWN' not in c.upper() or count == 0 for c,count in a['counts'].items())
        check_inputs(a)
    reviews = ['review-inventory','review-gates','review-runtime-final',
               'review-supplement-final-r3','review-transversal-final-r4',
               'review-selection-r1','review-final']
    for name in reviews:
        review = read(name)
        assert review['verdict'] == 'Preserved', name
        assert not review.get('violations') and not review.get('unknowns'), name
        check_inputs(review)
    final = read('review-final')
    # Final review must cover the exact report and this closing algorithm.
    final_text = json.dumps(final)
    for p in [D/'p1335-o-que-falta-para-paridade.md', Path(__file__),
              D/'p1335-classification-summary-r1.json', D/'p1335-classification-selection-r1.json']:
        assert p.name in final_text and r.sha(p) in final_text, f'final review has no pin for {p}'
    classification = read('classification-summary-r1')
    selection = read('classification-selection-r1')
    for data in [classification, selection]:
        assert not data['blockers'] and data['implementation_authorized'] is False
        check_inputs(data)
    assert classification['principal']['three_orders_checked'] and classification['principal']['unknown'] == 0
    assert classification['transversal']['strict_successor_unknown'] == 0
    assert selection['selected']['id'] == 'primitive-instance-field-diagnostic'
    assert selection['ranked_eligible'][0]['id'] == selection['selected']['id']
    # Verify retained active and focal-successor exports, not lost R0 bytes.
    export_artifacts = {}
    for name in ['transversal-focal-r1','transversal-focal-r3','transversal-r3']:
        for p, h in read(name)['artifacts'].items():
            check_pin(p, h); export_artifacts[p] = h
    qa = read('visual-qa'); check_inputs(qa)
    for o in qa['observations'].values():
        for kind in ('pdf','png'):
            check_pin(o[kind]['path'], o[kind]['sha256'])
            export_artifacts[o[kind]['path']] = o[kind]['sha256']
    incident = read('transversal-retention-incident')
    assert len(incident['affected']) == 8
    for affected in incident['affected']:
        check_pin(affected['path'], affected['current'])
        assert affected['recorded'] != affected['current']
    # One full corpus per order, literal sources/raw receipts all retained.
    active = ['aggregate-r1','transversal-metrics','classification-summary-r1',
              'classification-selection-r1','classification-functional-reconciliation-r1',
              'classification-source-lineage-r1','classification-certification-debt',
              'cli-capability-overlay','visual-qa','location-control','math-controls-r1',
              'transversal-r3']
    active += ['matrix-'+p for p in ('normal','repeat','reverse')]
    active += ['sentinels-'+p+'-r3' for p in ('normal','repeat','reverse')]
    active += gates + frozen + reviews
    active_inputs = {str(D/('p1335-'+name+'.json')):r.sha(D/('p1335-'+name+'.json')) for name in active}
    artifacts = {}
    for root in sorted(D.glob('p1335-*')):
        paths = root.rglob('*') if root.is_dir() else [root]
        for p in paths:
            if p.is_file() and '__pycache__' not in p.parts:
                artifacts[str(p)] = r.sha(p)
    after = r.state(); r.verify(after)
    assert after['branch'] == before['branch']
    r.save('closure',dict(at=r.now(),state=after,baseline_sha256=r.sha(D/'p1335-baseline.json'),
        manifest_sha256=r.sha(D/'p1335-manifest.json'),previous_closure_sha256=baseline['previous_closure_sha256'],
        step=dict(path=str(r.STEP),sha256=r.sha(r.STEP)),binaries=bins,
        active_inputs=active_inputs,artifacts=artifacts,export_artifacts=export_artifacts,
        historical_preserved=baseline['historical_preserved'],
        output_retention_limitation=dict(receipt=str(D/'p1335-transversal-retention-incident.json'),sha256=r.sha(D/'p1335-transversal-retention-incident.json'),
            affected=incident['affected'],scope='Eight R0 temporary PDFs were overwritten by focal R1 and are unrecoverable. Original JSON/raw CLI and all pre-P1335 evidence are preserved. Active full R3 outputs retained and checked; no R0 PDF preservation claimed.'),
        outcome='P1335_AUDIT_VALID_WITH_ONE_RECOMMENDATION_AND_EXPLICIT_LIMITS',
        selected=selection['selected']['id'],product_changed=False,l0_changed=False,staged=False,committed=False,
        regime=manifest['regime'],production_mutants_executed=0,global_language_parity_claim=False,
        closure_algorithm_sha256=r.sha(__file__)))

if __name__ == '__main__': main()
