#!/usr/bin/env python3
"""Benchmark P959 — shifts de limites de operadores grandes (4 termos MATH) + extents de tinta da variante de Display.

"antes" = binário release do estado pós-P957 (pré-P959), copiado para
temp/p959/typst-antes antes do rebuild. "depois" = release actual.
7 cenários canónicos, output para /dev/null. Estado da árvore: working tree
com as alterações de P959 (attach.rs + math/layout/mod.rs + math_constants.rs + font_metrics.rs) sobre `8a71a39a8`
(ver relatório P959).
"""

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p959-canonical"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
ANTES_BIN = ROOT / "temp" / "p959" / "typst-antes"
CURRENT_BIN = ROOT / "target" / "release" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 5
MIN_RUNS = 20


def run(cmd, **kwargs):
    print(f"[p959] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path):
    out = RESULTS_DIR / f"{name}.json"
    run([
        str(HYPERFINE),
        "--warmup", str(WARMUP),
        "--min-runs", str(MIN_RUNS),
        "--export-json", str(out),
        "--command-name", "antes",
        "--command-name", "p959",
        f"{ANTES_BIN} {doc} /dev/null",
        f"{CURRENT_BIN} {doc} /dev/null",
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
        means = run_hyperfine(name, doc)
        ratio = means["p959"] / means["antes"]
        rows.append((name, means["antes"], means["p959"], ratio))
        print(f"{name}: antes={means['antes']:.2f}ms depois={means['p959']:.2f}ms ratio={ratio:.3f}")
    avg = sum(r[3] for r in rows) / len(rows)
    print(f"\nratio médio: {avg:.3f}")


if __name__ == "__main__":
    main()
