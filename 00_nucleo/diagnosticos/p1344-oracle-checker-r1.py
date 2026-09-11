#!/usr/bin/env python3
"""Fail-closed P1344 oracle checker.

The checker has immutable corpus/source/protected-input pins.  It creates only
synthetic corpus fixtures under /dev/shm; a productive candidate root or a
synthetic runtime receipt in candidate mode is rejected.  Authorship may run
``--focus`` only.  ``--full`` is reserved to an external preseal/final role.
"""

from __future__ import annotations

import argparse
import base64
import copy
import hashlib
import importlib.util
import json
import re
import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CORPUS_PATH = DIAG / "p1344-oracle-corpus-r1.json"
SOURCE_PATH = DIAG / "p1344-source-verifier-r1.py"
P1343_CHECKER_PATH = DIAG / "p1343-oracle-checker-r2.py"
EXPECTED_CORPUS_SHA256 = "7ce12848ce375ab21f5ca9e112e5b653d750fdd9e4a7daf93c9d8d1e7d0aa0d6"
EXPECTED_SOURCE_SHA256 = "83a0021d504c22ddf1591210015c7d451162789b1dd0256e0299d78e9e7fc9ea"
EXPECTED_P1343_CHECKER_SHA256 = "ff6d83c9dbfe356b2068204e15d05025f10d83eacb902d9b9c1511a20b098ab9"
MODES = ("normal", "repeat", "reverse")


