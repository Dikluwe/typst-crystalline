from pathlib import Path
import sys,os,json,importlib.util,subprocess,shutil
D=Path(__file__).resolve().parent
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK,EXES,run,cycle
spec=importlib.util.spec_from_file_location('pdf_check',D.parent/'references-500-visible-stable-cids-2026-11-21/pdf_check.py');check=importlib.util.module_from_spec(spec);spec.loader.exec_module(check)
protect={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
base=b'''#set page(width:300pt,height:300pt,margin:20pt)\n#set text(size:11pt)\n'''
cases={
 'plain':base+b'#for i in range(20) [Paragraph #i with *bold* and _emphasis_.\\\n]\n',
 'context':base+b'#let c=counter("local")\n#for i in range(15) [#c.step() #context [Value #c.get().first() at page #counter(page).get().first()]\n\n]\n',
 'nested':base+b'#context [Outer #context [Inner #counter(page).get().first()]]\n#block[Wrapped #context [Page #counter(page).get().first()]]\n',
 'references':base+b'#set heading(numbering: "1.")\n#outline()\n= First <first>\nBody @second.\n#pagebreak()\n= Second <second>\nSee @first.\n',
 'styles':base+b'#show "word": it => strong(it)\n#context [word #emph[word] #[#set text(fill:red); word]]\n',
 'page-marginals':base+b'#set page(header: [Header #context counter(page).display()], footer: [Footer #context counter(page).display()])\n#title[Visible title]\n#for i in range(30) [Entry #i\n\n]\n',
 'figure-grid':base+b'#figure(rect(width:20pt,height:10pt),caption:[Caption #context [Page #counter(page).get().first()]])\n#grid(columns:2,[Cell #context [#counter(page).get().first()]],[Other])\n',
}
out=D.parents[1]/'results/construction/diagnostics/reserve-locations-borrowed-2026-11-21'
results={}
for index,(name,source) in enumerate(cases.items()):
 states=[('open',source,None),('edit',source.replace(b'width:300pt',b'width:240pt').replace(b'size:11pt',b'size:13pt')+b'\nNew visible text.\n',None),('restore',source,None)]
 results[name]={}
 for j,arm in enumerate(['control','candidate']):
  EXES['candidate']=WORK/arm
  snapshots=[]
  original_key=cycle.cycle.key
  def capture(engine,pdf):
   target=pdf.parent/f"snapshot-{len(snapshots)}.pdf"
   shutil.copyfile(pdf,target);snapshots.append(target)
   return original_key(engine,pdf)
  cycle.cycle.key=capture
  try:
   row=run(index*2+j,'candidate',0,overrides=protect,states=states,check=False,directory='directed-all-states')
  finally:cycle.cycle.key=original_key
  folder=WORK/row['directory']
  env=cycle.environment(folder,'candidate',False);env.update(protect)
  outputs=[]
  assert len(snapshots)==len(states)
  for n,((state,text,leaf),pdf) in enumerate(zip(states,snapshots)):
   assert leaf is None
   (folder/'main.typ').write_bytes(text)
   fresh=folder/f'fresh-{n}.pdf'
   subprocess.run(['taskset','-c','1',str(WORK/arm),'compile',str(folder/'main.typ'),str(fresh),'--root',str(folder),'--pdf-tagged=true'],env=env,stdout=(folder/f'fresh-{n}.log').open('wb'),stderr=subprocess.STDOUT,check=True,timeout=60)
   layout=check.text_layout(pdf);assert layout==check.text_layout(fresh),(name,arm,state,'fresh text')
   raster=check.render(pdf,folder,f'warm-{n}');assert raster==check.render(fresh,folder,f'fresh-{n}'),(name,arm,state,'fresh raster')
   structure=check.structure(pdf);assert structure==check.structure(fresh),(name,arm,state,'fresh structure')
   outputs.append(dict(state=state,layout=layout,raster=raster,structure=structure))
  results[name][arm]={'hashes':row['all_pdfs'],'outputs':outputs}
 assert results[name]['control']==results[name]['candidate'],name
 print(name,'passed',flush=True)
 (out/'directed.json').write_text(json.dumps(results,indent=2))
