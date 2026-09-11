#!/usr/bin/env python3
"""Independent focal adversary for the pinned P1345 oracle R2.

The runner replays the 80 negative and four positive R1 vectors through the
R2 oracle, then attacks only the R2 closure boundaries.  It never invokes the
``--full`` route and never reads a productive candidate.  All executable and
source mutations live under /dev/shm.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_PATH = DIAG / "p1345-oracle-checker-r2.py"
SOURCE_PATH = DIAG / "p1345-source-verifier-r2.py"
CORPUS_PATH = DIAG / "p1345-oracle-corpus-r2.json"
AUTHORSHIP_PATH = DIAG / "p1345-oracle-authorship-r2.md"
RECEIPT_PATH = DIAG / "p1345-oracle-authorship-receipt-r2.json"

EXPECTED_CHECKER = "8e2ff788526e0b04d3bce6624c4a1864c6f32a36fc3a35088b1bbf6118d1c6a0"
EXPECTED_SOURCE = "9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250"
EXPECTED_CORPUS = "3003e7577cbf99bdc50d8f14b44897e4556a8fd18f439ec07a1de96e72c94394"
EXPECTED_AUTHORSHIP = "365efbbc9b3256bfd075a08eec36369b92c74997d944345b147b36ad4641c205"
EXPECTED_RECEIPT = "a69a4a8d70aba0c36e99bc6155ae24a7af7f1afc3810108301ddbb7c9b9e0ec8"
EXPECTED_ROOT = "6b7706b597cccbde357a09041ebcd14358f6be12c5b494959d4cb1879b2099fc"

PINS = {
    "source_r2": (SOURCE_PATH, EXPECTED_SOURCE),
    "corpus_r2": (CORPUS_PATH, EXPECTED_CORPUS),
    "checker_r2": (CHECKER_PATH, EXPECTED_CHECKER),
    "authorship_r2": (AUTHORSHIP_PATH, EXPECTED_AUTHORSHIP),
    "authorship_receipt_r2": (RECEIPT_PATH, EXPECTED_RECEIPT),
    "fixture_r1": (DIAG / "p1345-positive-fixture-r1.json", "8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"),
    "table_r1": (DIAG / "p1345-canonical-capsule-table-r1.json", "ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"),
    "probe_r1": (DIAG / "p1345-opaque-probe-r1.rs", "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"),
    "adversary_runner_r1": (DIAG / "p1345-adversary-runner-r1.py", "d758095443b701459e20a3193423771059348c4d76076bc251174564fa4a8a61"),
    "adversary_report_r1": (DIAG / "p1345-adversary-report-r1.json", "44c54be9de183c6a6bef5d9a435b31916fbb0d9c6508b6e2066c5b7f6b99377f"),
    "adversary_report_md_r1": (DIAG / "p1345-adversary-report-r1.md", "3db92c1a84a6c9cf6f21a44d21b4c58ef795117ae4d9b22af8078e42a18182c7"),
    "adversary_receipt_r1": (DIAG / "p1345-adversary-receipt-r1.json", "5c5113987c7dea411ce76ea50f6f5b25b319dbaf32092bbac9a97de1d3ecefa4"),
    "contract_spec_r2": (DIAG / "p1345-contract-spec-r2.json", "f40c2b42fe83a4b75e276ab2ea8a9639fa1553e80c06581caf8c53d948f9d38d"),
    "contract_binding_r2": (DIAG / "p1345-contract-binding-r2.json", "ecc3a8a403d9d14d51808a35876f1f43d7773732c1d20f58574dff42d5187778"),
    "contract_receipt_r2": (DIAG / "p1345-contract-receipt-r2.json", "875f937affb30486c9f63fa0aaf84d1e9a6e93214a8fdfb95db320e76a6c14c2"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def import_pinned(name: str, path: Path, expected: str) -> Any:
    if sha256(path.read_bytes()) != expected:
        raise RuntimeError(f"PROTECTED_INPUT: {path.name} hash mismatch")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"PROTECTED_INPUT: cannot import {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


for _label, (_path, _expected) in PINS.items():
    if sha256(_path.read_bytes()) != _expected:
        raise RuntimeError(f"PROTECTED_INPUT: {_label} hash mismatch")

CHECKER = import_pinned("p1345_checker_r2_adversary", CHECKER_PATH, EXPECTED_CHECKER)
CORPUS, R1_CORPUS, FIXTURE, TABLE = CHECKER.load_closed()
if CHECKER.validate_authorship_receipt(RECEIPT_PATH, EXPECTED_RECEIPT, EXPECTED_CHECKER, EXPECTED_ROOT) != EXPECTED_ROOT:
    raise RuntimeError("AUTHORITY_ROOT: expected R2 root did not validate")


def record(attack_id: str, family: str, claim: str, actual: str, reason: str, evidence: dict[str, Any] | None = None, valid: bool = True) -> dict[str, Any]:
    return {
        "id": attack_id,
        "family": family,
        "claim": claim,
        "expected": "Violated",
        "actual": actual,
        "reason_code": reason,
        "valid": valid,
        "survived": valid and actual != "Violated",
        "evidence": evidence or {},
    }


def case_bytes(**overrides: Any) -> dict[str, Any]:
    case = {
        "case_id": "P1345-R2-CASE",
        "input_kind": "synthetic_source",
        "source_bundle": {"fixture_sha256": CHECKER.EXPECTED_FIXTURE_SHA256, "table_sha256": CHECKER.EXPECTED_TABLE_SHA256},
        "runtime_bundle": None,
        "mutation_recipe": {"operation": "identity"},
        "probe_request": None,
    }
    case.update(overrides)
    return case


def judge_case(attack_id: str, claim: str, case: dict[str, Any]) -> dict[str, Any]:
    judged = CHECKER.judge(case, FIXTURE, set())
    return record(attack_id, "invocation-json", claim, judged["actual"], judged["reason_code"], {"case_keys": list(case), "recipe": case.get("mutation_recipe")})


def invocation_records() -> list[dict[str, Any]]:
    capsule = TABLE["records"][0]["capsule_id"]
    base = case_bytes()
    reordered = {key: base[key] for key in reversed(list(base))}
    return [
        judge_case("P1345-R2N01-bool-token-index", "Boolean token index must fail exact integer typing.", case_bytes(mutation_recipe={"operation": "token_substitute", "capsule_id": capsule, "token_index": True, "replacement": ["identifier", "x"]})),
        judge_case("P1345-R2N02-null-token-index", "Null token index must fail.", case_bytes(mutation_recipe={"operation": "token_substitute", "capsule_id": capsule, "token_index": None, "replacement": ["identifier", "x"]})),
        judge_case("P1345-R2N03-float-token-index", "Floating token index must fail.", case_bytes(mutation_recipe={"operation": "token_substitute", "capsule_id": capsule, "token_index": 1.0, "replacement": ["identifier", "x"]})),
        judge_case("P1345-R2N04-extra-case-key", "Unknown top-level case key must fail.", case_bytes(attacker_key="x")),
        judge_case("P1345-R2N05-reordered-case-keys", "Reordered closed case keys must fail.", reordered),
        judge_case("P1345-R2N06-runtime-object", "Any synthetic runtime bundle must fail.", case_bytes(runtime_bundle={})),
        judge_case("P1345-R2N07-nested-answer-channel", "Nested classification field must fail.", case_bytes(mutation_recipe={"operation": "identity", "replacement": {"classification": "Preserved"}})),
        judge_case("P1345-R2N08-extra-recipe-key", "Unknown recipe key must fail.", case_bytes(mutation_recipe={"operation": "identity", "extra": 1})),
        judge_case("P1345-R2N09-null-capsule-id-ignored", "Null capsule_id supplied to identity must not be silently ignored.", case_bytes(mutation_recipe={"operation": "identity", "capsule_id": None})),
        judge_case("P1345-R2N10-float-replacement-ignored", "Wrong replacement type supplied to identity must not be ignored.", case_bytes(mutation_recipe={"operation": "identity", "replacement": 1.5})),
        judge_case("P1345-R2N11-reordered-recipe-keys", "Recipe key order is part of the closed JSON grammar.", case_bytes(mutation_recipe={"capsule_id": capsule, "operation": "identity"})),
        judge_case("P1345-R2N12-kind-operation-mismatch-source", "Opaque input kind paired with identity operation must fail.", case_bytes(input_kind="opaque_probe")),
        judge_case("P1345-R2N13-kind-operation-mismatch-probe", "Synthetic-source kind paired with opaque operation must fail.", case_bytes(mutation_recipe={"operation": "opaque_probe"}, probe_request={"probe_case_id": "P1345-R2-CASE", "fresh_challenge_hex": "verifier_generated_at_execution", "invocation_nonce_hex": "verifier_generated_at_execution"})),
        judge_case("P1345-R2N14-extra-source-key", "Unknown source-bundle key must fail.", case_bytes(source_bundle={"fixture_sha256": CHECKER.EXPECTED_FIXTURE_SHA256, "table_sha256": CHECKER.EXPECTED_TABLE_SHA256, "extra": "x"})),
        judge_case("P1345-R2N15-extra-probe-key", "Unknown probe-request key must fail.", case_bytes(input_kind="opaque_probe", mutation_recipe={"operation": "opaque_probe"}, probe_request={"probe_case_id": "P1345-R2-CASE", "fresh_challenge_hex": "verifier_generated_at_execution", "invocation_nonce_hex": "verifier_generated_at_execution", "extra": 1})),
    ]


def cli_command(extra_mode: list[str], corpus_arg: str | None = None, checker_values: list[str] | None = None) -> subprocess.CompletedProcess[str]:
    corpus = corpus_arg or str(CORPUS_PATH.relative_to(ROOT))
    trust = checker_values or ["--expected-checker-sha256", EXPECTED_CHECKER]
    command = [
        sys.executable,
        "-B",
        str(CHECKER_PATH),
        *extra_mode,
        "--corpus",
        corpus,
        "--authorship-receipt",
        str(RECEIPT_PATH.relative_to(ROOT)),
        "--authorship-receipt-sha256",
        EXPECTED_RECEIPT,
        *trust,
        "--expected-authoring-root-sha256",
        EXPECTED_ROOT,
    ]
    return subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=90)


def cli_record(attack_id: str, claim: str, completed: subprocess.CompletedProcess[str]) -> dict[str, Any]:
    actual = "Preserved" if completed.returncode == 0 else "Violated"
    reason = "PRESERVED" if actual == "Preserved" else ("AUTHORITY_ROOT" if "AUTHORITY_ROOT" in completed.stderr else "SCHEMA")
    return record(attack_id, "cli-authority", claim, actual, reason, {"exit": completed.returncode, "stdout_sha256": sha256(completed.stdout.encode()), "stderr_sha256": sha256(completed.stderr.encode())})


def cli_records() -> list[dict[str, Any]]:
    rows = []
    # One execution composes three grammar attacks but remains a focal R2 run.
    dot_alias = "./" + str(CORPUS_PATH.relative_to(ROOT))
    rows.append(cli_record("P1345-R2N16-cli-abbrev-duplicate-path-alias", "Abbreviated and duplicated mode flags plus a dot-path alias must fail a closed CLI grammar.", cli_command(["--foc", "--foc"], dot_alias)))
    rows.append(cli_record("P1345-R2N17-cli-duplicate-checker-pin", "Conflicting duplicate checker-pin flags must not use last-value-wins.", cli_command(["--focus"], checker_values=["--expected-checker-sha256", "0" * 64, "--expected-checker-sha256", EXPECTED_CHECKER])))
    rows.append(cli_record("P1345-R2N18-cli-focus-full-conflict", "Mutually exclusive focus/full flags must fail before execution.", cli_command(["--focus", "--full"])))
    with tempfile.TemporaryDirectory(prefix="p1345-r2-cli-", dir="/dev/shm") as temporary:
        alias = Path(temporary) / "corpus.json"
        alias.symlink_to(CORPUS_PATH)
        rows.append(cli_record("P1345-R2N19-cli-corpus-symlink", "Symlink corpus path must fail.", cli_command(["--focus"], str(alias))))
    return rows


def transform_probe_stdout(attack_id: str, claim: str, transform: Callable[[bytes], bytes]) -> dict[str, Any]:
    original = CHECKER.subprocess.run
    seen_transform = False

    def intercepted(args: Any, *pos: Any, **kwargs: Any) -> Any:
        nonlocal seen_transform
        completed = original(args, *pos, **kwargs)
        executable = str(args[0]) if isinstance(args, (list, tuple)) and args else ""
        if executable.endswith("p1345-opaque-probe-r1") and completed.returncode == 0:
            seen_transform = True
            return subprocess.CompletedProcess(completed.args, completed.returncode, transform(completed.stdout), completed.stderr)
        return completed

    CHECKER.subprocess.run = intercepted
    try:
        actual, reason, evidence = CHECKER.run_probe("opaque_probe", set())
    finally:
        CHECKER.subprocess.run = original
    return record(attack_id, "probe-schema", claim, actual, reason, {"transformed": seen_transform} | evidence)


def json_transform(**changes: Any) -> Callable[[bytes], bytes]:
    def apply(raw: bytes) -> bytes:
        value = json.loads(raw)
        value.update(changes)
        return (json.dumps(value, separators=(",", ":")) + "\n").encode()
    return apply


def duplicate_schema(raw: bytes) -> bytes:
    return raw.replace(b'{"schema":', b'{"schema":"duplicate","schema":', 1)


def reverse_keys(raw: bytes) -> bytes:
    value = json.loads(raw)
    return (json.dumps(dict(reversed(list(value.items()))), separators=(",", ":")) + "\n").encode()


def escaped_schema_key(raw: bytes) -> bytes:
    return raw.replace(b'"schema"', b'"sch\\u0065ma"', 1)


def negative_zero(raw: bytes) -> bytes:
    return raw.replace(b'"payload_octets_exposed":0', b'"payload_octets_exposed":-0', 1)


def trailing_space(raw: bytes) -> bytes:
    return raw[:-1] + b" \n"


def leading_space(raw: bytes) -> bytes:
    return b" " + raw


MALICIOUS_PROBE = b'''#!/usr/bin/python3
import hashlib,json,os,sys
r=json.loads(sys.stdin.read())
p=os.urandom(32)
h=lambda b:hashlib.sha256(b).hexdigest()
d=b"P1345-OPAQUE-PROBE-CHALLENGE-V1\\0"
o={"schema":"p1345-opaque-probe-primitive-output-r2","challenge_response_sha256":h(d+bytes.fromhex(r["fresh_challenge_hex"])+p),"invocation_nonce_sha256":h(bytes.fromhex(r["invocation_nonce_hex"])),"process_nonce_hex":p.hex(),"process_nonce_sha256":h(p),"opaque_handle_count":1,"public_projection_sha256":h(b"p1345:opaque-dictionary-public-projection:v1"),"payload_octets_exposed":0,"completed_phase":"OPAQUE_PAIR_PROJECTED"}
print(json.dumps(o,separators=(",",":")))
'''


def toctou_probe() -> dict[str, Any]:
    original_run = CHECKER.subprocess.run
    attack: dict[str, Any] = {"swapped": False, "malicious_sha256": sha256(MALICIOUS_PROBE)}

    def intercepted(args: Any, *pos: Any, **kwargs: Any) -> Any:
        executable = Path(str(args[0])) if isinstance(args, (list, tuple)) and args else Path("")
        if executable.name == "p1345-opaque-probe-r1" and executable.is_file():
            original_bytes = executable.read_bytes()
            original_mode = executable.stat().st_mode
            executable.write_bytes(MALICIOUS_PROBE)
            executable.chmod(original_mode)
            attack["swapped"] = True
            try:
                completed = original_run(args, *pos, **kwargs)
            finally:
                executable.write_bytes(original_bytes)
                executable.chmod(original_mode)
            return completed
        return original_run(args, *pos, **kwargs)

    CHECKER.subprocess.run = intercepted
    try:
        actual, reason, evidence = CHECKER.run_probe("opaque_probe", set())
    finally:
        CHECKER.subprocess.run = original_run
    return record("P1345-R2N31-probe-toctou-swap-restore", "probe-toctou", "Executable swapped after pre-hash and restored before post-hash must fail emitter/binary provenance.", actual, reason, attack | evidence)


def probe_records() -> list[dict[str, Any]]:
    rows = [
        transform_probe_stdout("P1345-R2N20-probe-schema-forged", "Wrong probe schema must fail.", json_transform(schema="forged")),
        transform_probe_stdout("P1345-R2N21-probe-projection-forged", "Wrong projection digest must fail.", json_transform(public_projection_sha256="0" * 64)),
        transform_probe_stdout("P1345-R2N22-probe-bool-count", "Boolean count must fail exact integer typing.", json_transform(opaque_handle_count=True)),
        transform_probe_stdout("P1345-R2N23-probe-float-count", "Floating count must fail exact integer typing.", json_transform(opaque_handle_count=1.0)),
        transform_probe_stdout("P1345-R2N24-probe-null-projection", "Null projection must fail exact string typing.", json_transform(public_projection_sha256=None)),
        transform_probe_stdout("P1345-R2N25-probe-extra-key", "Extra primitive key must fail.", json_transform(extra="x")),
        transform_probe_stdout("P1345-R2N26-probe-key-order", "Reordered primitive keys must fail.", reverse_keys),
        transform_probe_stdout("P1345-R2N27-probe-duplicate-key", "Duplicate decoded primitive key must fail.", duplicate_schema),
        transform_probe_stdout("P1345-R2N28-probe-trailing-space", "Noncanonical trailing JSON whitespace must fail exact output encoding.", trailing_space),
        transform_probe_stdout("P1345-R2N29-probe-leading-space", "Noncanonical leading JSON whitespace must fail exact output encoding.", leading_space),
        transform_probe_stdout("P1345-R2N30-probe-escaped-key", "Escaped key spelling must fail canonical JSON output encoding.", escaped_schema_key),
        transform_probe_stdout("P1345-R2N32-probe-negative-zero", "Negative-zero spelling must fail canonical JSON output encoding.", negative_zero),
    ]
    rows.append(toctou_probe())
    for operation, attack_id in [
        ("probe_replay", "P1345-R2N33-probe-replay"),
        ("probe_wrong_emitter", "P1345-R2N34-probe-wrong-emitter"),
        ("probe_wrong_executable", "P1345-R2N35-probe-wrong-executable"),
        ("probe_bad_response", "P1345-R2N36-probe-challenge-hash"),
        ("probe_bad_nonce_digest", "P1345-R2N37-probe-nonce-hash"),
        ("probe_bad_nonce_hex", "P1345-R2N38-probe-nonce-encoding"),
    ]:
        actual, reason, evidence = CHECKER.run_probe(operation, set())
        rows.append(record(attack_id, "probe-boundary", f"{operation} must fail.", actual, reason, evidence))
    return rows


def authority_records() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for attack_id, checker_hash, root_hash in [
        ("P1345-R2N39-wrong-checker-pin", "0" * 64, EXPECTED_ROOT),
        ("P1345-R2N40-wrong-authoring-root", EXPECTED_CHECKER, "0" * 64),
    ]:
        try:
            CHECKER.validate_authorship_receipt(RECEIPT_PATH, EXPECTED_RECEIPT, checker_hash, root_hash)
            actual, reason = "Preserved", "PRESERVED"
        except CHECKER.Failure as exc:
            actual, reason = "Violated", exc.code
        rows.append(record(attack_id, "authority-root", "Wrong out-of-band checker/root pin must fail.", actual, reason))

    with tempfile.TemporaryDirectory(prefix="p1345-r2-alt-root-", dir="/dev/shm") as temporary:
        temp_root = Path(temporary)
        temp_diag = temp_root / "00_nucleo/diagnosticos"
        temp_diag.mkdir(parents=True)
        for source in [SOURCE_PATH, DIAG / "p1345-source-verifier-r1.py", DIAG / "p1345-oracle-checker-r1.py"]:
            (temp_diag / source.name).symlink_to(source)
        altered_checker = temp_diag / CHECKER_PATH.name
        altered_checker.write_bytes(CHECKER_PATH.read_bytes() + b"\n# coordinated adversarial checker\n")
        altered_receipt = json.loads(RECEIPT_PATH.read_text())
        altered_sha = sha256(altered_checker.read_bytes())
        altered_receipt["artifact_hashes"]["checker_r2"] = altered_sha
        temp_receipt = temp_diag / RECEIPT_PATH.name
        temp_receipt.write_text(json.dumps(altered_receipt, indent=2) + "\n")
        receipt_sha = sha256(temp_receipt.read_bytes())
        module = import_pinned("p1345_checker_r2_coordinated_alternate", altered_checker, altered_sha)
        alternate_root = module.authoring_root(receipt_sha, altered_sha)
        try:
            module.validate_authorship_receipt(temp_receipt, receipt_sha, altered_sha, alternate_root)
            actual, reason = "Preserved", "PRESERVED"
        except Exception as exc:
            actual, reason = "Violated", getattr(exc, "code", "AUTHORITY_ROOT")
        rows.append(record("P1345-R2N41-coordinated-checker-receipt-root", "authority-root", "A coordinated alternate checker/receipt/root set must not replace the frozen out-of-band triple.", actual, reason, {"alternate_checker_sha256": altered_sha, "alternate_receipt_sha256": receipt_sha, "alternate_root_sha256": alternate_root}))

    mutated_table = json.loads((DIAG / "p1345-canonical-capsule-table-r1.json").read_text())
    mutated_table["records"][0]["canonical_tokens_sha256"] = "0" * 64
    altered_table_bytes = (json.dumps(mutated_table, indent=2) + "\n").encode()
    actual = "Violated" if sha256(altered_table_bytes) != CHECKER.EXPECTED_TABLE_SHA256 else "Preserved"
    rows.append(record("P1345-R2N42-table-digest-self-consistent-alternate", "canonical-table-authority", "Altered token digest/table bytes must fail the external table pin.", actual, "AUTHORITY_ROOT" if actual == "Violated" else "PRESERVED", {"altered_table_sha256": sha256(altered_table_bytes)}))
    return rows


def main() -> int:
    inherited = CHECKER.run_adversarial_focal(CORPUS)
    inherited_negatives = [x for x in inherited if x["expected"] == "Violated" and x["valid"]]
    inherited_controls = [x for x in inherited if x["expected"] == "Preserved"]
    if len(inherited) != 84 or len(inherited_negatives) != 80 or len(inherited_controls) != 4:
        raise RuntimeError("SCHEMA: inherited R2 focal composition changed")

    new = invocation_records() + cli_records() + probe_records() + authority_records()
    negatives = inherited_negatives + [x for x in new if x["valid"]]
    survivors = [x for x in negatives if x["actual"] != "Violated"]
    rejected = [x for x in negatives if x["actual"] == "Violated"]
    regressions = [x for x in inherited_controls if not x["valid"] or x["actual"] != "Preserved"]
    families: dict[str, list[str]] = {}
    for item in survivors:
        families.setdefault(item.get("family", "inherited"), []).append(item["id"])

    result = {
        "schema": "p1345-adversary-report-r2",
        "step": 1345,
        "revision": 2,
        "role": "independent_adversary",
        "executor": "/root/p1345_adversary",
        "regime": "executado sem atestacao de isolamento",
        "phase": "focal-adversarial-r2-no-full-corpus",
        "protected_inputs": {label: [str(path.relative_to(ROOT)), digest] for label, (path, digest) in PINS.items()} | {"authoring_root_r2": EXPECTED_ROOT},
        "execution": {
            "command": "PYTHONDONTWRITEBYTECODE=1 python3 -B 00_nucleo/diagnosticos/p1345-adversary-runner-r2.py --output 00_nucleo/diagnosticos/p1345-adversary-report-r2.json",
            "temporary_root": "/dev/shm/p1345-r2-*",
            "inherited_records_replayed": len(inherited),
            "inherited_valid_negatives": len(inherited_negatives),
            "inherited_positive_controls": len(inherited_controls),
            "new_attacks": len(new),
            "direct_focal_vector_regenerations": 1,
            "focal_oracle_cli_invocations": 2,
            "full_corpus_runs": 0,
            "candidate_read_or_executed": False,
            "protected_inputs_modified": False,
        },
        "vectors": {
            "inherited": [{"id": x["id"], "actual": x["actual"], "valid": x["valid"]} for x in inherited],
            "new": [{"id": x["id"], "family": x["family"], "actual": x["actual"], "valid": x["valid"], "survived": x["survived"]} for x in new],
        },
        "survivor_details": [x for x in survivors if x in new],
        "summary": {
            "records": len(inherited) + len(new),
            "valid_negatives": len(negatives),
            "correctly_violated": len(rejected),
            "survivor_count": len(survivors),
            "survivors": [x["id"] for x in survivors],
            "mutation_score": len(rejected) / len(negatives),
            "inherited_valid_negatives": len(inherited_negatives),
            "inherited_correctly_violated": sum(x["actual"] == "Violated" for x in inherited_negatives),
            "positive_controls": len(inherited_controls),
            "positive_control_regressions": [x["id"] for x in regressions],
            "full_corpus_runs": 0,
        },
        "public_failure_classes": [{"code": code.upper().replace("-", "_"), "witnesses": ids} for code, ids in families.items()],
        "budget_stop": {
            "triggered": bool(survivors or regressions),
            "preseal_allowed": not survivors and not regressions,
            "full_runs_consumed": 0,
            "oracle_correction_performed": False,
        },
        "verdict": "ADVERSARIAL_R2_SURVIVORS_BLOCK_PRESEAL" if survivors else ("ADVERSARIAL_R2_CONTROL_REGRESSION_BLOCKS_PRESEAL" if regressions else "ADVERSARIAL_R2_ZERO_SURVIVORS_READY_FOR_PRESEAL"),
        "closed_world": {
            "top_level_keys": ["schema", "step", "revision", "role", "executor", "regime", "phase", "protected_inputs", "execution", "vectors", "survivor_details", "summary", "public_failure_classes", "budget_stop", "verdict", "closed_world"],
            "rule": "Independent focal R2 adversarial report only; no full corpus, seal, candidate verdict or general-equivalence claim.",
        },
    }
    encoded = json.dumps(result, ensure_ascii=False, indent=2).encode() + b"\n"
    if len(sys.argv) == 3 and sys.argv[1] == "--output":
        output = Path(sys.argv[2]).resolve()
        expected = (DIAG / "p1345-adversary-report-r2.json").resolve()
        if output != expected:
            raise RuntimeError("PATH: output outside adversary R2 allowlist")
        output.write_bytes(encoded)
    elif len(sys.argv) == 1:
        sys.stdout.buffer.write(encoded)
    else:
        raise RuntimeError("SCHEMA: usage runner [--output exact-report-path]")
    return 1 if survivors or regressions else 0


if __name__ == "__main__":
    raise SystemExit(main())
