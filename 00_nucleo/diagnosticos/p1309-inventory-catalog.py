#!/usr/bin/env python3
"""Seal structural union without reading or classifying candidate matrices."""
import csv
import importlib.util
import io
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('inventory_runner', HERE / 'p1309-inventory-runner.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)

def main():
    history_path = HERE / 'p1304-probe-catalog.json'
    historical = json.loads(history_path.read_text())
    old = historical['probes']
    assert len(old) == 627
    assert len({p['id'] for p in old}) == len(old)
    records = {}
    inventories = []
    unknowns = []
    for side in ['vanilla', 'crystalline']:
        for profile in ['default', 'html']:
            path = HERE / f'p1309-inventory-{side}-{profile}-r2.json'
            inv = json.loads(path.read_text())
            observations_path = HERE / f'p1309-inventory-observables-{side}-{profile}-r2.json'
            observations = json.loads(observations_path.read_text())
            assert observations['structural_input']['sha256'] == r.sha(path)
            unknowns.extend(dict(path=x,side=side,profile=profile,reason='runtime_enumeration_unknown') for x in observations['unknowns'])
            inventories.append(dict(path=str(path.relative_to(r.ROOT)), sha256=r.sha(path), side=side, profile=profile, entries=len(inv['entries']), language_observations=dict(path=str(observations_path.relative_to(r.ROOT)),sha256=r.sha(observations_path))))
            for route, entry in inv['entries'].items():
                records.setdefault(route, []).append(dict(side=side, profile=profile, owner_path=entry['owner_path'], owner_kind=entry['owner_kind'], access_form=entry['access_form'], kind=entry['kind'], present=entry['present']))
                if entry['availability'] == 'unknown' or 'truncated' in route:
                    unknowns.append(dict(path=route, side=side, profile=profile, reason='structural_enumeration_unknown'))
    by_path = {}
    for probe in old:
        by_path.setdefault(probe['path'], []).append(probe['id'])
    fresh = []
    for route in sorted(set(records)-set(by_path)):
        fresh.append(dict(id='p1309-path-'+route, path=route, expression=f'repr((type({route}), repr({route})))', profiles=['default','html','a11y','html+a11y'], origins=['p1309:fresh-structural-union'], roles=['fresh_surface_route']))
    probes = old + fresh
    assert len({p['id'] for p in probes}) == len(probes)
    all_by_path = {p['path']:p['id'] for p in probes}
    with (HERE / 'p1304-owner-ledger.tsv').open() as f:
        # Only identity columns are consulted by role A; semantic columns are not read into records.
        reader = csv.DictReader(f, delimiter='\t')
        ledger_paths = [(row['path'], row['probe_id']) for row in reader]
    assert len(ledger_paths) == 167
    reconciliation = []
    for route, probe_id in ledger_paths:
        source = next((p for p in old if p['id'] == probe_id), None)
        if source is None:
            unknowns.append(dict(path=route, reason='historical_ledger_probe_unresolved', probe_id=probe_id))
        reconciliation.append(dict(historical_path=route, historical_probe_id=probe_id, current_probe_id=probe_id if source else None, current_probe_path=source['path'] if source else None, status='unchanged' if source else 'EXECUTION_UNKNOWN'))
    ancestors = []
    for route, entries in sorted(records.items()):
        pieces = route.split('.')
        chain = ['.'.join(pieces[:n]) for n in range(1, len(pieces))]
        for side in ['vanilla','crystalline']:
            for profile in ['default','html']:
                missing = [a for a in chain if not any(x['side']==side and x['profile']==profile and x['present'] for x in records.get(a, []))]
                if missing:
                    ancestors.append(dict(path=route, side=side, profile=profile, ancestors=chain, absent_ancestors=missing, executable_probe_id=all_by_path.get(route)))
    controls = {
        'encoder_public_routes':['cbor.encode','json.encode','toml.encode','yaml.encode'],
        'encoder_absence_controls':['csv.encode','xml.encode','read.encode'],
        'module_repr':['std','color.map'],
        'constructor_routes':['color.hsl','color.hsv','color.linear-rgb'],
        'feature_gates':['html','pdf.table-summary','pdf.header-cell','pdf.data-cell'],
    }
    # Explicit negative routes are language probes, not supplemental fixtures.
    for route in controls['encoder_absence_controls']:
        if route not in all_by_path:
            probe = dict(id='p1309-path-'+route,path=route,expression=f'repr((type({route}), repr({route})))',profiles=['default','html','a11y','html+a11y'],origins=['p1309:explicit-encoder-negative'],roles=['explicit_negative_control'])
            probes.append(probe)
            fresh.append(probe)
            all_by_path[route]=probe['id']
    catalog = dict(schema_version='p1309-probe-catalog-v1', author='/root/p1309_inventory', role='A', created_at=r.utc(), baseline='eb24cd657fc2333dc7ea5393f7cfebf8c7192d39', baseline_receipt=dict(path='00_nucleo/diagnosticos/p1309-baseline-r2.json',sha256=r.sha(HERE/'p1309-baseline-r2.json')),manifest=dict(path='00_nucleo/diagnosticos/p1309-manifest-r2.json',sha256=r.sha(HERE/'p1309-manifest-r2.json')), source_state=r.command(['git','diff','HEAD','--stat'])['stdout'], profiles=historical['profiles'], counts=dict(probes=len(probes),historical_probes=len(old),added_probes=len(fresh),fresh_union_paths=len(records)), historical_input=dict(path=str(history_path.relative_to(r.ROOT)),sha256=r.sha(history_path)),inventory_inputs=inventories, preservation='All 627 P1304 records retained verbatim, same order. Fresh union follows; IDs never recycled.',canonical_execution_order='Sort by probe id, then profiles default/html/a11y/html+a11y; reverse the full keyed job list for inverted order.', probes=probes, unknowns=unknowns)
    catalog_pin=r.publish('p1309-probe-catalog.json',catalog)
    report=dict(schema_version='p1309-catalog-reconciliation-v1',author='/root/p1309_inventory',role='A',created_at=r.utc(),baseline=catalog['baseline'],source_state=catalog['source_state'],catalog=catalog_pin, historical_catalog=catalog['historical_input'],historical_ledger=dict(path='00_nucleo/diagnosticos/p1304-owner-ledger.tsv',sha256=r.sha(HERE/'p1304-owner-ledger.tsv')),added=[dict(probe_id=p['id'],path=p['path'],reason='fresh structural discovery absent from historical catalog' if p['path'] in records else 'explicit negative encoder control') for p in fresh],removed=[],renamed=[],split=[],merged=[],unchanged=[dict(probe_id=p['id'],path=p['path'],verbatim=True,currently_structurally_enumerated=p['path'] in records) for p in old], historical_paths=reconciliation,blocked_by_ancestor=ancestors,controls={k:[dict(path=p,probe_id=all_by_path.get(p)) for p in v] for k,v in controls.items()},separate_supplement_required=['imports named/anonymous/shadowed globals','P1303/P1306 field spans','P1300 constructor calls','full four-encoder semantics','PDF/HTML positive/negative','P1308 causal spans and repr data retention'],unknowns=unknowns,judgment='Only route identity and mechanical structural availability; no semantic intent classification',regime='executado sem atestação de isolamento técnico')
    r.publish('p1309-inventory-reconciliation.json',report)
    print(json.dumps(dict(catalog=catalog_pin,counts=catalog['counts'],ledger_paths=len(reconciliation),blocked_ancestor_records=len(ancestors),unknowns=len(unknowns))))

if __name__=='__main__':
    main()
