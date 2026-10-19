"""CLI controls of single evaluation; pinned vanilla, preserved baseline, candidate."""
import datetime,gzip,hashlib,json,os
from pathlib import Path
import subprocess
ROOT=Path(__file__).resolve().parents[3]
HERE=Path(__file__).resolve().parent
OUT=Path('/tmp/tekt-index-reuse-verify-2026-10-05');OUT.mkdir(exist_ok=True)
ENGINES={'baseline':Path('/tmp/tekt-index-reuse-2026-10-05/typst-baseline'),'candidate':ROOT/'target/release/typst','vanilla':ROOT/'lab/typst-original/target/release/typst'}
CASES={
'original':'#show heading.where(level: 1): it => upper(it.body)\n= Original',
'generated_one':'#show heading.where(level: 1): it => heading(level: 2)[Generated A]\n= Original',
'generated_two':'#show heading.where(level: 1): it => [#heading(level: 2)[Generated A] #heading(level: 2)[Generated B]]\n= Original',
'equal_generated':'#show heading.where(level: 1): it => [#heading(level: 2)[Equal] #heading(level: 2)[Equal]]\n= Original',
'alias':'#show heading: it => it\n= Original',
'double_alias':'#show heading: it => [#it #it]\n= Original',
'cloned_content':'#let h = heading(level: 2)[Equal]\n#show heading.where(level: 1): it => [#h #h]\n= Original',
'label':'#show heading.where(level: 1): it => upper(it.body)\n= Original <original>',
'fields_location':'#show heading: it => { assert(it.location() != none); assert(it.has("level")); assert(it.fields().at("level") == 1); it }\n#heading(level: 1)[Original]',
}
COMPILE={
'columns_show':'#set page(columns: 2)\n#show heading.where(level: 1): it => heading(level: 2)[Generated]\n= First <first>\n= Second',
'numbering_order':'#set heading(numbering: \"1.\")\n#show heading.where(level: 1): it => heading(level: 2)[Generated]\n= First\n= Second',
'matrix_mutation':'#let calls = (1,)\n#let m = math.mat({ calls += calls; [a] })\n#assert(calls.len() == 2)\n#m',
'matrix_once':'#let calls = ()\n#let m = math.mat({ calls.push("once"); [a] })\n#assert(calls == ("once",))\n#m',
'context_captured_ids':'#let cached = [#context [Captured]]\n#show heading: it => [#cached #context [Generated]]\n= Original',
'context_query':'#show heading.where(level: 1): it => heading(level: 2)[Generated]\n= Original\n#context query(heading).map(it => it.body).join([ | ])',
'table_cell_alias':'#show table.cell: it => it\n#table(columns: 2, table.cell(colspan: 2)[Wide], [A], [B])',
'show_set':'#show heading: set text(fill: red)\n= Original',
'include':'#include "included.typ"',
'import':'#import "imported.typ": value\n#assert(value == 42)\n= Imported',
}
(OUT/'included.typ').write_text('#show heading.where(level: 1): it => heading(level: 2)[Generated]\n= Original')
(OUT/'imported.typ').write_text('#let value = 42')
record={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'status':subprocess.check_output(['git','status','--short'],cwd=ROOT,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','binaries':{k:{'path':str(v),'sha256':hashlib.sha256(v.read_bytes()).hexdigest()} for k,v in ENGINES.items()},'cases':[]}
env=dict(os.environ,SOURCE_DATE_EPOCH='0',NO_COLOR='1')
def run(name,src,mode,engine):
 fixture=OUT/(name+'.typ');fixture.write_text(src)
 if mode=='query':args=['query',str(fixture),'heading','--field','body']
 else:args=['compile',str(fixture),str(OUT/(name+'-'+engine+'.'+mode)),'--root',str(OUT)]
 if mode=='html':args += ['--features','html','--pretty=false']
 if mode=='pdf':args += ['--pdf-tagged=false','--pretty=false']
 cmd=[str(ENGINES[engine]),*args];p=subprocess.run(cmd,cwd=ROOT,env=env,capture_output=True,text=True,timeout=60)
 item={'name':name,'mode':mode,'engine':engine,'source':src,'command':cmd,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
 if p.returncode==0 and mode=='pdf':item['text']=subprocess.check_output(['pdftotext',str(OUT/(name+'-'+engine+'.pdf')),'-'],text=True)
 if p.returncode==0 and mode=='html':item['html']=(OUT/(name+'-'+engine+'.html')).read_text()
 record['cases'].append(item)
for name,src in CASES.items():
 for engine in ENGINES:run(name,src,'query',engine)
for name,src in COMPILE.items():
 for engine in ENGINES:run(name,src,'pdf',engine)
for name,src in {'html_show':'#show strong: it => emph(it.body)\nHello *world*','html_text':'#show "ab": [MATCH]\na#text[b]','html_nested':'#show strong: it => [a] + [b]\n#show "ab": [MATCH]\n*x*'}.items():
 for engine in ENGINES:run(name,src,'html',engine)
record['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'verification.json.gz','wt') as f:json.dump(record,f,ensure_ascii=False,indent=2)
for name,mode in sorted({(x['name'],x['mode']) for x in record['cases']}):
 group={x['engine']:x for x in record['cases'] if x['name']==name and x['mode']==mode}
 c,v=group['candidate'],group['vanilla'];key={'query':'stdout','pdf':'text','html':'html'}[mode]
 print(name,mode,'candidate_rc',c['returncode'],'vanilla_rc',v['returncode'],'same',c.get(key)==v.get(key))
 if c['returncode'] or mode=='query':print(' candidate',c.get(key),c['stderr'].strip(),' vanilla',v.get(key),v['stderr'].strip())
