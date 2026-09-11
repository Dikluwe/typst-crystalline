#!/usr/bin/env python3
"""Independent focal adversary for the pinned P1345 oracle R1.

This runner never invokes ``--full`` and never reads a productive candidate.
It reconstructs synthetic trees only under /dev/shm, replays the forty P1344
attack intentions against the P1345 closed-token oracle, and adds independent
source, authority, path, schema and opaque-probe mutations.
"""

from __future__ import annotations

import base64
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
CHECKER_PATH = DIAG / "p1345-oracle-checker-r1.py"
SOURCE_PATH = DIAG / "p1345-source-verifier-r1.py"
CORPUS_PATH = DIAG / "p1345-oracle-corpus-r1.json"
FIXTURE_PATH = DIAG / "p1345-positive-fixture-r1.json"
TABLE_PATH = DIAG / "p1345-canonical-capsule-table-r1.json"
PROBE_PATH = DIAG / "p1345-opaque-probe-r1.rs"
AUTHORSHIP_PATH = DIAG / "p1345-oracle-authorship-r1.md"
AUTHORSHIP_RECEIPT_PATH = DIAG / "p1345-oracle-authorship-receipt-r1.json"

PINS = {
    "step": (ROOT / "00_nucleo/materialization/typst-passo-1345.md", "c2ef7ff7b0555a4f44ca6811ff3e687ef6d6a30f55e0cd106fba4118f3a18fb0"),
    "manifest": (DIAG / "p1345-authority-manifest-r1.json", "2ed02b31b39c6d588db7772cf932af40e344e87947662e12b7a8fe5c7e96b2c2"),
    "freeze": (DIAG / "p1345-l0-toolchain-freeze-r1.json", "9b1ed01ac9c7d0e5c5121906138fe4d4777ba2ce2982d0b89cafb2fef631bbf7"),
    "baseline": (DIAG / "p1345-capsule-baseline-r1.json", "e083f6b28d989afecf95d4b24d5483cef1d6bed45d5ce239f818156f56f05d08"),
    "topology": (DIAG / "p1345-topology-receipt-r1.json", "f4c18c856ab039ae6c99357442a97661425e2a135c87ce35194ecaaf9c571072"),
    "contract_spec_r2": (DIAG / "p1345-contract-spec-r2.json", "f40c2b42fe83a4b75e276ab2ea8a9639fa1553e80c06581caf8c53d948f9d38d"),
    "contract_binding_r2": (DIAG / "p1345-contract-binding-r2.json", "ecc3a8a403d9d14d51808a35876f1f43d7773732c1d20f58574dff42d5187778"),
    "contract_receipt_r2": (DIAG / "p1345-contract-receipt-r2.json", "875f937affb30486c9f63fa0aaf84d1e9a6e93214a8fdfb95db320e76a6c14c2"),
    "fixture": (FIXTURE_PATH, "8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"),
    "table": (TABLE_PATH, "ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"),
    "corpus": (CORPUS_PATH, "daf610f19634bea6245f08e530fcb75a53532500c63c2f8a98ef583096573a1f"),
    "source": (SOURCE_PATH, "a787f52fbcc2590255d50d982c8f0ccbca84f0c750062ede05aa6b171744d8b7"),
    "probe": (PROBE_PATH, "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"),
    "checker": (CHECKER_PATH, "8edf345e27e4b12bf4e2eb39cefa2a854f6ea73c2749e6a76f3d93c1119abc5e"),
    "authorship": (AUTHORSHIP_PATH, "7896fe50771838e85e1560b1b56754b30699f7c3b3925fdefc688c891fe510ac"),
    "authorship_receipt": (AUTHORSHIP_RECEIPT_PATH, "d96404bf1811b5ab891438c117115c94657837a05cc11c0c0e1f1ef5588d54b4"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def import_pinned(name: str, path: Path, digest: str) -> Any:
    if sha256(path.read_bytes()) != digest:
        raise RuntimeError(f"PROTECTED_INPUT: {path.name} hash mismatch")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"PROTECTED_INPUT: cannot load {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


for _label, (_path, _digest) in PINS.items():
    if sha256(_path.read_bytes()) != _digest:
        raise RuntimeError(f"PROTECTED_INPUT: {_label} hash mismatch")

SOURCE = import_pinned("p1345_source_adversary", SOURCE_PATH, PINS["source"][1])
CHECKER = import_pinned("p1345_checker_adversary", CHECKER_PATH, PINS["checker"][1])
CORPUS, FIXTURE, TABLE = CHECKER.load_closed()
RECORDS = {item["capsule_id"]: item for item in TABLE["records"]}


def rustfmt_file(path: Path) -> tuple[bool, str]:
    completed = subprocess.run(
        ["rustfmt", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", str(path)],
        capture_output=True,
        text=True,
        timeout=30,
    )
    lines = (completed.stderr or completed.stdout).splitlines()
    return completed.returncode == 0, (lines[0] if lines else "rustfmt accepted")


def bounds(raw: bytes, capsule_id: str) -> tuple[int, int]:
    begin = b"// P1343-CAPSULE-BEGIN " + capsule_id.encode() + b"\n"
    end = b"// P1343-CAPSULE-END " + capsule_id.encode() + b"\n"
    start = raw.index(begin) + len(begin)
    return start, raw.index(end, start)


def replace_body(root: Path, capsule_id: str, transform: Callable[[bytes], bytes]) -> Path:
    record = RECORDS[capsule_id]
    path = root / record["path"]
    raw = path.read_bytes()
    start, end = bounds(raw, capsule_id)
    path.write_bytes(raw[:start] + transform(raw[start:end]) + raw[end:])
    return path


def source_record(
    attack_id: str,
    capsule_id: str,
    transform: Callable[[bytes], bytes],
    claim: str,
    origin: str,
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="p1345-adversary-source-", dir="/dev/shm") as temporary:
        root = Path(temporary)
        SOURCE.materialize_fixture(root, FIXTURE)
        path = replace_body(root, capsule_id, transform)
        parser_valid, parser_witness = rustfmt_file(path)
        try:
            result = SOURCE.verify_root(root, attack_id)
            actual = "Preserved" if result["source_result"]["passed"] else "Violated"
            reason = result["source_result"]["first_reason_code"] or "PRESERVED"
        except SOURCE.VerificationFailure as exc:
            actual, reason = "Violated", exc.code
        return {
            "id": attack_id,
            "origin": origin,
            "family": "source-negative",
            "capsule": capsule_id,
            "claim": claim,
            "expected": "Violated",
            "actual": actual,
            "valid": parser_valid,
            "parser": "PASS" if parser_valid else "FAIL",
            "parser_witness": parser_witness,
            "reason_code": reason,
            "survived": parser_valid and actual != "Violated",
        }


def source_control(
    control_id: str,
    capsule_id: str,
    transform: Callable[[bytes], bytes],
    claim: str,
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="p1345-adversary-control-", dir="/dev/shm") as temporary:
        root = Path(temporary)
        SOURCE.materialize_fixture(root, FIXTURE)
        path = replace_body(root, capsule_id, transform)
        parser_valid, parser_witness = rustfmt_file(path)
        result = SOURCE.verify_root(root, control_id)
        actual = "Preserved" if result["source_result"]["passed"] else "Violated"
        return {
            "id": control_id,
            "origin": "p1345-new",
            "family": "source-control",
            "capsule": capsule_id,
            "claim": claim,
            "expected": "Preserved",
            "actual": actual,
            "valid": parser_valid,
            "parser": "PASS" if parser_valid else "FAIL",
            "parser_witness": parser_witness,
            "reason_code": result["source_result"]["first_reason_code"] or "PRESERVED",
            "survived": False,
        }


def meta_record(
    attack_id: str,
    family: str,
    claim: str,
    actual: str,
    reason: str,
    origin: str = "p1345-new",
    evidence: dict[str, Any] | None = None,
) -> dict[str, Any]:
    return {
        "id": attack_id,
        "origin": origin,
        "family": family,
        "capsule": None,
        "claim": claim,
        "expected": "Violated",
        "actual": actual,
        "valid": True,
        "parser": "NOT_APPLICABLE",
        "parser_witness": "not a source mutation",
        "reason_code": reason,
        "survived": actual != "Violated",
        "evidence": evidence or {},
    }


CFG = b"#[cfg(p1339_observation)]\n"


# Exact P1344 R1 attack intentions.
def r1_content_no_match(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h11(&self) -> Option<()> { let _variant = Content::CounterUpdate; None }\n"


def r1_content_inverts(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h11(&self) -> Option<()> { match self { Content::CounterUpdate(_) => None, _ => None } }\n"


def r1_content_static(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h11() { let _variant = Content::CounterUpdate; }\n"


def r1_content_foreign(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h11(&self) { let _variant = Content::CounterUpdate; let _foreign = Func::p1343_carrier; }\n"


def r1_counter_no_match(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h10(&self) { let _variants = (CounterUpdate::Func, CounterUpdate::Set, CounterUpdate::Step); }\n"


def r1_counter_inverts(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h10(&self) -> Option<()> { match self { CounterUpdate::Func(_) => None, CounterUpdate::Set(_) => Some(()), CounterUpdate::Step(_) => Some(()) } }\n"


def r1_counter_static(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h10() { let _variants = (CounterUpdate::Func, CounterUpdate::Set, CounterUpdate::Step); }\n"


def r1_counter_executes(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h10(&self) -> Option<()> { match self { CounterUpdate::Func(function) => { function.call(); None }, CounterUpdate::Set(_) | CounterUpdate::Step(_) => None } }\n"


def r1_func_loop(body: bytes) -> bytes:
    return body.replace(b"(&self) {}", b"(&self) { loop {} }", 1)


def r1_func_shadow(body: bytes) -> bytes:
    return body.replace(b"(&self) {}", b"(&self) { let mut shadow = Vec::<u8>::new(); shadow.pop(); }", 1)


def r1_eval_wrong(_: bytes) -> bytes:
    return CFG + b"impl EvalContext { fn p1343_totally_wrong(&mut self) {} }\n"


def r1_direct_extra(body: bytes) -> bytes:
    return body.replace(b"attempt_kind);", b"side_effect(), attempt_kind);", 1)


def r1_comment_decoy(body: bytes) -> bytes:
    return body.replace(b"p1343_observe_h01", b"p1343_wrong /* p1343_observe_h01 */", 1)


def r1_carrier_eraser(body: bytes) -> bytes:
    return body.replace(b"fn snapshot(&mut self)", b"fn erase(&mut self) { self.state.events.pop(); }\nfn snapshot(&mut self)", 1)


def r1_carrier_mutable(body: bytes) -> bytes:
    return body.replace(b"fn project(&self)", b"fn project(&mut self)", 1)


def r1_carrier_second(body: bytes) -> bytes:
    return CFG + b"struct P1343ShadowStorage { values: Vec<u8> }\n" + body


def r1_facade_wrong_world(body: bytes) -> bytes:
    body = body.replace(b"world: &World, source:", b"world: &World, other_world: &World, source:", 1)
    return body.replace(b"run_context_stabilization_once(world,", b"run_context_stabilization_once(other_world,", 1)


def r1_expression_loop(body: bytes) -> bytes:
    return body.replace(b"{\np1343_append_raw", b"{\nloop {}\np1343_append_raw", 1)


def r1_string_owner(_: bytes) -> bytes:
    return CFG + b"pub(crate) fn p1343_observe_h11(&self) { let _s = \"Content::CounterUpdate\"; }\n"


R1_SOURCE = [
    ("P1344-A01-content-token-without-match", "P1344-CONTENT-CARRIER-HELPERS", r1_content_no_match),
    ("P1344-A02-content-counter-update-returns-absence", "P1344-CONTENT-CARRIER-HELPERS", r1_content_inverts),
    ("P1344-A03-content-helper-wrong-signature", "P1344-CONTENT-CARRIER-HELPERS", r1_content_static),
    ("P1344-A04-content-foreign-func-indirection", "P1344-CONTENT-CARRIER-HELPERS", r1_content_foreign),
    ("P1344-A05-counter-variant-tokens-without-match", "P1344-COUNTER-UPDATE-CARRIER-HELPERS", r1_counter_no_match),
    ("P1344-A06-counter-set-step-return-presence", "P1344-COUNTER-UPDATE-CARRIER-HELPERS", r1_counter_inverts),
    ("P1344-A07-counter-helper-wrong-signature", "P1344-COUNTER-UPDATE-CARRIER-HELPERS", r1_counter_static),
    ("P1344-A08-counter-executes-callback", "P1344-COUNTER-UPDATE-CARRIER-HELPERS", r1_counter_executes),
    ("P1344-A09-func-helper-nonterminating", "P1343-FUNC-CARRIER-HELPERS", r1_func_loop),
    ("P1344-A10-func-helper-alternate-storage", "P1343-FUNC-CARRIER-HELPERS", r1_func_shadow),
    ("P1344-A11-inherited-eval-helper-set-replaced", "P1343-EVAL-CARRIER-HELPERS", r1_eval_wrong),
    ("P1344-A12-direct-hook-extra-side-effect-argument", "P1343-H01-ATTEMPT-OPEN", r1_direct_extra),
    ("P1344-A13-comment-callee-decoy", "P1343-H01-ATTEMPT-OPEN", r1_comment_decoy),
    ("P1344-A14-carrier-extra-eraser", "P1343-FUNC-CARRIER-TYPES", r1_carrier_eraser),
    ("P1344-A15-carrier-mutable-projection", "P1343-FUNC-CARRIER-TYPES", r1_carrier_mutable),
    ("P1344-A16-carrier-second-storage", "P1343-FUNC-CARRIER-TYPES", r1_carrier_second),
    ("P1344-A17-facade-substitutes-world", "P1343-TEST-FACADE", r1_facade_wrong_world),
    ("P1344-A18-observed-expression-nontermination", "P1343-H06-BOUND-UPDATE", r1_expression_loop),
    ("P1344-A19-string-owner-decoy", "P1344-CONTENT-CARRIER-HELPERS", r1_string_owner),
]


# Exact P1344 R2 attack intentions.
def r2_func_decoy(body: bytes) -> bytes:
    return body.replace(b"pub(crate) fn p1343_carrier(&self) {}", b"pub(crate) fn p1343_carrier(&self) { let (p1343_carrier, append) = (0, 0); }", 1)


def r2_func_foreign(body: bytes) -> bytes:
    return body.replace(b"(&self) {}", b"(&self) { other.p1343_carrier(); }", 1)


def r2_eval_static(body: bytes) -> bytes:
    return body.replace(b"(&mut self)", b"()")


def r2_eval_shadow(body: bytes) -> bytes:
    return body.replace(b"(&mut self) {}", b"(&mut self) { let mut shadow = Vec::<u8>::new(); shadow.pop(); }", 1)


def r2_alias_storage(body: bytes) -> bytes:
    body = CFG + b"type P1343ShadowStorage = Option<Box<[u8]>>;\n" + body
    body = body.replace(b"struct P1343Carrier { state: P1343LedgerState }", b"struct P1343Carrier { state: P1343LedgerState, shadow: P1343ShadowStorage }")
    return body.replace(b"Self { state: P1343LedgerState", b"Self { shadow: None, state: P1343LedgerState", 1)


def r2_remove(body: bytes) -> bytes:
    return body.replace(b"self.state.frozen = true;", b"self.state.events.remove(0); self.state.frozen = true;", 1)


def r2_dead_writer(body: bytes) -> bytes:
    return body.replace(b"self.state.events.push(event); self.state.receipts.push(receipt);", b"let _dead = || { self.state.events.push(event); self.state.receipts.push(receipt); };", 1)


def r2_conditional(body: bytes) -> bytes:
    return body.replace(b"self.state.events.push(event);", b"if false { self.state.events.push(event); }", 1)


def r2_swap_remove(body: bytes) -> bytes:
    return body.replace(b"self.state.frozen = true;", b"self.state.events.swap_remove(0); self.state.frozen = true;", 1)


def r2_facade_side(body: bytes) -> bytes:
    return body.replace(b"let raw =", b"side_effect();\nlet raw =", 1)


def r2_facade_fabricated(body: bytes) -> bytes:
    return body.replace(b"let projection = raw.project();\n(raw, projection)", b"let _ignored = raw.project();\n(raw, P1343Projection)", 1)


def r2_facade_dead(_: bytes) -> bytes:
    return CHECKER.SOURCE.cfg("all(test,p1339_observation)") + b"fn p1342_run_fixture_for_test(world: &World, source: Source, challenge: [u8; 32], mode: P1343Mode) -> (P1343RawSnapshot, P1343Projection) { let _dead = || run_context_stabilization_once(world, source, challenge, mode); let raw = P1343RawSnapshot { events: Vec::new(), receipts: Vec::new() }; let projection = raw.project(); (raw, projection) }\n"


def r2_dead_observation(body: bytes) -> bytes:
    return body.replace(b'p1343_append_raw("H06", p1343_event());', b'if false { p1343_append_raw("H06", p1343_event()); }', 1)


def r2_owner_side(body: bytes) -> bytes:
    return body.replace(b"match self", b"side_effect(); (match self", 1).replace(b"_ => None }", b"_ => None })", 1)


R2_SOURCE = [
    ("P1344-R2A01-func-observer-token-decoys", "P1343-FUNC-CARRIER-HELPERS", r2_func_decoy),
    ("P1344-R2A02-func-observer-foreign-receiver", "P1343-FUNC-CARRIER-HELPERS", r2_func_foreign),
    ("P1344-R2A03-eval-exact-names-static-signatures", "P1343-EVAL-CARRIER-HELPERS", r2_eval_static),
    ("P1344-R2A04-eval-exact-names-shadow-storage", "P1343-EVAL-CARRIER-HELPERS", r2_eval_shadow),
    ("P1344-R2A05-carrier-aliased-non-vec-storage", "P1343-FUNC-CARRIER-TYPES", r2_alias_storage),
    ("P1344-R2A06-carrier-remove-outside-append", "P1343-FUNC-CARRIER-TYPES", r2_remove),
    ("P1344-R2A07-carrier-pushes-in-dead-closure", "P1343-FUNC-CARRIER-TYPES", r2_dead_writer),
    ("P1344-R2A08-carrier-conditional-event-loss", "P1343-FUNC-CARRIER-TYPES", r2_conditional),
    ("P1344-R2A09-carrier-swap-remove-spelling", "P1343-FUNC-CARRIER-TYPES", r2_swap_remove),
    ("P1344-R2A10-facade-unlisted-lateral-call", "P1343-TEST-FACADE", r2_facade_side),
    ("P1344-R2A11-facade-fabricated-return-projection", "P1343-TEST-FACADE", r2_facade_fabricated),
    ("P1344-R2A12-facade-product-call-dead-closure", "P1343-TEST-FACADE", r2_facade_dead),
    ("P1344-R2A13-replacement-hook-under-if-false", "P1343-H06-BOUND-UPDATE", r2_dead_observation),
    ("P1344-R2A14-owner-parenthesized-with-lateral-call", "P1344-CONTENT-CARRIER-HELPERS", r2_owner_side),
]


def duplicate_json() -> tuple[str, str]:
    try:
        SOURCE.strict_json(b'{"owner":1,"\\u006fwner":2}', "P1345 adversarial duplicate")
    except SOURCE.VerificationFailure as exc:
        return "Violated", exc.code
    return "Preserved", "PRESERVED"


def alternate_corpus_cli() -> tuple[str, str, dict[str, Any]]:
    with tempfile.TemporaryDirectory(prefix="p1345-alt-corpus-", dir="/dev/shm") as temporary:
        alternate = Path(temporary) / "corpus.json"
        alternate.write_text('{"schema":"p1345-oracle-corpus-r1","cases":[]}\n')
        completed = subprocess.run(
            [sys.executable, "-B", str(CHECKER_PATH), "--focus", "--corpus", str(alternate)],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
        )
        actual = "Violated" if completed.returncode == 2 and "AUTHORITY_ROOT" in completed.stderr else "Preserved"
        return actual, "AUTHORITY_ROOT" if actual == "Violated" else "PRESERVED", {"exit": completed.returncode, "stderr_sha256": sha256(completed.stderr.encode())}


def judge_injected_case(case_id: str, operation: str, extra: dict[str, Any]) -> tuple[str, str, dict[str, Any]]:
    case = {
        "case_id": case_id,
        "input_kind": "candidate",
        "source_bundle": {"fixture_sha256": PINS["fixture"][1], "table_sha256": PINS["table"][1]},
        "runtime_bundle": None,
        "mutation_recipe": {"operation": operation},
        "probe_request": None,
    }
    case.update(extra)
    actual = CHECKER.judge(case, FIXTURE, set())["actual"]
    return actual, "PRESERVED" if actual == "Preserved" else ("OPAQUE_PAYLOAD" if actual == "Unknown" else "VIOLATED"), {"injected_keys": sorted(extra)}


def replay_meta() -> list[dict[str, Any]]:
    actual, reason = duplicate_json()
    records = [meta_record("P1344-A20-escaped-duplicate-json-key", "duplicate-json", "Decoded-equivalent duplicate JSON keys must fail.", actual, reason, "p1344-replay")]
    actual, reason, evidence = alternate_corpus_cli()
    records.append(meta_record("P1344-A21-alternate-corpus-and-digest", "authority-root", "Alternate corpus bytes must fail the compiled root.", actual, reason, "p1344-replay", evidence))
    actual, reason, evidence = judge_injected_case("P1344-A22-unknown-without-dict-witness", "probe_missing", {})
    records.append(meta_record("P1344-A22-unknown-without-dict-witness", "opaque-probe", "Missing opaque probe evidence must be Violated.", actual, reason, "p1344-replay", evidence))
    actual, reason, evidence = judge_injected_case("P1344-A23-synthetic-candidate-runtime", "identity", {"runtime_bundle": {"classification": "Preserved", "synthetic": True}})
    records.append(meta_record("P1344-A23-synthetic-candidate-runtime", "candidate-boundary", "Synthetic candidate runtime must not be ignored.", actual, reason, "p1344-replay", evidence))
    actual, reason, evidence = judge_injected_case("P1344-R2A15-unknown-canonical-self-attestation", "opaque_probe", {"classification": "Unknown", "witness": "candidate supplied"})
    records.append(meta_record("P1344-R2A15-unknown-canonical-self-attestation", "answer-channel", "Injected classification and witness must fail before probe execution.", actual, reason, "p1344-replay", evidence))
    actual, reason = duplicate_json()
    records.append(meta_record("P1344-R2A16-escaped-duplicate-json-key", "duplicate-json", "Escaped duplicate JSON key must fail.", actual, reason, "p1344-replay"))
    actual, reason, evidence = alternate_corpus_cli()
    records.append(meta_record("P1344-R2A17-alternate-corpus-root", "authority-root", "Self-consistent alternate corpus root must fail.", actual, reason, "p1344-replay", evidence))
    return records


def mutate_insert_item(item: bytes) -> Callable[[bytes], bytes]:
    return lambda body: item + body


def new_source_records() -> list[dict[str, Any]]:
    carrier = "P1343-FUNC-CARRIER-TYPES"
    records = [
        source_record("P1345-N01-token-extra-item", carrier, mutate_insert_item(CFG + b"const P1345_EXTRA: () = ();\n"), "Extra item tokens must diverge.", "p1345-new"),
        source_record("P1345-N02-token-absent-visibility", "P1343-FUNC-CARRIER-HELPERS", lambda b: b.replace(b"pub(crate) ", b"", 1), "Missing visibility tokens must diverge.", "p1345-new"),
        source_record("P1345-N03-token-reorder-enum", carrier, lambda b: b.replace(b"H00D, H00S", b"H00S, H00D", 1), "Reordered tokens must diverge.", "p1345-new"),
        source_record("P1345-N04-token-substitute-ident", carrier, lambda b: b.replace(b"H00D", b"H00X", 1), "Substituted identifier must diverge.", "p1345-new"),
        source_record("P1345-N05-token-substitute-literal", "P1343-H06-BOUND-UPDATE", lambda b: b.replace(b'"H06"', b'"H99"', 1), "Substituted literal must diverge.", "p1345-new"),
        source_record("P1345-N06-token-substitute-keyword", carrier, lambda b: b.replace(b"frozen: false", b"frozen: true", 1), "Substituted keyword must diverge.", "p1345-new"),
        source_record("P1345-N07-lexer-normal-string-comment-decoy", carrier, mutate_insert_item(CFG + b'const P1345_STR: &str = "// /* token */";\n'), "Comment delimiters in a normal literal remain literal tokens.", "p1345-new"),
        source_record("P1345-N08-lexer-raw-string-comment-decoy", carrier, mutate_insert_item(CFG + b'const P1345_RAW: &str = r#"// /* token */"#;\n'), "Comment delimiters in a raw literal remain literal tokens.", "p1345-new"),
        source_record("P1345-N09-lexer-raw-byte-string", carrier, mutate_insert_item(CFG + b'const P1345_RAW_BYTES: &[u8] = br#"/* */"#;\n'), "Raw byte-string token insertion must diverge.", "p1345-new"),
        source_record("P1345-N10-lexer-byte-char", carrier, mutate_insert_item(CFG + b"const P1345_BYTE: u8 = b'x';\n"), "Byte-character literal insertion must diverge.", "p1345-new"),
        source_record("P1345-N11-lexer-raw-identifier", carrier, mutate_insert_item(CFG + b"const r#match: usize = 1usize;\n"), "Raw identifier and suffixed literal insertion must diverge.", "p1345-new"),
    ]
    records.extend([
        source_control("P1345-C01-whitespace-control", "P1343-PIPE-SUPPORT", lambda b: b" \n\t" + b.replace(b"struct", b"struct ", 1), "Whitespace is the sole ignored lexical surface."),
        source_control("P1345-C02-line-comment-control", "P1343-PIPE-SUPPORT", lambda b: b"// ignored token-looking decoy H01\n" + b, "Line comments are intentionally ignored."),
        source_control("P1345-C03-nested-comment-control", "P1343-PIPE-SUPPORT", lambda b: b"/* ignored /* nested */ token decoy */\n" + b, "Nested block comments are intentionally ignored."),
        source_control("P1345-C04-doc-comment-control", "P1343-PIPE-SUPPORT", lambda b: b"/// ignored doc comment\n" + b, "Doc comments are explicitly non-observable by contract."),
    ])
    return records


def path_marker_records() -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    cid = "P1343-PIPE-SUPPORT"
    record = RECORDS[cid]

    def run(attack_id: str, mutator: Callable[[Path, Path], None], claim: str, expected_reason: str | None = None) -> None:
        with tempfile.TemporaryDirectory(prefix="p1345-adversary-path-", dir="/dev/shm") as temporary:
            root = Path(temporary)
            SOURCE.materialize_fixture(root, FIXTURE)
            path = root / record["path"]
            mutator(root, path)
            try:
                result = SOURCE.verify_root(root, attack_id)
                actual = "Preserved" if result["source_result"]["passed"] else "Violated"
                reason = result["source_result"]["first_reason_code"] or "PRESERVED"
            except SOURCE.VerificationFailure as exc:
                actual, reason = "Violated", exc.code
            out.append(meta_record(attack_id, "path-marker-owner-normalization", claim, actual, reason, evidence={"expected_reason": expected_reason}))

    run("P1345-N12-marker-missing", lambda _r, p: p.write_bytes(p.read_bytes().replace(("// P1343-CAPSULE-BEGIN " + cid + "\n").encode(), b"", 1)), "Missing marker must fail.", "MARKER")
    run("P1345-N13-marker-duplicate", lambda _r, p: p.write_bytes(("// P1343-CAPSULE-BEGIN " + cid + "\n").encode() + p.read_bytes()), "Duplicate marker must fail.", "MARKER")
    run("P1345-N14-marker-unknown", lambda _r, p: p.write_bytes(b"// P1343-CAPSULE-BEGIN P1345-UNKNOWN\n" + p.read_bytes()), "Unknown marker must fail by normalization or marker inventory.", "NORMALIZATION")
    run("P1345-N15-anchor-drift", lambda _r, p: p.write_bytes(p.read_bytes().replace(base64.b64decode(record["start_anchor_base64"]), b"// changed anchor\n", 1)), "Anchor drift must fail.", "ANCHOR")
    run("P1345-N16-owner-outside-delta", lambda _r, p: p.write_bytes(b"const P1345_WRONG_OWNER: () = ();\n" + p.read_bytes()), "Outside-owner delta must fail inverse normalization.", "NORMALIZATION")
    run("P1345-N17-normalization-comment-outside", lambda _r, p: p.write_bytes(b"// outside-capsule comment\n" + p.read_bytes()), "Even ignored comments outside capsules must fail whole-file normalization.", "NORMALIZATION")

    def file_symlink(_root: Path, path: Path) -> None:
        external = Path(tempfile.mkdtemp(prefix="p1345-outside-", dir="/dev/shm")) / "source.rs"
        external.write_bytes(path.read_bytes())
        path.unlink()
        path.symlink_to(external)

    run("P1345-N18-file-symlink-escape", file_symlink, "A source-file symlink escaping root must fail.", "PATH")

    with tempfile.TemporaryDirectory(prefix="p1345-real-root-", dir="/dev/shm") as temporary:
        real_root = Path(temporary) / "real"
        real_root.mkdir()
        SOURCE.materialize_fixture(real_root, FIXTURE)
        alias = Path(temporary) / "alias"
        alias.symlink_to(real_root, target_is_directory=True)
        try:
            result = SOURCE.verify_root(alias, "P1345-N19-symlink-root-alias")
            actual = "Preserved" if result["source_result"]["passed"] else "Violated"
            reason = result["source_result"]["first_reason_code"] or "PRESERVED"
        except SOURCE.VerificationFailure as exc:
            actual, reason = "Violated", exc.code
        out.append(meta_record("P1345-N19-symlink-root-alias", "path-marker-owner-normalization", "An attacker-controlled symlink root must not become its own authority by resolution.", actual, reason, evidence={"root_is_symlink": alias.is_symlink()}))
    return out


def run_probe_with_transform(attack_id: str, transform: Callable[[dict[str, Any]], dict[str, Any]]) -> tuple[str, str, dict[str, Any]]:
    original = CHECKER.subprocess.run
    captured: dict[str, Any] = {"transformed": False}

    def intercepted(args: Any, *pos: Any, **kwargs: Any) -> Any:
        completed = original(args, *pos, **kwargs)
        executable = str(args[0]) if isinstance(args, (list, tuple)) and args else ""
        if executable.endswith("p1345-opaque-probe-r1") and completed.returncode == 0:
            primitive = json.loads(completed.stdout)
            primitive = transform(primitive)
            stdout = (json.dumps(primitive, separators=(",", ":")) + "\n").encode()
            captured["transformed"] = True
            captured["output_sha256"] = sha256(stdout)
            return subprocess.CompletedProcess(completed.args, completed.returncode, stdout, completed.stderr)
        return completed

    CHECKER.subprocess.run = intercepted
    try:
        actual, reason, evidence = CHECKER.run_probe("opaque_probe", set())
    finally:
        CHECKER.subprocess.run = original
    return actual, reason, captured | evidence


def probe_records() -> list[dict[str, Any]]:
    transforms: list[tuple[str, str, Callable[[dict[str, Any]], dict[str, Any]]]] = [
        ("P1345-N20-probe-schema-forged", "Wrong primitive schema enum must fail.", lambda x: x | {"schema": "p1345-forged"}),
        ("P1345-N21-probe-public-projection-forged", "Wrong public projection digest must fail.", lambda x: x | {"public_projection_sha256": "0" * 64}),
        ("P1345-N22-probe-public-projection-null", "Wrong public projection type must fail.", lambda x: x | {"public_projection_sha256": None}),
        ("P1345-N23-probe-bool-handle-count", "Boolean true is not the required integer 1.", lambda x: x | {"opaque_handle_count": True}),
        ("P1345-N24-probe-bool-payload-count", "Boolean false is not the required integer 0.", lambda x: x | {"payload_octets_exposed": False}),
        ("P1345-N40-probe-float-handle-count", "Floating-point 1.0 is not the required integer 1.", lambda x: x | {"opaque_handle_count": 1.0}),
        ("P1345-N25-probe-phase-forged", "Wrong completed phase must fail.", lambda x: x | {"completed_phase": "FORGED"}),
        ("P1345-N26-probe-answer-injected", "Primitive classification answer channel must fail.", lambda x: x | {"classification": "Unknown"}),
        ("P1345-N27-probe-process-digest-forged", "Wrong process nonce digest must fail.", lambda x: x | {"process_nonce_sha256": "0" * 64}),
        ("P1345-N28-probe-response-forged", "Wrong challenge response must fail.", lambda x: x | {"challenge_response_sha256": "0" * 64}),
    ]
    records = []
    for attack_id, claim, transform in transforms:
        actual, reason, evidence = run_probe_with_transform(attack_id, transform)
        records.append(meta_record(attack_id, "opaque-probe-negative", claim, actual, reason, evidence=evidence))
    for operation, attack_id in [
        ("probe_replay", "P1345-N29-probe-replay"),
        ("probe_missing", "P1345-N30-probe-missing"),
        ("probe_wrong_emitter", "P1345-N31-probe-wrong-emitter"),
        ("probe_wrong_executable", "P1345-N32-probe-wrong-executable"),
        ("probe_bad_nonce_hex", "P1345-N33-probe-bad-nonce-hex"),
        ("probe_extra_stdout", "P1345-N34-probe-extra-stdout"),
    ]:
        actual, reason, evidence = CHECKER.run_probe(operation, set())
        records.append(meta_record(attack_id, "opaque-probe-negative", f"{operation} must fail.", actual, reason, evidence=evidence))
    return records


def authority_records() -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    actual, reason = duplicate_json()
    records.append(meta_record("P1345-N35-json-duplicate-unicode-key", "authority-schema", "Duplicate decoded JSON keys must fail.", actual, reason))

    # A changed checker can still validate the exact authorship receipt and derive
    # a new self-consistent root because validate_authorship_receipt has no
    # verifier-supplied expected checker digest parameter.
    with tempfile.TemporaryDirectory(prefix="p1345-alt-checker-", dir="/dev/shm") as temporary:
        temp_root = Path(temporary)
        temp_diag = temp_root / "00_nucleo/diagnosticos"
        temp_diag.mkdir(parents=True)
        (temp_diag / SOURCE_PATH.name).symlink_to(SOURCE_PATH)
        (temp_diag / AUTHORSHIP_RECEIPT_PATH.name).symlink_to(AUTHORSHIP_RECEIPT_PATH)
        altered = temp_diag / CHECKER_PATH.name
        altered.write_bytes(CHECKER_PATH.read_bytes() + b"\n# adversarial alternate checker bytes\n")
        module = import_pinned("p1345_alternate_checker", altered, sha256(altered.read_bytes()))
        try:
            root_digest = module.validate_authorship_receipt(temp_diag / AUTHORSHIP_RECEIPT_PATH.name, PINS["authorship_receipt"][1])
            actual, reason = "Preserved", "PRESERVED"
        except Exception as exc:
            root_digest = None
            actual, reason = "Violated", getattr(exc, "code", "AUTHORITY_ROOT")
        records.append(meta_record("P1345-N36-alternate-checker-self-root", "authority-root", "Altered checker bytes must be rejected against an out-of-band checker pin, not folded into a new self-root.", actual, reason, evidence={"altered_checker_sha256": sha256(altered.read_bytes()), "derived_root_sha256": root_digest}))

    actual, reason, evidence = judge_injected_case("P1345-N37-wrong-source-bundle", "identity", {"source_bundle": {"fixture_sha256": "0" * 64, "table_sha256": "0" * 64}})
    records.append(meta_record("P1345-N37-wrong-source-bundle", "invocation-schema", "Case-supplied wrong fixture/table pins must fail before source judgment.", actual, reason, evidence=evidence))
    actual, reason, evidence = judge_injected_case("P1345-N38-unknown-input-kind", "identity", {"input_kind": "attacker-defined-kind"})
    records.append(meta_record("P1345-N38-unknown-input-kind", "invocation-schema", "Unknown input_kind enum must fail.", actual, reason, evidence=evidence))
    actual, reason, evidence = judge_injected_case("P1345-N39-extra-case-key", "identity", {"attacker_key": "ignored"})
    records.append(meta_record("P1345-N39-extra-case-key", "invocation-schema", "Unknown case key must fail closed.", actual, reason, evidence=evidence))
    return records


def main() -> int:
    records: list[dict[str, Any]] = []
    for attack_id, capsule, transform in R1_SOURCE + R2_SOURCE:
        records.append(source_record(attack_id, capsule, transform, "Exact P1344 source attack intention replayed under P1345.", "p1344-replay"))
    records.extend(replay_meta())
    historical = [item for item in records if item["origin"] == "p1344-replay"]
    if len(historical) != 40 or len({item["id"] for item in historical}) != 40:
        raise RuntimeError("SCHEMA: historical replay is not exact 40-ID cardinality")

    records.extend(new_source_records())
    records.extend(path_marker_records())
    records.extend(probe_records())
    records.extend(authority_records())

    negatives = [item for item in records if item["expected"] == "Violated" and item["valid"]]
    rejected = [item for item in negatives if item["actual"] == "Violated"]
    survivors = [item for item in negatives if item["survived"]]
    controls = [item for item in records if item["expected"] == "Preserved"]
    control_regressions = [item for item in controls if not item["valid"] or item["actual"] != "Preserved"]
    historical_survivors = [item for item in historical if item["survived"]]
    new_records = [item for item in records if item["origin"] == "p1345-new"]
    result = {
        "schema": "p1345-adversary-report-r1",
        "step": 1345,
        "revision": 1,
        "role": "independent_adversary",
        "executor": "/root/p1345_adversary",
        "regime": "executado sem atestacao de isolamento",
        "phase": "focal-adversarial-r1-no-full-corpus",
        "protected_inputs": {label: [str(path.relative_to(ROOT)), digest] for label, (path, digest) in PINS.items()},
        "execution": {
            "command": "python3 -B 00_nucleo/diagnosticos/p1345-adversary-runner-r1.py --output 00_nucleo/diagnosticos/p1345-adversary-report-r1.json",
            "temporary_root": "/dev/shm/p1345-adversary-*",
            "p1344_attacks_replayed": len(historical),
            "new_attacks_and_controls": len(new_records),
            "full_corpus_runs": 0,
            "candidate_read_or_executed": False,
            "protected_inputs_modified": False,
        },
        "record_vectors": {
            "historical": [
                {"id": item["id"], "actual": item["actual"], "valid": item["valid"], "survived": item["survived"]}
                for item in historical
            ],
            "new": [
                {"id": item["id"], "expected": item["expected"], "actual": item["actual"], "valid": item["valid"], "survived": item["survived"]}
                for item in new_records
            ],
        },
        "survivor_details": [
            {
                "id": item["id"],
                "origin": item["origin"],
                "family": item["family"],
                "claim": item["claim"],
                "actual": item["actual"],
                "reason_code": item["reason_code"],
                "evidence": item.get("evidence", {}),
            }
            for item in survivors
        ],
        "summary": {
            "records": len(records),
            "valid_negatives": len(negatives),
            "correctly_violated": len(rejected),
            "survivor_count": len(survivors),
            "survivors": [item["id"] for item in survivors],
            "mutation_score": len(rejected) / len(negatives) if negatives else None,
            "historical_replay_count": len(historical),
            "historical_survivor_count": len(historical_survivors),
            "historical_survivors": [item["id"] for item in historical_survivors],
            "positive_controls": len(controls),
            "positive_control_regressions": [item["id"] for item in control_regressions],
            "full_corpus_runs": 0,
        },
        "public_failure_classes": [
            {
                "code": "INVOCATION_SCHEMA_AND_RUNTIME_BOUNDARY_NOT_ENFORCED",
                "witnesses": [item["id"] for item in survivors if item["id"] in {"P1344-A23-synthetic-candidate-runtime", "P1344-R2A15-unknown-canonical-self-attestation", "P1345-N37-wrong-source-bundle", "P1345-N38-unknown-input-kind", "P1345-N39-extra-case-key"}],
                "cause": "judge consumes operation but does not validate the closed case schema, source bundle, input kind, runtime bundle or injected answer fields at the invocation boundary.",
            },
            {
                "code": "OPAQUE_PRIMITIVE_SCHEMA_AND_PROJECTION_NOT_CLOSED",
                "witnesses": [item["id"] for item in survivors if item["family"] == "opaque-probe-negative"],
                "cause": "run_probe checks key names but not the schema enum, public projection digest/type, or exact JSON integer types; Python bool values compare equal to integers.",
            },
            {
                "code": "CHECKER_AND_ROOT_TRUST_DELIVERY_NOT_ENFORCED_BY_EXECUTABLE_INTERFACE",
                "witnesses": [item["id"] for item in survivors if item["id"] in {"P1345-N19-symlink-root-alias", "P1345-N36-alternate-checker-self-root"}],
                "cause": "the checker hashes its own bytes into a new root but accepts no verifier-supplied checker digest, and root symlink identity is normalized away.",
            },
        ],
        "budget_stop": {
            "triggered": bool(survivors or control_regressions),
            "basis": "Any valid survivor, control regression or mutation score below 1.0 blocks preseal under P1345.",
            "full_runs_consumed": 0,
            "preseal_allowed": not survivors and not control_regressions,
            "oracle_correction_performed": False,
        },
        "verdict": "ADVERSARIAL_R1_SURVIVORS_BLOCK_PRESEAL" if survivors else ("ADVERSARIAL_R1_CONTROL_REGRESSION_BLOCKS_PRESEAL" if control_regressions else "ADVERSARIAL_R1_ZERO_SURVIVORS_READY_FOR_PRESEAL"),
        "closed_world": {
            "top_level_keys": ["schema", "step", "revision", "role", "executor", "regime", "phase", "protected_inputs", "execution", "record_vectors", "survivor_details", "summary", "public_failure_classes", "budget_stop", "verdict", "closed_world"],
            "rule": "Focal adversarial report only; it is not a full corpus run, seal, candidate verdict or general equivalence claim.",
        },
    }
    encoded = json.dumps(result, ensure_ascii=False, indent=2, sort_keys=False).encode() + b"\n"
    if len(sys.argv) == 3 and sys.argv[1] == "--output":
        output = Path(sys.argv[2]).resolve()
        expected = (DIAG / "p1345-adversary-report-r1.json").resolve()
        if output != expected:
            raise RuntimeError("PATH: --output is restricted to the P1345 adversary report allowlist")
        output.write_bytes(encoded)
    elif len(sys.argv) == 1:
        sys.stdout.buffer.write(encoded)
    else:
        raise RuntimeError("SCHEMA: usage: runner [--output exact-report-path]")
    return 1 if survivors or control_regressions else 0


if __name__ == "__main__":
    raise SystemExit(main())
