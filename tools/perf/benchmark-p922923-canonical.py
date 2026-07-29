#!/usr/bin/env python3
"""Benchmark P922/P923 nos 7 cenários canônicos — razão depois/antes.

Compara o binário cristalino compilado do commit antes de P922/P923
(`c9df6fde`, P921) contra o binário cristalino do commit depois
(`da18ea9f3`, P922+P923+P924 integrados).

Usa `hyperfine --warmup 5 --min-runs 20`, output para /dev/null para
minimizar I/O de disco, e gera attestation com hashes dos binários.

Este benchmark restaura a metodologia canônica da frente (P872-P921):
mesmos 7 cenários, razão depois/antes, para detectar regressões
introduzidas especificamente por P922/P923.
"""

import hashlib
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p922923-canonical"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p922923-canonical"
HYPERFINE = Path.home() / ".cargo" / "bin" / "hyperfine"
WARMUP = 5
MIN_RUNS = 20
BEFORE_COMMIT = "c9df6fde"
AFTER_COMMIT = "da18ea9f3"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def run(cmd, **kwargs):
    print(f"[bench] {' '.join(str(c) for c in cmd)}")
    subprocess.run(cmd, check=True, **kwargs)


def build_at_commit(commit: str, output_name: str) -> Path:
    """Compila o binário cristalino num worktree temporário no commit dado."""
    worktree = ROOT.parent / f"typst-crystalline-{output_name}"
    binary = worktree / "target" / "release" / "typst"

    if binary.exists():
        print(f"[bench] binário já existe: {binary}")
        return binary

    print(f"[bench] a criar worktree {worktree} para {commit}")
    run(["git", "worktree", "add", "-f", str(worktree), commit], cwd=ROOT)
    try:
        print(f"[bench] a compilar release em {worktree}")
        run(["cargo", "build", "--workspace", "--release"], cwd=worktree)
    except Exception:
        run(["git", "worktree", "remove", "-f", str(worktree)], cwd=ROOT)
        raise

    return binary


def cleanup_worktree(output_name: str):
    worktree = ROOT.parent / f"typst-crystalline-{output_name}"
    if worktree.exists():
        print(f"[bench] a remover worktree {worktree}")
        run(["git", "worktree", "remove", "-f", str(worktree)], cwd=ROOT)


def run_hyperfine(name: str, doc: Path, before_bin: Path, after_bin: Path):
    out = RESULTS_DIR / f"{name}.json"
    # Cristalino: `typst <INPUT> [OUTPUT]` (sem subcomando compile).
    cmd_before = f"{before_bin} {doc} /dev/null"
    cmd_after = f"{after_bin} {doc} /dev/null"
    run([
        str(HYPERFINE),
        "--warmup", str(WARMUP),
        "--min-runs", str(MIN_RUNS),
        "--export-json", str(out),
        "--command-name", "before",
        "--command-name", "after",
        cmd_before,
        cmd_after,
    ])
    data = json.loads(out.read_text())
    return {r["command"]: r["mean"] * 1000.0 for r in data["results"]}


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    before_bin = None
    after_bin = None
    try:
        before_bin = build_at_commit(BEFORE_COMMIT, "before")
        after_bin = build_at_commit(AFTER_COMMIT, "after")

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
                print(f"[bench] SKIP {name}: ficheiro não encontrado: {doc}")
                continue
            print(f"[bench] {name}", flush=True)
            hf = run_hyperfine(name, doc, before_bin, after_bin)
            before_ms = hf.get("before", 0.0)
            after_ms = hf.get("after", 0.0)
            ratio = after_ms / before_ms if before_ms > 0 else float("inf")
            rows.append({
                "name": name,
                "before_ms": before_ms,
                "after_ms": after_ms,
                "ratio": ratio,
            })
            print(f"[bench] {name}: before={before_ms:.2f}ms after={after_ms:.2f}ms ratio={ratio:.2f}x")

        attestation = {
            "passo": "P922/P923-canonical",
            "warmup": WARMUP,
            "min_runs": MIN_RUNS,
            "before_commit": BEFORE_COMMIT,
            "after_commit": AFTER_COMMIT,
            "binaries": {
                "before": {"path": str(before_bin), "sha256": sha256_file(before_bin)},
                "after": {"path": str(after_bin), "sha256": sha256_file(after_bin)},
            },
            "hyperfine": rows,
        }
        attestation_path = RESULTS_DIR / "attestation.json"
        attestation_path.write_text(json.dumps(attestation, indent=2))
        print(f"\n[bench] Attestation escrita em {attestation_path}")
        print(json.dumps(rows, indent=2))
    finally:
        cleanup_worktree("before")
        cleanup_worktree("after")


if __name__ == "__main__":
    main()
