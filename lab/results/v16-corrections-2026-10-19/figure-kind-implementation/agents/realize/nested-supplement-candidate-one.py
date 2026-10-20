#!/usr/bin/env python3
from pathlib import Path
import json,subprocess,datetime,hashlib
ROOT=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
OUT=D/'nested-supplement-candidate-one';OUT.mkdir(exist_ok=True)
BIN=ROOT/'lab/scratch/figure-kind-candidate-one-typst';BUILD=ROOT/'lab/results/v16-corrections-2026-10-19/figure-kind-implementation/candidate-build-1/receipt.json'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
report={'utc_start':now(),'runner_sha256':sha(Path(__file__)),'candidate_binary':str(BIN.relative_to(ROOT)),'candidate_sha256':sha(BIN),'build_receipt':str(BUILD.relative_to(ROOT)),'build_receipt_sha256':sha(BUILD),'source_state':'Bound to recorded frozen candidate-one build receipt; current dirty tree is not the tested executable.','cases':[]}
for family in ['nested-supplement-oracle-v3','nested-supplement-effects-oracle']:
 receipt=D/family/'receipt.json';prior=json.loads(receipt.read_text())
 for case in prior['cases']:
  name=family+'--'+case['name'];p=D/family/(case['name']+'.typ');assert sha(p)==case['source_sha256']
  row={'name':name,'source':case['source'],'source_sha256':sha(p),'oracle_receipt':str(receipt.relative_to(ROOT)),'oracle_receipt_sha256':sha(receipt),'oracle_runs':case['runs'],'candidate_runs':[]}
  for command,argv in [('compile',[str(BIN),'compile',str(p),str(OUT/(name+'.pdf'))]),('query',[str(BIN),'query',str(p),'<probe>','--field','value'])]:
   start=now();cp=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=35);run={'command':command,'utc_start':start,'utc_end':now(),'argv':argv,'returncode':cp.returncode,'stdout':cp.stdout,'stderr':cp.stderr}
   if command=='compile' and cp.returncode==0:
    pdf=OUT/(name+'.pdf');cp2=subprocess.run(['pdftotext',str(pdf),'-'],cwd=ROOT,text=True,capture_output=True,timeout=15);run.update(pdf_sha256=sha(pdf),text_argv=['pdftotext',str(pdf),'-'],text_returncode=cp2.returncode,text=cp2.stdout,text_stderr=cp2.stderr)
   row['candidate_runs'].append(run)
  report['cases'].append(row);print(name,[(x['command'],x['returncode'],x.get('text',x['stdout']).strip()) for x in row['candidate_runs']],flush=True)
report['utc_end']=now();(OUT/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
