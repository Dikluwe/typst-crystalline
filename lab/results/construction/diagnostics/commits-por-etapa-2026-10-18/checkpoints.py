from pathlib import Path
import subprocess,json,gzip,hashlib,tarfile,shutil,datetime
root=Path.cwd();out=Path(__file__).resolve().parent;checkpoint=out/'checkpoint'
plan=json.loads(gzip.decompress((out/'commit-plan.json.gz').read_bytes()))
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==plan['base_head']
assert subprocess.run(['git','diff','--cached','--quiet']).returncode==0
if checkpoint.exists():shutil.rmtree(checkpoint)
checkpoint.mkdir()
for name in ['crystalline.toml','Cargo.toml','Cargo.lock','01_core/Cargo.toml','02_shell/Cargo.toml','03_infra/Cargo.toml','04_wiring/Cargo.toml','benches/Cargo.toml','tools/ci/check_reverse_lineage.py']:
 dest=checkpoint/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(root/name,dest)
proc=subprocess.Popen(['git','archive','--format=tar','HEAD','00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring'],stdout=subprocess.PIPE)
with tarfile.open(fileobj=proc.stdout,mode='r|') as tf:
 for item in tf:
  if item.isfile() and (item.name.startswith('00_nucleo/prompts/') or item.name.endswith('.rs')):
   dest=checkpoint/item.name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(tf.extractfile(item).read())
while proc.stdout.read(1024*1024):pass
assert proc.wait()==0
pin='/tmp/tekt-linter-none-target/debug/crystalline-lint';receipts=[]
for stage in plan['stages']:
 for name,text in stage['source_changes'].items():
  dest=checkpoint/name
  if text is None:dest.unlink(missing_ok=True)
  else:dest.parent.mkdir(parents=True,exist_ok=True);dest.write_text(text)
 assert all(hashlib.sha256((checkpoint/f).read_bytes()).hexdigest()==h for f,h in stage['hashes'].items())
 print(stage['key'],'lineage',flush=True);logs={}
 for label,cmd in [('forward',[pin,str(checkpoint),'--checks','v1,v5,v15,v26','--fail-on','warning']),('reverse',['python3',str(checkpoint/'tools/ci/check_reverse_lineage.py')])]:
  z=subprocess.run(cmd,text=True,capture_output=True);logs[label]=dict(command=cmd,exit=z.returncode,stdout=z.stdout,stderr=z.stderr)
  assert z.returncode==0,(stage['key'],label,z.stdout[-5000:],z.stderr[-2000:])
 receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),base_head=plan['base_head'],stage=stage['key'],source_tree_sha256=hashlib.sha256(json.dumps(stage['hashes'],sort_keys=True).encode()).hexdigest(),files=len(stage['hashes']),logs=logs,no_build_or_test_rerun=True,method='Exact historical source checkpoint reconstructed outside working tree; strict lineage and reverse seals rerun.')
 path=root/'00_nucleo/diagnosticos'/stage['diagnostic']/'commit-lineage.json.gz';path.write_bytes(gzip.compress(json.dumps(receipt,ensure_ascii=False,indent=2).encode()))
 receipts.append(dict(receipt=str(path.relative_to(root)),**receipt))
(out/'checkpoint-validation.json.gz').write_bytes(gzip.compress(json.dumps(receipts,ensure_ascii=False,indent=2).encode()))
assert all(hashlib.sha256((root/f).read_bytes()).hexdigest()==h for f,h in plan['final_hashes'].items())
print('Validated',len(receipts),'stage checkpoints without modifying production working tree',flush=True)
