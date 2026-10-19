from pathlib import Path
import subprocess,json,hashlib
R=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent;W=Path('/tmp/watch-catalogo-2026-10-07/html-baseline');W.mkdir(parents=True,exist_ok=True)
bins={'previous':Path('/tmp/watch-catalogo-2026-10-07/previous-96f22a1d'),'current':R/'target/release/typst'}
fixtures=[]
for name in ('p1171-1-html-br','p1173-1-html-global-only-2','p1174-1-html-global-only-3','p1176-1-html-residual-normal','p1177-1-html-ruby','p1178-1-html-documento'):
 choices=list(Path('/tmp').glob('typst-passo-114-'+name+'-*.typ'));p=max(choices,key=lambda p:p.stat().st_mtime_ns);fixtures.append((name,p.read_text(),'html','html'))
fixtures.append(('p1485-cross-introspection','#document("one.html")[= One <one>\n#context query(heading).len()]\n#document("two.html")[= Two <two>\n= Three <three>\n#context query(heading).len()]','bundle','bundle,html'))
rows=[]
for name,source,format_,features in fixtures:
 inp=W/(name+'.typ');inp.write_text(source);results={}
 for engine,binary in bins.items():
  out=W/(name+'-'+engine+('.html' if format_=='html' else '.bundle'))
  cmd=[str(binary),'compile',str(inp),str(out),'--format',format_,'--features',features];p=subprocess.run(cmd,cwd=R,text=True,capture_output=True)
  files={str(x.relative_to(out)):x.read_text() for x in out.rglob('*.html')} if out.is_dir() else {'html':out.read_text()} if out.exists() else {}
  results[engine]={'command':cmd,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'html':files}
 assert results['previous']['exit']==results['current']['exit'];assert results['previous']['stderr']==results['current']['stderr'];assert results['previous']['html']==results['current']['html']
 rows.append({'case':name,'source':source,'source_sha256':hashlib.sha256(source.encode()).hexdigest(),'results':results,'same_before_change':True})
(D/'html-baseline.json').write_text(json.dumps({'binaries':{k:hashlib.sha256(v.read_bytes()).hexdigest() for k,v in bins.items()},'cases':rows},indent=2)+'\n');print('All seven HTML/bundle failures reproduce with frozen prior binary; current outputs identical.')
