"""Diagnostic-only GDB script. Offsets apply only to release 70206018."""
import collections
import gdb
import json
import os

SYMBOL = "_ZN10typst_core8compiler7realize11table_cells12realize_cell17haf77255a6373af52E"


def pointer(address):
    return int.from_bytes(gdb.selected_inferior().read_memory(address, 8), "little")


class CellEntry(gdb.Breakpoint):
    def __init__(self):
        super().__init__(f"'{SYMBOL}'", internal=True)
        self.silent = True
        self.counts = collections.Counter()

    def stop(self):
        frame = gdb.newest_frame()
        caller = frame.older().name() if frame.older() else None
        address = int(frame.read_register("rsi"))
        resolved = pointer(address + 0x230)
        standalone = pointer(address + 0x238)
        if resolved:
            stage = f"resolved:{int.from_bytes(gdb.selected_inferior().read_memory(resolved + 0x223, 1), 'little')}"
        elif standalone:
            stage = f"standalone:{int.from_bytes(gdb.selected_inferior().read_memory(standalone + 0x378, 1), 'little')}"
        else:
            stage = "raw"
        self.counts[(caller or "??", stage)] += 1
        return False


probe = CellEntry()
gdb.execute("run")
with open(os.environ["CELL_CALLSITE_OUT"], "w") as output:
    json.dump([{"caller": caller, "stage": stage, "count": count}
               for (caller, stage), count in probe.counts.items()], output)
