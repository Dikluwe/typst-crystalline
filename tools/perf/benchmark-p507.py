#!/usr/bin/env python3
"""P507 — Benchmark de performance: Cristalino vs Typst vanilla 0.15.0."""

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from statistics import mean, stdev

ROOT = Path(__file__).resolve().parents[2]
RESULTS_DIR = ROOT / "tools" / "perf" / "results"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus"
VANILLA = ROOT / "lab" / "typst-original" / "target" / "release" / "typst"
CRISTALINO = ROOT / "target" / "release" / "typst"
CRISTALINO_DEBUG = ROOT / "target" / "debug" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
if not HYPERFINE.exists():
    HYPERFINE = Path("/usr/local/bin/hyperfine")


def sh(cmd, **kw):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True, **kw)


def ensure_binaries():
    missing = []
    for name, path in [("vanilla", VANILLA), ("cristalino", CRISTALINO)]:
        if not path.exists():
            missing.append(name)
    if missing:
        print(f"Binários em falta: {missing}")
        sys.exit(1)
    if not HYPERFINE.exists():
        print("hyperfine não encontrado; a usar time manual.")


def generate_corpus():
    CORPUS_DIR.mkdir(parents=True, exist_ok=True)
    docs = []

    # Micro: P490 + P500
    for d in [ROOT / "lab" / "parity" / "corpus" / "p490",
              ROOT / "lab" / "parity" / "corpus" / "p500"]:
        for f in sorted(d.glob("*.typ")):
            docs.append((f.stem, f, "micro"))

    # Médio: concatenação de benchmarks existentes
    medium_files = [
        ROOT / "benches" / "corpus" / "b2_text.typ",
        ROOT / "benches" / "corpus" / "b3_math.typ",
        ROOT / "benches" / "corpus" / "b4_code.typ",
        ROOT / "benches" / "corpus" / "b5_utf8.typ",
    ]
    medium_path = CORPUS_DIR / "medium-combined.typ"
    with open(medium_path, "w") as out:
        for f in medium_files:
            if f.exists():
                out.write(f"\n// === {f.name} ===\n")
                out.write(open(f).read())
    docs.append(("medium-combined", medium_path, "medium"))

    # 0.15.0.typ como médio/grande
    v0150 = ROOT / "00_nucleo" / "0.15.0.typ"
    if v0150.exists():
        docs.append(("0.15.0-spec", v0150, "medium"))

    # Macro: 10x o corpus grande
    big = ROOT / "00_nucleo" / "diagnosticos" / "medicao-pre-f-passo-318-corpus.typ"
    if big.exists():
        macro_path = CORPUS_DIR / "macro-10x.typ"
        content = open(big).read()
        with open(macro_path, "w") as out:
            for i in range(10):
                out.write(f"\n// === repeat {i} ===\n")
                out.write(content)
        docs.append(("macro-10x", macro_path, "macro"))

    return docs


def hyperfine_compare(name, doc_path, output_pdf, runs=5):
    """Compara vanilla vs cristalino release com hyperfine."""
    json_path = RESULTS_DIR / f"hyperfine-{name}.json"
    cmd = (
        f'"{HYPERFINE}" --warmup 1 --runs {runs} '
        f'--export-json "{json_path}" '
        f'"{VANILLA} compile \"{doc_path}\" \"{output_pdf}\"" '
        f'"{CRISTALINO} \"{doc_path}\" \"{output_pdf}\""'
    )
    r = sh(cmd)
    if r.returncode != 0:
        print(f"hyperfine falhou para {name}: {r.stderr}")
        return None
    data = json.load(open(json_path))
    results = {}
    for res in data["results"]:
        label = "vanilla" if "lab/typst-original" in res["command"] else "cristalino"
        results[label] = {
            "mean_ms": res["mean"] * 1000,
            "stddev_ms": res["stddev"] * 1000,
            "min_ms": res["min"] * 1000,
            "max_ms": res["max"] * 1000,
        }
    if "vanilla" in results and "cristalino" in results:
        results["ratio"] = results["cristalino"]["mean_ms"] / results["vanilla"]["mean_ms"]
    return results


