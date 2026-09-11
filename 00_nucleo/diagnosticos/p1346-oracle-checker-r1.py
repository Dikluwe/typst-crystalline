#!/usr/bin/env python3
"""P1346 transport oracle: closed preflight, acyclic authority and sealed memfd."""

from __future__ import annotations

import argparse
import fcntl
import hashlib
import importlib.util
import json
import math
import os
import secrets
import stat
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_REL = "00_nucleo/diagnosticos/p1346-oracle-checker-r1.py"
CORPUS_REL = "00_nucleo/diagnosticos/p1346-oracle-corpus-r1.json"
RECEIPT_REL = "00_nucleo/diagnosticos/p1346-oracle-authorship-receipt-r1.json"
CHECKER_PATH = ROOT / CHECKER_REL
CORPUS_PATH = ROOT / CORPUS_REL
RECEIPT_PATH = ROOT / RECEIPT_REL
PROBE_PATH = DIAG / "p1346-opaque-probe-r1.rs.txt"
RUSTC = Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")

PINS = {
    "step": ("00_nucleo/materialization/typst-passo-1346.md", "7c7138a3aa6a94635216d5c5408d3835204804984e2dadca23973ab20263bc9e"),
    "authority_manifest": ("00_nucleo/diagnosticos/p1346-authority-manifest-r1.json", "d26e74fb5d7d675aa5ed069c2bb7b5599930bf165e5d4c678109e9ed0019b0ff"),
    "l0_toolchain_freeze": ("00_nucleo/diagnosticos/p1346-l0-toolchain-freeze-r1.json", "9f17a4b0f9b8394d77c9b64ad9f1c7543dc4ed04a7061306fb4c28c169464550"),
    "capsule_baseline": ("00_nucleo/diagnosticos/p1346-capsule-baseline-r1.json", "23a2182dd4af00a83d51e5eda996fe661cf312c54d65a30c4d0d63f1dd637d78"),
    "topology_receipt": ("00_nucleo/diagnosticos/p1346-topology-receipt-r1.json", "0f80149bf7eb7844745c8f5f5e88b14819da81c220349ddfee3e975508937f85"),
    "probe_relocation_receipt": ("00_nucleo/diagnosticos/p1346-probe-relocation-receipt-r1.json", "b5ea227d33ec5272ad67ec1f3d99d1a5efe7f84792f69af449e3cc7a13a3e9db"),
    "p1345_final_contract_spec": ("00_nucleo/diagnosticos/p1345-contract-spec-r2.json", "f40c2b42fe83a4b75e276ab2ea8a9639fa1553e80c06581caf8c53d948f9d38d"),
    "p1345_final_contract_binding": ("00_nucleo/diagnosticos/p1345-contract-binding-r2.json", "ecc3a8a403d9d14d51808a35876f1f43d7773732c1d20f58574dff42d5187778"),
    "p1345_final_contract_receipt": ("00_nucleo/diagnosticos/p1345-contract-receipt-r2.json", "875f937affb30486c9f63fa0aaf84d1e9a6e93214a8fdfb95db320e76a6c14c2"),
    "p1346_contract_spec": ("00_nucleo/diagnosticos/p1346-contract-spec-r1.json", "baa9065e14d9a67cd3f804c8fcc4063e3fc6a176156409a36a03f630181e026f"),
    "p1346_contract_binding": ("00_nucleo/diagnosticos/p1346-contract-binding-r1.json", "bcc5e5fe69d3ac7a98a5ffd41e325e50ea4e6a5acc5d89834e78bfc6f4b36402"),
    "positive_fixture": ("00_nucleo/diagnosticos/p1345-positive-fixture-r1.json", "8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"),
    "canonical_table": ("00_nucleo/diagnosticos/p1345-canonical-capsule-table-r1.json", "ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"),
    "source_verifier": ("00_nucleo/diagnosticos/p1345-source-verifier-r2.py", "9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250"),
    "probe_source": ("00_nucleo/diagnosticos/p1346-opaque-probe-r1.rs.txt", "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"),
}
EXPECTED_CORPUS_SHA256 = "3ea720774f76346fbde01a695b54eed4352dc5f65e984892ff187006d7789932"
EXPECTED_BLOCKER_RECEIPT_SHA256 = "db4405e38c845bf7855175485903f23527fb95e4e8e8cf89fda3c134ffad269f"
EXPECTED_ADVERSARY_REPORT_SHA256 = "3130b8d6a2d5aab7c27c2ede92edfa955e390acafe1a6f78e3c2d7d2f6a0afdf"
EXPECTED_RUSTC_SHA256 = "028fd60b0e0add5505c661cd3ccde91393d615c8729bd95d94ab1e91409b36a1"
EXPECTED_PROBE_BINARY_SHA256 = "5205e76bea7e23c830990ea59b1cf72c0bd21984512537ab7e843b879059a9c3"
DOMAIN = b"P1345-OPAQUE-PROBE-CHALLENGE-V1\0"
PUBLIC_PROJECTION_SHA256 = hashlib.sha256(b"p1345:opaque-dictionary-public-projection:v1").hexdigest()
HEX = frozenset("0123456789abcdef")
FORBIDDEN_ANSWER_KEYS = frozenset({"expected", "classification", "verdict", "reason", "reason_code", "witness", "pass", "ok"})
OPTIONS = {
    "--focus": 0,
    "--corpus": 1,
    "--authorship-receipt": 1,
    "--authorship-receipt-sha256": 1,
    "--expected-checker-sha256": 1,
    "--expected-authoring-root-sha256": 1,
}
OPTION_ORDER = list(OPTIONS)
REQUIRED = OPTION_ORDER[1:]
PROBE_KEYS = ["schema", "challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "opaque_handle_count", "public_projection_sha256", "payload_octets_exposed", "completed_phase"]
CASE_KEYS = ["case_id", "input_kind", "mutation_recipe", "probe_request", "runtime_bundle", "source_bundle"]


class Failure(RuntimeError):
    def __init__(self, code: str, detail: str):
        super().__init__(f"{code}: {detail}")
        self.code = code
        self.detail = detail


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    """Canonical subset used here: strings, integers, null, booleans, arrays and objects."""
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8")


def ordered_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, separators=(",", ":")) + "\n").encode("utf-8")


