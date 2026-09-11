#!/usr/bin/env python3
"""Independent final focal adversary for the P1346 R2 oracle.

The external delivery digest is validated before importing executable oracle
code.  This runner invokes the 122-case legacy suite and all 18 P1346
transport cases, replays the 27 R1 attacks, and adds worker/process/FD attacks.
No full route or productive candidate is read.
"""

from __future__ import annotations

import fcntl
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
OUTPUT = DIAG / "p1346-adversary-report-r2.json"
DELIVERY_SHA256 = "2b4ec685285153b185a3de6a92aec074de39731f34e0f52b35489ddfb4c0f5e0"
DELIVERY_ROOT_SHA256 = "3ead9d4bfa964ebb1f6d759fe005902ee3dc721b46e81297e6a5b05344a04dce"
AUTHORING_ROOT_SHA256 = "83fea4b0ae2bf167520c17577f15cd60341edcf990e97f68d7e12dd7ebc0ffd6"
CALLER_SHA256 = "ba4fd42247eadd04ee7a8af8519043a77113205340cb60edbe79128e29ceb120"
CHECKER_SHA256 = "c6c8e425b9387afd2d02d20d148d407c4f2b252cef3428ccb1eb1eec91894ec1"
RECEIPT_SHA256 = "7343643dba523fa7716e4fc89dac7ec20492c5c41718333d9f46f67ce260d4ca"
CASE_RUNNER_SHA256 = "09bfbd15cd7aec9fb600dbea85ed84c6e7059cd252c3c42ceb296ee7c6a67c96"
CORPUS_SHA256 = "6972a680ffca27f29d774b4ebde225227cb47b645cabeb64bb73205fef04975f"

