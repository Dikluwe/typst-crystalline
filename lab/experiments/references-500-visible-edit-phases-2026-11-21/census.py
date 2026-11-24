"""Trace causal gates with exact-ELF breakpoints, independently of the CPU campaign."""
import json
from pathlib import Path
import re
import subprocess

from sample import D, WORK, cycle, STATES, EXES

MATCH = {
    "watch_wait": "::watch::wait_for_change_since",
    "memo": "::physical_memo::layout_with",
    "root": "::layout_planned_root",
    "journal": "::physical_memo::Journal::validates",
    "page_inputs": "::physical_memo::Journal::page_inputs_match",
    "pagination": "::context_stabilization::pagination_stability",
    "outline": "::prepare_outline_candidate",
}


def addresses(exe):
    rows = subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", exe], text=True)
    symbols, disassembly = {}, {}
    for line in rows.splitlines():
        parts = line.split(maxsplit=3)
        if len(parts) != 4:
            continue
        addr, size, _, name = parts
        for key, suffix in MATCH.items():
            if name.split(".llvm.", 1)[0].endswith(suffix):
                symbols.setdefault(key, []).append(int(addr, 16))
                if key in ("memo", "journal", "pagination"):
                    disassembly.setdefault(key, []).append(subprocess.check_output([
                        "objdump", "-d", "--no-show-raw-insn", f"--start-address=0x{addr}",
                        f"--stop-address={int(addr,16)+int(size,16)}", exe], text=True))
    assert set(MATCH) - {"outline"} <= symbols.keys(), symbols
    plan_tests = []
    for body in disassembly["memo"]:
        tests = re.findall(r"^\s*([a-f0-9]+):\s+cmp\s+0x110\(%(\w+)\),%(\w+)", body, re.M)
        assert len(tests) == 1, (exe, tests)
        plan_tests.extend((int(pc, 16), lhs, rhs) for pc, lhs, rhs in tests)
    journal = disassembly["journal"][0]
    # The predicate returns AL to the same conditional after either closure type.
    matches = re.findall(r"^\s*([a-f0-9]+):\s+test\s+%al,%al", journal, re.M)
    assert len(matches) == 1, (exe, matches)
    pagination = disassembly["pagination"][0]
    assert "0x2b0(%r15)" in pagination and "0x720(%r15)" in pagination, exe
    return dict(symbols=symbols, plan_tests=plan_tests,
                journal_checks=[int(m, 16) for m in matches], disassembly=disassembly)


def run(arm, golden, tail_control=False):
    case = WORK / "census" / (arm + "-tail-control" if tail_control else arm)
    case.mkdir(parents=True)
    (case / "edits").mkdir()
    (case / "main.typ").write_bytes(STATES[0][1])
    for step, (_, source, _) in enumerate(STATES[1:], 1):
        if tail_control and step == 3:
            source = STATES[0][1].replace(b"[Paragraph", b"[Visible paragraph", 1)
            assert source != STATES[0][1] and source != STATES[3][1]
        (case / "edits" / f"{step}.typ").write_bytes(source)
    (case / "addresses.json").write_text(json.dumps(addresses(EXES[arm]), indent=2))
    command = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch", "-x", str(D / "census.gdb"),
               "--args", str(EXES[arm]), "watch", "main.typ", "out.pdf", "--root", str(case), "--pdf-tagged=true"]
    with (case / "gdb.log").open("wb") as log:
        subprocess.run(command, cwd=case, env=cycle.environment(case, arm, trace=True),
                       stdout=log, stderr=subprocess.STDOUT, timeout=120, check=True)
    for step, (state, _, _) in enumerate(STATES):
        assert cycle.key(arm, case / f"publication-{step}.pdf") == golden[arm][state]
    events = json.loads((case / "events.json").read_text())
    visible = [event for event in events if event["publication"] == 3]
    counts = {kind: sum(row["kind"] == kind for row in visible) for kind in ("root", "memo", "pagination")}
    if not tail_control:
        assert counts["root"] == 4 and counts["memo"] == 8, (arm, counts)
    print(case.name, counts, flush=True)


if __name__ == "__main__":
    import os
    gold = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]
    for label in ("hybrid", "tekt"):
        run(label, gold)
    run("hybrid", gold, tail_control=True)
