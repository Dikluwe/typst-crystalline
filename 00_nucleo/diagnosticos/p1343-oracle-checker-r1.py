#!/usr/bin/env python3
"""Candidate-free discriminatory oracle for the P1343 capsule contract.

P/A/X runtime semantics are composed from the protected P1342 R5 corpus and
its frozen R3 ledger checker.  P1343 source cases are generated from the exact
ten candidate-free files and judged by recomputation through the separately
authored source verifier.  ``--focus`` is the only mode used by this author;
the independent preseal authority owns the sole full-corpus execution.
"""

from __future__ import annotations

import argparse
import base64
import copy
import hashlib
import importlib.util
import json
import shutil
import sys
import tempfile
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
P1342_R5_CHECKER = DIAG / "p1342-oracle-checker-r5.py"
P1342_R5_CORPUS = DIAG / "p1342-oracle-corpus-r5.json"
P1342_R2_CONTRACT = DIAG / "p1342-contract-spec-r2.json"
P1342_R2_BINDING = DIAG / "p1342-contract-binding-r2.json"
MODES = ("normal", "repeat", "reverse")


class OracleFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        super().__init__(f"{code}: {detail}")
        self.code = code
        self.detail = detail


def fail(code: str, detail: str) -> None:
    raise OracleFailure(code, detail)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8")


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        fail("PROTECTED_INPUT", f"cannot load {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_bytes())
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as exc:
        fail("SCHEMA", f"cannot parse {path}: {exc}")


def verify_pin(path: Path, expected: str, label: str) -> bytes:
    try:
        raw = path.read_bytes()
    except OSError as exc:
        fail("PROTECTED_INPUT", f"cannot read {label}: {exc}")
    if sha256(raw) != expected:
        fail("PROTECTED_INPUT", f"{label} hash mismatch")
    return raw


def exact_keys(value: Any, keys: set[str], where: str) -> None:
    if not isinstance(value, dict) or set(value) != keys:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        fail("SCHEMA", f"{where} closed keys mismatch: {got}")


def cfg_line(required: str) -> bytes:
    return b"#[cfg(p1339_observation)]\n" if required == "p1339_observation" else b"#[cfg(all(test, p1339_observation))]\n"


def positive_body(capsule: dict[str, Any], replacement: bytes) -> bytes:
    capsule_id = capsule["capsule_id"]
    observed = cfg_line(capsule["required_cfg"])
    hook = capsule["covers"][0] if capsule["covers"] else capsule_id
    if capsule_id == "P1343-PIPE-SUPPORT":
        payload = (
            b"mod p1343_observation_support {\n"
            b"    fn p1343_append_raw(event: Event) { ledger.push(event); }\n"
            b"    fn p1343_project_dto(raw: &[Event]) -> Dto { project(raw) }\n"
            b"}\n"
        )
    elif capsule_id == "P1343-H16-RAW-FREEZE":
        payload = b'{ let raw_freeze = ledger.clone(); p1343_append_raw("H16"); }\n'
    elif capsule_id == "P1343-TEST-FACADE":
        payload = b"fn p1342_run_fixture_for_test() { run_three_fresh_worlds(); }\n"
    else:
        payload = b'{ p1343_append_raw("' + hook.encode("utf-8") + b'"); }\n'
    if capsule["kind"] == "insert":
        return observed + payload
    return b"#[cfg(not(p1339_observation))]\n" + replacement + observed + payload


def capsule_bytes(capsule: dict[str, Any], body: bytes) -> bytes:
    capsule_id = capsule["capsule_id"].encode("ascii")
    return b"// P1343-CAPSULE-BEGIN " + capsule_id + b"\n" + body + b"// P1343-CAPSULE-END " + capsule_id + b"\n"


def populate_positive_tree(destination: Path, manifest: dict[str, Any], baseline_root: Path) -> None:
    by_path: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for capsule in manifest["capsules"]:
        by_path[capsule["path"]].append(capsule)
    for file_item in manifest["files"]:
        path = file_item["path"]
        raw = (baseline_root / path).read_bytes()
        if sha256(raw) != file_item["baseline_file_sha256"]:
            fail("PROTECTED_INPUT", f"candidate-free source drift for {path}")
        replacements = []
        for capsule in by_path[path]:
            before = capsule["anchor_before"]["text"].encode("utf-8")
            after = capsule["anchor_after"]["text"].encode("utf-8")
            replacement = base64.b64decode(capsule["baseline_replacement"]["bytes_base64"], validate=True)
            needle = before + replacement + after
            if raw.count(needle) != 1:
                fail("PROTECTED_INPUT", f"candidate-free witness drift for {capsule['capsule_id']}")
            start = raw.index(needle) + len(before)
            end = start + len(replacement)
            replacements.append((start, end, capsule_bytes(capsule, positive_body(capsule, replacement))))
        for start, end, value in sorted(replacements, reverse=True):
            raw = raw[:start] + value + raw[end:]
        target = destination / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)


