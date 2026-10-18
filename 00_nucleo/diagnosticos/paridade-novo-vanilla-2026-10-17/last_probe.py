from pathlib import Path
s=Path(__file__).with_name('measure.py').read_text();exec(compile(s[:s.index("\nflat_eval('tekt-v16")],str(Path(__file__).with_name('measure.py')),'exec'))
for amount in [None,'1.5em','3em']:
 name='bibliography-where-'+str(amount);p=work/(name+'.typ');setting='' if amount is None else '#show bibliography.where(): set par(hanging-indent:'+amount+')\n'
 p.write_text('#set page(width:200pt,height:auto,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'+setting+'OUTSIDE Words outside the bibliography must retain their position and wrap consistently without a hanging indent.\n\n@audit\n\n#bibliography("audit.bib",style:"mla",title:none)');add_compile('bibliography-where',name,p,third=True)
for name,body in {'cancel-function-direct':'#let compute(angle)=panic("CALLED")\n#math.cancel($x$,angle:compute)', 'cancel-function-math':'#let compute(angle)=panic("CALLED")\n$cancel(x,angle:#compute)$','state-show-value':'#let update(n)=n+1\n#state("audit",0).update(update)\n#context repr(state("audit",0).final())'}.items():
 p=work/(name+'.typ');p.write_text(body);add_compile('callback-isolation',name,p,third=True)
records=[]
for i,spec in enumerate(specs):
 results={}
 for name in ['tekt','vanilla','vanilla_old']:
  args=spec['args'][:];folder=work/f'last-{i:03d}-{name}';folder.mkdir(exist_ok=True);dest=folder/'result.pdf';args[args.index('OUTPUT')]=str(dest);r=execute([bins[name]['path'],*args]);observe_file(dest,r);results[name]=r
 records.append(dict(spec,results=results));print(spec['label'],{n:(r['exit'],r['errors']) for n,r in results.items()},flush=True)
with gzip.open(out/'last.json.gz','wt') as f:json.dump({'utc':now(),'cases':records,'fixtures':fixtures},f,ensure_ascii=False)
