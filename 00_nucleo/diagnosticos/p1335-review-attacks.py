"""Frozen P1335 attacks on copies of audit data; stdout only, no judged writes."""
import base64
import collections
import copy
import importlib.util
import json
from pathlib import Path
import sys
import time
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('review', D/'p1335-review-check.py')
c = importlib.util.module_from_spec(spec)
spec.loader.exec_module(c)

def fingerprint(obj):
    return c.sha(json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode())

def attack(id_, original, mutant, judge, transformation, control_expected='Preserved'):
    positive, negative = judge(original).result(), judge(mutant).result()
    return dict(id=id_, expected='Violated', control_expected=control_expected,
        control=positive, observed=negative, control_sha256=fingerprint(original),
        mutant_sha256=fingerprint(mutant), transformation=transformation,
        valid=positive['verdict']==control_expected,
        rejected=negative['verdict']=='Violated' and bool(negative['violations']))

def catalog():
    cat = c.read('p1335-probe-catalog.json')
    rec = c.read('p1335-inventory-reconciliation.json')
    history = c.read('p1322-probe-catalog.json')
    inv = [c.read(c.ROOT/s['path']) for s in cat['inventory_inputs']]
    original = dict(catalog=cat, reconciliation=rec)
    def judge(data):
        return c.verify_catalog(data['catalog'], data['reconciliation'], history, inv)
    rows = []
    old_id = history['probes'][0]['id']
    mutant = copy.deepcopy(original)
    mutant['reconciliation']['unchanged'] = [p for p in rec['unchanged'] if p['probe_id']!=old_id]
    rows.append(attack('R01', original, mutant, judge, dict(remove_historical_id=old_id)))
    mutant = copy.deepcopy(original)
    mutant['catalog']['probes'].append(copy.deepcopy(cat['probes'][0]))
    rows.append(attack('R02', original, mutant, judge, dict(duplicate_id=old_id)))
    mutant = copy.deepcopy(original)
    genuine = next(p for p in cat['probes'] if p['path'].startswith('sym.'))
    fake = copy.deepcopy(genuine)
    fake.update(id='p1335-review-fiction', path=genuine['path']+'.__p1335_fiction__.r.double')
    fake['expression']='repr((type('+fake['path']+'), repr('+fake['path']+')))'
    mutant['catalog']['probes'].append(fake)
    mutant['reconciliation']['added'].append(dict(probe_id=fake['id'],path=fake['path']))
    rows.append(attack('R03', original, mutant, judge, dict(fictional_symbol_route=fake)))
    shuffled = copy.deepcopy(original)
    shuffled['catalog']['probes'].reverse()
    shuffled['reconciliation']['unchanged'].reverse()
    return rows, dict(reordered=judge(shuffled).result())

def runtime():
    cat = c.read('p1335-probe-catalog.json')
    matrix = c.read('p1335-matrix-normal.json')
    frozen = c.read('p1335-runtime-freeze.json')
    def focal(id_):
        catalog = {**cat,'probes':[p for p in cat['probes'] if p['id']==id_]}
        data = {**matrix,'results':[copy.deepcopy(p) for p in matrix['results'] if p['id']==id_]}
        data['counts']=dict(collections.Counter(p['runtime_class'] for p in data['results']))
        data['pairs'],data['probes']=len(data['results']),len(catalog['probes'])
        return catalog,data
    candidate = next(p for p in matrix['results'] if p['runtime_class'] in c.MATCH)
    catalog,original = focal(candidate['id'])
    def judge(data):
        return c.verify_matrix(data,catalog,frozen['binaries'],'normal')[0]
    rows=[]
    mutant=copy.deepcopy(original)
    mutant['results'][0]['crystalline']['binary_sha256']=frozen['binaries']['vanilla']['sha256']
    rows.append(attack('R04',original,mutant,judge,dict(replace='crystalline binary digest with vanilla digest')))
    mutant=copy.deepcopy(original)
    mutant['results'][0]['crystalline']['profile']='html' if mutant['results'][0]['profile']=='default' else 'default'
    rows.append(attack('R05',original,mutant,judge,dict(replace='profile label, unchanged argv')))
    mutant=copy.deepcopy(original)
    item=next(p for p in mutant['results'] if p['profile']=='html')['crystalline']
    i=item['argv'].index('--features');del item['argv'][i:i+2]
    rows.append(attack('R06',original,mutant,judge,dict(remove='html features argv pair')))
    diag=next(p for p in matrix['results'] if p['runtime_class']=='MATCH_DIAGNOSTIC' and p['crystalline']['stderr'])
    dc,diagnostic=focal(diag['id'])
    trusted={(p['id'],p['profile']):p for p in diagnostic['results']}
    def djudge(data):
        return c.verify_matrix(data,dc,frozen['binaries'],'normal',trusted=trusted)[0]
    mutant=copy.deepcopy(diagnostic)
    item=mutant['results'][0]['crystalline'];item['stderr']=''
    item['stderr_base64']='';item['stderr_sha256']=c.sha(b'')
    rows.append(attack('R07',diagnostic,mutant,djudge,dict(remove='nonempty stderr with internally consistent replacement hashes; retain MATCH')))
    opaque=copy.deepcopy(original)
    opaque['results'][0]['crystalline'].update(complete=False,reason_code='TIMEOUT',exit_code=None)
    opaque['results'][0]['runtime_class']='EXECUTION_UNKNOWN'
    opaque['counts']=dict(collections.Counter(p['runtime_class'] for p in opaque['results']))
    mutant=copy.deepcopy(opaque);mutant['results'][0]['runtime_class']='MATCH_VALUE'
    mutant['counts']=dict(collections.Counter(p['runtime_class'] for p in mutant['results']))
    rows.append(attack('R08',opaque,mutant,judge,dict(scenario='simulated opacity on copy, no product timeout claimed',replace='Unknown with MATCH'),control_expected='Unknown'))
    supplement=c.read('p1335-sentinels-cases-r2.json')
    supplementary_cells=sum(len(x['profiles']) for x in supplement['cases'])
    mutant=copy.deepcopy(original);mutant['pairs']+=supplementary_cells
    rows.append(attack('R11',original,mutant,judge,dict(add_frozen_supplement_cells_to_denominator=supplementary_cells)))
    shuffled=copy.deepcopy(original);shuffled['results'].reverse()
    return rows,dict(reordered=judge(shuffled).result(),opaque=judge(opaque).result())

def main():
    phase=sys.argv[1];start,tick=c.utc(),time.monotonic()
    rows,controls={'catalog':catalog,'runtime':runtime}[phase]()
    print(json.dumps(dict(start=start,end=c.utc(),seconds=time.monotonic()-tick,
        role='D',isolation='not technically attested',phase=phase,
        plan_sha256=c.digest(D/'p1335-review-plan.md'),manifest_sha256=c.digest(D/'p1335-manifest.json'),
        checker_sha256=c.digest(D/'p1335-review-check.py'),adversary_sha256=c.digest(__file__),
        inputs=c.INPUTS,attacks=rows,controls=controls,valid=sum(x['valid'] for x in rows),
        rejected=sum(x['valid'] and x['rejected'] for x in rows),
        scope='Audit data only; not a product mutation score.'),ensure_ascii=False,indent=2))

if __name__=='__main__':
    main()
