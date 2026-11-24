set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
starti
python
import gdb, json, os
from collections import Counter
from pathlib import Path

case = Path.cwd()
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0],16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2]=="00000000" and line.endswith(exe))
addresses = json.loads((case/"addresses.json").read_text())
counts = Counter()
publication = 0
in_eval = False
in_group = 0

class Leave(gdb.FinishBreakpoint):
    def __init__(self, kind):
        super().__init__(gdb.newest_frame(),internal=True)
        self.kind = kind

    def stop(self):
        global in_eval, in_group
        if self.kind == "eval":
            assert not in_group
            in_eval = False
        else:
            in_group -= 1
        return False

    def out_of_scope(self):
        counts["unwound"] += 1

class Join(gdb.Breakpoint):
    def __init__(self, address, kind):
        super().__init__("*"+hex(base+address),internal=True)
        self.kind = kind

    def stop(self):
        global publication, in_eval, in_group
        if self.kind == "wait":
            (case/f"pub-{publication}.pdf").write_bytes((case/"out.pdf").read_bytes())
            if publication == 3:
                assert not in_eval and not in_group
                (case/"census.json").write_text(json.dumps(counts,indent=2)+"\n")
                return True
            publication += 1
            update = case/"new.typ"
            update.write_bytes((case/"edits"/f"{publication}.typ").read_bytes())
            update.replace(case/"main.typ")
        elif publication == 3:
            if self.kind == "eval":
                assert not in_eval
                in_eval = True
                Leave("eval")
            elif in_eval:
                if self.kind == "group":
                    counts["comparisons"] += 1
                    in_group += 1
                    Leave("group")
                elif self.kind == "collapse":
                    counts["eval_collapses"] += 1
                    if in_group:
                        counts["group_collapses"] += 1
        return False

for kind, addresses in addresses.items():
    for address in addresses:
        Join(address,kind)
end
continue
kill
