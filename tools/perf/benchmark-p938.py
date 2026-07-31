#!/usr/bin/env python3
"""Benchmark P938 — coverage exacta lazy.

Compara o binário cristalino P938 (coverage exacta lazy) contra:
- P937 (coverage exacta eager)
- P933-fixed (coverage aproximada lazy)
- vanilla real (lab/typst-original)

Usa os 11 cenários do corpus p923 (7 canônicos + 4 UTF-8).
"""

import hashlib
import json
import subprocess
from pathlib import Path
from statistics import mean

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p938"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p923"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 1
MIN_RUNS = 10

BINS = {
    "p938": ROOT / "target" / "release" / "typst-p938",
    "p937": ROOT / "target" / "release" / "typst-p937",
    "p933": ROOT / "target" / "release" / "typst-p933",
    "vanilla": ROOT / "lab" / "typst-original" / "target" / "release" / "typst",
}


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def run(cmd, **kwargs):
    print(f"[p938] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path):
    out = RESULTS_DIR / f"{name}.json"
    commands = [
        f"{BINS['p938']} {doc} /dev/null",
        f"{BINS['p937']} {doc} /dev/null",
        f"{BINS['p933']} {doc} /dev/null",
        f"{BINS['vanilla']} compile {doc} /dev/null -f pdf",
    ]
    run([
        str(HYPERFINE),
        "--warmup", str(WARMUP),
        "--min-runs", str(MIN_RUNS),
        "--export-json", str(out),
        "--command-name", "p938",
        "--command-name", "p937",
        "--command-name", "p933",
        "--command-name", "vanilla",
        *commands,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    docs = [
        ("01-hello", CORPUS_DIR / "01-hello.typ"),
        ("02-lorem", CORPUS_DIR / "02-lorem.typ"),
        ("03-math", CORPUS_DIR / "03-math.typ"),
        ("04-code", CORPUS_DIR / "04-code.typ"),
        ("05-utf8", CORPUS_DIR / "05-utf8.typ"),
        ("06-matrix", CORPUS_DIR / "06-matrix.typ"),
        ("07-cases", CORPUS_DIR / "07-cases.typ"),
        ("utf8-latin", CORPUS_DIR / "utf8-latin.typ"),
        ("utf8-greek", CORPUS_DIR / "utf8-greek.typ"),
        ("utf8-cjk", CORPUS_DIR / "utf8-cjk.typ"),
        ("utf8-emoji", CORPUS_DIR / "utf8-emoji.typ"),
    ]

    rows = []
    for name, doc in docs:
        if not doc.exists():
            print(f"[p938] SKIP {name}: ficheiro não encontrado: {doc}")
            continue
        print(f"[p938] {name}", flush=True)
        hf = run_hyperfine(name, doc)
        p938_ms = hf.get("p938", 0.0)
        p937_ms = hf.get("p937", 0.0)
        p933_ms = hf.get("p933", 0.0)
        vanilla_ms = hf.get("vanilla", 0.0)
        rows.append({
            "name": name,
            "p938_ms": p938_ms,
            "p937_ms": p937_ms,
            "p933_ms": p933_ms,
            "vanilla_ms": vanilla_ms,
            "ratio_p938_p937": p938_ms / p937_ms if p937_ms > 0 else float("inf"),
            "ratio_p938_p933": p938_ms / p933_ms if p933_ms > 0 else float("inf"),
            "ratio_p938_vanilla": p938_ms / vanilla_ms if vanilla_ms > 0 else float("inf"),
        })
        print(
            f"[p938] {name}: p938={p938_ms:.2f}ms p937={p937_ms:.2f}ms "
            f"p933={p933_ms:.2f}ms vanilla={vanilla_ms:.2f}ms"
        )

    attestation = {
        "passo": "P938",
        "warmup": WARMUP,
        "min_runs": MIN_RUNS,
        "corpus": str(CORPUS_DIR),
        "binaries": {
            k: {"path": str(v), "sha256": sha256_file(v)}
            for k, v in BINS.items()
        },
        "scenarios": rows,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.write_text(json.dumps(attestation, indent=2))
    print(f"\n[p938] Attestation escrita em {attestation_path}")
    print(json.dumps(rows, indent=2))


if __name__ == "__main__":
    main()
