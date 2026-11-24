"""Compare deferred eviction and LRU under two budgets on the same lab ELF."""
import json
import os
import re
from random import Random

from watch import EXES, STATES, WORK, run

ARMS = ("lru8", "protect8", "lru2_5", "protect2_5", "vanilla")
BUDGET = {"8":8 * 1024 * 1024, "2_5":5 * 1024 * 1024 // 2}
PATTERN = re.compile(r"^lab-eval-memo LabReport \{ (.*) \}$", re.M)


def options(arm, trace=False):
    settings = {}
    if arm != "vanilla":
        policy = ("protect" if arm.startswith("protect") else "lru")
        settings["TEKT_LAB_MEMO_BYTES"] = str(BUDGET[arm.removeprefix(policy)])
        if policy == "protect":
            settings["TEKT_LAB_MEMO_PROTECT"] = "1"
        if trace:
            settings["TEKT_LAB_MEMO_TRACE"] = "1"
    return settings


def cycle(block, arm, period=0, trace=False, states=STATES, check=True, directory="cases"):
    engine = "vanilla" if arm == "vanilla" else "candidate"
    arm_options = options(arm, trace)
    return run(block, engine, period, overrides=arm_options, states=states,
               check=check, directory=directory)


def traces(row):
    log = (WORK / row["directory"] / "watch.log").read_text()
    reports = {}
    for state, part in zip(row["all_pdfs"], log.split("compiled successfully")[:-1]):
        entries = PATTERN.findall(part)
        assert len(entries) <= 1, (state, entries)
        reports[state] = (dict((name, int(value)) for name, value in
                              (field.split(": ") for field in entries[0].split(", ")))
                          if entries else None)
    return reports


def directed():
    edits = {}
    for index, arm in enumerate(ARMS[:-1]):
        row = cycle(index, arm, trace=True, directory="directed")
        report = traces(row)
        edits[arm] = report
        assert report["startup"] and report["visible"]
        assert report["comment"] is None and report["comment-2"] is None
        budget = int(options(arm)["TEKT_LAB_MEMO_BYTES"])
        assert all(r["peak"] <= budget and r["charged"] <= r["peak"]
                   for r in report.values() if r is not None)
        print(arm, report["visible"], flush=True)
    for arm in ("lru8", "protect8"):
        r = edits[arm]["visible"]
        assert (r["hits"], r["misses"], r["evicted_during"], r["evicted_after"]) == (19,1,0,0), r
    r = edits["lru2_5"]["visible"]
    assert (r["hits"], r["misses"], r["evicted_during"], r["evicted_after"]) == (0,20,20,0), r
    r = edits["protect2_5"]["visible"]
    assert (r["hits"], r["misses"], r["evicted_during"], r["evicted_after"]) == (18,2,0,1), r
    assert (r["deferred"], r["stores"]) == (2,0), r

    source = STATES[-1][1]
    follow_up = source.replace(b"[Paragraph", b"[Second paragraph", 1)
    assert follow_up != source
    history = [*STATES, ("edit-second-loop", follow_up, None), ("restore-first-edit", source, None)]
    repeated, verification = {}, {}
    for i, arm in enumerate(("lru2_5","protect2_5")):
        row = cycle(i, arm, trace=True, states=history, check=False, directory="repeated")
        repeated[arm], verification[arm] = traces(row), row["all_pdfs"]
        assert all(r["peak"] <= BUDGET["2_5"] for r in repeated[arm].values() if r)
    assert verification["lru2_5"] == verification["protect2_5"]
    assert repeated["protect2_5"]["edit-second-loop"]["stores"] > 0
    (WORK/"directed.json").write_text(json.dumps(dict(original=edits,repeated=repeated,
                                                    repeated_pdfs=verification),indent=2)+"\n")


def main():
    os.sched_setaffinity(0, {2})
    EXES["candidate"] = WORK/"candidate"
    directed()
    rng = Random(50134)
    periods = [0, 577000, 937000]
    rng.shuffle(periods)
    with (WORK/"measurements.jsonl").open("w") as output:
        for group, period in enumerate(periods):
            labels = rng.sample(ARMS, len(ARMS))
            orders = [[(i + step) % 5 for step in (0, 1, 4, 2, 3)] for i in range(5)]
            orders += [list(reversed(order)) for order in orders]
            for subblock, order in enumerate(rng.sample(orders, len(orders))):
                block = 10*group + subblock
                for i, index in enumerate(order):
                    arm = labels[index]
                    row = cycle(5*block+i, arm, period)
                    print(json.dumps(row|dict(block=block,arm=arm)),file=output,flush=True)
                print(f"{block+1}/30 {period}",flush=True)


if __name__ == "__main__":
    main()
