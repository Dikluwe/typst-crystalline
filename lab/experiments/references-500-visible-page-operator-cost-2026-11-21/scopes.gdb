set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
start
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
meta = json.loads((case / "symbols.json").read_text())
durations = defaultdict(list)
publication = 0
active_stream = 0
cid = None
cid_map = None

def cpu():
    return time.clock_gettime_ns((~pid << 3) | 2)

class Return(gdb.FinishBreakpoint):
    def __init__(self, kind):
        super().__init__(gdb.newest_frame(), internal=True)
        self.kind = kind
        self.started = cpu()

    def stop(self):
        global active_stream
        durations[self.kind].append(cpu() - self.started)
        if self.kind == "serialize":
            active_stream -= 1
        return False

class Point(gdb.Breakpoint):
    def __init__(self, addr, kind):
        super().__init__("*" + hex(base + addr), internal=True)
        self.kind = kind

    def stop(self):
        global publication, active_stream, cid, cid_map
        kind = self.kind
        if kind == "watch_wait":
            (case / f"publication-{publication}.pdf").write_bytes((case / "out.pdf").read_bytes())
            if publication == 3:
                assert active_stream == 0 and cid is None and cid_map is None
                (case / "scopes.json").write_text(json.dumps(durations, indent=2) + "\n")
                return True
            publication += 1
            pending = case / "main.typ.new"
            pending.write_bytes((case / "edits" / f"{publication}.typ").read_bytes())
            pending.replace(case / "main.typ")
        elif publication == 3:
            if kind == "cid_start":
                assert cid is None
                cid = cpu()
            elif kind == "cid_end":
                if cid is not None:
                    durations["cid"].append(cpu() - cid)
                    cid = None
            elif kind == "cid_map_start":
                assert cid_map is None
                cid_map = cpu()
            elif kind == "cid_map_end":
                assert cid_map is not None
                durations["cid_map"].append(cpu() - cid_map)
                cid_map = None
            elif kind == "vanilla_page_filter":
                # FlateMemoized takes discriminant 1 in this exact ELF.
                if int(gdb.parse_and_eval("$esi")) & 255 == 1:
                    Return(kind)
            elif kind == "vanilla_deflate":
                caller = gdb.newest_frame().older()
                # Return PC of the cache-miss call in FlateMemoized.
                if caller is not None and caller.pc() - base == 0x1e12177:
                    Return("vanilla_page_deflate")
            elif kind != "compression" or active_stream:
                if kind == "serialize":
                    active_stream += 1
                Return(kind)
        return False

for kind, addresses in meta.items():
    for address in addresses:
        Point(address, kind)
end
continue
kill
