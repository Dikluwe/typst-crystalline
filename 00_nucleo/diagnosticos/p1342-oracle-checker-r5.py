#!/usr/bin/env python3
"""R5 focal oracle: derive source claims from external fixture bytes.

This diagnostic checker never opens productive source as a candidate.  Its
closed synthetic Rust fixtures model the external baseline/candidate channel;
all structure, region hashes, hook adjacency and diff boundaries are derived
from those bytes before the frozen R4/R3/R2 predicates run.
"""

from __future__ import annotations

import argparse
import copy
import difflib
import hashlib
import importlib.util
import json
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
R4_CHECKER = DIAG / "p1342-oracle-checker-r4.py"
R2_CONTRACT = DIAG / "p1342-contract-spec-r2.json"
R2_BINDING = DIAG / "p1342-contract-binding-r2.json"
EVIDENCE_PATH = "fixture://independent-verifier/p1342-candidate-source-evidence-v1.json"
MODES = ["normal", "repeat", "reverse"]


def load_module(path: Path) -> Any:
    spec = importlib.util.spec_from_file_location("p1342_oracle_checker_r4_frozen_for_r5", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen R4 checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


R4 = load_module(R4_CHECKER)
R3 = R4.R3
Failure = R4.Failure
digest = R4.digest
sha_text = R4.sha_text


def exact_keys(value: Any, keys: set[str], where: str) -> None:
    if not isinstance(value, dict) or set(value) != keys:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        raise Failure(f"{where}: closed keys {sorted(keys)} required, got {got}")


def integer(value: Any, where: str) -> int:
    if type(value) is not int:
        raise Failure(f"{where}: exact integer required; bool is not int")
    return value


def hook_block(hook: str) -> str:
    return (
        "#[cfg(p1339_observation)]\n"
        "{\n"
        f"    p1342_observe(\"{hook}\");\n"
        f"}} /*P1342_HOOK_END:{sha_text(hook)}*/\n"
    )


def marker(kind: str, value: str) -> str:
    return f"/*P1342_{kind}:{sha_text(value)}*/"


def fixture_function(index: int, row: dict[str, Any], include_hook: bool) -> tuple[str, tuple[int, int] | None]:
    prefix = (
        f"fn p1342_row_{index:02d}() {{\n"
        f"{marker('SYMBOL', row['symbol'])}\n"
        f"{marker('BRANCH', row['branch'])}\n"
        f"{marker('ANCHOR', row['anchor_needle'])}\n"
    )
    block = hook_block(row["hook"]) if include_hook else ""
    value = prefix + block + "}\n"
    span = (len(prefix.encode("utf-8")), len((prefix + block).encode("utf-8"))) if include_hook else None
    return value, span


def scan_balanced(text: str) -> dict[int, int]:
    pairs = {"(": ")", "[": "]", "{": "}"}
    stack: list[tuple[str, int]] = []
    matches: dict[int, int] = {}
    index = 0
    state = "code"
    block_depth = 0
    while index < len(text):
        char = text[index]
        nxt = text[index + 1] if index + 1 < len(text) else ""
        if state == "line-comment":
            if char == "\n":
                state = "code"
        elif state == "block-comment":
            if char == "/" and nxt == "*":
                block_depth += 1
                index += 1
            elif char == "*" and nxt == "/":
                block_depth -= 1
                index += 1
                if block_depth == 0:
                    state = "code"
        elif state in {"string", "char"}:
            if char == "\\":
                index += 1
            elif (state == "string" and char == '"') or (state == "char" and char == "'"):
                state = "code"
        else:
            if char == "/" and nxt == "/":
                state = "line-comment"
                index += 1
            elif char == "/" and nxt == "*":
                state = "block-comment"
                block_depth = 1
                index += 1
            elif char == '"':
                state = "string"
            elif char == "'":
                state = "char"
            elif char in pairs:
                stack.append((char, index))
            elif char in pairs.values():
                if not stack or pairs[stack[-1][0]] != char:
                    raise Failure("external candidate Rust fixture has unbalanced delimiters")
                _, opening = stack.pop()
                matches[opening] = index
        index += 1
    if stack or state in {"block-comment", "string", "char"}:
        raise Failure("external candidate Rust fixture has unbalanced delimiters")
    return matches


FUNCTION_RE = re.compile(r"fn p1342_row_(\d{2})\(\) \{")


def parse_fixture(text: str, rows_by_index: dict[int, dict[str, Any]], require_hooks: bool) -> dict[int, dict[str, Any]]:
    brace_pairs = scan_balanced(text)
    parsed: dict[int, dict[str, Any]] = {}
    for found in FUNCTION_RE.finditer(text):
        index = int(found.group(1))
        if index in parsed or index not in rows_by_index:
            raise Failure("external Rust fixture function identity/cardinality mismatch")
        opening = found.end() - 1
        closing = brace_pairs.get(opening)
        if closing is None:
            raise Failure("external Rust fixture function body is structurally incomplete")
        function_text = text[found.start():closing + 1]
        row = rows_by_index[index]
        markers = {
            "symbol": marker("SYMBOL", row["symbol"]),
            "branch": marker("BRANCH", row["branch"]),
            "anchor": marker("ANCHOR", row["anchor_needle"]),
        }
        positions = {}
        for name, value in markers.items():
            if function_text.count(value) != 1:
                raise Failure(f"external Rust fixture {name} marker is not unique in its function")
            local = function_text.index(value)
            positions[name] = (found.start() + local, found.start() + local + len(value))
        if not (positions["symbol"][0] < positions["branch"][0] < positions["anchor"][0]):
            raise Failure("external Rust fixture symbol/branch/anchor order mismatch")
        block = hook_block(row["hook"])
        block_count = function_text.count(block)
        if require_hooks and block_count != 1:
            raise Failure("candidate hook is not directly present in external candidate bytes")
        if not require_hooks and block_count != 0:
            raise Failure("candidate-free baseline fixture contains an observation hook")
        block_span = None
        if require_hooks:
            local = function_text.index(block)
            block_span = (found.start() + local, found.start() + local + len(block))
            if block_span[0] != positions["anchor"][1] + 1:
                raise Failure("candidate hook is not immediately adjacent to the resolved anchor")
        parsed[index] = {
            "function_span": (found.start(), closing + 1),
            "function_text": function_text,
            "anchor_text": markers["anchor"],
            "hook_span": block_span,
        }
    if set(parsed) != set(rows_by_index):
        raise Failure("external Rust fixture does not resolve every expected row function")
    return parsed


def diff_insert_spans(baseline: str, candidate: str) -> list[tuple[int, int]]:
    matcher = difflib.SequenceMatcher(None, baseline.encode("utf-8"), candidate.encode("utf-8"), autojunk=False)
    spans = []
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag == "equal":
            continue
        if tag != "insert" or i1 != i2:
            raise Failure("baseline-to-candidate diff is not a pure cfg-only insertion set")
        spans.append((j1, j2))
    return spans


def build_direct_source_fixture(dto: dict[str, Any], contract_pin: str, binding_pin: str, fixture_pin: str, binding_r3: dict[str, Any], r2_binding: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:
    evidence, _ = R4.build_source_fixture(dto, contract_pin, binding_pin, fixture_pin, binding_r3, r2_binding)
    rows = r2_binding["hooks"]
    groups: dict[str, list[tuple[int, dict[str, Any]]]] = defaultdict(list)
    for index, row in enumerate(rows):
        groups[row["path"]].append((index, row))
    baseline_file_map = {item["path"]: item for item in evidence["baseline_attestation"]["files"]}
    candidate_file_map = {item["path"]: item for item in evidence["files"]}
    boundary_map = {item["id"]: item for item in evidence["allowed_diff_boundaries"]}
    for path, grouped in groups.items():
        baseline_parts = [f"// closed P1342 candidate-free Rust fixture\n// path:{path}\n"]
        candidate_parts = [f"// closed P1342 candidate-free Rust fixture\n// path:{path}\n"]
        candidate_offset = len(candidate_parts[0].encode("utf-8"))
        spans: dict[int, tuple[int, int]] = {}
        for index, row in grouped:
            base_fn, _ = fixture_function(index, row, False)
            cand_fn, local_span = fixture_function(index, row, True)
            baseline_parts.append(base_fn)
            candidate_parts.append(cand_fn)
            assert local_span is not None
            spans[index] = (candidate_offset + local_span[0], candidate_offset + local_span[1])
            candidate_offset += len(cand_fn.encode("utf-8"))
        baseline_text = "".join(baseline_parts)
        candidate_text = "".join(candidate_parts)
        baseline_record = baseline_file_map[path]
        baseline_record["baseline_bytes"] = baseline_text
        baseline_record["baseline_file_sha256"] = sha_text(baseline_text)
        candidate_record = candidate_file_map[path]
        candidate_record["baseline_file_sha256"] = baseline_record["baseline_file_sha256"]
        candidate_record["candidate_bytes"] = candidate_text
        candidate_record["candidate_file_sha256"] = sha_text(candidate_text)
        candidate_record["rust_parse_ok"] = True
        candidate_record["outside_allowed_boundaries_equal"] = True
        base_parsed = parse_fixture(baseline_text, dict(grouped), False)
        cand_parsed = parse_fixture(candidate_text, dict(grouped), True)
        for index, row in grouped:
            item = evidence["rows"][index]
            item["candidate_file_sha256"] = candidate_record["candidate_file_sha256"]
            item["candidate_anchor_snippet_sha256"] = sha_text(cand_parsed[index]["anchor_text"])
            item["candidate_symbol_sha256"] = sha_text(cand_parsed[index]["function_text"])
            boundary = boundary_map[f"YB{index:02d}"]
            boundary["candidate_byte_range"] = list(spans[index])
            boundary["baseline_ast_region_sha256"] = sha_text(base_parsed[index]["function_text"])
            boundary["candidate_ast_region_sha256"] = sha_text(cand_parsed[index]["function_text"])
    R4.refresh_candidate_tree(evidence)
    R4.seal_evidence(evidence)
    invocation = {
        "schema": "p1342-r5-trusted-invocation-fixture-v1",
        "candidate_evidence_path": EVIDENCE_PATH,
        "candidate_evidence_sha256": digest(evidence),
        "channel": "independent-verifier-fixture",
    }
    return evidence, invocation


def validate_direct_source(runtime_evidence: dict[str, Any], evidence: Any, invocation: Any, binding_r3: dict[str, Any], r2_binding: dict[str, Any]) -> None:
    exact_keys(invocation, {"schema", "candidate_evidence_path", "candidate_evidence_sha256", "channel"}, "R5 trusted invocation")
    if invocation["schema"] != "p1342-r5-trusted-invocation-fixture-v1" or invocation["channel"] != "independent-verifier-fixture":
        raise Failure("candidate source evidence did not arrive on the fixed external channel")
    if invocation["candidate_evidence_path"] != EVIDENCE_PATH:
        raise Failure("external candidate evidence path adulterated")
    if invocation["candidate_evidence_sha256"] != digest(evidence):
        raise Failure("external candidate evidence hash adulterated")
    rows = r2_binding["hooks"]
    rows_by_path: dict[str, dict[int, dict[str, Any]]] = defaultdict(dict)
    for index, row in enumerate(rows):
        rows_by_path[row["path"]][index] = row
    baseline_files = {item["path"]: item for item in evidence["baseline_attestation"]["files"]}
    candidate_files = {item["path"]: item for item in evidence["files"]}
    if set(baseline_files) != set(rows_by_path) or set(candidate_files) != set(rows_by_path):
        raise Failure("external baseline/candidate file set differs from composed binding")
    boundaries_by_path: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for boundary in evidence["allowed_diff_boundaries"]:
        boundaries_by_path[boundary["path"]].append(boundary)
    derived_rows: dict[int, dict[str, Any]] = {}
    for path, expected_rows in rows_by_path.items():
        baseline_record = baseline_files[path]
        candidate_record = candidate_files[path]
        baseline_text = baseline_record["baseline_bytes"]
        candidate_text = candidate_record["candidate_bytes"]
        if not isinstance(baseline_text, str) or baseline_record["baseline_file_sha256"] != sha_text(baseline_text):
            raise Failure("historical baseline hash is not derived from external baseline bytes")
        if not isinstance(candidate_text, str) or candidate_record["candidate_file_sha256"] != sha_text(candidate_text):
            raise Failure("candidate_file_sha256 is not derived from external candidate bytes")
        baseline_parsed = parse_fixture(baseline_text, expected_rows, False)
        candidate_parsed = parse_fixture(candidate_text, expected_rows, True)
        derived_rows.update(candidate_parsed)
        actual_spans = sorted(diff_insert_spans(baseline_text, candidate_text))
        declared = sorted(tuple(boundary["candidate_byte_range"]) for boundary in boundaries_by_path[path])
        derived_hook_spans = sorted(value["hook_span"] for value in candidate_parsed.values())
        if actual_spans != declared or declared != derived_hook_spans:
            raise Failure("real baseline-to-candidate diff is not exactly the union of declared cfg-only boundaries")
        for index, base in expected_rows.items():
            parsed_base = baseline_parsed[index]
            parsed_candidate = candidate_parsed[index]
            boundary = next((item for item in boundaries_by_path[path] if item["id"] == f"YB{index:02d}"), None)
            if boundary is None:
                raise Failure("derived hook has no owned allowed boundary")
            start, end = parsed_candidate["hook_span"]
            if candidate_text[start:end] != hook_block(base["hook"]):
                raise Failure("allowed boundary content is not the derived cfg-only hook")
            if boundary["baseline_ast_region_sha256"] != sha_text(parsed_base["function_text"]) or boundary["candidate_ast_region_sha256"] != sha_text(parsed_candidate["function_text"]):
                raise Failure("AST region hashes are not derived from external source bytes")
    overlays = binding_r3["row_overlay"]
    for index, (claim, base, overlay) in enumerate(zip(evidence["rows"], rows, overlays)):
        parsed = derived_rows[index]
        if claim["baseline_anchor_snippet_sha256"] != overlay["baseline_anchor_snippet_sha256"]:
            raise Failure("baseline anchor hash differs from composed binding")
        if claim["candidate_anchor_snippet_sha256"] != sha_text(parsed["anchor_text"]):
            raise Failure("candidate anchor snippet hash is not derived from candidate bytes")
        if claim["candidate_symbol_sha256"] != sha_text(parsed["function_text"]):
            raise Failure("candidate symbol hash is not derived from candidate bytes")
        if claim["anchor_matches_in_symbol_branch"] != 1:
            raise Failure("candidate row claim disagrees with directly parsed unique anchor")
    coverage = evidence["runtime_coverage"]
    if len(coverage["row_coverage"]) != 19:
        raise Failure("runtime coverage does not contain every directly found hook")
    runtime_by_mode = {item["mode"]: item for item in runtime_evidence["runtime_runs"]}
    for index, (coverage_row, base) in enumerate(zip(coverage["row_coverage"], rows)):
        if coverage_row.get("row_index") != index or coverage_row.get("hook") != base["hook"]:
            raise Failure("runtime coverage row does not match directly found hook marker")
        for mode in MODES:
            expected_ref = f"coverage:{mode}:{index}"
            hits = [item for item in runtime_by_mode[mode]["hook_hits"] if item["hook"] == base["hook"]]
            if coverage_row.get(mode) != expected_ref or sum(item["count"] for item in hits if type(item.get("count")) is int) <= 0:
                raise Failure("hook/coverage claim is not backed by a directly found marker and runtime hit")


def internal_r4_invocation(evidence: dict[str, Any]) -> dict[str, Any]:
    source = R4.checker_fixture_source()
    return {
        "schema": "p1342-r4-trusted-invocation-fixture-v1",
        "candidate_evidence_path": EVIDENCE_PATH,
        "candidate_evidence_sha256": digest(evidence),
        "channel": "independent-verifier-fixture",
        "checker_fixture": {"source": source, "sha256": sha_text(source)},
    }


def judge(dto: dict[str, Any], runtime_evidence: dict[str, Any], runtime_pin: str, candidate_evidence: Any, invocation: Any, contract_r3: dict[str, Any], binding_r3: dict[str, Any], r2_contract: dict[str, Any], local_r2_binding: dict[str, Any], exact_r2_binding: dict[str, Any], pins: dict[str, str]) -> str:
    if "candidate_source_evidence" in dto or "candidate_evidence_path" in dto or "candidate_evidence_sha256" in dto:
        raise Failure("DTO/facade may not authenticate candidate source evidence")
    if candidate_evidence is None:
        raise Failure("candidate evidence supplied only by DTO/facade is outside the trusted channel")
    R4.validate_contract_binding(contract_r3, binding_r3, exact_r2_binding, pins)
    validate_direct_source(runtime_evidence, candidate_evidence, invocation, binding_r3, exact_r2_binding)
    return R4.judge(dto, runtime_evidence, runtime_pin, candidate_evidence, internal_r4_invocation(candidate_evidence), contract_r3, binding_r3, r2_contract, local_r2_binding, exact_r2_binding, pins)


def reseal(evidence: dict[str, Any], invocation: dict[str, Any]) -> None:
    R4.refresh_candidate_tree(evidence)
    R4.seal_evidence(evidence)
    invocation["candidate_evidence_sha256"] = digest(evidence)


def replace_first_candidate(evidence: dict[str, Any], invocation: dict[str, Any], new_bytes: str, update_hash: bool = True) -> None:
    target = evidence["files"][0]
    target["candidate_bytes"] = new_bytes
    if update_hash:
        target["candidate_file_sha256"] = sha_text(new_bytes)
        for row in evidence["rows"]:
            if row["path"] == target["path"]:
                row["candidate_file_sha256"] = target["candidate_file_sha256"]
    reseal(evidence, invocation)


def apply_y(case_id: str, dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any]) -> tuple[Any, dict[str, Any]]:
    if case_id == "Y01-live-changed-authorized":
        pass
    elif case_id == "Y02-live-baseline-equal-hook-absent":
        replace_first_candidate(evidence, invocation, evidence["baseline_attestation"]["files"][0]["baseline_bytes"])
        evidence["runtime_coverage"]["row_coverage"][0]["normal"] = None
        reseal(evidence, invocation)
    elif case_id == "Y03-forbidden-live-baseline-comparator":
        invocation["checker_fixture"] = {"decoy": "live==candidate", "effective": "live==baseline"}
    elif case_id == "Y04-evidence-only-in-dto-facade":
        dto["candidate_source_evidence"] = copy.deepcopy(evidence)
        evidence = None
    elif case_id == "Y05-external-evidence-path-hash-adulterated":
        invocation["candidate_evidence_path"] = "fixture://candidate-controlled/forged.json"
        invocation["candidate_evidence_sha256"] = "0" * 64
    elif case_id == "Y06-change-outside-allowed-boundary":
        replace_first_candidate(evidence, invocation, "fn unauthorized_change() {}\n" + evidence["files"][0]["candidate_bytes"])
    else:
        raise Failure(f"unknown R5 Y case {case_id}")
    return evidence, invocation


def apply_z(case_id: str, dto: dict[str, Any], evidence: dict[str, Any], invocation: dict[str, Any], pins: dict[str, str]) -> tuple[Any, dict[str, Any]]:
    del dto
    if case_id == "Z01-live-baseline-via-alias-and-decoy":
        invocation["checker_fixture"] = {"decoy": "live==candidate", "alias": "historical=baseline", "effective": "live==historical"}
    elif case_id == "Z02-baseline-equal-with-fabricated-hooks":
        replace_first_candidate(evidence, invocation, evidence["baseline_attestation"]["files"][0]["baseline_bytes"])
    elif case_id == "Z03-candidate-file-sha-altered":
        evidence["files"][0]["candidate_file_sha256"] = "0" * 64
        reseal(evidence, invocation)
    elif case_id == "Z04-external-evidence-path-swapped":
        invocation["candidate_evidence_path"] = "fixture://candidate-controlled/swapped.json"
    elif case_id == "Z05-external-evidence-hash-swapped":
        invocation["candidate_evidence_sha256"] = "0" * 64
    elif case_id == "Z06-outside-cfg-invalid-rust-self-attested":
        replace_first_candidate(evidence, invocation, "fn unguarded_change( {\n" + evidence["files"][0]["candidate_bytes"])
    elif case_id == "Z07-empty-boundary-over-changed-source":
        target_path = evidence["files"][0]["path"]
        for boundary in evidence["allowed_diff_boundaries"]:
            if boundary["path"] == target_path:
                boundary["candidate_byte_range"] = [0, 0]
        reseal(evidence, invocation)
    elif case_id == "Z08-preseal-pins-r2":
        baseline = evidence["baseline_attestation"]
        baseline["contract_sha256"] = pins["contract_r2"]
        baseline["binding_sha256"] = pins["binding_r2"]
        evidence["preseal"]["sha256"] = digest(baseline)
        reseal(evidence, invocation)
    elif case_id == "Z09-candidate-bytes-hash-diverge":
        replace_first_candidate(evidence, invocation, evidence["files"][0]["candidate_bytes"] + "// divergent bytes\n", update_hash=False)
    elif case_id == "Z10-missing-rust-aware-row":
        evidence["rows"][0]["anchor_matches_in_symbol_branch"] = 0
        reseal(evidence, invocation)
    elif case_id == "Z11-missing-runtime-coverage":
        evidence["runtime_coverage"]["row_coverage"].pop()
        reseal(evidence, invocation)
    elif case_id == "Z12-forged-candidate-anchor-hashes":
        evidence["rows"][0]["candidate_anchor_snippet_sha256"] = "0" * 64
        evidence["rows"][0]["candidate_symbol_sha256"] = "f" * 64
        reseal(evidence, invocation)
    else:
        raise Failure(f"unknown R5 Z case {case_id}")
    return evidence, invocation


def build_case(case_id: str, corpus: dict[str, Any], contract_pin: str, binding_pin: str, fixture_pin: str, binding_r3: dict[str, Any], r2_binding: dict[str, Any], r3_pins: dict[str, str], pins: dict[str, str]) -> tuple[Any, ...]:
    base_id = case_id if case_id.startswith(("P", "A", "X")) else "P01-inspectable-composite"
    dto, runtime_evidence, local_r2_binding, runtime_pin = R3.build_case(base_id, corpus, r2_binding, r3_pins)
    candidate_evidence, invocation = build_direct_source_fixture(dto, contract_pin, binding_pin, fixture_pin, binding_r3, r2_binding)
    if case_id.startswith("Y"):
        candidate_evidence, invocation = apply_y(case_id, dto, candidate_evidence, invocation)
    elif case_id.startswith("Z"):
        candidate_evidence, invocation = apply_z(case_id, dto, candidate_evidence, invocation, pins)
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
    exact_keys(corpus, {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "challenges", "r5_closure", "budget", "cases", "closed_case_order", "scope_exclusions"}, "R5 corpus")
    if corpus["schema"] != "p1342-oracle-corpus-v5" or integer(corpus["revision"], "corpus revision") != 5:
        raise Failure("R5 corpus identity mismatch")
    for pin in corpus["protected_inputs"].values():
        if not isinstance(pin, list) or len(pin) != 2 or hashlib.sha256((ROOT / pin[0]).read_bytes()).hexdigest() != pin[1]:
            raise Failure("R5 protected input changed")
    cases = corpus["cases"]
    if corpus["closed_case_order"] != [case["id"] for case in cases]:
        raise Failure("R5 closed case order mismatch")
    for case in cases:
        exact_keys(case, {"id", "family", "expected", "validity", "purpose"}, f"case {case.get('id', '?')}")
        if case["validity"] != "valid":
            raise Failure("no invalid vector is permitted in R5 corpus")
    if args.focus:
        focal = set(corpus["budget"]["focal_case_ids"])
        cases = [case for case in cases if case["id"] in focal]
        if [case["id"] for case in cases] != corpus["budget"]["focal_case_ids"]:
            raise Failure("R5 focal case order mismatch")
    pins = {
        "contract_r3": hashlib.sha256(args.contract.read_bytes()).hexdigest(),
        "binding_r3": hashlib.sha256(args.binding.read_bytes()).hexdigest(),
        "contract_r2": corpus["protected_inputs"]["contract_r2"][1],
        "binding_r2": corpus["protected_inputs"]["binding_r2"][1],
        "fixture": corpus["protected_inputs"]["fixture"][1],
        "l0_freeze": corpus["protected_inputs"]["l0_freeze"][1],
    }
    if pins["contract_r3"] != corpus["protected_inputs"]["contract_r3_json"][1] or pins["binding_r3"] != corpus["protected_inputs"]["binding_r3"][1]:
        raise Failure("R3 contract/binding invocation pin mismatch")
    R4.validate_contract_binding(contract_r3, binding_r3, r2_binding, pins)
    r3_pins = {"contract": pins["contract_r2"], "binding": pins["binding_r2"], "fixture": pins["fixture"], "l0_freeze": pins["l0_freeze"]}
    orders = {"normal": cases, "repeat": cases, "reverse": list(reversed(cases))}
    outcomes: dict[str, list[str]] = defaultdict(list)
    runs: dict[str, list[dict[str, str]]] = {}
    agreement = True
    for mode, ordered in orders.items():
        results = []
        for case in ordered:
            built = build_case(case["id"], corpus, pins["contract_r3"], pins["binding_r3"], pins["fixture"], binding_r3, r2_binding, r3_pins, pins)
            dto, runtime_evidence, local_binding, runtime_pin, candidate_evidence, invocation = built
            try:
                actual = judge(dto, runtime_evidence, runtime_pin, candidate_evidence, invocation, contract_r3, binding_r3, r2_contract, local_binding, r2_binding, pins)
                witness = "all R5 byte-derived source and inherited runtime predicates passed" if actual == "Preserved" else "deliberate payload opacity after all R5 byte-derived and runtime predicates"
            except Failure as exc:
                actual = "Violated"
                witness = str(exc)
            outcomes[case["id"]].append(actual)
            if actual != case["expected"]:
                agreement = False
            results.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        runs[mode] = results
    if any(len(set(values)) != 1 for values in outcomes.values()):
        agreement = False
    negatives = [case for case in cases if case["family"] in {"A", "X", "Y-negative", "Z"}]
    rejected = sum(outcomes[case["id"]][0] == "Violated" for case in negatives)
    survivors = [case["id"] for case in negatives if outcomes[case["id"]][0] != "Violated"]
    report = {
        "schema": "p1342-oracle-run-report-v5",
        "phase": "focal" if args.focus else "full-final",
        "agreement": agreement,
        "runs": runs,
        "summary": {
            "cases": len(cases),
            "valid_negatives": len(negatives),
            "negative_violated": rejected,
            "mutation_score": rejected / len(negatives),
            "preserved_controls": sum(case["expected"] == "Preserved" and outcomes[case["id"]][0] == "Preserved" for case in cases),
            "opaque_unknown": sum(case["family"] == "opaque" and outcomes[case["id"]][0] == "Unknown" for case in cases),
            "survivors": survivors,
        },
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if agreement and not survivors else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Failure as exc:
        print(json.dumps({"schema": "p1342-oracle-run-report-v5", "fatal": str(exc)}, indent=2), file=sys.stderr)
        raise SystemExit(2)
