from pathlib import Path
import subprocess,json,gzip,datetime,hashlib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14/controls';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out.parent/'before.json.gz','rt') as f:base=json.load(f)
rows=[]
queries={
 'body-none':'#metadata(repr(rotate(23deg,none).fields())) <probe>',
 'body-content-number':'#metadata(repr(rotate(23deg,12).fields())) <probe>',
 'style-presence':'#set rotate(angle:23deg,origin:top+left)\n#metadata(repr(rotate[X].fields())) <probe>',
 'style-fold-origin':'#set rotate(origin:top+left)\n#metadata(repr(rotate(origin:bottom)[X].fields())) <probe>',
 'scale-order':'#metadata(repr(scale("X",150%).fields())) <probe>',
 'rotate-order':'#metadata(repr(rotate("X",23deg).fields())) <probe>',
 'scale-explicit-zero':'#metadata(repr(scale(x:0%).fields())) <probe>',
 'scale-factor-auto':'#metadata(repr(scale(auto,x:20pt)[X].fields())) <probe>',
 'rotate-context-read':'#set rotate(angle:23deg)\n#context metadata(repr(rotate.angle)) <probe>',
}
for name,src in queries.items():
 p=out/(name+'.typ');p.write_text(src)
 for key,info in base['binaries'].items():
  cmd=[info['path'],'query',str(p),'<probe>','--field','value','--one'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  rows.append(dict(case=name,kind='query',binary=key,command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,source=src))
for name,src in {'rotate':'#rotate(23deg)[TEXT]', 'move':'#move(dx:1pt)[TEXT]', 'scale':'#scale(150%)[TEXT]', 'skew':'#skew(ax:12deg)[TEXT]'}.items():
 p=out/(name+'-html.typ');p.write_text(src)
 for key,info in base['binaries'].items():
  dest=out/(name+'-'+key+'.html');cmd=[info['path'],'compile','--features','html',str(p),str(dest)];r=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  rows.append(dict(case=name,kind='html',binary=key,command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,source=src,output=dest.read_text() if dest.exists() else None))
assert all(sha(p)==h for p,h in base['files'].items());assert all(sha(v['path'])==v['sha256'] for v in base['binaries'].values())
(out/'results.json').write_text(json.dumps(dict(utc=now(),before_sha256=sha(out.parent/'before.json.gz'),rows=rows,sources_unchanged=True),indent=2));print('controls done')
