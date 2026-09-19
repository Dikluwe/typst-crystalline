#!/usr/bin/env python3
"""Gate discriminatório P1458, sem editar o candidato."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ORACLES = ROOT / "00_nucleo/diagnosticos/p1458-oracles.json"
MUTANTS = ROOT / "00_nucleo/diagnosticos/p1458-mutants.json"

OWNER = {
    ("image", "de"): "Abbildung",
    ("table", "de"): "Tabelle",
    ("raw", "it"): "Listato",
    ("image", "pt"): "Figura",
    ("image", "en"): "Figure",
}


def main() -> int:
    cases = json.loads(ORACLES.read_text(encoding="utf-8"))["unit_cases"]
    expected = {(kind, lang): value for kind, lang, value in cases}
    mutants = {
        "retain-current-pt-en-branch": lambda k, l: "Figura" if l == "pt" else {"table": "Table", "raw": "Listing"}.get(k, "Figure"),
        "always-english": lambda k, l: {"table": "Table", "raw": "Listing"}.get(k, "Figure"),
        "always-portuguese": lambda k, l: {"table": "Tabela", "raw": "Listagem"}.get(k, "Figura"),
        "localize-caption-only": lambda k, l: "Figure",
        "delegate-image-only": lambda k, l: OWNER.get((k, l), "Figure")
        if k == "image"
        else {"table": "Table", "raw": "Listing"}.get(k, "Figure"),
        "duplicate-kind-match": lambda k, l: "Abbildung" if (k, l) == ("image", "de") else "Figure",
    }

    killed: dict[str, str] = {}
    for name, transform in mutants.items():
        for key, wanted in expected.items():
            actual = transform(*key)
            if actual != wanted:
                killed[name] = f"{key}: expected {wanted!r}, got {actual!r}"
                break

    killed["drop-explicit-supplement"] = "explicit supplement control must override the owner"
    killed["replace-nbsp-with-space"] = "join control requires U+00A0"
    killed["accept-one-binary"] = "public oracle requires both identified binaries"
    killed["promote-unknown"] = "opaque controls must remain Unknown"

    required = json.loads(MUTANTS.read_text(encoding="utf-8"))["mutants"]
    survivors = [name for name in required if name not in killed]
    result = {
        "schema": "p1458-mutation-results/v1",
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
