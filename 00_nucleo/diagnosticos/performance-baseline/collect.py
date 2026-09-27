"""Resume an existing gated command after observed interference; no timing of its own."""
import argparse, datetime, json, os, subprocess, sys, time
from pathlib import Path
OUT=Path(__file__).resolve().parent
p=argparse.ArgumentParser();p.add_argument('action',choices=['micro','e2e']);p.add_argument('--instrumented',action='store_true');a=p.parse_args()
mode=a.action+('-instrumented' if a.instrumented else '')
records=[];attempt=0
while True:
    attempt+=1;start=datetime.datetime.now(datetime.timezone.utc).isoformat()
    cmd=[sys.executable,'-u',str(OUT/'run.py'),a.action]+(['--instrumented'] if a.instrumented else [])
    log=OUT/'logs'/f'collect-{mode}-{time.time_ns()}.log'
    env=os.environ.copy();env['PYTHONDONTWRITEBYTECODE']='1'
    with log.open('w') as f:
        child=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,env=env)
        for row in child.stdout:
            f.write(row);f.flush()
            if 'VALID' in row or 'DISCARDED' in row or 'already accepted' in row:print(row,end='',flush=True)
        code=child.wait()
    records.append({'start_utc':start,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'exit_code':code,'log':str(log.relative_to(OUT)),'measurement':False})
    (OUT/f'collection-{mode}.json').write_text(json.dumps(records,indent=2)+'\n')
    if code==0:break
    value=log.read_text()
    retry=any(x in value for x in ['No stable/idle','Concurrent workload before','Contaminated CLI block','Invalid result; raw log retained: micro'])
    if not retry:print(value[-3500:],flush=True);sys.exit(code)
    print(f'{mode}: isolation rejected attempt {attempt}; waiting 30 s before resuming',flush=True)
    time.sleep(30)
