set pagination off
set confirm off
set debuginfod enabled off
maintenance set worker-threads 1
set disable-randomization off
start
python
import gdb, json, os, struct, time
from pathlib import Path

case = Path.cwd()
meta = json.loads((case / "addresses.json").read_text())
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0], 16)
            for line in Path(f"/proc/{pid}/maps").read_text().splitlines()
            if line.split()[2] == "00000000" and line.endswith(exe))
events = []
publication = 0
active = []

def registers(name):
    return int(gdb.parse_and_eval("$" + name))

def read_word(addr):
    return struct.unpack("<Q", gdb.selected_inferior().read_memory(addr, 8))[0]

def clock():
    return time.clock_gettime_ns((~pid << 3) | 2)

def mark(kind, **data):
    events.append(dict(publication=publication, kind=kind, memo=active[-1] if active else None,
                       process_cpu_ns=clock(), **data))
    return len(events) - 1

class Return(gdb.FinishBreakpoint):
    def __init__(self, kind, number):
        super().__init__(gdb.newest_frame(), internal=True)
        self.kind, self.event_id = kind, number

    def stop(self):
        row = events[self.event_id]
        row["inclusive_cpu_ns"] = clock() - row["process_cpu_ns"]
        if self.kind in ("page_inputs", "journal", "pagination"):
            row["valid"] = bool(registers("rax") & 255)
        if self.kind == "pagination":
            row["covered"] = bool(registers("rdx") & 255)
        if self.kind == "memo":
            assert active.pop() == self.event_id
        return False

class Point(gdb.Breakpoint):
    def __init__(self, offset, kind, detail=None):
        super().__init__("*" + hex(base + offset), internal=True)
        self.kind, self.detail = kind, detail

    def stop(self):
        global publication
        kind = self.kind
        if kind == "watch_wait":
            (case / f"publication-{publication}.pdf").write_bytes((case / "out.pdf").read_bytes())
            mark(kind)
            if publication == 3:
                (case / "events.json").write_text(json.dumps(events, indent=2) + "\n")
                return True
            publication += 1
            pending = case / "main.typ.new"
            pending.write_bytes((case / "edits" / f"{publication}.typ").read_bytes())
            pending.replace(case / "main.typ")
        elif kind == "plan_test":
            pointer_reg, prior_reg = self.detail
            current = read_word(registers(pointer_reg) + 0x110)
            previous = registers(prior_reg)
            mark(kind, same=current == previous)
        elif kind == "journal_check":
            if registers("rax") & 255 == 0:
                slot = registers("r12") + registers("rbp")
                table = read_word(slot + 16)
                function = read_word(table + 40)
                mark(kind, check="index" if read_word(slot) & 0xffffffff == 1 else "metric",
                     check_symbol=gdb.execute("info symbol " + hex(function), to_string=True).strip())
        elif kind == "pagination_stage":
            mark(kind, stage=self.detail)
        elif kind == "pagination":
            lhs, rhs = [read_word(registers(reg)) for reg in ("rdi", "rsi")]
            i = mark(kind, page_total=[read_word(addr + 0x2b0) for addr in (lhs, rhs)],
                     position_count=[read_word(addr + 0x720) for addr in (lhs, rhs)])
            Return(kind, i)
        else:
            i = mark(kind)
            if kind == "memo":
                active.append(i)
            if kind in ("memo", "root", "page_inputs", "journal", "outline"):
                Return(kind, i)
        return False

for kind, addresses in meta["symbols"].items():
    for address in addresses:
        Point(address, kind)
for address, lhs, rhs in meta["plan_tests"]:
    Point(address, "plan_test", (lhs, rhs))
for address in meta["journal_checks"]:
    Point(address, "journal_check")
# Both binary-specific instruction layouts were checked against objdump.
for address in meta["symbols"]["pagination"]:
    for stage, offset in (("coverage_equal", 0x184), ("page_store_equal", 0x2d1),
                          ("page_counters_at_location_equal", 0x2e7), ("page_counters_equal", 0x2fd), ("position_counts_equal", 0x30d)):
        Point(address + offset, "pagination_stage", stage)
end
continue
kill
