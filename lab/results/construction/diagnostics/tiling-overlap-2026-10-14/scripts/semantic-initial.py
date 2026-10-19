from pathlib import Path
import subprocess,json,hashlib,datetime,gzip
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tiling-overlap-2026-10-14/semantic';out.mkdir(exist_ok=False)
exe=root/'target/debug/typst';sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
source='''#set document(title: [Tiling opacity])
#set page(width: 120pt, height: 90pt, margin: 5pt)
#let nested = tiling(size: (6pt, 6pt))[#rect(width: 3pt, height: 3pt, fill: rgb(255,0,0,50%), stroke: none)]
#let tile = tiling(size: (12pt, 12pt), spacing: (-2pt,-2pt))[#rect(width: 8pt, height: 8pt, fill: nested, stroke: none)#text(size:4pt)[CELL]]
#text(fill:rgb(0,0,255,50%))[PAGE ONE]
#rect(width: 50pt, height: 30pt, fill: tile, stroke: none)
#pagebreak()
#text(fill:rgb(0,0,255,50%))[PAGE TWO]
#rect(width: 50pt, height: 30pt, fill: tile, stroke: none)
'''
src=out/'input.typ';src.write_text(source)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source=source,binary_sha256=sha(exe),source_snapshot='candidate-final.json.gz',rows=[])
for mode in ['verbose','compact']:
 for tags in ['default','off','ua1','selected']:
  name=f'{mode}-{tags}';dest=out/(name+'.pdf');opts=(['--compact'] if mode=='compact' else [])+({'default':[],'off':['--no-pdf-tags'],'ua1':['--pdf-standard','ua-1'],'selected':['--pages','2']}[tags]);row=dict(name=name,commands=[])
  for cmd in [[str(exe),'compile',str(src),str(dest),*opts],['qpdf','--check',str(dest)],['pdftotext',str(dest),'-']]:
   r=subprocess.run(cmd,capture_output=True,text=True);row['commands'].append(dict(command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr));assert r.returncode==0,row
  text=row['commands'][-1]['stdout'];assert 'PAGE TWO' in text and 'CELL' not in text
  assert ('PAGE ONE' in text)==(tags!='selected'),text
  row['sha256']=sha(dest);row['passed']=True;record['rows'].append(row)
(out/'result.json').write_text(json.dumps(record,indent=2));assert sha(exe)==record['binary_sha256'];print('Semantic, page selection, compact/verbose and UA1 checks passed')
