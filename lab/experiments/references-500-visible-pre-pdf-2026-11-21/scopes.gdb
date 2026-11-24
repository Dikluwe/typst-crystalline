set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
starti
python
from collections import defaultdict
import gdb, json, os, time
from pathlib import Path

case = Path.cwd()
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0], 16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2] == "00000000" and line.endswith(exe))
metadata = json.loads((case / "symbols.json").read_text())
spans = defaultdict(list)
pending = set()
publication = 0

def cpu(tid):
    return int(Path(f"/proc/{pid}/task/{tid}/schedstat").read_text().split()[0])

class Exit(gdb.FinishBreakpoint):
    def __init__(self, kind):
        super().__init__(gdb.newest_frame(), internal=True)
        self.kind = kind
        self.tid = gdb.selected_thread().ptid[1]
        pending.add(self)
        self.begin = cpu(self.tid)

    def stop(self):
        end = cpu(self.tid)
        spans[self.kind].append([self.tid, self.begin, end])
        pending.remove(self)
        return False

    def out_of_scope(self):
        spans["out_of_scope"].append(self.kind)
        pending.discard(self)

class Enter(gdb.Breakpoint):
    def __init__(self, address, kind):
        super().__init__("*" + hex(base + address), internal=True)
        self.kind = kind

    def stop(self):
        global publication
        if self.kind == "watch_wait":
            (case / f"publication-{publication}.pdf").write_bytes((case / "out.pdf").read_bytes())
            if publication == 3:
                assert not pending, [entry.kind for entry in pending]
                (case / "scopes.json").write_text(json.dumps(spans, indent=2) + "\n")
                return True
            publication += 1
            next_source = case / "main.typ.pending"
            next_source.write_bytes((case / "edits" / f"{publication}.typ").read_bytes())
            next_source.replace(case / "main.typ")
        elif publication == 3:
            Exit(self.kind)
        return False

for kind, addresses in metadata.items():
    for address in addresses:
        Enter(address, kind)
end
continue
kill
