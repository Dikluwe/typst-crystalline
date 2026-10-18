from pathlib import Path
s=Path(__file__).with_name('measure.py').read_text();exec(compile(s[:s.index("\nflat_eval('tekt-v16")],str(Path(__file__).with_name('measure.py')),'exec'))
cases={
 'span-equality-file':'#assert.eq(heading(supplement:[A])[B],heading(supplement:[A])[B])',
 'supplement-inequality-file':'#assert.ne(heading(supplement:[A])[B],heading(supplement:[C])[B])',
 'stretch-leaf-valid':'A $stretch(arrow.t,size:#200%)$ Z',
 'stretch-leaf-natural-valid':'A $stretch(arrow.t,size:#100%)$ Z',
 'stretch-attach-valid':'A $stretch(arrow.t^2,size:#200%)$ Z',
 'stretch-attach-natural-valid':'A $stretch(arrow.t^2,size:#100%)$ Z',
 'stretch-attached-outside-valid':'A $stretch(arrow.t,size:#200%)^"map"$ Z',
 'stretch-horizontal-valid':'A $stretch(=,size:#200%)$ Z',
 'stretch-horizontal-attach-valid':'A $stretch(=^"long attachment",size:#200%)$ Z',
 'stretch-mixed-valid':'A $stretch(arrow.t,size:#(100%+2em))$ Z',
 'bibliography-selector-control':'#show bibliography: it=>it\n@audit\n#bibliography("audit.bib",style:"mla",title:none)',
 'bibliography-where-indent':'#show bibliography.where(): set par(hanging-indent:3em)\n@audit\n#bibliography("audit.bib",style:"mla",title:none)',
 'hanging-indent-par-control':'#set par(hanging-indent:3em)\nFIRST Words in this paragraph force a line break and reveal whether hanging indent is accepted at all by the paragraph constructor.',
}
for name,body in cases.items():
 p=work/(name+'.typ');p.write_text('#set page(width:200pt,height:auto,margin:8pt)\n'+body);add_compile('focal',name,p,third=True)
records=[]
for i,spec in enumerate(specs):
 results={}
 for name in ['tekt','vanilla','vanilla_old']:
  args=spec['args'][:];folder=work/f'focal-{i:03d}-{name}';folder.mkdir(exist_ok=True);dest=folder/'result.pdf';args[args.index('OUTPUT')]=str(dest);r=execute([bins[name]['path'],*args]);observe_file(dest,r);results[name]=r
 records.append(dict(spec,results=results));print(spec['label'],{n:(r['exit'],r['errors']) for n,r in results.items()},flush=True)
with gzip.open(out/'focal.json.gz','wt') as f:json.dump({'utc':now(),'cases':records,'fixtures':fixtures},f,ensure_ascii=False)