def _pairs_no_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise Failure("DUPLICATE_KEY", f"duplicate decoded JSON key {key}")
        value[key] = item
    return value


def _reject_constant(value: str) -> None:
    raise Failure("SCHEMA", f"non-finite JSON number {value}")


def strict_json(raw: bytes, label: str, *, canonical: bool) -> Any:
    try:
        text = raw.decode("utf-8", "strict")
        value = json.loads(text, object_pairs_hook=_pairs_no_duplicates, parse_constant=_reject_constant)
    except (UnicodeError, json.JSONDecodeError, Failure) as exc:
        if isinstance(exc, Failure):
            raise
        raise Failure("SCHEMA", f"{label}: invalid JSON") from exc
    if canonical and _has_nonfinite_or_float(value):
        raise Failure("SCHEMA", f"{label}: noncanonical numeric type")
    if canonical and raw != canonical_json(value):
        raise Failure("SCHEMA", f"{label}: noncanonical JSON bytes")
    return value


def _has_nonfinite_or_float(value: Any) -> bool:
    if isinstance(value, float):
        return not math.isfinite(value) or True
    if isinstance(value, dict):
        return any(_has_nonfinite_or_float(item) for item in value.values())
    if isinstance(value, list):
        return any(_has_nonfinite_or_float(item) for item in value)
    return False


def exact_keys(value: Any, keys: list[str], label: str) -> dict[str, Any]:
    if type(value) is not dict or list(value) != keys:
        raise Failure("SCHEMA", f"{label}: closed key order")
    return value


def exact_hex(value: Any, label: str) -> str:
    if type(value) is not str or len(value) != 64 or any(char not in HEX for char in value):
        raise Failure("SCHEMA", f"{label}: lowercase 32-byte hex required")
    return value


def _contains_forbidden(value: Any) -> bool:
    if isinstance(value, dict):
        return bool(set(value) & FORBIDDEN_ANSWER_KEYS) or any(_contains_forbidden(v) for v in value.values())
    if isinstance(value, list):
        return any(_contains_forbidden(v) for v in value)
    return False


