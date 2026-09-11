#!/usr/bin/env python3
"""P1345 Oracle R2 focal correction for the three R1 adversarial classes.

The canonical fixture/table/probe remain byte-identical R1 artifacts.  This
checker closes only invocation schema/runtime, opaque primitive projection and
executable authority delivery.  Oracle authors execute ``--focus`` only.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import secrets
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
SOURCE_PATH = DIAG / "p1345-source-verifier-r2.py"
CHECKER_R1_PATH = SOURCE_PATH.resolve().parent / "p1345-oracle-checker-r1.py"
FIXTURE_PATH = DIAG / "p1345-positive-fixture-r1.json"
TABLE_PATH = DIAG / "p1345-canonical-capsule-table-r1.json"
CORPUS_R1_PATH = DIAG / "p1345-oracle-corpus-r1.json"
CORPUS_PATH = DIAG / "p1345-oracle-corpus-r2.json"
PROBE_PATH = DIAG / "p1345-opaque-probe-r1.rs"
ADVERSARY_PATH = DIAG / "p1345-adversary-runner-r1.py"
AUTHORSHIP_RECEIPT_PATH = DIAG / "p1345-oracle-authorship-receipt-r2.json"

EXPECTED_SOURCE_SHA256 = "9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250"
EXPECTED_CHECKER_R1_SHA256 = "8edf345e27e4b12bf4e2eb39cefa2a854f6ea73c2749e6a76f3d93c1119abc5e"
EXPECTED_FIXTURE_SHA256 = "8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"
EXPECTED_TABLE_SHA256 = "ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"
EXPECTED_CORPUS_R1_SHA256 = "daf610f19634bea6245f08e530fcb75a53532500c63c2f8a98ef583096573a1f"
EXPECTED_CORPUS_SHA256 = "3003e7577cbf99bdc50d8f14b44897e4556a8fd18f439ec07a1de96e72c94394"
EXPECTED_PROBE_SHA256 = "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"
EXPECTED_ADVERSARY_SHA256 = "d758095443b701459e20a3193423771059348c4d76076bc251174564fa4a8a61"
RUSTC_SHA256 = "028fd60b0e0add5505c661cd3ccde91393d615c8729bd95d94ab1e91409b36a1"
DOMAIN = bytes.fromhex("50313334352d4f50415155452d50524f42452d4348414c4c454e47452d563100")
PUBLIC_PROJECTION_SHA256 = hashlib.sha256(b"p1345:opaque-dictionary-public-projection:v1").hexdigest()
PROBE_KEYS = ["schema", "challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "opaque_handle_count", "public_projection_sha256", "payload_octets_exposed", "completed_phase"]
CASE_KEYS = ["case_id", "input_kind", "source_bundle", "runtime_bundle", "mutation_recipe", "probe_request"]
INPUT_KINDS = {"synthetic_source", "p1344_historical_attack", "capsule_token_mutation", "token_edit_class", "source_or_authority_attack", "probe_attack", "opaque_probe"}
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


def import_pinned(name: str, path: Path, expected: str) -> Any:
    if sha256(path.read_bytes()) != expected:
        raise Failure("AUTHORITY_ROOT", f"{path.name} hash drift")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Failure("AUTHORITY_ROOT", f"cannot load {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


SOURCE = import_pinned("p1345_source_verifier_r2_pinned", SOURCE_PATH, EXPECTED_SOURCE_SHA256)
R1 = import_pinned("p1345_oracle_checker_r1_pinned", CHECKER_R1_PATH, EXPECTED_CHECKER_R1_SHA256)
R1.SOURCE = SOURCE


def load_closed() -> tuple[dict[str, Any], dict[str, Any], dict[str, Any], dict[str, Any]]:
    for path, expected in [(FIXTURE_PATH, EXPECTED_FIXTURE_SHA256), (TABLE_PATH, EXPECTED_TABLE_SHA256), (CORPUS_R1_PATH, EXPECTED_CORPUS_R1_SHA256), (CORPUS_PATH, EXPECTED_CORPUS_SHA256), (PROBE_PATH, EXPECTED_PROBE_SHA256), (ADVERSARY_PATH, EXPECTED_ADVERSARY_SHA256)]:
        if sha256(path.read_bytes()) != expected:
            raise Failure("AUTHORITY_ROOT", f"{path.name} hash drift")
    corpus = SOURCE.strict_json(CORPUS_PATH.read_bytes(), "P1345 R2 corpus")
    SOURCE.exact_keys(corpus, ["schema", "step", "revision", "role", "regime", "protected_inputs", "composition", "focal_revision_scope", "runtime_boundary", "budget", "closed_world"], "R2 corpus")
    if corpus["schema"] != "p1345-oracle-corpus-r2" or type(corpus["step"]) is not int or corpus["step"] != 1345 or type(corpus["revision"]) is not int or corpus["revision"] != 2:
        raise Failure("SCHEMA", "R2 corpus identity/type mismatch")
    if corpus["composition"]["valid_negatives"] != 80 or corpus["composition"]["positive_controls"] != 4 or corpus["budget"]["full_corpus_runs_by_oracle_author"] != 0:
        raise Failure("SCHEMA", "R2 focal cardinality/budget mismatch")
    r1_corpus = SOURCE.strict_json(CORPUS_R1_PATH.read_bytes(), "P1345 R1 corpus")
    fixture, table = SOURCE.load_fixture_table()
    if len(table["records"]) != 38 or len(fixture["files"]) != 12:
        raise Failure("SCHEMA", "canonical table/fixture cardinality")
    return corpus, r1_corpus, fixture, table


def _require_hex(value: Any, label: str) -> str:
    if type(value) is not str or len(value) != 64 or any(char not in HEX for char in value):
        raise Failure("SCHEMA", f"{label} must be exact lowercase 32-byte hex")
    return value


def _contains_forbidden(value: Any) -> bool:
    if isinstance(value, dict):
        return bool(set(value) & FORBIDDEN) or any(_contains_forbidden(v) for v in value.values())
    if isinstance(value, list):
        return any(_contains_forbidden(v) for v in value)
    return False


def validate_case(case: dict[str, Any]) -> None:
    if type(case) is not dict or list(case) != CASE_KEYS:
        raise Failure("SCHEMA", "case exact ordered keys mismatch")
    if _contains_forbidden(case):
        raise Failure("RUNTIME_AUTHORITY", "case contains a classification/witness answer channel")
    if type(case["case_id"]) is not str or not case["case_id"]:
        raise Failure("SCHEMA", "case_id type/value")
    if type(case["input_kind"]) is not str or case["input_kind"] not in INPUT_KINDS:
        raise Failure("RUNTIME_AUTHORITY", "input_kind is outside the closed synthetic/oracle enum")
    source = case["source_bundle"]
    if type(source) is not dict or list(source) != ["fixture_sha256", "table_sha256"] or source["fixture_sha256"] != EXPECTED_FIXTURE_SHA256 or source["table_sha256"] != EXPECTED_TABLE_SHA256:
        raise Failure("AUTHORITY_ROOT", "case source bundle is not the pinned fixture/table pair")
    if case["runtime_bundle"] is not None:
        raise Failure("RUNTIME_AUTHORITY", "oracle corpus accepts no candidate/runtime bundle")
    recipe = case["mutation_recipe"]
    if type(recipe) is not dict or "operation" not in recipe or type(recipe["operation"]) is not str or not set(recipe).issubset({"operation", "capsule_id", "token_index", "replacement"}):
        raise Failure("SCHEMA", "mutation recipe schema")
    if "token_index" in recipe and type(recipe["token_index"]) is not int:
        raise Failure("SCHEMA", "token_index must be an integer, not bool/float")
    probe = case["probe_request"]
    if recipe["operation"] == "opaque_probe":
        if type(probe) is not dict or list(probe) != ["probe_case_id", "fresh_challenge_hex", "invocation_nonce_hex"] or probe["probe_case_id"] != case["case_id"] or probe["fresh_challenge_hex"] != "verifier_generated_at_execution" or probe["invocation_nonce_hex"] != "verifier_generated_at_execution":
            raise Failure("PROBE_AUTHORITY", "opaque probe request is not verifier-owned")
    elif probe is not None:
        raise Failure("SCHEMA", "non-opaque case has probe request")


def authoring_root(receipt_sha256: str, expected_checker_sha256: str) -> str:
    order = ["step", "manifest", "freeze", "inventory", "baseline", "topology", "p1344_contract", "p1344_binding", "p1344_adversary_report", "p1344_adversary_receipt", "contract_r1", "binding_r1", "receipt_r1", "blocker", "blocker_receipt", "contract_r2", "binding_r2"]
    values = ["p1345-authoring-root-r2"] + [SOURCE.PINS[name][1] for name in order] + [EXPECTED_FIXTURE_SHA256, EXPECTED_TABLE_SHA256, EXPECTED_CORPUS_SHA256, EXPECTED_SOURCE_SHA256, EXPECTED_PROBE_SHA256, expected_checker_sha256, receipt_sha256]
    return sha256(canonical_json(values))


def validate_authorship_receipt(path: Path, expected_receipt_sha256: str, expected_checker_sha256: str, expected_root_sha256: str) -> str:
    _require_hex(expected_receipt_sha256, "expected receipt hash")
    _require_hex(expected_checker_sha256, "expected checker hash")
    _require_hex(expected_root_sha256, "expected authoring root")
    if sha256(Path(__file__).read_bytes()) != expected_checker_sha256:
        raise Failure("AUTHORITY_ROOT", "out-of-band checker hash mismatch")
    path = Path(path)
    if path.is_symlink() or path.absolute() != AUTHORSHIP_RECEIPT_PATH.absolute():
        raise Failure("AUTHORITY_ROOT", "receipt path is not the canonical real path")
    raw = path.read_bytes()
    if sha256(raw) != expected_receipt_sha256:
        raise Failure("AUTHORITY_ROOT", "out-of-band authorship receipt hash mismatch")
    receipt = SOURCE.strict_json(raw, "R2 authorship receipt")
    SOURCE.exact_keys(receipt, ["schema", "step", "revision", "role", "regime", "artifact_hashes", "execution", "verdict", "closed_world"], "R2 authorship receipt")
    if receipt["schema"] != "p1345-oracle-authorship-receipt-r2" or receipt["verdict"] != "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED":
        raise Failure("AUTHORITY_ROOT", "authorship receipt identity/verdict")
    artifacts = receipt["artifact_hashes"]
    required = {"checker_r2": expected_checker_sha256, "source_verifier_r2": EXPECTED_SOURCE_SHA256, "corpus_r2": EXPECTED_CORPUS_SHA256, "fixture_r1": EXPECTED_FIXTURE_SHA256, "table_r1": EXPECTED_TABLE_SHA256, "probe_r1": EXPECTED_PROBE_SHA256}
    if type(artifacts) is not dict or any(artifacts.get(key) != value for key, value in required.items()):
        raise Failure("AUTHORITY_ROOT", "authorship receipt artifact pins")
    computed = authoring_root(expected_receipt_sha256, expected_checker_sha256)
    if computed != expected_root_sha256:
        raise Failure("AUTHORITY_ROOT", "out-of-band authoring root mismatch")
    return computed


def compile_probe(root: Path) -> tuple[Path, str]:
    rustc = Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")
    if not rustc.is_file() or sha256(rustc.read_bytes()) != RUSTC_SHA256:
        raise Failure("PROBE_AUTHORITY", "frozen rustc mismatch")
    binary = root / "p1345-opaque-probe-r1"
    completed = subprocess.run([str(rustc), "--edition", "2021", "-C", "opt-level=0", "-C", "debuginfo=0", "-o", str(binary), str(PROBE_PATH)], cwd=ROOT, capture_output=True, timeout=60)
    if completed.returncode or completed.stderr:
        raise Failure("PROBE_AUTHORITY", "probe build failed or emitted stderr")
    return binary, sha256(binary.read_bytes())


def _copy_completed(value: subprocess.CompletedProcess[bytes], stdout: bytes) -> subprocess.CompletedProcess[bytes]:
    return subprocess.CompletedProcess(value.args, value.returncode, stdout, value.stderr)


def run_probe(operation: str, seen: set[str]) -> tuple[str, str, dict[str, str]]:
    if operation == "probe_missing":
        return "Violated", "PROBE_AUTHORITY", {}
    with tempfile.TemporaryDirectory(prefix="p1345-probe-r2-", dir="/dev/shm") as temporary:
        binary, compiled_hash = compile_probe(Path(temporary))
        challenge = secrets.token_bytes(32)
        invocation = secrets.token_bytes(32)
        while invocation == challenge:
            invocation = secrets.token_bytes(32)
        request = {"schema": "p1345-opaque-probe-request-r2", "fresh_challenge_hex": challenge.hex(), "invocation_nonce_hex": invocation.hex()}
        before = sha256(binary.read_bytes())
        completed = subprocess.run([str(binary)], input=(json.dumps(request, separators=(",", ":")) + "\n").encode(), capture_output=True, timeout=10, env={"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
        after = sha256(binary.read_bytes())
        if operation == "probe_wrong_executable":
            after = "0" * 64
        if operation in {"probe_wrong_emitter", "probe_extra_stdout"}:
            completed = _copy_completed(completed, completed.stdout + b"extra\n")
        if completed.args != [str(binary)] or completed.returncode or completed.stderr or not completed.stdout.endswith(b"\n") or completed.stdout.count(b"\n") != 1:
            return "Violated", "PROBE_AUTHORITY", {}
        try:
            primitive = SOURCE.strict_json(completed.stdout, "probe primitive output")
            if type(primitive) is not dict or list(primitive) != PROBE_KEYS or _contains_forbidden(primitive):
                raise Failure("SCHEMA", "probe primitive exact keys")
            if primitive["schema"] != "p1345-opaque-probe-primitive-output-r2":
                raise Failure("SCHEMA", "probe primitive schema enum")
            for key in ["challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "public_projection_sha256"]:
                _require_hex(primitive[key], key)
            if type(primitive["opaque_handle_count"]) is not int or type(primitive["payload_octets_exposed"]) is not int:
                raise Failure("SCHEMA", "probe counts must be JSON integers, not bool/float")
        except (SOURCE.VerificationFailure, Failure) as exc:
            return "Violated", getattr(exc, "code", "SCHEMA"), {}
        process_hex = primitive["process_nonce_hex"]
        if operation == "probe_bad_nonce_hex":
            process_hex = "G" + process_hex[1:]
        if len(process_hex) != 64 or any(char not in HEX for char in process_hex):
            return "Violated", "PROBE_CHALLENGE", {}
        process = bytes.fromhex(process_hex)
        expected_process = sha256(process)
        expected_invocation = sha256(invocation)
        expected_response = sha256(DOMAIN + challenge + process)
        nonce_digest = primitive["process_nonce_sha256"]
        response = primitive["challenge_response_sha256"]
        if operation == "probe_bad_nonce_digest":
            nonce_digest = "0" * 64
        if operation == "probe_bad_response":
            response = "0" * 64
        if operation == "probe_replay":
            seen.add(process_hex)
        values = [challenge.hex(), invocation.hex(), process_hex, expected_process, expected_invocation, expected_response, sha256(completed.stdout)]
        if before != compiled_hash or after != compiled_hash:
            return "Violated", "PROBE_AUTHORITY", {}
        if nonce_digest != expected_process or primitive["invocation_nonce_sha256"] != expected_invocation or response != expected_response or len(set(values)) != len(values) or any(value in seen for value in values):
            return "Violated", "PROBE_CHALLENGE", {}
        seen.update(values)
        if primitive["opaque_handle_count"] != 1 or primitive["payload_octets_exposed"] != 0 or primitive["completed_phase"] != "OPAQUE_PAIR_PROJECTED" or primitive["public_projection_sha256"] != PUBLIC_PROJECTION_SHA256:
            return "Violated", "OPAQUE_PAYLOAD", {}
        return "Unknown", "OPAQUE_PAYLOAD", {"binary": compiled_hash, "output": sha256(completed.stdout), "process_nonce": process_hex, "registry": sha256(canonical_json(sorted(seen)))}


def judge(case: dict[str, Any], fixture: dict[str, Any], seen: set[str]) -> dict[str, Any]:
    case_id = case.get("case_id", "invalid-case") if isinstance(case, dict) else "invalid-case"
    try:
        validate_case(case)
        operation = case["mutation_recipe"]["operation"]
        if operation.startswith("probe_") or operation == "opaque_probe":
            actual, reason, evidence = run_probe(operation, seen)
        else:
            actual, reason, source = R1.source_case(case, fixture)
            evidence = {"source_result_sha256": sha256(canonical_json(source)) if source else None}
        return {"case_id": case_id, "expected_derived": R1.expected_for(operation), "actual": actual, "reason_code": reason, "evidence": evidence}
    except (Failure, SOURCE.VerificationFailure) as exc:
        return {"case_id": case_id, "expected_derived": "Violated", "actual": "Violated", "reason_code": exc.code, "evidence": {"boundary_rejection": sha256(str(exc).encode())}}


def run_adversarial_focal(corpus: dict[str, Any]) -> list[dict[str, Any]]:
    adversary = import_pinned("p1345_adversary_r1_replayed_by_oracle_r2", ADVERSARY_PATH, EXPECTED_ADVERSARY_SHA256)
    adversary.SOURCE = SOURCE
    adversary.CHECKER = sys.modules[__name__]
    adversary.CHECKER_PATH = Path(__file__).resolve()
    adversary.SOURCE_PATH = SOURCE_PATH
    adversary.AUTHORSHIP_RECEIPT_PATH = AUTHORSHIP_RECEIPT_PATH
    records: list[dict[str, Any]] = []
    for attack_id, capsule, transform in adversary.R1_SOURCE + adversary.R2_SOURCE:
        records.append(adversary.source_record(attack_id, capsule, transform, "Exact P1344 source attack intention replayed under P1345.", "p1344-replay"))
    records.extend(adversary.replay_meta())
    records.extend(adversary.new_source_records())
    records.extend(adversary.path_marker_records())
    records.extend(adversary.probe_records())
    records.extend(adversary.authority_records())
    historical = [record for record in records if record["origin"] == "p1344-replay"]
    new = [record for record in records if record["origin"] == "p1345-new"]
    composition = corpus["composition"]
    if len(historical) != 40 or len(new) != 44 or sha256(canonical_json([x["id"] for x in historical])) != composition["historical_record_ids_sha256"] or sha256(canonical_json([x["id"] for x in new])) != composition["new_record_ids_sha256"]:
        raise Failure("AUTHORITY_ROOT", "regenerated adversarial vectors do not match pinned corpus composition")
    return records


def focal_report(records: list[dict[str, Any]], root_sha256: str, checker_sha256: str) -> dict[str, Any]:
    negatives = [record for record in records if record["expected"] == "Violated" and record["valid"]]
    survivors = [record for record in negatives if record["actual"] != "Violated"]
    controls = [record for record in records if record["expected"] == "Preserved"]
    regressions = [record for record in controls if not record["valid"] or record["actual"] != "Preserved"]
    return {"schema": "p1345-oracle-focal-report-r2", "step": 1345, "revision": 2, "role": "independent_oracle_author", "regime": "executado sem atestacao de isolamento", "phase": "first-focal-oracle-revision-r2-no-full", "records": [{"id": r["id"], "origin": r["origin"], "expected": r["expected"], "actual": r["actual"], "valid": r["valid"], "reason_code": r["reason_code"], "survived": r["survived"]} for r in records], "summary": {"records": len(records), "valid_negatives": len(negatives), "correctly_violated": len(negatives) - len(survivors), "survivors": [r["id"] for r in survivors], "mutation_score": (len(negatives) - len(survivors)) / len(negatives), "positive_controls": len(controls), "positive_control_regressions": [r["id"] for r in regressions], "full_corpus_runs": 0}, "authority": {"checker_sha256": checker_sha256, "authoring_root_sha256": root_sha256, "root_delivery": "expected checker and root supplied out of band"}, "verdict": "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" if not survivors and not regressions else "FOCAL_REVISION_SURVIVORS_NOT_SEALED"}


def main() -> int:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--focus", action="store_true")
    mode.add_argument("--full", action="store_true")
    parser.add_argument("--corpus", type=Path, required=True)
    parser.add_argument("--authorship-receipt", type=Path)
    parser.add_argument("--authorship-receipt-sha256")
    parser.add_argument("--expected-checker-sha256")
    parser.add_argument("--expected-authoring-root-sha256")
    args = parser.parse_args()
    if args.corpus.is_symlink() or args.corpus.absolute() != CORPUS_PATH.absolute() or sha256(args.corpus.read_bytes()) != EXPECTED_CORPUS_SHA256:
        raise Failure("AUTHORITY_ROOT", "alternate corpus rejected")
    supplied = [args.authorship_receipt, args.authorship_receipt_sha256, args.expected_checker_sha256, args.expected_authoring_root_sha256]
    if any(value is None for value in supplied):
        raise Failure("AUTHORITY_ROOT", "receipt/checker/root trust values are required out of band")
    root_digest = validate_authorship_receipt(args.authorship_receipt, args.authorship_receipt_sha256, args.expected_checker_sha256, args.expected_authoring_root_sha256)
    corpus, r1_corpus, fixture, _ = load_closed()
    if args.focus:
        output = focal_report(run_adversarial_focal(corpus), root_digest, args.expected_checker_sha256)
    else:
        seen: set[str] = set()
        records = [judge(case, fixture, seen) for case in r1_corpus["cases"]]
        output = {"schema": "p1345-oracle-full-report-r2", "phase": "full-external-authority", "records": records, "authority": {"authoring_root_sha256": root_digest}, "verdict": "FULL_PASS_NOT_SELF_SEALED" if all(r["actual"] == r["expected_derived"] for r in records) else "SURVIVOR_BLOCKS_PRESEAL"}
    print(json.dumps(output, ensure_ascii=False, indent=2))
    return 0 if output["verdict"] in {"FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED", "FULL_PASS_NOT_SELF_SEALED"} else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        code = exc.code if isinstance(exc, Failure) else "SCHEMA"
        print(json.dumps({"schema": "p1345-oracle-fatal-r2", "classification": "Violated", "reason_code": code, "detail": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
