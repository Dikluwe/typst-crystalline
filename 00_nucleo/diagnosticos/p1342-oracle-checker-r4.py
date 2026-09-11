#!/usr/bin/env python3
"""Candidate-free R4 oracle calibration for the P1342 R3 contract.

The source side is exercised with closed synthetic baseline/candidate fixtures.
No productive file is opened as a candidate.  The R3 ledger checker remains the
normative base for P/A/X cases; this module adds only the R3 source-evidence
separation and the Y01-Y06 meta-controls.
"""

from __future__ import annotations

import argparse
import ast
import copy
import hashlib
import importlib.util
import json
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
R3_CHECKER = DIAG / "p1342-oracle-checker-r3.py"
R2_CONTRACT = DIAG / "p1342-contract-spec-r2.json"
R2_BINDING = DIAG / "p1342-contract-binding-r2.json"
EVIDENCE_PATH = "fixture://independent-verifier/p1342-candidate-source-evidence-v1.json"
PRESEAL_PATH = "fixture://independent-preseal/p1342-r3-baseline-attestation.json"
MODES = ["normal", "repeat", "reverse"]


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load frozen checker {path.name}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


R3 = load_module(R3_CHECKER, "p1342_oracle_checker_r3_frozen")
R2 = R3.R2
Failure = R3.Failure
digest = R3.digest


def exact_keys(value: Any, keys: set[str], where: str) -> None:
    if not isinstance(value, dict) or set(value) != keys:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        raise Failure(f"{where}: closed keys {sorted(keys)} required, got {got}")


def integer(value: Any, where: str) -> int:
    if type(value) is not int:
        raise Failure(f"{where}: exact integer required; bool is not int")
    return value


