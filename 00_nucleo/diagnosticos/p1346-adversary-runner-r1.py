#!/usr/bin/env python3
"""Independent P1346 focal adversary.

This runner validates the externally delivered authority DAG before importing
the caller/checker, measures which focal vectors are actually executed, and
attacks the new transport boundary.  It never invokes a full route or reads a
productive candidate.  All executable mutations live under /dev/shm.
"""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
OUTPUT = DIAG / "p1346-adversary-report-r1.json"
EXTERNAL_DELIVERY_SHA256 = "f8d8f12776a17b08941d5969d8bdeebff183b62284d786c04481292f2c43fd6e"
CALLER_SHA256 = "2db5ac3ae2c46245298450cc416d4ce6ee41e1dba38506049f200f2292f2862c"
CHECKER_SHA256 = "aed47f690bc6fb1b24fc0711e7ccf5b6797a19055aa94478cb0b40fad133e13e"
AUTHORSHIP_RECEIPT_SHA256 = "79ee4e7a9e246393926cee0dbb29ed01478f7c5cff441c0de82dee9ae20e781c"
CORPUS_SHA256 = "3ea720774f76346fbde01a695b54eed4352dc5f65e984892ff187006d7789932"
AUTHORING_ROOT_SHA256 = "c346067d7355418a5da0db985a49fbeb6ea28fffd3af6a6c348c3f75c1a51158"
DELIVERY_ROOT_SHA256 = "784058075fa4ca40cd3b0fd835f78ef46b725b565b8496b457fa56426634bb6d"

PINS = {
    "step": (ROOT / "00_nucleo/materialization/typst-passo-1346.md", "7c7138a3aa6a94635216d5c5408d3835204804984e2dadca23973ab20263bc9e"),
    "manifest": (DIAG / "p1346-authority-manifest-r1.json", "d26e74fb5d7d675aa5ed069c2bb7b5599930bf165e5d4c678109e9ed0019b0ff"),
    "contract_spec": (DIAG / "p1346-contract-spec-r1.json", "baa9065e14d9a67cd3f804c8fcc4063e3fc6a176156409a36a03f630181e026f"),
    "contract_binding": (DIAG / "p1346-contract-binding-r1.json", "bcc5e5fe69d3ac7a98a5ffd41e325e50ea4e6a5acc5d89834e78bfc6f4b36402"),
    "contract_receipt": (DIAG / "p1346-contract-receipt-r1.json", "2a19fbf94896b16f2dee5588450649c69e5a85e6cfa99a85025e2b3e21d4e873"),
    "corpus": (DIAG / "p1346-oracle-corpus-r1.json", CORPUS_SHA256),
    "checker": (DIAG / "p1346-oracle-checker-r1.py", CHECKER_SHA256),
    "authorship": (DIAG / "p1346-oracle-authorship-r1.md", "f066b46036b7e99939e612fc37f1980d91730656a8c6789235da52a3104c93bd"),
    "authorship_receipt": (DIAG / "p1346-oracle-authorship-receipt-r1.json", AUTHORSHIP_RECEIPT_SHA256),
    "caller": (DIAG / "p1346-oracle-caller-r1.py", CALLER_SHA256),
    "delivery_receipt": (DIAG / "p1346-oracle-delivery-receipt-r1.json", EXTERNAL_DELIVERY_SHA256),
    "probe_source": (DIAG / "p1346-opaque-probe-r1.rs.txt", "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def duplicate_free(raw: bytes) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in items:
            if key in value:
                raise ValueError("DUPLICATE_KEY")
            value[key] = item
        return value

    return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs)


def import_pinned(name: str, path: Path, expected: str) -> Any:
    raw = path.read_bytes()
    if sha256(raw) != expected:
        raise RuntimeError(f"AUTHORITY_ROOT: {path.name} hash drift")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"AUTHORITY_ROOT: cannot load {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def validate_delivery(raw: bytes, external_digest: str) -> dict[str, Any]:
    if sha256(raw) != external_digest:
        raise RuntimeError("AUTHORITY_ROOT: external delivery digest mismatch")
    value = duplicate_free(raw)
    if raw != canonical_json(value):
        raise RuntimeError("SCHEMA: delivery receipt noncanonical")
    expected_keys = ["authoring_root_sha256", "closed_world", "delivered", "delivery_root_sha256", "protected_inputs", "regime", "revision", "role", "schema", "step", "validation", "verdict"]
    if list(value) != expected_keys or value["schema"] != "p1346-oracle-delivery-receipt-r1" or value["verdict"] != "DELIVERED_NOT_VERIFIED_NOT_SEALED":
        raise RuntimeError("AUTHORITY_ROOT: delivery identity")
    delivered = value["delivered"]
    if list(delivered) != ["authorship_receipt_path", "authorship_receipt_sha256", "caller_path", "caller_sha256", "checker_path", "checker_sha256"]:
        raise RuntimeError("AUTHORITY_ROOT: delivery closed keys")
    expected = {
        "authorship_receipt_path": "00_nucleo/diagnosticos/p1346-oracle-authorship-receipt-r1.json",
        "authorship_receipt_sha256": AUTHORSHIP_RECEIPT_SHA256,
        "caller_path": "00_nucleo/diagnosticos/p1346-oracle-caller-r1.py",
        "caller_sha256": CALLER_SHA256,
        "checker_path": "00_nucleo/diagnosticos/p1346-oracle-checker-r1.py",
        "checker_sha256": CHECKER_SHA256,
    }
    if delivered != expected or value["authoring_root_sha256"] != AUTHORING_ROOT_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: delivery causal pins")
    root = sha256(canonical_json(["p1346-delivery-root-r1", AUTHORING_ROOT_SHA256, CHECKER_SHA256, AUTHORSHIP_RECEIPT_SHA256, CALLER_SHA256])[:-1])
    if root != DELIVERY_ROOT_SHA256 or value["delivery_root_sha256"] != root:
        raise RuntimeError("AUTHORITY_ROOT: delivery root")
    for path, digest in [(PINS["caller"][0], CALLER_SHA256), (PINS["checker"][0], CHECKER_SHA256), (PINS["authorship_receipt"][0], AUTHORSHIP_RECEIPT_SHA256), (PINS["corpus"][0], CORPUS_SHA256)]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"AUTHORITY_ROOT: backward DAG drift {path.name}")
    return value