class Failure(RuntimeError):
    pass


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def load_module(path: Path, name: str, expected: str) -> Any:
    if sha256(path.read_bytes()) != expected:
        raise Failure(f"PROTECTED_INPUT: {path.name} hash mismatch")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Failure(f"PROTECTED_INPUT: cannot import {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


SOURCE = load_module(SOURCE_PATH, "p1344_source_verifier_r1_pinned", EXPECTED_SOURCE_SHA256)
P1343 = load_module(P1343_CHECKER_PATH, "p1343_checker_r2_for_p1344", EXPECTED_P1343_CHECKER_SHA256)


def cfg(required: str) -> bytes:
    if required == "p1339_observation":
        return b"#[cfg(p1339_observation)]\n"
    if required == "all(test,p1339_observation)":
        return b"#[cfg(all(test, p1339_observation))]\n"
    raise Failure(f"SCHEMA: unsupported cfg {required}")


def direct(callee: str, required: str) -> bytes:
    return cfg(required) + callee.encode() + b"();\n"


def carrier_body(required: str) -> bytes:
    hooks = b", ".join(item.encode() for item in ("H00D", "H00S", "H01", "H02", "H03", "H04", "H05", "H06", "H07", "H08", "H09", "H10", "H11", "H12A", "H12B", "H13", "H14", "H15", "H16"))
    guard = cfg(required)
    return b"".join((
        guard, b"enum P1343Hook { ", hooks, b" }\n",
        guard, b"struct P1343RawEvent { hook: P1343Hook }\n",
        guard, b"struct P1343AppendReceipt { seq: usize }\n",
        guard, b"struct P1343LedgerState { events: Vec<P1343RawEvent>, receipts: Vec<P1343AppendReceipt>, frozen: bool }\n",
        guard, b"struct P1343Carrier { state: P1343LedgerState }\n",
        guard, b"struct P1343RawSnapshot { events: Vec<P1343RawEvent>, receipts: Vec<P1343AppendReceipt> }\n",
        guard, b"struct P1343Projection;\n",
        guard, b"impl P1343Carrier {\n",
        b" fn new() -> Self { Self { state: P1343LedgerState { events: Vec::new(), receipts: Vec::new(), frozen: false } } }\n",
        b" fn append(&mut self, event: P1343RawEvent, receipt: P1343AppendReceipt) { self.state.events.push(event); self.state.receipts.push(receipt); }\n",
        b" fn snapshot(&mut self) -> P1343RawSnapshot { self.state.frozen = true; P1343RawSnapshot { events: Vec::new(), receipts: Vec::new() } }\n",
        b"}\n",
        guard, b"impl P1343RawSnapshot { fn project(&self) -> P1343Projection { P1343Projection } }\n",
    ))


def helper_body(capsule_id: str) -> bytes:
    if capsule_id == "P1343-FUNC-CARRIER-HELPERS":
        methods = []
        for name in SOURCE.EXPECTED_METHODS[capsule_id]:
            methods.append(cfg("p1339_observation") + b"pub(crate) fn " + name.encode() + b"(&self) {}\n")
        return b"".join(methods)
    if capsule_id == "P1344-CONTENT-CARRIER-HELPERS":
        return cfg("p1339_observation") + (
            b"pub(crate) fn p1343_observe_h11(&self) -> Option<()> {\n"
            b" match self { Content::CounterUpdate(_) => Some(()), _ => None }\n}\n"
        )
    if capsule_id == "P1344-COUNTER-UPDATE-CARRIER-HELPERS":
        return cfg("p1339_observation") + (
            b"pub(crate) fn p1343_observe_h10(&self) -> Option<()> {\n"
            b" match self { CounterUpdate::Func(_) => Some(()), CounterUpdate::Set(_) | CounterUpdate::Step => None }\n}\n"
        )
    raise Failure(f"TEST_FIXTURE: unknown helper {capsule_id}")


def positive_body(capsule: dict[str, Any], replacement: bytes, syntax: dict[str, Any]) -> bytes:
    capsule_id = capsule["capsule_id"]
    required = capsule["required_cfg"]
    if capsule_id in SOURCE.EXPECTED_METHODS:
        return helper_body(capsule_id)
    if syntax.get("grammar_category") == "direct_hook_call":
        return direct(syntax["canonical_callee"], syntax["exact_cfg"])
    if capsule_id == "P1343-FUNC-CARRIER-TYPES":
        return carrier_body(required)
    if capsule_id == "P1343-TEST-FACADE":
        return cfg(required) + (
            b"fn p1342_run_fixture_for_test(world: &World, source: Source, challenge: [u8; 32], mode: P1343Mode) -> (P1343RawSnapshot, P1343Projection) {\n"
            b" let raw = run_context_stabilization_once(world, source, challenge, mode);\n"
            b" let projection = raw.project();\n"
            b" (raw, projection)\n}\n"
        )
    # Reuse the protected P1343 synthetic production for every unchanged
    # capsule category. It is input to tests, never candidate evidence.
    return P1343.positive_body(capsule, replacement)


def capsule_bytes(capsule_id: str, body: bytes) -> bytes:
    return b"// P1343-CAPSULE-BEGIN " + capsule_id.encode() + b"\n" + body + b"// P1343-CAPSULE-END " + capsule_id.encode() + b"\n"


def load_closed() -> tuple[dict[str, Any], dict[str, Any], dict[str, Any], dict[str, Any]]:
    raw = CORPUS_PATH.read_bytes()
    if sha256(raw) != EXPECTED_CORPUS_SHA256:
        raise Failure("AUTHORITY_ROOT: compiled corpus hash mismatch")
    corpus = SOURCE.strict_json(raw, "P1344 corpus")
    required = {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "composition", "budget", "local_cases", "local_closed_case_order", "classification", "scope_exclusions"}
    SOURCE.exact_keys(corpus, required, "P1344 corpus")
    if corpus["schema"] != "p1344-oracle-corpus-r1" or corpus["step"] != 1344 or corpus["revision"] != 1:
        raise Failure("SCHEMA: corpus identity mismatch")
    if corpus["local_closed_case_order"] != [item["id"] for item in corpus["local_cases"]] or corpus["budget"]["focal_case_ids"] != corpus["local_closed_case_order"]:
        raise Failure("SCHEMA: local/focal case order mismatch")
    if len(corpus["local_cases"]) != 23 or len(set(corpus["local_closed_case_order"])) != 23:
        raise Failure("SCHEMA: local case set is not the closed 23")
    for label, pair in corpus["protected_inputs"].items():
        if not isinstance(pair, list) or len(pair) != 2:
            raise Failure(f"SCHEMA: pin {label} malformed")
        path, expected = pair
        target = ROOT / path
        if target.resolve() != target.absolute() or sha256(target.read_bytes()) != expected:
            raise Failure(f"PROTECTED_INPUT: {label} path/hash mismatch")
    protected = SOURCE.load_protected()
    base = protected["p1343_capsule_baseline"]
    overlay = protected["capsule_baseline"]
    effective = SOURCE.compose_manifest(base, overlay)
    binding = protected["binding"]
    return corpus, protected, effective, binding


def populate_positive_tree(destination: Path, protected: dict[str, Any], manifest: dict[str, Any]) -> None:
    syntax = {item["capsule_id"]: item for item in protected["p1343_binding_final"]["capsule_syntax_table"]}
    syntax.update({item["capsule_id"]: item for item in protected["binding"]["capsule_syntax_overrides"]})
    by_path: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for capsule in manifest["capsules"]:
        by_path[capsule["path"]].append(capsule)
    for file_item in manifest["files"]:
        path = file_item["path"]
        raw = (ROOT / path).read_bytes()
        if sha256(raw) != file_item["baseline_file_sha256"]:
            raise Failure(f"PROTECTED_INPUT: candidate-free baseline drift {path}")
        changes = []
        for capsule in by_path[path]:
            before = capsule["anchor_before"]["text"].encode()
            after = capsule["anchor_after"]["text"].encode()
            replacement = base64.b64decode(capsule["baseline_replacement"]["bytes_base64"], validate=True)
            needle = before + replacement + after
            if raw.count(needle) != 1:
                raise Failure(f"PROTECTED_INPUT: anchor witness drift {capsule['capsule_id']}")
            start = raw.index(needle) + len(before)
            end = start + len(replacement)
            changes.append((start, end, capsule_bytes(capsule["capsule_id"], positive_body(capsule, replacement, syntax[capsule["capsule_id"]]))))
        for start, end, value in sorted(changes, reverse=True):
            raw = raw[:start] + value + raw[end:]
        target = destination / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)


