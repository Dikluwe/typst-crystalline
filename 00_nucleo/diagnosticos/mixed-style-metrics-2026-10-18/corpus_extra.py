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
for value in ['0pt','10pt']:
 setup=base+f'#set par(hanging-indent:{value})\n'
 for name,body in [('block',f'#block[{words}]'),('par-block',f'#block[#par()[{words}]]'),('multi-block',f'#block[{words}\n\n{words}]'),('box',f'#box(width:100pt)[{words}]'),('par-box',f'#box(width:100pt)[#par()[{words}]]'),('heading',f'= {words}'),('raw',f'```\n{words}\n```'),('list',f'- {words}'),('quote',f'#quote(block:true)[{words}]')]:
  cases[f'context-{name}-{value}']=setup+body+'\n\nNEXT'
for val in ['200pt','-200pt']:
 cases[f'extreme-{val}']=base+f'#set par(hanging-indent:{val})\n'+words
for val in ['10pt','2pt+1em']:
 cases[f'partial-scope-{val}']=base+f'FIRST #[#set par(hanging-indent:{val})\nalpha beta] gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi omicron pi LAST'
 cases[f'integral-scope-{val}']=base+f'#[#set par(hanging-indent:{val})\n{words}]'
 cases[f'partial-font-{val}']=base+f'#set par(hanging-indent:{val})\nFIRST #text(size:24pt)[alpha] beta gamma delta epsilon zeta eta theta iota kappa lambda mu LAST'
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