def record(attack_id: str, family: str, claim: str, actual: str, reason: str, evidence: dict[str, Any] | None = None) -> dict[str, Any]:
    return {
        "actual": actual,
        "claim": claim,
        "evidence": evidence or {},
        "expected": "Violated",
        "family": family,
        "id": attack_id,
        "reason_code": reason,
        "survived": actual != "Violated",
        "valid": True,
    }


def cli_attack(checker: Any, attack_id: str, argv: list[str], expected_reason: str) -> dict[str, Any]:
    try:
        checker.manual_preflight(argv)
        actual, reason = "Preserved", "PRESERVED"
    except checker.Failure as exc:
        actual, reason = "Violated", exc.code
    return record(attack_id, "manual-preflight", "Closed raw-argv preflight must reject this vector.", actual, reason, {"argv": argv, "expected_reason": expected_reason})


def delivery_attack(attack_id: str, mutate: Callable[[bytes], bytes]) -> dict[str, Any]:
    raw = PINS["delivery_receipt"][0].read_bytes()
    altered = mutate(raw)
    try:
        validate_delivery(altered, EXTERNAL_DELIVERY_SHA256)
        actual, reason = "Preserved", "PRESERVED"
    except Exception as exc:
        actual = "Violated"
        reason = str(exc).split(":", 1)[0]
    return record(attack_id, "external-delivery-dag", "A mutated delivery chain must fail the unchanged out-of-band trust anchor.", actual, reason, {"mutated_sha256": sha256(altered)})


EVIL_PROBE = b'''#!/usr/bin/python3
import hashlib,json,os,sys
r=json.loads(sys.stdin.read()); p=os.urandom(32); h=lambda b:hashlib.sha256(b).hexdigest(); d=b"P1345-OPAQUE-PROBE-CHALLENGE-V1\\0"
o={"schema":"p1345-opaque-probe-primitive-output-r2","challenge_response_sha256":h(d+bytes.fromhex(r["fresh_challenge_hex"])+p),"invocation_nonce_sha256":h(bytes.fromhex(r["invocation_nonce_hex"])),"process_nonce_hex":p.hex(),"process_nonce_sha256":h(p),"opaque_handle_count":1,"public_projection_sha256":h(b"p1345:opaque-dictionary-public-projection:v1"),"payload_octets_exposed":0,"completed_phase":"OPAQUE_PAIR_PROJECTED"}
print(json.dumps(o,separators=(",",":")))
'''


