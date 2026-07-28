#!/usr/bin/env python3
"""Benchmark P922 — gap real de acento com `accent_base_height`.

Compara Typst vanilla 0.15.0 vs cristalino (release) em 7 cenários de
acentos/underover. Corrige cada cenário com `hyperfine --warmup 5 --min-runs 20`,
coleta `--timings-json` do cristalino e gera attestation com hashes dos
binários e resultados.
"""

import hashlib
import json
import subprocess
from pathlib import Path
from statistics import mean

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p922"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p922"
VANILLA = ROOT / "lab" / "typst-original" / "target" / "release" / "typst"
CRISTALINO = ROOT / "target" / "release" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 5
MIN_RUNS = 20


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def vanilla_compiles(doc: Path) -> bool:
    tmp = RESULTS_DIR / f"_compat-{doc.stem}.pdf"
    try:
        subprocess.run(
            [str(VANILLA), "compile", str(doc), str(tmp)],
            check=True,
            capture_output=True,
        )
        return True
    except subprocess.CalledProcessError:
        return False
    finally:
        tmp.unlink(missing_ok=True)


def run_hyperfine(name: str, doc: Path):
    out = RESULTS_DIR / f"{name}.json"
    vanilla_pdf = RESULTS_DIR / f"{name}-vanilla.pdf"
    cristalino_pdf = RESULTS_DIR / f"{name}-cristalino.pdf"
    cmd_vanilla = f"{VANILLA} compile {doc} {vanilla_pdf}"
    cmd_cristalino = f"{CRISTALINO} {doc} {cristalino_pdf}"
    subprocess.run(
        [
            str(HYPERFINE),
            "--warmup", str(WARMUP),
            "--min-runs", str(MIN_RUNS),
            "--export-json", str(out),
            "--command-name", "vanilla",
            "--command-name", "cristalino",
            cmd_vanilla,
            cmd_cristalino,
        ],
        check=True,
        capture_output=True,
    )
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def run_timings(doc: Path, output_pdf: Path, timings_json: Path):
    subprocess.run(
        [str(CRISTALINO), str(doc), str(output_pdf), "--timings-json", str(timings_json)],
        check=True,
        capture_output=True,
    )
    return json.loads(timings_json.read_text())


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    docs = sorted(CORPUS_DIR.glob("*.typ"))
    rows = []
    timings_rows = []

    for doc in docs:
        name = doc.stem
        if not vanilla_compiles(doc):
            print(f"[P922] SKIP {name}: vanilla 0.15.0 não compila")
            continue

        print(f"[P922] {name}", flush=True)
        hf = run_hyperfine(name, doc)
        timings = run_timings(
            doc,
            RESULTS_DIR / f"{name}-cristalino.pdf",
            RESULTS_DIR / f"{name}-timings.json",
        )

        vanilla_ms = hf.get("vanilla", 0.0)
        cristalino_ms = hf.get("cristalino", 0.0)
        ratio = cristalino_ms / vanilla_ms if vanilla_ms > 0 else float("inf")

        rows.append({
            "name": name,
            "vanilla_ms": vanilla_ms,
            "cristalino_ms": cristalino_ms,
            "ratio": ratio,
        })

        # Timings internos do cristalino: fases eval/layout/export.
        timings_rows.append({
            "name": name,
            "timings": timings,
        })

    attestation = {
        "passo": "P922",
        "warmup": WARMUP,
        "min_runs": MIN_RUNS,
        "binaries": {
            "vanilla": {"path": str(VANILLA), "sha256": sha256_file(VANILLA)},
            "cristalino": {"path": str(CRISTALINO), "sha256": sha256_file(CRISTALINO)},
        },
        "hyperfine": rows,
        "timings": timings_rows,
    }
    attestation_path = RESULTS_DIR / "attestation.json"
    attestation_path.write_text(json.dumps(attestation, indent=2))

    print(f"\n[P922] Attestation escrita em {attestation_path}")
    print(json.dumps(rows, indent=2))


if __name__ == "__main__":
    main()
