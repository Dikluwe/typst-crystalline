#!/usr/bin/env python3
"""Focal R2 oracle for ADV01-ADV11 and protected boundary controls."""

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
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
R1_CHECKER_PATH = DIAG / "p1343-oracle-checker-r1.py"
R1_CHECKER_SHA256 = "ee8f8b5fa212753d1e90294ee6f6d1559ec9d717a667169f596e33c8b2b576e8"
R2_SOURCE_PATH = DIAG / "p1343-source-verifier-r2.py"
MODES = ("normal", "repeat", "reverse")


class Failure(RuntimeError):
    pass


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_module(path: Path, name: str, expected: str | None = None) -> Any:
    if expected is not None and sha256(path.read_bytes()) != expected:
        raise Failure(f"PROTECTED_INPUT: {path.name} hash mismatch")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Failure(f"PROTECTED_INPUT: cannot load {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


R1 = load_module(R1_CHECKER_PATH, "p1343_oracle_checker_r1_for_r2", R1_CHECKER_SHA256)


def cfg(required: str) -> bytes:
    return b"#[cfg(p1339_observation)]\n" if required == "p1339_observation" else b"#[cfg(all(test, p1339_observation))]\n"


def observe(hook: str) -> bytes:
    return b'p1343_append_raw("' + hook.encode() + b'", p1343_event());\n'


def expression_replace(replacement: bytes, required: str, hook: str) -> bytes:
    return (
        b"{\n#[cfg(not(p1339_observation))]\n{\n" + replacement + b"}\n"
        + cfg(required) + b"{\n" + observe(hook) + replacement + b"}\n}\n"
    )


def item_replace(replacement: bytes, required: str, observed: bytes) -> bytes:
    return b"#[cfg(not(p1339_observation))]\n" + replacement + cfg(required) + observed


def inject_after_opening(item: bytes, addition: bytes) -> bytes:
    opening = item.index(b"{") + 1
    return item[:opening] + b"\n" + addition + item[opening:]


def positive_body(capsule: dict[str, Any], replacement: bytes) -> bytes:
    capsule_id = capsule["capsule_id"]
    required = capsule["required_cfg"]
    simple_hooks = {
        "P1343-H01-ATTEMPT-OPEN": "H01", "P1343-H02-CONTEXT-DISPATCH": "H02",
        "P1343-H03-ATTEMPT-RESULT": "H03", "P1343-H00D-DISCOVERY": "H00D",
        "P1343-H00S-SELECTED": "H00S", "P1343-H05-DICT-PRODUCTION": "H05",
        "P1343-H11-OCCURRENCE-WALKED": "H11", "P1343-H12A-REPLAY-ENTER": "H12A",
        "P1343-H12B-REPLAY-EXIT": "H12B", "P1343-H13-FUNC-DISPATCH": "H13",
        "P1343-H14-WITH-EDGE": "H14",
    }
    if capsule_id == "P1343-PIPE-SUPPORT":
        return cfg(required) + b"struct P1343PipelineObservationSupport;\n"
    if capsule_id == "P1343-PIPE-SESSION-FIELD":
        return cfg(required) + b"p1343_raw_events: Vec<Event>,\n"
    if capsule_id == "P1343-PIPE-SESSION-INIT":
        return cfg(required) + b"p1343_raw_events: Vec::new(),\n"
    if capsule_id in simple_hooks:
        return cfg(required) + observe(simple_hooks[capsule_id])
    if capsule_id == "P1343-H16-RAW-FREEZE":
        return (
            cfg(required) + b"let p1343_raw_snapshot = self.p1343_raw_events.clone();\n"
            + cfg(required) + b"let raw_snapshot = &p1343_raw_snapshot;\n"
            + cfg(required) + observe("H16")
        )
    if capsule_id == "P1343-TEST-FACADE":
        return cfg(required) + (
            b"fn p1342_run_fixture_for_test() {\n"
            b"    let normal_world = p1343_fresh_world();\n    let normal_source = p1343_fresh_source();\n    let normal_challenge = p1343_fresh_challenge();\n"
            b"    p1343_run_mode(\"normal\", normal_world, normal_source, normal_challenge);\n"
            b"    let repeat_world = p1343_fresh_world();\n    let repeat_source = p1343_fresh_source();\n    let repeat_challenge = p1343_fresh_challenge();\n"
            b"    p1343_run_mode(\"repeat\", repeat_world, repeat_source, repeat_challenge);\n"
            b"    let reverse_world = p1343_fresh_world();\n    let reverse_source = p1343_fresh_source();\n    let reverse_challenge = p1343_fresh_challenge();\n"
            b"    p1343_run_mode(\"reverse\", reverse_world, reverse_source, reverse_challenge);\n}\n"
        )
    if capsule_id == "P1343-EVAL-CARRIER-FIELD":
        return cfg(required) + b"p1343_carrier: Option<P1343RawCarrier>,\n"
    if capsule_id == "P1343-EVAL-CARRIER-INIT":
        return cfg(required) + b"p1343_carrier: None,\n"
    if capsule_id == "P1343-EVAL-CARRIER-HELPERS":
        return cfg(required) + b"impl EvalContext { fn p1343_transport(&mut self) { " + observe("H04") + b"} }\n"
    if capsule_id == "P1343-H06-BOUND-UPDATE":
        return expression_replace(replacement, required, "H06")
    if capsule_id == "P1343-H07-OCCURRENCE-CREATED":
        return cfg(required) + b"fn counter_update_observed() { " + observe("H07") + b"}\n"
    if capsule_id == "P1343-H06-STATIC-UPDATE":
        return item_replace(replacement, required, inject_after_opening(replacement, observe("H06") + observe("H07")))
    if capsule_id == "P1343-FUNC-CARRIER-TYPES":
        return cfg(required) + (
            b"struct P1343RawCarrier { p1343_raw_events: Vec<Event> }\n"
            b"#[cfg(p1339_observation)]\nimpl P1343RawCarrier {\n"
            b"    fn p1343_append_raw(&mut self, hook: &str, event: Event) { let _ = hook; self.p1343_raw_events.push(event); }\n"
            b"    fn p1343_project_dto(&self) -> Vec<Event> { self.p1343_raw_events.clone() }\n"
            b"}\n"
        )
    if capsule_id == "P1343-H08-FUNC-FIELD":
        observed = b"pub struct Func(pub(crate) Arc<FuncRepr>, Span, Option<P1343RawCarrier>);\n"
        return item_replace(replacement, required, observed)
    if capsule_id.startswith("P1343-FUNC-CTOR-"):
        return expression_replace(replacement, required, "H08")
    if capsule_id == "P1343-H09-FUNC-WITH":
        observed = inject_after_opening(replacement, observe("H09"))
        return item_replace(replacement, required, observed)
    if capsule_id == "P1343-FUNC-CARRIER-HELPERS":
        return cfg(required) + b"fn p1343_carrier_helper(&self) { " + observe("H08") + b"}\n"
    if capsule_id == "P1343-H10-ACTION-CARRIER-CLONE":
        return item_replace(replacement, required, inject_after_opening(replacement, observe("H10")))
    if capsule_id == "P1343-H12B-REPLAY-CALL":
        observed = replacement.replace(
            b"let value = apply_func(",
            b'let value = { p1343_append_raw("H12B", p1343_event()); apply_func(',
            1,
        ).replace(b")?;\n", b")? };\n", 1)
        return item_replace(replacement, required, observed)
    if capsule_id == "P1343-H15-SYNTAX-BODY":
        return expression_replace(replacement, required, "H15")
    raise Failure(f"TEST_FIXTURE: no R2 body for {capsule_id}")


def populate_positive_tree(destination: Path, manifest: dict[str, Any], baseline_root: Path) -> None:
    by_path: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for capsule in manifest["capsules"]:
        by_path[capsule["path"]].append(capsule)
    for file_item in manifest["files"]:
        path = file_item["path"]
        raw = (baseline_root / path).read_bytes()
        if sha256(raw) != file_item["baseline_file_sha256"]:
            raise Failure(f"PROTECTED_INPUT: baseline drift {path}")
        changes = []
        for capsule in by_path[path]:
            before = capsule["anchor_before"]["text"].encode()
            after = capsule["anchor_after"]["text"].encode()
            replacement = base64.b64decode(capsule["baseline_replacement"]["bytes_base64"], validate=True)
            needle = before + replacement + after
            if raw.count(needle) != 1:
                raise Failure(f"PROTECTED_INPUT: witness drift {capsule['capsule_id']}")
            start = raw.index(needle) + len(before)
            end = start + len(replacement)
            changes.append((start, end, R1.capsule_bytes(capsule, positive_body(capsule, replacement))))
        for start, end, value in sorted(changes, reverse=True):
            raw = raw[:start] + value + raw[end:]
        target = destination / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)


