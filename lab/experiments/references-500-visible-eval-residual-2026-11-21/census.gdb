set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
starti
python
from collections import Counter, defaultdict
import gdb, json, os
from pathlib import Path

case = Path.cwd()
inferior = gdb.selected_inferior()
pid = inferior.pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0],16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2]=="00000000" and line.endswith(exe))
addresses = json.loads((case/"addresses.json").read_text())
points = []
counts = defaultdict(Counter)
publication = 0
active = False

class Exit(gdb.FinishBreakpoint):
    def stop(self):
        global active
        active = False
        for point in points:
            point.enabled = False
        return False

class Point(gdb.Breakpoint):
    def __init__(self,address,kind):
        super().__init__("*"+hex(base+address),internal=True)
        self.kind = kind
        if kind not in ("watch_wait","eval"):
            self.enabled = False
            points.append(self)

    def stop(self):
        global publication, active
        kind = self.kind
        if kind == "watch_wait":
            (case/f"pub-{publication}.pdf").write_bytes((case/"out.pdf").read_bytes())
            if publication == 3:
                assert not active
                (case/"counts.json").write_text(json.dumps(counts,indent=2)+"\n")
                return True
            publication += 1
            new = case/"new.typ"
            new.write_bytes((case/"edits"/f"{publication}.typ").read_bytes())
            new.replace(case/"main.typ")
        elif publication == 3 and kind == "eval":
            counts["eval"][0] += 1
            if not active:
                active = True
                Exit(gdb.newest_frame(),internal=True)
                for point in points:
                    point.enabled = True
        elif active and kind.startswith("native_"):
            counts[kind][0] += 1
        elif active:
            rsp = int(gdb.newest_frame().read_register("rsp"))
            ret = int.from_bytes(inferior.read_memory(rsp,8),"little")
            counts[kind][ret-base] += 1
        return False

for kind, values in addresses.items():
    for address in values:
        Point(address,kind)
end
continue
kill
