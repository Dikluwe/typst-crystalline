"""Census the exact nominal ELFs; force the four inputs at the watch boundary."""
from collections import Counter
import json
import subprocess
import sys

from bench import D, WORK, EXES, REGULAR, command, describe, measure, write_json

SUFFIXES = {
    "memo": "::physical_memo::layout_with",
    "copy_leaf": "::physical_memo::clone_document",
    "document_clone": "<typst_core::entities::layout_types::PagedDocument as core::clone::Clone>::clone",
    "transfer": "::physical_memo::finish_replay",
    "produce": "::convergence::run_planned",
    "root": "::layout_planned_root",
    "wait": "::watch::wait_for_change_since",
    "plan": "::introspect::page_running::prepare_page_running_body",
    "page-input": "::physical_memo::Journal::page_inputs_match",
    "dependencies": "::physical_memo::Journal::validates",
}


def census(arm, receipt):
    exe = EXES[arm]
    digest = measure.sha(exe)
    assert receipt["elves"][arm] == digest
    symbols = {name: [] for name in SUFFIXES}
    for line in subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", str(exe)],
                                        text=True).splitlines():
        parts = line.split(" ", 3)
        if len(parts) != 4:
            continue
        for name, suffix in SUFFIXES.items():
            if parts[-1].split(".llvm.", 1)[0].endswith(suffix):
                symbols[name].append(int(parts[0], 16))
    assert all(symbols[key] for key in ("memo", "produce", "root", "wait", "transfer")), symbols
    case = WORK / "census" / arm
    case.mkdir(parents=True)
    for name, source, _ in REGULAR:
        (case / (name + ".typ")).write_bytes(source)
    (case / "main.typ").write_bytes(REGULAR[0][1])
    write_json(case / "symbols.json", symbols)
    args, env = command(case, arm, "watch", case / "out.pdf", arm != "control")
    cmd = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch", "-x", str(D / "census.gdb"), "--args", *args]
    with (case / "gdb.log").open("wb") as log:
        subprocess.run(cmd, cwd=case, env=env, stdout=log, stderr=subprocess.STDOUT,
                       timeout=90, check=True)
    rows = json.loads((case / "observed.json").read_text())
    assert len(rows) == len(REGULAR), rows
    for row, (name, _, _) in zip(rows, REGULAR):
        counts = Counter(row["events"])
        comparison = D / "runs/validation/regular/control" / (name + "-fresh.pdf")
        assert describe(case / f'{row["cycle"]}.pdf') == describe(comparison)
        row.update(state=name, counts=dict(counts), equivalent=True)
        hit = arm == "retain" and name in ("comment", "comment-2")
        assert counts["root"] == counts["produce"], (arm, name, counts)
        if hit:
            assert counts["root"] == counts["memo"] == counts["transfer"] == 0, (arm, name, counts)
        else:
            full_compilations = 2 if arm == "retain" and name == "startup" else 1
            assert counts["transfer"] == full_compilations, (arm, name, counts)
            if name != "visible":
                assert counts["root"] == 4 * full_compilations and counts["memo"] == 8 * full_compilations, (arm, name, counts)
            elif arm != "control":
                control_rows = json.loads((D / "census-control.json").read_text())["rows"]
                control_counts = next(row["counts"] for row in control_rows if row["state"] == name)
                assert all(counts[key] == control_counts.get(key, 0)
                           for key in ("memo", "produce", "root")), (arm, name, counts)
        assert counts["copy_leaf"] == counts["defensive_copy"] == 0, (arm, name, counts)
    write_json(D / f"census-{arm}.json", dict(utc=measure.utc(), elf=digest,
               command=cmd, symbols=symbols, rows=rows))
    print(arm, {row["state"]: row["counts"] for row in rows}, flush=True)


if __name__ == "__main__":
    receipt = json.loads((D / "receipt.json").read_text())
    for arm in sys.argv[1:] or EXES:
        census(arm, receipt)