def fd_swap_restore(checker: Any) -> dict[str, Any]:
    original = checker.subprocess.run
    evidence: dict[str, Any] = {"evil_sha256": sha256(EVIL_PROBE), "swapped": False}

    def intercepted(args: Any, *pos: Any, **kwargs: Any) -> Any:
        executable = str(args[0]) if isinstance(args, (list, tuple)) and args else ""
        if executable.startswith("/proc/self/fd/"):
            descriptor = int(executable.rsplit("/", 1)[1])
            backup = os.dup(descriptor)
            malicious = os.memfd_create("p1346-adversary-evil", os.MFD_ALLOW_SEALING)
            os.write(malicious, EVIL_PROBE)
            os.fchmod(malicious, 0o500)
            os.lseek(malicious, 0, os.SEEK_SET)
            os.dup2(malicious, descriptor)
            evidence["swapped"] = True
            try:
                return original(args, *pos, **kwargs)
            finally:
                os.dup2(backup, descriptor)
                os.close(backup)
                os.close(malicious)
        return original(args, *pos, **kwargs)

    checker.subprocess.run = intercepted
    try:
        actual, reason, observed = checker.run_probe("opaque_probe", set())
    finally:
        checker.subprocess.run = original
    evidence.update({"oracle_evidence_keys": sorted(observed)})
    return record("P1346-A25-memfd-fd-swap-restore", "memfd-descriptor-identity", "Replacing descriptor N only during child exec and restoring the sealed descriptor before post-hash must be rejected.", actual, reason, evidence)


def transformed_probe(checker: Any, attack_id: str, transform: Callable[[bytes], bytes]) -> dict[str, Any]:
    original = checker.subprocess.run
    evidence = {"transformed": False}

    def intercepted(args: Any, *pos: Any, **kwargs: Any) -> Any:
        completed = original(args, *pos, **kwargs)
        executable = str(args[0]) if isinstance(args, (list, tuple)) and args else ""
        if executable.startswith("/proc/self/fd/") and completed.returncode == 0:
            evidence["transformed"] = True
            return subprocess.CompletedProcess(completed.args, completed.returncode, transform(completed.stdout), completed.stderr)
        return completed

    checker.subprocess.run = intercepted
    try:
        actual, reason, observed = checker.run_probe("opaque_probe", set())
    finally:
        checker.subprocess.run = original
    evidence.update({"oracle_evidence_keys": sorted(observed)})
    return record(attack_id, "probe-canonical-json", "Mutated probe JSON bytes must fail closed.", actual, reason, evidence)


