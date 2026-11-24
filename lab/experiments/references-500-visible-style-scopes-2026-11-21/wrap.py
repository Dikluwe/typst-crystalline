"""Measure the short vanilla wrapper and associate calls with the fourth publication."""
import json
import os
from pathlib import Path
from statistics import median
import struct
import subprocess

ROW = struct.Struct("<QQQ")


def eval_range(case):
    clocks = dict(struct.iter_unpack("<QQ", (case / "clock.bin").read_bytes()))
    trace = json.loads((case / "timings-3.json").read_text())
    offsets = [round(item["ts"] * 1000) for item in trace]
    zero, = (clock for clock in clocks if all(clock + off in clocks for off in offsets[:12]))
    return [clocks[zero + off] for item, off in zip(trace, offsets) if item["name"] == "eval"]


def main(work, out, native):
    probe = work / "vanilla-wrap.so"
    subprocess.run(["cc", "-O2", "-Wall", "-Wextra", "-Werror", "-shared", "-fPIC", "-o", probe,
                    Path(__file__).with_name("vanilla-wrap.c")], check=True)
    res = subprocess.check_output(["sha256sum", probe], text=True)
    (out / "vanilla-wrap.sha256").write_text(res)
    old_env = native.cycle.environment

    def environment(tmp, arm, trace):
        env = old_env(tmp, arm, trace)
        env["TEKT_VANILLA_WRAP"] = str(tmp.parent / "wrap.bin")
        return env

    original_child = native.driver.Child

    def traced_child(command, case, env, *args):
        env["LD_PRELOAD"] += ":" + str(probe)
        return original_child(command, case, env, *args)

    native.cycle.environment = environment
    native.driver.Child = traced_child
    gold = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]["vanilla"]
    results = []
    try:
        for block in range(12, 32):
            row = native.run(block, gold)
            case = work / "vanilla-traces" / f"{block:02d}"
            lower, upper = eval_range(case)
            items = list(ROW.iter_unpack((case / "wrap.bin").read_bytes()))
            calibration = median(end - begin for kind, begin, end in items if kind == 0)
            calls = [end - begin for kind, begin, end in items if kind == 1 and lower <= begin <= end <= upper]
            assert len(calls) == 2, (block, calls, items)
            assert all(kind in (0, 1) and begin <= end for kind, begin, end in items)
            results.append(dict(**row, eval_range_ns=[lower, upper], calibration_ns=calibration,
                                all_calls=sum(kind == 1 for kind, *_ in items), visible_calls_ns=calls,
                                adjusted_ns=sum(calls) - len(calls) * calibration))
    finally:
        native.cycle.environment = old_env
        native.driver.Child = original_child
    output = dict(
        clock_read_pair_ns=median(r["calibration_ns"] for r in results),
        median_adjusted_ns=median(r["adjusted_ns"] for r in results),
        raw_median_ns=median(sum(r["visible_calls_ns"]) for r in results),
        min_adjusted_ns=min(r["adjusted_ns"] for r in results),
        max_adjusted_ns=max(r["adjusted_ns"] for r in results),
        rows=results,
    )
    (out / "vanilla-wrap.json").write_text(json.dumps(output, indent=2) + "\n")
    print("vanilla lexical wrapping", {k: v for k, v in output.items() if k != "rows"}, flush=True)