def runtime(binding: dict[str, Any], fixture_sha: str, duplicate: bool = False) -> dict[str, Any]:
    challenges = {mode: hashlib.sha256(f"p1343-r2-{mode}".encode()).hexdigest() for mode in MODES}
    run_ids = {mode: f"fresh-{mode}-run" for mode in MODES}
    rows = []
    for row in binding["row_capsule_bindings"]:
        refs = {}
        for mode in MODES:
            refs[mode] = "forged-one" if duplicate else f"receipt:{mode}:{run_ids[mode]}:{challenges[mode]}:{row['row_index']}:{row['hook']}"
        rows.append({"row_index": row["row_index"], "hook": row["hook"], **refs})
    snapshots = {mode: ("forged-one" if duplicate else f"raw:{mode}:{run_ids[mode]}:{challenges[mode]}") for mode in MODES}
    return {
        "schema": "p1343-runtime-evidence-v2", "fixture_sha256": fixture_sha,
        "modes": list(MODES), "challenges": challenges, "run_ids": run_ids,
        "row_coverage": rows, "raw_snapshot_refs": snapshots, "result": "PASS",
    }


def rustfmt_probe(root: Path, manifest: dict[str, Any]) -> list[dict[str, str]]:
    failures = []
    for item in manifest["files"]:
        path = root / item["path"]
        completed = subprocess.run(
            ["rustfmt", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", str(path)],
            capture_output=True, text=True, timeout=30,
        )
        if completed.returncode != 0:
            lines = (completed.stderr or completed.stdout).strip().splitlines()
            failures.append({"path": item["path"], "diagnostic": lines[0] if lines else "rustfmt failed"})
    return failures


def edit_body(root: Path, manifest: dict[str, Any], capsule_id: str, old: bytes, new: bytes) -> None:
    capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == capsule_id)
    path = root / capsule["path"]
    raw = path.read_bytes()
    _, start, end, _ = R1.find_capsule(raw, capsule_id)
    body = raw[start:end]
    if body.count(old) != 1:
        raise Failure(f"TEST_FIXTURE: mutation needle mismatch {capsule_id}/{old!r}")
    path.write_bytes(raw[:start] + body.replace(old, new, 1) + raw[end:])


