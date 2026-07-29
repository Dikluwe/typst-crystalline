#!/usr/bin/env python3
"""Benchmark P929 — Ideia 2: cache em disco da coverage.

Compara o binário baseline (P927, target-original/release/typst) contra o
binário com P929-proto2, fazendo duas passagens:
  - run1: cache em disco vazio (miss);
  - run2: cache em disco preenchido (hit).
"""

import hashlib
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p929-ideia2"
CORPUS_CANONICAL = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
CORPUS_UTF8 = ROOT / "tools" / "perf" / "corpus" / "p923"
CACHE_DIR = ROOT / "target" / "tmp" / "coverage-cache"
ORIGINAL_BIN = Path.home() / "Documentos" / "Antigravity" / "typst-crystalline" / "target-original" / "release" / "typst"
PROTO_BIN = ROOT / "target" / "release" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP_CANONICAL = 5
MIN_RUNS_CANONICAL = 20
WARMUP_UTF8 = 2
MIN_RUNS_UTF8 = 10


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def run(cmd, **kwargs):
    print(f"[p929-ideia2] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path, out: Path, warmup: int, min_runs: int):
    out.parent.mkdir(parents=True, exist_ok=True)
    cmd_original = f"{ORIGINAL_BIN} {doc} /dev/null"
    cmd_proto = f"{PROTO_BIN} {doc} /dev/null"
    run([
        str(HYPERFINE),
        "--warmup", str(warmup),
        "--min-runs", str(min_runs),
        "--export-json", str(out),
        "--command-name", "baseline",
        "--command-name", "p929",
        cmd_original,
        cmd_proto,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def benchmark_suite(suite_name: str, docs, corpus_dir: Path, warmup: int, min_runs: int):
    rows = []
    for name, _ in docs:
        doc = corpus_dir / f"{name}.typ"
        if not doc.exists():
            print(f"[p929-ideia2] SKIP {suite_name}/{name}: ficheiro não encontrado: {doc}")
            continue
        out = RESULTS_DIR / suite_name / f"{name}.json"
        print(f"[p929-ideia2] {suite_name}/{name}", flush=True)
        hf = run_hyperfine(name, doc, out, warmup, min_runs)
        original_ms = hf.get("baseline", 0.0)
        proto_ms = hf.get("p929", 0.0)
        ratio = proto_ms / original_ms if original_ms > 0 else float("inf")
        rows.append({
            "name": name,
            "baseline_ms": original_ms,
            "p929_ms": proto_ms,
            "ratio": ratio,
        })
        print(f"[p929-ideia2] {suite_name}/{name}: baseline={original_ms:.2f}ms p929={proto_ms:.2f}ms ratio={ratio:.2f}x")
    return rows


def main():
    canonical_docs = [
        ("01-hello", False),
        ("02-lorem", False),
        ("03-images", False),
        ("04-math", False),
        ("05-tables", False),
        ("06-long", False),
        ("07-context", False),
    ]
    utf8_docs = [
        ("05-utf8", True),
        ("utf8-latin", False),
        ("utf8-greek", False),
        ("utf8-cjk", False),
        ("utf8-emoji", False),
    ]

    # Run 1: cache vazio.
    if CACHE_DIR.exists():
        shutil.rmtree(CACHE_DIR)
    print("[p929-ideia2] === RUN 1 (cache vazio) ===", flush=True)
    canonical_run1 = benchmark_suite(
        "canonical-run1", canonical_docs, CORPUS_CANONICAL,
        WARMUP_CANONICAL, MIN_RUNS_CANONICAL
    )
    utf8_run1 = benchmark_suite(
        "utf8-run1", utf8_docs, CORPUS_UTF8,
        WARMUP_UTF8, MIN_RUNS_UTF8
    )

    # Run 2: cache preenchido.
    print("[p929-ideia2] === RUN 2 (cache preenchido) ===", flush=True)
    canonical_run2 = benchmark_suite(
        "canonical-run2", canonical_docs, CORPUS_CANONICAL,
        WARMUP_CANONICAL, MIN_RUNS_CANONICAL
    )
    utf8_run2 = benchmark_suite(
        "utf8-run2", utf8_docs, CORPUS_UTF8,
        WARMUP_UTF8, MIN_RUNS_UTF8
    )

    attestation = {
        "passo": "P929-ideia2",
        "binaries": {
            "baseline": {"path": str(ORIGINAL_BIN), "sha256": sha256_file(ORIGINAL_BIN)},
            "p929": {"path": str(PROTO_BIN), "sha256": sha256_file(PROTO_BIN)},
        },
        "canonical_run1": canonical_run1,
        "utf8_run1": utf8_run1,
        "canonical_run2": canonical_run2,
        "utf8_run2": utf8_run2,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.parent.mkdir(parents=True, exist_ok=True)
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[p929-ideia2] Attestation escrita em {attestation_path}")


if __name__ == "__main__":
    main()
