#!/usr/bin/env python3
"""Independent focal adversary for the P1342 R3/R4 pre-seal boundary.

This runner deliberately imports the frozen R4 checker and exercises only
Y01-Y06 plus narrowly scoped source-evidence controls.  It never opens a
productive candidate and never invokes the R4 full-corpus entry point.
"""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from collections import defaultdict
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER = DIAG / "p1342-oracle-checker-r4.py"
CONTRACT = DIAG / "p1342-contract-spec-r3.json"
BINDING = DIAG / "p1342-contract-binding-r3.json"
CORPUS = DIAG / "p1342-oracle-corpus-r4.json"
MODES = ["normal", "repeat", "reverse"]


def load_module(path: Path) -> Any:
    spec = importlib.util.spec_from_file_location("p1342_oracle_checker_r4_adversary", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen R4 checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


R4 = load_module(CHECKER)


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def refresh(evidence: dict[str, Any], invocation: dict[str, Any]) -> None:
    R4.refresh_candidate_tree(evidence)
    R4.seal_evidence(evidence)
    invocation["candidate_evidence_sha256"] = R4.digest(evidence)


def replace_candidate_bytes(
    evidence: dict[str, Any], invocation: dict[str, Any], new_bytes: str
) -> None:
    target = evidence["files"][0]
    target["candidate_bytes"] = new_bytes
    target["candidate_file_sha256"] = R4.sha_text(new_bytes)
    for row in evidence["rows"]:
        if row["path"] == target["path"]:
            row["candidate_file_sha256"] = target["candidate_file_sha256"]
    for boundary in evidence["allowed_diff_boundaries"]:
        if boundary["path"] == target["path"]:
            boundary["candidate_byte_range"] = [0, len(new_bytes.encode("utf-8"))]
    refresh(evidence, invocation)


def alias_live_baseline(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, evidence, pins
    source = (
        "def compare_live(live_candidate_sha256, candidate_file_sha256, baseline_file_sha256):\n"
        "    decoy = live_candidate_sha256 == candidate_file_sha256\n"
        "    historical = baseline_file_sha256\n"
        "    return decoy and live_candidate_sha256 == historical\n"
    )
    invocation["checker_fixture"] = {"source": source, "sha256": R4.sha_text(source)}


def baseline_equal_claimed_hooks(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    replace_candidate_bytes(
        evidence, invocation, evidence["baseline_attestation"]["files"][0]["baseline_bytes"]
    )


def candidate_hash_only(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    evidence["files"][0]["candidate_file_sha256"] = "0" * 64
    refresh(evidence, invocation)


def external_path_only(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, evidence, pins
    invocation["candidate_evidence_path"] = "fixture://candidate-controlled/swapped.json"


def external_hash_only(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, evidence, pins
    invocation["candidate_evidence_sha256"] = "0" * 64


def outside_cfg_invalid_rust(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    forged = "fn unguarded_change( {\n" + evidence["files"][0]["candidate_bytes"]
    replace_candidate_bytes(evidence, invocation, forged)


def empty_boundary_over_changed_source(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    target_path = evidence["files"][0]["path"]
    for boundary in evidence["allowed_diff_boundaries"]:
        if boundary["path"] == target_path:
            boundary["candidate_byte_range"] = [0, 0]
    refresh(evidence, invocation)


def preseal_r2_pins(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto
    baseline = evidence["baseline_attestation"]
    baseline["contract_sha256"] = pins["contract_r2"]
    baseline["binding_sha256"] = pins["binding_r2"]
    evidence["preseal"]["sha256"] = R4.digest(baseline)
    refresh(evidence, invocation)


def candidate_bytes_hash_diverge(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    evidence["files"][0]["candidate_bytes"] += "// divergent unpinned bytes\n"
    refresh(evidence, invocation)


def missing_rust_aware_row(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    evidence["rows"][0]["anchor_matches_in_symbol_branch"] = 0
    refresh(evidence, invocation)


def missing_coverage(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    evidence["runtime_coverage"]["row_coverage"].pop()
    refresh(evidence, invocation)


def forged_anchor_hashes(
    dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]
) -> None:
    del dto, pins
    evidence["rows"][0]["candidate_anchor_snippet_sha256"] = "0" * 64
    evidence["rows"][0]["candidate_symbol_sha256"] = "f" * 64
    refresh(evidence, invocation)


Mutator = Callable[[dict[str, Any], dict[str, Any], dict[str, Any], dict[str, str]], None]


EXTRAS: list[tuple[str, str, Mutator]] = [
    ("Z01-live-baseline-via-alias-and-decoy", "Violated", alias_live_baseline),
    ("Z02-baseline-equal-with-fabricated-hooks", "Violated", baseline_equal_claimed_hooks),
    ("Z03-candidate-file-sha-altered", "Violated", candidate_hash_only),
    ("Z04-external-evidence-path-swapped", "Violated", external_path_only),
    ("Z05-external-evidence-hash-swapped", "Violated", external_hash_only),
    ("Z06-outside-cfg-invalid-rust-self-attested", "Violated", outside_cfg_invalid_rust),
    ("Z07-empty-boundary-over-changed-source", "Violated", empty_boundary_over_changed_source),
    ("Z08-preseal-pins-r2", "Violated", preseal_r2_pins),
    ("Z09-candidate-bytes-hash-diverge", "Violated", candidate_bytes_hash_diverge),
    ("Z10-missing-rust-aware-row", "Violated", missing_rust_aware_row),
    ("Z11-missing-runtime-coverage", "Violated", missing_coverage),
    ("Z12-forged-candidate-anchor-hashes", "Violated", forged_anchor_hashes),
]


def main() -> int:
    contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
    binding = json.loads(BINDING.read_text(encoding="utf-8"))
    corpus = json.loads(CORPUS.read_text(encoding="utf-8"))
    r2_contract = json.loads(R4.R2_CONTRACT.read_text(encoding="utf-8"))
    r2_binding = json.loads(R4.R2_BINDING.read_text(encoding="utf-8"))
    pins = {
        "contract_r3": file_sha(CONTRACT),
        "binding_r3": file_sha(BINDING),
        "contract_r2": corpus["protected_inputs"]["contract_r2"][1],
        "binding_r2": corpus["protected_inputs"]["binding_r2"][1],
        "fixture": corpus["protected_inputs"]["fixture"][1],
        "l0_freeze": corpus["protected_inputs"]["l0_freeze"][1],
    }
    r3_pins = {
        "contract": pins["contract_r2"],
        "binding": pins["binding_r2"],
        "fixture": pins["fixture"],
        "l0_freeze": pins["l0_freeze"],
    }
    focal = [
        (case["id"], case["expected"], None)
        for case in corpus["cases"]
        if case["id"] in set(corpus["budget"]["focal_case_ids"])
    ]
    cases = focal + EXTRAS
    orders = {"normal": cases, "repeat": cases, "reverse": list(reversed(cases))}
    outcomes: dict[str, list[str]] = defaultdict(list)
    runs: dict[str, list[dict[str, str]]] = {}
    for mode, ordered in orders.items():
        results = []
        for case_id, expected, mutator in ordered:
            built = R4.build_case(
                case_id if case_id.startswith("Y") else "Y01-live-changed-authorized",
                corpus,
                pins["contract_r3"],
                pins["binding_r3"],
                pins["fixture"],
                binding,
                r2_binding,
                r3_pins,
            )
            dto, runtime_evidence, local_binding, runtime_pin, evidence, invocation = built
            if mutator is not None:
                mutator(dto, evidence, invocation, pins)
            try:
                actual = R4.judge(
                    dto,
                    runtime_evidence,
                    runtime_pin,
                    evidence,
                    invocation,
                    contract,
                    binding,
                    r2_contract,
                    local_binding,
                    r2_binding,
                    pins,
                )
                witness = "all composite predicates passed"
            except R4.Failure as exc:
                actual = "Violated"
                witness = str(exc)
            outcomes[case_id].append(actual)
            results.append(
                {"id": case_id, "expected": expected, "actual": actual, "witness": witness}
            )
        runs[mode] = results
    negatives = [case_id for case_id, expected, _ in cases if expected == "Violated"]
    rejected = sum(outcomes[case_id][0] == "Violated" for case_id in negatives)
    survivors = [case_id for case_id in negatives if outcomes[case_id][0] != "Violated"]
    unstable = [case_id for case_id, values in outcomes.items() if len(set(values)) != 1]
    report = {
        "schema": "p1342-adversary-run-report-r3",
        "regime": "executado sem atestacao de isolamento",
        "scope": "R3/R4 focal only; no full corpus and no productive candidate",
        "protected_input_sha256": {
            "contract_r3": pins["contract_r3"],
            "binding_r3": pins["binding_r3"],
            "checker_r4": file_sha(CHECKER),
            "corpus_r4": file_sha(CORPUS),
        },
        "runs": runs,
        "summary": {
            "cases": len(cases),
            "valid_negatives": len(negatives),
            "negative_violated": rejected,
            "mutation_score": rejected / len(negatives),
            "survivors": survivors,
            "unstable": unstable,
            "verdict": "BLOCKER_NOT_SEALED" if survivors or unstable else "NOT_SEALED",
        },
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 1 if survivors or unstable else 0


if __name__ == "__main__":
    raise SystemExit(main())