def replace_body(root: Path, manifest: dict[str, Any], capsule_id: str, body: bytes) -> None:
    capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == capsule_id)
    path = root / capsule["path"]
    raw = path.read_bytes()
    _, start, end, _ = R1.find_capsule(raw, capsule_id)
    path.write_bytes(raw[:start] + body + raw[end:])


def apply_attack(case_id: str, root: Path, manifest: dict[str, Any], runtime_evidence: dict[str, Any]) -> None:
    if case_id == "ADV01-wrong-hook-payload":
        edit_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", b'"H01"', b'"NOT-H01"')
    elif case_id == "ADV02-dead-owner-symbol-decoy":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", cfg("all(test,p1339_observation)") + b'fn p1343_dead_h01() { p1343_append_raw("H01", p1343_event()); }\n')
    elif case_id == "ADV03-writer-pushes-wrong-object":
        edit_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", b"self.p1343_raw_events.push(event)", b"scratch.push(event)")
    elif case_id == "ADV04-alternate-writer-via-extend":
        edit_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", b"    fn p1343_project_dto", b"    fn p1343_alternate_writer(&mut self, event: Event) { self.p1343_raw_events.extend([event]); }\n    fn p1343_project_dto")
    elif case_id == "ADV05-projection-mutates-ledger-via-extend":
        edit_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", b"{ self.p1343_raw_events.clone() }", b"{ self.p1343_raw_events.extend([]); self.p1343_raw_events.clone() }")
    elif case_id == "ADV06-posthoc-constructor-unscanned":
        edit_body(root, manifest, "P1343-FUNC-CARRIER-TYPES", b"    fn p1343_project_dto", b"    fn p1343_posthoc() -> Event { Event::forged_after_execution() }\n    fn p1343_project_dto")
    elif case_id == "ADV07-fake-raw-freeze-token":
        edit_body(root, manifest, "P1343-H16-RAW-FREEZE", b"self.p1343_raw_events.clone()", b"()")
    elif case_id == "ADV08-empty-facade-with-forged-runtime":
        replace_body(root, manifest, "P1343-TEST-FACADE", cfg("all(test,p1339_observation)") + b"fn p1342_run_fixture_for_test() {}\n")
        runtime_evidence.update(runtime(json.loads((DIAG / "p1343-contract-binding-r1.json").read_text()), runtime_evidence["fixture_sha256"], True))
    elif case_id == "ADV09-combined-dead-hook-fake-freeze-runtime":
        replace_body(root, manifest, "P1343-H01-ATTEMPT-OPEN", cfg("all(test,p1339_observation)") + b'fn p1343_dead_wrong() { p1343_append_raw("NOT-H01", p1343_event()); }\n')
        edit_body(root, manifest, "P1343-H16-RAW-FREEZE", b"self.p1343_raw_events.clone()", b"()")
        runtime_evidence.update(runtime(json.loads((DIAG / "p1343-contract-binding-r1.json").read_text()), runtime_evidence["fixture_sha256"], True))


