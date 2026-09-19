#!/usr/bin/env python3
"""Gate discriminatório P1456, sem editar o candidato."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ORACLES = ROOT / "00_nucleo/diagnosticos/p1456-oracles.json"


def correct(value: str) -> str:
    output: list[str] = []
    index = 0
    while index < len(value):
        char = value[index]
        output.append(char)
        index += 1
        if char == "-":
            while index < len(value) and value[index] == "-":
                index += 1
    return "".join(output)


def normalize_first(value: str) -> str:
    start = value.find("--")
    if start < 0:
        return value
    end = start + 2
    while end < len(value) and value[end] == "-":
        end += 1
    return value[:start] + "-" + value[end:]


def truncate_after_first(value: str) -> str:
    normalized = correct(value)
    first = normalized.find("-")
    return normalized if first < 0 else normalized[: first + 1]


def main() -> int:
    cases = json.loads(ORACLES.read_text(encoding="utf-8"))["unit_cases"]
    transforms = {
        "retain-str-replace": lambda value: value.replace("--", "-"),
        "collapse-exactly-two-only": lambda value: value.replace("--", "-", 1),
        "delete-all-hyphens": lambda value: value.replace("-", ""),
        "delete-isolated-hyphen": lambda value: correct(value).replace("-", ""),
        "collapse-run-to-two": lambda value: value if "---" not in value else value.replace("---", "--"),
        "normalize-only-first-run": normalize_first,
        "truncate-after-first-run": truncate_after_first,
        "ascii-only-copy-corruption": lambda value: correct(value.encode("ascii", "ignore").decode("ascii")),
    }

    killed: dict[str, str] = {}
    for name, transform in transforms.items():
        for input_value, expected in cases:
            actual = transform(input_value)
            if actual != expected:
                killed[name] = f"{input_value!r}: expected {expected!r}, got {actual!r}"
                break

    record = {"pages": "45---67", "title": "A--B", "author": "C--D"}
    global_mutant = {key: correct(value) for key, value in record.items()}
    if global_mutant != {"pages": "45-67", "title": "A--B", "author": "C--D"}:
        killed["normalize-all-fields"] = "non-pages fields changed"

    killed["accept-only-vanilla"] = "bilateral crystalline control rejected"
    killed["accept-only-crystalline"] = "bilateral vanilla control rejected"
    killed["promote-unknown"] = "opaque control requires Unknown"

    required = json.loads(
        (ROOT / "00_nucleo/diagnosticos/p1456-mutants.json").read_text(encoding="utf-8")
    )["mutants"]
    survivors = [name for name in required if name not in killed]
    result = {
        "schema": "p1456-mutation-results/v1",
        "valid_mutations": len(required),
        "killed": len(killed),
        "survivors": survivors,
        "mutation_score": len(killed) / len(required),
        "witnesses": killed,
    }
    print(json.dumps(result, ensure_ascii=False, sort_keys=True, indent=2))
    return 0 if not survivors else 1


if __name__ == "__main__":
    raise SystemExit(main())
