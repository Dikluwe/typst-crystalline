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
class Entry(gdb.Breakpoint):
 def __init__(self,address,kind):
  super().__init__('*'+hex(base+address),internal=True);self.kind=kind
 def stop(self):
  global publication
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
   if self.kind=='node-compare' and (int(gdb.parse_and_eval('$eflags')) & 64)==0:
    read=lambda expr: int(gdb.parse_and_eval(expr))
    at=read('$pc')-base
    event=read('*(unsigned long*)($rsp+8)')
    old=read('$r15')
    new=read('$r12')
    data=dict(publication=publication, branch=hex(at), packet=read(f'*(unsigned long*)({event}+0x28)'),
              phase=read(f'*(unsigned char*)({event}+0x30)'), page=read(f'*(unsigned long*)({event}+0x20)'))
    if at not in (0x20dc42c,0x20dc439):
     for label,node in [('old',old),('new',new)]:
      data[label]=dict(kind=read(f'*(unsigned long*)({node}+0x40)'),
                       location_lo=read(f'*(unsigned long*)({node})'),
                       location_hi=read(f'*(unsigned long*)({node}+8)'),
                       parent=read(f'*(unsigned long*)({node}+0x28)'),
                       role=read(f'*(unsigned char*)({node}+0x117)'),
                       original=read(f'*(unsigned char*)({node}+0x115)'),
                       math=read(f'*(unsigned char*)({node}+0x116)'),
                       children_len=read(f'*(unsigned long*)({node}+0xc8)'),
                       auxiliary_len=read(f'*(unsigned long*)({node}+0xe0)'))
    trials.append(data)
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
