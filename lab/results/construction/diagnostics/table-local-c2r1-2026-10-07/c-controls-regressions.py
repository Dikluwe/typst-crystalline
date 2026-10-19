from pathlib import Path
import subprocess,sys,concurrent.futures,json,datetime
R=Path.cwd();D=Path(__file__).parent;label=sys.argv[1]
groups={
 'presence':[('table-cell-presenca-2026-10-07','measure.py',label)],
 'selector':[('table-cell-selector-2026-10-06','measure.py',label)],
 'equation':[('equation-where-2026-10-06','measure.py',label)],
 'math':[('math-bare-2026-10-06','measure.py',label)],
 'matrix':[('matrix-morfologia-2026-10-06','measure.py',label),('matrix-morfologia-2026-10-06','additional.py',label+'-additional')],
 'query':[('query-compilada-2026-10-06','measure.py',label)],
 'table':[('table-cell-realizacao-d08b3-2026-10-07','measure.py',label),('table-cell-realizacao-d08b3-2026-10-07','refuters.py',label+'-refuters')],
 'raw':[('raw-defaults-2026-10-07','measure.py',label)],
 'raw-block':[( 'raw-block-spacing-2026-10-07',script,label+('-'+script[:-3] if script!='measure.py' else '')) for script in ['measure.py','additional.py','aliases.py','edge_cases.py','refuters.py']],
}
def run(item):
 name,steps=item;rows=[]
 for folder,script,out in steps:
  cmd=['python3',str(R/'00_nucleo/diagnosticos'/folder/script),out]
  log=D/(out+'-'+name+'-'+script+'.log')
  with log.open('w') as f:r=subprocess.run(cmd,cwd=R,stdout=f,stderr=subprocess.STDOUT)
  rows.append({'command':cmd,'status':r.returncode,'log':str(log),'collection':str(R/'00_nucleo/diagnosticos'/folder/(out+'.json.gz'))})
 return name,rows
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:results=dict(pool.map(run,groups.items()))
(D/(label+'-regressions.json')).write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'groups':results},indent=2)+'\n')
print(json.dumps({k:[r['status'] for r in rows] for k,rows in results.items()},indent=2))
assert all(r['status']==0 for rows in results.values() for r in rows)