def find_capsule(raw: bytes, capsule_id: str) -> tuple[int, int, int, int]:
    begin = b"// P1343-CAPSULE-BEGIN " + capsule_id.encode("ascii") + b"\n"
    end = b"// P1343-CAPSULE-END " + capsule_id.encode("ascii") + b"\n"
    if raw.count(begin) != 1 or raw.count(end) != 1:
        fail("TEST_FIXTURE", f"cannot uniquely mutate {capsule_id}")
    start = raw.index(begin)
    body_start = start + len(begin)
    body_end = raw.index(end, body_start)
    return start, body_start, body_end, body_end + len(end)


def first_capsule(manifest: dict[str, Any], kind: str | None = None) -> dict[str, Any]:
    return next(c for c in manifest["capsules"] if kind is None or c["kind"] == kind)


def mutate_tree(root: Path, manifest: dict[str, Any], mutation: str) -> None:
    capsule = first_capsule(manifest, "replace" if mutation in {"false-replacement", "normal-altered"} else None)
    capsule_id = capsule["capsule_id"]
    path = root / capsule["path"]
    raw = path.read_bytes()
    start, body_start, body_end, end = find_capsule(raw, capsule_id)
    complete = raw[start:end]
    begin_line = raw[start:body_start]
    end_line = raw[body_end:end]
    if mutation == "candidate-free":
        for file_item in manifest["files"]:
            source = ROOT / file_item["path"]
            (root / file_item["path"]).write_bytes(source.read_bytes())
        return
    if mutation == "missing-id":
        raw = raw[:start] + raw[end:]
    elif mutation == "normal-string-marker":
        decoy = b'const P1343_DECOY: &str = "' + begin_line.rstrip() + b' ' + end_line.rstrip() + b'";\n'
        raw = raw[:start] + decoy + raw[end:]
    elif mutation == "raw-string-marker":
        decoy = b'const P1343_DECOY: &str = r#"\n' + begin_line + end_line + b'"#;\n'
        raw = raw[:start] + decoy + raw[end:]
    elif mutation == "char-marker":
        decoy = b"const P1343_DECOY: &[char] = &['/', '/', 'P', '1', '3', '4', '3'];\n"
        raw = raw[:start] + decoy + raw[end:]
    elif mutation == "block-marker":
        raw = raw[:start] + b"/*\n" + begin_line + end_line + b"*/\n" + raw[end:]
    elif mutation == "macro-marker":
        raw = raw[:start] + b'macro_rules! decoy { () => { concat!("' + begin_line.rstrip() + b'", "' + end_line.rstrip() + b'") } }\n' + raw[end:]
    elif mutation == "nested":
        nested = b"// P1343-CAPSULE-BEGIN " + capsule_id.encode() + b"\n// P1343-CAPSULE-END " + capsule_id.encode() + b"\n"
        raw = raw[:body_start] + nested + raw[body_start:]
    elif mutation == "duplicate":
        raw = raw[:start] + complete + complete + raw[end:]
    elif mutation == "unknown-id":
        changed = complete.replace(capsule_id.encode(), b"P1343-UNKNOWN", 2)
        raw = raw[:start] + changed + raw[end:]
    elif mutation == "mismatched-id":
        changed = complete.replace(b"// P1343-CAPSULE-END " + capsule_id.encode(), b"// P1343-CAPSULE-END P1343-UNKNOWN")
        raw = raw[:start] + changed + raw[end:]
    elif mutation in {"cfg-removed", "normal-altered", "false-replacement", "invalid-rust", "unguarded-tail"}:
        body = raw[body_start:body_end]
        if mutation == "cfg-removed":
            body = body.replace(cfg_line(capsule["required_cfg"]), b"", 1)
        elif mutation in {"normal-altered", "false-replacement"}:
            replacement = base64.b64decode(capsule["baseline_replacement"]["bytes_base64"])
            body = body.replace(replacement, b"/* forged */" + replacement, 1)
        else:
            body += b"fn invalid( {\n" if mutation == "invalid-rust" else b"unguarded_call();\n"
        raw = raw[:body_start] + body + raw[body_end:]
    elif mutation == "outside-byte":
        raw = b"// unauthorized outside capsule\n" + raw
    elif mutation == "anchor-source":
        before = capsule["anchor_before"]["text"].encode()
        anchor_start = start - len(before)
        raw = raw[:anchor_start] + b"X" + raw[anchor_start + 1:]
    elif mutation == "empty-capsule":
        raw = raw[:body_start] + b"" + raw[body_end:]
    else:
        fail("TEST_FIXTURE", f"unknown source mutation {mutation}")
    path.write_bytes(raw)


