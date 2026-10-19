from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,collections
# Reuse only the local measurement definitions, not its execution/corpora.
s=Path(__file__).with_name('measure.py').read_text();exec(compile(s[:s.index("\nflat_eval('tekt-v16")],str(Path(__file__).with_name('measure.py')),'exec'))
extra={
 'negative-min-braced':'{-9223372036854775808}','negative-group-braced':'{-(9223372036854775808)}','positive-overflow-type':'type(9223372036854775808)','hex-overflow':'0x8000000000000000','float-overflow-control':'type(9223372036854775808.0)',
 'text-field':'[A].text','text-has-field':'[A].has("text")','text-fields':'[A].fields()','raw-field':'raw("A").text',
 'h-amount':'h(50%+3pt).amount','box-width':'box(width:50%+3pt)[A].width','block-width':'block(width:50%+3pt)[A].width',
 'state-default-init':'state("audit")','state-explicit-init':'state("audit",none)',
 'counter-equation-alias':'{let e=math.equation;counter(e)}','counter-equation-filter':'counter(math.equation.where(block:true))',
 'heading-supplement-type':'heading(supplement:[A])[B].supplement',
 'typed-symbol-counterexample':'{let a=[$alpha$];(a.func(),a.fields())}',
 'negative-padding':'pad(left:-2pt)[A]',
}
for name,expr in extra.items():add_eval('supplement-eval',name,expr,third=True)
prefix='#set page(width:200pt,height:200pt,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'
bodies={
 'h-ratio':'L#h(50%)R','h-absolute-control':'L#h(92pt)R','box-ratio':'#box(width:50%)[A]Z','box-absolute-control':'#box(width:92pt)[A]Z',
 'block-ratio':'#block(width:50%)[#rect(width:100%,height:10pt,fill:red,stroke:none)]\nZ','block-absolute-control':'#block(width:92pt)[#rect(width:100%,height:10pt,fill:red,stroke:none)]\nZ',
 'v-ratio':'A\n#v(50%)\nB','v-absolute-control':'A\n#v(92pt)\nB',
}
for name,body in bodies.items():
 p=work/(name+'.typ');p.write_text(prefix+body);add_compile('relative-render',name,p,third=True)
for f in ['after-measurement.json.gz','supplement-after-measurement.json.gz']:
 p=root/'00_nucleo/diagnosticos/par-box-fronteira-2026-10-17'/f
 if p.exists():
  for c in load(p)['cases']:
   if c.get('binary')=='tekt' and c['name']=='empty-fr-box':add_compile('box-fr',c['name'],c['source'],third=True)
for name,body in {
 'local-cjk':'#let part = [\n#show "水果": [X]\n水\n果\n]\n#part',
 'root-cjk-control':'#show "水果": [X]\n水\n果',
 'identical-rules':'#show "a": [X]\n#show "a": [Y]\na',
 'local-identical-rules':'#let part=[#show "a": [X]\n#show "a": [Y]\na]\n#part',
}.items():
 p=work/(name+'.typ');p.write_text(body);add_compile('matching',name,p,'html',['html'],True)
for fmt in ['"html"','format.html','auto']:
 p=work/('document-format-'+fmt.replace('"','')+'.typ');p.write_text('#document("child.html",format:'+fmt+')[A]')
 add_compile('document-format',fmt,p,'bundle',['bundle','html'],True)
# State callbacks with explicit init isolate their execution from default-init debt.
for name,body in {'state-callback':'#let update(n)=panic("CALLED")\n#state("audit",0).update(update)\n#context state("audit",0).final()',
 'heading-supplement-callback':'#set heading(supplement:h=>panic("CALLED"))\n= Heading',
 'heading-fields-callback':'#show heading: it => {assert.eq(it.body.text,"Heading");it.body}\n= Heading'}.items():
 p=work/(name+'.typ');p.write_text(body);add_compile('callbacks',name,p,third=True)
# Long, local MLA entry: evaluate hanging-indent changes within each backend too.
bib=work/'audit.bib';bib.write_text('@book{audit, author={Silva, Ana}, title={BIBWORD A considerably long publication title that must wrap onto several lines for testing indentation}, year={2020}, publisher={Example Press}}\n');remember(bib)
for indent in [None,'1.5em','3em']:
 name='bibliography-'+str(indent);p=work/(name+'.typ');setting='' if indent is None else '#show bibliography: set par(hanging-indent:'+indent+')\n'
 p.write_text('#set page(width:200pt,height:auto,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'+setting+'OUTSIDE Words outside the bibliography must retain their position and wrap consistently without a hanging indent.\n\n@audit\n\n#bibliography("audit.bib",style:"mla",title:none)')
 add_compile('bibliography',name,p,third=True)
for name,expr in {'stretch-leaf':'stretch(arrow.t,size:200%)','stretch-attach':'stretch(arrow.t^2,size:200%)','stretch-sequence':'stretch((arrow.t thin),size:200%)','stretch-natural-attach':'stretch(arrow.t^2,size:100%)'}.items():
 p=work/(name+'.typ');p.write_text('#set page(width:200pt,height:auto,margin:8pt)\nA $'+expr+'$ Z');add_compile('math-stretch',name,p,third=True)
records=[]
for i,spec in enumerate(specs):
 results={}
 for name in ['tekt','vanilla','vanilla_old']:
  args=spec['args'][:]
  if 'OUTPUT' in args:
   folder=work/f'supp-{i:03d}-{name}';folder.mkdir(exist_ok=True);dest=folder/('result.'+spec['fmt']);args[args.index('OUTPUT')]=str(dest)
  r=execute([bins[name]['path'],*args])
  if spec['mode']=='compile':
   observe_file(dest,r)
   if spec['fmt']=='bundle' and dest.is_dir():r['bundle_files']={str(p.relative_to(dest)):p.read_text() for p in dest.rglob('*.html')}
  results[name]=r
 records.append(dict(spec,results=results));print(spec['label'],{n:r['exit'] for n,r in results.items()},flush=True)
assert all(sha(p)==h for p,h in before['source_hashes'].items())
with gzip.open(out/'supplement.json.gz','wt') as f:json.dump({'utc':now(),'cases':records,'source_unchanged':True,'fixtures':fixtures},f,ensure_ascii=False)
print('Supplement pairs',len(records),flush=True)
