#!/usr/bin/env python3
"""Benchmark P926 — casos UTF-8 (05-utf8 + blocos Unicode isolados).

Compara o binário cristalino original (target-original/release/typst) contra
o protótipo da Opção 5 (thread de fundo, target/release/typst).
"""

import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p926-utf8"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p923"
ORIGINAL_BIN = Path.home() / "Documentos" / "Antigravity" / "typst-crystalline" / "target-original" / "release" / "typst"
PROTO_BIN = ROOT / "target" / "release" / "typst"
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
    print(f"[p926-utf8] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path):
    out = RESULTS_DIR / f"{name}.json"
    cmd_original = f"{ORIGINAL_BIN} {doc} /dev/null"
    cmd_proto = f"{PROTO_BIN} {doc} /dev/null"
    run([
        str(HYPERFINE),
        "--warmup", str(WARMUP),
        "--min-runs", str(MIN_RUNS),
        "--export-json", str(out),
        "--command-name", "original",
        "--command-name", "proto",
        cmd_original,
        cmd_proto,
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

    docs = [
        ("05-utf8", CORPUS_DIR / "05-utf8.typ", True),
        ("utf8-latin", CORPUS_DIR / "utf8-latin.typ", False),
        ("utf8-greek", CORPUS_DIR / "utf8-greek.typ", False),
        ("utf8-cjk", CORPUS_DIR / "utf8-cjk.typ", False),
        ("utf8-emoji", CORPUS_DIR / "utf8-emoji.typ", False),
    ]

    rows = []
    timings = {}
    for name, doc, capture_timings in docs:
        if not doc.exists():
            print(f"[p926-utf8] SKIP {name}: ficheiro não encontrado: {doc}")
            continue
        print(f"[p926-utf8] {name}", flush=True)
        hf = run_hyperfine(name, doc)
        original_ms = hf.get("original", 0.0)
        proto_ms = hf.get("proto", 0.0)
        ratio = proto_ms / original_ms if original_ms > 0 else float("inf")
        rows.append({
            "name": name,
            "original_ms": original_ms,
            "proto_ms": proto_ms,
            "ratio": ratio,
        })
        print(f"[p926-utf8] {name}: original={original_ms:.2f}ms proto={proto_ms:.2f}ms ratio={ratio:.2f}x")

        if capture_timings:
            timings["original"] = run_timings(doc, ORIGINAL_BIN, RESULTS_DIR / f"{name}-original-timings.json")
            timings["proto"] = run_timings(doc, PROTO_BIN, RESULTS_DIR / f"{name}-proto-timings.json")

    attestation = {
        "passo": "P926-utf8",
        "warmup": WARMUP,
        "min_runs": MIN_RUNS,
        "binaries": {
            "original": {"path": str(ORIGINAL_BIN), "sha256": sha256_file(ORIGINAL_BIN)},
            "proto": {"path": str(PROTO_BIN), "sha256": sha256_file(PROTO_BIN)},
        },
        "hyperfine": rows,
        "timings": timings,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[p926-utf8] Attestation escrita em {attestation_path}")
    print(json.dumps(rows, indent=2))


if __name__ == "__main__":
    main()
