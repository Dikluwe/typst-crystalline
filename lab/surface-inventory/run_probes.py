#!/usr/bin/env python3
"""Executa sondas bilaterais P1282 com features e identidades simétricas."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import pathlib
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_PROBES = pathlib.Path(__file__).with_name("probes.json")


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def sampled_inventory_probes(path: pathlib.Path) -> list[dict]:
    payload = json.loads(path.read_text())
    by_class: dict[str, list[dict]] = {}
    for entry in payload["entries"]:
        by_class.setdefault(entry["classification"], []).append(entry)
    selected = []
    for classification, entries in sorted(by_class.items()):
        count = min(len(entries), max(10, math.ceil(math.sqrt(len(entries)))))
        ranked = sorted(
            entries,
            key=lambda entry: hashlib.sha256(
                (
                    "C-P1282-v1"
                    + payload["profile"]
                    + classification
                    + ".".join(entry["path_segments"])
                    + str(entry.get("access_form"))
                ).encode()
            ).hexdigest(),
        )
        for entry in ranked[:count]:
            display = entry["display_path"]
            selected.append(
                {
                    "id": f"sample-{classification.lower()}-{display}",
                    "path": display,
                    "classification": classification,
                }
            )
    return selected


def command(binary: pathlib.Path, expression: str, profile: str) -> list[str]:
    result = [str(binary), "eval", expression, "--format", "json"]
    if profile == "html":
        result.extend(["--features", "html"])
    return result


def run_side(binary: pathlib.Path, expression: str, profile: str) -> dict:
    argv = command(binary, expression, profile)
    run = subprocess.run(
        argv,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return {
        "command": argv,
        "exit_code": run.returncode,
        "stdout": run.stdout,
        "stderr": run.stderr,
    }


def receipts_match(vanilla: dict, crystalline: dict) -> bool:
    return (
        vanilla["exit_code"] == 0
        and crystalline["exit_code"] == 0
        and vanilla["stdout"] == crystalline["stdout"]
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", choices=("default", "html"), required=True)
    parser.add_argument("--vanilla-bin", type=pathlib.Path, required=True)
    parser.add_argument("--crystalline-bin", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    parser.add_argument("--probes", type=pathlib.Path, default=DEFAULT_PROBES)
    parser.add_argument("--inventory", type=pathlib.Path)
    args = parser.parse_args()

    probes = json.loads(args.probes.read_text())
    if args.inventory:
        probes.extend(sampled_inventory_probes(args.inventory))
    unique = {}
    for probe in probes:
        profiles = probe.get("profiles", ["default", "html"])
        if args.profile in profiles:
            unique[probe["id"]] = probe

    results = []
    for probe_id, probe in sorted(unique.items()):
        expression = probe.get("expression")
        if expression is None:
            expression = f'repr((type({probe["path"]}), repr({probe["path"]})))'
        vanilla = run_side(args.vanilla_bin, expression, args.profile)
        crystalline = run_side(args.crystalline_bin, expression, args.profile)
        same = receipts_match(vanilla, crystalline)
        results.append(
            {
                **probe,
                "id": probe_id,
                "profile": args.profile,
                "features": [] if args.profile == "default" else ["html"],
                "expression": expression,
                "vanilla": vanilla,
                "crystalline": crystalline,
                "same": same,
                "verdict": "MATCH" if same else "DIFFERENCE_OR_DISABLED",
            }
        )

    payload = {
        "schema_version": "p1282-probes-v1",
        "profile": args.profile,
        "features": [] if args.profile == "default" else ["html"],
        "cwd": str(ROOT),
        "binaries": {
            "vanilla": {
                "path": str(args.vanilla_bin),
                "sha256": sha256(args.vanilla_bin),
            },
            "crystalline": {
                "path": str(args.crystalline_bin),
                "sha256": sha256(args.crystalline_bin),
            },
        },
        "counts": {
            "total": len(results),
            "same": sum(result["same"] for result in results),
            "different_or_disabled": sum(not result["same"] for result in results),
        },
        "results": results,
    }
    args.output.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    )
    print(json.dumps(payload["counts"], sort_keys=True))


if __name__ == "__main__":
    main()
