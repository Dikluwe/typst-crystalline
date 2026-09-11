#!/usr/bin/env python3
"""Frozen, oracle-blind adversarial suite for P1347.

Authorship mode validates only the P1347 step/contract and failed P1346
baseline.  It never discovers, imports or executes a P1347 oracle.  A later
independent verifier may call ``run_with_adapter`` with externally frozen
checker/caller/delivery and adapter hashes.
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
P1346_SURVIVORS = [
    "P1346-R2A01-forged-worker-all-violated",
    "P1346-R2A02-forged-worker-duplicate-ids",
    "P1346-R2A03-forged-worker-pid-string",
    "P1346-R2A07-forged-dag-worker",
    "P1346-R2A16-postcheck-execve-fd-swap",
]
PINS = {
    "step_p1347": (ROOT / "00_nucleo/materialization/typst-passo-1347.md", "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c"),
    "contract_spec_p1347": (DIAG / "p1347-contract-spec-r1.json", "fac6e30c7bbee3b50eec6cad8c8a9a8b30f3c09027f63293e73b4bf2341db0ea"),
    "contract_binding_p1347": (DIAG / "p1347-contract-binding-r1.json", "de1dcc87e77ab5fd6c8395fa4faacfd74fd90769dcafb5d2ef5c0bd30fda5fd1"),
    "contract_receipt_p1347": (DIAG / "p1347-contract-receipt-r1.json", "85c6ad312cf9ac83ff653e2a19ac96fdd9e29ad72d2eb4af90fe8fff14fafb1d"),
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
        out: dict[str, Any] = {}
        for key, value in items:
            if key in out:
                raise RuntimeError(f"DUPLICATE_KEY: {label}:{key}")
            out[key] = value
        return out
    try:
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs)
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"SCHEMA: {label}") from exc


def attack(identifier: str, family: str, operation: str, reason: str, **parameters: Any) -> dict[str, Any]:
    return {
        "adapter_operation": operation,
        "case_id": identifier,
        "expected_classification": "Violated",
        "expected_reason_code": reason,
        "family": family,
        "parameters": parameters,
        "phase": "focal_transport",
        "valid": True,
    }


ATTACKS = [
    # Five inherited survivors, frozen independently of any P1347 oracle.
    attack("P1347-I01-forged-worker-all-violated", "inherited-worker-authenticity", "stdout_without_child", "WORKER_AUTHORITY", inherited="P1346-R2A01-forged-worker-all-violated", ids="exact", classifications="all_violated"),
    attack("P1347-I02-forged-worker-duplicate-ids", "inherited-case-identity", "mutate_id_sequence", "PROTECTED_INPUT", inherited="P1346-R2A02-forged-worker-duplicate-ids", mutation="duplicate"),
    attack("P1347-I03-forged-worker-pid-string", "inherited-process-identity", "inject_worker_pid", "SCHEMA", inherited="P1346-R2A03-forged-worker-pid-string", value="not-a-pid"),
    attack("P1347-I04-forged-dag-worker", "inherited-dag-self-judgment", "dag_labels_without_validator", "DAG_VALIDATOR", inherited="P1346-R2A07-forged-dag-worker"),
    attack("P1347-I05-postcheck-exec-fd-swap", "inherited-exec-image", "swap_fd_inside_exec", "PROBE_AUTHORITY", inherited="P1346-R2A16-postcheck-execve-fd-swap", image="canonical_output_imitator"),
    # Parent-owned worker and exact identity.
    attack("P1347-W01-correct-stdout-copied-nonce-no-child", "worker-authenticity", "stdout_without_child", "WORKER_AUTHORITY", ids="exact", nonce="copied", response="canonical"),
    attack("P1347-W02-id-omitted", "case-identity", "mutate_id_sequence", "PROTECTED_INPUT", mutation="omit", ordinal=61),
    attack("P1347-W03-id-extra", "case-identity", "mutate_id_sequence", "PROTECTED_INPUT", mutation="extra", value="P1347-EXTRA"),
    attack("P1347-W04-id-duplicated", "case-identity", "mutate_id_sequence", "PROTECTED_INPUT", mutation="duplicate", ordinal=0),
    attack("P1347-W05-id-reordered", "case-identity", "mutate_id_sequence", "PROTECTED_INPUT", mutation="swap", ordinals=[0, 1]),
    attack("P1347-W06-id-unicode-alias", "case-identity", "mutate_id_sequence", "PROTECTED_INPUT", mutation="unicode_confusable", ordinal=0, replacement="P1344-A01-content-token-without-matcһ"),
    attack("P1347-W07-pid-bool", "process-identity", "inject_worker_pid", "SCHEMA", value=True),
    attack("P1347-W08-pid-zero", "process-identity", "inject_worker_pid", "SCHEMA", value=0),
    attack("P1347-W09-pid-negative", "process-identity", "inject_worker_pid", "SCHEMA", value=-1),
    attack("P1347-W10-pid-string", "process-identity", "inject_worker_pid", "SCHEMA", value="31337"),
    attack("P1347-W11-pid-sibling", "process-identity", "substitute_process_identity", "WORKER_AUTHORITY", source="sibling_pid"),
    attack("P1347-W12-correct-pid-without-waitpid", "process-identity", "drop_parent_wait", "WORKER_AUTHORITY", pid="observed_child"),
    attack("P1347-W13-stdout-from-sibling", "pipe-identity", "mix_process_evidence", "WORKER_AUTHORITY", foreign="stdout"),
    attack("P1347-W14-stderr-from-sibling", "pipe-identity", "mix_process_evidence", "WORKER_AUTHORITY", foreign="stderr"),
    attack("P1347-W15-status-from-sibling", "wait-identity", "mix_process_evidence", "WORKER_AUTHORITY", foreign="wait_status"),
    attack("P1347-W16-partial-stdout", "worker-envelope", "truncate_worker_stdout", "WORKER_AUTHORITY", fraction="1/2"),
    attack("P1347-W17-extra-stdout-line", "worker-envelope", "append_worker_stdout", "WORKER_AUTHORITY", bytes="0a"),
    attack("P1347-W18-timeout", "worker-termination", "worker_timeout", "WORKER_AUTHORITY"),
    attack("P1347-W19-signal", "worker-termination", "worker_signal", "WORKER_AUTHORITY", signal="SIGKILL"),
    attack("P1347-W20-unexpected-exit", "worker-termination", "worker_exit", "WORKER_AUTHORITY", exit=7),
    attack("P1347-W21-challenge-replay", "worker-freshness", "replay_worker_challenge", "WORKER_AUTHORITY"),
    # Answer-free mutation DAG and external anchor.
    attack("P1347-D01-labels-without-validator", "dag-self-judgment", "dag_labels_without_validator", "DAG_VALIDATOR", labels=["classification", "reason_code"]),
    attack("P1347-D02-byte-change-validator-accepts", "dag-validation", "dag_mutation_accepted_by_real_validator", "DAG_VALIDATOR"),
    attack("P1347-D03-mutant-provided-anchor", "dag-anchor", "dag_mutant_anchor", "AUTHORITY_ROOT", anchor="mutated_root"),
    attack("P1347-D04-validator-not-parent-observed", "dag-process", "dag_validator_without_parent_process", "DAG_VALIDATOR"),
    attack("P1347-D05-receipt-answer-channel", "dag-schema", "dag_receipt_answer_channel", "SCHEMA", keys=["reason", "verdict", "witness"]),
    attack("P1347-D06-coordinated-anchor-substitution", "dag-anchor", "dag_coordinated_substitution", "AUTHORITY_ROOT"),
    # Traced direct-FD execution and loaded-image attestation.
    attack("P1347-P01-fd-swap-before-exec", "probe-image", "swap_fd_before_exec", "PROBE_AUTHORITY"),
    attack("P1347-P02-fd-swap-inside-wrapper", "probe-image", "swap_fd_inside_exec", "PROBE_AUTHORITY", image="canonical_output_imitator"),
    attack("P1347-P03-postcheck-dup-over", "probe-image", "dup_over_after_child_check", "PROBE_AUTHORITY"),
    attack("P1347-P04-swap-after-exec-stop", "probe-image", "swap_fd_after_exec_stop", "PROBE_AUTHORITY"),
    attack("P1347-P05-second-exec", "probe-trace", "second_exec_after_attestation", "PROBE_AUTHORITY"),
    attack("P1347-P06-imitator-canonical-output", "probe-image", "adversarial_image", "PROBE_AUTHORITY", behavior="valid_challenge_nonce_schema_canonical_json"),
    attack("P1347-P07-pidfd-absent", "probe-capability", "remove_capability", "PROBE_AUTHORITY", capability="pidfd_open"),
    attack("P1347-P08-pidfd-failure", "probe-capability", "fail_syscall", "PROBE_AUTHORITY", syscall="pidfd_open"),
    attack("P1347-P09-ptrace-absent", "probe-capability", "remove_capability", "PROBE_AUTHORITY", capability="ptrace"),
    attack("P1347-P10-ptrace-failure", "probe-capability", "fail_syscall", "PROBE_AUTHORITY", syscall="PTRACE_SETOPTIONS"),
    attack("P1347-P11-exec-stop-absent", "probe-trace", "suppress_exec_stop", "PROBE_AUTHORITY"),
    attack("P1347-P12-proc-exe-unreadable", "probe-image", "fail_proc_exe", "PROBE_AUTHORITY", failure="EACCES"),
    attack("P1347-P13-proc-exe-digest-mismatch", "probe-image", "substitute_proc_exe_image", "PROBE_AUTHORITY"),
    attack("P1347-P14-fd-exec-unavailable", "probe-capability", "remove_capability", "PROBE_AUTHORITY", capability="execve_fd"),
    attack("P1347-P15-seal-visibility-missing", "probe-capability", "fail_syscall", "PROBE_AUTHORITY", syscall="F_GET_SEALS"),
    attack("P1347-P16-sibling-probe-stream", "probe-process", "mix_process_evidence", "PROBE_AUTHORITY", foreign="probe_stdout_and_status"),
    attack("P1347-P17-tracer-detached", "probe-trace", "detach_before_terminal_wait", "PROBE_AUTHORITY"),
]

CONTROLS = [
    {"adapter_operation": "canonical_worker", "case_id": "P1347-C01-real-worker-exact-order", "expected_classification": "Preserved"},
    {"adapter_operation": "canonical_dag", "case_id": "P1347-C02-canonical-dag", "expected_classification": "Preserved"},
    *[{"adapter_operation": "real_dag_validator", "case_id": f"P1347-C0{index}-{name}", "expected_classification": "Violated", "expected_reason_code": "DAG_VALIDATOR", "mutation": name} for index, name in enumerate(["T08", "T09", "T10", "T11"], 3)],
    {"adapter_operation": "canonical_traced_probe", "case_id": "P1347-C07-authenticated-opaque-probe", "expected_classification": "Unknown"},
    *[{"adapter_operation": "p1346_control_projection", "case_id": f"P1347-C{index:02d}-p1346-control-{index - 7}", "expected_classification": "Preserved", "ordinal": index - 8} for index in range(8, 12)],
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
    def exercise(self, case: dict[str, Any], targets: FrozenTargets, exact_ids: list[str]) -> dict[str, Any]: ...


def frozen_ids() -> list[str]:
    binding = strict_json(PINS["contract_binding_p1347"][0].read_bytes(), "P1347 binding")
    ids = binding["canonical_case_sequence"]["exact_ids"]
    if type(ids) is not list or len(ids) != 122 or len(set(ids)) != 122 or any(type(item) is not str or not item.isascii() for item in ids):
        raise RuntimeError("PROTECTED_INPUT: exact 122 IDs")
    if sha256(canonical(ids)[:-1]) != SEQUENCE_SHA256:
        raise RuntimeError("PROTECTED_INPUT: sequence digest")
    return ids


def prior_killed_replays() -> list[dict[str, Any]]:
    report = strict_json(PINS["adversary_report_p1346"][0].read_bytes(), "P1346 adversary report")
    rows = report["vectors"]["canonical_inherited"] + report["vectors"]["canonical_transport"] + report["vectors"]["replayed_r1_and_new"]
    killed = [item for item in rows if item.get("actual") == "Violated"]
    if len(killed) != 181:
        raise RuntimeError("PROTECTED_INPUT: expected 181 previously killed negatives")
    return [attack(f"P1347-R{index:03d}-{item['id']}", "p1346-regression", "replay_p1346_negative", item.get("reason_code", "PROTECTED_INPUT"), source_case_id=item["id"]) for index, item in enumerate(killed, 1)]


def validate_authorship() -> dict[str, Any]:
    for label, (path, digest) in PINS.items():
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label}")
    ids = frozen_ids()
    prior = strict_json(PINS["adversary_report_p1346"][0].read_bytes(), "P1346 adversary report")
    if prior["summary"]["survivors"] != P1346_SURVIVORS or prior["summary"]["valid_negatives"] != 186 or prior["summary"]["correctly_violated"] != 181:
        raise RuntimeError("PROTECTED_INPUT: P1346 vector")
    attack_ids = [item["case_id"] for item in ATTACKS]
    control_ids = [item["case_id"] for item in CONTROLS]
    if len(ATTACKS) != 49 or len(set(attack_ids)) != len(attack_ids) or len(CONTROLS) != 11 or set(attack_ids) & set(control_ids):
        raise RuntimeError("SCHEMA: attack/control cardinality or identity")
    replays = prior_killed_replays()
    return {
        "attacks_authored": len(ATTACKS),
        "candidate_read_or_executed": False,
        "controls_authored": len(CONTROLS),
        "full_runs": 0,
        "oracle_files_read_or_executed": False,
        "previously_killed_replays_bound": len(replays),
        "sequence_count": len(ids),
        "sequence_sha256": SEQUENCE_SHA256,
        "verdict": "AUTHORED_NOT_EXECUTED",
    }


def _load_adapter(path: Path, digest: str) -> VerifierAdapter:
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("AUTHORITY_ROOT: verifier adapter")
    spec = importlib.util.spec_from_file_location("p1347_frozen_verifier_adapter", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: adapter loader")
    module = importlib.util.module_from_spec(spec); sys.modules[spec.name] = module; spec.loader.exec_module(module)
    if not hasattr(module, "exercise"):
        raise RuntimeError("SCHEMA: adapter.exercise missing")
    return module


def run_with_adapter(targets: FrozenTargets, adapter_path: Path, adapter_sha256: str) -> dict[str, Any]:
    """Future verifier entrypoint; never called during adversarial authorship."""
    validate_authorship()
    for path, digest in [(targets.checker, targets.checker_sha256), (targets.caller, targets.caller_sha256), (targets.delivery_receipt, targets.delivery_receipt_sha256)]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError("AUTHORITY_ROOT: frozen target drift")
    adapter = _load_adapter(adapter_path, adapter_sha256)
    ids = frozen_ids()
    focal_records = [adapter.exercise(item, targets, ids) for item in ATTACKS]
    survivors = [item["case_id"] for item, observed in zip(ATTACKS, focal_records) if observed.get("classification") != item["expected_classification"] or observed.get("reason_code") != item["expected_reason_code"]]
    regression_records: list[dict[str, Any]] = []
    control_records: list[dict[str, Any]] = []
    if not survivors:
        regression_records = [adapter.exercise(item, targets, ids) for item in prior_killed_replays()]
        survivors += [item["case_id"] for item, observed in zip(prior_killed_replays(), regression_records) if observed.get("classification") != "Violated"]
        control_records = [adapter.exercise(item, targets, ids) for item in CONTROLS]
    return {"controls": control_records, "focal": focal_records, "full_runs": 0, "regressions": regression_records, "survivors": survivors, "verdict": "FOCAL_ZERO_SURVIVORS" if not survivors else "SURVIVORS_BLOCK_PRESEAL"}


def main() -> int:
    if sys.argv[1:] != ["--author-check"]:
        raise RuntimeError("SCHEMA: authorship CLI accepts exactly --author-check; later verifier imports run_with_adapter")
    sys.stdout.buffer.write(canonical(validate_authorship()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
