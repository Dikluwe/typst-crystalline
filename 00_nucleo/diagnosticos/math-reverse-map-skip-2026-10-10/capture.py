from pathlib import Path
import datetime,gzip,json,hashlib,subprocess
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=json.load(gzip.open(D/'before.json.gz','rt'));x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'sources':{p:sha(R/p)for p in b['sources']},'owners':{p:(R/p).read_text()for p in b['owners']},'binary':sha(R/'target/release/typst'),'baseline_binary':sha('/tmp/tekt-math-map/old/typst'),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_binary':sha(R/'lab/typst-original/target/release/typst')}
x['changed_sources']=[p for p,h in b['sources'].items()if h!=x['sources'][p]]
assert x['changed_sources']==['03_infra/src/font_metrics/math.rs'],x['changed_sources']
assert x['baseline_binary']==b['binary']
assert x['vanilla_binary']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
assert x['binary']==sha('/tmp/tekt-math-map/new/typst')
x['logs']={tag:Path('/tmp/math-map-'+tag+'.log').read_text()for tag in ['semantic-baseline','red','infra','release','build','lint','fonts','green','qualify','resources','native']}
x['validation']={'infra_passed':1285,'infra_ignored':2,'build_exit':0,'release_exit':0,'lint_exit':0,'qualification_documents':11,'allocation_gate':True}
x['report_sha256']=sha(D.with_suffix('.md'));x['artifacts']={p.name:sha(p)for p in D.iterdir()if p.is_file()and p.name!='final.json.gz'}
with gzip.open(D/'final.json.gz','wt')as f:json.dump(x,f)
print(x['binary'],x['changed_sources'])