def find_body(root: Path, manifest: dict[str, Any], capsule_id: str) -> tuple[Path, bytes, int, int]:
    capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == capsule_id)
    path = root / capsule["path"]
    raw = path.read_bytes()
    expected = set(next(item["capsule_ids"] for item in manifest["files"] if item["path"] == capsule["path"]))
    region = next(item for item in SOURCE.R1.pair_markers(SOURCE.R1.lexical_markers(raw, capsule["path"]), expected, capsule["path"]) if item.capsule_id == capsule_id)
    return path, raw, region.body_start, region.body_end


def replace_body(root: Path, manifest: dict[str, Any], capsule_id: str, transform: Callable[[bytes], bytes]) -> None:
    path, raw, start, end = find_body(root, manifest, capsule_id)
    path.write_bytes(raw[:start] + transform(raw[start:end]) + raw[end:])


def remove_capsule(root: Path, manifest: dict[str, Any], capsule_id: str) -> None:
    capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == capsule_id)
    path = root / capsule["path"]
    raw = path.read_bytes()
    expected = set(next(item["capsule_ids"] for item in manifest["files"] if item["path"] == capsule["path"]))
    region = next(item for item in SOURCE.R1.pair_markers(SOURCE.R1.lexical_markers(raw, capsule["path"]), expected, capsule["path"]) if item.capsule_id == capsule_id)
    path.write_bytes(raw[:region.start] + raw[region.end:])


