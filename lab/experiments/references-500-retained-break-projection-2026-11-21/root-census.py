"""Count actual entries into the planned root in the nominal Tekt ELFs."""
from collections import Counter
from functools import lru_cache
import json
from pathlib import Path
import subprocess

from bench import BASE, D, WORK, PATHS, environment, base, digest, grid_states, positions_from_log

SUFFIXES = {
    "root": "::layout_planned_root",
    "memo": "::physical_memo::layout_with",
    "production": "::convergence::run_planned",
    "projection_entry": "::physical_memo::lab_retain",
    "prefix_entry": "::lab_retain::layout_prefix",
    "watch_wait": "::watch::wait_for_change_since",
}


@lru_cache(maxsize=None)
def symbols(exe):
    result = {}
    nm = subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", str(exe)], text=True)
    entries = [line.split(" ", 3) for line in nm.splitlines()]
    for kind, suffix in SUFFIXES.items():
        addresses = {int(fields[0], 16) for fields in entries if len(fields) == 4
                     and fields[3].split(".llvm.", 1)[0].endswith(suffix)}
        if kind not in ("production", "projection_entry", "prefix_entry"):
            assert addresses, (exe, suffix)
        result[kind] = sorted(addresses)
    return result


def census(case, arm, source, expected_qdf):
    case.mkdir(parents=True)
    (case / "main.typ").write_bytes(source)
    exe = PATHS[arm]
    offsets = symbols(exe)
    (case / "symbols.json").write_text(json.dumps(offsets) + "\n")
    env = environment(arm, case)
    env.update(TEKT_LAB_WATCH_RETAIN="1", TEKT_LAB_WATCH_LAZY="1",
               TEKT_LAB_WATCH_INERT="1", TEKT_LAB_PDF_REUSE="both", TEKT_LAB_WATCH_TRACE="1")
    cmd = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch",
           "-x", str(D / "root-census.gdb"), "--args", str(exe),
           "watch", "main.typ", "out.pdf", "--root", str(case), "--pdf-tagged=true"]
    with (case / "gdb.log").open("wb") as output:
        subprocess.run(cmd, cwd=case, env=env, stdout=output, stderr=subprocess.STDOUT,
                       timeout=90, check=True)
    assert base.qdf_hash(case / "out.pdf") == expected_qdf
    counts = json.loads((case / "counts.json").read_text())
    counts.setdefault("projection_root", 0)
    counts.setdefault("prefix_entry", 0)
    counts["ordinary_root"] = counts["root"] - counts["projection_root"]
    return dict(counts, sha256=digest(exe), offsets=offsets, command=cmd)


def main():
    expected = D.parent / "references-500-pdf-export-reuse-2026-11-20/fresh/regular/tekt/startup.pdf"
    qdf = base.qdf_hash(expected)
    original = {arm:census(WORK / arm, arm, BASE, qdf) for arm in ("both", "omit", "same-off")}
    (WORK / "root-census.json").write_text(json.dumps(original, indent=2) + "\n")
    print("original", {a:d["root"] for a,d in original.items()}, flush=True)

    cases = json.loads((WORK / "grid-input.json").read_text())
    result = {}
    for name, data in sorted(cases.items()):
        [(label, source, _)] = grid_states(data["height"], data["count"])
        case = WORK / "grid" / name
        arms = ("both", "omit")
        counts = {arm:census(case / arm, arm, source, data["tekt_qdf_sha256"])
                  for arm in arms}
        control, native = (positions_from_log(case / arm / "gdb.log") for arm in arms)
        result[name] = dict(counts, positions_compared=len(native),
                            same_coordinates=Counter(control.values()) == Counter(native.values()),
                            exact_positions=control == native)
        print(name, {arm:counts[arm]["root"] for arm in arms}, flush=True)
    (WORK / "root-grid.json").write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
