#!/usr/bin/env python3
"""Closed-grammar refinement of the protected P1344 source verifier R1.

R2 composes R1's lexical marker, owner and inverse-normalization proof, then
closes the productions exposed by the independent adversary: owner helper
signatures/match effects, Func/Eval method sets, direct-hook arguments,
carrier storage/API, facade forwarding and observed-wrapper control flow.
It remains source-only and never treats synthetic evidence as candidate proof.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
R1_PATH = DIAG / "p1344-source-verifier-r1.py"
R1_SHA256 = "83a0021d504c22ddf1591210015c7d451162789b1dd0256e0299d78e9e7fc9ea"
EXPECTED_CORPUS_SHA256 = "1246920352da5678603dadcd07efd2f1e2364e2e397d2d26fdd36b8b7ef757e5"
ADVERSARY_PINS = {
    "runner": (DIAG / "p1344-adversary-runner-r1.py", "db23812f7bd90b232b7bd3f63de6ea66118b84ba7a02ff50d9863360b905d3fe"),
    "report": (DIAG / "p1344-adversary-report-r1.json", "c0705aa3175c80866e52546acb8fb7a22ab0df088a57ae18d13112275ae18d09"),
    "report_md": (DIAG / "p1344-adversary-report-r1.md", "5e5f07688f071b865582f40824e8866951e4ef871f9d13fd73a779dbd961902d"),
    "receipt": (DIAG / "p1344-adversary-receipt-r1.json", "430fd1ba9bdae3adcac49e3b775c4540c2ad106627c2172c1008d34afdc982b9"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_module(path: Path, name: str, expected: str) -> Any:
    if sha256(path.read_bytes()) != expected:
        raise RuntimeError(f"PROTECTED_INPUT: {path.name} hash mismatch")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"PROTECTED_INPUT: cannot import {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


R1 = load_module(R1_PATH, "p1344_source_verifier_r1_for_r2", R1_SHA256)
VerificationFailure = R1.VerificationFailure
fail = R1.fail
canonical_json = R1.canonical_json


DIRECT_ARGUMENTS = {
    "P1343-H00D-DISCOVERY": ["index", "introspector", "location"],
    "P1343-H00S-SELECTED": ["index", "input", "location"],
    "P1343-H01-ATTEMPT-OPEN": ["index", "input", "location", "attempt_kind"],
    "P1343-H02-CONTEXT-DISPATCH": ["node", "span", "carrier"],
    "P1343-H03-ATTEMPT-RESULT": ["result", "selected", "incorporation"],
    "P1343-H05-DICT-PRODUCTION": ["map", "span"],
    "P1343-H11-OCCURRENCE-WALKED": ["loc", "action"],
    "P1343-H12A-REPLAY-ENTER": ["ctx", "location", "pre_state"],
    "P1343-H12B-REPLAY-EXIT": ["location", "pre_state", "post_state"],
    "P1343-H13-FUNC-DISPATCH": ["ctx"],
    "P1343-H14-WITH-EDGE": ["outer", "args", "ctx"],
    "P1343-H16-RAW-FREEZE": [],
}
EVAL_METHODS = [
    "p1343_install_carrier", "p1343_carrier", "p1343_counter_update_observed",
    "p1343_observe_h02", "p1343_observe_h03", "p1343_observe_h04",
    "p1343_observe_h05",
]
FUNC_METHODS = [name.split("::", 1)[1] for name in R1.load_protected()["binding"]["owner_helper_binding"]["func"]]


def check_adversary_pins() -> None:
    for label, (path, expected) in ADVERSARY_PINS.items():
        if sha256(path.read_bytes()) != expected:
            fail("PROTECTED_INPUT", f"adversary {label} drift")


def effective_manifest() -> tuple[dict[str, Any], dict[str, Any]]:
    protected = R1.load_protected()
    return protected, R1.compose_manifest(protected["p1343_capsule_baseline"], protected["capsule_baseline"])


def body_map(candidate_root: Path, manifest: dict[str, Any]) -> dict[str, bytes]:
    result: dict[str, bytes] = {}
    for file_item in manifest["files"]:
        path = file_item["path"]
        raw = R1.open_source(candidate_root, path)
        regions = R1.R1.pair_markers(R1.R1.lexical_markers(raw, path), set(file_item["capsule_ids"]), path)
        for region in regions:
            result[region.capsule_id] = raw[region.body_start:region.body_end]
    return result


def match_pair(tokens: list[str], opening: int, left: str, right: str) -> int:
    if opening >= len(tokens) or tokens[opening] != left:
        fail("CFG_GRAMMAR", f"expected {left}")
    depth = 1
    cursor = opening + 1
    while cursor < len(tokens) and depth:
        if tokens[cursor] == left: depth += 1
        elif tokens[cursor] == right: depth -= 1
        cursor += 1
    if depth:
        fail("CFG_GRAMMAR", f"unclosed {left}")
    return cursor - 1


def methods_from_body(body: bytes, capsule_id: str) -> list[dict[str, Any]]:
    tokens = R1.rust_tokens(body)
    methods = R1.split_top_level_methods(tokens, capsule_id)
    output = []
    for name, method in methods:
        fn_at = method.index("fn")
        params_start = method.index("(", fn_at + 2)
        params_end = match_pair(method, params_start, "(", ")")
        body_start = method.index("{", params_end + 1)
        body_end = match_pair(method, body_start, "{", "}")
        returns = method[params_end + 1:body_start]
        output.append({"name": name, "params": method[params_start + 1:params_end], "returns": returns, "body": method[body_start + 1:body_end]})
    return output


def require_exact_method(method: dict[str, Any], name: str, params: list[str], returns: list[str], body: list[str]) -> None:
    if method["name"] != name or method["params"] != params or method["returns"] != returns or method["body"] != body:
        fail("OWNER_LOCAL", f"{name}: signature, return or effect expression differs from the closed production")


def validate_owner_helpers(bodies: dict[str, bytes]) -> None:
    content = methods_from_body(bodies["P1344-CONTENT-CARRIER-HELPERS"], "P1344-CONTENT-CARRIER-HELPERS")
    if len(content) != 1:
        fail("OWNER_LOCAL", "Content helper cardinality must be one")
    require_exact_method(
        content[0], "p1343_observe_h11", ["&", "self"], ["->", "Option", "<", "P1343Carrier", ">"],
        ["match", "self", "{", "Content", "::", "CounterUpdate", "(", "elem", ")", "=>", "elem", ".", "action", ".", "p1343_observe_h10", "(", ")", ",", "_", "=>", "None", ",", "}"],
    )
    counter = methods_from_body(bodies["P1344-COUNTER-UPDATE-CARRIER-HELPERS"], "P1344-COUNTER-UPDATE-CARRIER-HELPERS")
    if len(counter) != 1:
        fail("OWNER_LOCAL", "CounterUpdate helper cardinality must be one")
    require_exact_method(
        counter[0], "p1343_observe_h10", ["&", "self"], ["->", "Option", "<", "P1343Carrier", ">"],
        ["match", "self", "{", "CounterUpdate", "::", "Func", "(", "function", ")", "=>", "function", ".", "p1343_carrier", "(", ")", ",", "CounterUpdate", "::", "Set", "(", "_", ")", "|", "CounterUpdate", "::", "Step", "(", "_", ")", "=>", "None", ",", "}"],
    )


def validate_func_helpers(body: bytes) -> None:
    methods = methods_from_body(body, "P1343-FUNC-CARRIER-HELPERS")
    if [item["name"] for item in methods] != FUNC_METHODS:
        fail("OWNER_LOCAL", "Func helper method set/order differs from closed binding")
    forbidden = {"loop", "while", "for", "Vec", "HashMap", "pop", "clear", "retain", "drain", "insert", "extend", "push", "get_mut", "unsafe", "static", "macro_rules"}
    for method in methods:
        if set(method["body"]) & forbidden or "!" in method["body"]:
            fail("CARRIER_SKELETON", f"Func::{method['name']} contains lateral control/storage/mutation")
        receiver = method["params"][:3] if method["params"][:2] == ["&", "mut"] else method["params"][:2]
        if receiver not in (["&", "self"], ["&", "mut", "self"]):
            fail("OWNER_LOCAL", f"Func::{method['name']} lacks an exact self receiver")
    carrier = next(item for item in methods if item["name"] == "p1343_carrier")
    if carrier["params"] != ["&", "self"] or carrier["returns"] != ["->", "Option", "<", "P1343Carrier", ">"] or carrier["body"] != ["self", ".", "2", ".", "clone", "(", ")"]:
        fail("CARRIER_SKELETON", "Func::p1343_carrier is not the exact read-only carrier projection")
    attach = next(item for item in methods if item["name"] == "p1343_attach_carrier")
    if attach["params"] != ["&", "mut", "self", ",", "carrier", ":", "P1343Carrier"] or attach["returns"] or attach["body"] != ["self", ".", "2", "=", "Some", "(", "carrier", ")", ";"]:
        fail("CARRIER_SKELETON", "Func::p1343_attach_carrier is not the exact slot assignment")
    for method in methods[2:]:
        if method["params"] != ["&", "self", ",", "event", ":", "P1343RawEvent"] or method["returns"]:
            fail("OWNER_LOCAL", f"Func::{method['name']} observation signature differs")
        if method["body"].count("append") != 1 or method["body"].count("event") != 1 or method["body"].count("p1343_carrier") != 1:
            fail("CARRIER_SKELETON", f"Func::{method['name']} must read one carrier and append the one supplied event")


def impl_method_names(body: bytes, owner: str) -> list[str]:
    tokens = R1.rust_tokens(body)
    try:
        impl_at = tokens.index("impl")
        if tokens[impl_at + 1] != owner:
            fail("CFG_GRAMMAR", f"expected impl {owner}")
        opening = tokens.index("{", impl_at + 2)
        closing = match_pair(tokens, opening, "{", "}")
    except (ValueError, IndexError):
        fail("CFG_GRAMMAR", f"missing closed impl {owner}")
    inner = tokens[opening + 1:closing]
    names = [inner[index + 1] for index, token in enumerate(inner[:-1]) if token == "fn"]
    return names


def validate_eval_helpers(body: bytes) -> None:
    names = impl_method_names(body, "EvalContext")
    if names != EVAL_METHODS:
        fail("CFG_GRAMMAR", f"EvalContext helper set/order {names!r} differs from exact seven")
    tokens = R1.rust_tokens(body)
    if set(tokens) & {"loop", "while", "for", "static", "unsafe", "macro_rules"} or "!" in tokens:
        fail("CFG_GRAMMAR", "EvalContext helper contains lateral control/container")


def validate_direct_hooks(bodies: dict[str, bytes], protected: dict[str, Any]) -> None:
    syntax = {item["capsule_id"]: item for item in protected["p1343_binding_final"]["capsule_syntax_table"]}
    for capsule_id, names in DIRECT_ARGUMENTS.items():
        tokens = R1.rust_tokens(bodies[capsule_id])
        cfg_end = tokens.index("]") + 1
        tail = tokens[cfg_end:]
        callee = syntax[capsule_id]["canonical_callee"].split(".")
        prefix = [callee[0], ".", callee[1], "("]
        if tail[:4] != prefix or tail[-2:] != [")", ";"]:
            fail("CFG_GRAMMAR", f"{capsule_id}: direct callee production differs")
        args = tail[4:-2]
        expected: list[str] = []
        for index, name in enumerate(names):
            if index: expected.append(",")
            expected.append(name)
        if args != expected:
            fail("CFG_GRAMMAR", f"{capsule_id}: exact {len(names)}-argument atom list differs")


def struct_names(tokens: list[str]) -> list[str]:
    return [tokens[index + 1] for index, token in enumerate(tokens[:-1]) if token == "struct"]


def validate_carrier(body: bytes) -> None:
    tokens = R1.rust_tokens(body)
    expected_structs = ["P1343RawEvent", "P1343AppendReceipt", "P1343LedgerState", "P1343Carrier", "P1343RawSnapshot", "P1343Projection"]
    if struct_names(tokens) != expected_structs or tokens.count("enum") != 1:
        fail("CARRIER_SKELETON", "carrier type/item inventory differs from one enum plus six exact structs")
    methods = [tokens[index + 1] for index, token in enumerate(tokens[:-1]) if token == "fn"]
    if methods != ["new", "append", "snapshot", "project"]:
        fail("CARRIER_SKELETON", f"storage API method set/order {methods!r} is not exact")
    forbidden = {"pop", "clear", "retain", "drain", "splice", "extend", "insert", "get_mut", "swap", "replace", "take", "unsafe", "static", "macro_rules"}
    if set(tokens) & forbidden or "!" in tokens:
        fail("CARRIER_SKELETON", "carrier contains unlisted destructive/escape production")
    project_at = tokens.index("project")
    project_open = tokens.index("(", project_at)
    project_close = match_pair(tokens, project_open, "(", ")")
    if tokens[project_open + 1:project_close] != ["&", "self"]:
        fail("CARRIER_SKELETON", "projection receiver must be exactly &self")
    # Four and only four Vec-backed storage fields: ledger events/receipts and
    # immutable snapshot events/receipts. Constructor Vec::new calls are not
    # field declarations and therefore do not contribute.
    field_vecs = sum(tokens[index:index + 3] == [":", "Vec", "<"] for index in range(len(tokens) - 2))
    if field_vecs != 4:
        fail("CARRIER_SKELETON", f"expected four closed Vec storage fields, found {field_vecs}")
    if tokens.count("push") != 2:
        fail("CARRIER_SKELETON", "append must own exactly event+receipt push and no other writer")


def validate_facade(body: bytes) -> None:
    tokens = R1.rust_tokens(body)
    fn_at = tokens.index("fn")
    if tokens[fn_at + 1] != "p1342_run_fixture_for_test":
        fail("CFG_GRAMMAR", "facade symbol differs")
    start = tokens.index("(", fn_at + 2)
    end = match_pair(tokens, start, "(", ")")
    params = tokens[start + 1:end]
    expected_params = ["world", ":", "&", "World", ",", "source", ":", "Source", ",", "challenge", ":", "[", "u8", ";", "32", "]", ",", "mode", ":", "P1343Mode"]
    if params != expected_params:
        fail("CFG_GRAMMAR", "facade must accept exactly world/source/challenge/mode")
    call = ["run_context_stabilization_once", "(", "world", ",", "source", ",", "challenge", ",", "mode", ")"]
    occurrences = sum(tokens[index:index + len(call)] == call for index in range(len(tokens) - len(call) + 1))
    if occurrences != 1:
        fail("CFG_GRAMMAR", "facade must forward the exact four inputs once")
    if set(tokens) & {"if", "match", "loop", "while", "for", "clone", "unsafe", "macro_rules", "other_world"} or "!" in tokens:
        fail("CFG_GRAMMAR", "facade contains substitution/control/creation")


def validate_replacement_controls(bodies: dict[str, bytes], manifest: dict[str, Any]) -> None:
    by_id = {item["capsule_id"]: item for item in manifest["capsules"]}
    for capsule_id, capsule in by_id.items():
        if capsule["kind"] != "replace":
            continue
        replacement = base64.b64decode(capsule["baseline_replacement"]["bytes_base64"], validate=True)
        body = bodies[capsule_id]
        # Remove the exact normal payload before checking the authored delta;
        # product baseline control flow is protected rather than reclassified.
        delta = body.replace(replacement, b"", 1)
        tokens = R1.rust_tokens(delta)
        if set(tokens) & {"loop", "while", "for", "unsafe", "macro_rules"}:
            fail("CFG_GRAMMAR", f"{capsule_id}: observed delta contains lateral/nonterminating control")


def verify_candidate(candidate_root: Path) -> dict[str, Any]:
    check_adversary_pins()
    r1_evidence = R1.verify_candidate(candidate_root)
    protected, manifest = effective_manifest()
    root = candidate_root.resolve(strict=True)
    bodies = body_map(root, manifest)
    if len(bodies) != 38:
        fail("SCHEMA", "R2 body map is not 38 capsules")
    validate_owner_helpers(bodies)
    validate_func_helpers(bodies["P1343-FUNC-CARRIER-HELPERS"])
    validate_eval_helpers(bodies["P1343-EVAL-CARRIER-HELPERS"])
    validate_direct_hooks(bodies, protected)
    validate_carrier(bodies["P1343-FUNC-CARRIER-TYPES"])
    validate_facade(bodies["P1343-TEST-FACADE"])
    validate_replacement_controls(bodies, manifest)
    return {
        "schema": "p1344-candidate-source-evidence-r2",
        "step": 1344,
        "revision": 2,
        "classification": "Preserved",
        "source_only": True,
        "runtime_claim": "NOT_EVALUATED_REQUIRES_EXTERNAL_COMPILED_P1344_RECEIPT",
        "r1_evidence_sha256": sha256(canonical_json(r1_evidence)),
        "corpus_sha256": EXPECTED_CORPUS_SHA256,
        "adversary_pins": {label: expected for label, (_, expected) in ADVERSARY_PINS.items()},
        "closed_productions": {
            "owner_helpers": 3,
            "func_methods": len(FUNC_METHODS),
            "eval_methods": len(EVAL_METHODS),
            "direct_hooks": len(DIRECT_ARGUMENTS),
            "carrier_storage_fields": 4,
            "carrier_methods": 4,
            "facade_forwarded_args": 4,
            "replacement_capsules": 14,
            "result": "PASS"
        },
        "verifier_sha256": sha256(Path(__file__).read_bytes()),
    }


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    value.add_argument("--candidate-root", required=True, type=Path)
    value.add_argument("--output", required=True, type=Path)
    value.add_argument("--runtime-evidence", type=Path)
    return value


def main() -> int:
    args = parser().parse_args()
    if args.runtime_evidence is not None:
        fail("RUNTIME_AUTHORITY", "source verifier never accepts runtime evidence")
    args.output.write_bytes(canonical_json(verify_candidate(args.candidate_root)))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (VerificationFailure, R1.R1.VerificationFailure) as exc:
        print(json.dumps({"schema": "p1344-source-verifier-error-r2", "classification": "Violated", "reason_code": exc.code, "witness": exc.detail}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
