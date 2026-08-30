#!/usr/bin/env python3
"""Inventário reproduzível dos warnings que bloqueiam o gate P1250."""

import argparse
import csv
import hashlib
import json
import re
import subprocess
from collections import Counter
from pathlib import Path


WARNING = re.compile(
    r"warning:(.*?)\[(V\d+)\]\s*\n\s*--> \./([^:\n]+):(\d+)", re.S
)

FIELDS = [
    "warning_id", "code", "file", "line", "message", "owner_l0",
    "classification", "lot", "status",
]


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def owner(root: Path, relative: str) -> str:
    path = root / relative
    if not path.exists():
        return "UNKNOWN"
    head = "\n".join(path.read_text(errors="replace").splitlines()[:30])
    match = re.search(r"@prompt\s+([^\s]+)", head)
    return match.group(1) if match else "UNKNOWN"


def classify(code: str, relative: str) -> tuple[str, str]:
    if code == "V21":
        return "mecanico", "L1-V21-proveniencia"
    if code == "V18":
        return "incerto", "L2-V18-ranges"
    if code == "V17":
        return "semantico", "L3-V17-guards"
    if code == "V16" and relative.endswith("tests.rs"):
        return "mecanico", "L4-V16-testes"
    if code == "V16":
        if relative.startswith("02_shell/"):
            suffix = "shell"
        elif relative.startswith("03_infra/"):
            suffix = "infra"
        elif "/eval/" in relative or "/introspect" in relative:
            suffix = "eval-introspect"
        elif "/stdlib/" in relative:
            suffix = "stdlib"
        elif "/entities/" in relative:
            suffix = "entities"
        elif "/layout/" in relative or "/math/" in relative:
            suffix = "layout-math"
        else:
            suffix = "residual"
        return "incerto", f"L5-V16-{suffix}"
    raise AssertionError(f"warning fora do plano: {code} {relative}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--measured-at", required=True)
    parser.add_argument("--working-tree", required=True)
    parser.add_argument("--baseline", type=Path)
    args = parser.parse_args()
    root = args.root.resolve()
    proc = subprocess.run(
        ["crystalline-lint", "."], cwd=root, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, check=False,
    )
    raw = proc.stdout.encode()
    rows = []
    for message, code, relative, line in WARNING.findall(proc.stdout):
        classification, lot = classify(code, relative)
        rows.append({
            "warning_id": "",
            "code": code,
            "file": relative,
            "line": int(line),
            "message": " ".join(message.split()),
            "owner_l0": owner(root, relative),
            "classification": classification,
            "lot": lot,
            "status": "OPEN",
        })
    rows.sort(key=lambda row: (row["code"], row["file"], row["line"], row["message"]))
    for index, row in enumerate(rows, 1):
        row["warning_id"] = f"W{index:03d}"
    counts = {}
    for row in rows:
        counts[row["code"]] = counts.get(row["code"], 0) + 1
    expected = {"V16": 202, "V17": 37, "V18": 2, "V21": 15}
    verdict = "PASS_BASELINE" if counts == expected and len(rows) == 256 else "FAIL_BASELINE"
    baseline_delta = None
    if args.baseline:
        with args.baseline.open(newline="") as source:
            old = list(csv.DictReader(source, delimiter="\t"))
        old_keys = Counter(
            (r["code"], r["file"], r["message"])
            for r in old if r["status"] == "OPEN"
        )
        new_keys = Counter((r["code"], r["file"], r["message"]) for r in rows)
        baseline_delta = {
            "resolved": sum((old_keys - new_keys).values()),
            "new": sum((new_keys - old_keys).values()),
            "remaining": sum(new_keys.values()),
        }
        verdict = "PASS_DELTA" if baseline_delta["new"] == 0 else "FAIL_DELTA"
    args.out.mkdir(parents=True, exist_ok=True)
    inventory = args.out / "inventory.tsv"
    with inventory.open("w", newline="") as target:
        writer = csv.DictWriter(target, fieldnames=FIELDS, delimiter="\t")
        writer.writeheader()
        writer.writerows(rows)
    summary = {
        "verdict": verdict,
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "lint_exit": proc.returncode,
        "raw_sha256": sha(raw),
        "inventory_sha256": sha(inventory.read_bytes()),
        "warning_count": len(rows),
        "counts_by_code": counts,
        "unknown_owners": sum(row["owner_l0"] == "UNKNOWN" for row in rows),
        "baseline_delta": baseline_delta,
    }
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    if verdict not in {"PASS_BASELINE", "PASS_DELTA"}:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
