set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
start
python
from collections import Counter
import json
import os
from pathlib import Path
import gdb

case = Path.cwd()
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0], 16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2] == "00000000" and line.endswith(exe))
symbols = json.loads((case / "symbols.json").read_text())
counts = Counter()

class Point(gdb.Breakpoint):
    def __init__(self, offset, name):
        super().__init__("*" + hex(base + offset), internal=True)
        self.name = name

    def stop(self):
        if self.name == "watch_wait":
            (case / "counts.json").write_text(json.dumps(counts) + "\n")
            return True
        counts[self.name] += 1
        if self.name == "root":
            frame = gdb.newest_frame()
            while frame is not None:
                if "::physical_memo::lab_project" in (frame.name() or ""):
                    counts["projection_root"] += 1
                    break
                frame = frame.older()
        return False

for name, offsets in symbols.items():
    for offset in offsets:
        Point(offset, name)
end
continue
kill
