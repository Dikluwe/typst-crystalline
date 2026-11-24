set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
start
python
from collections import Counter
import gdb
import json
import os
from pathlib import Path

case = Path.cwd()
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0], 16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2] == "00000000" and line.endswith(exe))
symbols = json.loads((case / "symbols.json").read_text())
counts = [Counter() for _ in range(4)]
publication = 0

class Point(gdb.Breakpoint):
    def __init__(self, addr, kind):
        super().__init__("*" + hex(base + addr), internal=True)
        self.kind = kind

    def stop(self):
        global publication
        counts[publication][self.kind] += 1
        if self.kind == "watch_wait":
            (case / f"publication-{publication}.pdf").write_bytes((case / "out.pdf").read_bytes())
            if publication == 3:
                (case / "counts.json").write_text(json.dumps(counts, indent=2) + "\n")
                return True
            publication += 1
            pending = case / "main.typ.new"
            pending.write_bytes((case / "edits" / f"{publication}.typ").read_bytes())
            pending.replace(case / "main.typ")
        return False

for kind, addresses in symbols.items():
    for address in addresses:
        Point(address, kind)
end
continue
kill