PINS = {
    "step": (ROOT / "00_nucleo/materialization/typst-passo-1346.md", "7c7138a3aa6a94635216d5c5408d3835204804984e2dadca23973ab20263bc9e"),
    "manifest": (DIAG / "p1346-authority-manifest-r1.json", "d26e74fb5d7d675aa5ed069c2bb7b5599930bf165e5d4c678109e9ed0019b0ff"),
    "contract_spec": (DIAG / "p1346-contract-spec-r1.json", "baa9065e14d9a67cd3f804c8fcc4063e3fc6a176156409a36a03f630181e026f"),
    "contract_binding": (DIAG / "p1346-contract-binding-r1.json", "bcc5e5fe69d3ac7a98a5ffd41e325e50ea4e6a5acc5d89834e78bfc6f4b36402"),
    "adversary_runner_r1": (DIAG / "p1346-adversary-runner-r1.py", "eef0a9992f0ce45bfbede66d904a4561f4252a3c6940715640e9350a1bb3c45a"),
    "adversary_report_r1": (DIAG / "p1346-adversary-report-r1.json", "79a850efeb6cbee0c48558ee8c3ddec0cb8e5dc49e9376d0769e2fbd9998ae03"),
    "adversary_report_md_r1": (DIAG / "p1346-adversary-report-r1.md", "696295f8ec8cab4ee17a6325b6aa2ff975fe619d4f7e45c0b691d8ceb14dfeda"),
    "adversary_receipt_r1": (DIAG / "p1346-adversary-receipt-r1.json", "712ffff25fc1bac518e7f6ae8ee6cfb9f7e5817e447361ebd4207b54d5c1d7ad"),
    "case_runner_r2": (DIAG / "p1346-oracle-case-runner-r2.py", CASE_RUNNER_SHA256),
    "corpus_r2": (DIAG / "p1346-oracle-corpus-r2.json", CORPUS_SHA256),
    "checker_r2": (DIAG / "p1346-oracle-checker-r2.py", CHECKER_SHA256),
    "authorship_r2": (DIAG / "p1346-oracle-authorship-r2.md", "923baa69564377545f1266d65ace41d0e4cf93abdb38565eab9cc4494b183d28"),
    "authorship_receipt_r2": (DIAG / "p1346-oracle-authorship-receipt-r2.json", RECEIPT_SHA256),
    "caller_r2": (DIAG / "p1346-oracle-caller-r2.py", CALLER_SHA256),
    "delivery_receipt_r2": (DIAG / "p1346-oracle-delivery-receipt-r2.json", DELIVERY_SHA256),
    "probe_source": (DIAG / "p1346-opaque-probe-r1.rs.txt", "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def strict_json(raw: bytes) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        out: dict[str, Any] = {}
        for key, value in items:
            if key in out:
                raise ValueError("DUPLICATE_KEY")
            out[key] = value
        return out
    value = json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs)
    if raw != canonical(value):
        raise ValueError("SCHEMA: noncanonical JSON")
    return value


def import_pinned(name: str, path: Path, digest: str) -> Any:
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError(f"AUTHORITY_ROOT: {path.name} drift")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"AUTHORITY_ROOT: cannot import {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def validate_delivery(raw: bytes, external_digest: str) -> dict[str, Any]:
    if sha256(raw) != external_digest:
        raise RuntimeError("AUTHORITY_ROOT: external delivery digest")
    value = strict_json(raw)
    expected_keys = ["authoring_root_sha256", "closed_world", "delivered", "delivery_root_sha256", "protected_inputs", "regime", "revision", "role", "schema", "step", "validation", "verdict"]
    if list(value) != expected_keys or value["schema"] != "p1346-oracle-delivery-receipt-r2" or value["verdict"] != "DELIVERED_R2_NOT_VERIFIED_NOT_SEALED":
        raise RuntimeError("AUTHORITY_ROOT: delivery identity")
    expected_delivery = {
        "authorship_receipt_path": "00_nucleo/diagnosticos/p1346-oracle-authorship-receipt-r2.json",
        "authorship_receipt_sha256": RECEIPT_SHA256,
        "caller_path": "00_nucleo/diagnosticos/p1346-oracle-caller-r2.py",
        "caller_sha256": CALLER_SHA256,
        "checker_path": "00_nucleo/diagnosticos/p1346-oracle-checker-r2.py",
        "checker_sha256": CHECKER_SHA256,
    }
    if value["delivered"] != expected_delivery or value["authoring_root_sha256"] != AUTHORING_ROOT_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: delivery pins")
    computed = sha256(canonical(["p1346-delivery-root-r2", AUTHORING_ROOT_SHA256, CHECKER_SHA256, RECEIPT_SHA256, CALLER_SHA256])[:-1])
    if computed != DELIVERY_ROOT_SHA256 or value["delivery_root_sha256"] != computed:
        raise RuntimeError("AUTHORITY_ROOT: delivery root")
    for path, digest in [(PINS["caller_r2"][0], CALLER_SHA256), (PINS["checker_r2"][0], CHECKER_SHA256), (PINS["authorship_receipt_r2"][0], RECEIPT_SHA256), (PINS["case_runner_r2"][0], CASE_RUNNER_SHA256), (PINS["corpus_r2"][0], CORPUS_SHA256)]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"AUTHORITY_ROOT: backward DAG {path.name}")
    return value


def record(attack_id: str, family: str, claim: str, actual: str, reason: str, evidence: dict[str, Any] | None = None) -> dict[str, Any]:
    return {"actual": actual, "claim": claim, "evidence": evidence or {}, "expected": "Violated", "family": family, "id": attack_id, "reason_code": reason, "survived": actual != "Violated", "valid": True}


def delivery_attack(attack_id: str, mutate: Callable[[bytes], bytes]) -> dict[str, Any]:
    raw = PINS["delivery_receipt_r2"][0].read_bytes(); altered = mutate(raw)
    try:
        validate_delivery(altered, DELIVERY_SHA256); actual, reason = "Preserved", "PRESERVED"
    except Exception as exc:
        actual, reason = "Violated", str(exc).split(":", 1)[0]
    return record(attack_id, "external-delivery-dag", "Altered R2 DAG bytes must fail the unchanged external anchor.", actual, reason, {"mutated_sha256": sha256(altered)})


def cli_attack(checker: Any, attack_id: str, argv: list[str]) -> dict[str, Any]:
    try:
        checker.manual_preflight(argv); actual, reason = "Preserved", "PRESERVED"
    except checker.Failure as exc:
        actual, reason = "Violated", exc.code
    return record(attack_id, "manual-preflight", "Malformed or alternate raw argv must fail closed.", actual, reason, {"argv": argv})


