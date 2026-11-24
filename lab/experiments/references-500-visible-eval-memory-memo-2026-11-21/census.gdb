set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
starti
python
import gdb, json, os
from pathlib import Path

case = Path.cwd()
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0], 16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2] == "00000000" and line.endswith(exe))
publications = [[]]
enabled = False

class Exit(gdb.FinishBreakpoint):
    def stop(self):
        global enabled
        enabled = False
        return False

class Point(gdb.Breakpoint):
    def __init__(self, address, kind):
        super().__init__("*"+hex(base+address), internal=True)
        self.kind = kind

    def stop(self):
        global enabled
        kind = self.kind
        if kind == "wait":
            i = len(publications)-1
            (case/f"pub-{i}.pdf").write_bytes((case/"out.pdf").read_bytes())
            if i == 3:
                (case/"census.json").write_text(json.dumps(publications,indent=2)+"\n")
                return True
            publications.append([])
            new = case/"next.typ"
            new.write_bytes((case/"edits"/f"{i+1}.typ").read_bytes())
            new.replace(case/"main.typ")
        elif kind == "eval":
            assert not enabled
            enabled = True
            Exit(gdb.newest_frame(),internal=True)
        elif enabled:
            loops = publications[-1]
            if kind == "loop":
                loops.append({"body":0, "miss":0, "hit":0, "store":0, "evictions":[], "retained_bytes":0, "retained_entries":0})
            else:
                row = loops[-1]
                if kind == "evict":
                    row["evictions"].append(int(gdb.parse_and_eval("$r12")))
                elif kind == "charged":
                    row["retained_bytes"] = int(gdb.parse_and_eval("$rax"))
                    row["retained_entries"] = int(gdb.parse_and_eval("$r12"))+1
                else:
                    row[kind] += 1
        return False

for kind,address in {
    "wait":0x2915e50, "eval":0x29aab68, "loop":0x1fca2b0,
    "body":0x1fca612, "miss":0x1fcb3c5, "hit":0x1fcba7b,
    "store":0x1fcab28, "charged":0x1fcaabb, "evict":0x1fcaa93,
}.items():
    Point(address,kind)
end
continue
kill
