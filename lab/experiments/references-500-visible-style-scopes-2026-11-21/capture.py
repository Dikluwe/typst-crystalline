"""Time and count captures in each original watch publication."""
import json
import os
from pathlib import Path
from statistics import median
import struct
import subprocess
import sys

TABLE = struct.Struct("<11Q")
WORD = struct.Struct("<Q")


def main(bench):
    work, out, base = bench.WORK, bench.OUT, bench.base
    os.sched_setaffinity(0, {2})
    probe = work / "capture.so"
    subprocess.run(["cc", "-O2", "-Wall", "-Wextra", "-Werror", "-fPIC", "-shared", "-o", probe,
                    Path(__file__).with_suffix(".c")], check=True)
    (out / "capture.sha256").write_text(subprocess.check_output(["sha256sum", probe], text=True))
    original_sources = base.cycle.base.runner.setup_sources
    original_env = base.environment

    def setup(case, *args):
        path = case / "capture.bin"
        if not path.exists():
            path.write_bytes(TABLE.pack(0, *([0] * 10)))
        with path.open("r+b") as stream:
            publication, = WORD.unpack(stream.read(WORD.size))
            stream.seek(0)
            stream.write(WORD.pack(publication + 1))
        original_sources(case, *args)

    def env(tmp, arm, trace):
        values = original_env(tmp, arm, trace)
        values.update(LD_PRELOAD=str(probe), TEKT_CAPTURE_TRACE=str(tmp.parent / "capture.bin"))
        return values

    base.cycle.base.runner.setup_sources = setup
    base.environment = env
    observations = []
    try:
        for i in range(20):
            case = work / "capture" / str(i)
            rows = base.watch(case, "candidate", bench.STATES)
            publication, calibration_count, calibration_ns, *totals = TABLE.unpack(
                (case / "capture.bin").read_bytes())
            phases = list(zip(totals[::2], totals[1::2]))
            assert publication == 4 and calibration_count == 512
            count, elapsed = phases[-1]
            assert count > 0
            observations.append(dict(calibration_ns=calibration_ns / calibration_count,
                                     adjusted_ns=elapsed-count*calibration_ns/calibration_count,
                                     phases=phases, measurements=rows))
    finally:
        base.cycle.base.runner.setup_sources = original_sources
        base.environment = original_env
    result = dict(
        median_ns=median(r["adjusted_ns"] for r in observations),
        min_ns=min(r["adjusted_ns"] for r in observations),
        max_ns=max(r["adjusted_ns"] for r in observations),
        median_count=median(r["phases"][-1][0] for r in observations),
        rows=observations,
    )
    (out / "capture.json").write_text(json.dumps(result, indent=2) + "\n")
    print({k: v for k, v in result.items() if k != "rows"}, flush=True)


if __name__ == "__main__":
    import bench
    main(bench)
