"""Read-only GDB census of show-rule slice lengths at realize_cell's call site."""

import datetime
import gzip
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
B = R / "target/release/typst"
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
CASES = ("local-set-100", "no-fill-100", "local-set-1600", "no-fill-1600",
         "cell-show-set-match", "cell-show-set-no-match")


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def main():
    nm = subprocess.check_output(["nm", str(B)], text=True)
    match = re.search(r"^([0-9a-f]+) [tT] (\S*apply_show_rules_inner17h\S+)$", nm, re.M)
    assert match
    symbol = match.group(2)
    script = f'''import gdb
from collections import Counter
counts = Counter()
class Probe(gdb.Breakpoint):
    def stop(self):
        frame = gdb.newest_frame()
        caller = frame.older()
        if caller is not None and caller.name() and "table_cells::realize_cell" in caller.name():
            n = int(gdb.parse_and_eval("$rcx"))
            counts[n] += 1
        return False
Probe("'{symbol}'")
'''
    gdb_commands = "set pagination off\nset confirm off\npython\n" + script + "end\nrun\npython\nprint('RULE_LENGTHS=' + repr(sorted(counts.items())))\nend\n"
    env = os.environ.copy()
    for key in list(env):
        if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
            env.pop(key)
    env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
    state = {
        "utc_start": utc(), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
        "working_tree": "uncommitted", "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
        "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
        "binary_sha256": sha(B), "driver_sha256": sha(__file__),
        "gdb_commands": gdb_commands, "samples": [],
        "scope": "GDB call census; debugger wall time is not a performance measurement.",
    }
    with tempfile.TemporaryDirectory(prefix="cell-rule-lengths-") as temp:
        out = Path(temp)
        commands = out / "probe.gdb"
        commands.write_text(gdb_commands)
        for name in CASES:
            fixture = F / f"{name}.typ"
            pdf = out / f"{name}.pdf"
            cmd = ["gdb", "--batch", "-x", str(commands), "--args", str(B), "compile", str(fixture), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            result = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=240)
            found = re.search(r"RULE_LENGTHS=(\[[^\n]+\])", result.stdout)
            row = {"case": name, "utc": utc(), "fixture_sha256": sha(fixture), "command": cmd,
                   "exit_code": result.returncode, "rule_lengths": found.group(1) if found else None,
                   "stdout": result.stdout, "stderr": result.stderr,
                   "pdf_sha256": sha(pdf) if pdf.exists() else None}
            state["samples"].append(row)
            print(name, row["rule_lengths"], "exit", result.returncode, flush=True)
            if result.returncode or found is None:
                break
    state["utc_end"] = utc()
    with gzip.open(D / "cell-rule-lengths-before.json.gz", "wt") as output:
        json.dump(state, output, ensure_ascii=False)
    assert len(state["samples"]) == len(CASES)
    assert all(row["exit_code"] == 0 and row["rule_lengths"] is not None for row in state["samples"])
    assert sha(B) == state["binary_sha256"]


if __name__ == "__main__":
    main()
