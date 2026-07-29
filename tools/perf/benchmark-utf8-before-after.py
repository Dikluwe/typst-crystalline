#!/usr/bin/env python3
"""Benchmark 05-utf8 — compara commit before (c9df6fde) vs after (da18ea9f3).

Usa os binários cristalinos dos dois commits para determinar se o outlier
25.91× é regressão de P922/P923 ou problema pré-existente do fallback lazy de
fontes.
"""

import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "utf8-before-after"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p923"
BEFORE_BIN = ROOT.parent / "typst-crystalline-utf8-before" / "target" / "release" / "typst"
AFTER_BIN = ROOT / "target" / "release" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 2
MIN_RUNS = 10


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def run(cmd, **kwargs):
    print(f"[utf8] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path):
    out = RESULTS_DIR / f"{name}.json"
    cmd_before = f"{BEFORE_BIN} {doc} /dev/null"
    cmd_after = f"{AFTER_BIN} {doc} /dev/null"
    run([
        str(HYPERFINE),
        "--warmup", str(WARMUP),
        "--min-runs", str(MIN_RUNS),
        "--export-json", str(out),
        "--command-name", "before",
        "--command-name", "after",
        cmd_before,
        cmd_after,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def run_timings(doc: Path, bin: Path, timings_json: Path):
    run([
        str(bin), str(doc), "/dev/null",
        "--timings-json", str(timings_json),
    ], capture_output=True)
    return json.loads(timings_json.read_text())


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    doc = CORPUS_DIR / "05-utf8.typ"
    if not doc.exists():
        raise FileNotFoundError(doc)
    if not BEFORE_BIN.exists():
        raise FileNotFoundError(BEFORE_BIN)
    if not AFTER_BIN.exists():
        raise FileNotFoundError(AFTER_BIN)

    print("[utf8] 05-utf8.typ — before vs after")
    hf = run_hyperfine("05-utf8", doc)
    timings_before = run_timings(doc, BEFORE_BIN, RESULTS_DIR / "05-utf8-before-timings.json")
    timings_after = run_timings(doc, AFTER_BIN, RESULTS_DIR / "05-utf8-after-timings.json")

    before_ms = hf["before"]
    after_ms = hf["after"]
    ratio = after_ms / before_ms if before_ms > 0 else float("inf")

    def layout_ms(t):
        return t.get("layout_ms", 0.0)

    def shape_ms(t):
        return t.get("shape_ms", 0.0)

    attestation = {
        "commit_before": "c9df6fde",
        "commit_after": "da18ea9f3",
        "binary_before_sha256": sha256_file(BEFORE_BIN),
        "binary_after_sha256": sha256_file(AFTER_BIN),
        "doc": str(doc),
        "hyperfine_ms": {"before": before_ms, "after": after_ms, "ratio_after_before": ratio},
        "timings_before": timings_before,
        "timings_after": timings_after,
    }
    (RESULTS_DIR / "attestation.json").write_text(json.dumps(attestation, indent=2))

    print(f"\nbefore: {before_ms:.2f} ms")
    print(f"after:  {after_ms:.2f} ms")
    print(f"ratio after/before: {ratio:.2f}×")
    print(f"layout before: {layout_ms(timings_before):.2f} ms")
    print(f"layout after:  {layout_ms(timings_after):.2f} ms")
    print(f"shape before: {shape_ms(timings_before):.2f} ms")
    print(f"shape after:  {shape_ms(timings_after):.2f} ms")
    print(f"\nresultados em {RESULTS_DIR}")


if __name__ == "__main__":
    main()