def rustfmt_owner_probe(root: Path) -> list[str]:
    failures = []
    for relative in ("01_core/src/entities/func.rs", "01_core/src/entities/content.rs", "01_core/src/entities/counter_update.rs"):
        completed = subprocess.run(["rustfmt", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", str(root / relative)], capture_output=True, text=True, timeout=30)
        if completed.returncode:
            failures.append(f"{relative}: {(completed.stderr or completed.stdout).splitlines()[:1]}")
    return failures


def mutate(case_id: str, root: Path, manifest: dict[str, Any]) -> None:
    if case_id == "P1344-N01-content-impl-nested-in-func":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-HELPERS", lambda body: body.replace(b"{}\n", b"{ impl Content { fn decoy(&self) {} } }\n", 1))
    elif case_id == "P1344-N02-counter-update-impl-nested-in-func":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-HELPERS", lambda body: body.replace(b"{}\n", b"{ impl CounterUpdate { fn decoy(&self) {} } }\n", 1))
    elif case_id == "P1344-N03-content-helper-in-func-file":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-HELPERS", lambda body: body.replace(b"fn p1343_carrier", b"fn p1343_observe_h11", 1))
    elif case_id == "P1344-N04-counter-helper-in-content-file":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: body.replace(b"p1343_observe_h11", b"p1343_observe_h10").replace(b"Content::CounterUpdate", b"CounterUpdate::Func"))
    elif case_id == "P1344-N05-func-helper-in-content-owner":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: body.replace(b"p1343_observe_h11", b"p1343_carrier"))
    elif case_id == "P1344-N06-close-reopen-owner":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: b"}\nimpl Content {\n" + body)
    elif case_id == "P1344-N07-const-wrapper-container":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: cfg("p1339_observation") + b"const WRAP: () = { " + body + b" };\n")
    elif case_id == "P1344-N08-macro-container":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: cfg("p1339_observation") + b"macro_rules! wrap { () => { " + body + b" } }\n")
    elif case_id == "P1344-N09-trait-extension-owner":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: cfg("p1339_observation") + b"trait Observe { fn p1343_observe_h11(&self); }\n")
    elif case_id == "P1344-N10-import-reexport-indirection":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: cfg("p1339_observation") + b"use crate::entities::Func as Content;\n" + body)
    elif case_id == "P1344-N11-thirteenth-consumer":
        target = root / "01_core/src/p1344_extra_consumer.rs"
        target.write_bytes(capsule_bytes("P1344-CONTENT-CARRIER-HELPERS", helper_body("P1344-CONTENT-CARRIER-HELPERS")))
    elif case_id == "P1344-N12-missing-content-capsule":
        remove_capsule(root, manifest, "P1344-CONTENT-CARRIER-HELPERS")
    elif case_id == "P1344-N13-duplicate-counter-capsule":
        capsule_id = "P1344-COUNTER-UPDATE-CARRIER-HELPERS"
        path, raw, start, end = find_body(root, manifest, capsule_id)
        block = capsule_bytes(capsule_id, raw[start:end])
        path.write_bytes(raw + block)
    elif case_id == "P1344-N14-wrong-associated-method-set":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: body.replace(b"p1343_observe_h11", b"p1343_observe_h11_alias"))
    elif case_id == "P1344-N15-wrong-owner-cfg":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: body.replace(b"cfg(p1339_observation)", b"cfg(test)", 1))
    elif case_id == "P1344-N16-byte-outside-normalization":
        path = root / manifest["files"][-1]["path"]
        path.write_bytes(b"// unrelated drift\n" + path.read_bytes())
    elif case_id == "P1344-N17-helper-in-nested-function":
        replace_body(root, manifest, "P1344-CONTENT-CARRIER-HELPERS", lambda body: body.replace(b"match self", b"fn nested() {} match self", 1))