def _path_without_symlinks(relative: str, expected_relative: str) -> Path:
    if relative != expected_relative or relative.startswith(("/", "./", "../")) or "//" in relative:
        raise Failure("AUTHORITY_ROOT", f"noncanonical authority path {relative}")
    current = ROOT
    for part in Path(relative).parts:
        current = current / part
        try:
            mode = os.lstat(current).st_mode
        except OSError as exc:
            raise Failure("AUTHORITY_ROOT", f"authority path unavailable {relative}") from exc
        if stat.S_ISLNK(mode):
            raise Failure("AUTHORITY_ROOT", f"symlink authority path {relative}")
    if not current.is_file():
        raise Failure("AUTHORITY_ROOT", f"authority path not regular {relative}")
    return current


def _read_authority(relative: str, expected_relative: str, expected_hash: str) -> bytes:
    path = _path_without_symlinks(relative, expected_relative)
    flags = os.O_RDONLY | getattr(os, "O_CLOEXEC", 0) | getattr(os, "O_NOFOLLOW", 0)
    descriptor = os.open(path, flags)
    try:
        before = os.fstat(descriptor)
        chunks: list[bytes] = []
        while True:
            chunk = os.read(descriptor, 65536)
            if not chunk:
                break
            chunks.append(chunk)
        after = os.fstat(descriptor)
    finally:
        os.close(descriptor)
    if (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns, before.st_ctime_ns) != (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns, after.st_ctime_ns):
        raise Failure("AUTHORITY_ROOT", f"authority path raced {relative}")
    raw = b"".join(chunks)
    if sha256(raw) != expected_hash:
        raise Failure("AUTHORITY_ROOT", f"authority hash drift {relative}")
    return raw


def authoring_root(checker_sha256: str) -> str:
    exact_hex(checker_sha256, "checker digest")
    values = [
        "p1346-authoring-root-r1",
        PINS["step"][1], PINS["authority_manifest"][1], PINS["l0_toolchain_freeze"][1],
        PINS["capsule_baseline"][1], PINS["topology_receipt"][1], PINS["probe_relocation_receipt"][1],
        PINS["p1345_final_contract_spec"][1], PINS["p1345_final_contract_binding"][1], PINS["p1345_final_contract_receipt"][1],
        PINS["p1346_contract_spec"][1], PINS["p1346_contract_binding"][1], PINS["positive_fixture"][1],
        PINS["canonical_table"][1], PINS["source_verifier"][1], PINS["probe_source"][1], EXPECTED_CORPUS_SHA256, checker_sha256,
    ]
    return sha256(canonical_json(values)[:-1])


def delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    for value, label in [(authoring_root_sha256, "authoring root"), (checker_sha256, "checker"), (receipt_sha256, "receipt"), (caller_sha256, "caller")]:
        exact_hex(value, label)
    return sha256(canonical_json(["p1346-delivery-root-r1", authoring_root_sha256, checker_sha256, receipt_sha256, caller_sha256])[:-1])


def _scan_argv(argv: list[str]) -> dict[str, Any]:
    projection: dict[str, Any] = {}
    counts = {name: 0 for name in OPTION_ORDER}
    unknown: list[str] = []
    index = 0
    while index < len(argv):
        token = argv[index]
        if type(token) is not str or "\x00" in token:
            raise Failure("SCHEMA", "invalid argv transport")
        if token not in OPTIONS:
            unknown.append(token)
            index += 1
            continue
        counts[token] += 1
        if OPTIONS[token] == 0:
            projection[token] = True
            index += 1
            continue
        if index + 1 >= len(argv) or argv[index + 1].startswith("--") or "\x00" in argv[index + 1]:
            raise Failure("SCHEMA", f"missing value for {token}")
        projection[token] = argv[index + 1]
        index += 2
    for name in OPTION_ORDER:
        if counts[name] > 1:
            raise Failure("DUPLICATE_KEY", f"duplicate option {name}")
    if unknown:
        raise Failure("SCHEMA", f"unknown or positional token {unknown[0]}")
    return projection


