#!/usr/bin/env python3
"""Benchmark P931 Braço 1 — cache único de metadados de fontes.

Compara o binário de referência P932-lazy contra o protótipo com cache de
fontes. Mede run fria (cache apagado antes de cada execução) e run quente
(cache populado). O warmup do hyperfine é desligado para runs frias para
que cada execução reconstrua o cache.
"""

import hashlib
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p931-braco1"
CACHE_DIR = ROOT / "target" / "tmp" / "font-cache"
CORPUS_CANONICAL = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
CORPUS_UTF8 = ROOT / "tools" / "perf" / "corpus" / "p923"
REF_BIN = ROOT / "target-original" / "release" / "typst"
PROTO_BIN = ROOT / "target" / "tmp" / "typst-braco1"
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


def clear_cache():
    shutil.rmtree(CACHE_DIR, ignore_errors=True)


def ensure_cache():
    # Executa uma vez para popular o cache.
    subprocess.run(
        [str(PROTO_BIN), str(CORPUS_CANONICAL / "01-hello.typ"), "/dev/null"],
        check=True, capture_output=True,
    )


def run(cmd, **kwargs):
    print(f"[p931-braco1] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(
    name: str,
    doc: Path,
    out: Path,
    warmup: int,
    min_runs: int,
    cold: bool,
):
    out.parent.mkdir(parents=True, exist_ok=True)
    # Comando do protótipo envolve apagar ou garantir cache antes de cada run.
    if cold:
        cmd_proto = f"rm -rf {CACHE_DIR} && {PROTO_BIN} {doc} /dev/null"
    else:
        cmd_proto = f"{PROTO_BIN} {doc} /dev/null"
    cmd_ref = f"{REF_BIN} {doc} /dev/null"

    # Para runs frias, sem warmup (o warmup invalidaria a medição).
    warmup_arg = "0" if cold else str(warmup)

    run([
        str(HYPERFINE),
        "--warmup", warmup_arg,
        "--min-runs", str(min_runs),
        "--export-json", str(out),
        "--command-name", "p932-lazy",
        "--command-name", "p931-braco1",
        cmd_ref,
        cmd_proto,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def benchmark_suite(
    suite_name: str,
    docs,
    corpus_dir: Path,
    warmup: int,
    min_runs: int,
    cold: bool,
):
    rows = []
    for name, doc in docs:
        if not doc.exists():
            print(f"[p931-braco1] SKIP {suite_name}/{name}: ficheiro não encontrado: {doc}")
            continue
        state = "cold" if cold else "hot"
        out = RESULTS_DIR / suite_name / state / f"{name}.json"
        print(f"[p931-braco1] {suite_name}/{state}/{name}", flush=True)
        if not cold:
            ensure_cache()
        hf = run_hyperfine(name, doc, out, warmup, min_runs, cold)
        ref_ms = hf.get("p932-lazy", 0.0)
        proto_ms = hf.get("p931-braco1", 0.0)
        ratio = proto_ms / ref_ms if ref_ms > 0 else float("inf")
        rows.append({
            "name": name,
            "state": state,
            "ref_ms": ref_ms,
            "proto_ms": proto_ms,
            "ratio": ratio,
        })
        print(f"[p931-braco1] {suite_name}/{state}/{name}: ref={ref_ms:.2f}ms proto={proto_ms:.2f}ms ratio={ratio:.2f}x")
    return rows


def main():
    clear_cache()

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

    canonical_cold = benchmark_suite(
        "canonical", canonical_docs, CORPUS_CANONICAL,
        WARMUP_CANONICAL, MIN_RUNS_CANONICAL, cold=True
    )
    canonical_hot = benchmark_suite(
        "canonical", canonical_docs, CORPUS_CANONICAL,
        WARMUP_CANONICAL, MIN_RUNS_CANONICAL, cold=False
    )
    utf8_cold = benchmark_suite(
        "utf8", utf8_docs, CORPUS_UTF8,
        WARMUP_UTF8, MIN_RUNS_UTF8, cold=True
    )
    utf8_hot = benchmark_suite(
        "utf8", utf8_docs, CORPUS_UTF8,
        WARMUP_UTF8, MIN_RUNS_UTF8, cold=False
    )

    attestation = {
        "passo": "P931-braco1",
        "binaries": {
            "p932_lazy": {"path": str(REF_BIN), "sha256": sha256_file(REF_BIN)},
            "p931_braco1": {"path": str(PROTO_BIN), "sha256": sha256_file(PROTO_BIN)},
        },
        "canonical_cold": canonical_cold,
        "canonical_hot": canonical_hot,
        "utf8_cold": utf8_cold,
        "utf8_hot": utf8_hot,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.parent.mkdir(parents=True, exist_ok=True)
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[p931-braco1] Attestation escrita em {attestation_path}")


if __name__ == "__main__":
    main()
