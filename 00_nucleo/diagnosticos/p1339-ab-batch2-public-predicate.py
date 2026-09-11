"""Final-batch observation translation successor, no product execution.
Uses unchanged transport/provenance/tuple helpers from immutable predecessors;
replaces only frozen wrapper/plan lookup and causal show-effect evaluation.
"""
import argparse, hashlib, importlib.util, json
from pathlib import Path
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('p1339_public_v2',D/'p1339-ab-batch1-public-predicate-v2.py')
v2=importlib.util.module_from_spec(spec);spec.loader.exec_module(v2)
base=v2.prior;require=base.require
ALIASES={'p1339-positive-oracles.json': ('p1339-ab-batch2-positive-oracles.json', '2d33aac0f559d377b4275b07a1cdc77997cc90b9d887fdabf5ff9bccf28ab55f'), 'p1339-opaque-oracles.json': ('p1339-ab-batch2-opaque-oracles.json', 'dc612c50cf80e7eedd6072d8956ed3534f09c0c0eba5a16edbce7b95256db182'), 'p1339-ab-batch1-cli-plan.json': ('p1339-ab-batch2-cli-plan.json', 'e36f89070d5de5c9ac9e340f443255797aea109a42b8cfe71e448bc34dcb380b')}
def root(name):
 target,sha=ALIASES[name]
 return base.pinned(D/target,sha)
base.root=root
for old,(new,sha) in ALIASES.items():base.PINS[old]=sha
old_evaluate=base.evaluate
def evaluate(row,purpose,positive,opaque,focal=None,derived=None):
 lookup={o['id']:o for o in positive['oracles']}
 o=lookup.get(row['id'])
 if o is None or not o.get('show_primary_effect_policy'):
  return old_evaluate(row,purpose,positive,opaque,focal,derived)
 if row['execution']!='Observed' or row['unknown_reason'] is not None:
  return {'id':row['id'],'raw_execution':row['execution'],'classification':'Unknown','reason':'mandatory_unknown','positive_credit':0}
 wanted=base.expected_for(o,row,purpose,focal,derived)
 actual=base.tuple_of(row,wanted)
 differences=[{'dimension':d,'expected':wanted[d],'actual':actual[d]} for d in wanted if actual[d]!=wanted[d]]
 policy=o['show_primary_effect_policy']
 reference_product=row['product'] if purpose=='calibration' else policy['phase_reference_product'][purpose]
 chosen=policy['by_reference_product'][reference_product][row['profile']]
 effect=chosen['effect'];primary=row['stderr'].split('\n',1)[0]
 exact_panic='error: panicked with: '+o['show_marker']
 if effect=='CallbackExecuted':valid=primary==exact_panic and primary==chosen['primary_diagnostic_line'] and row['exit']==1
 elif effect=='CompileSucceededNoCallback':valid=row['exit']==0 and row['stderr']=='' and row['pdf_exists'] is True
 elif effect=='OtherDiagnosticBeforeCallback':valid=row['exit']==1 and primary==chosen['primary_diagnostic_line'] and primary!=exact_panic
 else:require(False,'unknown causal effect variant')
 if not valid:differences.append({'dimension':'causal_show_effect','expected':chosen,'actual':{'primary_diagnostic_line':primary,'exit':row['exit'],'pdf_exists':row['pdf_exists']}})
 return {'id':row['id'],'raw_execution':'Observed','classification':'Violated' if differences else 'Preserved','reason':'observable_difference' if differences else 'exact_designated_reference','differences':differences,'positive_credit':0 if differences else 1,'show_effect':{'reference_product':reference_product,'expected':effect,'actual_primary_line':primary,'matched':valid}}
base.evaluate=evaluate
def check(raw,purpose,ids,focal=None,derived=None):return v2.check(raw,purpose,ids,focal,derived)
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--raw',required=True);p.add_argument('--purpose',choices=['calibration','candidate','mutation_host'],required=True)
 p.add_argument('--ids',required=True);p.add_argument('--focal');p.add_argument('--focal-sha256');p.add_argument('--derived');p.add_argument('--derived-sha256');a=p.parse_args()
 raw=json.loads(Path(a.raw).read_text())
 focal=base.pinned(a.focal,a.focal_sha256) if a.focal else None
 derived=base.pinned(a.derived,a.derived_sha256) if a.derived else None
 if derived is not None:require(derived['raw']=={'path':str(Path(a.focal).resolve()),'sha256':a.focal_sha256},'derived raw identity')
 for result in check(raw,a.purpose,a.ids.split(','),focal,derived):print(json.dumps(result,ensure_ascii=True))
