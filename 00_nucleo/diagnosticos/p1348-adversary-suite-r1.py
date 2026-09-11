#!/usr/bin/env python3
"""Oracle/adapter-blind adversarial authorship suite for P1348.

Only ``--author-check`` is executable during authorship.  A later independent
verifier supplies whole-file-pinned targets and an adapter, then calls
``run_with_adapter``.  This module does not discover or import any P1348 target.
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

PINS = {
    "step_p1348": (ROOT / "00_nucleo/materialization/typst-passo-1348.md", "4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7"),
    "step_p1347": (ROOT / "00_nucleo/materialization/typst-passo-1347.md", "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c"),
    "contract_spec_p1348": (DIAG / "p1348-contract-spec-r1.json", "e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af"),
    "contract_binding_p1348": (DIAG / "p1348-contract-binding-r1.json", "de05f7c3a33bae7e131bca1aec0774a1fc370cdb3a6f43d50b1e2d050db5785f"),
    "contract_receipt_p1348": (DIAG / "p1348-contract-receipt-r1.json", "1f49bdd80627edb61f115da108eca0b4216050bd81aaba093839e051f31b0daf"),
    "verifier_blocker_p1347": (DIAG / "p1347-verifier-blocker-r1.json", "3b649ced677059a459262710ce3466cb2efbd69edbfb6e60c98df07cffa64522"),
    "adversary_suite_p1347": (DIAG / "p1347-adversary-suite-r1.py", "d51fa54da1c68dcc5c812847b90097dfe94f1b4eb83f1009294230eb5f37dd4c"),
    "adversary_authorship_p1347": (DIAG / "p1347-adversary-authorship-r1.md", "dd2d67799a989eb8910e3102aa8d066b1d376ed5895d061455f54e61101b99da"),
    "adversary_authorship_receipt_p1347": (DIAG / "p1347-adversary-authorship-receipt-r1.json", "fd5aad168f4c22e5cc781456f7b000df0c542da6637a4a55459d5ccbccad3aa0"),
    "adversary_report_p1346": (DIAG / "p1346-adversary-report-r2.json", "60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6"),
    "adversary_report_md_p1346": (DIAG / "p1346-adversary-report-r2.md", "bd0ada205586718106e2e4851dfb8cfb7f50475967bd272567ef10d92512b466"),
    "adversary_receipt_p1346": (DIAG / "p1346-adversary-receipt-r2.json", "2fdf8efdd0f805c3dc321f34192a01715b273b9b071f51d231b80e71c88115a3"),
}

SURVIVOR_BINDINGS = [
    ("P1347-I04-forged-dag-worker", "DAG_VALIDATOR"),
    ("P1347-W11-pid-sibling", "WORKER_AUTHORITY"),
    ("P1347-W16-partial-stdout", "WORKER_AUTHORITY"),
    ("P1347-W17-extra-stdout-line", "WORKER_AUTHORITY"),
    ("P1347-W21-challenge-replay", "WORKER_AUTHORITY"),
    ("P1347-D01-labels-without-validator", "DAG_VALIDATOR"),
    ("P1347-D03-mutant-provided-anchor", "AUTHORITY_ROOT"),
    ("P1347-D06-coordinated-anchor-substitution", "AUTHORITY_ROOT"),
    ("P1347-P04-swap-after-exec-stop", "PROBE_AUTHORITY"),
    ("P1347-P17-tracer-detached", "PROBE_AUTHORITY"),
]


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


def classification_case(identifier: str, family: str, operation: str, reason: str, **parameters: Any) -> dict[str, Any]:
    return {
        "adapter_operation": operation,
        "case_id": identifier,
        "expected_classification": "Violated",
        "expected_outcome_kind": "classification",
        "expected_reason_code": reason,
        "family": family,
        "parameters": parameters,
        "phase": "p1348_first_focal",
        "valid": True,
    }


def blocker_case(identifier: str, family: str, operation: str, expected_verdict: str | None = None, **parameters: Any) -> dict[str, Any]:
    case = {
        "adapter_operation": operation,
        "case_id": identifier,
        "expected_outcome_kind": "integration_blocker",
        "family": family,
        "parameters": parameters,
        "phase": "p1348_first_focal",
        "valid": True,
    }
    if expected_verdict is not None:
        case["expected_blocker_verdict"] = expected_verdict
    return case


NEW_VARIANTS = [
    classification_case("P1348-X01-coordinated-pid-pidfd-substitution", "parent-transaction", "substitute_pid_and_pidfd", "WORKER_AUTHORITY", stdout="canonical", waitid="coordinated"),
    classification_case("P1348-X02-stale-terminated-or-reused-pid", "parent-transaction", "reuse_terminated_child_pid", "WORKER_AUTHORITY", variants=["terminated", "kernel_reused_if_observed"]),
    classification_case("P1348-X03-same-identity-double-validation", "freshness", "validate_same_identity_twice", "WORKER_AUTHORITY", expected_observations=[["Preserved", "PRESERVED"], ["Violated", "WORKER_AUTHORITY"]], registry_states=["CONSUMED", "CONSUMED"]),
    classification_case("P1348-X04-validation-order-swap", "freshness", "swap_validation_slots", "WORKER_AUTHORITY", schedule="second_before_first", registry_commit_count=0),
    classification_case("P1348-X05-failure-before-registry-commit", "freshness", "fail_then_retry_before_commit", "WORKER_AUTHORITY", first_failure="partial_stdout", expected_observations=[["Violated", "WORKER_AUTHORITY"], ["Preserved", "PRESERVED"]], registry_states=["ABSENT", "CONSUMED"]),
    blocker_case("P1348-X06-failure-after-registry-commit", "freshness", "fail_after_registry_commit", expected_postconditions={"identity": "CONSUMED", "reopened": False}, replay_expected=["Violated", "WORKER_AUTHORITY"]),
    classification_case("P1348-X07-payload-or-adapter-registry", "freshness-authority", "inject_registry", "WORKER_AUTHORITY", sources=["payload", "adapter"], actions=["seed", "clear", "replace"]),
    classification_case("P1348-X08-superficially-valid-dag-self-answer", "dag-self-judgment", "dag_valid_schema_self_answer", "DAG_VALIDATOR", answer_keys=["classification", "reason_code"], validator_called=False),
    classification_case("P1348-X09-validator-authority-root-altered-stream", "dag-transport-precedence", "alter_validator_stream_with_authority_root", "WORKER_AUTHORITY", validator_reason="AUTHORITY_ROOT", altered=["stdout", "stderr"]),
    classification_case("P1348-X10-traceexit-option-removed", "trace-lifecycle", "remove_trace_option", "PROBE_AUTHORITY", option="PTRACE_O_TRACEEXIT", expected_mask=1048656),
    classification_case("P1348-X11-detach-at-exec-stop", "trace-lifecycle", "detach_at_event", "PROBE_AUTHORITY", event="PTRACE_EVENT_EXEC"),
    classification_case("P1348-X12-detach-before-exit-event", "trace-lifecycle", "detach_before_event", "PROBE_AUTHORITY", event="PTRACE_EVENT_EXIT"),
    classification_case("P1348-X13-terminal-without-exit-event", "trace-lifecycle", "fabricate_terminal_status", "PROBE_AUTHORITY", exec_events=1, exit_events=0),
    classification_case("P1348-X14-second-exec-before-exit", "trace-lifecycle", "inject_second_exec", "PROBE_AUTHORITY", event_sequence=["EXEC", "EXEC", "EXIT"]),
    blocker_case("P1348-X15-historical-original-open-nameerror", "adapter-exception", "raise_historical_original_open_nameerror", expected_verdict="ADAPTER_EXCEPTION_BLOCKS_INTEGRATION", exception_type="NameError", exception_phase="mutation", restoration_required=True),
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
    def exercise(self, case: dict[str, Any], targets: FrozenTargets, order: str) -> dict[str, Any]: ...


def load_p1347_suite() -> Any:
    path, digest = PINS["adversary_suite_p1347"]
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("PROTECTED_INPUT: P1347 adversary suite")
    spec = importlib.util.spec_from_file_location("p1347_frozen_adversary_baseline", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("PROTECTED_INPUT: P1347 suite loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def frozen_slices() -> dict[str, list[dict[str, Any]]]:
    base = load_p1347_suite()
    by_id = {item["case_id"]: item for item in base.ATTACKS}
    survivor_ids = [item[0] for item in SURVIVOR_BINDINGS]
    if len(by_id) != 49 or any(identifier not in by_id for identifier in survivor_ids):
        raise RuntimeError("PROTECTED_INPUT: P1347 focal identity")
    survivors = []
    for identifier, reason in SURVIVOR_BINDINGS:
        inherited = dict(by_id[identifier])
        inherited.update(expected_classification="Violated", expected_outcome_kind="classification", expected_reason_code=reason, phase="p1348_first_focal")
        survivors.append(inherited)
    regressions = [dict(item, phase="p1347_focal_regression") for item in base.ATTACKS if item["case_id"] not in set(survivor_ids)]
    controls = [dict(item, phase="p1347_control") for item in base.CONTROLS]
    for item in controls:
        item.setdefault("expected_outcome_kind", "classification")
        if item["case_id"] == "P1347-C07-authenticated-opaque-probe":
            item["expected_reason_code"] = "OPAQUE_PAYLOAD"
    return {"survivors": survivors, "new_variants": list(NEW_VARIANTS), "regressions": regressions, "controls": controls}


def p1346_replays() -> list[dict[str, Any]]:
    report = strict_json(PINS["adversary_report_p1346"][0].read_bytes(), "P1346 adversary report")
    rows = report["vectors"]["canonical_inherited"] + report["vectors"]["canonical_transport"] + report["vectors"]["replayed_r1_and_new"]
    killed = [item for item in rows if item.get("actual") == "Violated"]
    if len(killed) != 181:
        raise RuntimeError("PROTECTED_INPUT: 181 killed P1346 replays")
    return [{
        "adapter_operation": "replay_p1346_negative",
        "case_id": f"P1348-R{index:03d}-{item['id']}",
        "expected_classification": "Violated",
        "expected_outcome_kind": "classification",
        "expected_reason_code": item.get("reason_code", "PROTECTED_INPUT"),
        "family": "p1346-regression",
        "parameters": {"source_case_id": item["id"]},
        "phase": "p1346_regression",
        "valid": True,
    } for index, item in enumerate(killed, 1)]


def validate_authorship() -> dict[str, Any]:
    for label, (path, digest) in PINS.items():
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label}")
    binding = strict_json(PINS["contract_binding_p1348"][0].read_bytes(), "P1348 binding")
    blocker = strict_json(PINS["verifier_blocker_p1347"][0].read_bytes(), "P1347 blocker")
    slices = frozen_slices()
    replays = p1346_replays()
    expected_survivors = [item[0] for item in SURVIVOR_BINDINGS]
    measured_survivors = [item["case_id"] for item in blocker["survivors"]]
    if measured_survivors != expected_survivors or blocker["execution"]["correct_expected_classification_and_reason"] != 39:
        raise RuntimeError("PROTECTED_INPUT: exact P1347 survivor vector")
    bound_ten = binding["focal_binding"]["ten_replays"]
    if [(item[0], item[2]) for item in bound_ten] != SURVIVOR_BINDINGS:
        raise RuntimeError("PROTECTED_INPUT: P1348 ten survivor bindings")
    if len(NEW_VARIANTS) != 15 or [item["case_id"] for item in NEW_VARIANTS] != [f"P1348-X{index:02d}-{suffix}" for index, suffix in enumerate([
        "coordinated-pid-pidfd-substitution", "stale-terminated-or-reused-pid", "same-identity-double-validation", "validation-order-swap", "failure-before-registry-commit", "failure-after-registry-commit", "payload-or-adapter-registry", "superficially-valid-dag-self-answer", "validator-authority-root-altered-stream", "traceexit-option-removed", "detach-at-exec-stop", "detach-before-exit-event", "terminal-without-exit-event", "second-exec-before-exit", "historical-original-open-nameerror"], 1)]:
        raise RuntimeError("SCHEMA: exact P1348 new variants")
    cardinalities = {key: len(value) for key, value in slices.items()}
    if cardinalities != {"survivors": 10, "new_variants": 15, "regressions": 39, "controls": 11} or len(replays) != 181:
        raise RuntimeError("SCHEMA: frozen slice cardinality")
    all_ids = [item["case_id"] for values in slices.values() for item in values] + [item["case_id"] for item in replays]
    if len(all_ids) != len(set(all_ids)):
        raise RuntimeError("SCHEMA: duplicate adversarial identity")
    return {
        "candidate_read_or_executed": False,
        "controls_bound": 11,
        "first_focal_attacks": 25,
        "full_runs": 0,
        "new_variants": 15,
        "oracle_or_adapter_read_or_executed": False,
        "p1346_replays_bound": 181,
        "p1347_regressions_bound": 39,
        "p1347_survivors_bound": 10,
        "ptrace_runs": 0,
        "sequence_count": 122,
        "sequence_sha256": SEQUENCE_SHA256,
        "verdict": "AUTHORED_NOT_EXECUTED",
    }


def load_adapter(path: Path, digest: str) -> VerifierAdapter:
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("AUTHORITY_ROOT: frozen verifier adapter")
    spec = importlib.util.spec_from_file_location("p1348_frozen_verifier_adapter", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: adapter loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    if not hasattr(module, "exercise"):
        raise RuntimeError("SCHEMA: adapter.exercise missing")
    return module


def matches_expected(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    if case["expected_outcome_kind"] == "integration_blocker":
        if observed.get("outcome_kind") != "integration_blocker" or observed.get("classification") is not None:
            return False
        expected_verdict = case.get("expected_blocker_verdict")
        if expected_verdict is not None and observed.get("verdict") != expected_verdict:
            return False
        expected_postconditions = case.get("parameters", {}).get("expected_postconditions")
        return expected_postconditions is None or observed.get("postconditions") == expected_postconditions
    if "expected_observations" in case.get("parameters", {}):
        return observed.get("observations") == case["parameters"]["expected_observations"]
    return observed.get("classification") == case["expected_classification"] and observed.get("reason_code") == case.get("expected_reason_code")


def run_slice(adapter: VerifierAdapter, cases: list[dict[str, Any]], targets: FrozenTargets, order: str) -> tuple[list[dict[str, Any]], list[str]]:
    records = [adapter.exercise(case, targets, order) for case in cases]
    failures = [case["case_id"] for case, observed in zip(cases, records) if not matches_expected(case, observed)]
    return records, failures


def run_with_adapter(targets: FrozenTargets, adapter_path: Path, adapter_sha256: str, order: str = "normal") -> dict[str, Any]:
    """Future verifier entrypoint. It is deliberately unreachable from the CLI."""
    validate_authorship()
    if order not in {"normal", "repeat", "reverse"}:
        raise RuntimeError("SCHEMA: execution order")
    for path, digest in [(targets.checker, targets.checker_sha256), (targets.caller, targets.caller_sha256), (targets.delivery_receipt, targets.delivery_receipt_sha256)]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError("AUTHORITY_ROOT: frozen target drift")
    adapter = load_adapter(adapter_path, adapter_sha256)
    slices = frozen_slices()
    first_cases = slices["survivors"] + slices["new_variants"]
    if order == "reverse":
        first_cases = list(reversed(first_cases))
    first_records, failures = run_slice(adapter, first_cases, targets, order)
    regression_records: list[dict[str, Any]] = []
    replay_records: list[dict[str, Any]] = []
    control_records: list[dict[str, Any]] = []
    if not failures:
        regression_records, failures = run_slice(adapter, slices["regressions"], targets, order)
    if not failures:
        replay_records, failures = run_slice(adapter, p1346_replays(), targets, order)
    if not failures:
        control_records, failures = run_slice(adapter, slices["controls"], targets, order)
    return {
        "controls": control_records,
        "first_focal": first_records,
        "full_runs": 0,
        "order": order,
        "p1346_replays": replay_records,
        "p1347_regressions": regression_records,
        "survivors_or_blockers": failures,
        "verdict": "FOCAL_ZERO_SURVIVORS" if not failures else "SURVIVORS_OR_BLOCKERS_STOP",
    }


def main() -> int:
    if sys.argv[1:] != ["--author-check"]:
        raise RuntimeError("SCHEMA: authorship CLI accepts exactly --author-check")
    sys.stdout.buffer.write(canonical(validate_authorship()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