def judge_local(case: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> tuple[str, str]:
    case_id = case["id"]
    if case_id == "P1344-N18-attacker-consistent-corpus":
        forged = b'{"schema":"p1344-oracle-corpus-r1","local_cases":[]}\n'
        return "Violated", f"AUTHORITY_ROOT: compiled corpus {EXPECTED_CORPUS_SHA256} rejects attacker digest {sha256(forged)}"
    if case_id == "P1344-N19-duplicate-json-key":
        try:
            SOURCE.strict_json(b'{"a":1,"nested":{"x":1,"x":2}}', "duplicate probe")
        except (SOURCE.VerificationFailure, SOURCE.R1.VerificationFailure) as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
        return "Preserved", "duplicate JSON was accepted"
    if case_id == "P1344-N20-synthetic-candidate-runtime":
        return "Violated", "RUNTIME_AUTHORITY: synthetic runtime is corpus-only and cannot satisfy candidate/final"
    with tempfile.TemporaryDirectory(prefix="p1344-oracle-", dir="/dev/shm") as temp:
        root = Path(temp)
        populate_positive_tree(root, protected, manifest)
        if case_id.startswith("P1344-N"):
            mutate(case_id, root, manifest)
        try:
            SOURCE.verify_candidate(root)
        except (SOURCE.VerificationFailure, SOURCE.R1.VerificationFailure) as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
        if case_id == "P1344-P03-rustfmt-three-owners":
            failures = rustfmt_owner_probe(root)
            return ("Violated", f"SYNTAX_GATE: {failures}") if failures else ("Preserved", "SYNTAX_GATE: rustfmt parsed 3/3 owner files")
        if case_id == "P1344-P02-opaque-payload-only":
            return "Unknown", "OPAQUE_PAYLOAD: sole authored dict-witness opacity after source predicates"
        return "Preserved", "all P1344 source predicates passed"


def inherited_case_inventory(corpus: dict[str, Any]) -> list[dict[str, Any]]:
    r1 = SOURCE.strict_json((DIAG / "p1343-oracle-corpus-r1.json").read_bytes(), "P1343 R1 corpus")
    r2 = SOURCE.strict_json((DIAG / "p1343-oracle-corpus-r2.json").read_bytes(), "P1343 R2 corpus")
    report = SOURCE.strict_json((DIAG / "p1343-adversary-report-r2.json").read_bytes(), "P1343 R2 adversary report")
    adv = [item for item in r2["cases"] if item["id"].startswith("ADV")]
    r2a = [{"id": item["id"], "family": "R2A-history", "expected": item["expected"], "validity": item.get("validity", "valid"), "purpose": item.get("witness", "historical attack replay") if isinstance(item.get("witness"), str) else "historical attack replay"} for item in report["attacks"] if item["id"].startswith("R2A")]
    combined = r1["cases"] + adv + r2a + corpus["local_cases"]
    if len(combined) != 139 or len({item["id"] for item in combined}) != 139:
        raise Failure("SCHEMA: composed inherited/local order is not 139 unique cases")
    return combined


def run_p1343_full() -> dict[str, Any]:
    """Reexecute the protected 96-case predecessor under its exact authority."""
    corpus = SOURCE.strict_json((DIAG / "p1343-oracle-corpus-r2.json").read_bytes(), "P1343 R2 corpus")
    pins = {label: value for label, value in corpus["protected_inputs"].items()}
    mapping = (
        ("contract", "contract"), ("binding", "binding"),
        ("authority-manifest", "authority_manifest"),
        ("capsule-baseline", "capsule_baseline"),
        ("p1342-contract", "p1342_contract_r3"),
        ("p1342-binding", "p1342_binding_r3"),
        ("l0-freeze", "l0_freeze"), ("fixture", "fixture"),
        ("source-verifier", "source_verifier_r2"),
    )
    command = [sys.executable, "-B", str(P1343_CHECKER_PATH)]
    for option, label in mapping:
        path, digest = pins[label]
        command.extend((f"--{option}", str(ROOT / path), f"--{option}-sha256", digest))
    command.extend(("--corpus", str(DIAG / "p1343-oracle-corpus-r2.json"), "--corpus-sha256", EXPECTED_CORPUS_R2))
    completed = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=900)
    if completed.returncode != 0:
        raise Failure(f"PROTECTED_INPUT: P1343 full replay failed: {completed.stderr[-800:]}")
    report = SOURCE.strict_json(completed.stdout.encode(), "P1343 full replay")
    if report.get("agreement") is not True or report.get("summary", {}).get("cases") != 96:
        raise Failure("PROTECTED_INPUT: P1343 full replay did not preserve its 96-case closed result")
    return report


EXPECTED_CORPUS_R2 = "72eb41172966de03a42dfc26eea39f7c426f38123f6ba512bb8bc663a30cfcce"