def p1343_runtime(binding: dict[str, Any], fixture_sha: str) -> dict[str, Any]:
    rows = []
    for row in binding["row_capsule_bindings"]:
        rows.append({
            "row_index": row["row_index"], "hook": row["hook"],
            "normal": f"receipt:normal:{row['row_index']}",
            "repeat": f"receipt:repeat:{row['row_index']}",
            "reverse": f"receipt:reverse:{row['row_index']}",
        })
    return {
        "schema": "p1343-runtime-evidence-v1",
        "fixture_sha256": fixture_sha,
        "modes": list(MODES),
        "row_coverage": rows,
        "raw_snapshot_refs": {mode: f"raw:{mode}:fixture" for mode in MODES},
        "result": "PASS",
    }


def inherited_runtime(case_id: str, inherited: Any, corpus: dict[str, Any]) -> str:
    base_id = case_id if case_id.startswith(("P", "A", "X")) else "P01-inspectable-composite"
    binding = load_json(P1342_R2_BINDING)
    contract = load_json(P1342_R2_CONTRACT)
    old = load_json(P1342_R5_CORPUS)
    pins = {
        "contract": old["protected_inputs"]["contract_r2"][1],
        "binding": old["protected_inputs"]["binding_r2"][1],
        "fixture": old["protected_inputs"]["fixture"][1],
        "l0_freeze": old["protected_inputs"]["l0_freeze"][1],
    }
    dto, evidence, local_binding, evidence_pin = inherited.R3.build_case(base_id, old, binding, pins)
    try:
        return inherited.R3.judge(dto, evidence, evidence_pin, contract, local_binding, pins)
    except inherited.Failure:
        return "Violated"


