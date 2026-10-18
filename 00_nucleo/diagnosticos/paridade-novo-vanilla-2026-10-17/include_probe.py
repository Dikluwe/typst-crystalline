from pathlib import Path
s=Path(__file__).with_name('measure.py').read_text();exec(compile(s[:s.index("\nflat_eval('tekt-v16")],str(Path(__file__).with_name('measure.py')),'exec'))
p=work/'valid_defs.typ';p.write_text('#let x=42\n#let f(y)=y+1\n');remember(p)
for route in ['"valid_defs.typ"','path("valid_defs.typ")']:
 for suffix,body in [('render','#let a=include '+route+'\nL#a;R'),('eq','#repr((include '+route+') == [])')]:
  name=('path' if route.startswith('path') else 'string')+'-'+suffix;p=work/('include-'+name+'.typ');p.write_text(body);add_compile('include-morphology',name,p,'html',['html'],True)
records=[]
for i,spec in enumerate(specs):
 results={}
 for name in ['tekt','vanilla','vanilla_old']:
  args=spec['args'][:];folder=work/f'include-{i:03d}-{name}';folder.mkdir(exist_ok=True);dest=folder/'result.html';args[args.index('OUTPUT')]=str(dest);r=execute([bins[name]['path'],*args]);observe_file(dest,r);results[name]=r
 records.append(dict(spec,results=results));print(spec['label'],{n:(r['exit'],r.get('body_text'),r['errors']) for n,r in results.items()},flush=True)
with gzip.open(out/'include.json.gz','wt') as f:json.dump({'utc':now(),'cases':records,'fixtures':fixtures},f,ensure_ascii=False)
