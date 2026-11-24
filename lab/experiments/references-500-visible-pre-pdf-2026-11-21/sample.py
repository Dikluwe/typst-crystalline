"""Arm the CPU sampler only for the fourth publication of an original watch session."""
import importlib.util
import json
import os
from pathlib import Path
from random import Random
import signal
import struct
import time

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
prior = D.parent / "references-500-visible-page-operators-2026-11-21" / "bench.py"
spec = importlib.util.spec_from_file_location("watch_cycle", prior)
cycle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cycle)
driver, measure = cycle.cycle.driver, cycle.cycle.measure
STATES, ARMS, EXES = cycle.STATES, cycle.ARMS, cycle.EXES

RECEIPT = D.parents[1] / "results/construction/diagnostics/references-500-visible-stable-cids-2026-11-21/directed.json"
stable = {row["state"]:row["stable_qdf"] for row in json.loads(RECEIPT.read_text())["original"]}


def golden(arm, name):
    return stable[name] if arm == "candidate" else cycle.gold(arm, name)


def toggle(child, case, epoch):
    os.kill(child.process.pid, signal.SIGUSR2)
    ack = case / "samples.ack"
    until = time.monotonic() + 5
    while ack.read_bytes() != struct.pack("<i", epoch):
        if time.monotonic() > until or child.process.poll() is not None:
            raise RuntimeError((case, epoch, child.process.poll()))
        time.sleep(0.001)


def run(block, arm, period):
    case = WORK / "cases" / f"{block:02d}-{arm}"
    case.mkdir(parents=True)
    tmp, source, pdf = case / "tmp", case / "main.typ", case / "out.pdf"
    tmp.mkdir()
    cycle.base.runner.setup_sources(case, *STATES[0][1:])
    env = cycle.environment(tmp, arm, trace=False)
    if period:
        env.update(LD_PRELOAD=str(WORK / "profiler.so"), TEKT_CPU_TRACE=str(case / "samples.bin"),
                   TEKT_CPU_ACK=str(case / "samples.ack"), TEKT_CPU_PERIOD=str(period))
    command = [str(EXES[arm]), "watch", str(source), str(pdf), "--root", str(case), "--pdf-tagged=true"]
    with (case / "watch.log").open("wb") as log:
        child = driver.Child(command, case, env, 1, log)
        try:
            publication = None
            for step, (_, source_data, leaf) in enumerate(STATES):
                if step == 3 and period:
                    proc = Path(f"/proc/{child.process.pid}")
                    (case / "maps").write_text((proc / "maps").read_text())
                    toggle(child, case, 1)
                before = child.cpu() if step else 0
                start = time.perf_counter_ns()
                if step:
                    cycle.base.runner.setup_sources(case, source_data, leaf)
                publication, published, _ = driver.await_pdf(child, pdf, publication)
                after = measure.settle(child)
                if step == 3 and period:
                    toggle(child, case, 2)
                if step == 3:
                    observed = cycle.cycle.key(arm, pdf)
                    assert observed == golden(arm, STATES[step][0]), (block, arm, step)
        finally:
            child.stop()
            child.close()
    assert (case / "watch.log").read_text().count("compiled successfully") == 4
    return dict(block=block, arm=arm, period_ns=period, cpu_ns=after-before, wall_ns=published-start,
                utc=measure.utc(), exe=str(EXES[arm]), directory=str(case.relative_to(WORK)), pdf_hash=observed)


def main():
    os.sched_setaffinity(0, {2})
    rng = Random(50044)
    periods = [0] * 8 + [577000] * 12 + [937000] * 12
    rng.shuffle(periods)
    with (WORK / "measurements.jsonl").open("w") as output:
        for block, period in enumerate(periods):
            for arm in rng.sample(ARMS, len(ARMS)):
                row = run(block, arm, period)
                assert row["pdf_hash"] == golden(arm,"visible"), (block, arm)
                print(json.dumps(row), file=output, flush=True)
            print(f"{block+1}/{len(periods)} {period}", flush=True)


if __name__ == "__main__":
    main()
