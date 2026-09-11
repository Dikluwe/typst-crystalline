#!/usr/bin/env python3
"""Frozen P1349 adversarial authorship, blind to all future P1349 targets.

The authorship CLI accepts only ``--author-check``.  Later verification supplies
whole-file-pinned checker/caller/delivery paths and a separately pinned adapter.
META_TESTS and classification FOCAL cases are deliberately disjoint.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Protocol


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
CHECKER_DEADLINE_NS = 8_000_000_000
SUPERVISOR_DEADLINE_NS = 12_000_000_000

PINS = {
    "step_p1349": (ROOT / "00_nucleo/materialization/typst-passo-1349.md", "071ee50547bab6dc86b84d630b6c501867967121234cce79ab8de14075973a81"),
    "step_p1348": (ROOT / "00_nucleo/materialization/typst-passo-1348.md", "4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7"),
    "step_p1347": (ROOT / "00_nucleo/materialization/typst-passo-1347.md", "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c"),
    "contract_spec_p1349": (DIAG / "p1349-contract-spec-r1.json", "9b3363fe2c60d634a7547357217299a2fc610e04f3c0116b667ce23caf555335"),
    "contract_binding_p1349": (DIAG / "p1349-contract-binding-r1.json", "3db25f33a5af2716b432788cd06e8ec527567584b803def215bd45cad06e29bd"),
    "contract_receipt_p1349": (DIAG / "p1349-contract-receipt-r1.json", "6baf1c42baaea917035e0d54729dba9430c5799c5cd61212f5f31dbce33b6ef6"),
    "verifier_blocker_p1348": (DIAG / "p1348-verifier-blocker-r1.json", "e5b9e9423e2e3ad20064cbc20f2594e25396f7e1bffe20123a4f8392cb8501de"),
    "adversary_suite_p1348": (DIAG / "p1348-adversary-suite-r1.py", "14f3f41b28cfaa927197f5b5767612833e38944a9db74fb9f5ace7ee9b32e30c"),
    "adversary_authorship_p1348": (DIAG / "p1348-adversary-authorship-r1.md", "75f347b9c88dd8c0099814ddbc506e2a962ee916ad7526051871f669fcb7f424"),
    "adversary_authorship_receipt_p1348": (DIAG / "p1348-adversary-authorship-receipt-r1.json", "32ebf58fe116982e6c51190caf198e8346e5e1014bd10eb9d276ce96ac6e7bb6"),
    "contract_spec_p1348": (DIAG / "p1348-contract-spec-r1.json", "e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af"),
    "contract_binding_p1348": (DIAG / "p1348-contract-binding-r1.json", "de05f7c3a33bae7e131bca1aec0774a1fc370cdb3a6f43d50b1e2d050db5785f"),
    "contract_receipt_p1348": (DIAG / "p1348-contract-receipt-r1.json", "1f49bdd80627edb61f115da108eca0b4216050bd81aaba093839e051f31b0daf"),
    "adversary_report_p1346": (DIAG / "p1346-adversary-report-r2.json", "60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6"),
    "adversary_report_md_p1346": (DIAG / "p1346-adversary-report-r2.md", "bd0ada205586718106e2e4851dfb8cfb7f50475967bd272567ef10d92512b466"),
    "adversary_receipt_p1346": (DIAG / "p1346-adversary-receipt-r2.json", "2fdf8efdd0f805c3dc321f34192a01715b273b9b071f51d231b80e71c88115a3"),
}


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def canonical(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def strict_json(raw: bytes, label: str) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in items:
            if key in result:
                raise RuntimeError(f"DUPLICATE_KEY: {label}:{key}")
            result[key] = value
        return result

    try:
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs)
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"SCHEMA: {label}") from exc


META_TESTS = [
    {
        "adapter_operation": "postcommit_fault_in_disposable_process",
        "case_id": "P1348-X06-failure-after-registry-commit",
        "domain": "meta",
        "expected_meta_evidence": {
            "observed_blocker": {"blocker_code": "ADAPTER_EXCEPTION", "outcome_kind": "integration_blocker", "verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION"},
            "postconditions": {"identity_state": "CONSUMED", "process_tree_empty": True, "registry_commit_ordinal_unchanged": True, "registry_digest_unchanged": True, "reopened": False, "restoration_identity": True, "second_acceptance": False},
        },
        "expected_meta_status": "PASS",
        "phase": "G04_META",
        "score_eligible": False,
    },
    {
        "adapter_operation": "historical_nameerror_in_disposable_process",
        "case_id": "P1348-X15-historical-original-open-nameerror",
        "domain": "meta",
        "expected_meta_evidence": {
            "observed_blocker": {"blocker_code": "ADAPTER_EXCEPTION", "exception_type": "NameError", "outcome_kind": "integration_blocker", "verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION"},
            "postconditions": {"classification_absent": True, "open_captured_before_patching": True, "open_identity_before_equals_after": True, "process_tree_empty": True, "reason_code_absent": True, "restoration_completed": True},
        },
        "expected_meta_status": "PASS",
        "phase": "G04_META",
        "score_eligible": False,
    },
]


def focal_case(identifier: str, family: str, operation: str, reason: str, **parameters: Any) -> dict[str, Any]:
    return {
        "adapter_operation": operation,
        "case_id": identifier,
        "domain": "classification",
        "expected_classification": "Violated",
        "expected_reason_code": reason,
        "family": family,
        "forbidden_result_domains": ["meta_result", "integration_blocker"],
        "parameters": parameters,
        "phase": "G05_SUPERVISOR_FOCAL",
        "score_eligible": True,
        "valid": True,
    }


NEW_FOCAL_VARIANTS = [
    focal_case("P1349-S01-second-exec-after-exit-event", "trace-sequence", "second_exec_after_exit_event", "PROBE_AUTHORITY", event_sequence=["EXEC", "EXIT", "EXEC"], checker_deadline_ns=CHECKER_DEADLINE_NS),
    focal_case("P1349-S02-second-exec-deadline-edge", "trace-deadline", "second_exec_near_checker_deadline", "PROBE_AUTHORITY", inject_before_deadline_ns=1_000_000, external_signal_forbidden=True),
    focal_case("P1349-S03-second-exec-ignores-graceful-stop", "trace-deadline", "second_exec_ignores_graceful_termination", "PROBE_AUTHORITY", checker_must_force_reap=True, supervisor_deadline_ns=SUPERVISOR_DEADLINE_NS),
    focal_case("P1349-S04-pidfd-diverges-after-handshake", "process-authority", "substitute_checker_pidfd_after_supervisor_handshake", "WORKER_AUTHORITY", supervisor_pidfd_unchanged=True),
    focal_case("P1349-S05-reused-pid-starttime-mismatch", "process-authority", "reuse_pid_with_changed_starttime", "WORKER_AUTHORITY", retained_pidfd_required=True),
    focal_case("P1349-S06-sibling-pidfd-stream-status", "process-authority", "mix_sibling_pidfd_stream_and_status", "WORKER_AUTHORITY", stdout="canonical"),
    focal_case("P1349-S07-detach-after-first-exec", "trace-lifetime", "detach_after_first_exec_before_exit", "PROBE_AUTHORITY", cleanup_by_checker_required=True),
    focal_case("P1349-S08-detach-at-exit-before-terminal", "trace-lifetime", "detach_at_exit_event_before_terminal", "PROBE_AUTHORITY", terminal_wait_untrusted=True),
    focal_case("P1349-S09-mutant-anchor-authentic-streams", "dag-authority", "substitute_anchor_with_authentic_validator_streams", "AUTHORITY_ROOT", validator_called=True),
    focal_case("P1349-S10-valid-schema-self-answer", "dag-self-judgment", "self_answer_without_real_validator", "DAG_VALIDATOR", superficial_schema_valid=True),
    focal_case("P1349-S11-unexpected-blocker-cannot-mask-second-exec", "control-plane-terminality", "second_exec_with_unexpected_adapter_failure", "PROBE_AUTHORITY", adapter_exception_forbidden=True, any_blocker_is_terminal=True),
]


@dataclass(frozen=True)
class FrozenTargets:
    checker: Path
    checker_sha256: str
    caller: Path
    caller_sha256: str
    delivery_receipt: Path
    delivery_receipt_sha256: str


class VerifierAdapter(Protocol):
    def exercise_meta(self, case: dict[str, Any], targets: FrozenTargets) -> dict[str, Any]: ...
    def exercise_focal(self, case: dict[str, Any], targets: FrozenTargets, exact_ids: list[str]) -> dict[str, Any]: ...


def load_p1348_suite() -> Any:
    path, digest = PINS["adversary_suite_p1348"]
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("PROTECTED_INPUT: P1348 adversary suite")
    spec = importlib.util.spec_from_file_location("p1348_frozen_adversary_baseline", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("PROTECTED_INPUT: P1348 suite loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def frozen_classification_slices() -> dict[str, list[dict[str, Any]]]:
    base = load_p1348_suite()
    slices = base.frozen_slices()
    prior_focal = slices["survivors"] + slices["new_variants"]
    by_id = {item["case_id"]: item for item in prior_focal}
    meta_ids = {item["case_id"] for item in META_TESTS}
    x14_id = "P1348-X14-second-exec-before-exit"
    if len(by_id) != 25 or not meta_ids.issubset(by_id) or x14_id not in by_id:
        raise RuntimeError("PROTECTED_INPUT: P1348 first focal")
    x14 = dict(by_id[x14_id])
    x14.update(domain="classification", forbidden_result_domains=["meta_result", "integration_blocker"], phase="G05_SUPERVISOR_FOCAL", score_eligible=True)
    others = []
    for item in prior_focal:
        if item["case_id"] in meta_ids | {x14_id}:
            continue
        inherited = dict(item)
        inherited.update(domain="classification", forbidden_result_domains=["meta_result", "integration_blocker"], phase="G06_OTHER_P1348_FOCALS", score_eligible=True)
        inherited.pop("expected_outcome_kind", None)
        inherited.pop("expected_blocker_verdict", None)
        others.append(inherited)
    regressions = [dict(item, domain="classification", forbidden_result_domains=["meta_result", "integration_blocker"], phase="G07_P1347_REGRESSIONS", score_eligible=True) for item in slices["regressions"]]
    controls = [dict(item, domain="classification", forbidden_result_domains=["meta_result", "integration_blocker"], phase="G08_CONTROLS", score_eligible=False) for item in slices["controls"]]
    replays = [dict(item, domain="classification", forbidden_result_domains=["meta_result", "integration_blocker"], phase="G08_P1346_REPLAYS", score_eligible=True) for item in base.p1346_replays()]
    return {"x14": [x14], "new_variants": list(NEW_FOCAL_VARIANTS), "other_p1348": others, "p1347_regressions": regressions, "p1346_replays": replays, "controls": controls}


def exact_language_ids() -> list[str]:
    """Resolved only by a later verifier, never by authorship mode."""
    base = load_p1348_suite()
    return base.load_p1347_suite().frozen_ids()


def validate_authorship() -> dict[str, Any]:
    for label, (path, digest) in PINS.items():
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label}")
    binding = strict_json(PINS["contract_binding_p1349"][0].read_bytes(), "P1349 binding")
    blocker = strict_json(PINS["verifier_blocker_p1348"][0].read_bytes(), "P1348 blocker")
    slices = frozen_classification_slices()
    expected_meta_ids = binding["meta_allowlist"]["exact_order"]
    if [item["case_id"] for item in META_TESTS] != expected_meta_ids or len(META_TESTS) != 2:
        raise RuntimeError("SCHEMA: exact X06/X15 meta allowlist")
    if any(item.get("score_eligible") is not False or item.get("domain") != "meta" for item in META_TESTS):
        raise RuntimeError("SCHEMA: meta tests outside score")
    if blocker["decisive_blocker"]["case_id"] != expected_meta_ids[0] or blocker["additional_observations_after_logical_stop"]["x15"]["case_id"] != expected_meta_ids[1]:
        raise RuntimeError("PROTECTED_INPUT: P1348 meta observations")
    if blocker["additional_observations_after_logical_stop"]["x14"]["case_id"] != "P1348-X14-second-exec-before-exit":
        raise RuntimeError("PROTECTED_INPUT: P1348 X14 observation")
    cardinalities = {key: len(value) for key, value in slices.items()}
    expected = {"x14": 1, "new_variants": 11, "other_p1348": 22, "p1347_regressions": 39, "p1346_replays": 181, "controls": 11}
    if cardinalities != expected:
        raise RuntimeError(f"SCHEMA: classification slice cardinality {cardinalities}")
    all_cases = META_TESTS + [item for values in slices.values() for item in values]
    all_ids = [item["case_id"] for item in all_cases]
    if len(all_ids) != len(set(all_ids)):
        raise RuntimeError("SCHEMA: duplicate case identity")
    scored = sum(1 for item in all_cases if item.get("score_eligible") is True)
    if scored != 254:
        raise RuntimeError("SCHEMA: expected denominator 254")
    return {
        "candidate_read_or_executed": False,
        "controls_bound": 11,
        "focal_valid_negatives_bound": scored,
        "full_runs": 0,
        "meta_tests": 2,
        "new_p1349_variants": 11,
        "oracle_or_adapter_read_or_executed": False,
        "p1346_replays_bound": 181,
        "p1347_regressions_bound": 39,
        "p1348_classification_focals_bound": 23,
        "ptrace_runs": 0,
        "sequence_count": 122,
        "sequence_sha256": SEQUENCE_SHA256,
        "verdict": "AUTHORED_NOT_EXECUTED",
    }


def load_adapter(path: Path, digest: str) -> VerifierAdapter:
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("AUTHORITY_ROOT: frozen adapter")
    spec = importlib.util.spec_from_file_location("p1349_frozen_verifier_adapter", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: adapter loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    if not hasattr(module, "exercise_meta") or not hasattr(module, "exercise_focal"):
        raise RuntimeError("SCHEMA: disjoint adapter APIs required")
    return module


def classification_matches(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    expected_reason = case.get("expected_reason_code")
    return (
        observed.get("schema") == "p1349-classification-result-r1"
        and observed.get("case_id") == case["case_id"]
        and observed.get("classification") == case["expected_classification"]
        and (expected_reason is None or observed.get("reason_code") == expected_reason)
        and "meta_status" not in observed
        and "blocker_code" not in observed
        and "verdict" not in observed
    )


def meta_matches(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    return (
        observed.get("schema") == "p1349-meta-result-r1"
        and observed.get("case_id") == case["case_id"]
        and observed.get("meta_status") == case["expected_meta_status"]
        and observed.get("meta_evidence") == case["expected_meta_evidence"]
        and "classification" not in observed
        and "reason_code" not in observed
        and "blocker_code" not in observed
        and "verdict" not in observed
    )


def run_focal_slice(adapter: VerifierAdapter, cases: list[dict[str, Any]], targets: FrozenTargets, exact_ids: list[str]) -> tuple[list[dict[str, Any]], list[str], int]:
    records = [adapter.exercise_focal(case, targets, exact_ids) for case in cases]
    failures = [case["case_id"] for case, observed in zip(cases, records) if not classification_matches(case, observed)]
    blockers = sum(1 for observed in records if observed.get("schema") == "p1349-integration-blocker-r1" or "blocker_code" in observed)
    return records, failures, blockers


def run_with_adapter(targets: FrozenTargets, adapter_path: Path, adapter_sha256: str) -> dict[str, Any]:
    """Future verifier entrypoint; deliberately unreachable from the CLI."""
    validate_authorship()
    for path, digest in [(targets.checker, targets.checker_sha256), (targets.caller, targets.caller_sha256), (targets.delivery_receipt, targets.delivery_receipt_sha256)]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError("AUTHORITY_ROOT: frozen target drift")
    adapter = load_adapter(adapter_path, adapter_sha256)
    meta_records = [adapter.exercise_meta(case, targets) for case in META_TESTS]
    meta_failures = [case["case_id"] for case, observed in zip(META_TESTS, meta_records) if not meta_matches(case, observed)]
    result: dict[str, Any] = {"meta": meta_records, "meta_failures": meta_failures, "full_runs": 0, "unexpected_integration_blockers": 0}
    if meta_failures:
        result["verdict"] = "META_GATE_STOP"
        return result
    slices = frozen_classification_slices()
    ids = exact_language_ids()
    stages = [
        ("supervisor_focal", slices["x14"] + slices["new_variants"]),
        ("other_p1348", slices["other_p1348"]),
        ("p1347_regressions", slices["p1347_regressions"]),
        ("p1346_replays", slices["p1346_replays"]),
        ("controls", slices["controls"]),
    ]
    for label, cases in stages:
        records, failures, blockers = run_focal_slice(adapter, cases, targets, ids)
        result[label] = records
        result["unexpected_integration_blockers"] += blockers
        if failures:
            result["failures"] = failures
            result["verdict"] = "FOCAL_TERMINAL_STOP"
            return result
    result["verdict"] = "ZERO_SURVIVORS_PENDING_INDEPENDENT_COUNTING"
    return result


def main() -> int:
    if sys.argv[1:] != ["--author-check"]:
        raise RuntimeError("SCHEMA: authorship CLI accepts exactly --author-check")
    sys.stdout.buffer.write(canonical(validate_authorship()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
