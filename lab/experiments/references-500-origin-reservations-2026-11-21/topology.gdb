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
reserve_mapper=None
symbols=json.loads((case/'addresses.json').read_text())
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
   kind=self.kind
   if kind=='shared_reuse':
    caller = gdb.execute('info symbol '+hex(int(gdb.parse_and_eval('*(unsigned long*)$rsp'))),
                         to_string=True)
    if '::lab_build_body' in caller:
     kind='plan_reuse'
    elif '::lab_introspect_suffix_index' in caller:
     kind='index_reuse'
    else:
     raise RuntimeError(caller)
   if kind=='reserve_switch' and os.getenv('TEKT_LAB_RESERVE_VERIFY'):
    gdb.execute('set $rax=0',to_string=True)
    gdb.execute('set $pc=*(unsigned long*)$rsp',to_string=True)
    gdb.execute('set $rsp=$rsp+8',to_string=True)
   if kind=='reserve' and reserve_mapper is None:
    attach_reservation_counter()
   counts[f"{publication}:{kind}"]+=1
   if kind in ('plan_reuse','index_reuse'):
    trials.append(dict(publication=publication,scope=kind,
                       result=int(gdb.parse_and_eval('$rdi'))))
   if kind=='reserve_reuse':
    parts,floor,lo,hi=(int(gdb.parse_and_eval('$'+r)) for r in ('rdi','rsi','rdx','rcx'))
    mask=(1<<64)-1
    trials.append(dict(publication=publication,scope=kind,parts=parts,
                       context_floor=floor,location_after=((hi&mask)<<64)|(lo&mask)))
   if kind=='block':
    reused,reason=(int(gdb.parse_and_eval('$'+r)) for r in ('rdi','rsi'))
    trials.append(dict(publication=publication,scope='block',reused_packets=reused,block_reason=reason))
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
def attach_reservation_counter():
 global reserve_mapper
 import re
 pc = int(gdb.parse_and_eval('$pc'))
 address = lambda value: int(value, 16)
 for instruction in gdb.execute('disassemble '+hex(pc),to_string=True).splitlines():
  if 'call' not in instruction: continue
  got = re.search(r'#[ ]+(0x[0-9a-f]+)',instruction)
  direct = re.search(r'call[q]?[ ]+(0x[0-9a-f]+)',instruction)
  if got:
   target = int(gdb.parse_and_eval('*(unsigned long*)'+got[1]))
  elif direct:
   target = address(direct[1])
  else:
   continue
  if target-base in symbols['mappers']:
   reserve_mapper=Entry(target-base,'reserve_visit')
   return
 raise RuntimeError('source reservation mapper not found in the pinned ELF')
for kind,addresses in symbols.items():
 if kind != 'mappers':
  for address in addresses: Entry(address,kind)
end
continue
kill
