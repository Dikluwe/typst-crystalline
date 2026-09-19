#!/usr/bin/env python3
"""Gate discriminatório P1457, sem editar o candidato."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ORACLES = ROOT / "00_nucleo/diagnosticos/p1457-oracles.json"
MUTANTS = ROOT / "00_nucleo/diagnosticos/p1457-mutants.json"


def correct(value: str) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def current_classifier(value: str) -> str:
    first = value[:1]
    quoted = (
        first.isdigit()
        or first in "-:[{*&!|>'\"%@`"
        or ":" in value
        or "#" in value
        or "\n" in value
        or "\t" in value
        or value in {"true", "false", "null", "~"}
    )
    if not quoted:
        return value
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def lowercase_catalog(value: str) -> str:
    if value in {"true", "false", "null", "~"}:
        return correct(value)
    return value


def main() -> int:
    oracle = json.loads(ORACLES.read_text(encoding="utf-8"))
    cases = oracle["unit_cases"]
    transforms = {
        "retain-current-classifier": current_classifier,
        "extend-lowercase-catalog": lowercase_catalog,
        "leave-plain-unquoted": lambda value: value,
        "quote-without-escaping": lambda value: f'"{value}"',
        "escape-quotes-only": lambda value: '"' + value.replace('"', '\\"') + '"',
        "escape-backslashes-only": lambda value: '"' + value.replace("\\", "\\\\") + '"',
        "leave-controls-raw": lambda value: '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"',
        "ascii-only-copy": lambda value: correct(value.encode("ascii", "ignore").decode("ascii")),
        "coerce-typed-spellings": lambda value: correct(value.lower() if value in {"True", "NULL"} else value),
    }

    killed: dict[str, str] = {}
    for name, transform in transforms.items():
        for input_value, expected in cases:
            actual = transform(input_value)
            if actual != expected:
                killed[name] = f"{input_value!r}: expected {expected!r}, got {actual!r}"
                break

    fields = {field: value for field, value in oracle["field_cases"]}
    title_only = {
        field: correct(value) if field == "title" else value
        for field, value in fields.items()
    }
    if title_only != {field: correct(value) for field, value in fields.items()}:
        killed["quote-title-only"] = "author, publisher and note remain unquoted"

    killed["accept-one-binary"] = "public oracle requires success in both identified binaries"
    killed["promote-unknown"] = "opaque controls must remain Unknown"

    required = json.loads(MUTANTS.read_text(encoding="utf-8"))["mutants"]
    survivors = [name for name in required if name not in killed]
    result = {
        "schema": "p1457-mutation-results/v1",
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
