"""Count realized-cell entries and Complete exits in the pinned release via GDB."""
import datetime, gzip, hashlib, json, os, re, subprocess, tempfile
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
B = R / "target/release/typst"
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
CASES = ("local-set-100", "no-fill-100", "local-set-1600", "no-fill-1600")
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()

nm = subprocess.check_output(["nm", str(B)], text=True)
symbol = next(name for _, name in re.findall(r"^([0-9a-f]+) [tT] (\S*table_cells12realize_cell\S*)$", nm, re.M))
dis = subprocess.check_output(["objdump", "-d", "--start-address=0x26541d0", "--stop-address=0x26542a5", str(B)], text=True)
assert sha(B) == "702060185fdc0533de86e7bf00c107e26fd0d281170ccd9c76d12b91e4e46883"
assert "2654220" in dis and "265422e" in dis and "2654294" in dis
script = "\n".join([
    "set pagination off", "set confirm off", "set breakpoint pending off",
    "set $total = 0", "set $complete = 0",
    f"break '{symbol}'", "commands", "silent", "set $total = $total + 1", "continue", "end",
    f"break *'{symbol}'+0x50", "commands", "silent", "set $complete = $complete + 1", "continue", "end",
    "run", 'printf "COUNT_total=%d\\n", $total', 'printf "COUNT_complete=%d\\n", $complete',
]) + "\n"
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
state = {
    "utc_start": utc(), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted", "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binary_sha256": sha(B), "driver_sha256": sha(__file__), "symbol": symbol, "branch_disassembly": dis, "gdb_script": script,
    "fixtures": {name: {"text": (F / f"{name}.typ").read_text(), "sha256": sha(F / f"{name}.typ")} for name in CASES},
    "samples": [], "scope": "Breakpoints count calls, not time. Complete branch offset verified only for pinned release bytes.",
}
with tempfile.TemporaryDirectory(prefix="table-walk-stage-") as tmp:
    out = Path(tmp)
    commands = out / "counts.gdb"
    commands.write_text(script)
    for name in CASES:
        fixture = F / f"{name}.typ"
        pdf = out / f"{name}.pdf"
        cmd = ["gdb", "--batch", "-x", str(commands), "--args", str(B), "compile", str(fixture), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
        proc = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=240)
        counts = {key: int(value) for key, value in re.findall(r"COUNT_(\w+)=(\d+)", proc.stdout)}
        state["samples"].append({"case": name, "utc": utc(), "command": cmd, "exit_code": proc.returncode,
                                 "counts": counts, "stdout": proc.stdout, "stderr": proc.stderr,
                                 "pdf_sha256": sha(pdf) if pdf.exists() else None})
        print(name, counts, proc.returncode, flush=True)
        assert proc.returncode == 0 and set(counts) == {"total", "complete"}, proc.stderr
state["utc_end"] = utc()
assert sha(B) == state["binary_sha256"] and sha(__file__) == state["driver_sha256"]
assert all(sha(F / f"{name}.typ") == entry["sha256"] for name, entry in state["fixtures"].items())
with gzip.open(D / "walk-cell-stage-before.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
