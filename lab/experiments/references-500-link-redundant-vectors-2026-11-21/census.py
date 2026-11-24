"""Collect per-reduction vector and item visits plus capacity growth."""
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import tarfile

from bench import ARMS, D, OUT, STATES, WORK, base

FIELDS = ("vectors", "items", "removed_links", "retained_allocations",
          "allocations", "reallocations")
EVENT = struct.Struct("<14Q")
MAGIC = (0xb27ce64254ba881f, 0x2710a95ebc07f361)


def measure(i, preload):
    case = WORK / "vector-census" / str(i)
    case.mkdir(parents=True)
    trace = case / "events.bin"
    source, pdf = case / "main.typ", case / "out.pdf"
    base.cycle.base.runner.setup_sources(case, *STATES[0][1:])
    env = base.environment(case, "candidate", False)
    env.update(LD_PRELOAD=str(preload), TEKT_LINK_VECTOR_CENSUS=str(trace))
    command = [str(WORK / "census"), "watch", str(source), str(pdf),
               "--root", str(case), "--pdf-tagged=true"]
    offsets = [0]
    with (case / "watch.log").open("wb") as log:
        child = base.driver.Child(command, case, env, 1, log)
        try:
            publication = None
            for j, (state, data, leaf) in enumerate(STATES):
                if j:
                    base.cycle.base.runner.setup_sources(case, data, leaf)
                publication, _, _ = base.driver.await_pdf(child, pdf, publication)
                base.measure.settle(child)
                expected = base.golden("candidate", state)
                assert base.cycle.cycle.key("candidate", pdf) == expected, (i, state)
                offsets.append(trace.stat().st_size)
        finally:
            child.stop()
            child.close()
    assert (case / "watch.log").read_text().count("compiled successfully") == 4, case
    raw = trace.read_bytes()
    events = list(EVENT.iter_unpack(raw[offsets[-2]:offsets[-1]]))
    assert events and all(values[:2] == MAGIC for values in events), (i, offsets)
    assert all(values[2:5] == values[8:11] for values in events)
    totals = {}
    for j, arm in enumerate(ARMS[:2]):
        totals[arm] = {field:sum(values[2 + 6*j + k] for values in events)
                       for k,field in enumerate(FIELDS)}
    assert totals["control"]["retained_allocations"] == 0
    assert totals["candidate"]["allocations"] < totals["control"]["allocations"]
    evidence = dict(wrap_calls=len(events), totals=totals, offsets=offsets)
    (case / "summary.json").write_text(json.dumps(evidence, indent=2) + "\n")
    return evidence


def main():
    os.sched_setaffinity(0, {2})
    preload = WORK / "vector-census.so"
    subprocess.run(["cc", "-O2", "-Wall", "-Wextra", "-Werror", "-fPIC", "-shared", "-fno-builtin",
                    "-o", preload, D/"census-preload.c", "-ldl"], check=True)
    with preload.open("rb") as binary:
        (OUT/"census-preload.sha256").write_text(
            hashlib.file_digest(binary, "sha256").hexdigest() + "  vector-census.so\n")
    observations = []
    for i in range(3):
        observations.append(measure(i, preload))
        print("census", i + 1, observations[-1], flush=True)
    assert observations[0] == observations[1] == observations[2], observations
    (WORK / "census.json").write_text(json.dumps(observations, indent=2) + "\n")
    with tarfile.open(OUT/"census-cases.tar.gz", "w:gz") as archive:
        for path in sorted((WORK/"vector-census").rglob("*")):
            if path.name in ("events.bin","summary.json","watch.log"):
                archive.add(path, arcname=str(path.relative_to(WORK)))


if __name__ == "__main__":
    main()
