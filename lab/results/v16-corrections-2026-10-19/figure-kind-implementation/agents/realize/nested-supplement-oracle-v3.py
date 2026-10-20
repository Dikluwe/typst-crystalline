#!/usr/bin/env python3
from pathlib import Path
import subprocess,hashlib,json,datetime
ROOT=Path('/repos/Antigravity/typst-crystalline')
D=Path(__file__).resolve().parent/'nested-supplement-oracle-v3';D.mkdir(exist_ok=True)
BIN=ROOT/'lab/typst-original/target/release/typst'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert sha(BIN)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
inner='figure([INNER], kind: table, caption: [INNER-CAP], supplement: [Inner])'
counts='\n#context [#metadata((query(figure).len(),counter(figure.where(kind: table)).final(),counter(figure.where(kind: image)).final())) <probe>]\n'
cases={
'explicit-caption':f'#figure([OUTER], caption:[OUTER-CAP], supplement:{inner})'+counts,
'callback-caption':f'#figure([OUTER], caption:[OUTER-CAP], supplement:it=>{inner})'+counts,
'explicit-no-caption':f'#figure([OUTER], supplement:{inner})'+counts,
'callback-no-caption':f'#figure([OUTER], supplement:it=>{inner})'+counts,
'explicit-reference':f'#figure([OUTER], supplement:{inner}) <outer>\n@outer'+counts,
'callback-reference':f'#figure([OUTER], supplement:it=>{inner}) <outer>\n@outer'+counts,
'explicit-unused':f'#let unused=figure([OUTER], caption:[OUTER-CAP], supplement:{inner})\n[VISIBLE]'+counts,
'callback-unused':f'#let unused=figure([OUTER], caption:[OUTER-CAP], supplement:it=>{inner})\n[VISIBLE]'+counts,
}
panic_event='#counter(figure.where(kind:table)).update(n=>panic("NESTED_IMPLICIT_DEMAND"))\n'
for form, supplement in [('explicit',inner),('callback','it=>'+inner)]:
 cases[form+'-caption-implicit-demand-panic']=panic_event+f'#figure([OUTER], caption:[OUTER-CAP], supplement:{supplement})'
 cases[form+'-no-caption-no-implicit-demand']=panic_event+f'#figure([OUTER], supplement:{supplement})'
 cases[form+'-reference-implicit-demand-panic']=panic_event+f'#figure([OUTER], supplement:{supplement}) <outer>\n@outer'
 cases[form+'-unused-no-implicit-demand']=panic_event+f'#let unused=figure([OUTER], caption:[OUTER-CAP], supplement:{supplement})\n[VISIBLE]'
report={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','oracle_sha256':sha(BIN),'runner_sha256':sha(Path(__file__)),'cases':[]}
for name,source in cases.items():
 p=D/(name+'.typ');p.write_text(source);row={'name':name,'source':source,'source_sha256':sha(p),'runs':[]}
 for command,args in [('compile',[str(BIN),'compile',str(p),str(D/(name+'.pdf'))]),('query',[str(BIN),'query',str(p),'<probe>','--field','value'])]:
  start=now();cp=subprocess.run(args,cwd=ROOT,text=True,capture_output=True,timeout=35)
  run={'command':command,'utc_start':start,'utc_end':now(),'argv':args,'returncode':cp.returncode,'stdout':cp.stdout,'stderr':cp.stderr}
  if command=='compile' and cp.returncode==0:
   pdf=D/(name+'.pdf');out=subprocess.run(['pdftotext',str(pdf),'-'],cwd=ROOT,text=True,capture_output=True,timeout=15)
   run['pdf_sha256']=sha(pdf);run['text_argv']=['pdftotext',str(pdf),'-'];run['text_returncode']=out.returncode;run['text']=out.stdout;run['text_stderr']=out.stderr
  row['runs'].append(run)
 report['cases'].append(row)
 print(name,[(r['command'],r['returncode'],r.get('text',r['stdout']).strip()) for r in row['runs']])
report['utc_end']=now();(D/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
