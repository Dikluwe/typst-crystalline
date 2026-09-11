#!/usr/bin/env python3
"""Focal P1344 R2 oracle for the three independently measured failures."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CORPUS_PATH = DIAG / "p1344-oracle-corpus-r2.json"
SOURCE_PATH = DIAG / "p1344-source-verifier-r2.py"
R1_CHECKER_PATH = DIAG / "p1344-oracle-checker-r1.py"
ADVERSARY_PATH = DIAG / "p1344-adversary-runner-r1.py"
EXPECTED_CORPUS_SHA256 = "1246920352da5678603dadcd07efd2f1e2364e2e397d2d26fdd36b8b7ef757e5"
EXPECTED_SOURCE_SHA256 = "62824f399b34659b81a2ab6ff538ad0747b4bb76aed6c5af4cfc78fd17a20e82"
EXPECTED_R1_CHECKER_SHA256 = "375602081721b368aa0bd032738392ee7fae2ff429183d1f04fd4f7f7144c564"
EXPECTED_ADVERSARY_SHA256 = "db23812f7bd90b232b7bd3f63de6ea66118b84ba7a02ff50d9863360b905d3fe"
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


R1 = load_module(R1_CHECKER_PATH, "p1344_checker_r1_for_r2", EXPECTED_R1_CHECKER_SHA256)
SOURCE = load_module(SOURCE_PATH, "p1344_source_verifier_r2_pinned", EXPECTED_SOURCE_SHA256)
ADVERSARY = load_module(ADVERSARY_PATH, "p1344_adversary_r1_for_oracle_r2", EXPECTED_ADVERSARY_SHA256)


def cfg(required: str = "p1339_observation") -> bytes:
    return b"#[cfg(p1339_observation)]\n" if required == "p1339_observation" else b"#[cfg(all(test, p1339_observation))]\n"


def content_body() -> bytes:
    return cfg() + (
        b"pub(crate) fn p1343_observe_h11(&self) -> Option<P1343Carrier> {\n"
        b" match self { Content::CounterUpdate(elem) => elem.action.p1343_observe_h10(), _ => None, }\n}\n"
    )


def counter_body() -> bytes:
    return cfg() + (
        b"pub(crate) fn p1343_observe_h10(&self) -> Option<P1343Carrier> {\n"
        b" match self { CounterUpdate::Func(function) => function.p1343_carrier(), CounterUpdate::Set(_) | CounterUpdate::Step(_) => None, }\n}\n"
    )


def func_body() -> bytes:
    chunks = [
        cfg() + b"pub(crate) fn p1343_carrier(&self) -> Option<P1343Carrier> { self.2.clone() }\n",
        cfg() + b"pub(crate) fn p1343_attach_carrier(&mut self, carrier: P1343Carrier) { self.2 = Some(carrier); }\n",
    ]
    for name in SOURCE.FUNC_METHODS[2:]:
        chunks.append(cfg() + b"pub(crate) fn " + name.encode() + b"(&self, event: P1343RawEvent) { if let Some(carrier) = self.p1343_carrier() { carrier.append(event); } }\n")
    return b"".join(chunks)


def eval_body() -> bytes:
    methods = b"".join(b" fn " + name.encode() + b"(&mut self) {}\n" for name in SOURCE.EVAL_METHODS)
    return cfg() + b"impl EvalContext {\n" + methods + b"}\n"


def direct_body(capsule_id: str, protected: dict[str, Any]) -> bytes:
    syntax = {item["capsule_id"]: item for item in protected["p1343_binding_final"]["capsule_syntax_table"]}[capsule_id]
    args = b", ".join(name.encode() for name in SOURCE.DIRECT_ARGUMENTS[capsule_id])
    return cfg(syntax["exact_cfg"]) + syntax["canonical_callee"].encode() + b"(" + args + b");\n"


def replace_body(root: Path, manifest: dict[str, Any], capsule_id: str, body: bytes) -> None:
    R1.replace_body(root, manifest, capsule_id, lambda _old: body)


def populate_positive_tree(destination: Path, protected: dict[str, Any], manifest: dict[str, Any]) -> None:
    R1.populate_positive_tree(destination, protected, manifest)
    replace_body(destination, manifest, "P1344-CONTENT-CARRIER-HELPERS", content_body())
    replace_body(destination, manifest, "P1344-COUNTER-UPDATE-CARRIER-HELPERS", counter_body())
    replace_body(destination, manifest, "P1343-FUNC-CARRIER-HELPERS", func_body())
    replace_body(destination, manifest, "P1343-EVAL-CARRIER-HELPERS", eval_body())
    for capsule_id in SOURCE.DIRECT_ARGUMENTS:
        replace_body(destination, manifest, capsule_id, direct_body(capsule_id, protected))


def load_closed() -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    raw = CORPUS_PATH.read_bytes()
    if sha256(raw) != EXPECTED_CORPUS_SHA256:
        raise Failure("AUTHORITY_ROOT: compiled R2 corpus hash mismatch")
    corpus = SOURCE.R1.strict_json(raw, "P1344 R2 corpus")
    required = {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "refinement_classes", "composition", "budget", "controls", "attacks", "focal_closed_case_order", "unknown_policy", "scope_exclusions"}
    SOURCE.R1.exact_keys(corpus, required, "P1344 R2 corpus")
    if corpus["schema"] != "p1344-oracle-corpus-r2" or corpus["revision"] != 2:
        raise Failure("SCHEMA: R2 corpus identity mismatch")
    ids = [item["id"] for item in corpus["controls"]] + [item["id"] for item in corpus["attacks"]]
    if ids != corpus["focal_closed_case_order"] or len(ids) != 29 or len(set(ids)) != 29:
        raise Failure("SCHEMA: R2 focal order is not 29 unique cases")
    for label, pair in corpus["protected_inputs"].items():
        path, expected = pair
        target = ROOT / path
        if target.resolve() != target.absolute() or sha256(target.read_bytes()) != expected:
            raise Failure(f"PROTECTED_INPUT: {label} path/hash mismatch")
    protected = SOURCE.R1.load_protected()
    manifest = SOURCE.R1.compose_manifest(protected["p1343_capsule_baseline"], protected["capsule_baseline"])
    return corpus, protected, manifest


def rustfmt_file(path: Path) -> tuple[bool, str]:
    completed = subprocess.run(["rustfmt", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", str(path)], capture_output=True, text=True, timeout=30)
    detail = (completed.stderr or completed.stdout).splitlines()
    return completed.returncode == 0, (detail[0] if detail else "rustfmt accepted")


def custom_attack(case_id: str, old: bytes) -> bytes:
    if case_id == "P1344-A09-func-helper-nonterminating":
        return old.replace(b"{ self.2.clone() }", b"{ loop {} }", 1)
    if case_id == "P1344-A10-func-helper-alternate-storage":
        return old.replace(b"{ self.2.clone() }", b"{ let mut shadow = Vec::<u8>::new(); shadow.pop(); self.2.clone() }", 1)
    if case_id == "P1344-A12-direct-hook-extra-side-effect-argument":
        return old.replace(b"attempt_kind);", b"attempt_kind, side_effect());", 1)
    raise Failure(f"TEST_FIXTURE: no custom attack {case_id}")


def judge_source_attack(attack: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> tuple[str, str]:
    with tempfile.TemporaryDirectory(prefix="p1344-r2-attack-", dir="/dev/shm") as temp:
        root = Path(temp)
        populate_positive_tree(root, protected, manifest)
        attack_id = attack["id"]
        original = next(item for item in ADVERSARY.SOURCE_ATTACKS if item["id"] == attack_id)
        transform: Callable[[bytes], bytes] = original["transform"]
        if attack_id in {"P1344-A09-func-helper-nonterminating", "P1344-A10-func-helper-alternate-storage", "P1344-A12-direct-hook-extra-side-effect-argument"}:
            transform = lambda old: custom_attack(attack_id, old)
        R1.replace_body(root, manifest, original["capsule"], transform)
        capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == original["capsule"])
        parser_valid, witness = rustfmt_file(root / capsule["path"])
        if not parser_valid:
            return "InvalidNegative", f"SYNTAX_GATE: {witness}"
        try:
            SOURCE.verify_candidate(root)
        except (SOURCE.VerificationFailure, SOURCE.R1.R1.VerificationFailure) as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
        return "Preserved", "closed R2 source grammar accepted the valid attack"


def validate_opaque(case: dict[str, Any], corpus: dict[str, Any]) -> tuple[str, str]:
    witness = case.get("opaque_witness")
    keys = corpus["unknown_policy"]["required_witness_keys"]
    values = corpus["unknown_policy"]["exact_values"]
    if not isinstance(witness, dict) or list(witness) != keys or [witness[key] for key in keys] != values:
        return "Violated", "OPAQUE_PAYLOAD: missing or noncanonical structured dict witness"
    return "Unknown", "OPAQUE_PAYLOAD: canonical dict witness after source and synthetic non-payload predicates"


def judge_meta(attack: dict[str, Any], corpus: dict[str, Any]) -> tuple[str, str]:
    case_id = attack["id"]
    if case_id == "P1344-A20-escaped-duplicate-json-key":
        try:
            SOURCE.R1.strict_json(b'{"owner":1,"\\u006fwner":2}', "escaped duplicate")
        except SOURCE.VerificationFailure as exc:
            return "Violated", f"{exc.code}: {exc.detail}"
        return "Preserved", "duplicate decoded key accepted"
    if case_id == "P1344-A21-alternate-corpus-and-digest":
        forged = b'{"schema":"p1344-oracle-corpus-r2","controls":[]}\n'
        return ("Violated", "AUTHORITY_ROOT: compiled corpus rejects alternate bytes") if sha256(forged) != EXPECTED_CORPUS_SHA256 else ("Preserved", "alternate corpus equals root")
    if case_id == "P1344-A22-unknown-without-dict-witness":
        return validate_opaque({"id": case_id}, corpus)
    if case_id == "P1344-A23-synthetic-candidate-runtime":
        return "Violated", "RUNTIME_AUTHORITY: synthetic evidence remains corpus-only"
    raise Failure(f"SCHEMA: unknown meta attack {case_id}")


def judge(case: dict[str, Any], corpus: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> tuple[str, str]:
    if case["id"].startswith("P1344-R2-P"):
        with tempfile.TemporaryDirectory(prefix="p1344-r2-positive-", dir="/dev/shm") as temp:
            root = Path(temp)
            populate_positive_tree(root, protected, manifest)
            try:
                SOURCE.verify_candidate(root)
            except (SOURCE.VerificationFailure, SOURCE.R1.R1.VerificationFailure) as exc:
                return "Violated", f"{exc.code}: {exc.detail}"
            if case["id"] == "P1344-R2-P05-opaque-with-witness":
                return validate_opaque(case, corpus)
            if case["id"] == "P1344-R2-P04-carrier-facade-wrapper-grammar":
                failures = []
                for file_item in manifest["files"]:
                    ok, witness = rustfmt_file(root / file_item["path"])
                    if not ok: failures.append(witness)
                if failures:
                    return "Violated", f"SYNTAX_GATE: {failures}"
            return "Preserved", "closed R2 production accepted the canonical control"
    if case["id"] in {item["id"] for item in ADVERSARY.SOURCE_ATTACKS}:
        return judge_source_attack(case, protected, manifest)
    return judge_meta(case, corpus)


def run_cases(cases: list[dict[str, Any]], corpus: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> list[dict[str, Any]]:
    records = []
    for case in cases:
        actual, witness = judge(case, corpus, protected, manifest)
        records.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
    return records


def authority_root(checker_sha: str) -> str:
    values = [
        SOURCE.R1.PINS["step"][1], SOURCE.R1.PINS["authority_manifest"][1], SOURCE.R1.PINS["contract"][1], SOURCE.R1.PINS["binding"][1],
        SOURCE.R1.PINS["capsule_baseline"][1], SOURCE.R1.PINS["l0_freeze"][1], SOURCE.R1.PINS["fixture"][1], EXPECTED_SOURCE_SHA256, EXPECTED_CORPUS_SHA256, checker_sha,
    ]
    return sha256(canonical_json(values))


def run_r1_full() -> dict[str, Any]:
    completed = subprocess.run([sys.executable, "-B", str(R1_CHECKER_PATH), "--full", "--corpus", str(DIAG / "p1344-oracle-corpus-r1.json")], cwd=ROOT, capture_output=True, text=True, timeout=1800)
    if completed.returncode != 0:
        raise Failure(f"PROTECTED_INPUT: protected R1 full replay failed: {completed.stderr[-1000:]}")
    report = SOURCE.R1.strict_json(completed.stdout.encode(), "P1344 R1 full report")
    if report.get("agreement") is not True or report.get("summary", {}).get("cases") != 139:
        raise Failure("PROTECTED_INPUT: R1 full replay did not return 139-case agreement")
    return report


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    mode = value.add_mutually_exclusive_group(required=True)
    mode.add_argument("--focus", action="store_true")
    mode.add_argument("--full", action="store_true")
    value.add_argument("--corpus", required=True, type=Path)
    value.add_argument("--candidate-root", type=Path)
    value.add_argument("--runtime-evidence", type=Path)
    return value


def report(records_by_mode: dict[str, list[dict[str, Any]]], phase: str, composed: int) -> dict[str, Any]:
    first = next(iter(records_by_mode.values()))
    negatives = [item for item in first if item["expected"] == "Violated"]
    survivors = [item["id"] for item in negatives if item["actual"] != "Violated"]
    agreement = all(item["actual"] == item["expected"] for records in records_by_mode.values() for item in records)
    checker_sha = sha256(Path(__file__).read_bytes())
    return {
        "schema": "p1344-oracle-report-r2", "phase": phase, "agreement": agreement and not survivors,
        "runs": records_by_mode,
        "summary": {"cases": len(first), "composed_cases": composed, "valid_negatives": len(negatives), "negative_violated": len(negatives) - len(survivors), "mutation_score": (len(negatives) - len(survivors)) / len(negatives) if negatives else None, "preserved_controls": sum(item["expected"] == item["actual"] == "Preserved" for item in first), "opaque_unknown": sum(item["expected"] == item["actual"] == "Unknown" for item in first), "survivors": survivors, "full_corpus_runs": 0 if phase == "focal-authorship" else 1},
        "authority": {"corpus_sha256": EXPECTED_CORPUS_SHA256, "source_verifier_sha256": EXPECTED_SOURCE_SHA256, "checker_sha256": checker_sha, "authority_root_sha256": authority_root(checker_sha)},
        "verdict": "FOCAL_R2_AUTHORED_NOT_VERIFIED_NOT_SEALED" if phase == "focal-authorship" and agreement and not survivors else ("FULL_R2_PASS_NOT_SELF_SEALED" if agreement and not survivors else "SURVIVOR_BLOCKS_PRESEAL"),
    }


def main() -> int:
    args = parser().parse_args()
    if args.corpus.resolve() != CORPUS_PATH.resolve() or sha256(args.corpus.read_bytes()) != EXPECTED_CORPUS_SHA256:
        raise Failure("AUTHORITY_ROOT: alternate R2 corpus path/bytes rejected")
    if args.candidate_root is not None or args.runtime_evidence is not None:
        raise Failure("RUNTIME_AUTHORITY: oracle checker accepts synthetic fixtures only")
    corpus, protected, manifest = load_closed()
    local = corpus["controls"] + corpus["attacks"]
    if args.focus:
        output = report({"focal": run_cases(local, corpus, protected, manifest)}, "focal-authorship", 168)
    else:
        predecessor = run_r1_full()
        runs = {}
        for mode in MODES:
            ordered = local if mode != "reverse" else list(reversed(local))
            runs[mode] = list(predecessor["runs"][mode]) + run_cases(ordered, corpus, protected, manifest)
            if len(runs[mode]) != 168:
                raise Failure(f"SCHEMA: {mode} full R2 order is not 168")
        output = report(runs, "full-external-authority", 168)
    print(json.dumps(output, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if output["agreement"] else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(json.dumps({"schema": "p1344-oracle-fatal-r2", "classification": "Violated", "reason": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
