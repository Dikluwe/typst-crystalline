from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,sys
p=Path(__file__).resolve().parent
steps=[('seal',['python3',str(p/'seal.py')]),('focus',['cargo','test','--offline','--locked','-p','typst-core','--lib','sequence_r3_','--','--test-threads=1']),('build',['cargo','build','--offline','--locked','--workspace']),('geometry',['python3',str(p/'markup-geometry.py')])]
rows=[]
for label,cmd in steps:
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (p/('prefix-'+label+'.log')).open('w') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 rows.append({'label':label,'command':cmd,'utc':start,'exit':r.returncode});(p/'prefix-results.json.gz').write_bytes(gzip.compress(json.dumps(rows,indent=2).encode()));print(label,r.returncode,flush=True)
 if r.returncode:sys.exit(r.returncode)
