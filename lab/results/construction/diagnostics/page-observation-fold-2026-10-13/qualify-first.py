import importlib.util,datetime,gzip,hashlib,json,os,resource,subprocess,tempfile
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2]
spec=importlib.util.spec_from_file_location('q',R/'00_nucleo/diagnosticos/watch-context-current-2026-10-10/qualify.py')
# Load helpers only, without running the old experiment.
text=spec.origin
ns={'__file__':text};exec(Path(text).read_text().split('import itertools')[0],ns)
B={'before':Path('/tmp/tekt-page-fold/before'),'after':Path('/tmp/tekt-page-fold/after'),'vanilla':R/'lab/typst-original/target/release/typst'}
sha=ns['sha'];utc=ns['utc'];run=ns['run'];compare=ns['compare']
programs={
 'current':'#context [B#counter(page).display("1 / 1", both: true) F#counter(page).final().first()]',
 'late':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#counter(page).update(7)\n#context counter(page).display("1 / 1", both: true)',
 'empty':'#counter(page).update(())\n#context repr(counter(page).get())\n#pagebreak()\n#context repr(counter(page).get())',
 'multi':'#counter(page).update((7, 2))\n#context repr(counter(page).get())\n#pagebreak()\n#context repr(counter(page).get())',
 'callback':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#counter(page).update(n => n + 5)\n#context counter(page).get().first()',
 'position':'= Heading <h>\n#context repr(here().position())\n#pagebreak()\n#context [#repr(here().position()) #repr(query(heading).map(it => it.location().page()))]',
 'numbering':'#set page(numbering: (..nums) => nums.pos().map(str).join("/"))\n#context repr(here().page-numbering())\n#pagebreak()\n= Head <h>\n#context repr(here().page-numbering())\n#ref(<h>, form: "page")',
 'styles':'#set text(size: 11pt)\n#context [#set text(size: 16pt); #counter(page).get().first()]\n#pagebreak()\n#context counter(page).final().first()',
 'margin':'#set page(header: context [P#counter(page).display("1/1", both: true)])\n#context counter(page).get().first()\n#pagebreak()\nEnd',
 'nested':'#context [A #context counter(page).get().first()]\n#pagebreak()\n#context [B #context counter(page).final().first()]',
}
for n in [300,1600]:programs[f'context{n}']=(D/'fixtures'/f'context{n}.typ').read_text()
x={'utc_start':utc(),'driver_sha256':sha(__file__),'helper_sha256':sha(text),'binaries':{k:sha(v) for k,v in B.items()},'state_sha256':sha(D/'candidate.json.gz'),'cases':{}}
with tempfile.TemporaryDirectory() as tmp:
 out=Path(tmp);ns['F']=out
 for name,body in programs.items():
  fixture=out/(name+'.typ');fixture.write_text(body)
  rows={e:run(e,b,fixture,out) for e,b in B.items()}
  ab=compare(rows['before'],rows['after']);av=compare(rows['after'],rows['vanilla'])
  assert ab['same_text'] and ab['same_words'] and ab['same_page_size'] and ab['same_raster'] and ab['bbox_max_delta_pt']==0,(name,ab)
  x['cases'][name]={'source':body,'source_sha256':sha(fixture),'engines':rows,'before_after':ab,'after_vanilla':av};print(name,ab,av,flush=True)
 # Errors preserve complete diagnostic output between local binaries.
 for name,body in {'error':'#counter(page).update(n => false)\n#context counter(page).final().first()','nested_error':'#context [#context unknown]'}.items():
  fixture=out/(name+'.typ');fixture.write_text(body);rows={}
  for e,b in B.items():
   cmd=[str(b),'compile',str(fixture),str(out/'out.pdf'),'--pdf-tagged=false','--pretty=false'];p=subprocess.run(cmd,cwd=R,capture_output=True,text=True,env=ns['ENV']);rows[e]={'command':cmd,'code':p.returncode,'stderr':p.stderr};assert p.returncode!=0
  assert rows['before']['stderr']==rows['after']['stderr'];x['cases'][name]={'source':body,'engines':rows}
x['utc_end']=utc()
with gzip.open(D/'qualification.json.gz','wt') as f:json.dump(x,f)