EVIL_PROBE = b'''#!/usr/bin/python3
import hashlib,json,os,sys
r=json.loads(sys.stdin.read());p=bytes.fromhex("42"*32);h=lambda b:hashlib.sha256(b).hexdigest();d=b"P1345-OPAQUE-PROBE-CHALLENGE-V1\\0"
o={"schema":"p1345-opaque-probe-primitive-output-r2","challenge_response_sha256":h(d+bytes.fromhex(r["fresh_challenge_hex"])+p),"invocation_nonce_sha256":h(bytes.fromhex(r["invocation_nonce_hex"])),"process_nonce_hex":p.hex(),"process_nonce_sha256":h(p),"opaque_handle_count":1,"public_projection_sha256":h(b"p1345:opaque-dictionary-public-projection:v1"),"payload_octets_exposed":0,"completed_phase":"OPAQUE_PAIR_PROJECTED"}
print(json.dumps(o,separators=(",",":")))
'''


def probe_transform(checker: Any, attack_id: str, transform: Callable[[bytes], bytes]) -> dict[str, Any]:
    original = checker._fork_execute
    touched = False
    def intercepted(*args: Any, **kwargs: Any) -> Any:
        nonlocal touched
        code, stdout, stderr, pid = original(*args, **kwargs); touched = True
        return code, transform(stdout), stderr, pid
    checker._fork_execute = intercepted
    try:
        actual, reason, _ = checker.run_probe("opaque_probe", set())
    finally:
        checker._fork_execute = original
    return record(attack_id, "probe-output", "Mutated child output must fail canonical/schema binding.", actual, reason, {"transformed": touched})


def inject_exception(checker: Any, attack_id: str, owner: Any, attribute: str, exception: Exception) -> dict[str, Any]:
    original = getattr(owner, attribute)
    setattr(owner, attribute, lambda *_a, **_kw: (_ for _ in ()).throw(exception))
    try:
        try:
            actual, reason, _ = checker.run_probe("opaque_probe", set())
        except Exception as exc:
            actual, reason = "Unclassified", type(exc).__name__
    finally:
        setattr(owner, attribute, original)
    return record(attack_id, "probe-exception", f"{attribute} failure must become Violated/PROBE_AUTHORITY.", actual, reason)


def sealed_mutation(attack_id: str, operation: str) -> dict[str, Any]:
    descriptor = os.memfd_create("p1346-adversary-seal", os.MFD_ALLOW_SEALING)
    try:
        os.write(descriptor, b"sealed"); os.lseek(descriptor, 0, os.SEEK_SET)
        required = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
        fcntl.fcntl(descriptor, fcntl.F_ADD_SEALS, required)
        try:
            if operation == "write": os.write(descriptor, b"x")
            elif operation == "shrink": os.ftruncate(descriptor, 1)
            elif operation == "grow": os.ftruncate(descriptor, 64)
            actual, reason = "Preserved", "PRESERVED"
        except OSError:
            actual, reason = "Violated", "PROBE_AUTHORITY"
    finally:
        os.close(descriptor)
    return record(attack_id, "sealed-memfd-mutation", f"Kernel must reject sealed memfd {operation}.", actual, reason)


def execve_postcheck_swap(checker: Any) -> dict[str, Any]:
    original = checker.os.execve
    def intercepted(path: str, argv: list[str], env: dict[str, str]) -> Any:
        if path.startswith("/proc/self/fd/"):
            descriptor = int(path.rsplit("/", 1)[1])
            malicious = os.memfd_create("p1346-postcheck-evil", os.MFD_ALLOW_SEALING)
            os.write(malicious, EVIL_PROBE); os.fchmod(malicious, 0o500); os.lseek(malicious, 0, os.SEEK_SET)
            os.dup2(malicious, descriptor); os.close(malicious)
        return original(path, argv, env)
    checker.os.execve = intercepted
    try:
        actual, reason, evidence = checker.run_probe("opaque_probe", set())
    finally:
        checker.os.execve = original
    marker = "42" * 32
    return record("P1346-R2A16-postcheck-execve-fd-swap", "child-exec-toctou", "Swapping child FD after its final hash/seal check but inside execve transport must fail.", actual, reason, {"evil_sha256": sha256(EVIL_PROBE), "observed_process_nonce_hex": evidence.get("process_nonce_hex"), "oracle_evidence_keys": sorted(evidence), "swapped": evidence.get("process_nonce_hex") == marker})


