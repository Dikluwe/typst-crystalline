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
   if self.kind=='terminal_sequence' and publication==3:
    return True
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
   counts[f"{publication}:{kind}"]+=1
   if kind in ('plan_reuse','index_reuse'):
    trials.append(dict(publication=publication,scope=kind,
                       result=int(gdb.parse_and_eval('$rdi'))))
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
for kind,addresses in json.loads((case/'addresses.json').read_text()).items():
 for address in addresses:Entry(address,kind)
end
continue
python
gdb.execute('set language c')
terminal=0x1fa42f0
stops=(0x1fa4447,0x1fa4449)
for _ in range(280):
 pc=int(gdb.parse_and_eval('$pc'))-base
 registers=[hex(int(gdb.parse_and_eval('$'+r))) for r in ('rax','rcx','rdx','rsi','r12','r13')]
 print('GUARD',hex(pc),*registers)
 if pc == 0x1fa43d1:
  print('COUNTS',int(gdb.parse_and_eval('*(unsigned long*)($rcx+0xc8)')),
        int(gdb.parse_and_eval('*(unsigned long*)($rcx+0x50)')),
        int(gdb.parse_and_eval('$r12')))
  import struct
  inferior=gdb.selected_inferior()
  def word(addr):
   return struct.unpack('<Q',bytes(inferior.read_memory(addr,8)))[0]
  node=int(gdb.parse_and_eval('$rcx'))
  old=word(node+0x48)+16
  new=word(int(gdb.parse_and_eval('$rsp'))+0x40)
  n=word(node+0x50)
  m=int(gdb.parse_and_eval('$r12'))
  def shape(addr):
   a=word(addr)
   if a==2:
    payload=word(addr+8)
    return (a,word(payload+0x88),word(payload+0x10))
   if a>127:
    return ('text',)
   return (a,)
  a=[shape(old+120*i) for i in range(n)]
  b=[shape(new+120*i) for i in range(m)]
  if os.getenv('TEKT_LAB_COMPARE_PROOF'):
   f=base+0x1faba30
   for index in sorted(set(range(100,n-3,59)).union(i for i in range(100,n-3) if a[i][0] not in (2,7,9,10,11,'text'))):
    left,right=old+120*index,new+120*index
    result=int(gdb.parse_and_eval('((unsigned long (*)(void*,void*))'+hex(f)+')('+hex(left)+','+hex(right)+')'))
    if result & 0xffffffffffffffff:
     print('COMPARE_PROOF',index,a[index],result>>64)
  if os.getenv('TEKT_LAB_END_PROBE'):
   old_edges=word(node+0xc0)
   first=int(gdb.parse_and_eval('*(unsigned long*)($rsp+0x100)'))
   index=next(i for i in range(n) if word(old_edges+24*i+16)>=first)
   print('FRONTIER',first,index,word(old_edges+24*index+16))
   for i in range(index,n-2):
    result=int(gdb.parse_and_eval('((unsigned long (*)(void*,void*))'+hex(base+0x1faba30)+')('+hex(old+120*i)+','+hex(new+120*i)+')'))
    if result & 0xffffffffffffffff:
     print('FULL_COMPARE_REFUSED',i,a[i],result>>64)
     break
   else:
    print('FULL_COMPARE_PASSED',n-2-index)
   gdb.execute('set $r12='+str(n))
  from difflib import SequenceMatcher
  for diff in SequenceMatcher(None,a,b,autojunk=False).get_opcodes():
   print('ALIGN',diff)
  print('TAIL',a[-8:],b[-8:])
  print('PURE_LENGTH',n-1-max(i for i,x in enumerate(a) if x[0] not in (6,7,9,10,11)),m-1-max(i for i,x in enumerate(b) if x[0] not in (6,7,9,10,11)))
 if os.getenv('TEKT_LAB_END_PROBE') and pc==0x1fa4c29: print('SLOT_MARK_FOUND')
 if pc in stops: break
 gdb.execute('nexti',to_string=True)
(case/'counts.json').write_text('{}')
(case/'resume.json').write_text('[]')
if os.getenv('TEKT_LAB_END_PROBE'):
 gdb.execute('tbreak *'+hex(base+0x1fa4577),to_string=True)
 gdb.execute('continue',to_string=True)
 if int(gdb.parse_and_eval('$pc'))-base == 0x1fa4577:
  builder=int(gdb.parse_and_eval('*(unsigned long*)($rsp+8)'))
  print('ALLOCATION_CUT',
    word(builder+0x20),word(int(gdb.parse_and_eval('$rsp'))+0x58),
    word(builder+0x38),word(int(gdb.parse_and_eval('$rsp'))+0xe8),
    int(gdb.parse_and_eval('$rbp')),word(int(gdb.parse_and_eval('$rsp'))+0x48))
end
kill