SOURCE_MUTATIONS = {
    "Y02-live-baseline-equal-hook-absent": "candidate-free",
    "Y06-change-outside-allowed-boundary": "outside-byte",
    "Z01-live-baseline-via-alias-and-decoy": "raw-string-marker",
    "Z02-baseline-equal-with-fabricated-hooks": "candidate-free",
    "Z06-outside-cfg-invalid-rust-self-attested": "invalid-rust",
    "Z07-empty-boundary-over-changed-source": "empty-capsule",
    "C01-marker-normal-string": "normal-string-marker",
    "C02-marker-raw-string": "raw-string-marker",
    "C03-marker-block-comment": "block-marker",
    "C04-marker-macro-decoy": "macro-marker",
    "C05-nested-capsule": "nested",
    "C06-duplicate-capsule": "duplicate",
    "C07-unknown-id": "unknown-id",
    "C08-missing-id": "missing-id",
    "C09-mismatched-id": "mismatched-id",
    "C10-false-replacement": "false-replacement",
    "C11-change-outside": "outside-byte",
    "C12-cfg-removed": "cfg-removed",
    "C13-normal-branch-altered": "normal-altered",
    "C14-anchor-source-altered": "anchor-source",
    "C15-invalid-rust-self-attested": "invalid-rust",
    "C16-empty-decoy-capsule": "empty-capsule",
    "C21-marker-char-byte-char": "char-marker",
    "C22-insert-unguarded-node": "unguarded-tail",
}


EVIDENCE_MUTATIONS = {
    "Y03-forbidden-live-baseline-comparator": "extra-key",
    "Y04-evidence-only-in-dto-facade": "missing-external",
    "Y05-external-evidence-path-hash-adulterated": "external-hash",
    "Z03-candidate-file-sha-altered": "file-hash",
    "Z04-external-evidence-path-swapped": "external-path",
    "Z05-external-evidence-hash-swapped": "external-hash",
    "Z08-preseal-pins-r2": "predecessor-pin",
    "Z09-candidate-bytes-hash-diverge": "file-hash",
    "Z10-missing-rust-aware-row": "missing-row",
    "Z11-missing-runtime-coverage": "runtime-row",
    "Z12-forged-candidate-anchor-hashes": "anchor-symbol",
    "C17-forged-path": "path",
    "C18-forged-anchor-hash": "anchor",
    "C19-forged-symbol-hash": "symbol",
    "C20-forged-result-parse-envelope": "result",
}


def mutate_evidence(evidence: dict[str, Any], mutation: str) -> Any:
    value = copy.deepcopy(evidence)
    if mutation == "extra-key":
        value["live_equals_baseline"] = True
    elif mutation == "missing-external":
        return None
    elif mutation in {"external-hash", "external-path"}:
        # Modeled by a projection mismatch because the trusted invocation owns both.
        value["verifier"]["invocation_sha256"] = "0" * 64
    elif mutation == "file-hash":
        value["files"][0]["live_file_sha256"] = "0" * 64
    elif mutation == "missing-row":
        value["rows"].pop()
    elif mutation == "runtime-row":
        value["runtime_coverage"]["row_coverage"].pop()
    elif mutation == "anchor-symbol":
        value["capsules"][0]["anchor_before_sha256"] = "0" * 64
        value["capsules"][0]["symbol_sha256"] = "f" * 64
    elif mutation == "path":
        value["files"][0]["path"] = "01_core/../forged.rs"
    elif mutation == "anchor":
        value["capsules"][0]["anchor_after_sha256"] = "0" * 64
    elif mutation == "symbol":
        value["capsules"][0]["symbol_sha256"] = "0" * 64
    elif mutation == "result":
        value["verifier"]["result"] = "PASS"
        value["parse_ok"] = True
    elif mutation == "predecessor-pin":
        value["p1342_contract_sha256"] = "0" * 64
    else:
        fail("TEST_FIXTURE", f"unknown evidence mutation {mutation}")
    return value


