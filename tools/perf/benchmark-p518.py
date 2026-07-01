#!/usr/bin/env python3
"""Benchmark P518 — DEBT-42 revalidado com produção real.

Compara Typst vanilla 0.15.0 vs cristalino (release) no corpus P490+P500
mais documentos macro. Coleta timings internos do cristalino via
--timings-json e gera tabela de resultados.
"""

import json
import os
import subprocess
import sys
from pathlib import Path
from statistics import median, mean

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p518"
VANILLA = ROOT / "lab" / "typst-original" / "target" / "release" / "typst"
CRISTALINO = ROOT / "target" / "release" / "typst"
WARMUP = 2
RUNS = 10
WARMUP_MACRO = 0
RUNS_MACRO = 3


def vanilla_compiles(doc):
    """Verifica se o vanilla 0.15.0 consegue compilar o documento."""
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


def run_hyperfine(name, cmd_a, cmd_b, label_a, label_b, warmup, runs):
    """Corre hyperfine e devolve dicionário {label: mean_ms}."""
    out = RESULTS_DIR / f"{name}.json"
    subprocess.run(
        [
            "hyperfine",
            "--warmup", str(warmup),
            "--runs", str(runs),
            "--export-json", str(out),
            "--command-name", label_a,
            "--command-name", label_b,
            cmd_a,
            cmd_b,
        ],
        check=True,
        capture_output=True,
    )
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def run_timings(doc, output_pdf, timings_json):
    """Compila com cristalino e devolve o dict Timings."""
    subprocess.run(
        [str(CRISTALINO), str(doc), str(output_pdf), "--timings-json", str(timings_json)],
        check=True,
        capture_output=True,
    )
    return json.loads(timings_json.read_text())


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    all_micro = sorted(
        list((ROOT / "lab" / "parity" / "corpus" / "p490").glob("*.typ"))
        + list((ROOT / "lab" / "parity" / "corpus" / "p500").glob("*.typ"))
    )
    corpus = {
        # Apenas documentos que compilem no vanilla 0.15.0 — senão o
        # benchmark de tempo perde o sentido de comparação.
        "micro": [d for d in all_micro if vanilla_compiles(d)],
        "macro": [ROOT / "tools" / "perf" / "corpus" / "macro-10x.typ"],
        "subset": [
            ROOT / "tools" / "perf" / "corpus" / "subset-dejavu.typ",
            ROOT / "tools" / "perf" / "corpus" / "subset-lato.typ",
        ],
    }

    rows = []
    timings_rows = []

    for category, docs in corpus.items():
        for doc in docs:
            name = f"{category}-{doc.stem}"
            vanilla_pdf = RESULTS_DIR / f"{name}-vanilla.pdf"
            cristalino_pdf = RESULTS_DIR / f"{name}-cristalino.pdf"
            timings_json = RESULTS_DIR / f"{name}-timings.json"

            print(f"[P518] {name}", flush=True)

            cmd_vanilla = f"{VANILLA} compile {doc} {vanilla_pdf}"
            cmd_cristalino = f"{CRISTALINO} {doc} {cristalino_pdf}"

            warmup, runs = (WARMUP_MACRO, RUNS_MACRO) if category == "macro" else (WARMUP, RUNS)
            hf = run_hyperfine(
                name, cmd_vanilla, cmd_cristalino, "vanilla", "cristalino",
                warmup, runs,
            )
            timings = run_timings(doc, cristalino_pdf, timings_json)

            vanilla_ms = hf["vanilla"]
            cristalino_ms = hf["cristalino"]
            ratio = cristalino_ms / vanilla_ms if vanilla_ms > 0 else float("inf")

            vanilla_size = vanilla_pdf.stat().st_size if vanilla_pdf.exists() else 0
            cristalino_size = cristalino_pdf.stat().st_size if cristalino_pdf.exists() else 0
            size_ratio = cristalino_size / vanilla_size if vanilla_size > 0 else float("inf")

            rows.append({
                "doc": name,
                "category": category,
                "vanilla_ms": vanilla_ms,
                "cristalino_ms": cristalino_ms,
                "ratio": ratio,
                "vanilla_bytes": vanilla_size,
                "cristalino_bytes": cristalino_size,
                "size_ratio": size_ratio,
            })
            timings["doc"] = name
            timings["category"] = category
            timings_rows.append(timings)

    # Estatísticas globais.
    ratios = [r["ratio"] for r in rows]
    size_ratios = [r["size_ratio"] for r in rows]
    stats = {
        "ratio_mean": mean(ratios),
        "ratio_median": median(ratios),
        "ratio_min": min(ratios),
        "ratio_max": max(ratios),
        "size_ratio_mean": mean(size_ratios),
        "size_ratio_median": median(size_ratios),
    }

    summary = {"stats": stats, "rows": rows, "timings": timings_rows}
    summary_path = RESULTS_DIR / "summary.json"
    summary_path.write_text(json.dumps(summary, indent=2))

    # Tabela legível.
    print("\n=== Resultados P518 ===")
    print(f"{'doc':<40} {'vanilla_ms':>12} {'cristalino_ms':>14} {'ratio':>8} {'size_ratio':>11}")
    for r in rows:
        print(
            f"{r['doc']:<40} "
            f"{r['vanilla_ms']:>12.2f} "
            f"{r['cristalino_ms']:>14.2f} "
            f"{r['ratio']:>8.2f} "
            f"{r['size_ratio']:>11.2f}"
        )

    print(f"\nRatio médio:   {stats['ratio_mean']:.2f}x")
    print(f"Ratio mediana: {stats['ratio_median']:.2f}x")
    print(f"Ratio min/max: {stats['ratio_min']:.2f}x / {stats['ratio_max']:.2f}x")
    print(f"Size ratio médio: {stats['size_ratio_mean']:.2f}x")
    print(f"\nSumário guardado em: {summary_path}")


if __name__ == "__main__":
    main()
