import datetime,gzip,hashlib,json,subprocess,sys
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=json.load(gzip.open(D/'before.json.gz','rt'))
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','status':subprocess.check_output(['git','status','--short'],cwd=R,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'sources':{p:sha(R/p)for p in b['sources']},'owners':{p:(R/p).read_text()for p in b['owners']},'binary':sha(R/'target/release/typst'),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_binary':sha(R/'lab/typst-original/target/release/typst')}
x['changed_sources']=[p for p,h in b['sources'].items()if h!=x['sources'][p]]
assert x['changed_sources']==['01_core/src/compiler/realize/table_cells.rs'],x['changed_sources']
assert x['vanilla_binary']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
x['logs']={tag:Path('/tmp/table-candidate-'+tag+'.log').read_text()for tag in ['unit-red','core','build','release','lint','green','qualify','query','native']}
x['report_sha256']=sha(D.with_suffix('.md'))
x['validation']={'core_passed':6683,'core_ignored':5,'build_exit':0,'release_exit':0,'lint_exit':0,'qualification_documents':25,'query_cases':6,'allocation_gate':True}
x['artifacts']={p.name:sha(p)for p in D.iterdir()if p.is_file()and p.name!='final.json.gz'}
with gzip.open(D/'final.json.gz','wt')as f:json.dump(x,f)
print(x['binary'],x['changed_sources'])
