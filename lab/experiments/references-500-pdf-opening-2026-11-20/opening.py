"""First watch publication in fresh processes, with separate nominal and sampled arms."""
import gzip
import importlib.util
import json
import os
from pathlib import Path
import random
import signal
import struct
import sys
import subprocess
import time

HERE = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
PRIOR = HERE.parent / "references-500-pdf-export-reuse-2026-11-20"
spec = importlib.util.spec_from_file_location("reuse", PRIOR / "bench.py")
reuse = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reuse)
driver, measure = reuse.driver, reuse.measure
ARMS = ("both", "off", "vanilla")
PERIODS = (677_000, 937_000)
STATE = reuse.REGULAR[0]


def run(group, block, arm, period=0, directed=False, ignore_fonts=False):
    directory = WORK / "cases" / group / f"{block:02d}-{arm}"
    directory.mkdir(parents=True)
    tmp, pdf = directory / "tmp", directory / "out.pdf"
    tmp.mkdir()
    reuse.runner.setup_sources(directory, *STATE[1:])
    exe = WORK / ("vanilla" if arm == "vanilla" else "candidate")
    args = [str(exe), "watch", str(directory / "main.typ"), str(pdf),
            "--root", str(directory), "--pdf-tagged=true"]
    if ignore_fonts:
        args.append("--ignore-system-fonts")
    env = driver.environment(tmp)
    if arm != "vanilla":
        env.update(TEKT_LAB_WATCH_RETAIN="1", TEKT_LAB_WATCH_LAZY="1",
                   TEKT_LAB_WATCH_INERT="1", TEKT_LAB_PDF_REUSE=arm)
        if directed:
            env["TEKT_LAB_WATCH_TRACE"] = "1"
    if period:
        env.update(LD_PRELOAD=str(WORK / "profiler.so"), TEKT_CPU_AUTOSTART="1",
                   TEKT_CPU_TRACE=str(directory / "samples.bin"),
                   TEKT_CPU_ACK=str(directory / "samples.ack"), TEKT_CPU_PERIOD=str(period))
    if group.startswith("vanilla-cpu-trace"):
        env.update(LD_PRELOAD=str(WORK / "trace-clock.so"),
                   TEKT_TRACE_CLOCK=str(directory / "clock.bin"))
    if directed:
        if arm == "vanilla":
            args.extend(["--timings", str(directory / "timings.json")])
        else:
            args.extend(["--timings-json", str(directory / "timings.json")])
    log_path = directory / "watch.log"
    utc = measure.utc()
    with log_path.open("wb") as log:
        start = time.perf_counter_ns()
        child = driver.Child(args, directory, env, 1, log)
        try:
            _, published, _ = driver.await_pdf(child, pdf, None)
            cpu = measure.settle(child)
            if period:
                pid = child.process.pid
                (directory / "maps").write_text(Path(f"/proc/{pid}/maps").read_text())
                os.kill(pid, signal.SIGUSR2)
                deadline = time.monotonic() + 5
                while (directory / "samples.ack").read_bytes() != struct.pack("<i", 2):
                    if time.monotonic() > deadline:
                        raise TimeoutError(directory)
                    time.sleep(0.001)
        finally:
            child.stop()
            child.close()
    assert log_path.read_text().count("compiled successfully") == 1, log_path.read_text()
    assert measure.sha(directory / "main.typ") == "658f5f45d99f9245c7d700d7b5b7d87df7276885b35403c95ab0b889fb39b1e6"
    row = dict(group=group, block=block, arm=arm, period_ns=period, utc=utc,
               cpu_ns=cpu, wall_ns=published-start, exe=str(exe), directory=str(directory.relative_to(WORK)),
               pdf_sha256=measure.sha(pdf))
    print(json.dumps(row), flush=True)
    return row


def main():
    assert 1 in os.sched_getaffinity(0) and 2 in os.sched_getaffinity(0)
    os.sched_setaffinity(0, {2})
    if "--font-control" in sys.argv:
        cases = (("font-default", "both", False, False),
                 ("font-default", "vanilla", False, False),
                 ("vanilla-cpu-trace-default", "vanilla", True, False),
                 ("font-ignored", "vanilla", False, True),
                 ("vanilla-cpu-trace-ignored", "vanilla", True, True))
        rng = random.Random(20501)
        with (WORK / "font-control.jsonl").open("w") as output:
            for block in range(12):
                for group, arm, directed, ignore in rng.sample(cases, len(cases)):
                    row = run(group, block, arm, directed=directed, ignore_fonts=ignore)
                    output.write(json.dumps(row) + "\n")
                    output.flush()
            for block in range(12):
                name = f"{block:02d}-vanilla"
                default = WORK / "cases" / "font-default" / name / "out.pdf"
                ignored = WORK / "cases" / "font-ignored" / name / "out.pdf"
                assert measure.sha(default) == measure.sha(ignored), block
            print("12 vanilla PDF pairs byte-identical", flush=True)
        return
    if "--trace-cpu-only" in sys.argv:
        with (WORK / "trace-cpu.jsonl").open("w") as output:
            for block in range(12):
                row = run("vanilla-cpu-trace", block, "vanilla", directed=True)
                output.write(json.dumps(row) + "\n")
        return
    if "--directed-only" in sys.argv:
        rows = [run("verify", n, arm, period) for n, period in enumerate((0, *PERIODS)) for arm in ARMS]
    else:
        for arm in ARMS:
            run("warmup", 0, arm)
        rng = random.Random(20500)
        rows = []
        schedule = [("nominal", n, 0) for n in range(18)]
        schedule += [(f"profile-{p}", n, p) for p in PERIODS for n in range(12)]
        rng.shuffle(schedule)
        with (WORK / "measurements.jsonl").open("w") as output:
            for group, block, period in schedule:
                arms = list(ARMS)
                rng.shuffle(arms)
                for arm in arms:
                    row = run(group, block, arm, period)
                    rows.append(row)
                    output.write(json.dumps(row) + "\n")
                    output.flush()
    for arm in ARMS:
        rows.append(run("directed", 0, arm, directed=True))
    checked = {}
    for row in rows:
        if row["block"] > 2 and row["group"] != "directed":
            continue
        arm = row["arm"]
        pdf = WORK / row["directory"] / "out.pdf"
        fresh = PRIOR / "fresh" / "regular" / ("vanilla" if arm == "vanilla" else "tekt") / "startup.pdf"
        assert reuse.runner.describe(pdf) == reuse.runner.describe(fresh), (pdf, fresh)
        if arm != "vanilla":
            assert reuse.qdf_hash(pdf) == reuse.qdf_hash(fresh), (pdf, fresh)
        checked[row["directory"]] = row["pdf_sha256"]
    (WORK / "verified.json").write_text(json.dumps(checked, indent=2) + "\n")


if __name__ == "__main__":
    main()