def judge_case(
    case: dict[str, Any], *, manifest: dict[str, Any], binding: dict[str, Any],
    contract: dict[str, Any], authority_sha: str, manifest_sha: str,
    contract_sha: str, binding_sha: str, p1342_contract_sha: str,
    p1342_binding_sha: str, l0_sha: str, fixture_sha: str,
    source: Any, source_sha: str, inherited: Any, corpus: dict[str, Any],
    candidate_root: Path | None,
) -> tuple[str, str]:
    runtime_result = inherited_runtime(case["id"], inherited, corpus)
    source_mutation = SOURCE_MUTATIONS.get(case["id"])
    evidence_mutation = EVIDENCE_MUTATIONS.get(case["id"])
    with tempfile.TemporaryDirectory(prefix="p1343-oracle-", dir="/dev/shm") as temp:
        root = Path(temp)
        if candidate_root is None:
            populate_positive_tree(root, manifest, ROOT)
        else:
            for file_item in manifest["files"]:
                target = root / file_item["path"]
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(candidate_root / file_item["path"], target)
        if source_mutation:
            mutate_tree(root, manifest, source_mutation)
        runtime = p1343_runtime(binding, fixture_sha)
        try:
            derived = source.derive_source_evidence(
                contract=contract, contract_sha=contract_sha,
                binding=binding, binding_sha=binding_sha,
                authority_sha=authority_sha, manifest=manifest, manifest_sha=manifest_sha,
                p1342_contract_sha=p1342_contract_sha,
                p1342_binding_sha=p1342_binding_sha,
                l0_sha=l0_sha, fixture_sha=fixture_sha,
                candidate_root=root, runtime=runtime,
                source_verifier_sha=source_sha,
                invocation_sha=sha256(b"p1343-independent-oracle-invocation"),
            )
        except source.VerificationFailure as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
        supplied = mutate_evidence(derived, evidence_mutation) if evidence_mutation else copy.deepcopy(derived)
        if supplied is None:
            return "Violated", "PROTECTED_INPUT: source evidence absent from external channel"
        # The checker independently recomputed the projection above.  No evidence flag
        # or hash is authoritative until this byte comparison succeeds.
        if canonical_json(supplied) != canonical_json(derived):
            return "Violated", "SOURCE_RECOMPUTE: supplied evidence differs from byte-derived projection"
        if runtime_result == "Violated":
            return "Violated", "P1342_RAW_CAUSAL: inherited protected runtime predicate failed"
        if runtime_result == "Unknown":
            return "Unknown", "OPAQUE_PAYLOAD: deliberate dict witness after all non-payload predicates"
        return "Preserved", "all source-derived capsule/normalization and inherited runtime predicates passed"


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    for name in ("contract", "binding", "authority-manifest", "capsule-baseline", "p1342-contract", "p1342-binding", "l0-freeze", "fixture", "source-verifier"):
        value.add_argument(f"--{name}", required=True, type=Path)
        value.add_argument(f"--{name}-sha256", required=True)
    value.add_argument("--corpus", required=True, type=Path)
    value.add_argument("--focus", action="store_true")
    value.add_argument("--candidate-root", type=Path)
    return value


