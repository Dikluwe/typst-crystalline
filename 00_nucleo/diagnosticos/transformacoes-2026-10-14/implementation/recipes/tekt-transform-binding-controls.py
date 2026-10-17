from pathlib import Path
import subprocess,json,gzip,datetime,hashlib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14/binding-controls';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out.parent/'before.json.gz','rt') as f:base=json.load(f)
rows=[]
queries={
 'rotate-named-angle':'#metadata(repr(rotate(angle:23deg)[X].fields())) <probe>',
 'set-rotate-positional':'#set rotate(23deg)\n#metadata(repr(rotate[X].fields())) <probe>',
 'context-rotate-default':'#context metadata(repr(rotate.angle)) <probe>',
 'context-rotate-style':'#set rotate(23deg)\n#context metadata(repr(rotate.angle)) <probe>',
 'context-rotate-origin':'#set rotate(origin:top+left)\n#context metadata(repr(rotate.origin)) <probe>',
 'context-scale-x':'#set scale(x:150%)\n#context metadata(repr((scale.x,scale.y))) <probe>',
 'set-scale-factor':'#set scale(150%)\n#metadata(repr(scale[X].fields())) <probe>',
 'set-skew-positional':'#set skew(12deg)\n#metadata(repr(skew[X].fields())) <probe>',
 'scale-explicit-zero':'#metadata(repr(scale(x:0%)[X].fields())) <probe>',
}

for name,src in queries.items():
 p=out/(name+'.typ');p.write_text(src)
 for key,info in base['binaries'].items():
  cmd=[info['path'],'query',str(p),'<probe>','--field','value','--one'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  rows.append(dict(case=name,kind='query',binary=key,command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,source=src))
assert all(sha(p)==h for p,h in base['files'].items());assert all(sha(v['path'])==v['sha256'] for v in base['binaries'].values())
(out/'results.json').write_text(json.dumps(dict(utc=now(),before_sha256=sha(out.parent/'before.json.gz'),rows=rows,sources_unchanged=True),indent=2));print('controls done')
