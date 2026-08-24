#!/usr/bin/env python3
"""Combina os catálogos runtime do P1140 sem inferir assinaturas ausentes."""

import argparse
import datetime as dt
import json
import pathlib
import re
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[2]
UPSTREAM = ROOT / "lab/typst-original"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def resolve_source(raw: str | None) -> str:
    if not raw:
        return "lab/typst-original/crates/typst-library/src/lib.rs:357"
    path, key = raw.split(":", 1)
    file = UPSTREAM / path
    if not file.is_file():
        return f"lab/typst-original/{path}:1"
    needle = key.rsplit("::", 1)[-1]
    patterns = [
        re.compile(rf"\b(fn|struct|enum|type)\s+{re.escape(needle)}\b"),
        re.compile(rf"\b{re.escape(needle)}\b"),
    ]
    lines = file.read_text(encoding="utf-8").splitlines()
    for pattern in patterns:
        for number, line in enumerate(lines, 1):
            if pattern.search(line):
                return f"lab/typst-original/{path}:{number}"
    return f"lab/typst-original/{path}:1"


def absent_side() -> dict:
    return {"present": False, "kind": None, "params": None, "source": None}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("vanilla", type=pathlib.Path)
    parser.add_argument("crystalline", type=pathlib.Path)
    parser.add_argument("output", type=pathlib.Path)
    parser.add_argument("--generated-at")
    args = parser.parse_args()
    vanilla = json.loads(args.vanilla.read_text())
    crystalline = json.loads(args.crystalline.read_text())
    entries = []
    for path in sorted(set(vanilla) | set(crystalline)):
        v = vanilla.get(path, absent_side())
        c = crystalline.get(path, absent_side())
        if v["present"]:
            v["source"] = resolve_source(v.get("source"))
        if not v["present"]:
            classification = "EXTRA_BINDING"
        elif not c["present"]:
            classification = "MISSING_MEMBER" if "." in path else "MISSING_BINDING"
        elif v["kind"] != c["kind"]:
            classification = "WRONG_KIND"
        elif v.get("params") is not None and c.get("params") is None:
            classification = "UNVERIFIED_METADATA"
        else:
            classification = "MATCH"
        probe = None
        if path in crystalline and crystalline[path]["source"].startswith("runtime:probe:"):
            probe = path
        elif path in vanilla and path not in crystalline:
            probe = path
        entries.append({
            "path": path,
            "vanilla": v,
            "crystalline": c,
            "probe": probe,
            "classification": classification,
            "inference": None,
            "refutation": None,
        })
    counts = {}
    for entry in entries:
        key = entry["classification"]
        counts[key] = counts.get(key, 0) + 1
    payload = {
        "generated_at": args.generated_at or dt.datetime.now(dt.timezone.utc).isoformat(),
        "vanilla_revision": "upstream/main a51e02804",
        "crystalline_head": git("rev-parse", "HEAD"),
        "working_tree": "não commitada",
        "counts": dict(sorted(counts.items())),
        "entries": entries,
    }
    args.output.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(payload["counts"], ensure_ascii=False, sort_keys=True))


if __name__ == "__main__":
    main()