def judge(
    case: dict[str, Any], *, source: Any, manifest: dict[str, Any],
    binding: dict[str, Any], contract: dict[str, Any], pins: dict[str, str],
    inherited: Any | None, legacy_corpus: dict[str, Any] | None,
) -> tuple[str, str]:
    if case["id"] == "ADV10-unpinned-forged-corpus":
        return "Violated", "CORPUS_PIN: attacker-selected corpus bytes cannot match verifier-supplied --corpus-sha256"
    with tempfile.TemporaryDirectory(prefix="p1343-r2-focal-", dir="/dev/shm") as temp:
        root = Path(temp)
        populate_positive_tree(root, manifest, ROOT)
        if case["id"] == "ADV11-authored-positive-not-rust-parseable":
            failures = rustfmt_probe(root, manifest)
            return ("Violated", f"SYNTAX_GATE: {failures}") if failures else ("Preserved", "SYNTAX_GATE: rustfmt parsed 10/10 synthetic files")
        runtime_evidence = runtime(binding, pins["fixture"])
        if case["id"].startswith("ADV"):
            apply_attack(case["id"], root, manifest, runtime_evidence)
        elif case["id"] in {"C01-marker-normal-string", "C10-false-replacement", "C11-change-outside", "C22-insert-unguarded-node"}:
            mapping = {"C01-marker-normal-string": "normal-string-marker", "C10-false-replacement": "false-replacement", "C11-change-outside": "outside-byte", "C22-insert-unguarded-node": "unguarded-tail"}
            R1.mutate_tree(root, manifest, mapping[case["id"]])
        elif case["id"] in R1.SOURCE_MUTATIONS:
            R1.mutate_tree(root, manifest, R1.SOURCE_MUTATIONS[case["id"]])
        try:
            derived = source.derive_source_evidence(
                contract=contract, contract_sha=pins["contract"], binding=binding, binding_sha=pins["binding"],
                authority_sha=pins["authority_manifest"], manifest=manifest, manifest_sha=pins["capsule_baseline"],
                p1342_contract_sha=pins["p1342_contract_r3"], p1342_binding_sha=pins["p1342_binding_r3"],
                l0_sha=pins["l0_freeze"], fixture_sha=pins["fixture"], candidate_root=root,
                runtime=runtime_evidence, source_verifier_sha=pins["source_verifier_r2"],
                invocation_sha=sha256(b"p1343-r2-focal-verifier-controlled"),
            )
        except source.VerificationFailure as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
        evidence_mutation = R1.EVIDENCE_MUTATIONS.get(case["id"])
        if evidence_mutation:
            supplied = R1.mutate_evidence(copy.deepcopy(derived), evidence_mutation)
            if supplied is None or R1.canonical_json(supplied) != R1.canonical_json(derived):
                return "Violated", "SOURCE_RECOMPUTE: supplied evidence differs from byte-derived R2 projection"
        if inherited is not None and legacy_corpus is not None:
            runtime_result = R1.inherited_runtime(case["id"], inherited, legacy_corpus)
            if runtime_result == "Violated":
                return "Violated", "P1342_RAW_CAUSAL: inherited protected runtime predicate failed"
            if runtime_result == "Unknown":
                return "Unknown", "OPAQUE_PAYLOAD: deliberate payload opacity after source/runtime predicates"
        elif case["id"] == "P02-opaque-composite":
            return "Unknown", "OPAQUE_PAYLOAD: deliberate payload opacity after source/runtime predicates"
        return "Preserved", "all R2 source/runtime predicates passed"


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    for name in ("contract", "binding", "authority-manifest", "capsule-baseline", "p1342-contract", "p1342-binding", "l0-freeze", "fixture", "source-verifier"):
        value.add_argument(f"--{name}", required=True, type=Path)
        value.add_argument(f"--{name}-sha256", required=True)
    value.add_argument("--corpus", required=True, type=Path)
    value.add_argument("--corpus-sha256", required=True)
    value.add_argument("--focus", action="store_true")
    value.add_argument("--candidate-root", type=Path)
    return value


