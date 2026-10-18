from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={'constant-chain': '#show "B": [BB]\n#show "A": [B]\nA', 'constant-chain-inverse': '#show "A": [B]\n#show "B": [BB]\nA', 'function-chain': '#show "B":it=>[BB]\n#show "A":it=>[B]\nA', 'function-chain-inverse': '#show "A":it=>[B]\n#show "B":it=>[BB]\nA', 'alias-chain': '#let saved=[B]\n#show "B":it=>[BB]\n#show "A":it=>saved\nA', 'alias-chain-inverse': '#let saved=[B]\n#show "A":it=>saved\n#show "B":it=>[BB]\nA', 'cycle': '#show "A":it=>[B]\n#show "B":it=>[A]\nA', 'constant-cycle': '#show "A": [B]\n#show "B": [A]\nA', 'revoked-cross': '#show "C":it=>[A]\n#show "AB":it=>[X]\nCB', 'revoked-cross-inverse': '#show "AB":it=>[X]\n#show "C":it=>[A]\nCB', 'local-cross': '#let part={show "C":it=>[A];[C]}\n#show "AB":it=>[X]\n#part;B', 'leftmost': '#show "A":it=>[L]\n#show "B":it=>[R]\nAB', 'unicode-overlap': '#show "éX":it=>[L]\n#show "é":it=>[R]\néX', 'multiple-matches': '#show "A":it=>[AA]\n#show "B":it=>[BB]\nABAB', 'show-set': '#show "A":set text(fill:red)\nA', 'none-definition': '#let c=[#let x=1;A]\n#repr(c.children)', 'none-assert': '#let c=[#assert(true)A]\n#repr(c.children)', 'none-explicit': '#let c=[#none#[]A]\n#repr(c.children)', 'none-only': '#let c=[#none]\n#repr((c.func(),c.fields()))'}
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
