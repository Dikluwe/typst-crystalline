"""Attribute samples by stack presence, removing catalog samples before eval residuals."""
from pathlib import Path
import gzip,json,collections,hashlib
D=Path(__file__).resolve().parent
x=json.load(gzip.open(D/'cpu-style-current.json.gz','rt'))
summary={}
for case,entry in x['summary'].items():
 groups=collections.Counter();leaves=collections.Counter();inclusive=collections.Counter()
 for row in x['runs']:
  if row['fixture']!=case:continue
  for line in row['stacks']:
   fs=[x['symbols'][pc] for pc in line.split()]
   for name,term in [('clones','clone..Clone'),('collapse','StyleChain::collapse'),('capture','local_delta_since'),('source_span','with_source_span'),('eval','compiler::eval')]:
    if any(term in f for f in fs):inclusive[name]+=1
   catalog=any('world::fonts::fresh_catalog' in f or 'fontdb::selective' in f or 'fonts::coverage::extract' in f or 'fontdb::Database::load_font' in f for f in fs)
   if catalog:groups['catalog']+=1;continue
   if any('compiler::eval' in f for f in fs):group='eval_residual'
   elif any('compiler::layout' in f for f in fs):group='layout'
   elif any('shaper' in f for f in fs):group='shape'
   elif any('export::' in f for f in fs):group='export'
   else:group='other'
   groups[group]+=1
   if group=='eval_residual':
    for f in fs[2:]:
     if 'typst_core' in f or 'typst_infra' in f:leaves[f]+=1;break
 assert sum(groups.values())==entry['total']
 summary[case]={'total':entry['total'],'partition':dict(groups),'inclusive':dict(inclusive),'eval_residual_first_project_frame':leaves.most_common(20),'catalog_fraction':groups['catalog']/entry['total']}
result={'input_binary_sha256':x['binary_sha256'],'input_sha256':hashlib.sha256((D/'cpu-style-current.json.gz').read_bytes()).hexdigest(),'driver_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'summary':summary,'scope':'Stack-presence grouping, diagnostic sampling. Categories are assigned by stated priority and do not certify exclusive time in individual functions; absence can result from inlining or stack depth limits.'}
(D/'cpu-classification.json').write_text(json.dumps(result,indent=2))
for n,s in summary.items():print(n,round(100*s['catalog_fraction'],1),s['partition'])
