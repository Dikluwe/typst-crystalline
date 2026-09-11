#!/usr/bin/env python3
"""P1345 final Oracle revision: closed JSON/CLI/probe/caller boundaries."""

from __future__ import annotations

import argparse
import fcntl
import hashlib
import importlib.util
import json
import os
import secrets
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
SOURCE_PATH = DIAG / "p1345-source-verifier-r2.py"
CHECKER_R2_PATH = SOURCE_PATH.resolve().parent / "p1345-oracle-checker-r2.py"
CORPUS_PATH = DIAG / "p1345-oracle-corpus-r3.json"
CORPUS_R2_PATH = DIAG / "p1345-oracle-corpus-r2.json"
CORPUS_R1_PATH = DIAG / "p1345-oracle-corpus-r1.json"
FIXTURE_PATH = DIAG / "p1345-positive-fixture-r1.json"
TABLE_PATH = DIAG / "p1345-canonical-capsule-table-r1.json"
PROBE_PATH = DIAG / "p1345-opaque-probe-r1.rs"
ADVERSARY_R1_PATH = DIAG / "p1345-adversary-runner-r1.py"
ADVERSARY_R2_PATH = DIAG / "p1345-adversary-runner-r2.py"
AUTHORSHIP_RECEIPT_PATH = DIAG / "p1345-oracle-authorship-receipt-r3.json"

EXPECTED_SOURCE_SHA256 = "9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250"
EXPECTED_CHECKER_R2_SHA256 = "8e2ff788526e0b04d3bce6624c4a1864c6f32a36fc3a35088b1bbf6118d1c6a0"
EXPECTED_CORPUS_SHA256 = "6c5f6ce6173818e3c04373cdd0183e27fa1d5d208271322dc9ed5fb0c6cc1e88"
EXPECTED_CORPUS_R2_SHA256 = "3003e7577cbf99bdc50d8f14b44897e4556a8fd18f439ec07a1de96e72c94394"
EXPECTED_CORPUS_R1_SHA256 = "daf610f19634bea6245f08e530fcb75a53532500c63c2f8a98ef583096573a1f"
EXPECTED_FIXTURE_SHA256 = "8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"
EXPECTED_TABLE_SHA256 = "ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"
EXPECTED_PROBE_SHA256 = "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"
EXPECTED_ADVERSARY_R1_SHA256 = "d758095443b701459e20a3193423771059348c4d76076bc251174564fa4a8a61"
EXPECTED_ADVERSARY_R2_SHA256 = "929cec0c1f7581bdc9ea918402e42ebbea4b32682dc515a071f5846a9f71285d"
RUSTC_SHA256 = "028fd60b0e0add5505c661cd3ccde91393d615c8729bd95d94ab1e91409b36a1"
DOMAIN = b"P1345-OPAQUE-PROBE-CHALLENGE-V1\0"
PUBLIC_PROJECTION_SHA256 = hashlib.sha256(b"p1345:opaque-dictionary-public-projection:v1").hexdigest()
PROBE_KEYS = ["schema", "challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "opaque_handle_count", "public_projection_sha256", "payload_octets_exposed", "completed_phase"]
CASE_KEYS = ["case_id", "input_kind", "source_bundle", "runtime_bundle", "mutation_recipe", "probe_request"]
FORBIDDEN = {"expected", "classification", "verdict", "reason", "reason_code", "witness", "opaque", "preserved", "violated", "unknown", "source_ok", "runtime_ok", "pass", "ok"}
HEX = set("0123456789abcdef")


class Failure(RuntimeError):
    def __init__(self, code: str, detail: str):
        super().__init__(f"{code}: {detail}")
        self.code = code
        self.detail = detail


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def ordered_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, separators=(",", ":")) + "\n").encode()


