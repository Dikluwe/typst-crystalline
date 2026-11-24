set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
start
python
import gdb
import json
import os
import shutil
import time
from pathlib import Path

folder=Path.cwd()
pid=gdb.selected_inferior().pid
exe=os.readlink(f'/proc/{pid}/exe')
base=next(int(line.split()[0].split('-')[0],16) for line in
    Path(f'/proc/{pid}/maps').read_text().splitlines()
    if line.split()[2]=='00000000' and line.endswith(exe))
symbols=json.loads((folder/'symbols.json').read_text())
cases=[folder/'comment.typ',folder/'comment-2.typ',folder/'visible.typ']
rows=[]
events=[]

class Result(gdb.FinishBreakpoint):
    def __init__(self, frame, name):
        super().__init__(frame, internal=True)
        self.name = name

    def stop(self):
        bit = int(gdb.parse_and_eval('$al')) & 0xff
        if bit not in (0, 1):
            raise RuntimeError((self.name, bit))
        events.append(f'{self.name}:{bit}')
        return False

class Point(gdb.Breakpoint):
    def __init__(self, address, name):
        super().__init__('*'+hex(base+address),internal=True)
        self.name=name

    def stop(self):
        if self.name in ('page-input', 'dependencies'):
            Result(gdb.newest_frame(), self.name)
            return False
        if self.name=='document_clone':
            events.append('paged_document_clone')
            frame=gdb.newest_frame().older()
            while frame is not None:
                if 'physical_memo::finish_replay' in (frame.name() or ''):
                    events.append('defensive_copy')
                    break
                frame=frame.older()
            return False
        if self.name!='wait':
            events.append(self.name)
            return False
        index=len(rows)
        shutil.copyfile(folder/'out.pdf',folder/f'{index}.pdf')
        rows.append(dict(cycle=index,utc_ns=time.time_ns(),events=list(events)))
        (folder/'observed.json').write_text(json.dumps(rows,indent=2)+'\n')
        events.clear()
        if index==len(cases):
            return True
        (folder/'main.typ').write_bytes(cases[index].read_bytes())
        return False

for name, addresses in symbols.items():
    for address in addresses:
        Point(address,name)
end
continue
kill
