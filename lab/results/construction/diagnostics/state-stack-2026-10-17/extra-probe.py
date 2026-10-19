from pathlib import Path
import json,gzip,subprocess,hashlib,datetime
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sources={
 'box-height-type':'#assert.eq(type(box(height:2pt)[A].height),type(1%+1pt))\n#assert.eq(box(height:2pt)[A].height,2pt)',
 'block-fields-order':'#assert.eq(block(width:3pt,height:4pt)[A].fields().keys(),("width","height","body"))\n#assert.eq(block(width:3pt)[A].width,3pt)',
 'spacing-weak-zero':'#assert.eq(h(0pt,weak:false).weak,false)\n#assert.eq(v(0pt,weak:true).weak,true)',
 'utf8-text-field':'#assert.eq([水果].text,"水果")',
 'empty-text-field-negative':'#let x=[].text',
 'counter-equation-count':'#set math.equation(numbering:"1")\n#math.equation(block:true)[$x$]\n#math.equation(block:true)[$y$]\n#context assert.eq(counter(math.equation).get(),(2,))',
 'query-equation-alias':'#let e=math.equation\n#math.equation(block:true)[$x$]\n#math.equation(block:true)[$y$]\n#context assert.eq(query(e).len(),2)',
 'selector-nonlocatable-negative':'#context query(text)',
 'selector-partial-negative':'#selector(text.with())',
 'counter-equation-homonym-negative':'#let equation(x)=x\n#counter(equation)',
 'counter-footnote-control':'A#footnote[B]\n#context assert.eq(counter(footnote).get(),(1,))',
 'heading-constructor-callback':'#set heading(numbering:"1")\n#heading(supplement:h=>[Custom])[A] <a>\nSee @a.',
 'heading-body-capture':'#let prefix="Cap"\n#set heading(numbering:"1",supplement:h=>prefix+"-"+h.body.text)\n= A <a>\nSee @a.',
 'heading-numbering-preserved':'#set heading(numbering:"1")\n#set heading(supplement:h=>[Custom])\n= A <a>\nSee @a.',
 'heading-none':'#set heading(numbering:"1",supplement:none)\n= A <a>\nSee @a.',
 'heading-auto':'#set heading(numbering:"1",supplement:h=>[Custom])\n#set heading(supplement:auto)\n= A <a>\nSee @a.',
 'heading-local-scope':'#set heading(numbering:"1")\n#[\n#set heading(supplement:h=>[Local])\n= A <a>\n]\n= B <b>\nSee @a; @b.',
 'heading-same-body-depth':'#set heading(numbering:"1.1",supplement:h=>str(h.depth))\n= A <a>\n== A <b>\nSee @a; @b.',
 'heading-second-callback-error':'#set heading(supplement:h=>if h.depth==2 {panic("SECOND")})\n= A\n== B',
 'state-default-read':'#let s=state("a")\n#context assert.eq(s.get(),none)',
}
bins={'tekt':root/'target/debug/typst','tekt-before':work/'typst-before','vanilla':root/'lab/typst-original/target/release/typst'}
rows=[]
for label,source in sources.items():
 file=work/('extra-'+label+'.typ');file.write_text(source+'\n');row={'label':label,'source':source,'source_sha256':hashlib.sha256(file.read_bytes()).hexdigest(),'results':{}}
 for name,binary in bins.items():
  target=work/('extra-'+label+'-'+name+'.pdf');cmd=[str(binary),'compile',str(file),str(target)];start=now();r=subprocess.run(cmd,capture_output=True,text=True,timeout=45)
  text=subprocess.run(['pdftotext',str(target),'-'],capture_output=True,text=True).stdout if r.returncode==0 and target.exists() else ''
  row['results'][name]={'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'text':text,'command':cmd,'start':start,'end':now()}
 rows.append(row);print(label,[(n,r['exit']) for n,r in row['results'].items()],flush=True)
(p/'extra-probe.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'cases':rows},ensure_ascii=False,indent=2).encode()))