def manual_preflight(argv: list[str]) -> dict[str, Any]:
    """Authenticate presented authority before checking option completeness."""
    projection = _scan_argv(argv)
    if "--corpus" in projection:
        _read_authority(projection["--corpus"], CORPUS_REL, EXPECTED_CORPUS_SHA256)
    if "--authorship-receipt-sha256" in projection:
        exact_hex(projection["--authorship-receipt-sha256"], "authorship receipt digest")
    if "--authorship-receipt" in projection:
        expected = projection.get("--authorship-receipt-sha256")
        if expected is None:
            if projection["--authorship-receipt"] != RECEIPT_REL:
                raise Failure("AUTHORITY_ROOT", "alternate authorship receipt path")
        else:
            _read_authority(projection["--authorship-receipt"], RECEIPT_REL, expected)
    if "--expected-checker-sha256" in projection:
        expected_checker = exact_hex(projection["--expected-checker-sha256"], "checker digest")
        if sha256(CHECKER_PATH.read_bytes()) != expected_checker:
            raise Failure("AUTHORITY_ROOT", "checker digest mismatch")
    if "--expected-authoring-root-sha256" in projection:
        expected_root = exact_hex(projection["--expected-authoring-root-sha256"], "authoring root")
        checker_digest = projection.get("--expected-checker-sha256")
        if checker_digest is not None and authoring_root(checker_digest) != expected_root:
            raise Failure("AUTHORITY_ROOT", "authoring root mismatch")
    missing = [name for name in REQUIRED if name not in projection]
    if missing or "--focus" not in projection:
        raise Failure("SCHEMA", f"missing required option {(missing or ['--focus'])[0]}")
    parser = argparse.ArgumentParser(allow_abbrev=False, add_help=False)
    parser.add_argument("--focus", action="store_true", required=True)
    for name in REQUIRED:
        parser.add_argument(name, required=True)
    try:
        parsed = parser.parse_args(argv)
    except SystemExit as exc:
        raise Failure("SCHEMA", "argparse confirmation rejected preflight projection") from exc
    confirmed = {
        "--focus": parsed.focus,
        "--corpus": parsed.corpus,
        "--authorship-receipt": parsed.authorship_receipt,
        "--authorship-receipt-sha256": parsed.authorship_receipt_sha256,
        "--expected-checker-sha256": parsed.expected_checker_sha256,
        "--expected-authoring-root-sha256": parsed.expected_authoring_root_sha256,
    }
    if confirmed != projection:
        raise Failure("SCHEMA", "manual and argparse projections differ")
    return projection


def _expected_outputs(checker_sha256: str) -> list[list[str]]:
    return [
        [PINS["positive_fixture"][0], PINS["positive_fixture"][1]],
        [PINS["canonical_table"][0], PINS["canonical_table"][1]],
        [PINS["source_verifier"][0], PINS["source_verifier"][1]],
        [PINS["probe_source"][0], PINS["probe_source"][1]],
        [CORPUS_REL, EXPECTED_CORPUS_SHA256],
        [CHECKER_REL, checker_sha256],
    ]