def main() -> int:
    args = parser().parse_args()
    if args.candidate_root is not None:
        fail(
            "RUNTIME_COVERAGE",
            "corpus checker cannot accept a productive candidate with synthetic runtime; use the pinned source-verifier CLI with real external runtime evidence and the independent Rust A/B/final gates",
        )
    corpus = load_json(args.corpus)
    exact_keys(corpus, {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "composition", "budget", "cases", "closed_case_order", "scope_exclusions"}, "corpus")
    if corpus["schema"] != "p1343-oracle-corpus-r1" or corpus["revision"] != 1:
        fail("SCHEMA", "corpus identity mismatch")
    for label, pin in corpus["protected_inputs"].items():
        if not isinstance(pin, list) or len(pin) != 2:
            fail("SCHEMA", f"invalid protected pin {label}")
        verify_pin(ROOT / pin[0], pin[1], label)
    supplied = {
        "contract": (args.contract, args.contract_sha256),
        "binding": (args.binding, args.binding_sha256),
        "authority_manifest": (args.authority_manifest, args.authority_manifest_sha256),
        "capsule_baseline": (args.capsule_baseline, args.capsule_baseline_sha256),
        "p1342_contract_r3": (args.p1342_contract, args.p1342_contract_sha256),
        "p1342_binding_r3": (args.p1342_binding, args.p1342_binding_sha256),
        "l0_freeze": (args.l0_freeze, args.l0_freeze_sha256),
        "fixture": (args.fixture, args.fixture_sha256),
        "source_verifier": (args.source_verifier, args.source_verifier_sha256),
    }
    for label, (path, digest) in supplied.items():
        pin = corpus["protected_inputs"][label]
        if path.resolve() != (ROOT / pin[0]).resolve() or digest != pin[1]:
            fail("PROTECTED_INPUT", f"externally supplied {label} path/hash mismatch")
        verify_pin(path, digest, label)
    fixture_raw = args.fixture.read_bytes()
    if len(fixture_raw) != 178 or not fixture_raw.endswith(b"\n"):
        fail("PROTECTED_INPUT", "fixture shape mismatch")
    contract = load_json(args.contract)
    binding = load_json(args.binding)
    manifest = load_json(args.capsule_baseline)
    source = load_module(args.source_verifier, "p1343_source_verifier_r1_frozen")
    inherited = load_module(P1342_R5_CHECKER, "p1342_r5_inherited_by_p1343")
    cases = corpus["cases"]
    if corpus["closed_case_order"] != [case["id"] for case in cases]:
        fail("SCHEMA", "closed case order mismatch")
    for case in cases:
        exact_keys(case, {"id", "family", "expected", "validity", "purpose", "source"}, f"case {case.get('id', '?')}")
        if case["validity"] != "valid" or case["expected"] not in {"Preserved", "Violated", "Unknown"}:
            fail("SCHEMA", "invalid authored case metadata")
    if args.focus:
        wanted = corpus["budget"]["focal_case_ids"]
        by_id = {case["id"]: case for case in cases}
        cases = [by_id[case_id] for case_id in wanted]
        orders = {"focal": cases}
    else:
        orders = {"normal": cases, "repeat": cases, "reverse": list(reversed(cases))}
    outcomes: dict[str, list[str]] = defaultdict(list)
    runs: dict[str, list[dict[str, Any]]] = {}
    for mode, ordered in orders.items():
        results = []
        for case in ordered:
            actual, witness = judge_case(
                case, manifest=manifest, binding=binding, contract=contract,
                authority_sha=args.authority_manifest_sha256,
                manifest_sha=args.capsule_baseline_sha256,
                contract_sha=args.contract_sha256, binding_sha=args.binding_sha256,
                p1342_contract_sha=args.p1342_contract_sha256,
                p1342_binding_sha=args.p1342_binding_sha256,
                l0_sha=args.l0_freeze_sha256, fixture_sha=args.fixture_sha256,
                source=source, source_sha=args.source_verifier_sha256,
                inherited=inherited, corpus=corpus,
                candidate_root=args.candidate_root,
            )
            outcomes[case["id"]].append(actual)
            results.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        runs[mode] = results
    agreement = all(
        all(value == case["expected"] for value in outcomes[case["id"]])
        for case in cases
    ) and all(len(set(values)) == 1 for values in outcomes.values())
    negatives = [case for case in cases if case["expected"] == "Violated"]
    rejected = sum(outcomes[case["id"]][0] == "Violated" for case in negatives)
    survivors = [case["id"] for case in negatives if outcomes[case["id"]][0] != "Violated"]
    report = {
        "schema": "p1343-oracle-run-report-r1",
        "phase": "focal-authorship" if args.focus else "full-preseal-or-final",
        "agreement": agreement,
        "runs": runs,
        "summary": {
            "cases": len(cases),
            "valid_negatives": len(negatives),
            "negative_violated": rejected,
            "mutation_score": rejected / len(negatives) if negatives else None,
            "preserved_controls": sum(case["expected"] == "Preserved" and outcomes[case["id"]][0] == "Preserved" for case in cases),
            "opaque_unknown": sum(case["expected"] == "Unknown" and outcomes[case["id"]][0] == "Unknown" for case in cases),
            "survivors": survivors,
        },
    }
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if agreement and not survivors else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OracleFailure, Exception) as exc:
        if isinstance(exc, OracleFailure):
            code, detail = exc.code, exc.detail
        else:
            code, detail = "INTERNAL", f"{type(exc).__name__}: {exc}"
        print(json.dumps({"schema": "p1343-oracle-run-report-r1", "fatal": f"{code}: {detail}"}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
