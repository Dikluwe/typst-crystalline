from pathlib import Path
import subprocess,datetime,json,os
D=Path(__file__).parent
rows=[]
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
for name,cmd in [('build-final',['cargo','build','--offline']),('release',['cargo','build','--offline','--release']),('workspace',['cargo','test','--offline','--workspace','--lib']),('cli-query',['cargo','test','--offline','-p','typst-wiring','--test','cli','query']),('lint',['crystalline-lint','--format','sarif','.'])]:
 row={'name':name,'command':cmd,'start_utc':utc(),'snapshot':'tested-final.json.gz'}
 env=os.environ.copy();env['CARGO_TARGET_DIR']=str(Path.cwd()/'target')
 with (D/('lint.json' if name=='lint' else name+'.log')).open('w') as f:p=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
 row.update(status=p.returncode,end_utc=utc());rows.append(row);(D/'validation.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,p.returncode,flush=True);assert p.returncode==0
