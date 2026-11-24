set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
starti
python
import gdb,json,os
from pathlib import Path
from collections import Counter
case=Path.cwd(); pid=gdb.selected_inferior().pid; exe=os.readlink(f'/proc/{pid}/exe')
base=next(int(x.split()[0].split('-')[0],16) for x in Path(f'/proc/{pid}/maps').read_text().splitlines() if x.split()[2]=='00000000' and x.endswith(exe))
counts=Counter(); publication=0
resume=[]
class Entry(gdb.Breakpoint):
 def __init__(self,address,kind):
  super().__init__('*'+hex(base+address),internal=True);self.kind=kind
 def stop(self):
  global publication
  if self.kind=='wait':
   (case/f'publication-{publication}.pdf').write_bytes((case/'out.pdf').read_bytes())
   if publication==3:
    (case/'counts.json').write_text(json.dumps(dict(counts),indent=2))
    (case/'resume.json').write_text(json.dumps(resume,indent=2));return True
   publication+=1
   pending=case/'pending.typ';pending.write_bytes((case/'edits'/f'{publication}.typ').read_bytes());pending.replace(case/'main.typ')
  else:
   counts[f"{publication}:{self.kind}"]+=1
   if self.kind=='resume':
    resume.append({
     'publication':publication,
     'line_commits_so_far':counts[f"{publication}:line_commit"],
     'region_transitions_so_far':counts[f"{publication}:region"],
     'stack':gdb.execute('bt 16',to_string=True),
    })
  return False
for kind,addresses in json.loads((case/'addresses.json').read_text()).items():
 for address in addresses:Entry(address,kind)
end
continue
kill
