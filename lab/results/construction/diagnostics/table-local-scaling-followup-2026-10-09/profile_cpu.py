"""Diagnostic-only CPU-time stack sampling of equivalent table variants."""
import datetime
import gzip
import hashlib
import json
import os
import subprocess
import tempfile
from collections import Counter
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
B = R / "target/release/typst"
SO = Path("/tmp/tekt-cpu-probe.so")
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


build = ["gcc", "-shared", "-fPIC", "-O2", "-std=c11", "-Wall", "-Wextra", "-o", str(SO), str(D / "cpu_probe.c"), "-ldl"]
subprocess.run(build, check=True, cwd=R)
fixtures = [F / f"{v}-1600.typ" for v in ("block-no-set", "no-fill", "local-set")]
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C", LD_PRELOAD=str(SO))
state = {
    "utc_start": utc(),
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted",
    "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binary_sha256": sha(B), "source_sha256": sha(D / "cpu_probe.c"),
    "interposer_sha256": sha(SO), "driver_sha256": sha(__file__),
    "build": build, "cpu": CPU,
    "environment": {k: env[k] for k in ("RAYON_NUM_THREADS", "SOURCE_DATE_EPOCH", "NO_COLOR", "TZ", "LC_ALL")},
    "fixtures": {p.name: {"text": p.read_text(), "sha256": sha(p)} for p in fixtures},
    "runs": [],
    "scope": "1ms ITIMER_PROF samples; backtrace in signal handler is diagnostic and perturbs runtime. Stack presence is inclusive, not exclusive CPU ownership.",
}

with tempfile.TemporaryDirectory(prefix="table-cpu-") as tmp:
    out = Path(tmp)
    for rep in range(12):
        for f in (fixtures if rep % 2 == 0 else list(reversed(fixtures))):
            profile = out / f"{f.stem}-{rep}.txt"
            pdf = out / f"{f.stem}-{rep}.pdf"
            run_env = env.copy()
            run_env["CPU_PROFILE_OUT"] = str(profile)
            cmd = [str(B), "compile", str(f), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            proc = subprocess.run(cmd, cwd=R, env=run_env, capture_output=True, text=True, timeout=120,
                                  preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
            assert proc.returncode == 0, (cmd, proc.stderr)
            lines = profile.read_text().splitlines()
            assert lines[0].startswith("delivered=") and len(lines) > 100, (cmd, lines[:2])
            state["runs"].append({"fixture": f.stem, "rep": rep, "utc": utc(), "command": cmd,
                                  "header": lines[0], "stacks": lines[1:], "stderr": proc.stderr,
                                  "pdf_sha256": sha(pdf)})
        print("round", rep, "done", flush=True)

pcs = sorted({pc for row in state["runs"] for line in row["stacks"] for pc in line.split()}, key=lambda x: int(x, 16))
sym = subprocess.run(["eu-addr2line", "-f", "-C", "-e", str(B), "-a"],
                     input="".join(f"0x{pc}\n" for pc in pcs), text=True, capture_output=True, check=True)
sym_lines = sym.stdout.splitlines()
assert len(sym_lines) == 3 * len(pcs), (len(sym_lines), len(pcs))
symbols = {format(int(sym_lines[i], 16), "x"): sym_lines[i + 1] for i in range(0, len(sym_lines), 3)}
state["symbols"] = symbols
anchors = {
    "layout_styled": "style_scope::layout_styled",
    "context_collect": "collect_context_blocks_into",
    "table_map": "TableElem$u20$as",
    "cell_map": "TableCellElem$u20$as",
    "style_push": "StyleChain::push_styles",
    "style_collapse": "StyleChain::collapse",
    "font_selection": "FontBook::select_pattern",
    "cell_defaults": "table_cells::with_defaults",
    "layout_grid": "layout::grid",
    "layout_table": "layout::table",
    "eval_table": "eval::table",
}
summary = {}
for f in fixtures:
    counts = Counter()
    leaf = Counter()
    total = 0
    per_run = []
    for row in state["runs"]:
        if row["fixture"] != f.stem:
            continue
        local = Counter()
        for line in row["stacks"]:
            frames = [symbols[pc] for pc in line.split()]
            total += 1
            if len(frames) > 2:
                leaf[frames[2]] += 1
            for name, needle in anchors.items():
                if any(needle in frame for frame in frames):
                    counts[name] += 1
                    local[name] += 1
        per_run.append({"rep": row["rep"], "samples": len(row["stacks"]), "anchors": dict(local)})
    summary[f.stem] = {"total": total, "anchors": dict(counts), "top_leaf": leaf.most_common(30), "per_run": per_run}
state["summary"] = summary
state["utc_end"] = utc()
assert sha(B) == state["binary_sha256"] and sha(SO) == state["interposer_sha256"]
assert sha(__file__) == state["driver_sha256"]
assert all(sha(F / name) == item["sha256"] for name, item in state["fixtures"].items())
with gzip.open(D / "cpu-style-current.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
for name, item in summary.items():
    print(name, item["total"], item["anchors"], item["top_leaf"][:8], flush=True)
