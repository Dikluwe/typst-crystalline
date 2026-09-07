#!/usr/bin/env python3
"""Replay the authorized trace-only R6 successor, with the frozen classifier."""
import argparse, concurrent.futures, copy, importlib.util, re
from pathlib import Path

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('p1308_r2_frozen_r4',HERE/'p1307-r4-oracle.py')
base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)
classify=base.classify

def replay(args):
    oracle=base.read('p1308-r2-oracle.json')
    assert base.sha(__file__)==oracle['script_sha256']
    assert base.sha(HERE/'p1307-r4-oracle.py')=='ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68'
    for name,pin in oracle['inputs'].items():
        assert base.sha(HERE/name)==pin,'Protected predecessor changed: '+name
    for name,pin in oracle['trace_only_successor']['protected_inputs'].items():
        assert base.sha(HERE/name)==pin,'Successor evidence changed: '+name
    cases=[copy.deepcopy(c) for c in oracle['cases'] if not args.case or re.search(args.case,c['id'])]
    if args.reverse:cases.reverse()
    folder=base.fixtures(cases);byid={c['id']:c for c in cases}
    jobs=[(args.binary,'candidate',c,p) for c in cases for p in c['observations'] if not args.profile or p==args.profile]
    rows=[]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for row in pool.map(lambda x:base.run(*x),jobs):
            row['verdict']=classify(byid[row['case']]['observations'][row['profile']]['future_expected'],row['observable'])
            rows.append(row)
    import json
    print(json.dumps(dict(schema='p1308-r2-replay-v1',at=base.now(),binary=args.binary,
        binary_sha256=base.sha(args.binary),oracle_sha256=base.sha(HERE/'p1308-r2-oracle.json'),
        fixture_directory=folder,rows=rows,
        counts={v:sum(r['verdict']==v for r in rows) for v in ('Preserved','Violated','Unknown')}),ensure_ascii=False))

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('command',choices=['replay'])
    parser.add_argument('--binary',required=True)
    parser.add_argument('--case')
    parser.add_argument('--profile',choices=list(base.PROFILES))
    parser.add_argument('--reverse',action='store_true')
    replay(parser.parse_args())