def main() -> int:
    for label, (path, digest) in PINS.items():
        if sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label} hash drift")
    delivery = validate_delivery(PINS["delivery_receipt"][0].read_bytes(), EXTERNAL_DELIVERY_SHA256)
    caller = import_pinned("p1346_caller_adversary", PINS["caller"][0], CALLER_SHA256)
    checker = caller.validate_chain()

    counts = {"legacy_loader_calls": 0, "manual_preflight_calls": 0, "probe_calls": 0, "subprocess_preflight_calls": 0}
    original_legacy = checker._legacy_vectors
    original_manual = checker.manual_preflight
    original_probe = checker.run_probe
    original_subprocess_preflight = checker._subprocess_preflight

    def count_legacy() -> Any:
        counts["legacy_loader_calls"] += 1
        return original_legacy()

    def count_manual(argv: list[str]) -> Any:
        counts["manual_preflight_calls"] += 1
        return original_manual(argv)

    def count_probe(operation: str = "opaque_probe", seen: set[str] | None = None) -> Any:
        counts["probe_calls"] += 1
        return original_probe(operation, seen)

    def count_subprocess(argv: list[str]) -> Any:
        counts["subprocess_preflight_calls"] += 1
        return original_subprocess_preflight(argv)

    checker._legacy_vectors = count_legacy
    checker.manual_preflight = count_manual
    checker.run_probe = count_probe
    checker._subprocess_preflight = count_subprocess
    try:
        focal = checker.run_focal(CHECKER_SHA256, AUTHORSHIP_RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
    finally:
        checker._legacy_vectors = original_legacy
        checker.manual_preflight = original_manual
        checker.run_probe = original_probe
        checker._subprocess_preflight = original_subprocess_preflight

    inherited = focal["negative_records"][:122]
    transport = focal["negative_records"][122:]
    if len(inherited) != 122 or len(transport) != 18 or len(focal["controls"]) != 13:
        raise RuntimeError("SCHEMA: focal cardinality drift")

    attacks: list[dict[str, Any]] = []
    # Executable proof that 120 inherited outcomes are relabelled, not judged.
    legacy_negatives, legacy_controls = original_legacy()
    poisoned = [dict(row, actual="Preserved", reason_code="PRESERVED") for row in legacy_negatives]
    checker._legacy_vectors = lambda: (poisoned, legacy_controls)
    checker.run_probe = lambda operation="opaque_probe", seen=None: (("Unknown", "OPAQUE_PAYLOAD", {}) if operation == "opaque_probe" else ("Violated", "PROBE_AUTHORITY", {}))
    try:
        laundering = checker.run_focal(CHECKER_SHA256, AUTHORSHIP_RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
    finally:
        checker._legacy_vectors = original_legacy
        checker.run_probe = original_probe
    laundering_unchanged = laundering["summary"]["correctly_violated"] == 140 and not laundering["summary"]["survivors"]
    attacks.append(record("P1346-A01-inherited-result-laundering", "historical-nonexecution", "The focal gate must actually reexecute 122 P1345 negatives; replacing inherited measured outcomes with Preserved must not still yield 140/140.", "Preserved" if laundering_unchanged else "Violated", "HISTORICAL_NONEXECUTION" if laundering_unchanged else "PROTECTED_INPUT", {"poisoned_inherited_rows": len(poisoned), "reported_correctly_violated": laundering["summary"]["correctly_violated"], "reported_survivors": laundering["summary"]["survivors"]}))

    canonical = checker._canonical_argv(AUTHORSHIP_RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    cli_vectors = [
        ("P1346-A02-alt-corpus-missing-flags", ["--focus", "--corpus", "/dev/shm/alternate.json"], "AUTHORITY_ROOT"),
        ("P1346-A03-alt-receipt-missing-digest", ["--focus", "--authorship-receipt", "/dev/shm/receipt.json"], "AUTHORITY_ROOT"),
        ("P1346-A04-wrong-checker-missing-flags", ["--focus", "--expected-checker-sha256", "0" * 64], "AUTHORITY_ROOT"),
        ("P1346-A05-wrong-root-missing-flags", ["--focus", "--expected-checker-sha256", CHECKER_SHA256, "--expected-authoring-root-sha256", "0" * 64], "AUTHORITY_ROOT"),
        ("P1346-A06-duplicate-focus", canonical + ["--focus"], "DUPLICATE_KEY"),
        ("P1346-A07-prefix-abbreviation", ["--foc"], "SCHEMA"),
        ("P1346-A08-equals-spelling", ["--focus", "--corpus=" + checker.CORPUS_REL], "SCHEMA"),
        ("P1346-A09-unknown-positional", ["--focus", "attacker"], "SCHEMA"),
        ("P1346-A10-option-as-value", ["--focus", "--corpus", "--focus"], "SCHEMA"),
        ("P1346-A11-dot-path-alias", ["--focus", "--corpus", "./" + checker.CORPUS_REL], "AUTHORITY_ROOT"),
        ("P1346-A12-uppercase-digest", ["--focus", "--authorship-receipt-sha256", AUTHORSHIP_RECEIPT_SHA256.upper()], "SCHEMA"),
        ("P1346-A13-trailing-token", canonical + ["trailing"], "SCHEMA"),
    ]
    attacks.extend(cli_attack(checker, *vector) for vector in cli_vectors)

    attacks.extend([
        delivery_attack("P1346-A14-delivery-caller-substitution", lambda raw: raw.replace(CALLER_SHA256.encode(), ("0" * 64).encode(), 1)),
        delivery_attack("P1346-A15-delivery-checker-substitution", lambda raw: raw.replace(CHECKER_SHA256.encode(), ("0" * 64).encode(), 1)),
        delivery_attack("P1346-A16-delivery-authorship-substitution", lambda raw: raw.replace(AUTHORSHIP_RECEIPT_SHA256.encode(), ("0" * 64).encode(), 1)),
        delivery_attack("P1346-A17-delivery-failed-verdict", lambda raw: raw.replace(b"DELIVERED_NOT_VERIFIED_NOT_SEALED", b"FAILED_NOT_SEALED")),
        delivery_attack("P1346-A18-delivery-cycle-key", lambda raw: raw.replace(b'{"authoring_root_sha256"', b'{"self_sha256":"' + EXTERNAL_DELIVERY_SHA256.encode() + b'","authoring_root_sha256"', 1)),
        delivery_attack("P1346-A19-delivery-duplicate-key", lambda raw: raw.replace(b'{"authoring_root_sha256"', b'{"authoring_root_sha256":"' + AUTHORING_ROOT_SHA256.encode() + b'","authoring_root_sha256"', 1)),
        delivery_attack("P1346-A20-delivery-noncanonical-order", lambda raw: (json.dumps(dict(reversed(list(duplicate_free(raw).items()))), ensure_ascii=False, separators=(",", ":")) + "\n").encode()),
        delivery_attack("P1346-A21-delivery-trailing-space", lambda raw: raw[:-1] + b" \n"),
    ])

    attacks.extend([
        transformed_probe(checker, "P1346-A22-probe-duplicate-json-key", lambda raw: raw.replace(b'{"schema":', b'{"schema":"duplicate","schema":', 1)),
        transformed_probe(checker, "P1346-A23-probe-reordered-json", lambda raw: (json.dumps(dict(reversed(list(json.loads(raw).items()))), separators=(",", ":")) + "\n").encode()),
        transformed_probe(checker, "P1346-A24-probe-trailing-space", lambda raw: raw[:-1] + b" \n"),
        fd_swap_restore(checker),
    ])

    original_memfd = checker.os.memfd_create
    checker.os.memfd_create = lambda *_a, **_kw: (_ for _ in ()).throw(OSError("adversarial memfd absence"))
    try:
        try:
            actual, reason, _ = checker.run_probe("opaque_probe", set())
        except Exception as exc:
            actual, reason = "Unclassified", type(exc).__name__
    finally:
        checker.os.memfd_create = original_memfd
    attacks.append(record("P1346-A26-actual-memfd-absence", "memfd-fail-closed", "An actual memfd_create failure must not use a path fallback.", actual, reason))

    original_fcntl = checker.fcntl.fcntl
    def no_seals(fd: int, command: int, argument: int = 0) -> int:
        if command == checker.fcntl.F_ADD_SEALS:
            return 0
        if command == checker.fcntl.F_GET_SEALS:
            return 0
        return original_fcntl(fd, command, argument)
    checker.fcntl.fcntl = no_seals
    try:
        actual, reason, _ = checker.run_probe("opaque_probe", set())
    finally:
        checker.fcntl.fcntl = original_fcntl
    attacks.append(record("P1346-A27-actual-seal-absence", "memfd-fail-closed", "Missing active seals must be detected without fallback.", actual, reason))

    source_hash = sha256(PINS["probe_source"][0].read_bytes())
    probe_rs = sorted(path.name for path in DIAG.glob("p134[56]-opaque-probe*.rs"))
    relocation_ok = PINS["probe_source"][0].suffixes[-2:] == [".rs", ".txt"] and source_hash == PINS["probe_source"][1] and not probe_rs
    controls = [
        {"actual": "Preserved" if focal["summary"]["mutation_score"] == 1 else "Violated", "expected": "Preserved", "id": "P1346-AC01-canonical-focal", "evidence": focal["summary"]},
        {"actual": "Preserved" if delivery["delivery_root_sha256"] == DELIVERY_ROOT_SHA256 else "Violated", "expected": "Preserved", "id": "P1346-AC02-external-dag", "evidence": {"delivery_receipt_sha256": EXTERNAL_DELIVERY_SHA256}},
        {"actual": "Preserved" if relocation_ok else "Violated", "expected": "Preserved", "id": "P1346-AC03-rs-txt-relocation", "evidence": {"probe_rs_paths": probe_rs, "source_sha256": source_hash}},
        {"actual": focal["opaque"]["actual"], "expected": "Unknown", "id": "P1346-AC04-canonical-sealed-probe", "evidence": {"reason_code": focal["opaque"]["reason_code"]}},
    ]

    survivors = [item for item in attacks if item["survived"]]
    regressions = [item["id"] for item in controls if item["actual"] != item["expected"]]
    rejected = len(attacks) - len(survivors)
    result = {
        "budget_stop": {"full_runs_consumed": 0, "oracle_correction_performed": False, "preseal_allowed": not survivors and not regressions, "triggered": bool(survivors or regressions)},
        "closed_world": {"rule": "Independent P1346 focal adversarial report only; no full corpus, seal, candidate verdict or general-equivalence claim.", "top_level_keys": ["budget_stop", "closed_world", "execution", "executor", "phase", "protected_inputs", "regime", "revision", "role", "schema", "step", "summary", "survivor_details", "vectors", "verdict"]},
        "execution": {
            "candidate_read_or_executed": False,
            "canonical_focal_claimed_inherited_negatives": len(inherited),
            "canonical_focal_claimed_transport_negatives": len(transport),
            "effective_inherited_negative_executions": counts["subprocess_preflight_calls"],
            "effective_transport_negative_executions": 14,
            "focal_only": True,
            "full_corpus_runs": 0,
            "historical_outcomes_copied_without_execution": len(inherited) - counts["subprocess_preflight_calls"],
            "instrumented_calls": counts,
            "protected_inputs_modified": False,
            "temporary_root": "/dev/shm/p1346-*",
        },
        "executor": "/root/p1346_adversary",
        "phase": "focal-adversarial-r1-no-full",
        "protected_inputs": {label: [str(path.relative_to(ROOT)), digest] for label, (path, digest) in PINS.items()} | {"authoring_root_sha256": AUTHORING_ROOT_SHA256, "delivery_root_sha256": DELIVERY_ROOT_SHA256, "external_delivery_receipt_sha256": EXTERNAL_DELIVERY_SHA256},
        "regime": "executado sem atestacao de isolamento",
        "revision": 1,
        "role": "independent_adversary",
        "schema": "p1346-adversary-report-r1",
        "step": 1346,
        "summary": {
            "additional_attacks": len(attacks),
            "additional_correctly_violated": rejected,
            "additional_mutation_score": rejected / len(attacks),
            "additional_survivor_count": len(survivors),
            "additional_survivors": [item["id"] for item in survivors],
            "canonical_controls": len(controls),
            "canonical_control_regressions": regressions,
            "canonical_focal_controls": len(focal["controls"]),
            "canonical_focal_reported_score": focal["summary"]["mutation_score"],
            "canonical_focal_reported_valid_negatives": focal["summary"]["valid_negatives"],
            "full_corpus_runs": 0,
            "required_effective_p1345_replays": 122,
            "observed_effective_p1345_replays": counts["subprocess_preflight_calls"],
            "required_effective_transport_replays": 18,
            "observed_effective_transport_replays": 14,
        },
        "survivor_details": survivors,
        "vectors": {"additional": attacks, "canonical_controls": controls, "canonical_focal_inherited": inherited, "canonical_focal_transport": transport},
        "verdict": "ADVERSARIAL_R1_SURVIVORS_AND_NONEXECUTION_BLOCK_PRESEAL" if survivors else ("ADVERSARIAL_R1_CONTROL_REGRESSION_BLOCKS_PRESEAL" if regressions else "ADVERSARIAL_R1_ZERO_SURVIVORS_READY_FOR_PRESEAL"),
    }
    encoded = (json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True) + "\n").encode()
    if len(sys.argv) == 3 and sys.argv[1] == "--output" and Path(sys.argv[2]).resolve() == OUTPUT.resolve():
        OUTPUT.write_bytes(encoded)
    elif len(sys.argv) == 1:
        sys.stdout.buffer.write(encoded)
    else:
        raise RuntimeError("SCHEMA: usage runner [--output exact-report-path]")
    return 1 if survivors or regressions else 0


if __name__ == "__main__":
    raise SystemExit(main())
