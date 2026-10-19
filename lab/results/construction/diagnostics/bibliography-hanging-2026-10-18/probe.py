from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
import xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={}
base='#set page(width:160pt,height:auto,margin:8pt)\n#set text(size:12pt,lang:"en",hyphenate:false)\n'
words='FIRST alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi omicron pi rho sigma tau LAST'
for direction in ['ltr','rtl']:
 for strategy in ['simple','optimized']:
  for val in ['0pt','10pt','-5pt','2pt+1em']:
   setup=base+f'#set text(dir:{direction})\n'
   cases[f'par-set-{direction}-{strategy}-{val}']=setup+f'#set par(hanging-indent:{val},linebreaks:"{strategy}")\n'+words+'\n\nNEXT'
   cases[f'par-ctor-{direction}-{strategy}-{val}']=setup+f'#par(hanging-indent:{val},linebreaks:"{strategy}")[{words}]\nNEXT'
for strategy in ['simple','optimized']:
 for value in ['0pt','10pt']:
  cases[f'hyphen-{strategy}-{value}']=base+f'#set text(hyphenate:true)\n#set par(justify:true,linebreaks:"{strategy}",hanging-indent:{value})\nrepresentation extraordinary imagination characteristic representation extraordinary imagination characteristic'
 for style in ['mla','ieee','custom.csl']:
  for mode,value in [('default',''),('bare','0pt'),('bare','3em'),('show','0pt'),('show','3em'),('show','-5pt'),('show','2pt+1em')]:
   setup=base+f'#set par(linebreaks:"{strategy}")\n'
   if mode=='bare':setup+=f'#set par(hanging-indent:{value})\n'
   if mode=='show':setup+=f'#show bibliography:set par(hanging-indent:{value})\n'
   cases[f'bib-{strategy}-{style}-{mode}-{value}']=setup+'@audit\n'+f'#bibliography("audit.bib",style:"{style}",title:none)'+'\n\nOUTSIDE'
 for direction in ['ltr','rtl']:
  for height in ['70pt','120pt']:
   cases[f'region-{strategy}-{direction}-{height}']=base+f'#set page(height:{height})\n#set text(dir:{direction})\n#set par(linebreaks:"{strategy}",hanging-indent:10pt)\n'+words+' '+words+'\n\nNEXT'
for body in ['FIRST alpha\\ beta gamma delta epsilon zeta eta theta iota kappa LAST','FIRST *alpha beta* gamma delta epsilon zeta eta theta iota kappa LAST','FIRST #box[alpha beta] gamma delta epsilon zeta eta theta iota kappa LAST']:
 for form in ['set','ctor']:
  name=['forced','styled','box'][len([k for k in cases if k.startswith('scope-')])//2]
  cases[f'scope-{name}-{form}']=base+('#set par(hanging-indent:10pt)\n'+body if form=='set' else f'#par(hanging-indent:10pt)[{body}]')+'\n\nNEXT'
bins={'tekt':p/'bin/typst-before' if sys.argv[1] in ('before','extra-before') else r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'binaries':{k:{'path':str(b),'sha256':sha(b)} for k,b in bins.items()},'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3','cases':[]}
for label,source in cases.items():
 f=p/'outputs'/f'{label}.typ';f.write_text(source);row={'label':label,'source':source,'source_sha256':sha(f),'results':{}}
 for name,binary in bins.items():
  out=f.with_name(f.stem+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=45);txt=subprocess.check_output(['pdftotext',str(out),'-'],text=True) if not z.returncode else ''
  row['results'][name]={'command':cmd,'exit':z.returncode,'stderr':z.stderr,'text':' '.join(txt.split())}
  if not z.returncode:
   bbox=out.with_suffix('.bbox');subprocess.run(['pdftotext','-bbox',str(out),str(bbox)],check=True,capture_output=True)
   ns={'x':'http://www.w3.org/1999/xhtml'};root=ET.parse(bbox).getroot()
   row['results'][name]['pages']=[{'width':float(page.attrib['width']),'height':float(page.attrib['height']),'words':[{'text':w.text,'bounds':[float(w.attrib[k]) for k in ('xMin','yMin','xMax','yMax')]} for w in page.findall('.//x:word',ns)]} for page in root.findall('.//x:page',ns)]
   row['results'][name]['artifact_sha256']=sha(out)
 meta['cases'].append(row);print(label,[(k,v['exit']) for k,v in row['results'].items()],flush=True)
assert all(sha(b)==meta['binaries'][n]['sha256'] for n,b in bins.items())
(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(meta,indent=2).encode()))
