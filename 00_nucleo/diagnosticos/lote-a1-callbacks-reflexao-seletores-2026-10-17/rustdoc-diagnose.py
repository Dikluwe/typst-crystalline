from pathlib import Path
import subprocess,os,json,datetime
p=Path(__file__).resolve().parent
results=[]
for name,extra in [('normal',{}),('shm',{'TMPDIR':'/dev/shm'})]:
 env=dict(os.environ,**extra,RUSTC_LOG='rustc_metadata::locator=info')
 f=p/'outputs'/'comemo-diagnostic.rs';f.write_text('extern crate comemo; fn main() {}')
 cmd=['rustc',str(f),'--edition=2021','-L','dependency=target/debug/deps','--extern','comemo=target/debug/deps/libcomemo-1e5c47001c9019b8.rlib','--emit=metadata','-o',str(p/'outputs'/('comemo-'+name+'.rmeta'))]
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (p/('rustc-locator-'+name+'.log')).open('w') as log:r=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT)
 results.append({'name':name,'command':cmd,'exit':r.returncode,'start':start})
(p/'rustc-locator.json').write_text(json.dumps(results,indent=2));print(results)
