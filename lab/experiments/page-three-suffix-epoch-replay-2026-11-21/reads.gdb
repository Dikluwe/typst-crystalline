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
trials=[]
pending_read=None
class Entry(gdb.Breakpoint):
 def __init__(self,address,kind):
  super().__init__('*'+hex(base+address),internal=True);self.kind=kind
 def stop(self):
  global publication,pending_read
  if self.kind=='wait':
   (case/f'publication-{publication}.pdf').write_bytes((case/'out.pdf').read_bytes())
   if publication==3:
    (case/'counts.json').write_text(json.dumps(dict(counts),indent=2))
    (case/'resume.json').write_text(json.dumps(resume,indent=2))
    (case/'trials.json').write_text(json.dumps(trials,indent=2));return True
   publication+=1
   pending=case/'pending.typ';pending.write_bytes((case/'edits'/f'{publication}.typ').read_bytes());pending.replace(case/'main.typ')
  else:
   counts[f"{publication}:{self.kind}"]+=1
   if self.kind=='read-call' and publication==3:
    target=int(gdb.parse_and_eval('*(unsigned long*)($rax+0x28)'))
    pending_read=gdb.execute('info symbol '+hex(target),to_string=True).strip()
   if self.kind=='read-result' and publication==3:
    if (int(gdb.parse_and_eval('$al')) & 1)==0:
     trials.append(dict(publication=publication, failed_return_of=pending_read,
                        stack=gdb.execute('bt 5',to_string=True)))
    pending_read=None
   if self.kind=='cut':
    reason,prefix,reused=(int(gdb.parse_and_eval('$'+r)) for r in ('rdi','rsi','rdx'))
    trials.append(dict(publication=publication, reason=reason, prefix_pages=prefix,
                       reused_pages=reused, closures=counts[f"{publication}:line_commit"],
                       region_transitions=counts[f"{publication}:region"]))
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
