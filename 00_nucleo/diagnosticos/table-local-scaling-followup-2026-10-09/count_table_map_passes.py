"""Read-only GDB census of TableElem::map_content by realize_document return PC."""

import datetime
import gzip
import hashlib
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
B = R / "target/release/typst"
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
CASES = ("local-set-100", "no-fill-100", "local-set-1600", "no-fill-1600")
PIN = "702060185fdc0533de86e7bf00c107e26fd0d281170ccd9c76d12b91e4e46883"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def main():
    assert sha(B) == PIN
    rows = subprocess.check_output(["nm", str(B)], text=True).splitlines()
    symbols = {int(a, 16): name for row in rows if
               (m := re.match(r"([0-9a-f]+) [tT] (\S+)", row))
               for a, name in [m.groups()]
               if "TableElem$u20$as$u20$" in name and "map_content17h" in name}
    addresses = sorted(symbols)
    assert len(addresses) > 20, len(addresses)
    locate = [m.group(1) for row in rows if
              (m := re.match(r"[0-9a-f]+ [tT] (\S*realize6locate17h\S+)", row))]
    assert len(locate) == 1, locate
    py = '''import gdb, json
from collections import Counter
counts = Counter()
locate_counts = Counter()
class Probe(gdb.Breakpoint):
    def stop(self):
        frame = gdb.newest_frame()
        caller = frame.older()
        while caller is not None:
            name = caller.name() or ""
            if "realize::realize_document" in name:
                counts[hex(caller.pc())] += 1
                break
            caller = caller.older()
        else:
            counts["outside_realize_document"] += 1
        return False
class LocateProbe(gdb.Breakpoint):
    def stop(self):
        frame = gdb.newest_frame().older()
        locate_counts[(frame.name() or "unknown") if frame is not None else "unknown"] += 1
        return False
'''
    py += "\n".join(f"Probe(\"'{symbols[address]}'\")" for address in addresses)
    py += f"\nLocateProbe(\"'{locate[0]}'\")"
    commands = "set pagination off\nset confirm off\npython\n" + py + "\nend\nrun\npython\nprint('TABLE_MAP_PASSES=' + json.dumps(dict(counts), sort_keys=True))\nprint('LOCATE_CALLERS=' + json.dumps(dict(locate_counts), sort_keys=True))\nend\n"
    env = os.environ.copy()
    for key in list(env):
        if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
            env.pop(key)
    env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
    state = {
        "utc_start": utc(), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
        "working_tree": "uncommitted", "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
        "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
        "binary_sha256": sha(B), "driver_sha256": sha(__file__), "breakpoint_addresses": [hex(a) for a in addresses],
        "locate_symbol": locate[0],
        "gdb_commands": commands, "samples": [], "scope": "GDB call counts and pinned return PCs, not runtime or exclusive allocation.",
    }
    with tempfile.TemporaryDirectory(prefix="table-map-passes-") as temp:
        out = Path(temp)
        gdb = out / "probe.gdb"
        gdb.write_text(commands)
        for name in CASES:
            fixture = F / f"{name}.typ"
            pdf = out / f"{name}.pdf"
            cmd = ["gdb", "--batch", "-x", str(gdb), "--args", str(B), "compile", str(fixture), str(pdf),
                   "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            result = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=240)
            found = re.search(r"TABLE_MAP_PASSES=(\{[^\n]+\})", result.stdout)
            locate_found = re.search(r"LOCATE_CALLERS=(\{[^\n]+\})", result.stdout)
            counts = json.loads(found.group(1)) if found else None
            locate_counts = json.loads(locate_found.group(1)) if locate_found else None
            state["samples"].append({"case": name, "utc": utc(), "fixture_sha256": sha(fixture), "command": cmd,
                                      "exit_code": result.returncode, "counts": counts, "locate_callers": locate_counts, "stdout": result.stdout,
                                      "stderr": result.stderr, "pdf_sha256": sha(pdf) if pdf.exists() else None})
            print(name, counts, locate_counts, "exit", result.returncode, flush=True)
            if result.returncode or counts is None or locate_counts is None:
                break
    state["utc_end"] = utc()
    with gzip.open(D / "table-map-passes-before.json.gz", "wt") as output:
        json.dump(state, output, ensure_ascii=False)
    assert len(state["samples"]) == len(CASES)
    assert all(row["exit_code"] == 0 and row["counts"] is not None and row["locate_callers"] is not None for row in state["samples"])
    assert sha(B) == PIN


if __name__ == "__main__":
    main()