def sha_text(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def validate_contract_binding(contract: dict[str, Any], binding: dict[str, Any], r2_binding: dict[str, Any], pins: dict[str, str]) -> None:
    if contract.get("schema") != "p1342-real-binding-contract-v3" or binding.get("schema") != "p1342-static-binding-manifest-v3":
        raise Failure("R3 contract/binding identity mismatch")
    if contract.get("normative_composition", {}).get("binding_r3") != ["00_nucleo/diagnosticos/p1342-contract-binding-r3.json", pins["binding_r3"]]:
        raise Failure("R3 contract does not pin the supplied binding")
    if binding.get("predecessor") != ["00_nucleo/diagnosticos/p1342-contract-binding-r2.json", pins["binding_r2"]]:
        raise Failure("R3 binding predecessor mismatch")
    overlays = binding.get("row_overlay")
    hooks = r2_binding.get("hooks")
    if not isinstance(overlays, list) or not isinstance(hooks, list) or len(overlays) != 19 or len(hooks) != 19:
        raise Failure("R3 composition requires exactly 19 rows")
    for index, (overlay, row) in enumerate(zip(overlays, hooks)):
        exact_keys(overlay, {"row_index", "hook", "baseline_anchor_snippet_sha256"}, f"overlay[{index}]")
        if overlay["row_index"] != index or overlay["hook"] != row["hook"]:
            raise Failure("R3 row overlay order/hook mismatch")
        if overlay["baseline_anchor_snippet_sha256"] != sha_text(row["anchor_needle"]):
            raise Failure("R3 baseline anchor snippet pin mismatch")
    prohibition = contract.get("candidate_checker_contract", {}).get("explicit_prohibition", "")
    if "must not compare any live candidate file digest with baseline_file_sha256" not in prohibition:
        raise Failure("R3 live/baseline prohibition absent")


def baseline_bytes(path: str) -> str:
    return f"// closed candidate-free P1342 oracle fixture\n// path: {path}\nfn baseline_fixture() {{}}\n"


def candidate_bytes(path: str) -> str:
    return baseline_bytes(path) + "#[cfg(p1339_observation)]\nfn p1342_observation_fixture() {}\n"


def checker_fixture_source(compare_to: str = "candidate_file_sha256") -> str:
    return (
        "def compare_live(live_candidate_sha256, candidate_file_sha256, baseline_file_sha256):\n"
        "    del baseline_file_sha256\n"
        f"    return live_candidate_sha256 == {compare_to}\n"
    )


def seal_evidence(evidence: dict[str, Any]) -> None:
    body = {key: value for key, value in evidence.items() if key != "verifier_receipt"}
    evidence["verifier_receipt"] = {
        "schema": "p1342-candidate-source-verifier-receipt-v1",
        "issuer": "independent-oracle-fixture-verifier",
        "evidence_body_sha256": digest(body),
    }


def refresh_candidate_tree(evidence: dict[str, Any]) -> None:
    pairs = [[item["path"], item["candidate_file_sha256"]] for item in evidence["files"]]
    evidence["candidate_tree"]["tree_sha256"] = digest(pairs)
    evidence["candidate_tree"]["file_count"] = len(pairs)


def build_source_fixture(dto: dict[str, Any], contract_pin: str, binding_pin: str, fixture_pin: str, binding_r3: dict[str, Any], r2_binding: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:
    rows = r2_binding["hooks"]
    overlays = binding_r3["row_overlay"]
    paths = list(dict.fromkeys(row["path"] for row in rows))
    baseline_files = []
    files = []
    for path in paths:
        old = baseline_bytes(path)
        new = candidate_bytes(path)
        baseline_files.append({"path": path, "baseline_bytes": old, "baseline_file_sha256": sha_text(old)})
        files.append({
            "path": path,
            "baseline_file_sha256": sha_text(old),
            "candidate_file_sha256": sha_text(new),
            "candidate_bytes": new,
            "rust_parse_ok": True,
            "outside_allowed_boundaries_equal": True,
        })
    baseline_rows = [
        {
            "row_index": index,
            "hook": row["hook"],
            "path": row["path"],
            "baseline_anchor_snippet_sha256": overlays[index]["baseline_anchor_snippet_sha256"],
        }
        for index, row in enumerate(rows)
    ]
    baseline_attestation = {
        "schema": "p1342-r3-baseline-attestation-fixture-v1",
        "kind": "candidate-free-synthetic-calibration-fixture",
        "contract_sha256": contract_pin,
        "binding_sha256": binding_pin,
        "files": baseline_files,
        "rows": baseline_rows,
    }
    file_map = {item["path"]: item for item in files}
    boundaries = []
    candidate_rows = []
    row_coverage = []
    for index, row in enumerate(rows):
        boundary_id = f"YB{index:02d}"
        file_record = file_map[row["path"]]
        boundaries.append({
            "id": boundary_id,
            "path": row["path"],
            "rust_owner_symbol": row["symbol"],
            "cfg_predicate": "p1339_observation",
            "baseline_ast_region_sha256": sha_text("baseline-ast:" + row["hook"]),
            "candidate_ast_region_sha256": sha_text("candidate-ast:" + row["hook"]),
            "candidate_byte_range": [0, len(file_record["candidate_bytes"].encode("utf-8"))],
            "change_kind": "cfg-only-observation-hook",
            "semantic_surface": "observation-only",
            "outside_region_unchanged": True,
        })
        refs = [f"coverage:{mode}:{index}" for mode in MODES]
        candidate_rows.append({
            "row_index": index,
            "hook": row["hook"],
            "path": row["path"],
            "candidate_file_sha256": file_record["candidate_file_sha256"],
            "symbol": row["symbol"],
            "branch": row["branch"],
            "anchor_needle": row["anchor_needle"],
            "baseline_anchor_snippet_sha256": overlays[index]["baseline_anchor_snippet_sha256"],
            "candidate_anchor_snippet_sha256": sha_text("candidate-anchor:" + row["hook"]),
            "candidate_symbol_sha256": sha_text("candidate-symbol:" + row["symbol"] + ":" + row["hook"]),
            "cfg_predicate": "p1339_observation",
            "anchor_matches_in_symbol_branch": 1,
            "observation_adjacency": row["relative_position"],
            "allowed_diff_boundary_ids": [boundary_id],
            "runtime_coverage_refs": refs,
        })
        row_coverage.append({"row_index": index, "hook": row["hook"], "normal": refs[0], "repeat": refs[1], "reverse": refs[2]})
    raw_refs = {run["mode"]: digest(run["raw_snapshot"]) for run in dto["runs"]}
    evidence = {
        "schema": "p1342-candidate-source-evidence-v1",
        "step": 1342,
        "contract_sha256": contract_pin,
        "binding_sha256": binding_pin,
        "preseal": {"schema": "p1342-r3-preseal-fixture-reference-v1", "path": PRESEAL_PATH, "sha256": digest(baseline_attestation)},
        "baseline_attestation": baseline_attestation,
        "candidate_tree": {"schema": "p1342-candidate-tree-fixture-v1", "source_kind": "closed-synthetic-source-fixture", "tree_sha256": "", "file_count": 0},
        "files": files,
        "rows": candidate_rows,
        "allowed_diff_boundaries": boundaries,
        "writer_projection_audit": {
            "append_writer_symbol": "P1342FixtureLedger::append",
            "append_writer_candidate_symbol_sha256": sha_text("P1342FixtureLedger::append"),
            "all_writer_callsites": [row["hook"] for row in rows],
            "alternate_writers": [],
            "projection_symbols": ["P1342FixtureLedger::project"],
            "projection_writes": [],
            "raw_snapshot_created_before_projection": True,
            "result": "pass",
        },
        "runtime_coverage": {
            "fixture_sha256": fixture_pin,
            "modes": MODES,
            "row_coverage": row_coverage,
            "raw_snapshot_refs": raw_refs,
            "result": "pass",
        },
        "verifier_receipt": {},
    }
    refresh_candidate_tree(evidence)
    seal_evidence(evidence)
    source = checker_fixture_source()
    invocation = {
        "schema": "p1342-r4-trusted-invocation-fixture-v1",
        "candidate_evidence_path": EVIDENCE_PATH,
        "candidate_evidence_sha256": digest(evidence),
        "channel": "independent-verifier-fixture",
        "checker_fixture": {"source": source, "sha256": sha_text(source)},
    }
    return evidence, invocation


def validate_checker_meta_fixture(value: dict[str, Any]) -> None:
    exact_keys(value, {"source", "sha256"}, "checker meta fixture")
    source = value["source"]
    if not isinstance(source, str) or value["sha256"] != sha_text(source):
        raise Failure("checker meta fixture pin mismatch")
    try:
        tree = ast.parse(source)
    except SyntaxError as exc:
        raise Failure("checker meta fixture is not parseable") from exc
    candidate_compare = False
    for node in ast.walk(tree):
        if isinstance(node, ast.Compare):
            names = {part.id for part in [node.left, *node.comparators] if isinstance(part, ast.Name)}
            if "live_candidate_sha256" in names and "baseline_file_sha256" in names:
                raise Failure("meta-calibration rejected a checker that compares live candidate hash to baseline")
            if names == {"live_candidate_sha256", "candidate_file_sha256"}:
                candidate_compare = True
    if not candidate_compare:
        raise Failure("checker meta fixture lacks candidate-evidence live hash comparison")


def validate_source_evidence(dto: dict[str, Any], evidence: Any, invocation: Any, contract_pin: str, binding_pin: str, fixture_pin: str, binding_r3: dict[str, Any], r2_binding: dict[str, Any]) -> None:
    if evidence is None:
        raise Failure("candidate evidence supplied only by DTO/facade is outside the trusted channel")
    exact_keys(invocation, {"schema", "candidate_evidence_path", "candidate_evidence_sha256", "channel", "checker_fixture"}, "trusted invocation")
    if invocation["schema"] != "p1342-r4-trusted-invocation-fixture-v1" or invocation["channel"] != "independent-verifier-fixture":
        raise Failure("candidate evidence channel is not independently controlled")
    if invocation["candidate_evidence_path"] != EVIDENCE_PATH:
        raise Failure("external candidate evidence path adulterated")
    if invocation["candidate_evidence_sha256"] != digest(evidence):
        raise Failure("external candidate evidence hash adulterated")
    validate_checker_meta_fixture(invocation["checker_fixture"])
    top = set(binding_r3["candidate_source_evidence_contract"]["closed_top_level_keys"])
    exact_keys(evidence, top, "candidate source evidence")
    if evidence["schema"] != "p1342-candidate-source-evidence-v1" or integer(evidence["step"], "candidate evidence step") != 1342:
        raise Failure("candidate source evidence identity mismatch")
    if evidence["contract_sha256"] != contract_pin or evidence["binding_sha256"] != binding_pin:
        raise Failure("candidate evidence contract/binding pin mismatch")
    exact_keys(evidence["preseal"], {"schema", "path", "sha256"}, "preseal reference")
    baseline = evidence["baseline_attestation"]
    exact_keys(baseline, {"schema", "kind", "contract_sha256", "binding_sha256", "files", "rows"}, "baseline attestation fixture")
    if evidence["preseal"] != {"schema": "p1342-r3-preseal-fixture-reference-v1", "path": PRESEAL_PATH, "sha256": digest(baseline)}:
        raise Failure("historical baseline attestation is not preseal-pinned")
    if baseline["kind"] != "candidate-free-synthetic-calibration-fixture" or baseline["contract_sha256"] != contract_pin or baseline["binding_sha256"] != binding_pin:
        raise Failure("baseline attestation fixture identity mismatch")
    rows = r2_binding["hooks"]
    overlays = binding_r3["row_overlay"]
    paths = list(dict.fromkeys(row["path"] for row in rows))
    if len(baseline["files"]) != len(paths) or len(baseline["rows"]) != 19:
        raise Failure("baseline attestation fixture cardinality mismatch")
    baseline_file_map = {}
    for item in baseline["files"]:
        exact_keys(item, {"path", "baseline_bytes", "baseline_file_sha256"}, "baseline file fixture")
        if item["path"] in baseline_file_map or item["baseline_file_sha256"] != sha_text(item["baseline_bytes"]):
            raise Failure("baseline file fixture hash/cardinality mismatch")
        baseline_file_map[item["path"]] = item
    if list(baseline_file_map) != paths:
        raise Failure("baseline file fixture path order mismatch")
    file_keys = set(binding_r3["candidate_source_evidence_contract"]["file_keys"])
    if len(evidence["files"]) != len(paths):
        raise Failure("exactly one candidate file fixture per composed path required")
    file_map = {}
    for item in evidence["files"]:
        exact_keys(item, file_keys, "candidate file evidence")
        path = item["path"]
        if path in file_map or path not in baseline_file_map:
            raise Failure("candidate file evidence path/cardinality mismatch")
        if item["baseline_file_sha256"] != baseline_file_map[path]["baseline_file_sha256"]:
            raise Failure("candidate evidence does not reference the pinned historical fixture")
        # Deliberately compare live fixture bytes only with candidate_file_sha256.
        # Equality or inequality with the historical digest is not a predicate.
        if item["candidate_file_sha256"] != sha_text(item["candidate_bytes"]):
            raise Failure("live candidate fixture hash differs from external candidate_file_sha256")
        if item["rust_parse_ok"] is not True:
            raise Failure("candidate source fixture failed Rust-aware parse")
        if item["outside_allowed_boundaries_equal"] is not True:
            raise Failure("candidate changed bytes outside the allowed boundary")
        file_map[path] = item
    if list(file_map) != paths:
        raise Failure("candidate file evidence path order mismatch")
    tree = evidence["candidate_tree"]
    exact_keys(tree, {"schema", "source_kind", "tree_sha256", "file_count"}, "candidate tree")
    expected_tree = digest([[path, file_map[path]["candidate_file_sha256"]] for path in paths])
    if tree != {"schema": "p1342-candidate-tree-fixture-v1", "source_kind": "closed-synthetic-source-fixture", "tree_sha256": expected_tree, "file_count": len(paths)}:
        raise Failure("candidate tree fixture mismatch")
    boundary_keys = set(binding_r3["candidate_source_evidence_contract"]["allowed_diff_boundary_keys"])
    boundaries = {}
    for item in evidence["allowed_diff_boundaries"]:
        exact_keys(item, boundary_keys, "allowed diff boundary")
        if item["id"] in boundaries or item["path"] not in file_map:
            raise Failure("allowed diff boundary identity/path mismatch")
        if item["cfg_predicate"] not in {"p1339_observation", "all(test,p1339_observation)"} or item["semantic_surface"] != "observation-only" or item["outside_region_unchanged"] is not True:
            raise Failure("change outside the allowed cfg-only observation boundary")
        byte_range = item["candidate_byte_range"]
        if not isinstance(byte_range, list) or len(byte_range) != 2 or any(type(x) is not int for x in byte_range) or byte_range[0] < 0 or byte_range[1] > len(file_map[item["path"]]["candidate_bytes"].encode("utf-8")) or byte_range[0] > byte_range[1]:
            raise Failure("allowed diff byte boundary malformed")
        boundaries[item["id"]] = item
    row_keys = set(binding_r3["candidate_source_evidence_contract"]["row_keys"])
    if len(evidence["rows"]) != 19:
        raise Failure("exactly 19 candidate row records required")
    for index, (item, base, overlay) in enumerate(zip(evidence["rows"], rows, overlays)):
        exact_keys(item, row_keys, f"candidate row[{index}]")
        if item["row_index"] != index or item["hook"] != base["hook"] or item["path"] != base["path"] or item["symbol"] != base["symbol"] or item["branch"] != base["branch"] or item["anchor_needle"] != base["anchor_needle"]:
            raise Failure("candidate row does not resolve exact composed binding")
        if item["candidate_file_sha256"] != file_map[base["path"]]["candidate_file_sha256"] or item["baseline_anchor_snippet_sha256"] != overlay["baseline_anchor_snippet_sha256"]:
            raise Failure("candidate row file/snippet pin mismatch")
        if item["cfg_predicate"] != "p1339_observation" or item["anchor_matches_in_symbol_branch"] != 1 or item["observation_adjacency"] != base["relative_position"]:
            raise Failure("candidate row lacks Rust-aware unique hook adjacency")
        if item["allowed_diff_boundary_ids"] != [f"YB{index:02d}"] or item["allowed_diff_boundary_ids"][0] not in boundaries:
            raise Failure("candidate row is outside its allowed diff boundary")
        if item["runtime_coverage_refs"] != [f"coverage:{mode}:{index}" for mode in MODES]:
            raise Failure("candidate row runtime coverage refs mismatch")
    writer = evidence["writer_projection_audit"]
    exact_keys(writer, set(binding_r3["candidate_source_evidence_contract"]["writer_projection_audit_keys"]), "writer/projection audit")
    if not writer["append_writer_symbol"] or len(writer["all_writer_callsites"]) != 19 or writer["alternate_writers"] != [] or writer["projection_writes"] != [] or writer["raw_snapshot_created_before_projection"] is not True or writer["result"] != "pass":
        raise Failure("writer uniqueness or projection read-only audit failed")
    coverage = evidence["runtime_coverage"]
    exact_keys(coverage, set(binding_r3["candidate_source_evidence_contract"]["runtime_coverage_keys"]), "runtime coverage")
    if coverage["fixture_sha256"] != fixture_pin or coverage["modes"] != MODES or coverage["result"] != "pass" or len(coverage["row_coverage"]) != 19:
        raise Failure("candidate hook/coverage evidence absent or incomplete")
    for index, item in enumerate(coverage["row_coverage"]):
        exact_keys(item, {"row_index", "hook", "normal", "repeat", "reverse"}, f"row coverage[{index}]")
        expected = {"row_index": index, "hook": rows[index]["hook"], **{mode: f"coverage:{mode}:{index}" for mode in MODES}}
        if item != expected:
            raise Failure("candidate hook/coverage evidence absent or disconnected")
    expected_raw_refs = {run["mode"]: digest(run["raw_snapshot"]) for run in dto["runs"]}
    if coverage["raw_snapshot_refs"] != expected_raw_refs:
        raise Failure("candidate source coverage is disconnected from raw preprojection snapshots")
    receipt = evidence["verifier_receipt"]
    exact_keys(receipt, {"schema", "issuer", "evidence_body_sha256"}, "candidate verifier receipt")
    body = {key: value for key, value in evidence.items() if key != "verifier_receipt"}
    if receipt != {"schema": "p1342-candidate-source-verifier-receipt-v1", "issuer": "independent-oracle-fixture-verifier", "evidence_body_sha256": digest(body)}:
        raise Failure("candidate source evidence verifier receipt mismatch")


def judge(dto: dict[str, Any], runtime_evidence: dict[str, Any], runtime_pin: str, candidate_evidence: Any, invocation: Any, contract_r3: dict[str, Any], binding_r3: dict[str, Any], r2_contract: dict[str, Any], local_r2_binding: dict[str, Any], exact_r2_binding: dict[str, Any], pins: dict[str, str]) -> str:
    if "candidate_source_evidence" in dto or "candidate_evidence_path" in dto or "candidate_evidence_sha256" in dto:
        raise Failure("DTO/facade may not authenticate candidate source evidence")
    validate_contract_binding(contract_r3, binding_r3, exact_r2_binding, pins)
    validate_source_evidence(dto, candidate_evidence, invocation, pins["contract_r3"], pins["binding_r3"], pins["fixture"], binding_r3, exact_r2_binding)
    r3_pins = {"contract": pins["contract_r2"], "binding": pins["binding_r2"], "fixture": pins["fixture"], "l0_freeze": pins["l0_freeze"]}
    return R3.judge(dto, runtime_evidence, runtime_pin, r2_contract, local_r2_binding, r3_pins)


def apply_y(case_id: str, dto: dict[str, Any], candidate_evidence: dict[str, Any], invocation: dict[str, Any]) -> tuple[Any, dict[str, Any]]:
    if case_id == "Y01-live-changed-authorized":
        pass
    elif case_id == "Y02-live-baseline-equal-hook-absent":
        target = candidate_evidence["files"][0]
        baseline = candidate_evidence["baseline_attestation"]["files"][0]
        target["candidate_bytes"] = baseline["baseline_bytes"]
        target["candidate_file_sha256"] = baseline["baseline_file_sha256"]
        for row in candidate_evidence["rows"]:
            if row["path"] == target["path"]:
                row["candidate_file_sha256"] = target["candidate_file_sha256"]
        for boundary in candidate_evidence["allowed_diff_boundaries"]:
            if boundary["path"] == target["path"]:
                boundary["candidate_byte_range"] = [0, len(target["candidate_bytes"].encode("utf-8"))]
        candidate_evidence["runtime_coverage"]["row_coverage"][0]["normal"] = None
        refresh_candidate_tree(candidate_evidence)
    elif case_id == "Y03-forbidden-live-baseline-comparator":
        source = checker_fixture_source("baseline_file_sha256")
        invocation["checker_fixture"] = {"source": source, "sha256": sha_text(source)}
    elif case_id == "Y04-evidence-only-in-dto-facade":
        dto["candidate_source_evidence"] = copy.deepcopy(candidate_evidence)
        candidate_evidence = None
    elif case_id == "Y05-external-evidence-path-hash-adulterated":
        invocation["candidate_evidence_path"] = "fixture://candidate-controlled/forged.json"
        invocation["candidate_evidence_sha256"] = "0" * 64
    elif case_id == "Y06-change-outside-allowed-boundary":
        candidate_evidence["files"][0]["outside_allowed_boundaries_equal"] = False
        candidate_evidence["allowed_diff_boundaries"][0]["outside_region_unchanged"] = False
    else:
        raise Failure(f"unknown R4 Y case {case_id}")
    if candidate_evidence is not None:
        seal_evidence(candidate_evidence)
        if case_id != "Y05-external-evidence-path-hash-adulterated":
            invocation["candidate_evidence_sha256"] = digest(candidate_evidence)
    return candidate_evidence, invocation


def build_case(case_id: str, corpus: dict[str, Any], contract_pin: str, binding_pin: str, fixture_pin: str, binding_r3: dict[str, Any], r2_binding: dict[str, Any], r3_pins: dict[str, str]) -> tuple[Any, ...]:
    base_id = case_id if case_id.startswith(("P", "A", "X")) else "P01-inspectable-composite"
    dto, runtime_evidence, local_r2_binding, runtime_pin = R3.build_case(base_id, corpus, r2_binding, r3_pins)
    candidate_evidence, invocation = build_source_fixture(dto, contract_pin, binding_pin, fixture_pin, binding_r3, r2_binding)
    if case_id.startswith("Y"):
        candidate_evidence, invocation = apply_y(case_id, dto, candidate_evidence, invocation)
    return dto, runtime_evidence, local_r2_binding, runtime_pin, candidate_evidence, invocation


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--contract", required=True, type=Path)
    parser.add_argument("--binding", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    parser.add_argument("--focus", action="store_true")
    args = parser.parse_args()
    contract_r3 = json.loads(args.contract.read_text(encoding="utf-8"))
    binding_r3 = json.loads(args.binding.read_text(encoding="utf-8"))
    r2_contract = json.loads(R2_CONTRACT.read_text(encoding="utf-8"))
    r2_binding = json.loads(R2_BINDING.read_text(encoding="utf-8"))
    corpus = json.loads(args.corpus.read_text(encoding="utf-8"))
    exact_keys(corpus, {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "challenges", "r4_closure", "budget", "cases", "closed_case_order", "scope_exclusions"}, "R4 corpus")
    if corpus["schema"] != "p1342-oracle-corpus-v4" or integer(corpus["revision"], "corpus revision") != 4:
        raise Failure("R4 corpus identity mismatch")
    for pin in corpus["protected_inputs"].values():
        if not isinstance(pin, list) or len(pin) != 2 or hashlib.sha256((ROOT / pin[0]).read_bytes()).hexdigest() != pin[1]:
            raise Failure("R4 protected input changed")
    exact_keys(corpus["challenges"], set(MODES), "R4 challenges")
    cases = corpus["cases"]
    if corpus["closed_case_order"] != [case["id"] for case in cases]:
        raise Failure("R4 closed case order mismatch")
    for case in cases:
        exact_keys(case, {"id", "family", "expected", "validity", "purpose"}, f"case {case.get('id', '?')}")
        if case["validity"] != "valid":
            raise Failure("no invalid vector is permitted in R4 corpus")
    if args.focus:
        focus_ids = set(corpus["budget"]["focal_case_ids"])
        cases = [case for case in cases if case["id"] in focus_ids]
        if [case["id"] for case in cases] != corpus["budget"]["focal_case_ids"]:
            raise Failure("R4 focal case order mismatch")
    pins = {
        "contract_r3": hashlib.sha256(args.contract.read_bytes()).hexdigest(),
        "binding_r3": hashlib.sha256(args.binding.read_bytes()).hexdigest(),
        "contract_r2": corpus["protected_inputs"]["contract_r2"][1],
        "binding_r2": corpus["protected_inputs"]["binding_r2"][1],
        "fixture": corpus["protected_inputs"]["fixture"][1],
        "l0_freeze": corpus["protected_inputs"]["l0_freeze"][1],
    }
    if pins["contract_r3"] != corpus["protected_inputs"]["contract_r3_json"][1] or pins["binding_r3"] != corpus["protected_inputs"]["binding_r3"][1]:
        raise Failure("R3 contract/binding external pin mismatch")
    validate_contract_binding(contract_r3, binding_r3, r2_binding, pins)
    r3_pins = {"contract": pins["contract_r2"], "binding": pins["binding_r2"], "fixture": pins["fixture"], "l0_freeze": pins["l0_freeze"]}
    orders = {"normal": cases, "repeat": cases, "reverse": list(reversed(cases))}
    per_case: dict[str, list[str]] = defaultdict(list)
    report: dict[str, Any] = {"schema": "p1342-oracle-run-report-v4", "phase": "focal" if args.focus else "full-final", "runs": {}, "agreement": True}
    for batch, ordered in orders.items():
        results = []
        for case in ordered:
            built = build_case(case["id"], corpus, pins["contract_r3"], pins["binding_r3"], pins["fixture"], binding_r3, r2_binding, r3_pins)
            dto, runtime_evidence, local_r2_binding, runtime_pin, candidate_evidence, invocation = built
            try:
                actual = judge(dto, runtime_evidence, runtime_pin, candidate_evidence, invocation, contract_r3, binding_r3, r2_contract, local_r2_binding, r2_binding, pins)
                witness = "all R4 composite predicates passed" if actual == "Preserved" else "deliberate payload opacity after all R4 source/runtime predicates"
            except Failure as exc:
                actual = "Violated"
                witness = str(exc)
            per_case[case["id"]].append(actual)
            if actual != case["expected"]:
                report["agreement"] = False
            results.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        report["runs"][batch] = results
    if any(len(set(values)) != 1 for values in per_case.values()):
        report["agreement"] = False
    negatives = [case for case in cases if case["family"] in {"A", "X", "Y-negative"}]
    rejected = sum(per_case[case["id"]][0] == "Violated" for case in negatives)
    survivors = [case["id"] for case in negatives if per_case[case["id"]][0] != "Violated"]
    report["summary"] = {
        "cases": len(cases),
        "valid_negatives": len(negatives),
        "negative_violated": rejected,
        "mutation_score": rejected / len(negatives),
        "preserved_controls": sum(case["expected"] == "Preserved" and per_case[case["id"]][0] == "Preserved" for case in cases),
        "opaque_unknown": sum(case["family"] == "opaque" and per_case[case["id"]][0] == "Unknown" for case in cases),
        "survivors": survivors,
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["agreement"] and not survivors else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Failure as exc:
        print(json.dumps({"schema": "p1342-oracle-run-report-v4", "fatal": str(exc)}, indent=2), file=sys.stderr)
        raise SystemExit(2)