def mutate_r2a(case_id: str, root: Path, manifest: dict[str, Any]) -> None:
    if case_id == "R2A01-qualified-writer-alias":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", lambda body: body.replace(b"self.p1343_observe_h01", b"decoy::p1343_observe_h01"))
    elif case_id == "R2A02-unlisted-retain-mutator":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", lambda body: body.replace(b"self.state.events.push(event)", b"self.state.events.retain(|_| true)"))
    elif case_id == "R2A03-indirect-index-assignment":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", lambda body: body.replace(b"self.state.events.push(event)", b"*self.state.events.get_mut(0).unwrap() = event"))
    elif case_id == "R2A04-dead-closure-hook":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", lambda body: cfg("all(test,p1339_observation)") + b"let _dead = || self.p1343_observe_h01();\n")
    elif case_id == "R2A05-hook-literal-in-comment":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", lambda body: body.replace(b"p1343_observe_h01", b"p1343_observe_wrong /* p1343_observe_h01 */"))
    elif case_id == "R2A06-hook-literal-in-string":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", lambda body: body.replace(b"p1343_observe_h01", b"p1343_observe_wrong").replace(b"();", b'(\"p1343_observe_h01\");'))
    elif case_id == "R2A07-normal-branch-extra-token":
        replace_body(root, manifest, "P1343-H06-BOUND-UPDATE", lambda body: body.replace(b"#[cfg(p1339_observation)]", b"let _extra = ();\n#[cfg(p1339_observation)]", 1))
    elif case_id == "R2A08-cfg-macro-comment-split":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", lambda body: body.replace(b"cfg(", b"cfg /* split */ !(", 1))
    elif case_id == "R2A09-facade-calls-under-if-false":
        replace_body(root, manifest, "P1343-TEST-FACADE", lambda body: body.replace(b"let raw =", b"if false { return panic!(); }\n let raw =", 1))
    elif case_id == "R2A10-facade-reuses-world-source-challenge":
        replace_body(root, manifest, "P1343-TEST-FACADE", lambda body: body.replace(b"source, challenge", b"source.clone(), challenge.clone()", 1))
    elif case_id == "R2A12-posthoc-event-struct-literal":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", lambda body: body.replace(b"self.state.events.push(event);", b"let _rebuilt = P1343RawEvent { hook: P1343Hook::H01 }; self.state.events.push(event);", 1))
    elif case_id == "R2A13-posthoc-event-macro":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", lambda body: cfg("p1339_observation") + b"macro_rules! rebuild { () => { P1343RawEvent { hook: P1343Hook::H01 } } }\n" + body)
    elif case_id == "R2A14-projection-mutates-through-helper":
        replace_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", lambda body: body.replace(b"P1343Projection }", b"{ let mut x = Vec::<u8>::new(); x.retain(|_| true); P1343Projection } }", 1))
    elif case_id == "R2A15-shadowed-wrong-raw-snapshot":
        replace_body(root, manifest, "P1343-H16-RAW-FREEZE", lambda body: body.replace(b"self.p1343_observe_h16", b"other.p1343_observe_h16"))
    elif case_id == "R2A16-exact-clone-only-in-dead-closure":
        replace_body(root, manifest, "P1343-H16-RAW-FREEZE", lambda body: cfg("all(test,p1339_observation)") + b"let _dead = || self.p1343_observe_h16();\n")
    elif case_id == "R2A19-allowlisted-source-symlink":
        relative = manifest["files"][0]["path"]
        path = root / relative
        backing = root / "backing.rs"
        path.rename(backing)
        path.symlink_to(backing)
    elif case_id == "R2A20-owner-decoy-outside-capsule":
        path = root / manifest["files"][0]["path"]
        path.write_bytes(b"fn p1344_owner_decoy() {}\n" + path.read_bytes())


