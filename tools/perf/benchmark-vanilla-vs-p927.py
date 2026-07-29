#!/usr/bin/env python3
"""Benchmark vanilla vs P927.

Compara o binário Typst original (lab/typst-original/target/release/typst)
contra o binário cristalino no estado P927 (target-original/release/typst).
"""

import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "vanilla-vs-p927"
CORPUS_CANONICAL = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
CORPUS_UTF8 = ROOT / "tools" / "perf" / "corpus" / "p923"
VANILLA_BIN = Path.home() / "Documentos" / "Antigravity" / "typst-crystalline" / "lab" / "typst-original" / "target" / "release" / "typst"
P927_BIN = Path.home() / "Documentos" / "Antigravity" / "typst-crystalline" / "target-original" / "release" / "typst"
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
    print(f"[vanilla-vs-p927] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path, out: Path, warmup: int, min_runs: int):
    out.parent.mkdir(parents=True, exist_ok=True)
    # Vanilla usa subcomando `compile` e exige --format quando o output nao tem extensao.
    cmd_vanilla = f"{VANILLA_BIN} compile {doc} /dev/null --format pdf"
    cmd_p927 = f"{P927_BIN} {doc} /dev/null"
    run([
        str(HYPERFINE),
        "--warmup", str(warmup),
        "--min-runs", str(min_runs),
        "--export-json", str(out),
        "--command-name", "vanilla",
        "--command-name", "p927",
        cmd_vanilla,
        cmd_p927,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def benchmark_suite(suite_name: str, docs, corpus_dir: Path, warmup: int, min_runs: int):
    rows = []
    for name, doc in docs:
        if not doc.exists():
            print(f"[vanilla-vs-p927] SKIP {suite_name}/{name}: ficheiro não encontrado: {doc}")
            continue
        out = RESULTS_DIR / suite_name / f"{name}.json"
        print(f"[vanilla-vs-p927] {suite_name}/{name}", flush=True)
        hf = run_hyperfine(name, doc, out, warmup, min_runs)
        vanilla_ms = hf.get("vanilla", 0.0)
        p927_ms = hf.get("p927", 0.0)
        ratio = p927_ms / vanilla_ms if vanilla_ms > 0 else float("inf")
        rows.append({
            "name": name,
            "vanilla_ms": vanilla_ms,
            "p927_ms": p927_ms,
            "ratio": ratio,
        })
        print(f"[vanilla-vs-p927] {suite_name}/{name}: vanilla={vanilla_ms:.2f}ms p927={p927_ms:.2f}ms ratio={ratio:.2f}x")
    return rows


def main():
    canonical_docs = [
        ("01-hello", CORPUS_CANONICAL / "01-hello.typ"),
        ("02-lorem", CORPUS_CANONICAL / "02-lorem.typ"),
        ("03-images", CORPUS_CANONICAL / "03-images.typ"),
        ("04-math", CORPUS_CANONICAL / "04-math.typ"),
        ("05-tables", CORPUS_CANONICAL / "05-tables.typ"),
        ("06-long", CORPUS_CANONICAL / "06-long.typ"),
        ("07-context", CORPUS_CANONICAL / "07-context.typ"),
    ]
    utf8_docs = [
        ("05-utf8", CORPUS_UTF8 / "05-utf8.typ"),
        ("utf8-latin", CORPUS_UTF8 / "utf8-latin.typ"),
        ("utf8-greek", CORPUS_UTF8 / "utf8-greek.typ"),
        ("utf8-cjk", CORPUS_UTF8 / "utf8-cjk.typ"),
        ("utf8-emoji", CORPUS_UTF8 / "utf8-emoji.typ"),
    ]

    canonical = benchmark_suite(
        "canonical", canonical_docs, CORPUS_CANONICAL,
        WARMUP_CANONICAL, MIN_RUNS_CANONICAL
    )
    utf8 = benchmark_suite(
        "utf8", utf8_docs, CORPUS_UTF8,
        WARMUP_UTF8, MIN_RUNS_UTF8
    )

    attestation = {
        "passo": "vanilla-vs-p927",
        "binaries": {
            "vanilla": {"path": str(VANILLA_BIN), "sha256": sha256_file(VANILLA_BIN)},
            "p927": {"path": str(P927_BIN), "sha256": sha256_file(P927_BIN)},
        },
        "canonical": canonical,
        "utf8": utf8,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.parent.mkdir(parents=True, exist_ok=True)
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[vanilla-vs-p927] Attestation escrita em {attestation_path}")


if __name__ == "__main__":
    main()