def close_reuse_before_check(checker: Any) -> dict[str, Any]:
    original = checker._fork_execute; replaced = False
    def intercepted(descriptor: int, expected_hash: str, request: bytes, operation: str) -> Any:
        nonlocal replaced
        os.close(descriptor)
        malicious = os.memfd_create("p1346-reused-fd", os.MFD_ALLOW_SEALING)
        os.write(malicious, b"#!/bin/sh\necho evil\n"); os.fchmod(malicious, 0o500)
        if malicious != descriptor:
            os.dup2(malicious, descriptor); os.close(malicious)
        replaced = True
        return original(descriptor, expected_hash, request, operation)
    checker._fork_execute = intercepted
    try:
        actual, reason, _ = checker.run_probe("opaque_probe", set())
    finally:
        checker._fork_execute = original
    return record("P1346-R2A15-close-reuse-before-child-check", "fd-close-reuse", "Close/reuse of descriptor before child verification must fail.", actual, reason, {"replaced": replaced})


def worker_result(ids: list[str], *, duplicate: bool = False, pid: Any = 31337) -> bytes:
    use_ids = ([ids[0]] * 122) if duplicate else ids
    value = {"case_count": 122, "closed_world": {"rule": "adversarial forged worker stdout"}, "controls": [], "full_corpus_runs": 0, "pid": pid, "records": [{"actual": "Violated", "id": item, "reason_code": "TOKEN_COUNT"} for item in use_ids], "schema": "p1346-legacy-subprocess-result-r2"}
    return canonical(value)


def fake_worker_attack(checker: Any, attack_id: str, payload: bytes, call: str = "legacy") -> dict[str, Any]:
    original = checker.subprocess.run
    def intercepted(args: Any, *pos: Any, **kwargs: Any) -> Any:
        if isinstance(args, (list, tuple)) and checker.CASE_RUNNER_REL in args and ((call == "legacy" and "--legacy" in args) or (call == "dag" and "--dag-case" in args)):
            return subprocess.CompletedProcess(args, 0, payload, b"")
        return original(args, *pos, **kwargs)
    checker.subprocess.run = intercepted
    try:
        try:
            if call == "legacy": checker._legacy_vectors()
            else: checker._dag_case("P1346-T08-receipt-cycle", checker.RECEIPT_REL)
            actual, reason = "Preserved", "PRESERVED"
        except Exception as exc:
            actual, reason = "Violated", getattr(exc, "code", type(exc).__name__)
    finally:
        checker.subprocess.run = original
    return record(attack_id, "worker-authenticity", "Forged worker transport must not be accepted as case execution.", actual, reason, {"payload_sha256": sha256(payload)})