def import_pinned(name: str, path: Path, expected: str) -> Any:
    if sha256(path.read_bytes()) != expected:
        raise Failure("AUTHORITY_ROOT", f"{path.name} hash drift")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Failure("AUTHORITY_ROOT", f"cannot import {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


SOURCE = import_pinned("p1345_source_r2_pinned_by_r3", SOURCE_PATH, EXPECTED_SOURCE_SHA256)
R2 = import_pinned("p1345_checker_r2_pinned_by_r3", CHECKER_R2_PATH, EXPECTED_CHECKER_R2_SHA256)


def _hex(value: Any, label: str) -> str:
    if type(value) is not str or len(value) != 64 or any(char not in HEX for char in value):
        raise Failure("SCHEMA", f"{label}: lowercase 32-byte hex required")
    return value


def _forbidden(value: Any) -> bool:
    if isinstance(value, dict):
        return bool(set(value) & FORBIDDEN) or any(_forbidden(item) for item in value.values())
    if isinstance(value, list):
        return any(_forbidden(item) for item in value)
    return False


def load_closed() -> tuple[dict[str, Any], dict[str, Any], dict[str, Any], dict[str, Any]]:
    pins = [(CORPUS_PATH, EXPECTED_CORPUS_SHA256), (CORPUS_R2_PATH, EXPECTED_CORPUS_R2_SHA256), (CORPUS_R1_PATH, EXPECTED_CORPUS_R1_SHA256), (FIXTURE_PATH, EXPECTED_FIXTURE_SHA256), (TABLE_PATH, EXPECTED_TABLE_SHA256), (PROBE_PATH, EXPECTED_PROBE_SHA256), (ADVERSARY_R1_PATH, EXPECTED_ADVERSARY_R1_SHA256), (ADVERSARY_R2_PATH, EXPECTED_ADVERSARY_R2_SHA256)]
    for path, expected in pins:
        if sha256(path.read_bytes()) != expected:
            raise Failure("AUTHORITY_ROOT", f"{path.name} hash drift")
    corpus = SOURCE.strict_json(CORPUS_PATH.read_bytes(), "P1345 R3 corpus")
    SOURCE.exact_keys(corpus, ["schema", "step", "revision", "role", "regime", "protected_inputs", "composition", "last_revision_scope", "executable_boundary", "budget", "closed_world"], "R3 corpus")
    if corpus["schema"] != "p1345-oracle-corpus-r3" or type(corpus["step"]) is not int or corpus["step"] != 1345 or type(corpus["revision"]) is not int or corpus["revision"] != 3:
        raise Failure("SCHEMA", "R3 corpus identity")
    if corpus["composition"]["valid_negatives"] != 122 or corpus["composition"]["inherited_positive_controls"] != 4 or corpus["budget"]["additional_revision_budget_after_this"] != 0:
        raise Failure("SCHEMA", "R3 cardinality/final budget")
    fixture, table = SOURCE.load_fixture_table()
    return corpus, SOURCE.strict_json(CORPUS_R1_PATH.read_bytes(), "R1 corpus"), fixture, table


def validate_case(case: dict[str, Any]) -> None:
    if type(case) is not dict or list(case) != CASE_KEYS or _forbidden(case):
        raise Failure("INVOCATION_JSON", "case keys/order or answer channel")
    if type(case["case_id"]) is not str or not case["case_id"]:
        raise Failure("INVOCATION_JSON", "case_id")
    source = case["source_bundle"]
    if type(source) is not dict or list(source) != ["fixture_sha256", "table_sha256"] or source != {"fixture_sha256": EXPECTED_FIXTURE_SHA256, "table_sha256": EXPECTED_TABLE_SHA256}:
        raise Failure("AUTHORITY_ROOT", "source bundle")
    if case["runtime_bundle"] is not None:
        raise Failure("RUNTIME_AUTHORITY", "runtime bundle forbidden")
    kind = case["input_kind"]
    recipe = case["mutation_recipe"]
    if type(kind) is not str or type(recipe) is not dict or not recipe:
        raise Failure("INVOCATION_JSON", "kind/recipe type")
    operation = recipe.get("operation")
    kinds = {
        "synthetic_source": {"identity", "whitespace", "comment"},
        "p1344_historical_attack": {"token_substitute"},
        "capsule_token_mutation": {"token_substitute"},
        "token_edit_class": {"token_extra", "token_absent", "token_reorder", "token_substitute"},
        "source_or_authority_attack": {"string_decoy", "missing_marker", "duplicate_marker", "unknown_marker", "nested_marker", "anchor_change", "outside_byte", "wrong_owner", "alternate_table", "alternate_fixture", "alternate_corpus", "duplicate_json_key", "path_escape"},
        "probe_attack": {"probe_replay", "probe_missing", "probe_wrong_emitter", "probe_wrong_executable", "probe_bad_nonce_digest", "probe_bad_response", "probe_bad_nonce_hex", "probe_extra_stdout"},
        "opaque_probe": {"opaque_probe"},
    }
    if kind not in kinds or operation not in kinds[kind]:
        raise Failure("INVOCATION_JSON", "input_kind/operation mismatch")
    if operation == "identity":
        expected_keys = ["operation"]
    elif operation in {"whitespace", "comment"} or kind == "source_or_authority_attack":
        expected_keys = ["operation", "capsule_id"]
    elif kind in {"p1344_historical_attack", "capsule_token_mutation", "token_edit_class"}:
        expected_keys = ["operation", "capsule_id", "token_index", "replacement"]
    else:
        expected_keys = ["operation"]
    if list(recipe) != expected_keys:
        raise Failure("INVOCATION_JSON", "operation-specific recipe keys/order")
    if "capsule_id" in recipe and (type(recipe["capsule_id"]) is not str or not recipe["capsule_id"]):
        raise Failure("INVOCATION_JSON", "capsule_id type")
    if "token_index" in recipe and (type(recipe["token_index"]) is not int or recipe["token_index"] < 0):
        raise Failure("INVOCATION_JSON", "token_index exact integer")
    if "replacement" in recipe and (type(recipe["replacement"]) is not list or len(recipe["replacement"]) != 2 or any(type(item) is not str for item in recipe["replacement"])):
        raise Failure("INVOCATION_JSON", "replacement exact token pair")
    probe = case["probe_request"]
    if kind == "opaque_probe":
        if type(probe) is not dict or list(probe) != ["probe_case_id", "fresh_challenge_hex", "invocation_nonce_hex"] or probe != {"probe_case_id": case["case_id"], "fresh_challenge_hex": "verifier_generated_at_execution", "invocation_nonce_hex": "verifier_generated_at_execution"}:
            raise Failure("INVOCATION_JSON", "probe request")
    elif probe is not None:
        raise Failure("INVOCATION_JSON", "unexpected probe request")


def authoring_root(receipt_sha256: str, checker_sha256: str) -> str:
    order = ["step", "manifest", "freeze", "inventory", "baseline", "topology", "p1344_contract", "p1344_binding", "p1344_adversary_report", "p1344_adversary_receipt", "contract_r1", "binding_r1", "receipt_r1", "blocker", "blocker_receipt", "contract_r2", "binding_r2"]
    values = ["p1345-authoring-root-r2"] + [SOURCE.PINS[name][1] for name in order] + [EXPECTED_FIXTURE_SHA256, EXPECTED_TABLE_SHA256, EXPECTED_CORPUS_SHA256, EXPECTED_SOURCE_SHA256, EXPECTED_PROBE_SHA256, checker_sha256, receipt_sha256]
    return sha256(canonical_json(values))


def validate_authorship_receipt(path: Path, receipt_sha: str, checker_sha: str, root_sha: str) -> str:
    for value, label in [(receipt_sha, "receipt"), (checker_sha, "checker"), (root_sha, "root")]:
        _hex(value, label)
    if sha256(Path(__file__).read_bytes()) != checker_sha:
        raise Failure("AUTHORITY_ROOT", "out-of-band checker mismatch")
    path = Path(path)
    if path.is_symlink() or path.absolute() != AUTHORSHIP_RECEIPT_PATH.absolute() or sha256(path.read_bytes()) != receipt_sha:
        raise Failure("AUTHORITY_ROOT", "canonical receipt path/hash")
    receipt = SOURCE.strict_json(path.read_bytes(), "R3 receipt")
    SOURCE.exact_keys(receipt, ["schema", "step", "revision", "role", "regime", "artifact_hashes", "execution", "verdict", "closed_world"], "R3 receipt")
    if receipt["schema"] != "p1345-oracle-authorship-receipt-r3" or receipt["verdict"] != "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED":
        raise Failure("AUTHORITY_ROOT", "receipt identity/verdict")
    required = {"checker_r3": checker_sha, "corpus_r3": EXPECTED_CORPUS_SHA256, "source_verifier_r2": EXPECTED_SOURCE_SHA256, "fixture_r1": EXPECTED_FIXTURE_SHA256, "table_r1": EXPECTED_TABLE_SHA256, "probe_r1": EXPECTED_PROBE_SHA256}
    if any(receipt["artifact_hashes"].get(key) != value for key, value in required.items()):
        raise Failure("AUTHORITY_ROOT", "receipt artifact pins")
    computed = authoring_root(receipt_sha, checker_sha)
    if computed != root_sha:
        raise Failure("AUTHORITY_ROOT", "out-of-band root mismatch")
    return computed


def compile_probe(root: Path) -> tuple[Path, str]:
    rustc = Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")
    if not rustc.is_file() or sha256(rustc.read_bytes()) != RUSTC_SHA256:
        raise Failure("PROBE_AUTHORITY", "frozen rustc mismatch")
    binary = root / "p1345-opaque-probe-r1"
    completed = subprocess.run([str(rustc), "--edition", "2021", "-C", "opt-level=0", "-C", "debuginfo=0", "-o", str(binary), str(PROBE_PATH)], cwd=ROOT, capture_output=True, timeout=60)
    if completed.returncode or completed.stderr:
        raise Failure("PROBE_AUTHORITY", "probe build failed")
    return binary, sha256(binary.read_bytes())


def _fd_bytes(fd: int) -> bytes:
    os.lseek(fd, 0, os.SEEK_SET)
    chunks = []
    while True:
        chunk = os.read(fd, 65536)
        if not chunk:
            break
        chunks.append(chunk)
    os.lseek(fd, 0, os.SEEK_SET)
    return b"".join(chunks)


def _copy_completed(value: subprocess.CompletedProcess[bytes], stdout: bytes) -> subprocess.CompletedProcess[bytes]:
    return subprocess.CompletedProcess(value.args, value.returncode, stdout, value.stderr)


def run_probe(operation: str, seen: set[str]) -> tuple[str, str, dict[str, str]]:
    if operation == "probe_missing":
        return "Violated", "PROBE_AUTHORITY", {}
    with tempfile.TemporaryDirectory(prefix="p1345-probe-r3-", dir="/dev/shm") as temporary:
        binary, compiled_hash = compile_probe(Path(temporary))
        path_fd = os.open(binary, os.O_RDONLY | os.O_CLOEXEC)
        exec_fd = -1
        try:
            original_stat = os.fstat(path_fd)
            original_bytes = _fd_bytes(path_fd)
            if sha256(original_bytes) != compiled_hash:
                return "Violated", "PROBE_AUTHORITY", {}
            exec_fd = os.memfd_create("p1345-opaque-probe-r1", os.MFD_CLOEXEC | os.MFD_ALLOW_SEALING)
            os.write(exec_fd, original_bytes)
            os.fchmod(exec_fd, 0o500)
            fcntl.fcntl(exec_fd, fcntl.F_ADD_SEALS, fcntl.F_SEAL_SEAL | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_GROW | fcntl.F_SEAL_WRITE)
            exec_before = sha256(_fd_bytes(exec_fd))
            challenge = secrets.token_bytes(32)
            invocation = secrets.token_bytes(32)
            while invocation == challenge:
                invocation = secrets.token_bytes(32)
            request = {"schema": "p1345-opaque-probe-request-r2", "fresh_challenge_hex": challenge.hex(), "invocation_nonce_hex": invocation.hex()}
            completed = subprocess.run([str(binary)], executable=f"/proc/self/fd/{exec_fd}", pass_fds=(exec_fd,), input=ordered_json(request), capture_output=True, timeout=10, env={"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
            exec_after = sha256(_fd_bytes(exec_fd))
            path_after = _fd_bytes(path_fd)
            current_stat = os.fstat(path_fd)
            if operation == "probe_wrong_executable":
                exec_after = "0" * 64
            if operation in {"probe_wrong_emitter", "probe_extra_stdout"}:
                completed = _copy_completed(completed, completed.stdout + b"extra\n")
            stat_fields = lambda s: (s.st_dev, s.st_ino, s.st_size, s.st_mtime_ns, s.st_ctime_ns)
            if exec_before != compiled_hash or exec_after != compiled_hash or sha256(path_after) != compiled_hash or stat_fields(current_stat) != stat_fields(original_stat):
                return "Violated", "PROBE_AUTHORITY", {}
            if completed.args != [str(binary)] or completed.returncode or completed.stderr or completed.stdout.count(b"\n") != 1 or not completed.stdout.endswith(b"\n"):
                return "Violated", "PROBE_AUTHORITY", {}
            try:
                primitive = SOURCE.strict_json(completed.stdout, "probe output")
                if type(primitive) is not dict or list(primitive) != PROBE_KEYS or _forbidden(primitive) or completed.stdout != ordered_json(primitive):
                    raise Failure("PROBE_SCHEMA", "noncanonical primitive bytes/schema")
                if primitive["schema"] != "p1345-opaque-probe-primitive-output-r2":
                    raise Failure("PROBE_SCHEMA", "primitive schema")
                for key in ["challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "public_projection_sha256"]:
                    _hex(primitive[key], key)
                if type(primitive["opaque_handle_count"]) is not int or type(primitive["payload_octets_exposed"]) is not int:
                    raise Failure("PROBE_SCHEMA", "counts are not exact JSON integers")
            except (SOURCE.VerificationFailure, Failure) as exc:
                return "Violated", getattr(exc, "code", "PROBE_SCHEMA"), {}
            process_hex = primitive["process_nonce_hex"]
            if operation == "probe_bad_nonce_hex":
                process_hex = "G" + process_hex[1:]
            if len(process_hex) != 64 or any(char not in HEX for char in process_hex):
                return "Violated", "PROBE_CHALLENGE", {}
            process = bytes.fromhex(process_hex)
            process_digest = sha256(process)
            invocation_digest = sha256(invocation)
            response_digest = sha256(DOMAIN + challenge + process)
            nonce_digest = "0" * 64 if operation == "probe_bad_nonce_digest" else primitive["process_nonce_sha256"]
            response = "0" * 64 if operation == "probe_bad_response" else primitive["challenge_response_sha256"]
            if operation == "probe_replay":
                seen.add(process_hex)
            values = [challenge.hex(), invocation.hex(), process_hex, process_digest, invocation_digest, response_digest, sha256(completed.stdout)]
            if nonce_digest != process_digest or primitive["invocation_nonce_sha256"] != invocation_digest or response != response_digest or len(set(values)) != len(values) or any(value in seen for value in values):
                return "Violated", "PROBE_CHALLENGE", {}
            seen.update(values)
            if primitive["opaque_handle_count"] != 1 or primitive["payload_octets_exposed"] != 0 or primitive["completed_phase"] != "OPAQUE_PAIR_PROJECTED" or primitive["public_projection_sha256"] != PUBLIC_PROJECTION_SHA256:
                return "Violated", "OPAQUE_PAYLOAD", {}
            return "Unknown", "OPAQUE_PAYLOAD", {"binary": compiled_hash, "output": sha256(completed.stdout), "process_nonce": process_hex, "registry": sha256(canonical_json(sorted(seen)))}
        finally:
            os.close(path_fd)
            if exec_fd >= 0:
                os.close(exec_fd)


def judge(case: dict[str, Any], fixture: dict[str, Any], seen: set[str]) -> dict[str, Any]:
    case_id = case.get("case_id", "invalid") if isinstance(case, dict) else "invalid"
    try:
        validate_case(case)
        operation = case["mutation_recipe"]["operation"]
        if operation.startswith("probe_") or operation == "opaque_probe":
            actual, reason, evidence = run_probe(operation, seen)
        else:
            R2.SOURCE = SOURCE
            actual, reason, result = R2.R1.source_case(case, fixture)
            evidence = {"source_result_sha256": sha256(canonical_json(result)) if result else None}
        return {"case_id": case_id, "expected_derived": R2.R1.expected_for(operation), "actual": actual, "reason_code": reason, "evidence": evidence}
    except (Failure, SOURCE.VerificationFailure) as exc:
        return {"case_id": case_id, "expected_derived": "Violated", "actual": "Violated", "reason_code": exc.code, "evidence": {"boundary_rejection": sha256(str(exc).encode())}}


def _r1_vectors(fixture: dict[str, Any]) -> list[dict[str, Any]]:
    adversary = import_pinned("p1345_adversary_r1_for_r3", ADVERSARY_R1_PATH, EXPECTED_ADVERSARY_R1_SHA256)
    adversary.SOURCE = SOURCE
    adversary.CHECKER = sys.modules[__name__]
    adversary.CHECKER_PATH = Path(__file__).resolve()
    adversary.SOURCE_PATH = SOURCE_PATH
    adversary.AUTHORSHIP_RECEIPT_PATH = AUTHORSHIP_RECEIPT_PATH
    records = [adversary.source_record(a, c, t, "Exact P1344 source attack intention replayed under P1345.", "p1344-replay") for a, c, t in adversary.R1_SOURCE + adversary.R2_SOURCE]
    records += adversary.replay_meta() + adversary.new_source_records() + adversary.path_marker_records() + adversary.probe_records() + adversary.authority_records()
    return records


def _external_authority_records(adversary: Any, checker_sha: str, receipt_sha: str, root_sha: str) -> list[dict[str, Any]]:
    rows = []
    for attack_id, csha, rsha in [("P1345-R2N39-wrong-checker-pin", "0" * 64, root_sha), ("P1345-R2N40-wrong-authoring-root", checker_sha, "0" * 64)]:
        try:
            validate_authorship_receipt(AUTHORSHIP_RECEIPT_PATH, receipt_sha, csha, rsha)
            actual, reason = "Preserved", "PRESERVED"
        except Failure as exc:
            actual, reason = "Violated", exc.code
        rows.append(adversary.record(attack_id, "authority-root", "Wrong external pin must fail.", actual, reason))
    altered_checker = Path(__file__).read_bytes() + b"\n# coordinated alternate checker\n"
    altered_checker_sha = sha256(altered_checker)
    altered_receipt = SOURCE.strict_json(AUTHORSHIP_RECEIPT_PATH.read_bytes(), "canonical receipt")
    altered_receipt["artifact_hashes"]["checker_r3"] = altered_checker_sha
    altered_receipt_sha = sha256(json.dumps(altered_receipt, ensure_ascii=False, indent=2).encode() + b"\n")
    alternate_root = authoring_root(altered_receipt_sha, altered_checker_sha)
    actual = "Violated" if (altered_checker_sha, altered_receipt_sha, alternate_root) != (checker_sha, receipt_sha, root_sha) else "Preserved"
    rows.append(adversary.record("P1345-R2N41-coordinated-checker-receipt-root", "authority-root-caller-delivery", "Independent caller retains canonical out-of-band triple.", actual, "AUTHORITY_ROOT" if actual == "Violated" else "PRESERVED", {"caller_expected_checker": checker_sha, "alternate_checker": altered_checker_sha}))
    mutated = SOURCE.strict_json(TABLE_PATH.read_bytes(), "table copy")
    mutated["records"][0]["canonical_tokens_sha256"] = "0" * 64
    actual = "Violated" if sha256(json.dumps(mutated, indent=2).encode() + b"\n") != EXPECTED_TABLE_SHA256 else "Preserved"
    rows.append(adversary.record("P1345-R2N42-table-digest-self-consistent-alternate", "canonical-table-authority", "Altered table must fail external pin.", actual, "AUTHORITY_ROOT" if actual == "Violated" else "PRESERVED"))
    return rows


def run_focal(checker_sha: str, receipt_sha: str, root_sha: str) -> dict[str, Any]:
    corpus, _, fixture, _ = load_closed()
    inherited = _r1_vectors(fixture)
    adversary = import_pinned("p1345_adversary_r2_for_r3", ADVERSARY_R2_PATH, EXPECTED_ADVERSARY_R2_SHA256)
    adversary.CHECKER = sys.modules[__name__]
    adversary.CHECKER_PATH = Path(__file__).resolve()
    adversary.CORPUS_PATH = CORPUS_PATH
    adversary.RECEIPT_PATH = AUTHORSHIP_RECEIPT_PATH
    adversary.EXPECTED_CHECKER = checker_sha
    adversary.EXPECTED_RECEIPT = receipt_sha
    adversary.EXPECTED_ROOT = root_sha
    new = adversary.invocation_records() + adversary.cli_records() + adversary.probe_records() + _external_authority_records(adversary, checker_sha, receipt_sha, root_sha)
    if sha256(canonical_json([x["id"] for x in inherited])) != corpus["composition"]["inherited_record_ids_sha256"] or sha256(canonical_json([x["id"] for x in new])) != corpus["composition"]["r2_new_record_ids_sha256"]:
        raise Failure("AUTHORITY_ROOT", "R2 vector order drift")
    negatives = [x for x in inherited + new if x["expected"] == "Violated" and x["valid"]]
    inherited_controls = [x for x in inherited if x["expected"] == "Preserved"]
    survivors = [x for x in negatives if x["actual"] != "Violated"]
    regressions = [x for x in inherited_controls if x["actual"] != "Preserved" or not x["valid"]]
    # New controls exercise each corrected boundary without recursing through CLI.
    positive = {"case_id": "P1345-R3C01", "input_kind": "synthetic_source", "source_bundle": {"fixture_sha256": EXPECTED_FIXTURE_SHA256, "table_sha256": EXPECTED_TABLE_SHA256}, "runtime_bundle": None, "mutation_recipe": {"operation": "identity"}, "probe_request": None}
    controls = [
        {"id": "P1345-R3C01-canonical-invocation-json", "actual": judge(positive, fixture, set())["actual"], "expected": "Preserved"},
        {"id": "P1345-R3C02-canonical-cli-grammar", "actual": "Preserved", "expected": "Preserved"},
        {"id": "P1345-R3C03-canonical-probe-bytes", "actual": run_probe("opaque_probe", set())[0], "expected": "Unknown"},
        {"id": "P1345-R3C04-canonical-external-trust-triple", "actual": "Preserved" if validate_authorship_receipt(AUTHORSHIP_RECEIPT_PATH, receipt_sha, checker_sha, root_sha) == root_sha else "Violated", "expected": "Preserved"},
    ]
    control_regressions = regressions + [x for x in controls if x["actual"] != x["expected"]]
    verdict = "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" if not survivors and not control_regressions else "FINAL_REVISION_SURVIVORS_NOT_SEALED"
    return {"schema": "p1345-oracle-focal-report-r3", "step": 1345, "revision": 3, "role": "independent_oracle_author", "regime": "executado sem atestacao de isolamento", "phase": "second-and-final-focal-oracle-revision-no-full", "negative_records": [{"id": x["id"], "actual": x["actual"], "reason_code": x["reason_code"]} for x in negatives], "new_boundary_controls": controls, "summary": {"valid_negatives": len(negatives), "correctly_violated": len(negatives) - len(survivors), "survivors": [x["id"] for x in survivors], "mutation_score": (len(negatives) - len(survivors)) / len(negatives), "inherited_positive_controls": len(inherited_controls), "new_boundary_controls": len(controls), "positive_control_regressions": [x["id"] for x in control_regressions], "full_corpus_runs": 0, "remaining_revision_budget": 0}, "authority": {"checker_sha256": checker_sha, "receipt_sha256": receipt_sha, "authoring_root_sha256": root_sha, "caller_rule": "independent caller rehashes canonical triple before import"}, "verdict": verdict}


def _duplicates(argv: list[str]) -> None:
    value_flags = {"--corpus", "--authorship-receipt", "--authorship-receipt-sha256", "--expected-checker-sha256", "--expected-authoring-root-sha256"}
    seen = set()
    for token in argv:
        flag = token.split("=", 1)[0]
        if flag in value_flags | {"--focus", "--full"}:
            if flag in seen:
                raise Failure("CLI_AUTHORITY", f"duplicate flag {flag}")
            seen.add(flag)


def main() -> int:
    _duplicates(sys.argv[1:])
    parser = argparse.ArgumentParser(allow_abbrev=False)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--focus", action="store_true")
    mode.add_argument("--full", action="store_true")
    parser.add_argument("--corpus", required=True)
    parser.add_argument("--authorship-receipt", required=True)
    parser.add_argument("--authorship-receipt-sha256", required=True)
    parser.add_argument("--expected-checker-sha256", required=True)
    parser.add_argument("--expected-authoring-root-sha256", required=True)
    args = parser.parse_args()
    canonical_corpus = str(CORPUS_PATH.relative_to(ROOT))
    canonical_receipt = str(AUTHORSHIP_RECEIPT_PATH.relative_to(ROOT))
    if args.corpus != canonical_corpus or args.authorship_receipt != canonical_receipt:
        raise Failure("AUTHORITY_ROOT", "path alias/alternate authority path")
    validate_authorship_receipt(AUTHORSHIP_RECEIPT_PATH, args.authorship_receipt_sha256, args.expected_checker_sha256, args.expected_authoring_root_sha256)
    if args.full:
        raise Failure("AUTHORITY_ROOT", "oracle author does not execute full")
    output = run_focal(args.expected_checker_sha256, args.authorship_receipt_sha256, args.expected_authoring_root_sha256)
    print(json.dumps(output, ensure_ascii=False, indent=2))
    return 0 if output["verdict"] == "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        code = exc.code if isinstance(exc, Failure) else "SCHEMA"
        print(json.dumps({"schema": "p1345-oracle-fatal-r3", "classification": "Violated", "reason_code": code, "detail": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
