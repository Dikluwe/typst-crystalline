"""Real watch dependency edits, outside performance clocks."""
import sys
sys.dont_write_bytecode=True
import measure as m
from pathlib import Path
import shutil,os,subprocess as sp,time
D=m.TASK_RAM;R=Path(__file__).resolve().parents[3];F=R/'lab/experiments/stdlib-reuse/tests/fixtures'
for variant,target,policy in [('bas','base-target',None),('p03','candidate-target','3')]:
 for shape,relative,expected in [('imports_16','modules/m00.typ',17),('nested_16','nested16/n15.typ',17),('repeated_16','modules/shared.typ',32)]:
  case=D/'dependency-watch'/f'{variant}-{shape}';case.mkdir(parents=True,exist_ok=False);inputs=case/'inputs';shutil.copytree(F,inputs);pdf=case/'out.pdf';tmp=case/'tmp';tmp.mkdir();main=inputs/(shape+'.typ');child=None
  env=m.environment(tmp);env.pop('TEKT_LAB_PHASE_FILE',None)
  if policy is not None:env['TEKT_LAB_STDLIB_POLICY']=policy
  with (case/'watch.log').open('wb') as log:
   try:
    child=m.Child([str(D/target/'release/typst'),'watch',str(main),str(pdf),'--root',str(inputs),'--pdf-tagged=false'],inputs,env,min(os.sched_getaffinity(0)),log)
    previous,_,_=m.await_pdf(child,pdf,None);time.sleep(.03)
    old=sp.check_output(['pdftotext',str(pdf),'-'],text=True);assert 'Total 16' in old,(variant,shape,old)
    dep=inputs/relative;text=dep.read_text();assert '1' in text;dep.write_text(text.replace('1','2',1))
    m.await_pdf(child,pdf,previous);time.sleep(.03)
    current=sp.check_output(['pdftotext',str(pdf),'-'],text=True);assert f'Total {expected}' in current,(variant,shape,current)
    print(variant,shape,'16 ->',expected,flush=True)
   finally:
    if child:child.close()
