from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={
'markup-child':'#let c=[#([A]+[B]) C]\n#repr(c.children)',
'markup-alias':'#let x=[A]+[B]\n#let c=[#x C]\n#repr(c.children)',
'markup-two':'#let x=[A]+[B]\n#let c=[#x#x]\n#repr(c.children)',
'markup-single':'#let c=[#([A]+[B])]\n#repr(c.children)',
'markup-add':'#let c=[#([A]+[B]) C]+[D]\n#repr(c.children)',
'markup-for':'#let c=[#for i in range(2) {[A]+[B]} C]\n#repr(c.children)',
'markup-none':'#let c=[#none#[]A]\n#repr(c.children)',
'markup-unmatched-text':'#show "Z":it=>[X]\n#let c=[#([A]+[B]) C]\n#repr(c.children)',
'markup-shown-sequence':'#let f=[].func()\n#show f:it=>it\n#let c=[#([A]+[B]) C]\n#repr(c.children)',
'markup-label':'#let c=[#([A]+[B])<x>]\n#repr(c)',
'literal-identical':'#show "A":it=>[C]\n#show "A":it=>[B]\nA',
'literal-same-prefix':'#show "AB":it=>[X]\n#show "A":it=>[Y]\nAB',
'literal-first-position':'#show "B":it=>[X]\n#show "A":it=>[Y]\nAB',
'literal-transform-chain':'#show "B":it=>[C]\n#show "A":it=>[B]\nA',
'literal-transform-chain-inverse':'#show "A":it=>[B]\n#show "B":it=>[C]\nA',
'literal-local':'#show "A":it=>[C]\n#let c={show "A":it=>[B];[A]}\n#c',
'literal-local-reverse':'#let c={show "A":it=>[B];[A]}\n#show "A":it=>[C]\n#c',
'literal-loop-guards':'#show "A":it=>[AA]\nA',
'regex-literal':'#show regex("A"):it=>[R]\n#show "A":it=>[L]\nA',
'literal-regex':'#show "A":it=>[L]\n#show regex("A"):it=>[R]\nA',
'regex-chain':'#show regex("B"):it=>[C]\n#show regex("A"):it=>[B]\nA',
'literal-unmatched':'#show "B":it=>[X]\n#show "A":it=>[Y]\nCABD',
'literal-cross':'#show "AB":it=>[X]\n#([A]+[B])',
'styled-child':'#let c=[#text(fill:red)[A B] C]\n#repr(c.children)',
'styled-equality':'#let a=text(fill:red)[A B]\n#repr((a.func(),a.fields(),a==[A B]))',
'symbol-cast':'#let f=[].func()\n#let c=f((sym.alpha,))\n#repr((c.children.first().func(),c.children.first()==[α]))',
}
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
