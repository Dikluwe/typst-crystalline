from pathlib import Path
import subprocess,json,datetime,hashlib,gzip
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};assert hashes==b['hashes'];cmd=['cargo','build','--offline','--locked','--workspace','--message-format=json'];
with (out/'inventory-build.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
assert r.returncode==0;artifact=None
for line in (out/'inventory-build.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')=='compiler-artifact' and x['target']['name']=='typst_core':artifact=next((p for p in x['filenames'] if p.endswith('.rlib')),artifact)
assert artifact
cmd=['rustc','--edition=2021',str(out/'inventory.rs'),'--extern','typst_core='+artifact,'-L','dependency=target/debug/deps','-o',str(out/'inventory-bin')]
for p in sorted(Path('target/debug/build').glob('*/out')):cmd+=['-L','native='+str(p)]
with (out/'inventory-rustc.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
assert r.returncode==0
with (out/'inventory.tsv').open('w') as f, (out/'inventory-stderr.log').open('w') as e:r=subprocess.run([str(out/'inventory-bin')],stdout=f,stderr=e)
assert r.returncode==0 and not (out/'inventory-stderr.log').read_text();rows=[dict(profile=int(p),expression=n,kind=k,name=label) for line in (out/'inventory.tsv').read_text().splitlines() for p,n,k,label in [line.split('\t')]];assert rows
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,inventory_program_sha256=sha(out/'inventory.rs'),rustc_argv=cmd,linked_rlib=dict(path=artifact,sha256=sha(artifact)),rows=rows,sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert record['sources_unchanged']
with gzip.open(out/'inventory.json.gz','wt') as f:json.dump(record,f)
print(dict(rows=len(rows),unique_paths=len({x['expression'] for x in rows}),function_paths=len({x['expression'] for x in rows if x['kind']=='function'})))
