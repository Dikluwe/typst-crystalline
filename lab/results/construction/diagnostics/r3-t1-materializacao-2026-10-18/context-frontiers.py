from pathlib import Path
import json,gzip,datetime,hashlib,subprocess
p=Path(__file__).resolve().parent;r=p.parents[2];out=p/'outputs/context-frontiers-candidate9';out.mkdir(parents=True,exist_ok=True)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
cases={
 'equation-one': '#show math.equation.where():it=>[EQ]\n#context math.frac([1],[2])\n#context[Q#query(math.equation).len()]',
 'equation-two': '#show math.equation.where():it=>[EQ]\n#context math.frac([1],[2])\n#context math.frac([3],[4])\n#context[Q#query(math.equation).len()]',
 'equation-heading-prefix': '#show heading:it=>it\n= H\n#show math.equation.where():it=>[EQ]\n#context math.frac([1],[2])\n#context[Q#query(math.equation).len()]',
 'text-context-global': '#show "A":[L]\n#let c=context[A]\n#show "A":[R]\n#c',
 'text-context-local': '#let c=[#show "A":[L]\n#context[A]]\n#show "A":[R]\n#c',
}
bins={'before':p/'bin/typst-before','after':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
d=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_state='candidate9-sources.json.gz',pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={n:dict(path=str(b),sha256=sha(b)) for n,b in bins.items()},cases=[])
for label,source in cases.items():
 f=out/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),results={})
 for name,binary in bins.items():
  target=out/(label+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(target)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);result=dict(command=cmd,exit=z.returncode,stderr=z.stderr,text='')
  if z.returncode==0:result.update(pdf_sha256=sha(target),text=' '.join(subprocess.check_output(['pdftotext',str(target),'-'],text=True).split()))
  row['results'][name]=result
 d['cases'].append(row);print(label,[(k,v['exit'],v['text']) for k,v in row['results'].items()],flush=True)
d['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(b)==d['binaries'][n]['sha256'] for n,b in bins.items());(p/'context-frontiers-candidate9.json.gz').write_bytes(gzip.compress(json.dumps(d,indent=2).encode()))
