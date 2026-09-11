"""Provenance-complete wrapper of the immutable public predicate recipe.
Pure observation checking; no compiler execution or expectation modification.
"""
import argparse, hashlib, importlib.util, json
from pathlib import Path
P=Path(__file__).with_name('p1339-ab-batch1-public-predicate.py')
spec=importlib.util.spec_from_file_location('p1339_public_predecessor',P)
prior=importlib.util.module_from_spec(spec);spec.loader.exec_module(prior)
require=prior.require
REFERENCE_BINARIES={
 'vanilla':{'path':'/usr/local/bin/typst','sha256':'7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'},
 'baseline':{'path':'/tmp/p1338-target.vlNAmp/release/typst','sha256':'f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'}}

def check(raw,purpose,ids,focal=None,derived=None):
 for product in ('vanilla','baseline'):
  if product in raw['binaries']:
   for key,value in REFERENCE_BINARIES[product].items():
    require(raw['binaries'][product][key]==value,('ratified binary identity',product,key))
 plan=prior.root('p1339-ab-batch1-cli-plan.json');lookup={c['id']:c for c in plan['cases']}
 definitions=prior.root('p1339-positive-oracles.json')['oracles']+prior.root('p1339-opaque-oracles.json')['oracles']+prior.root('p1339-opaque-oracles.json')['conditional_angle_controls']
 source_by_id={c['id']:c['source_ref_or_literal_with_sha256'] for c in definitions}
 for row in raw['rows']:
  c=lookup[row['id']];source=source_by_id[row['id']]
  require(row['source_sha256']==source['sha256'],'exact frozen source hash for eval and named compile/query')
  require(row['argv'][0]==raw['binaries'][row['product']]['path'],'actual executable path')
  require(row['cwd']=='/repos/Antigravity/typst-crystalline','frozen working directory')
  require(row['env']['NO_COLOR']=='1' and row['env']['TERM']=='dumb','uncolored diagnostic environment')
  require(row['mode']==c['mode'],'actual transport mode')
  if 'path' in source:
   p=Path(source['path']);require(p.resolve().parent==prior.D,'source outside frozen diagnostics')
   require(hashlib.sha256(p.read_bytes()).hexdigest()==source['sha256'],'actual named source bytes')
  if row['id']=='opaque-budget-workload':
   require(c['timeout_seconds']==1,'frozen opacity deadline')
   require(type(row['timeout']) is bool,'raw timeout flag is boolean')
 return prior.check(raw,purpose,ids,focal,derived)

if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--raw',required=True);p.add_argument('--purpose',choices=['calibration','candidate','mutation_host'],required=True)
 p.add_argument('--ids',required=True);p.add_argument('--focal');p.add_argument('--focal-sha256');p.add_argument('--derived');p.add_argument('--derived-sha256');a=p.parse_args()
 raw=json.loads(Path(a.raw).read_text())
 focal=prior.pinned(a.focal,a.focal_sha256) if a.focal else None
 derived=prior.pinned(a.derived,a.derived_sha256) if a.derived else None
 if derived is not None:require(derived['raw']=={'path':str(Path(a.focal).resolve()),'sha256':a.focal_sha256},'derived raw identity')
 for result in check(raw,a.purpose,a.ids.split(','),focal,derived):print(json.dumps(result,ensure_ascii=True))
