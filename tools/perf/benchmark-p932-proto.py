#!/usr/bin/env python3
"""Benchmark P931-proto — coverage calculada no arranque (estilo vanilla).

Compara o binário cristalino P927 (target-original/release/typst) contra o
binário com coverage preenchida durante font_info_from_bytes (target/release/typst).
"""

import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p932-proto"
CORPUS_CANONICAL = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
CORPUS_UTF8 = ROOT / "tools" / "perf" / "corpus" / "p923"
P927_BIN = Path.home() / "Documentos" / "Antigravity" / "typst-crystalline" / "target-original" / "release" / "typst"
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
    print(f"[p932-proto] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path, out: Path, warmup: int, min_runs: int):
    out.parent.mkdir(parents=True, exist_ok=True)
    cmd_p927 = f"{P927_BIN} {doc} /dev/null"
    cmd_proto = f"{PROTO_BIN} {doc} /dev/null"
    run([
        str(HYPERFINE),
        "--warmup", str(warmup),
        "--min-runs", str(min_runs),
        "--export-json", str(out),
        "--command-name", "p927",
        "--command-name", "p932-proto",
        cmd_p927,
        cmd_proto,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def benchmark_suite(suite_name: str, docs, corpus_dir: Path, warmup: int, min_runs: int):
    rows = []
    for name, doc in docs:
        if not doc.exists():
            print(f"[p932-proto] SKIP {suite_name}/{name}: ficheiro não encontrado: {doc}")
            continue
        out = RESULTS_DIR / suite_name / f"{name}.json"
        print(f"[p932-proto] {suite_name}/{name}", flush=True)
        hf = run_hyperfine(name, doc, out, warmup, min_runs)
        p927_ms = hf.get("p927", 0.0)
        proto_ms = hf.get("p932-proto", 0.0)
        ratio = proto_ms / p927_ms if p927_ms > 0 else float("inf")
        rows.append({
            "name": name,
            "p927_ms": p927_ms,
            "p932_proto_ms": proto_ms,
            "ratio": ratio,
        })
        print(f"[p932-proto] {suite_name}/{name}: p927={p927_ms:.2f}ms p932-proto={proto_ms:.2f}ms ratio={ratio:.2f}x")
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
        "passo": "P931-proto",
        "binaries": {
            "p927": {"path": str(P927_BIN), "sha256": sha256_file(P927_BIN)},
            "p932_proto": {"path": str(PROTO_BIN), "sha256": sha256_file(PROTO_BIN)},
        },
        "canonical": canonical,
        "utf8": utf8,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.parent.mkdir(parents=True, exist_ok=True)
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[p932-proto] Attestation escrita em {attestation_path}")


if __name__ == "__main__":
    main()
