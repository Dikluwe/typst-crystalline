"""Associate clocks with the existing source phase boundaries by exact ELF PCs."""
from collections import Counter
from itertools import permutations
import json
from pathlib import Path
from random import Random
import re
from statistics import median
import struct
import subprocess

D = Path(__file__).resolve().parent
TABLE = struct.Struct(f"<{2 + 3 * 1024}Q")
NAMES = (
    "::pipeline::paged::compile_paged_in_session",
    "::pipeline::pdf::compile_to_pdf_bytes_impl",
)

def setup(bench, arm):
    exe = bench.WORK / arm
    symbols = bench.decoder.symbols(bench.WORK / f"{arm}.symbols")[0]
    scope = {name: next((start, start + size) for start, (size, symbol) in symbols.items()
                        if symbol.endswith(name)) for name in NAMES}
    target, stop = next((start, start + size) for start, (size, name) in symbols.items()
                        if name.endswith("::time::Timespec::now"))
    chunks = [subprocess.check_output(["objdump", "-dC", f"--start-address={lo}",
                                       f"--stop-address={hi}", exe], text=True)
              for lo, hi in scope.values()]
    listing = "".join(chunks)
    (bench.OUT / f"{arm}-phases.asm").write_text(listing)
    instant_calls = {int(site,16)+5 for site in re.findall(
        rf"^\s*([0-9a-f]+):\s+e8 .*\bcall\s+{target:x} ", listing, flags=re.M)}
    assert len(instant_calls) >= 8, (arm, target, sorted(instant_calls))
    path = bench.WORK / f"{arm}-clock"
    path.mkdir()
    (path / "ranges.inc").write_text("".join(f"{{{lo:#x},{hi:#x}}},\n" for lo, hi in scope.values()))
    probe = path / "timing.so"
    subprocess.run(["cc", "-O2", "-Wall", "-Wextra", "-Werror", "-fPIC", "-shared", "-I", path,
                    f"-DTIMESPEC_LO={target:#x}", f"-DTIMESPEC_HI={stop:#x}", "-o", probe,
                    D / "phase-clock.c", "-ldl"], check=True)
    return probe, scope, instant_calls


def run(bench, helper, gold):
    arms = ("candidate", "bridge", "proof")
    specs = {arm: setup(bench, arm) for arm in arms}
    records = {arm: [] for arm in arms}
    queue = list(permutations(arms)) * 2
    Random(13005).shuffle(queue)
    for block, order in enumerate(queue):
        for arm in order:
            probe, scope, instant_calls = specs[arm]
            folder = bench.WORK / "clock" / f"{arm}-{block:02d}"
            with helper.tracing(bench.base, probe, TABLE):
                pdf = bench.run_watch(folder, arm, gold, keep=False)
            publication, length, *items = TABLE.unpack((folder / "realization.bin").read_bytes())
            assert publication == 4 and length <= 1024
            marks = list(zip(items[::3], items[1::3], items[2::3]))[:length]
            assert all(pc in instant_calls for _, pc, _ in marks), (arm, marks)
            by_pub = {i: [(pc, ns) for pub, pc, ns in marks if pub == i] for i in range(1,5)}
            assert len(by_pub[4]) >= 8, (arm, by_pub)
            records[arm].append(dict(block=block, marks=by_pub, measurements=pdf))
        print(f"clock block {block+1}/{len(queue)}", flush=True)
    collected = {}
    for arm, batch in records.items():
        counts = Counter(tuple(pc for pc, _ in row["marks"][4]) for row in batch)
        assert len(counts) == 1, (arm, counts)
        order = next(iter(counts))
        collected[arm] = dict(ranges=specs[arm][1], pcs=[hex(pc) for pc in order], records=batch,
                              interval_medians_ms=[median((row["marks"][4][i+1][1]
                                  - row["marks"][4][i][1]) / 1e6 for row in batch)
                                  for i in range(len(order)-1)])
        print("clock", arm, collected[arm]["pcs"], collected[arm]["interval_medians_ms"], flush=True)
    bench.dump("phase-clock.json", collected)
