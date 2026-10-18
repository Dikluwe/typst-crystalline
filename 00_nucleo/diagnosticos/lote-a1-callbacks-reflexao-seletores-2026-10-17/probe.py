from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,sys
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';work.mkdir(exist_ok=True)
phase=sys.argv[1];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','tekt-before':work/'typst-before','vanilla':root/'lab/typst-original/target/release/typst'}
sources={
 'text-fields':'#assert.eq([A].text,"A")\n#assert.eq([A].fields(),(text:"A"))',
 'spacing-fields':'#assert.eq(h(2em).amount,2em)\n#assert.eq(h(1fr).amount,1fr)\n#assert.eq(v(3pt).amount,3pt)\n#assert.eq(h(2pt).fields(),(amount:2pt))\n#assert.eq(h(2pt,weak:false).fields(),(amount:2pt,weak:false))',
 'box-block-fields':'#assert.eq(box(width:3pt)[A].width,3pt)\n#assert.eq(box(width:2fr)[A].width,2fr)\n#assert.eq(block(width:50%+3pt)[A].width,50%+3pt)\n#assert.eq(block(height:2fr)[A].height,2fr)\n#assert.eq(block[A].body,[A])',
 'omitted-fields':'#assert.eq(box[A].has("width"),false)\n#assert.eq(block[A].has("width"),false)\n#assert.eq(h(2pt).has("weak"),false)\n#assert.eq(h(2pt,weak:false).has("weak"),true)',
 'show-text':'#show text: set text(fill:red)\nA',
 'show-bibliography':'#show bibliography: it=>[X]\n#bibliography("audit.bib")',
 'counter-equation':'#let eq=math.equation\n#assert.eq(type(counter(eq)),counter)\n#assert.eq(type(counter(math.equation)),counter)',
 'selector-native':'#assert.eq(selector(text),selector(text))\n#assert.ne(selector(text),text.where())\n#assert.eq(selector(bibliography),selector(bibliography))',
 'box-body-contract-pending':'#assert.eq(box[A].body,[A])',
 'box-omitted-body-control':'#assert.eq(box().fields(),(:))',
 'box-width-type':'#assert.eq(type(box(width:3pt)[A].width),type(1%+1pt))',
 'counter-key-equivalence':'#assert(counter(math.equation)==counter(selector(math.equation)))\n#assert(counter(footnote)==counter(selector(footnote)))',
 'assert-relative-control':'#assert.eq(1pt,0%+1pt)\n#assert.eq((1,),(1.0,))',
 'fake-page-counter':'#let page(x)=x\n#counter(page)',
 'styled-field-negative':'#assert.eq(text(fill:red)[A].has("text"),false)',
 'state-init-default':'#assert.eq(type(state("audit")),state)',
 'state-func-init':'#let s=state("a",3)\n#s.update(n=>n+2)\n#context assert.eq(s.final(),5)',
 'state-func-none':'#let s=state("a")\n#s.update(n=>if n==none {2} else {panic("INIT")})\n#context assert.eq(s.final(),2)',
 'state-func-set-order':'#let s=state("a",1)\n#s.update(n=>n+1)\n#s.update(7)\n#s.update(n=>n*2)\n#context assert.eq(s.final(),14)',
 'state-func-panic':'#let s=state("a",0)\n#s.update(n=>panic("A1_CALLBACK"))\n#context s.final()',
 'heading-supplement-panic':'#set heading(supplement:h=>panic("A1_HEADING"))\n= A',
 'heading-supplement-valid':'#set heading(numbering:"1",supplement:h=>[Custom])\n= A <a>\nSee @a.',
 'state-legacy-control':'#let s=state("a",0)\n#s.update(4)\n#context assert.eq(s.final(),4)',
 'non-element-selector':'#let text(x)=x\n#show text:it=>it\nA',
 'nonlocatable-counter':'#let t=text\n#context counter(t).get()',
}
(work/'audit.bib').write_text('@book{audit,author={Audit, Ana},title={Example},year={2026}}\n')
rows=[]
for label,source in sources.items():
 f=work/(label+'.typ');f.write_text(source+'\n');row={'label':label,'source':source,'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'results':{}}
 for name,binary in bins.items():
  target=work/(phase+'-'+label+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(target),'--format','pdf'];start=now();r=subprocess.run(cmd,text=True,capture_output=True,timeout=45)
  row['results'][name]={'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'start':start,'end':now(),'command':cmd,'text':subprocess.run(['pdftotext',str(target),'-'],capture_output=True,text=True).stdout if target.exists() else None}
 rows.append(row);print(label,[(n,r['exit']) for n,r in row['results'].items()],flush=True)
d={'utc':now(),'phase':phase,'cases':rows,'binaries':{n:{'path':str(b),'sha256':hashlib.sha256(b.read_bytes()).hexdigest()} for n,b in bins.items()},'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=root,text=True)}
(p/(phase+'-probe.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