def time_with_time(cmd, runs=5):
    """Fallback quando hyperfine não está disponível."""
    times = []
    for _ in range(runs):
        r = sh(f'/usr/bin/time -f "%e" {cmd} 2>&1 >/dev/null')
        try:
            times.append(float(r.stderr.strip().split("\n")[-1]))
        except Exception:
            pass
    if not times:
        return None
    return {
        "mean_ms": mean(times) * 1000,
        "stddev_ms": (stdev(times) if len(times) > 1 else 0.0) * 1000,
        "min_ms": min(times) * 1000,
        "max_ms": max(times) * 1000,
    }


def manual_compare(name, doc_path, output_pdf, runs=5):
    vanilla_res = time_with_time(f'"{VANILLA}" compile "{doc_path}" "{output_pdf}"', runs)
    crist_res = time_with_time(f'"{CRISTALINO}" "{doc_path}" "{output_pdf}"', runs)
    if not vanilla_res or not crist_res:
        return None
    return {
        "vanilla": vanilla_res,
        "cristalino": crist_res,
        "ratio": crist_res["mean_ms"] / vanilla_res["mean_ms"],
    }


def cristalino_phases(name, doc_path, output_pdf, runs=5):
    """Mede fases internas do cristalino com --timings-json."""
    timings_list = []
    for _ in range(runs):
        timings_file = tempfile.NamedTemporaryFile(suffix=".json", delete=False)
        timings_file.close()
        try:
            cmd = f'"{CRISTALINO}" "{doc_path}" "{output_pdf}" --timings-json "{timings_file.name}"'
            r = sh(cmd)
            if r.returncode != 0:
                continue
            data = json.load(open(timings_file.name))
            timings_list.append(data)
        finally:
            os.unlink(timings_file.name)
    if not timings_list:
        return None
    keys = ["eval_ms", "introspect_ms", "expand_context_ms", "layout_ms", "render_ms", "total_ms"]
    agg = {}
    for k in keys:
        vals = [t[k] for t in timings_list]
        agg[k] = {"mean": mean(vals), "stddev": stdev(vals) if len(vals) > 1 else 0.0}
    return agg


def main():
    ensure_binaries()
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    docs = generate_corpus()
    summary = []

    use_hyperfine = HYPERFINE.exists()
    print(f"Corpus: {len(docs)} documentos. Hyperfine: {use_hyperfine}")

    for name, path, category in docs:
        print(f"\n=== {name} ({category}) ===")
        output_pdf = tempfile.mktemp(suffix=".pdf")
        # **P594** — documentos macro (ex: macro-10x) demoram dezenas de
        # segundos no cristalino. Usar menos runs evita que o benchmark
        # exceda o tempo limite por lentidão genuína, não por ciclo.
        runs = 2 if category == "macro" else 5
        try:
            if use_hyperfine:
                comparison = hyperfine_compare(name, path, output_pdf, runs=runs)
            else:
                comparison = manual_compare(name, path, output_pdf, runs=runs)
            phases = cristalino_phases(name, path, output_pdf, runs=runs)

            if comparison is None:
                print(f"  [skip] comparação falhou")
                continue

            ratio = comparison.get("ratio", float('inf'))
            print(f"  vanilla mean: {comparison['vanilla']['mean_ms']:.2f} ms")
            print(f"  cristalino mean: {comparison['cristalino']['mean_ms']:.2f} ms")
            print(f"  ratio: {ratio:.2f}x")
            if phases:
                print(f"  eval: {phases['eval_ms']['mean']:.2f} ms | layout: {phases['layout_ms']['mean']:.2f} ms | render: {phases['render_ms']['mean']:.2f} ms")

            summary.append({
                "name": name,
                "category": category,
                "path": str(path),
                "comparison": comparison,
                "phases": phases,
            })
        finally:
            if os.path.exists(output_pdf):
                os.unlink(output_pdf)

    json_out = RESULTS_DIR / "benchmark-p507-summary.json"
    with open(json_out, "w") as f:
        json.dump(summary, f, indent=2)
    print(f"\nResultados: {json_out}")


if __name__ == "__main__":
    main()
