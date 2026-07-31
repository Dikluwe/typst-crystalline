#!/usr/bin/env python3
"""Strace P938 — contagem de aberturas de ficheiros de fonte."""

import json
import re
import subprocess
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
RESULTS_DIR = ROOT / "tools" / "perf" / "results" / "p938" / "strace"
CORPUS_DIR = ROOT / "tools" / "perf" / "corpus" / "p923"

BINS = {
    "p938": ROOT / "target" / "release" / "typst-p938",
    "vanilla": ROOT / "lab" / "typst-original" / "target" / "release" / "typst",
}

FONT_RE = re.compile(r"\.(ttf|otf|ttc|otc)$", re.IGNORECASE)
OPENAT_RE = re.compile(
    r"^(\[[^\]]+\]\s+)?(?P<time>\d{2}:\d{2}:\d{2}\.\d+)\s+openat\(AT_FDCWD,\s*\"(?P<path>[^\"]+)\".*"
)


def run_strace(label: str, doc: Path, bin_path: Path, is_vanilla: bool):
    log = RESULTS_DIR / f"{label}.log"
    log.parent.mkdir(parents=True, exist_ok=True)

    if is_vanilla:
        cmd = [str(bin_path), "compile", str(doc), "/dev/null", "-f", "pdf"]
    else:
        cmd = [str(bin_path), str(doc), "/dev/null"]

    with open(log, "w") as f:
        subprocess.run(
            ["strace", "-f", "-tt", "-e", "trace=openat", *cmd],
            stderr=f,
            stdout=subprocess.DEVNULL,
            check=False,
        )
    return log


def parse_log(log: Path):
    total = 0
    unique = set()
    times = []
    with open(log) as f:
        for line in f:
            m = OPENAT_RE.match(line.strip())
            if not m:
                continue
            path = m.group("path")
            if not FONT_RE.search(path):
                continue
            total += 1
            unique.add(path)
            t = datetime.strptime(m.group("time"), "%H:%M:%S.%f")
            times.append(t)
    if not times:
        return {"total": 0, "unique": 0, "first": None, "last": None, "duration_s": None}
    first = times[0]
    last = times[-1]
    delta = (last - first).total_seconds()
    return {
        "total": total,
        "unique": len(unique),
        "first": first.strftime("%H:%M:%S.%f")[:-3],
        "last": last.strftime("%H:%M:%S.%f")[:-3],
        "duration_s": round(delta, 3),
    }


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    runs = [
        ("p938-01-hello", CORPUS_DIR / "01-hello.typ", BINS["p938"], False),
        ("vanilla-01-hello", CORPUS_DIR / "01-hello.typ", BINS["vanilla"], True),
        ("p938-05-utf8", CORPUS_DIR / "05-utf8.typ", BINS["p938"], False),
        ("vanilla-05-utf8", CORPUS_DIR / "05-utf8.typ", BINS["vanilla"], True),
        ("p938-utf8-cjk", CORPUS_DIR / "utf8-cjk.typ", BINS["p938"], False),
        ("vanilla-utf8-cjk", CORPUS_DIR / "utf8-cjk.typ", BINS["vanilla"], True),
        ("p938-utf8-emoji", CORPUS_DIR / "utf8-emoji.typ", BINS["p938"], False),
        ("vanilla-utf8-emoji", CORPUS_DIR / "utf8-emoji.typ", BINS["vanilla"], True),
    ]

    rows = []
    for label, doc, bin_path, is_vanilla in runs:
        print(f"[strace] {label}")
        log = RESULTS_DIR / f"{label}.log"
        if log.exists():
            print(f"[strace] reutilizando log existente: {log}")
        else:
            log = run_strace(label, doc, bin_path, is_vanilla)
        stats = parse_log(log)
        rows.append({"label": label, "doc": str(doc.name), "binary": bin_path.name, **stats})
        print(f"[strace] {label}: {stats}")

    summary_path = RESULTS_DIR / "summary.json"
    summary_path.write_text(json.dumps(rows, indent=2))
    print(f"\n[strace] Summary escrito em {summary_path}")


if __name__ == "__main__":
    main()
