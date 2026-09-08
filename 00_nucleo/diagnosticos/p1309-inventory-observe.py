#!/usr/bin/env python3
"""Role A standalone per-side route observations, never a bilateral matrix."""
import argparse
import concurrent.futures
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('inventory_runner', HERE / 'p1309-inventory-runner.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)

def main():
    p = argparse.ArgumentParser()
    p.add_argument('side', choices=['vanilla','crystalline'])
    p.add_argument('--binary',required=True)
    p.add_argument('--expected-sha',required=True)
    p.add_argument('--suffix',default='-r2')
    args = p.parse_args()
    assert r.sha(args.binary)==args.expected_sha
    for profile in ['default','html']:
        source=HERE/f'p1309-inventory-{args.side}-{profile}{args.suffix}.json'
        payload=json.loads(source.read_text())
        features=[] if profile=='default' else ['--features','html']
        begin=r.utc()
        tick=time.monotonic_ns()
        def observe(item):
            route,entry=item
            expression=f'repr((type({route}), repr({route})))'
            run=r.command([args.binary,'eval',expression,'--format','json',*features],timeout=20)
            observation='EXECUTION_UNKNOWN' if run['unknown'] or run['exit_code'] is None or run['exit_code']<0 else ('VALUE' if run['exit_code']==0 else 'DIAGNOSTIC')
            decoded=None
            if observation=='VALUE':
                try:
                    decoded=json.loads(run['stdout'])
                    if not isinstance(decoded,str):
                        raise ValueError('required type/repr tuple not serialized to string')
                except (ValueError,TypeError) as e:
                    observation='EXECUTION_UNKNOWN'
                    run['unknown']=str(e)
            segments=route.split('.')
            return route,dict(name=segments[-1],namespace='.'.join(segments[:-1]),ancestors=['.'.join(segments[:i]) for i in range(1,len(segments))],expression=expression,expression_sha256=hashlib.sha256(expression.encode()).hexdigest(),kind_from_structural_inventory=entry['kind'],type_and_repr=decoded,observation=observation,execution=run)
        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
            observations=dict(pool.map(observe, sorted(payload['entries'].items())))
        unknowns=[route for route,v in observations.items() if v['observation']=='EXECUTION_UNKNOWN']
        result=dict(schema_version='p1309-inventory-language-observations-v1',author='/root/p1309_inventory',role='A',side=args.side,profile=profile,features=features,baseline=payload['p1309_provenance']['baseline'],source_state=r.command(['git','diff','HEAD','--stat'])['stdout'],started_utc=begin,ended_utc=r.utc(),duration_ns=time.monotonic_ns()-tick,product=dict(path=args.binary,sha256=r.sha(args.binary)),structural_input=dict(path=str(source.relative_to(r.ROOT)),sha256=r.sha(source)),processes=len(observations),unknowns=unknowns,entries=observations)
        pin=r.publish(f'p1309-inventory-observables-{args.side}-{profile}{args.suffix}.json',result)
        # Catalog pins both artifacts; observations pin immutable structural input.
        # Avoid a cyclic inventory <-> observations hash reference.
        print(args.side,profile,'observations',len(observations),'unknowns',len(unknowns),flush=True)

if __name__=='__main__':
    main()
