#!/usr/bin/env python3
"""Benchmark P972 — conteúdo de função de utilizador em math recebe default (braços Sequence/Styled).

"antes" = binário release do estado pós-P957 (pré-P972), copiado para
temp/p972/typst-antes antes do rebuild. "depois" = release actual.
7 cenários canónicos, output para /dev/null. Estado da árvore: working tree
com as alterações de P972 (math/layout/mod.rs) sobre `552fcbdeb`
(ver relatório P972).
"""

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p972-canonical"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
ANTES_BIN = ROOT / "temp" / "p972" / "typst-antes"
CURRENT_BIN = ROOT / "target" / "release" / "typst"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 5
MIN_RUNS = 20


def run(cmd, **kwargs):
    print(f"[p972] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def run_hyperfine(name: str, doc: Path):
    out = RESULTS_DIR / f"{name}.json"
    run([
        str(HYPERFINE),
        "--warmup", str(WARMUP),
        "--min-runs", str(MIN_RUNS),
        "--export-json", str(out),
        "--command-name", "antes",
        "--command-name", "p972",
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
        ratio = means["p972"] / means["antes"]
        rows.append((name, means["antes"], means["p972"], ratio))
        print(f"{name}: antes={means['antes']:.2f}ms depois={means['p972']:.2f}ms ratio={ratio:.3f}")
    avg = sum(r[3] for r in rows) / len(rows)
    print(f"\nratio médio: {avg:.3f}")


if __name__ == "__main__":
    main()
