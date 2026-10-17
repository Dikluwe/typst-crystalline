"""Reproduce sampled glibc allocation stacks for equivalent table variants."""
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
B = R / "target/release/typst"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
SO = D / "stack_probe_deep.so"
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


build = ["gcc", "-shared", "-fPIC", "-O2", "-std=c11", "-Wall", "-Wextra", "-o", str(SO), str(D / "stack_probe_deep.c"), "-ldl"]
subprocess.run(build, check=True, cwd=R)
fixtures = [F / f"{v}-1600.typ" for v in ("block-no-set", "local-set")]
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
    "binary_sha256": sha(B),
    "source_sha256": sha(D / "stack_probe_deep.c"),
    "interposer_sha256": sha(SO),
    "driver_sha256": sha(D / "stack_profile_after_reuse.py"),
    "build": build,
    "cpu": CPU,
    "fixtures": {p.name: {"text": p.read_text(), "sha256": sha(p)} for p in fixtures},
    "samples": [],
    "scope": "One of every 64 successful glibc allocation requests; stack unwinding and symbolization are observational and can perturb timings; function presence is not exclusive ownership.",
}

with tempfile.TemporaryDirectory(prefix="table-stack-") as tmp:
    out = Path(tmp)
    for f in fixtures:
        for rep in range(2):
            profile = out / f"{f.stem}-{rep}.txt"
            pdf = out / f"{f.stem}-{rep}.pdf"
            run_env = env.copy()
            run_env["STACK_PROFILE_OUT"] = str(profile)
            cmd = [str(B), "compile", str(f), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            proc = subprocess.run(cmd, cwd=R, env=run_env, capture_output=True, text=True, check=True, timeout=120,
                                  preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
            lines = profile.read_text().splitlines()
            state["samples"].append({"fixture": f.stem, "rep": rep, "utc": utc(), "command": cmd,
                                     "header": lines[0], "stacks": lines[1:], "stderr": proc.stderr,
                                     "pdf_sha256": sha(pdf)})
        print(f.stem, "done", flush=True)

pcs = sorted({pc for row in state["samples"] for line in row["stacks"] for pc in line.split()[2:]}, key=lambda x: int(x, 16))
sym = subprocess.run(["eu-addr2line", "-f", "-C", "-e", str(B), "-a"], input="".join(f"0x{pc}\n" for pc in pcs),
                     text=True, capture_output=True, check=True)
sym_lines = sym.stdout.splitlines()
assert len(sym_lines) == 3 * len(pcs)
symbols = {format(int(sym_lines[i], 16), "x"): sym_lines[i + 1] for i in range(0, len(sym_lines), 3)}
state["symbols"] = symbols

anchors = {
    "style_scope": "style_scope::layout_styled",
    "context_collect": "collect_context_blocks_into",
    "table_map": "TableElem$u20$as",
    "cell_map": "TableCellElem$u20$as",
    "style_push": "StyleChain::push_styles",
    "style_collapse": "StyleChain::collapse",
    "font_selection": "FontBook::select_pattern",
    "cell_defaults": "table_cells::with_defaults",
}
state["summary"] = {}
for row in state["samples"]:
    counts = Counter()
    for line in row["stacks"]:
        frames = [symbols[pc] for pc in line.split()[2:]]
        for name, needle in anchors.items():
            if any(needle in frame for frame in frames):
                counts[name] += 1
        if any(anchors["style_scope"] in frame for frame in frames):
            counts["exclusive_style_scope"] += 1
        elif any(anchors["context_collect"] in frame for frame in frames):
            counts["exclusive_context_collect"] += 1
        else:
            counts["exclusive_other"] += 1
    state["summary"][f"{row['fixture']}-{row['rep']}"] = dict(counts)
state["utc_end"] = utc()
assert sha(B) == state["binary_sha256"] and sha(SO) == state["interposer_sha256"]
with gzip.open(D / "stacks-after-reuse.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
for name, counts in state["summary"].items():
    print(name, {key: counts.get(key, 0) for key in ("exclusive_style_scope", "exclusive_context_collect", "exclusive_other", "table_map", "style_collapse")})
