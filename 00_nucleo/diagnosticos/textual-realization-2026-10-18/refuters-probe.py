from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={'hidden-sequence-in-generic-valid': '#let seq=[].func()\n#show seq:it=>panic("HIDDEN")\n#show strong:it=>[Q]\n#strong[#([B]+[C])]', 'hidden-sequence-product-valid': '#let seq=[].func()\n#show seq:it=>panic("HIDDEN")\n#show strong:it=>[Q]\n#show "A":it=>strong[#([B]+[C])]\nA', 'capture-with-local-text': '#show "B":it=>[G]\n#let c=[#show "B":it=>[L]\nB]\n#show "B":it=>[R]\n#c', 'captured-rule-binding-return': '#let make()={show "B":it=>[L];let c=[B];c}\n#let c=make()\n#show "B":it=>[R]\n#c', 'captured-global-binding-return': '#show "B":it=>[G]\n#let make()={let c=[B];c}\n#let c=make()\n#show "B":it=>[R]\n#c'}
bins={'tekt':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'binaries':{k:{'path':str(b),'sha256':sha(b)} for k,b in bins.items()},'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3','cases':[]}
for label,source in cases.items():
 f=p/'outputs'/f'{label}.typ';f.write_text(source);row={'label':label,'source':source,'source_sha256':sha(f),'results':{}}
 for name,binary in bins.items():
  out=f.with_name(f.stem+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=45);txt=subprocess.check_output(['pdftotext',str(out),'-'],text=True) if not z.returncode else ''
  row['results'][name]={'command':cmd,'exit':z.returncode,'stderr':z.stderr,'text':' '.join(txt.split())}
 meta['cases'].append(row);print(label,[(k,v['exit'],v['text'] if not v['exit'] else v['stderr'].splitlines()[0]) for k,v in row['results'].items()],flush=True)
assert all(sha(b)==meta['binaries'][n]['sha256'] for n,b in bins.items())
(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(meta,indent=2).encode()))
