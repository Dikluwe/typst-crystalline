#!/usr/bin/env python3
"""Benchmark P927 — impacto no caso comum do scan condicional de coverage.

Compara o binário cristalino original (commit `da18ea9f3`, sem protótipo)
contra o binário do protótipo da Opção 6 (working tree com scan condicional
no source bruto).

Usa os 7 cenários canônicos da frente (P872-P921), output para /dev/null.
"""

import hashlib
import json
import subprocess
from pathlib import Path
from statistics import mean

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p927-canonical"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
ORIGINAL_BIN = Path.home() / "Documentos" / "Antigravity" / "typst-crystalline" / "target-original" / "release" / "typst"
PROTO_BIN = ROOT / "target" / "release" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 5
MIN_RUNS = 20


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def run(cmd, **kwargs):
    print(f"[p927] {' '.join(str(c) for c in cmd)}")
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


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    docs = [
        ("01-hello", CORPUS_DIR / "01-hello.typ"),
        ("02-lorem", CORPUS_DIR / "02-lorem.typ"),
        ("03-images", CORPUS_DIR / "03-images.typ"),
        ("04-math", CORPUS_DIR / "04-math.typ"),
        ("05-tables", CORPUS_DIR / "05-tables.typ"),
        ("06-long", CORPUS_DIR / "06-long.typ"),
        ("07-context", CORPUS_DIR / "07-context.typ"),
    ]

    rows = []
    for name, doc in docs:
        if not doc.exists():
            print(f"[p927] SKIP {name}: ficheiro não encontrado: {doc}")
            continue
        print(f"[p927] {name}", flush=True)
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
        print(f"[p927] {name}: original={original_ms:.2f}ms proto={proto_ms:.2f}ms ratio={ratio:.2f}x")

    attestation = {
        "passo": "P927-canonical",
        "warmup": WARMUP,
        "min_runs": MIN_RUNS,
        "binaries": {
            "original": {"path": str(ORIGINAL_BIN), "sha256": sha256_file(ORIGINAL_BIN)},
            "proto": {"path": str(PROTO_BIN), "sha256": sha256_file(PROTO_BIN)},
        },
        "hyperfine": rows,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[p927] Attestation escrita em {attestation_path}")
    print(json.dumps(rows, indent=2))


if __name__ == "__main__":
    main()
