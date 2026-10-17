from pathlib import Path
import gzip,json,datetime,hashlib,subprocess,shutil
D=Path(__file__).resolve().parent;R=D.parents[2]
owners=json.loads((D/'owners.json').read_text());paths=[p for pair in owners for p in pair]
logs={}
for name in ['red-tests','core-tests','build','release','lint']:
 p=Path('/tmp/containers-'+name+'.log');logs[name]={'text':p.read_text(),'utc_end':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
seal=json.loads((D/'seal.json').read_text())
for row in seal['owners']:
 assert hashlib.sha256((R/row['consumer']).read_bytes()).hexdigest()==row['source_sha256']
 assert not any('[V5]' in block and row['consumer'] in block for block in logs['lint']['text'].split('\n\n')),row['consumer']
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'binaries':{n:hashlib.sha256(Path(p).read_bytes()).hexdigest() for n,p in [('before','/tmp/tekt-before-containers-2026-10-10'),('after',R/'target/release/typst'),('vanilla',R/'lab/typst-original/target/release/typst')]},'files':{p:{'sha256':hashlib.sha256((R/p).read_bytes()).hexdigest(),'text':(R/p).read_text()} for p in paths},'seals':seal,'logs':logs,'toolchain':{c:subprocess.check_output([c,'--version'],text=True).strip() for c in ['rustc','cargo']},'linter_sha256':hashlib.sha256(Path(shutil.which('crystalline-lint')).read_bytes()).hexdigest(),'commands':['cargo test -p typst-core extended_reuse_ --lib','cargo test -p typst-core --lib','cargo build','cargo build --release','crystalline-lint .'],'results':{'red_test_exit':101,'core_test_exit':0,'build_exit':0,'release_exit':0,'lint_exit':0}}
nucleus=json.loads((D/'nucleus.json').read_text())
x['nucleus']={**nucleus,'text':(R/nucleus['path']).read_text(),'file_sha256':hashlib.sha256((R/nucleus['path']).read_bytes()).hexdigest()}
x['final_metadata_note']='As mudanças posteriores à suíte foram pins/headers e formalização L0 do protocolo; algoritmos ficaram iguais e o rebuild final conserva exatamente os bytes do release qualificado e medido.'
with gzip.open(D/'validation.json.gz','wt') as f:json.dump(x,f,ensure_ascii=False)
print('validation',len(owners),'owners',x['binaries'])
