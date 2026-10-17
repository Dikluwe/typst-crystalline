"""Count release layout calls under GDB; counts are not timing samples."""

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
label = sys.argv[1]
assert label in ("red", "green")
R = D.parents[2]
B = R / "target/release/typst"
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
CASES = ("local-set-100", "no-fill-100", "local-set-1600", "no-fill-1600")
SYMBOLS = {
    "defaults": "table_cells13with_defaults",
    "realize_cell": "table_cells12realize_cell",
    "native_cell": "native_table_cell",
}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def breakpoints():
    rows = subprocess.check_output(["nm", str(B)], text=True).splitlines()
    grouped = {key: {} for key in SYMBOLS}
    for row in rows:
        parts = row.split()
        if len(parts) != 3 or parts[1] not in ("t", "T"):
            continue
        address, _, name = parts
        for key, needle in SYMBOLS.items():
            if needle in name:
                grouped[key].setdefault(address, name)
    assert all(grouped.values()), {key: len(value) for key, value in grouped.items()}
    return grouped


def script_for(grouped):
    lines = ["set pagination off", "set confirm off", "set breakpoint pending off"]
    for key in grouped:
        lines.append(f"set ${key} = 0")
    for key, names in grouped.items():
        for name in names.values():
            lines.extend((f"break '{name}'", "commands", "silent", f"set ${key} = ${key} + 1", "continue", "end"))
    lines.append("run")
    for key in grouped:
        lines.append(f'printf "COUNT_{key}=%d\\n", ${key}')
    return "\n".join(lines) + "\n"


def main():
    grouped = breakpoints()
    script = script_for(grouped)
    env = os.environ.copy()
    for key in list(env):
        if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
            env.pop(key)
    env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
    fixtures = {name: {"text": (F / f"{name}.typ").read_text(), "sha256": sha(F / f"{name}.typ")} for name in CASES}
    state = {
        "utc_start": utc(),
        "label": label,
        "threshold_local_defaults": 3200,
        "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
        "working_tree": "uncommitted",
        "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
        "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
        "binary_sha256": sha(B), "driver_sha256": sha(D / "count_default_calls_prepared.py"),
        "fixtures": fixtures, "gdb_script": script,
        "symbol_names": {key: list(names.values()) for key, names in grouped.items()},
        "samples": [], "scope": "GDB breakpoint call counts only; debugger wall time is not a performance metric.",
    }
    with tempfile.TemporaryDirectory(prefix="table-layout-count-") as temp:
        out = Path(temp)
        commands = out / "counts.gdb"
        commands.write_text(script)
        for name in CASES:
            fixture = F / f"{name}.typ"
            pdf = out / f"{name}.pdf"
            cmd = ["gdb", "--batch", "-x", str(commands), "--args", str(B), "compile", str(fixture), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            result = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=240)
            counts = {key: int(value) for key, value in re.findall(r"COUNT_(\w+)=(\d+)", result.stdout)}
            row = {"case": name, "utc": utc(), "command": cmd, "exit_code": result.returncode,
                   "counts": counts, "stdout": result.stdout, "stderr": result.stderr,
                   "pdf_sha256": sha(pdf) if pdf.exists() else None}
            state["samples"].append(row)
            print(name, counts, "exit", result.returncode, flush=True)
            if result.returncode != 0 or set(counts) != set(SYMBOLS):
                break
    state["utc_end"] = utc()
    with gzip.open(D / f"default-call-counts-prepared-{label}.json.gz", "wt") as out:
        json.dump(state, out, ensure_ascii=False)
    assert len(state["samples"]) == len(CASES)
    assert all(row["exit_code"] == 0 and set(row["counts"]) == set(SYMBOLS) for row in state["samples"])
    assert sha(B) == state["binary_sha256"]
    assert all(sha(F / f"{name}.typ") == item["sha256"] for name, item in fixtures.items())
    local = next(row for row in state["samples"] if row["case"] == "local-set-1600")
    if local["counts"]["defaults"] > state["threshold_local_defaults"]:
        sys.exit(1)


if __name__ == "__main__":
    main()
