import os,sys
if os.environ.get('TEKT_V18_MEMORY_LIMIT')!='12G':
    os.execvp('systemd-run',['systemd-run','--user','--quiet','--wait','--pipe',
        '--working-directory='+os.getcwd(),'-p','MemoryMax=12G','-p','MemorySwapMax=0',
        '/usr/bin/env','TEKT_V18_MEMORY_LIMIT=12G','CARGO_BUILD_JOBS=1',
        'PATH='+os.environ['PATH'],'python3',__file__,*sys.argv[1:]])
from pathlib import Path
import subprocess,json,datetime
out=Path('00_nucleo/diagnosticos/tekt-ranges-2026-10-16');out.mkdir(exist_ok=True)
label=sys.argv[1];command=sys.argv[2:];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
cg=Path('/sys/fs/cgroup')/Path('/proc/self/cgroup').read_text().strip().split('::',1)[1].lstrip('/')
memory=lambda:{k:(cg/k).read_text() for k in ['memory.max','memory.swap.max','memory.current','memory.peak','memory.events']}
record=dict(start=now(),label=label,command=command,memory_before=memory(),cargo_jobs=os.environ['CARGO_BUILD_JOBS'])
assert record['memory_before']['memory.max'].strip()==str(12*1024**3)
with (out/(label+'.log')).open('w') as f:r=subprocess.run(command,stdout=f,stderr=subprocess.STDOUT)
record.update(end=now(),exit=r.returncode,memory_after=memory())
(out/(label+'-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True);sys.exit(r.returncode)
