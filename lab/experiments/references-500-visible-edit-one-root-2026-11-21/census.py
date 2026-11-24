"""Count physical root entries in the exact ELFs timed by the watch benchmark."""
import json
import subprocess

from bench import D, WORK, STATES, EXES, cycle, environment, gold, positions

SUFFIXES = {
    "root": "::layout_planned_root",
    "native_finish": "::physical_memo::lab_finish_native",
    "checkpoint": "::lab_retain::layout_from_checkpoint",
    "memo": "::physical_memo::layout_with",
    "watch_wait": "::watch::wait_for_change_since",
}


def run(arm):
    case = WORK / "census" / arm
    case.mkdir(parents=True)
    (case / "edits").mkdir()
    for number, (_, source, _) in enumerate(STATES):
        path = case / ("main.typ" if not number else f"edits/{number}.typ")
        path.write_bytes(source)
    exe = EXES[arm]
    nm = subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", str(exe)], text=True)
    entries = [line.split(" ", 3) for line in nm.splitlines()]
    symbols = {kind:sorted({int(values[0], 16) for values in entries
                if len(values) == 4 and values[3].split(".llvm.", 1)[0].endswith(suffix)})
               for kind, suffix in SUFFIXES.items()}
    assert all(symbols.values()), (arm, symbols)
    (case / "symbols.json").write_text(json.dumps(symbols))
    log_path = case / "gdb.log"
    command = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch", "-x", str(D / "census.gdb"),
               "--args", str(exe), "watch", "main.typ", "out.pdf", "--root", str(case), "--pdf-tagged=true"]
    with log_path.open("wb") as log:
        subprocess.run(command, cwd=case, env=environment(case, arm, True),
                       stdout=log, stderr=subprocess.STDOUT, timeout=90, check=True)
    for number, (state, _, _) in enumerate(STATES):
        assert cycle.key(arm, case / f"publication-{number}.pdf") == gold(arm, state), (arm,state)
    measured = json.loads((case / "counts.json").read_text())
    counts = {state:{kind:data.get(kind, 0) for kind in SUFFIXES}
              for (state, _, _), data in zip(STATES, measured, strict=True)}
    return dict(binary_sha256=cycle.measure.sha(exe), publications=counts,
                positions=positions(log_path), symbols=symbols)


if __name__ == "__main__":
    rows = {arm:run(arm) for arm in ("candidate", "hybrid")}
    assert rows["candidate"]["positions"] == rows["hybrid"]["positions"]
    for arm, values in rows.items():
        pubs = values["publications"]
        assert [data["root"] for data in pubs.values()] == [1, 0, 0, 1 if arm == "candidate" else 4], rows
        assert pubs["visible"]["native_finish"] == (1 if arm == "candidate" else 0), rows
        assert pubs["visible"]["checkpoint"] == 0, rows
        print(arm, pubs)
    cycle.dump("census.json", rows)
