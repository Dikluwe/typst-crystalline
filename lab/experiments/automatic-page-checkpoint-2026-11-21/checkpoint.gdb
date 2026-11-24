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
frontiers={}
snapshots={}
# Offsets checked against next-region.asm and admit.asm for this exact ELF.
# The prefix count is the Vec<Page> length at the commit return site.
fields={
 'closed_pages':0x2020,'page_width':0x14e8,'page_height':0x14f0,
 'left_margin':0x14b0,'right_margin':0x14b8,'top_margin':0x14c0,'bottom_margin':0x14c8,
 'cursor_x':0x2070,'cursor_y':0x2078,'line_start_x':0x2080,
 'region_width':0x2088,'region_height':0x2090,
 'ascent_discriminant':0x1580,'ascent_payload':0x1588,
}
def scalar(address):
 memory=gdb.selected_inferior().read_memory(address,8)
 return int.from_bytes(memory.tobytes(),'little')
def state(address):
 result={k:scalar(address+offset) for k,offset in fields.items()}
 if not result['ascent_discriminant']:
  result.pop('ascent_payload')
 return result
class Successor(gdb.FinishBreakpoint):
 def __init__(self,address,probe):
  super().__init__(gdb.newest_frame(),internal=True)
  self.address=address;self.probe=probe
 def stop(self):
  after=state(self.address)
  self.probe['successor']=after
  active_emitter=any(name and ('::paragraph_emit::' in name or '::word_layout::' in name)
                     for name, _ in self.probe['stack'])
  refusals=[]
  if after['closed_pages']:
   refusals.append('existing outline checkpoint requires zero closed pages')
  if active_emitter:
   refusals.append('remaining paragraph or run is held by the active emitter')
  self.probe['refusals']=refusals
  self.probe['existing_checkpoint_admits']=not refusals
  self.probe['line_commits_after_transition']=counts.get(f"{publication}:line_commit",0)
  return False
class Entry(gdb.Breakpoint):
 def __init__(self,address,kind):
  super().__init__('*'+hex(base+address),internal=True);self.kind=kind
 def stop(self):
  global publication
  if self.kind=='wait':
   (case/f'publication-{publication}.pdf').write_bytes((case/'out.pdf').read_bytes())
   if publication==3:
    (case/'counts.json').write_text(json.dumps(dict(counts),indent=2))
    (case/'frontiers.json').write_text(json.dumps(frontiers,indent=2))
    (case/'snapshots.json').write_text(json.dumps(snapshots,indent=2));return True
   publication+=1
   pending=case/'pending.typ';pending.write_bytes((case/'edits'/f'{publication}.typ').read_bytes());pending.replace(case/'main.typ')
  else:
   counts[f"{publication}:{self.kind}"]+=1
   if self.kind=='region':
    stack=gdb.execute('bt 19',to_string=True)
    frontiers.setdefault(publication,[]).append(stack)
    if int(gdb.parse_and_eval('$pc'))-base == 0x26aa7b8:
     address=int(gdb.parse_and_eval('$rdi'))
     frames=[]; frame=gdb.newest_frame()
     while frame is not None and len(frames)<19:
      frames.append((frame.name(),frame.pc()-base))
      frame=frame.older()
     before=state(address)
     probe={
      'before':before,'stack':frames,
      'line_commits_before':counts.get(f"{publication}:line_commit",0),
     }
     caller=gdb.newest_frame().older()
     if caller is not None and caller.pc()-base==0x26ba7f5:
      probe['pending_selected_line']={
       'index':int(caller.read_register('r14')),
       'len':scalar(int(caller.read_register('rsp'))+0x18),
      }
     snapshots.setdefault(publication,[]).append(probe)
     Successor(address,probe)
  return False
for kind,addresses in json.loads((case/'addresses.json').read_text()).items():
 for address in addresses:Entry(address,kind)
end
continue
kill
