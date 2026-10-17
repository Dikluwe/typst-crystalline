"""Read-only GDB count of page-independent style calls and repeated recipe values."""
import datetime, gzip, hashlib, json, os, subprocess, tempfile
from pathlib import Path
D=Path(__file__).resolve().parent
R=D.parents[2]
F=D/'fixtures'
B=R/'target/release/typst'
G=D/'page_recipe_probe.gdb.py'
FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
sha=lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
       'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),
       'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),
       'binary_sha256':sha(B),'gdb_script_sha256':sha(G),'driver_sha256':sha(D/'count_page_recipe_keys.py'),
       'fixtures':{},'runs':[]}
with tempfile.TemporaryDirectory(prefix='page-recipe-probe-') as temp:
    for n in (100,1600):
        fixture=F/f'local-set-{n}.typ'
        result=Path(temp)/f'{n}.json'
        pdf=Path(temp)/f'{n}.pdf'
        env=os.environ.copy();env['PAGE_RECIPE_PROBE_OUT']=str(result)
        cmd=['gdb','-q','--batch','-x',str(G),'--args',str(B),'compile',str(fixture),str(pdf),'--root',str(F),
             '--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]
        proc=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=120,check=True)
        state['fixtures'][fixture.name]={'sha256':sha(fixture),'text':fixture.read_text()}
        state['runs'].append({'n':n,'utc':utc(),'command':cmd,'gdb_stdout':proc.stdout,'gdb_stderr':proc.stderr,
                              'probe':json.loads(result.read_text()),'pdf_sha256':sha(pdf)})
        print(n,state['runs'][-1]['probe'],flush=True)
state['utc_end']=utc()
assert sha(B)==state['binary_sha256'] and sha(G)==state['gdb_script_sha256']
with gzip.open(D/'page-recipe-key-counts-before.json.gz','wt') as output:json.dump(state,output,ensure_ascii=False)
