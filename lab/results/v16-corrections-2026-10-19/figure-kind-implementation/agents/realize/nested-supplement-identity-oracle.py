#!/usr/bin/env python3
from pathlib import Path
import subprocess,hashlib,json,datetime
ROOT=Path('/repos/Antigravity/typst-crystalline')
D=Path(__file__).resolve().parent/'nested-supplement-identity-oracle';D.mkdir(exist_ok=True)
BIN=ROOT/'lab/typst-original/target/release/typst'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert sha(BIN)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
inner='figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION"))'
cases={}
for form,supplement in [('explicit',inner),('callback','it=>'+inner)]:
 cases[form+'-caption']=f'#figure([OUTER],caption:[OUTER-CAP],supplement:{supplement})'
 cases[form+'-no-caption']=f'#figure([OUTER],supplement:{supplement})'
 cases[form+'-reference']=f'#figure([OUTER],supplement:{supplement}) <outer>\n@outer'
 cases[form+'-numbering-none']=f'#figure([OUTER],numbering:none,caption:[OUTER-CAP],supplement:{supplement})'
 cases[form+'-parent-hidden']=f'#show block:none\n#block(figure([OUTER],caption:[OUTER-CAP],supplement:{supplement}))'
 cases[form+'-unused']=f'#let unused=figure([OUTER],caption:[OUTER-CAP],supplement:{supplement})\n[VISIBLE]'
 cases[form+'-own-hidden']=f'#show figure:none\n#figure([OUTER],caption:[OUTER-CAP],supplement:{supplement})'
cases={name:'#show figure:it=>it\n'+source for name,source in cases.items()}
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