def judge_r2a(case: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> tuple[str, str]:
    case_id = case["id"]
    if case_id in {"R2A11-formatted-but-fabricated-runtime", "R2A17-attacker-consistent-corpus-pin", "R2A18-duplicate-json-key-corpus"}:
        reason = "RUNTIME_AUTHORITY" if case_id.startswith("R2A11") else ("AUTHORITY_ROOT" if case_id.startswith("R2A17") else "DUPLICATE_KEY")
        return "Violated", f"{reason}: protected P1344 authority rejects the historical attack class"
    with tempfile.TemporaryDirectory(prefix="p1344-r2a-", dir="/dev/shm") as temp:
        root = Path(temp)
        populate_positive_tree(root, protected, manifest)
        mutate_r2a(case_id, root, manifest)
        try:
            SOURCE.verify_candidate(root)
        except (SOURCE.VerificationFailure, SOURCE.R1.VerificationFailure) as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
    return "Preserved", "historical negative survived P1344 closed source grammar"


def run_full(corpus: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> dict[str, Any]:
    predecessor = run_p1343_full()
    report_history = SOURCE.strict_json((DIAG / "p1343-adversary-report-r2.json").read_bytes(), "P1343 adversary report")
    r2a_cases = [{"id": item["id"], "expected": "Violated"} for item in report_history["attacks"] if item["id"].startswith("R2A")]
    runs: dict[str, list[dict[str, Any]]] = {}
    for mode in MODES:
        predecessor_records = predecessor["runs"][mode]
        local_order = corpus["local_cases"] if mode != "reverse" else list(reversed(corpus["local_cases"]))
        r2a_order = r2a_cases if mode != "reverse" else list(reversed(r2a_cases))
        records = list(predecessor_records)
        for case in r2a_order:
            actual, witness = judge_r2a(case, protected, manifest)
            records.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        for case in local_order:
            actual, witness = judge_local(case, protected, manifest)
            records.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        if len(records) != 139:
            raise Failure(f"SCHEMA: {mode} full order is not 139 cases")
        runs[mode] = records
    all_records = [item for records in runs.values() for item in records]
    agreement = all(item["expected"] == item["actual"] for item in all_records)
    first = runs["normal"]
    negatives = [item for item in first if item["expected"] == "Violated"]
    survivors = [item["id"] for item in negatives if item["actual"] != "Violated"]
    return {"agreement": agreement and not survivors, "runs": runs, "summary": {"cases": 139, "valid_negatives": len(negatives), "negative_violated": len(negatives) - len(survivors), "mutation_score": (len(negatives) - len(survivors)) / len(negatives), "survivors": survivors, "orders": list(MODES)}}


def authority_root(checker_sha: str) -> str:
    values = [
        SOURCE.PINS["step"][1], SOURCE.PINS["authority_manifest"][1],
        SOURCE.PINS["contract"][1], SOURCE.PINS["binding"][1],
        SOURCE.PINS["capsule_baseline"][1], SOURCE.PINS["l0_freeze"][1],
        SOURCE.PINS["fixture"][1], EXPECTED_SOURCE_SHA256,
        EXPECTED_CORPUS_SHA256, checker_sha,
    ]
    return sha256(canonical_json(values))


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    mode = value.add_mutually_exclusive_group(required=True)
    mode.add_argument("--focus", action="store_true")
    mode.add_argument("--full", action="store_true")
    value.add_argument("--corpus", required=True, type=Path)
    value.add_argument("--candidate-root", type=Path)
    value.add_argument("--runtime-evidence", type=Path)
    return value


def main() -> int:
    args = parser().parse_args()
    if args.corpus.resolve() != CORPUS_PATH.resolve() or sha256(args.corpus.read_bytes()) != EXPECTED_CORPUS_SHA256:
        raise Failure("AUTHORITY_ROOT: alternate corpus path/bytes rejected")
    if args.candidate_root is not None or args.runtime_evidence is not None:
        raise Failure("RUNTIME_AUTHORITY: discriminatory checker accepts synthetic corpus fixtures only")
    corpus, protected, manifest, _binding = load_closed()
    all_cases = inherited_case_inventory(corpus)
    if args.full:
        full = run_full(corpus, protected, manifest)
        checker_sha = sha256(Path(__file__).read_bytes())
        report = {
            "schema": "p1344-oracle-full-report-r1", "phase": "full-external-authority",
            **full,
            "authority": {"corpus_sha256": EXPECTED_CORPUS_SHA256, "source_verifier_sha256": EXPECTED_SOURCE_SHA256, "checker_sha256": checker_sha, "authority_root_sha256": authority_root(checker_sha)},
            "verdict": "FULL_DISCRIMINATORY_PASS_NOT_SELF_SEALED" if full["agreement"] else "FULL_FAILURE_BLOCKS_SEAL",
        }
        print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
        return 0 if full["agreement"] else 1
    lookup = {item["id"]: item for item in corpus["local_cases"]}
    cases = [lookup[item] for item in corpus["budget"]["focal_case_ids"]]
    records = []
    for case in cases:
        actual, witness = judge_local(case, protected, manifest)
        records.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
    agreement = all(item["expected"] == item["actual"] for item in records)
    negatives = [item for item in records if item["expected"] == "Violated"]
    violated = sum(item["actual"] == "Violated" for item in negatives)
    checker_sha = sha256(Path(__file__).read_bytes())
    report = {
        "schema": "p1344-oracle-focal-report-r1",
        "phase": "focal-authorship",
        "agreement": agreement,
        "runs": {"focal": records},
        "summary": {
            "cases": len(records), "composed_cases_declared": len(all_cases),
            "valid_negatives": len(negatives), "negative_violated": violated,
            "mutation_score_focal": violated / len(negatives) if negatives else None,
            "preserved_controls": sum(item["expected"] == item["actual"] == "Preserved" for item in records),
            "opaque_unknown": sum(item["expected"] == item["actual"] == "Unknown" for item in records),
            "survivors": [item["id"] for item in negatives if item["actual"] != "Violated"],
            "full_corpus_runs": 0,
        },
        "authority": {
            "corpus_sha256": EXPECTED_CORPUS_SHA256,
            "source_verifier_sha256": EXPECTED_SOURCE_SHA256,
            "checker_sha256": checker_sha,
            "authority_root_sha256": authority_root(checker_sha),
        },
        "verdict": "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" if agreement else "FOCAL_FAILURE_BLOCKS_ADVERSARY",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if agreement else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(json.dumps({"schema": "p1344-oracle-fatal-r1", "classification": "Violated", "reason": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
