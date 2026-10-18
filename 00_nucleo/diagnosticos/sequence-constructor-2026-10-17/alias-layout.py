from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
bins={'tekt':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in bins.items()},'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3','cases':[]}
for label,prefix in [('plain','Lead'),('strong','*Lead*'),('styled','#text(size:16pt)[Lead]')]:
 source='#set page(width:200pt,height:240pt,margin:10pt)\n#columns(2,gutter:10pt)[\n'+prefix+'\n#block(height:1fr)[One]\nAfter\n#colbreak()\n#block(height:1fr)[Two]\nTail\n]\n'
 row={'label':label,'results':{}}
 for name,binary in bins.items():
  variants={}
  for variant,body in [('baseline',source),('alias','#let sequence=[].func()\n#show sequence:it=>it\n'+source)]:
   f=p/'outputs'/f'alias-layout-{label}-{variant}.typ';f.write_text(body);out=f.with_name(f.stem+'-'+name+'.pdf');command=[str(binary),'compile',str(f),str(out)];z=subprocess.run(command,text=True,capture_output=True,timeout=45)
   assert z.returncode==0,(label,name,variant,z.stderr)
   bbox=subprocess.check_output(['pdftotext','-bbox',str(out),'-'],text=True);dom=ET.fromstring(bbox);words=[];pages=[e for e in dom.iter() if e.tag.rsplit('}',1)[-1]=='page']
   for page,node in enumerate(pages):
    for w in node.iter():
     if w.tag.rsplit('}',1)[-1]=='word':words.append({'page':page,'word':w.text,'bounds':[float(w.attrib[k]) for k in ['xMin','yMin','xMax','yMax']]})
   variants[variant]={'source':body,'source_sha256':sha(f),'command':command,'stderr':z.stderr,'pages':len(pages),'words':words}
  a,b=[variants[k] for k in ['baseline','alias']]; same_pages=a['pages']==b['pages']
  same_words=[(w['page'],w['word']) for w in a['words']]==[(w['page'],w['word']) for w in b['words']]
  delta=max((abs(x-y) for left,right in zip(a['words'],b['words']) for x,y in zip(left['bounds'],right['bounds'])),default=0)
  row['results'][name]={'variants':variants,'max_delta_pt':delta,'same_pages':same_pages,'same_words':same_words,'accepted':same_pages and same_words and delta<1e-6};print(label,name,delta,flush=True)
 meta['cases'].append(row)
assert all(sha(b)==meta['binaries'][n]['sha256'] for n,b in bins.items())
(p/'alias-layout.json.gz').write_bytes(gzip.compress(json.dumps(meta,indent=2).encode()))
assert all(v['accepted'] for c in meta['cases'] for v in c['results'].values()),'alias changed page/word geometry'