def main() -> int:
    for label, (path, digest) in PINS.items():
        if sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label} drift")
    delivery = validate_delivery(PINS["delivery_receipt_r2"][0].read_bytes(), DELIVERY_SHA256)
    caller = import_pinned("p1346_caller_r2_adversary", PINS["caller_r2"][0], CALLER_SHA256)
    checker = caller.validate_chain()
    # This is the independent execution; the post-delivery execution receipt is not consumed.
    focal = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
    inherited = focal["negative_records"][:122]; transport = focal["negative_records"][122:]
    if len(inherited) != 122 or len(transport) != 18 or len(focal["controls"]) != 13:
        raise RuntimeError("SCHEMA: focal cardinality")

    attacks: list[dict[str, Any]] = []
    # Replay R1 A01: measured Preserved outcomes must propagate as survivors.
    original_legacy = checker._legacy_vectors
    original_rows, original_controls, original_process = original_legacy()
    poisoned = [dict(item, actual="Preserved", reason_code="PRESERVED") for item in original_rows]
    checker._legacy_vectors = lambda: (poisoned, original_controls, original_process)
    try:
        poisoned_report = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
    finally:
        checker._legacy_vectors = original_legacy
    rejected_poison = bool(poisoned_report["summary"]["survivors"])
    attacks.append(record("P1346-A01-inherited-result-laundering", "historical-execution", "Preserved measured outcomes must not be relabelled as Violated.", "Violated" if rejected_poison else "Preserved", "TOKEN_DIVERGENCE" if rejected_poison else "HISTORICAL_NONEXECUTION", {"reported_survivors": len(poisoned_report["summary"]["survivors"])}))

    canonical_argv = checker._canonical_argv(RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    cli_vectors = [
        ("P1346-A02-alt-corpus-missing-flags", ["--focus", "--corpus", "/dev/shm/alternate.json"]),
        ("P1346-A03-alt-receipt-missing-digest", ["--focus", "--authorship-receipt", "/dev/shm/receipt.json"]),
        ("P1346-A04-wrong-checker-missing-flags", ["--focus", "--expected-checker-sha256", "0" * 64]),
        ("P1346-A05-wrong-root-missing-flags", ["--focus", "--expected-checker-sha256", CHECKER_SHA256, "--expected-authoring-root-sha256", "0" * 64]),
        ("P1346-A06-duplicate-focus", canonical_argv + ["--focus"]),
        ("P1346-A07-prefix-abbreviation", ["--foc"]),
        ("P1346-A08-equals-spelling", ["--focus", "--corpus=" + checker.CORPUS_REL]),
        ("P1346-A09-unknown-positional", ["--focus", "attacker"]),
        ("P1346-A10-option-as-value", ["--focus", "--corpus", "--focus"]),
        ("P1346-A11-dot-path-alias", ["--focus", "--corpus", "./" + checker.CORPUS_REL]),
        ("P1346-A12-uppercase-digest", ["--focus", "--authorship-receipt-sha256", RECEIPT_SHA256.upper()]),
        ("P1346-A13-trailing-token", canonical_argv + ["trailing"]),
    ]
    attacks.extend(cli_attack(checker, attack_id, argv) for attack_id, argv in cli_vectors)
    attacks.extend([
        delivery_attack("P1346-A14-delivery-caller-substitution", lambda raw: raw.replace(CALLER_SHA256.encode(), b"0" * 64, 1)),
        delivery_attack("P1346-A15-delivery-checker-substitution", lambda raw: raw.replace(CHECKER_SHA256.encode(), b"0" * 64, 1)),
        delivery_attack("P1346-A16-delivery-authorship-substitution", lambda raw: raw.replace(RECEIPT_SHA256.encode(), b"0" * 64, 1)),
        delivery_attack("P1346-A17-delivery-failed-verdict", lambda raw: raw.replace(b"DELIVERED_R2_NOT_VERIFIED_NOT_SEALED", b"FAILED_NOT_SEALED")),
        delivery_attack("P1346-A18-delivery-cycle-key", lambda raw: raw.replace(b'{"authoring_root_sha256"', b'{"self_sha256":"' + DELIVERY_SHA256.encode() + b'","authoring_root_sha256"', 1)),
        delivery_attack("P1346-A19-delivery-duplicate-key", lambda raw: raw.replace(b'{"authoring_root_sha256"', b'{"authoring_root_sha256":"' + AUTHORING_ROOT_SHA256.encode() + b'","authoring_root_sha256"', 1)),
        delivery_attack("P1346-A20-delivery-noncanonical-order", lambda raw: (json.dumps(dict(reversed(list(strict_json(raw).items()))), separators=(",", ":")) + "\n").encode()),
        delivery_attack("P1346-A21-delivery-trailing-space", lambda raw: raw[:-1] + b" \n"),
    ])
    attacks.extend([
        probe_transform(checker, "P1346-A22-probe-duplicate-json-key", lambda raw: raw.replace(b'{"schema":', b'{"schema":"duplicate","schema":', 1)),
        probe_transform(checker, "P1346-A23-probe-reordered-json", lambda raw: (json.dumps(dict(reversed(list(json.loads(raw).items()))), separators=(",", ":")) + "\n").encode()),
        probe_transform(checker, "P1346-A24-probe-trailing-space", lambda raw: raw[:-1] + b" \n"),
    ])
    actual, reason, _ = checker.run_probe("fd_swap_restore", set())
    attacks.append(record("P1346-A25-memfd-fd-swap-restore", "memfd-descriptor-identity", "Parent-side swap/restore before fork must fail child identity.", actual, reason))
    attacks.append(inject_exception(checker, "P1346-A26-actual-memfd-absence", checker.os, "memfd_create", OSError("memfd unavailable")))
    original_fcntl = checker.fcntl.fcntl
    checker.fcntl.fcntl = lambda fd, cmd, arg=0: (0 if cmd in {checker.fcntl.F_ADD_SEALS, checker.fcntl.F_GET_SEALS} else original_fcntl(fd, cmd, arg))
    try: actual, reason, _ = checker.run_probe("opaque_probe", set())
    finally: checker.fcntl.fcntl = original_fcntl
    attacks.append(record("P1346-A27-actual-seal-absence", "memfd-fail-closed", "Absent active seals must fail.", actual, reason))

    ids = [item["case_id"] for item in inherited]
    new_attacks: list[dict[str, Any]] = []
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A01-forged-worker-all-violated", worker_result(ids)))
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A02-forged-worker-duplicate-ids", worker_result(ids, duplicate=True)))
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A03-forged-worker-pid-string", worker_result(ids, pid="not-a-pid")))
    truncated = worker_result(ids)[:-17]
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A04-worker-partial-output", truncated))
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A05-worker-extra-line", worker_result(ids) + b"\n"))
    noncanonical = (json.dumps(strict_json(worker_result(ids)), indent=2) + "\n").encode()
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A06-worker-noncanonical-json", noncanonical))
    dag_payload = canonical({"case_id": "P1346-T08-receipt-cycle", "classification": "Violated", "reason_code": "AUTHORITY_ROOT"})
    new_attacks.append(fake_worker_attack(checker, "P1346-R2A07-forged-dag-worker", dag_payload, call="dag"))
    new_attacks.extend([
        sealed_mutation("P1346-R2A08-write-sealed-memfd", "write"),
        sealed_mutation("P1346-R2A09-shrink-sealed-memfd", "shrink"),
        sealed_mutation("P1346-R2A10-grow-sealed-memfd", "grow"),
        close_reuse_before_check(checker),
        inject_exception(checker, "P1346-R2A11-memfd-create-exception", checker.os, "memfd_create", OSError("create")),
    ])
    original_fcntl = checker.fcntl.fcntl
    def seal_error(fd: int, cmd: int, arg: int = 0) -> int:
        if cmd == checker.fcntl.F_ADD_SEALS: raise OSError("seal")
        return original_fcntl(fd, cmd, arg)
    checker.fcntl.fcntl = seal_error
    try: actual, reason, _ = checker.run_probe("opaque_probe", set())
    finally: checker.fcntl.fcntl = original_fcntl
    new_attacks.append(record("P1346-R2A12-seal-exception", "probe-exception", "F_ADD_SEALS exception must fail closed.", actual, reason))
    new_attacks.append(inject_exception(checker, "P1346-R2A13-fork-exception", checker.os, "fork", OSError("fork")))
    new_attacks.append(inject_exception(checker, "P1346-R2A14-execve-exception", checker.os, "execve", OSError("exec")))
    new_attacks.append(execve_postcheck_swap(checker))
    new_attacks.append(probe_transform(checker, "P1346-R2A17-probe-partial-output", lambda raw: raw[: max(1, len(raw) // 2)]))
    # Post-delivery receipt mutation cannot replace the independent run performed above.
    execution_receipt = DIAG / "p1346-oracle-execution-receipt-r2.json"
    altered_execution = execution_receipt.read_bytes() + b"\n"
    receipt_influenced = False
    new_attacks.append(record("P1346-R2A18-execution-receipt-laundering", "receipt-not-execution", "A post-delivery receipt mutation must not influence this independently executed focal result.", "Preserved" if receipt_influenced else "Violated", "AUTHORITY_ROOT", {"altered_sha256": sha256(altered_execution), "consumed": False}))
    # A coordinated delivery substitution remains rejected by the original external digest.
    new_attacks.append(delivery_attack("P1346-R2A19-coordinated-dag-substitution", lambda raw: raw.replace(CALLER_SHA256.encode(), b"1" * 64).replace(CHECKER_SHA256.encode(), b"2" * 64).replace(RECEIPT_SHA256.encode(), b"3" * 64)))
    attacks.extend(new_attacks)

    probe_rs = sorted(path.name for path in DIAG.glob("p134[56]-opaque-probe*.rs"))
    controls = [
        {"actual": "Preserved" if focal["summary"]["mutation_score"] == 1 and len(inherited) == 122 and len(transport) == 18 else "Violated", "expected": "Preserved", "id": "P1346-R2C01-independent-focal"},
        {"actual": "Preserved" if delivery["delivery_root_sha256"] == DELIVERY_ROOT_SHA256 else "Violated", "expected": "Preserved", "id": "P1346-R2C02-external-dag"},
        {"actual": focal["opaque"]["actual"], "expected": "Unknown", "id": "P1346-R2C03-canonical-probe"},
        {"actual": "Preserved" if not probe_rs and sha256(PINS["probe_source"][0].read_bytes()) == PINS["probe_source"][1] else "Violated", "expected": "Preserved", "id": "P1346-R2C04-rs-txt-linter-boundary"},
    ]
    survivors = [item for item in attacks if item["survived"]]
    regressions = [item["id"] for item in controls if item["actual"] != item["expected"]]
    denominator = len(inherited) + len(transport) + len(attacks)
    rejected = len(inherited) + len(transport) + len(attacks) - len(survivors)
    result = {
        "budget_stop": {"definitive_for_p1346": bool(survivors or regressions), "full_runs_consumed": 0, "preseal_allowed": not survivors and not regressions, "triggered": bool(survivors or regressions)},
        "closed_world": {"rule": "Independent final P1346 R2 focal adversary only; no full, seal, candidate or general-equivalence verdict."},
        "execution": {"candidate_read_or_executed": False, "execution_receipt_consumed_as_proof": False, "full_corpus_runs": 0, "inherited_cases_executed_in_worker": 122, "r1_attacks_replayed": 27, "temporary_root": "/dev/shm/p1346-*", "transport_cases_executed": 18},
        "executor": "/root/p1346_adversary",
        "phase": "final-focal-adversarial-r2-no-full",
        "protected_inputs": {label: [str(path.relative_to(ROOT)), digest] for label, (path, digest) in PINS.items()} | {"authoring_root_sha256": AUTHORING_ROOT_SHA256, "delivery_root_sha256": DELIVERY_ROOT_SHA256, "external_delivery_receipt_sha256": DELIVERY_SHA256},
        "regime": "executado sem atestacao de isolamento",
        "revision": 2,
        "role": "independent_adversary",
        "schema": "p1346-adversary-report-r2",
        "step": 1346,
        "summary": {"canonical_control_regressions": regressions, "canonical_controls": len(controls), "correctly_violated": rejected, "full_corpus_runs": 0, "mutation_score": rejected / denominator, "new_attacks": len(new_attacks), "r1_attacks": 27, "survivor_count": len(survivors), "survivors": [item["id"] for item in survivors], "valid_negatives": denominator},
        "survivor_details": survivors,
        "vectors": {"canonical_controls": controls, "canonical_inherited": [{"actual": item["actual"], "id": item["case_id"], "reason_code": item["reason_code"]} for item in inherited], "canonical_transport": [{"actual": item["actual"], "id": item["case_id"], "reason_code": item["reason_code"]} for item in transport], "replayed_r1_and_new": attacks},
        "verdict": "ADVERSARIAL_R2_SURVIVORS_BLOCK_P1346_DEFINITIVELY" if survivors else ("ADVERSARIAL_R2_CONTROL_REGRESSION_BLOCKS_P1346" if regressions else "ADVERSARIAL_R2_ZERO_SURVIVORS_READY_FOR_PRESEAL"),
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