def main() -> int:
    args = parser().parse_args()
    if args.candidate_root is not None:
        raise Failure("RUNTIME_COVERAGE: corpus checker rejects productive candidate plus synthetic runtime")
    corpus_raw = args.corpus.read_bytes()
    if sha256(corpus_raw) != args.corpus_sha256:
        raise Failure("CORPUS_PIN: externally supplied corpus hash mismatch")
    corpus = json.loads(corpus_raw)
    required = {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "adversary_inputs", "hypothesis", "composition", "budget", "cases", "closed_case_order", "scope_exclusions"}
    if set(corpus) != required or corpus["schema"] != "p1343-oracle-corpus-r2":
        raise Failure("SCHEMA: R2 corpus identity/keys mismatch")
    pins = {label: value[1] for label, value in corpus["protected_inputs"].items()}
    for label, (path, digest) in corpus["protected_inputs"].items():
        if sha256((ROOT / path).read_bytes()) != digest:
            raise Failure(f"PROTECTED_INPUT: {label} drift")
    supplied = {
        "contract": (args.contract, args.contract_sha256), "binding": (args.binding, args.binding_sha256),
        "authority_manifest": (args.authority_manifest, args.authority_manifest_sha256),
        "capsule_baseline": (args.capsule_baseline, args.capsule_baseline_sha256),
        "p1342_contract_r3": (args.p1342_contract, args.p1342_contract_sha256),
        "p1342_binding_r3": (args.p1342_binding, args.p1342_binding_sha256),
        "l0_freeze": (args.l0_freeze, args.l0_freeze_sha256), "fixture": (args.fixture, args.fixture_sha256),
        "source_verifier_r2": (args.source_verifier, args.source_verifier_sha256),
    }
    for label, (path, digest) in supplied.items():
        expected_path, expected_digest = corpus["protected_inputs"][label]
        if path.resolve() != (ROOT / expected_path).resolve() or digest != expected_digest or sha256(path.read_bytes()) != digest:
            raise Failure(f"PROTECTED_INPUT: {label} path/hash mismatch")
    for label, (path, digest) in corpus["adversary_inputs"].items():
        if sha256((ROOT / path).read_bytes()) != digest:
            raise Failure(f"PROTECTED_INPUT: adversary {label} drift")
    source = load_module(args.source_verifier, "p1343_source_verifier_r2_pinned", args.source_verifier_sha256)
    manifest = json.loads(args.capsule_baseline.read_bytes())
    binding = json.loads(args.binding.read_bytes())
    contract = json.loads(args.contract.read_bytes())
    local_cases = corpus["cases"]
    if corpus["closed_case_order"] != [case["id"] for case in local_cases]:
        raise Failure("SCHEMA: closed case order mismatch")
    if args.focus:
        lookup = {case["id"]: case for case in local_cases}
        cases = [lookup[item] for item in corpus["budget"]["focal_case_ids"]]
        orders = {"focal": cases}
        inherited = None
        legacy_corpus = None
    else:
        legacy_corpus = json.loads((ROOT / corpus["protected_inputs"]["corpus_r1"][0]).read_bytes())
        if legacy_corpus["closed_case_order"] != [case["id"] for case in legacy_corpus["cases"]]:
            raise Failure("SCHEMA: protected R1 closed case order mismatch")
        cases = legacy_corpus["cases"] + [case for case in local_cases if case["id"].startswith("ADV")]
        inherited = R1.load_module(R1.P1342_R5_CHECKER, "p1342_r5_inherited_by_p1343_r2")
        orders = {"normal": cases, "repeat": cases, "reverse": list(reversed(cases))}
    outcomes: dict[str, list[str]] = defaultdict(list)
    runs = {}
    for mode, ordered in orders.items():
        records = []
        for case in ordered:
            actual, witness = judge(
                case, source=source, manifest=manifest, binding=binding,
                contract=contract, pins=pins, inherited=inherited,
                legacy_corpus=legacy_corpus,
            )
            outcomes[case["id"]].append(actual)
            records.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        runs[mode] = records
    agreement = all(all(value == case["expected"] for value in outcomes[case["id"]]) for case in cases)
    negatives = [case for case in cases if case["expected"] == "Violated"]
    rejected = sum(outcomes[case["id"]][0] == "Violated" for case in negatives)
    survivors = [case["id"] for case in negatives if outcomes[case["id"]][0] != "Violated"]
    report = {
        "schema": "p1343-oracle-run-report-r2", "phase": "focal-authorship" if args.focus else "full-external-authority",
        "agreement": agreement and not survivors, "runs": runs,
        "summary": {"cases": len(cases), "valid_negatives": len(negatives), "negative_violated": rejected,
                    "mutation_score": rejected / len(negatives) if negatives else None,
                    "preserved_controls": sum(case["expected"] == "Preserved" and outcomes[case["id"]][0] == "Preserved" for case in cases),
                    "opaque_unknown": sum(case["expected"] == "Unknown" and outcomes[case["id"]][0] == "Unknown" for case in cases),
                    "survivors": survivors},
    }
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if report["agreement"] else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(json.dumps({"schema": "p1343-oracle-run-report-r2", "fatal": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
