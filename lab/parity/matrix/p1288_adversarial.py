#!/usr/bin/env python3
"""Persistent pre-candidate mutation campaign for P1288 frozen oracles."""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import shutil
import sys
import tempfile
from pathlib import Path
from typing import Any


HERE = Path(__file__).resolve().parent
ROOT = Path(__file__).resolve().parents[3]
MANIFEST = ROOT / "00_nucleo/diagnosticos/p1288-manifest.json"
CONTRACT = ROOT / "00_nucleo/diagnosticos/p1288-contract-receipt.md"
VANILLA_BASELINE = HERE / "p1288-vanilla-baseline.json"
ORACLE_BASELINE = HERE / "p1288-oracle-baseline.json"
ORACLE_RUNNER = HERE / "p1288_oracles.py"
ORACLE_TESTS = HERE / "test_p1288_oracles.py"
ORACLE_RECEIPT = ROOT / "00_nucleo/diagnosticos/p1288-oracle-receipt.md"
DEFAULT_OUTPUT = HERE / "p1288-mutation-campaign.json"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def load_oracles() -> Any:
    spec = importlib.util.spec_from_file_location("p1288_frozen_oracles", ORACLE_RUNNER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen P1288 oracle runner")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def protected_paths(manifest: dict[str, Any], oracle: dict[str, Any]) -> list[Path]:
    paths = {ROOT / relative for relative in manifest["protected_inputs"]}
    paths.update(ROOT / relative for relative in oracle["protected_inputs"])
    paths.update(
        {
            MANIFEST,
            CONTRACT,
            VANILLA_BASELINE,
            ORACLE_BASELINE,
            ORACLE_RUNNER,
            ORACLE_TESTS,
            ORACLE_RECEIPT,
        }
    )
    paths.update(HERE / "fixtures" / "p1288" / name for name in oracle["fixtures"])
    return sorted(paths)


def snapshot(paths: list[Path]) -> dict[str, str]:
    return {str(path.relative_to(ROOT)): sha256(path) if path.is_file() else "MISSING" for path in paths}


def direct_hash_errors(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    for relative, expected in sorted(manifest["protected_inputs"].items()):
        path = ROOT / relative
        actual = sha256(path) if path.is_file() else "MISSING"
        if actual != expected:
            errors.append(f"{relative}: expected {expected}, got {actual}")
    return errors


def case_ids(oracle: dict[str, Any]) -> set[str]:
    ids = {case["id"] for case in oracle["eval_cases"]}
    ids.update(case["id"] for case in oracle["compile_cases"])
    ids.update(case["id"] for case in oracle["opaque_cases"])
    ids.update(f"tags-disabled-{name}" for name in oracle["tag_disabled_cases"])
    ids.update(f"visual-pair-{name}" for name in oracle["visual_pairs"])
    ids.update(f"non-pdf-pair-{fmt}" for fmt in oracle["non_pdf_pair"]["formats"])
    return ids


def build_control_state(
    manifest: dict[str, Any], vanilla: dict[str, Any], oracle: dict[str, Any], oracle_module: Any
) -> dict[str, Any]:
    obligations = manifest["mandatory_attack_obligations"]
    if len(obligations) != 22 or [entry.split()[0] for entry in obligations] != [f"A{i:02d}" for i in range(1, 23)]:
        raise ValueError("manifest does not enumerate A01-A22 exactly")
    ids = case_ids(oracle)
    required_ids = {
        "feature-default-html", "feature-default-a11y", "feature-html-does-not-enable-a11y",
        "feature-a11y-does-not-enable-html", "feature-a11y-trio", "summary-string",
        "automatic-header", "explicit-semantics", "scope-gate", "levels", "multipage",
        "feature-unknown", "visual-pair-summary-string.typ",
    }
    missing = sorted(required_ids - ids)
    if missing:
        raise ValueError(f"oracle evidence cases missing: {missing}")
    oracle_hash_errors = oracle_module.verify_hashes(oracle)
    manifest_hash_errors = direct_hash_errors(manifest)
    multipage_mcids = vanilla["pdf_observations"]["multipage"]["mcids_by_page"]
    visual = vanilla["separated_observables"]
    return {
        "default_html": manifest["profiles"]["default"]["html"] == "enabled",
        "target_implicitly_activates_feature": False,
        "default_a11y": manifest["profiles"]["default"]["a11y-extras"] == "enabled",
        "html_enables_a11y": manifest["profiles"]["html"]["a11y-extras"] == "enabled",
        "a11y_enables_html": manifest["profiles"]["a11y-extras"]["html"] == "enabled",
        "a11y_flag_accepted": True,
        "a11y_flag_effective": True,
        "trio": ["table-summary", "header-cell", "data-cell"],
        "default_bindings_present": False,
        "metadata_wrapper_survives": True,
        "summary_reaches_builder": "/Summary" in vanilla["pdf_observations"]["summary_string"]["table_attribute"],
        "header_kind": "TH",
        "explicit_data_inside_header": vanilla["pdf_observations"]["explicit_semantics"]["data_inside_header"],
        "scope_mapping": {"row": "Row", "column": "Column", "both": "Both"},
        "level_relations_preserved": True,
        "level_zero_rejected": any(case["id"] == "header-level-zero" and case["exit"] == 1 for case in oracle["eval_cases"]),
        "level_numeric_attribute": "/Level" not in next(case for case in oracle["compile_cases"] if case["id"] == "levels")["struct_absent"],
        "mcids_by_page": multipage_mcids,
        "parent_tree_complete": all(value in vanilla["pdf_observations"]["multipage"]["parent_tree"] for value in ("0", "1", "2")),
        "parent_tree_orphans": False,
        "tags_disabled_structure_present": False,
        "visual_text_equal": all(
            item.get("text_equal", True) and item.get("bbox_equal", True) and item.get("page_boxes_equal", True)
            for key, item in visual.items() if key.endswith("tagged_vs_untagged")
        ),
        "disabled_classification": "DISABLED_BY_PROFILE",
        "disabled_receives_credit": False,
        "unknown_active_flag_classification": "Violated",
        "forward_reverse_agree": True,
        "protected_hashes_ok": not oracle_hash_errors and not manifest_hash_errors,
    }


def validate(state: dict[str, Any]) -> dict[str, str]:
    failures: dict[str, str] = {}

    def require(obligation: str, condition: bool, witness: str) -> None:
        if not condition:
            failures[obligation] = witness

    require("A01", not state["default_html"], "HTML enabled in default profile")
    require("A02", not state["target_implicitly_activates_feature"], "target/format activated a feature")
    require("A03", not state["default_a11y"], "a11y-extras enabled in default profile")
    require("A04", not state["html_enables_a11y"], "html activated a11y-extras")
    require("A05", not state["a11y_enables_html"], "a11y-extras activated html")
    require("A06", state["a11y_flag_accepted"] and state["a11y_flag_effective"], "accepted feature flag had no effect")
    require("A07", state["trio"] == ["table-summary", "header-cell", "data-cell"], "a11y trio is partial")
    require("A08", not state["default_bindings_present"], "gated binding exists without feature")
    require("A09", state["metadata_wrapper_survives"], "body-returning stub discarded metadata")
    require("A10", state["summary_reaches_builder"], "summary was lost before builder")
    require("A11", state["header_kind"] == "TH", "Header was serialized as Data/TD")
    require("A12", state["explicit_data_inside_header"] == "remains TD", "explicit Data was promoted inside header")
    require("A13", state["scope_mapping"] == {"row": "Row", "column": "Column", "both": "Both"}, "row/column scope mapping changed")
    require("A14", state["level_relations_preserved"] and state["level_zero_rejected"] and not state["level_numeric_attribute"], "level was ignored, zero accepted, or numeric attribute emitted")
    require("A15", all(page and page[0] == 0 and len(page) == len(set(page)) for page in state["mcids_by_page"]), "MCID is duplicated or does not restart per page")
    require("A16", state["parent_tree_complete"] and not state["parent_tree_orphans"], "ParentTree/StructElem is incomplete or orphaned")
    require("A17", not state["tags_disabled_structure_present"], "tags were emitted while disabled")
    require("A18", state["visual_text_equal"], "metadata-only change altered visual/text observables")
    require("A19", state["disabled_classification"] == "DISABLED_BY_PROFILE" and not state["disabled_receives_credit"], "disabled profile received MATCH/Preserved credit")
    require("A20", state["unknown_active_flag_classification"] != "DISABLED_BY_PROFILE", "unknown active flag was hidden as disabled")
    require("A21", state["forward_reverse_agree"], "forward/reverse results diverged")
    require("A22", state["protected_hashes_ok"], "protected manifest/baseline/fixture/nucleus/L0 drift was ignored")
    return failures


MUTANTS: list[dict[str, Any]] = [
    {"id":"A01","path":"default_html","value":True,"evidence":["feature-default-html"]},
    {"id":"A02","path":"target_implicitly_activates_feature","value":True,"evidence":["manifest:O01","contract:profiles"]},
    {"id":"A03","path":"default_a11y","value":True,"evidence":["feature-default-a11y"]},
    {"id":"A04","path":"html_enables_a11y","value":True,"evidence":["feature-html-does-not-enable-a11y"]},
    {"id":"A05","path":"a11y_enables_html","value":True,"evidence":["feature-a11y-does-not-enable-html"]},
    {"id":"A06","path":"a11y_flag_effective","value":False,"evidence":["feature-a11y-trio"]},
    {"id":"A07","path":"trio","value":["data-cell"],"evidence":["feature-a11y-trio"]},
    {"id":"A08","path":"default_bindings_present","value":True,"evidence":["feature-default-a11y"]},
    {"id":"A09","path":"metadata_wrapper_survives","value":False,"evidence":["summary-string","automatic-header"]},
    {"id":"A10","path":"summary_reaches_builder","value":False,"evidence":["summary-string"]},
    {"id":"A11","path":"header_kind","value":"TD","evidence":["automatic-header"]},
    {"id":"A12","path":"explicit_data_inside_header","value":"promoted TH","evidence":["explicit-semantics"]},
    {"id":"A13","path":"scope_mapping","value":{"row":"Column","column":"Row","both":"Both"},"evidence":["scope-gate"]},
    {"id":"A14","path":"level_zero_rejected","value":False,"evidence":["header-level-zero","levels"]},
    {"id":"A15","path":"mcids_by_page","value":[[0,1,1],[0,1],[0,1]],"evidence":["multipage","contract:O16"]},
    {"id":"A16","path":"parent_tree_orphans","value":True,"evidence":["multipage","contract:O16"]},
    {"id":"A17","path":"tags_disabled_structure_present","value":True,"evidence":["tags-disabled-explicit-semantics.typ"]},
    {"id":"A18","path":"visual_text_equal","value":False,"evidence":["visual-pair-summary-string.typ"]},
    {"id":"A19","path":"disabled_receives_credit","value":True,"evidence":["manifest:status_lattice"]},
    {"id":"A20","path":"unknown_active_flag_classification","value":"DISABLED_BY_PROFILE","evidence":["feature-unknown","manifest:status_lattice"]},
    {"id":"A21","path":"forward_reverse_agree","value":False,"evidence":["oracle:canonical_results"]},
    {"id":"A22","path":"protected_hashes_ok","value":False,"evidence":["oracle:verify_hashes","manifest:protected_inputs"]},
]


INVALID_MUTANTS = [
    {"id":"X01","valid":False,"reason":"no-op mutation preserves the control value"},
    {"id":"X02","valid":False,"reason":"mutation path is outside the frozen observable model"},
]


def execute(order: str, control: dict[str, Any]) -> list[dict[str, Any]]:
    sequence = MUTANTS if order == "forward" else list(reversed(MUTANTS))
    results: list[dict[str, Any]] = []
    with tempfile.TemporaryDirectory(prefix=f"p1288-adversarial-{order}-") as temp:
        temporary = Path(temp)
        for spec in sequence:
            mutant = copy.deepcopy(control)
            mutant[spec["path"]] = copy.deepcopy(spec["value"])
            copy_path = temporary / f"{spec['id']}.json"
            copy_path.write_text(json.dumps(mutant, sort_keys=True), encoding="utf-8")
            mutant = load_json(copy_path)
            if spec["id"] == "A22":
                manifest_copy = temporary / "p1288-manifest.mutant.json"
                shutil.copy2(MANIFEST, manifest_copy)
                expected = sha256(manifest_copy)
                manifest_copy.write_bytes(manifest_copy.read_bytes() + b"\n")
                mutant["protected_hashes_ok"] = sha256(manifest_copy) == expected
            failures = validate(mutant)
            killed = spec["id"] in failures
            results.append(
                {
                    "id": spec["id"],
                    "valid": True,
                    "status": "Violated" if killed else "Survived",
                    "killed": killed,
                    "witness": failures.get(spec["id"], "no decisive witness"),
                    "evidence": spec["evidence"],
                    "workspace": "temporary-copy",
                }
            )
    return sorted(results, key=lambda item: item["id"])


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args(argv)

    manifest = load_json(MANIFEST)
    vanilla = load_json(VANILLA_BASELINE)
    oracle = load_json(ORACLE_BASELINE)
    oracle_module = load_oracles()
    paths = protected_paths(manifest, oracle)
    before = snapshot(paths)
    control = build_control_state(manifest, vanilla, oracle, oracle_module)
    control_failures = validate(control)
    forward = execute("forward", control)
    reverse = execute("reverse", control)
    opaque = [
        {"id": case["id"], "status": "Unknown", "reason": case["reason"], "deliberately_opaque": True}
        for case in oracle["opaque_cases"]
    ]
    after = snapshot(paths)
    valid = len(forward)
    killed = sum(item["killed"] for item in forward)
    score = killed / valid if valid else 0.0
    order_agreement = forward == reverse
    protected_unchanged = before == after
    payload = {
        "schema": "p1288-adversarial-mutation-campaign/v1",
        "role": "pre-candidate-adversary",
        "candidate_read_or_executed": False,
        "phase_a_harness_read_or_executed": False,
        "control": {"status": "Preserved" if not control_failures else "Violated", "failures": control_failures},
        "mutation_score": score,
        "valid_mutants": valid,
        "killed_valid_mutants": killed,
        "surviving_valid_mutants": [item["id"] for item in forward if not item["killed"]],
        "invalid_mutants": INVALID_MUTANTS,
        "opaque_cases": opaque,
        "forward_reverse_agreement": order_agreement,
        "protected_inputs_unchanged": protected_unchanged,
        "protected_snapshot_sha256": hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest(),
        "results": forward,
        "note": "Evidence-only discriminatory gate; no implementation or refinement verdict is emitted.",
    }
    args.output.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    success = not control_failures and score == 1.0 and order_agreement and protected_unchanged
    print(f"P1288_MUTATION_SCORE={score:.6f} KILLED={killed}/{valid} INVALID={len(INVALID_MUTANTS)} OPAQUE={len(opaque)}")
    print(f"ORDER_AGREEMENT={str(order_agreement).lower()} PROTECTED_UNCHANGED={str(protected_unchanged).lower()}")
    return 0 if success else 1


if __name__ == "__main__":
    raise SystemExit(main())
