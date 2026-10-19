from pathlib import Path
import os,subprocess,json,datetime
p=Path(__file__).resolve().parent;rows=[]
for name,extra in [('shm',{'TMPDIR':'/dev/shm'}),('normal',{})]:
 cmd=['cargo','test','--offline','--locked','-p','typst-core','--doc','--','--test-threads=1'];start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (p/('doctest-temp-'+name+'.log')).open('w') as log:r=subprocess.run(cmd,env=dict(os.environ,**extra),stdout=log,stderr=subprocess.STDOUT)
 row={'name':name,'command':cmd,'env':extra,'start':start,'exit':r.returncode};rows.append(row);print(row,flush=True)
(p/'doctest-temp-probe.json').write_text(json.dumps(rows,indent=2))
