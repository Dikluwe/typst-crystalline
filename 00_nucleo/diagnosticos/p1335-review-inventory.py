"""Reconstruct finite Symbol union and current public observations independently."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('review',D/'p1335-review-check.py')
c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)

def run():
    r=c.preflight();catalog=c.read('p1335-probe-catalog.json')
    history=c.read('p1322-probe-catalog.json')
    reconciliation=c.read('p1335-inventory-reconciliation.json')
    inventories=[];summaries=[]
    build=c.read('p1335-build.json');baseline=c.read('p1335-baseline.json')
    for item in catalog['inventory_inputs']:
        inv=c.read(c.ROOT/item['path']);inventories.append(inv)
        obs=c.read(c.ROOT/item['language_observations']['path'])
        r.require(c.digest(c.ROOT/item['path'])==item['sha256'],'INVENTORY_PIN',item['path'])
        r.require(c.digest(c.ROOT/item['language_observations']['path'])==item['language_observations']['sha256'],'OBSERVATION_PIN',item['path'])
        raw=json.loads(inv['p1335_provenance']['receipt']['stdout'])['entries']
        variants={route+'.'+mods for route,entry in raw.items() for mods,value in (entry.get('symbol') or {}).get('variants',[]) if mods}
        expected=set(raw)|variants
        r.require(expected==set(inv['entries']),'FINITE_SYMBOL_UNION',[item['path'],sorted(set(inv['entries'])-expected),sorted(expected-set(inv['entries']))])
        r.require(set(obs['entries'])==expected,'OBSERVATION_COVERAGE',item['path'])
        binary=baseline['vanilla'] if item['side']=='vanilla' else build['candidate']
        r.require(obs['product']['sha256']==binary['sha256'] and obs['product']['path']==binary['path'],'OBSERVATION_BINARY',item['path'])
        observed_counts={}
        for route,entry in obs['entries'].items():
            o=entry['execution'];expression='repr((type('+route+'), repr('+route+')))'
            argv=[binary['path'],'eval',expression,'--format','json']
            if item['profile']=='html':argv+=['--features','html']
            r.require(o['argv']==argv,'OBSERVATION_ARGV',[item['path'],route])
            r.require(o['cwd']==str(c.ROOT),'OBSERVATION_CWD',[item['path'],route])
            r.require(entry['expression']==expression and entry['expression_sha256']==c.sha(expression.encode()),'OBSERVATION_SOURCE',[item['path'],route])
            parts=route.split('.')
            r.require(entry['ancestors']==['.'.join(parts[:i]) for i in range(1,len(parts))],'ANCESTOR_IDENTITY',route)
            result='EXECUTION_UNKNOWN'
            if not o.get('unknown') and isinstance(o.get('stdout'),str) and isinstance(o.get('stderr'),str):
                if o['exit_code']==1 and o['stderr'].startswith('error:'):result='DIAGNOSTIC'
                if o['exit_code']==0:
                    try:
                        value=json.loads(o['stdout'])
                        if isinstance(value,str) and value.startswith('(') and value.endswith(')'):result='VALUE'
                    except ValueError:pass
            r.require(entry['observation']==result,'OBSERVATION_CLASS',[item['path'],route])
            if result=='EXECUTION_UNKNOWN':r.unknown('INVENTORY_EXECUTION_UNKNOWN',[item['path'],route])
            observed_counts[result]=observed_counts.get(result,0)+1
        summaries.append(dict(side=item['side'],profile=item['profile'],raw_routes=len(raw),declared_additional_routes=len(variants-set(raw)),observed_routes=len(expected),observed_counts=observed_counts,max_observed_path_components=max(len(x.split('.')) for x in expected)))
    c.merge_review(r,c.verify_catalog(catalog,reconciliation,history,inventories))
    return dict(at=c.utc(),role='D',manifest_sha256=c.digest(D/'p1335-manifest.json'),checker_sha256=c.digest(__file__),core_sha256=c.digest(D/'p1335-review-check.py'),inputs=c.INPUTS,summaries=summaries,
        limits=['Finite current structural metadata and declared Symbol variants, confirmed by CLI; does not enumerate all expressions.',
        'Crystalline adapter depth sentinel can be skipped by failed lookup (lab/surface-inventory/src/main.rs:106); no claim of unlimited depth or proof from sentinel absence. Current observed routes have at most six components.'],**r.result())

if __name__=='__main__':print(json.dumps(run(),indent=2,ensure_ascii=False))
