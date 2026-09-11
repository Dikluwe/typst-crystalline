#!/usr/bin/env python3
"""Independent replay of the frozen 84 materialized attacks against P1341 R5."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CONTRACT_PATH = DIAG / "p1341-contract-r5.json"
CHECKER_PATH = DIAG / "p1341-contract-r5-checker.py"
FOCAL_PATH = DIAG / "p1341-contract-r5-focal.json"
R1_PATH = DIAG / "p1341-adversary-materialized-mutations-r1.py"
R2_PATH = DIAG / "p1341-adversary-materialized-mutations-r2.py"
R3_PATH = DIAG / "p1341-adversary-materialized-mutations-r3.py"


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def import_module(path: Path, name: str):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def safely_classify(checker, contract, expectation, ledger, r4_contract, r4_checker, r3_contract, r3_checker):
    try:
        classification, reason = checker.classify(
            contract,
            expectation,
            ledger,
            r4_contract,
            r4_checker,
            r3_contract,
            r3_checker,
        )
        return {"classification": classification, "reason_code": reason}
    except Exception as error:
        return {
            "classification": "CheckerError",
            "reason_code": f"{type(error).__name__}:{error}",
        }


def main() -> int:
    contract = load_json(CONTRACT_PATH)
    focal = load_json(FOCAL_PATH)
    checker = import_module(CHECKER_PATH, "p1341_r5_checker_adversary_r4")
    r1 = import_module(R1_PATH, "p1341_r1_frozen_for_r4")
    r2 = import_module(R2_PATH, "p1341_r2_frozen_for_r4")
    r3 = import_module(R3_PATH, "p1341_r3_frozen_for_r4")
    built = checker.build_fixture(ROOT, contract, focal)
    expectation, positive, r4_contract, r4_checker, r3_contract, r3_checker, built_r1, built_r2, built_r3, fixture_errors = built
    if set(built_r1.MUTATIONS) != set(r1.MUTATIONS):
        raise RuntimeError("R1 suite changed across independent loads")
    if set(built_r2.NEW_MUTATIONS) != set(r2.NEW_MUTATIONS):
        raise RuntimeError("R2 suite changed across independent loads")
    if set(built_r3.NEW) != set(r3.NEW):
        raise RuntimeError("R3 suite changed across independent loads")

    r4_focal = load_json(ROOT / focal["base_focal"][0])
    r3_focal = load_json(ROOT / r4_focal["base_focal"][0])
    protected_ids = [
        row["id"]
        for row in load_json(ROOT / r3_focal["base_positive_ledger"][0])["mutations"]
    ]
    r1_ids = list(r1.ADDITIONAL_CLASS)
    r2_ids = list(r2.NEW_MUTATIONS)
    r3_ids = list(r3.NEW)
    all_ids = protected_ids + r1_ids + r2_ids + r3_ids
    materializers = dict(r1.MUTATIONS)
    materializers.update(r2.NEW_MUTATIONS)
    materializers.update(r3.NEW)
    cardinality = {
        "protected": len(protected_ids),
        "r1_additional": len(r1_ids),
        "r2_new": len(r2_ids),
        "r3_boundary": len(r3_ids),
        "total": len(all_ids),
        "unique": len(set(all_ids)),
        "materializers": len(materializers),
    }
    if cardinality != {
        "protected": 41,
        "r1_additional": 10,
        "r2_new": 16,
        "r3_boundary": 17,
        "total": 84,
        "unique": 84,
        "materializers": 84,
    } or set(materializers) != set(all_ids):
        raise RuntimeError({"frozen-corpus-bijection": cardinality})

    opaque = copy.deepcopy(positive)
    opaque["payload_state"] = {
        "kind": "opaque",
        "reason_code": focal["controls"]["opaque_reason_code"],
    }
    controls = {
        "positive": safely_classify(checker, contract, copy.deepcopy(expectation), copy.deepcopy(positive), r4_contract, r4_checker, r3_contract, r3_checker),
        "opaque": safely_classify(checker, contract, copy.deepcopy(expectation), opaque, r4_contract, r4_checker, r3_contract, r3_checker),
    }
    orders = {
        "normal": all_ids,
        "repeat": all_ids,
        "reverse": list(reversed(all_ids)),
    }
    runs = {}
    for order_name, identifiers in orders.items():
        rows = {}
        for identifier in identifiers:
            mutated_expectation = copy.deepcopy(expectation)
            mutated_ledger = copy.deepcopy(positive)
            try:
                materializers[identifier](mutated_expectation, mutated_ledger)
                rows[identifier] = safely_classify(
                    checker,
                    contract,
                    mutated_expectation,
                    mutated_ledger,
                    r4_contract,
                    r4_checker,
                    r3_contract,
                    r3_checker,
                )
            except Exception as error:
                rows[identifier] = {
                    "classification": "MaterializerError",
                    "reason_code": type(error).__name__,
                }
        runs[order_name] = rows
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]

    groups = {
        "protected_41": protected_ids,
        "r1_additional_10": r1_ids,
        "r2_new_16": r2_ids,
        "r3_boundary_17": r3_ids,
        "inherited_67": protected_ids + r1_ids + r2_ids,
        "combined": all_ids,
    }
    summaries = {}
    for name, identifiers in groups.items():
        killed = sum(
            runs["normal"][identifier]["classification"] == "Violated"
            for identifier in identifiers
        )
        survivors = [
            identifier
            for identifier in identifiers
            if runs["normal"][identifier]["classification"] != "Violated"
        ]
        summaries[name] = {
            "valid": len(identifiers),
            "killed": killed,
            "survived": len(survivors),
            "score": killed / len(identifiers),
            "survivors": survivors,
        }
    details = {
        identifier: runs["normal"][identifier]
        for identifier in all_ids
    }
    result = {
        "schema": "p1341-adversary-materialized-result-r4",
        "scope": "R5 frozen 84-case synthetic corpus only; no candidate/product access",
        "inputs": {
            str(path.relative_to(ROOT)): sha256(path)
            for path in (CONTRACT_PATH, CHECKER_PATH, FOCAL_PATH, R1_PATH, R2_PATH, R3_PATH)
        },
        "frozen_corpus": cardinality,
        "fixture_errors": fixture_errors,
        "controls": controls,
        "orders": list(orders),
        "deterministic": deterministic,
        "summaries": summaries,
        "details": details,
        "verdict": (
            "RECOMMEND_SEAL"
            if not fixture_errors
            and deterministic
            and controls["positive"]["classification"] == "Preserved"
            and controls["opaque"]["classification"] == "Unknown"
            and not summaries["combined"]["survivors"]
            and summaries["combined"]["score"] == 1.0
            else "BLOCK_SEAL"
        ),
        "limitations": [
            "The corpus is frozen at exactly 84; this run authored no new attack.",
            "Only the pinned R5 synthetic fragment is evaluated, not product behavior or reachability.",
            "Unknown, CheckerError and MaterializerError count as survivors for negatives.",
            "No candidate/product source or output was read or executed.",
        ],
    }
    print(json.dumps(result, sort_keys=True, indent=2))
    return 0 if result["verdict"] == "RECOMMEND_SEAL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
