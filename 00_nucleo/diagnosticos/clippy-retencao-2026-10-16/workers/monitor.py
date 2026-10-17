from pathlib import Path
import json,datetime
out=Path('00_nucleo/diagnosticos/clippy-retencao-2026-10-16');records=[];seen=set()
for entry in Path('/proc').iterdir():
 if not entry.name.isdigit():continue
 try:
  cmd=(entry/'cmdline').read_bytes().split(b'\0')
  if not any(arg.decode(errors='replace') in ['00_nucleo/diagnosticos/clippy-retencao-2026-10-16/workers/baseline.py','00_nucleo/diagnosticos/clippy-retencao-2026-10-16/workers/green.py','00_nucleo/diagnosticos/clippy-retencao-2026-10-16/workers/validate.py'] for arg in cmd):continue
  cg=Path('/sys/fs/cgroup')/(entry/'cgroup').read_text().strip().split('::',1)[1].lstrip('/')
  if cg in seen or (cg/'memory.max').read_text().strip()!=str(12*1024**3):continue
  seen.add(cg);records.append(dict(pid=int(entry.name),memory={k:(cg/k).read_text().strip() for k in ['memory.max','memory.swap.max','memory.current','memory.peak','memory.events']}))
 except (OSError,ValueError):continue
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),services=records)
with (out/'memory-samples.jsonl').open('a') as f:f.write(json.dumps(result)+'\n')
print(json.dumps(result))