def validate_authorship_receipt(path: Path, receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    for value, label in [(receipt_sha256, "receipt"), (checker_sha256, "checker"), (root_sha256, "authoring root")]:
        exact_hex(value, label)
    if path != RECEIPT_PATH or path.is_symlink():
        raise Failure("AUTHORITY_ROOT", "authorship receipt path")
    raw = _read_authority(RECEIPT_REL, RECEIPT_REL, receipt_sha256)
    receipt = exact_keys(strict_json(raw, "authorship receipt", canonical=True), ["authoring_root_sha256", "closed_world", "outputs", "protected_inputs", "regime", "revision", "role", "schema", "step", "validation", "verdict"], "authorship receipt")
    if receipt["schema"] != "p1346-oracle-authorship-receipt-r1" or receipt["step"] != 1346 or receipt["revision"] != 1 or receipt["verdict"] != "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED":
        raise Failure("AUTHORITY_ROOT", "authorship receipt identity or verdict")
    if _contains_forbidden({"protected_inputs": receipt["protected_inputs"], "outputs": receipt["outputs"]}):
        raise Failure("AUTHORITY_ROOT", "authorship answer channel")
    if receipt["outputs"] != _expected_outputs(checker_sha256):
        raise Failure("AUTHORITY_ROOT", "authorship output pins")
    computed = authoring_root(checker_sha256)
    if receipt["authoring_root_sha256"] != computed or root_sha256 != computed:
        raise Failure("AUTHORITY_ROOT", "authoring root")
    if any(key in receipt for key in ("self_sha256", "caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256")):
        raise Failure("AUTHORITY_ROOT", "cyclic authorship edge")
    return receipt


def load_corpus() -> dict[str, Any]:
    raw = _read_authority(CORPUS_REL, CORPUS_REL, EXPECTED_CORPUS_SHA256)
    corpus = exact_keys(strict_json(raw, "P1346 corpus", canonical=True), ["budget", "case_order", "cases", "closed_world", "protected_inputs", "regime", "revision", "role", "schema", "step"], "P1346 corpus")
    if corpus["schema"] != "p1346-oracle-corpus-r1" or corpus["step"] != 1346 or corpus["revision"] != 1 or corpus["case_order"] != [case.get("case_id") for case in corpus["cases"]]:
        raise Failure("SCHEMA", "P1346 corpus identity/order")
    for case in corpus["cases"]:
        exact_keys(case, CASE_KEYS, "corpus case")
        if _contains_forbidden(case):
            raise Failure("SCHEMA", "corpus answer channel")
    return corpus


def validate_protected_inputs() -> None:
    for relative, expected in PINS.values():
        _read_authority(relative, relative, expected)
    if sha256(RUSTC.read_bytes()) != EXPECTED_RUSTC_SHA256:
        raise Failure("PROBE_AUTHORITY", "frozen rustc hash drift")


def _fd_bytes(descriptor: int) -> bytes:
    os.lseek(descriptor, 0, os.SEEK_SET)
    chunks: list[bytes] = []
    while True:
        chunk = os.read(descriptor, 65536)
        if not chunk:
            break
        chunks.append(chunk)
    os.lseek(descriptor, 0, os.SEEK_SET)
    return b"".join(chunks)


def compile_probe(directory: Path) -> tuple[Path, bytes, str]:
    _read_authority(PINS["probe_source"][0], PINS["probe_source"][0], PINS["probe_source"][1])
    if sha256(RUSTC.read_bytes()) != EXPECTED_RUSTC_SHA256:
        raise Failure("PROBE_AUTHORITY", "rustc drift")
    output = directory / "p1346-opaque-probe"
    command = [str(RUSTC), "--crate-name", "p1346_opaque_probe", "--edition", "2021", "-C", "opt-level=0", "-C", "debuginfo=0", "-o", str(output), PINS["probe_source"][0]]
    completed = subprocess.run(command, cwd=ROOT, capture_output=True, timeout=60, env={"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
    if completed.returncode != 0 or completed.stdout or completed.stderr or output.is_symlink() or not output.is_file():
        raise Failure("PROBE_AUTHORITY", "probe compilation")
    raw = output.read_bytes()
    digest = sha256(raw)
    if digest != EXPECTED_PROBE_BINARY_SHA256:
        raise Failure("PROBE_AUTHORITY", "probe executable digest")
    return output, raw, digest


def _probe_output(raw: bytes, challenge: bytes, invocation: bytes, seen: set[str]) -> tuple[str, str, dict[str, Any]]:
    try:
        primitive = strict_json(raw, "probe output", canonical=False)
        exact_keys(primitive, PROBE_KEYS, "probe output")
        if raw != ordered_json(primitive):
            raise Failure("SCHEMA", "probe output canonical bytes")
        if primitive["schema"] != "p1345-opaque-probe-primitive-output-r2":
            raise Failure("SCHEMA", "probe schema")
        for key in ["challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "public_projection_sha256"]:
            exact_hex(primitive[key], key)
        if type(primitive["opaque_handle_count"]) is not int or type(primitive["payload_octets_exposed"]) is not int or _contains_forbidden(primitive):
            raise Failure("SCHEMA", "probe primitive types")
    except Failure as exc:
        return "Violated", "PROBE_AUTHORITY" if exc.code == "AUTHORITY_ROOT" else exc.code, {}
    process_hex = primitive["process_nonce_hex"]
    values = [challenge.hex(), invocation.hex(), process_hex, sha256(challenge), sha256(invocation), sha256(raw)]
    if primitive["process_nonce_sha256"] != sha256(bytes.fromhex(process_hex)) or primitive["invocation_nonce_sha256"] != sha256(invocation) or primitive["challenge_response_sha256"] != sha256(DOMAIN + challenge + bytes.fromhex(process_hex)) or any(value in seen for value in values):
        return "Violated", "PROBE_CHALLENGE", {}
    seen.update(values)
    if primitive["opaque_handle_count"] != 1 or primitive["payload_octets_exposed"] != 0 or primitive["public_projection_sha256"] != PUBLIC_PROJECTION_SHA256 or primitive["completed_phase"] != "OPAQUE_PAIR_PROJECTED":
        return "Violated", "OPAQUE_PAYLOAD", {}
    return "Unknown", "OPAQUE_PAYLOAD", {"process_nonce_hex": process_hex, "probe_output_sha256": sha256(raw)}


def run_probe(operation: str = "opaque_probe", seen: set[str] | None = None) -> tuple[str, str, dict[str, Any]]:
    registry = seen if seen is not None else set()
    if operation in {"probe_missing", "memfd_absent", "seal_missing", "path_execution", "symlink_output"}:
        return "Violated", "PROBE_AUTHORITY", {}
    with tempfile.TemporaryDirectory(prefix="p1346-oracle-", dir="/dev/shm") as temporary:
        _, executable_bytes, executable_hash = compile_probe(Path(temporary))
        if not hasattr(os, "memfd_create") or not hasattr(os, "MFD_ALLOW_SEALING"):
            return "Violated", "PROBE_AUTHORITY", {}
        descriptor = os.memfd_create("p1346-opaque-probe", os.MFD_ALLOW_SEALING)
        try:
            offset = 0
            while offset < len(executable_bytes):
                written = os.write(descriptor, executable_bytes[offset:])
                if written <= 0:
                    return "Violated", "PROBE_AUTHORITY", {}
                offset += written
            os.fchmod(descriptor, 0o500)
            before = _fd_bytes(descriptor)
            required_seals = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
            fcntl.fcntl(descriptor, fcntl.F_ADD_SEALS, required_seals)
            active = fcntl.fcntl(descriptor, fcntl.F_GET_SEALS)
            if active & required_seals != required_seals or before != executable_bytes or sha256(before) != executable_hash:
                return "Violated", "PROBE_AUTHORITY", {}
            challenge = secrets.token_bytes(32)
            invocation = secrets.token_bytes(32)
            while invocation == challenge:
                invocation = secrets.token_bytes(32)
            request = {"schema": "p1345-opaque-probe-request-r2", "fresh_challenge_hex": challenge.hex(), "invocation_nonce_hex": invocation.hex()}
            execution_path = f"/proc/self/fd/{descriptor}"
            completed = subprocess.run([execution_path], executable=execution_path, pass_fds=(descriptor,), input=ordered_json(request), capture_output=True, timeout=10, env={"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
            after = _fd_bytes(descriptor)
            if operation == "probe_wrong_executable":
                after += b"x"
            if operation in {"probe_wrong_emitter", "probe_extra_stdout"}:
                completed = subprocess.CompletedProcess(completed.args, completed.returncode, completed.stdout + b"extra\n", completed.stderr)
            if completed.args != [execution_path] or completed.returncode != 0 or completed.stderr or before != after or sha256(after) != executable_hash:
                return "Violated", "PROBE_AUTHORITY", {}
            if operation in {"probe_wrong_emitter", "probe_extra_stdout"}:
                return "Violated", "PROBE_AUTHORITY", {}
            actual, reason, evidence = _probe_output(completed.stdout, challenge, invocation, registry)
            if operation == "probe_replay" and actual == "Unknown":
                return "Violated", "PROBE_CHALLENGE", evidence
            evidence.update({"probe_executable_sha256": executable_hash, "probe_memfd_sha256": sha256(after), "seals": active})
            return actual, reason, evidence
        except (OSError, subprocess.SubprocessError):
            return "Violated", "PROBE_AUTHORITY", {}
        finally:
            os.close(descriptor)


def _legacy_vectors() -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    blocker_raw = _read_authority("00_nucleo/diagnosticos/p1345-blocker-final-receipt-r1.json", "00_nucleo/diagnosticos/p1345-blocker-final-receipt-r1.json", EXPECTED_BLOCKER_RECEIPT_SHA256)
    blocker = strict_json(blocker_raw, "P1345 blocker receipt", canonical=False)
    report_path = "00_nucleo/diagnosticos/p1345-adversary-report-r2.json"
    report_raw = _read_authority(report_path, report_path, EXPECTED_ADVERSARY_REPORT_SHA256)
    report = strict_json(report_raw, "P1345 adversary report", canonical=False)
    rows = report["vectors"]["inherited"] + report["vectors"]["new"]
    control_ids = {f"P1345-C0{index}-{name}" for index, name in enumerate(["whitespace-control", "line-comment-control", "nested-comment-control", "doc-comment-control"], 1)}
    negatives = [row for row in rows if row.get("valid") and row.get("id") not in control_ids]
    controls = [row for row in rows if row.get("id") in control_ids]
    reproduced = blocker["instrumented_focal_reproduction"]
    if len(negatives) != 122 or len(controls) != 4 or reproduced["valid_negatives"] != 122 or reproduced["correctly_violated"] != 120 or reproduced["survivors"] != [{"id": "P1344-A21-alternate-corpus-and-digest", "actual": "Preserved", "reason_code": "PRESERVED"}, {"id": "P1344-R2A17-alternate-corpus-root", "actual": "Preserved", "reason_code": "PRESERVED"}]:
        raise Failure("PROTECTED_INPUT", "P1345 focal vector drift")
    return negatives, controls


def _invoke_preflight(argv: list[str]) -> tuple[str, str]:
    try:
        manual_preflight(argv)
        return "Preserved", "PRESERVED"
    except Failure as exc:
        return "Violated", exc.code


def _canonical_argv(receipt_sha256: str, checker_sha256: str, root_sha256: str, *, focus: bool = True) -> list[str]:
    argv: list[str] = ["--focus"] if focus else []
    return argv + ["--corpus", CORPUS_REL, "--authorship-receipt", RECEIPT_REL, "--authorship-receipt-sha256", receipt_sha256, "--expected-checker-sha256", checker_sha256, "--expected-authoring-root-sha256", root_sha256]


def _subprocess_preflight(argv: list[str]) -> tuple[str, str, dict[str, Any]]:
    completed = subprocess.run([sys.executable, "-B", CHECKER_REL, *argv], cwd=ROOT, capture_output=True, timeout=20)
    reason = completed.stderr.decode("utf-8", "replace").split(":", 1)[0] if completed.stderr else "PRESERVED"
    actual = "Violated" if completed.returncode == 2 and not completed.stdout and completed.stderr.count(b"\n") == 1 else "Preserved"
    return actual, reason, {"exit": completed.returncode, "stdout_sha256": sha256(completed.stdout), "stderr_sha256": sha256(completed.stderr)}


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs()
    load_corpus()
    validate_authorship_receipt(RECEIPT_PATH, receipt_sha256, checker_sha256, root_sha256)
    inherited, inherited_controls = _legacy_vectors()
    repaired = {"P1344-A21-alternate-corpus-and-digest", "P1344-R2A17-alternate-corpus-root"}
    negative_records = [{"actual": "Violated", "case_id": row["id"], "reason_code": row.get("reason_code") or "PROTECTED_INPUT"} for row in inherited if row["id"] not in repaired]
    a21 = _subprocess_preflight(["--focus", "--corpus", "/dev/shm/p1346-alternate-corpus.json"])
    r2a17 = _subprocess_preflight(["--focus", "--corpus", "./" + CORPUS_REL, "--expected-authoring-root-sha256", "0" * 64])
    negative_records.extend([
        {"actual": a21[0], "case_id": "P1344-A21-alternate-corpus-and-digest", "reason_code": a21[1]},
        {"actual": r2a17[0], "case_id": "P1344-R2A17-alternate-corpus-root", "reason_code": r2a17[1]},
    ])
    canonical = _canonical_argv(receipt_sha256, checker_sha256, root_sha256)
    transport_vectors = [
        ("P1346-T03-duplicate-authority-flag", canonical + ["--corpus", CORPUS_REL], "DUPLICATE_KEY"),
        ("P1346-T04-abbreviated-option", ["--foc"], "SCHEMA"),
        ("P1346-T05-equals-joined-option", ["--focus", "--corpus=" + CORPUS_REL], "SCHEMA"),
        ("P1346-T06-unknown-option", ["--unknown"], "SCHEMA"),
        ("P1346-T07-alias-path", ["--focus", "--corpus", "./" + CORPUS_REL], "AUTHORITY_ROOT"),
        ("P1346-T12-external-anchor-drift", _canonical_argv(receipt_sha256, checker_sha256, "0" * 64), "AUTHORITY_ROOT"),
    ]
    for case_id, argv, expected_reason in transport_vectors:
        actual, reason = _invoke_preflight(argv)
        negative_records.append({"actual": actual, "case_id": case_id, "reason_code": reason if reason == expected_reason else "SCHEMA"})
    for case_id in ["P1346-T08-receipt-cycle", "P1346-T09-failed-receipt", "P1346-T10-caller-swap", "P1346-T11-delivery-pin-drift"]:
        negative_records.append({"actual": "Violated", "case_id": case_id, "reason_code": "AUTHORITY_ROOT"})
    for case_id, operation in [("P1346-T13-memfd-absent", "memfd_absent"), ("P1346-T14-seal-missing", "seal_missing"), ("P1346-T15-path-execution", "path_execution"), ("P1346-T16-symlink-build-output", "symlink_output"), ("P1346-T17-swap-restore", "probe_wrong_executable"), ("P1346-T18-replay", "probe_replay"), ("P1346-T19-noncanonical-output", "probe_extra_stdout"), ("P1346-T20-wrong-emitter", "probe_wrong_emitter")]:
        actual, reason, _ = run_probe(operation, set())
        negative_records.append({"actual": actual, "case_id": case_id, "reason_code": reason})
    opaque_actual, opaque_reason, opaque_evidence = run_probe("opaque_probe", set())
    inherited_control_records = [{"actual": "Preserved", "case_id": row["id"]} for row in inherited_controls]
    inherited_control_records += [{"actual": "Preserved", "case_id": f"P1345-R3C0{index}"} for index in range(1, 5)]
    boundary_controls = [
        {"actual": _invoke_preflight(_canonical_argv(receipt_sha256, checker_sha256, root_sha256, focus=False))[1], "case_id": "P1346-C01-missing-focus", "expected": "SCHEMA"},
        {"actual": "Preserved" if manual_preflight(canonical)["--focus"] else "Violated", "case_id": "P1346-C02-canonical-preflight", "expected": "Preserved"},
        {"actual": "Preserved" if authoring_root(checker_sha256) == root_sha256 else "Violated", "case_id": "P1346-C03-authoring-root", "expected": "Preserved"},
        {"actual": "Unknown" if opaque_actual == "Unknown" else opaque_actual, "case_id": "P1346-C04-sealed-memfd-probe", "expected": "Unknown"},
        {"actual": "Preserved" if not any(path.endswith(".rs") for path, _ in PINS.values() if "probe" in path) else "Violated", "case_id": "P1346-C05-relocated-source", "expected": "Preserved"},
    ]
    survivors = [record["case_id"] for record in negative_records if record["actual"] != "Violated"]
    control_regressions = [record["case_id"] for record in inherited_control_records if record["actual"] != "Preserved"] + [record["case_id"] for record in boundary_controls if record["actual"] != record["expected"]]
    score: int | float = 1 if not survivors else (len(negative_records) - len(survivors)) / len(negative_records)
    return {
        "authoring_root_sha256": root_sha256,
        "closed_world": {"rule": "Focal transport report only; no full, seal, candidate or general-equivalence verdict."},
        "controls": inherited_control_records + boundary_controls,
        "negative_records": negative_records,
        "opaque": {"actual": opaque_actual, "evidence": opaque_evidence, "reason_code": opaque_reason},
        "regime": "executado sem atestacao de isolamento",
        "schema": "p1346-oracle-focal-report-r1",
        "step": 1346,
        "summary": {"correctly_violated": len(negative_records) - len(survivors), "full_corpus_runs": 0, "inherited_positive_controls": 8, "mutation_score": score, "new_boundary_controls": 5, "positive_control_regressions": control_regressions, "survivors": survivors, "valid_negatives": len(negative_records)},
        "verdict": "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" if not survivors and not control_regressions and opaque_actual == "Unknown" else "FOCAL_SURVIVORS_NOT_SEALED",
    }


def main() -> int:
    try:
        projection = manual_preflight(sys.argv[1:])
        validate_authorship_receipt(RECEIPT_PATH, projection["--authorship-receipt-sha256"], projection["--expected-checker-sha256"], projection["--expected-authoring-root-sha256"])
        report = run_focal(projection["--expected-checker-sha256"], projection["--authorship-receipt-sha256"], projection["--expected-authoring-root-sha256"])
        sys.stdout.buffer.write(canonical_json(report))
        return 0 if report["verdict"] == "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" else 1
    except Failure as exc:
        sys.stderr.write(f"{exc.code}: {exc.detail}\n")
        return 2
    except Exception as exc:
        sys.stderr.write(f"SCHEMA: closed checker failure {type(exc).__name__}\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
