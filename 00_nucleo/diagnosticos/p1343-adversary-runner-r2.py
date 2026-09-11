#!/usr/bin/env python3
"""Independent focal adversarial revalidation of the protected P1343 R2 oracle.

The runner never executes the composed corpus.  It replays ADV01--ADV11 and a
finite set of source/runtime/checker attacks in synthetic trees under
``/dev/shm``.  It imports but never modifies the protected contract or oracle.
"""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
SOURCE_PATH = DIAG / "p1343-source-verifier-r2.py"
CHECKER_PATH = DIAG / "p1343-oracle-checker-r2.py"
CORPUS_PATH = DIAG / "p1343-oracle-corpus-r2.json"

PINS = {
    "step": "6db7b3bb119f4038f88b8916209a4fcd7d1485b820143308ae788192a921a87b",
    "authority_manifest": "d41aff08708b9fa72ac1d130ee71543f5048ebf2dc6944c25aba9a42b866dc3b",
    "capsule_baseline": "db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977",
    "contract": "67107164ddb631e2c1539ca88c90658041811dc389bbae47cb2738312a5f3467",
    "binding": "fbd5ff5f9f32594f95b6b76ea11b562fd1772172dde3a80d4083c9e401ccea85",
    "contract_receipt": "dfccfd81e7254c5c22d3de9530a3fd55b267e906e503de62c27da91539026e15",
    "source_verifier_r1": "07c5827a578ad298ba5f777d56395d9bc66eab156a9500a6730a5cdc335bf146",
    "checker_r1": "ee8f8b5fa212753d1e90294ee6f6d1559ec9d717a667169f596e33c8b2b576e8",
    "corpus_r1": "294e83c280086f64466651e9f5971e55968710dae639fdbaa6eb984cd2442ea1",
    "source_verifier_r2": "fa7c15eea4b45e83c5f078a7a3820bd0499d55087aaa96f7863ea5229495512d",
    "checker_r2": "ff6d83c9dbfe356b2068204e15d05025f10d83eacb902d9b9c1511a20b098ab9",
    "corpus_r2": "72eb41172966de03a42dfc26eea39f7c426f38123f6ba512bb8bc663a30cfcce",
    "authorship_r2": "eaf46ddbb047a5725c8f65832691a4b193f80c2c3028cd19d3131455f807171c",
    "authorship_receipt_r2": "d1b240846ab14f3501e05f7c675eb44bdea30cd183e167f73fdc288c94a18dc2",
    "adversary_runner_r1": "6064ab8d20199cd5fc0d938c97bbda4e0be1d3882a1d784b537565714595a899",
    "adversary_report_r1": "2ceaf77378591bb16458d3e594d1c9f4ff459f6bcdfffc2f3665df8894e9ff8d",
    "adversary_receipt_r1": "00dd7a228654f02ab01a10e29ed97195d8ff66570da75bfbf76e0b0064961802",
    "l0_freeze": "2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e",
    "fixture": "98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714",
    "p1342_contract": "07689c204f8741cdcfcc23bef9fa4af969dba6d991caaa93afaf117eb7ac73e7",
    "p1342_binding": "8630a376350d374f854345c37282ac8fbb657f2f9a8506b7de47bda7b8276bf5",
}

FILES = {
    "step": ROOT / "00_nucleo/materialization/typst-passo-1343.md",
    "authority_manifest": DIAG / "p1343-authority-manifest.json",
    "capsule_baseline": DIAG / "p1343-capsule-baseline-r1.json",
    "contract": DIAG / "p1343-contract-spec-r1.json",
    "binding": DIAG / "p1343-contract-binding-r1.json",
    "contract_receipt": DIAG / "p1343-contract-receipt-r1.json",
    "source_verifier_r1": DIAG / "p1343-source-verifier-r1.py",
    "checker_r1": DIAG / "p1343-oracle-checker-r1.py",
    "corpus_r1": DIAG / "p1343-oracle-corpus-r1.json",
    "source_verifier_r2": SOURCE_PATH,
    "checker_r2": CHECKER_PATH,
    "corpus_r2": CORPUS_PATH,
    "authorship_r2": DIAG / "p1343-oracle-authorship-r2.md",
    "authorship_receipt_r2": DIAG / "p1343-oracle-authorship-receipt-r2.json",
    "adversary_runner_r1": DIAG / "p1343-adversary-runner-r1.py",
    "adversary_report_r1": DIAG / "p1343-adversary-report-r1.json",
    "adversary_receipt_r1": DIAG / "p1343-adversary-receipt-r1.json",
    "l0_freeze": DIAG / "p1342-l0-freeze-r1.json",
    "fixture": DIAG / "p1342-contract-fixture-r1.typ",
    "p1342_contract": DIAG / "p1342-contract-spec-r3.json",
    "p1342_binding": DIAG / "p1342-contract-binding-r3.json",
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def load_module(path: Path, name: str, expected: str) -> Any:
    if sha256(path.read_bytes()) != expected:
        raise RuntimeError(f"protected input drift: {path.name}")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def check_pins() -> None:
    for label, path in FILES.items():
        actual = sha256(path.read_bytes())
        if actual != PINS[label]:
            raise RuntimeError(f"protected pin drift: {label}: {actual}")


def derive(
    root: Path,
    source: Any,
    manifest: dict[str, Any],
    binding: dict[str, Any],
    contract: dict[str, Any],
    runtime_evidence: dict[str, Any],
) -> tuple[str, str]:
    try:
        source.derive_source_evidence(
            contract=contract,
            contract_sha=PINS["contract"],
            binding=binding,
            binding_sha=PINS["binding"],
            authority_sha=PINS["authority_manifest"],
            manifest=manifest,
            manifest_sha=PINS["capsule_baseline"],
            p1342_contract_sha=PINS["p1342_contract"],
            p1342_binding_sha=PINS["p1342_binding"],
            l0_sha=PINS["l0_freeze"],
            fixture_sha=PINS["fixture"],
            candidate_root=root,
            runtime=runtime_evidence,
            source_verifier_sha=PINS["source_verifier_r2"],
            invocation_sha=sha256(b"p1343-independent-adversary-r2"),
        )
    except source.VerificationFailure as exc:
        return "Violated", f"{exc.code}: {exc.detail}"
    return "Preserved", "R2 source verifier emitted PASS evidence"


def record(
    case_id: str,
    family: str,
    purpose: str,
    expected: str,
    actual: str,
    witness: Any,
    *,
    validity: str = "valid",
) -> dict[str, Any]:
    return {
        "id": case_id,
        "family": family,
        "purpose": purpose,
        "expected": expected,
        "actual": actual,
        "validity": validity,
        "survived": validity == "valid" and expected == "Violated" and actual != "Violated",
        "witness": witness,
    }


def source_attack(
    case_id: str,
    purpose: str,
    mutate: Callable[[Path, dict[str, Any]], None] | None,
    *,
    checker: Any,
    source: Any,
    manifest: dict[str, Any],
    binding: dict[str, Any],
    contract: dict[str, Any],
    runtime_mutate: Callable[[dict[str, Any]], None] | None = None,
    expected: str = "Violated",
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix=f"p1343-r2-{case_id.lower()}-", dir="/dev/shm") as temp:
        root = Path(temp)
        checker.populate_positive_tree(root, manifest, ROOT)
        runtime_evidence = checker.runtime(binding, PINS["fixture"])
        if mutate is not None:
            mutate(root, runtime_evidence)
        if runtime_mutate is not None:
            runtime_mutate(runtime_evidence)
        actual, witness = derive(root, source, manifest, binding, contract, runtime_evidence)
        failures = checker.rustfmt_probe(root, manifest) if actual == "Preserved" else []
    if failures:
        return record(
            case_id, "independent-source-negative", purpose, expected,
            "InvalidNegative", {"source_verifier": witness, "rustfmt_failures": failures},
            validity="invalid-rust-syntax",
        )
    return record(case_id, "independent-source-negative", purpose, expected, actual, witness)


def edit(checker: Any, manifest: dict[str, Any], capsule_id: str, old: bytes, new: bytes) -> Callable[[Path, dict[str, Any]], None]:
    def mutate(root: Path, _runtime: dict[str, Any]) -> None:
        checker.edit_body(root, manifest, capsule_id, old, new)
    return mutate


def replace(checker: Any, manifest: dict[str, Any], capsule_id: str, body: bytes) -> Callable[[Path, dict[str, Any]], None]:
    def mutate(root: Path, _runtime: dict[str, Any]) -> None:
        checker.replace_body(root, manifest, capsule_id, body)
    return mutate


def checker_args(corpus: Path, digest: str) -> list[str]:
    args = [sys.executable, "-B", str(CHECKER_PATH)]
    mapping = (
        ("contract", "contract"),
        ("binding", "binding"),
        ("authority-manifest", "authority_manifest"),
        ("capsule-baseline", "capsule_baseline"),
        ("p1342-contract", "p1342_contract"),
        ("p1342-binding", "p1342_binding"),
        ("l0-freeze", "l0_freeze"),
        ("fixture", "fixture"),
        ("source-verifier", "source_verifier_r2"),
    )
    for option, label in mapping:
        args.extend((f"--{option}", str(FILES[label]), f"--{option}-sha256", PINS[label]))
    args.extend(("--corpus", str(corpus), "--corpus-sha256", digest, "--focus"))
    return args


def invoke_checker(corpus_raw: bytes, supplied_digest: str) -> tuple[bool, dict[str, Any]]:
    with tempfile.TemporaryDirectory(prefix="p1343-r2-checker-", dir="/dev/shm") as temp:
        corpus = Path(temp) / "corpus.json"
        corpus.write_bytes(corpus_raw)
        completed = subprocess.run(
            checker_args(corpus, supplied_digest), cwd=ROOT, capture_output=True, text=True, timeout=90,
        )
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError:
        payload = {"stdout": completed.stdout[-1200:], "stderr": completed.stderr[-1200:]}
    accepted = completed.returncode == 0 and payload.get("agreement") is True
    payload["exit_code"] = completed.returncode
    return accepted, payload


def replay_adv_cases(
    checker: Any,
    source: Any,
    manifest: dict[str, Any],
    binding: dict[str, Any],
    contract: dict[str, Any],
) -> list[dict[str, Any]]:
    cases = json.loads(CORPUS_PATH.read_bytes())["cases"]
    by_id = {case["id"]: case for case in cases}
    results = []
    for number in range(1, 10):
        case_id = next(key for key in by_id if key.startswith(f"ADV{number:02d}-"))
        case = by_id[case_id]
        with tempfile.TemporaryDirectory(prefix=f"p1343-r2-replay-{number:02d}-", dir="/dev/shm") as temp:
            root = Path(temp)
            checker.populate_positive_tree(root, manifest, ROOT)
            runtime_evidence = checker.runtime(binding, PINS["fixture"])
            checker.apply_attack(case_id, root, manifest, runtime_evidence)
            actual, witness = derive(root, source, manifest, binding, contract, runtime_evidence)
        results.append(record(case_id, "ADV-R1-replay", case["purpose"], case["expected"], actual, witness))

    original = CORPUS_PATH.read_bytes()
    forged = bytearray(original)
    needle = b'"full_corpus_runs": 0'
    if forged.count(needle) != 1:
        raise RuntimeError("ADV10 mutation needle drift")
    forged = forged.replace(needle, b'"full_corpus_runs": 9', 1)
    accepted, payload = invoke_checker(bytes(forged), PINS["corpus_r2"])
    results.append(record(
        "ADV10-unpinned-forged-corpus", "ADV-R1-replay",
        by_id["ADV10-unpinned-forged-corpus"]["purpose"], "Violated",
        "Preserved" if accepted else "Violated",
        {"reason": "externally supplied protected digest rejected altered bytes", **payload},
    ))

    with tempfile.TemporaryDirectory(prefix="p1343-r2-adv11-", dir="/dev/shm") as temp:
        root = Path(temp)
        checker.populate_positive_tree(root, manifest, ROOT)
        failures = checker.rustfmt_probe(root, manifest)
    results.append(record(
        "ADV11-authored-positive-not-rust-parseable", "ADV-R1-replay",
        by_id["ADV11-authored-positive-not-rust-parseable"]["purpose"], "Preserved",
        "Violated" if failures else "Preserved",
        {"rustfmt_failures": failures, "parsed_files": 10 - len(failures)},
    ))
    return results


def forged_one_case_corpus(base: dict[str, Any]) -> dict[str, Any]:
    forged = copy.deepcopy(base)
    positive = next(case for case in forged["cases"] if case["id"] == "P01-inspectable-composite")
    forged["cases"] = [positive]
    forged["closed_case_order"] = [positive["id"]]
    forged["budget"]["focal_case_ids"] = [positive["id"]]
    return forged


def main() -> int:
    check_pins()
    source = load_module(SOURCE_PATH, "p1343_source_verifier_r2_for_adversary_r2", PINS["source_verifier_r2"])
    checker = load_module(CHECKER_PATH, "p1343_oracle_checker_r2_for_adversary_r2", PINS["checker_r2"])
    manifest = json.loads(FILES["capsule_baseline"].read_bytes())
    binding = json.loads(FILES["binding"].read_bytes())
    contract = json.loads(FILES["contract"].read_bytes())
    results = replay_adv_cases(checker, source, manifest, binding, contract)

    cfg = checker.cfg("all(test,p1339_observation)")
    hook_call = b'p1343_append_raw("H01", p1343_event());'
    carrier_needle = b"    fn p1343_project_dto"

    attacks: list[tuple[str, str, Callable[[Path, dict[str, Any]], None] | None, Callable[[dict[str, Any]], None] | None]] = [
        (
            "R2A01-qualified-writer-alias",
            "Qualify the hook call through an unrelated namespace while retaining the matched writer suffix.",
            edit(checker, manifest, "P1343-H01-ATTEMPT-OPEN", hook_call, b'decoy::p1343_append_raw("H01", p1343_event());'),
            None,
        ),
        (
            "R2A02-unlisted-retain-mutator",
            "Mutate the raw ledger through Vec::retain, absent from the closed mutator regex.",
            edit(checker, manifest, "P1343-FUNC-CARRIER-TYPES", carrier_needle,
                 b"    fn p1343_change(&mut self) { self.p1343_raw_events.retain(|_| true); }\n" + carrier_needle),
            None,
        ),
        (
            "R2A03-indirect-index-assignment",
            "Overwrite an event through get_mut/dereference, bypassing direct index-assignment matching.",
            edit(checker, manifest, "P1343-FUNC-CARRIER-TYPES", carrier_needle,
                 b"    fn p1343_change(&mut self, event: Event) { *self.p1343_raw_events.get_mut(0).unwrap() = event; }\n" + carrier_needle),
            None,
        ),
        (
            "R2A04-dead-closure-hook",
            "Move H01 into an uncalled closure; only nested fn declarations are rejected.",
            replace(checker, manifest, "P1343-H01-ATTEMPT-OPEN", cfg + b'let _dead = || { p1343_append_raw("H01", p1343_event()); };\n'),
            None,
        ),
        (
            "R2A05-hook-literal-in-comment",
            "Call the writer with a variable and place the expected literal only in a trailing comment window.",
            edit(checker, manifest, "P1343-H01-ATTEMPT-OPEN", hook_call,
                 b'p1343_append_raw(hook_name, p1343_event()); // , "H01")'),
            None,
        ),
        (
            "R2A06-hook-literal-in-string",
            "Call the writer with a variable and place the expected literal only in a later string literal.",
            replace(checker, manifest, "P1343-H01-ATTEMPT-OPEN", cfg + b'{ p1343_append_raw(hook_name, p1343_event()); let _decoy = ", \\"H01\\")"; }\n'),
            None,
        ),
        (
            "R2A07-normal-branch-extra-token",
            "Add an effective semicolon after the exact normal replacement branch.",
            edit(checker, manifest, "P1343-H06-BOUND-UPDATE", b"}\n#[cfg(p1339_observation)]", b"}\n;\n#[cfg(p1339_observation)]"),
            None,
        ),
        (
            "R2A08-cfg-macro-comment-split",
            "Hide cfg! from the raw substring guard with a block comment while retaining a valid guarded macro call.",
            replace(checker, manifest, "P1343-H01-ATTEMPT-OPEN", cfg + b'{ p1343_append_raw("H01", p1343_event()); let _ = cfg /* split */ !(p1339_observation); }\n'),
            None,
        ),
        (
            "R2A09-facade-calls-under-if-false",
            "Keep all required facade tokens and exact calls but place execution behind if false.",
            replace(checker, manifest, "P1343-TEST-FACADE", cfg + (
                b"fn p1342_run_fixture_for_test() {\n"
                b" let normal_world = p1343_fresh_world(); let normal_source = p1343_fresh_source(); let normal_challenge = p1343_fresh_challenge();\n"
                b" let repeat_world = p1343_fresh_world(); let repeat_source = p1343_fresh_source(); let repeat_challenge = p1343_fresh_challenge();\n"
                b" let reverse_world = p1343_fresh_world(); let reverse_source = p1343_fresh_source(); let reverse_challenge = p1343_fresh_challenge();\n"
                b" if false {\n"
                b"p1343_run_mode(\"normal\", normal_world, normal_source, normal_challenge);\n"
                b"p1343_run_mode(\"repeat\", repeat_world, repeat_source, repeat_challenge);\n"
                b"p1343_run_mode(\"reverse\", reverse_world, reverse_source, reverse_challenge);\n"
                b" }\n}\n"
            )),
            None,
        ),
        (
            "R2A10-facade-reuses-world-source-challenge",
            "Give the three syntactically named paths aliases of the same world/source/challenge objects.",
            replace(checker, manifest, "P1343-TEST-FACADE", cfg + (
                b"fn p1342_run_fixture_for_test() {\n"
                b" let normal_world = p1343_fresh_world(); let normal_source = p1343_fresh_source(); let normal_challenge = p1343_fresh_challenge();\n"
                b" let repeat_world = normal_world.clone(); let repeat_source = normal_source.clone(); let repeat_challenge = normal_challenge.clone();\n"
                b" let reverse_world = normal_world.clone(); let reverse_source = normal_source.clone(); let reverse_challenge = normal_challenge.clone();\n"
                b"p1343_run_mode(\"normal\", normal_world, normal_source, normal_challenge);\n"
                b"p1343_run_mode(\"repeat\", repeat_world, repeat_source, repeat_challenge);\n"
                b"p1343_run_mode(\"reverse\", reverse_world, reverse_source, reverse_challenge);\n"
                b"}\n"
            )),
            None,
        ),
        (
            "R2A11-formatted-but-fabricated-runtime",
            "Supply perfectly formatted distinct challenge/run/receipt strings with no causal execution receipt.",
            None,
            None,
        ),
        (
            "R2A12-posthoc-event-struct-literal",
            "Construct an Event after execution via a neutral-named struct literal not covered by the keyword scan.",
            edit(checker, manifest, "P1343-FUNC-CARRIER-TYPES", carrier_needle,
                 b'    fn p1343_rebuild_event(&self) -> Event { Event { hook: "H01" } }\n' + carrier_needle),
            None,
        ),
        (
            "R2A13-posthoc-event-macro",
            "Introduce a neutral-named Event construction macro without the two forbidden identifier substrings.",
            edit(checker, manifest, "P1343-FUNC-CARRIER-TYPES", b"#[cfg(p1339_observation)]\nimpl P1343RawCarrier",
                 b'#[cfg(p1339_observation)]\nmacro_rules! p1343_rebuild { () => { Event { hook: "H01" } } }\n#[cfg(p1339_observation)]\nimpl P1343RawCarrier'),
            None,
        ),
        (
            "R2A14-projection-mutates-through-helper",
            "Have the projection call a helper that mutates through unenumerated retain.",
            edit(checker, manifest, "P1343-FUNC-CARRIER-TYPES", b"    fn p1343_project_dto(&self) -> Vec<Event> { self.p1343_raw_events.clone() }",
                 b"    fn p1343_change_for_output(&mut self) { self.p1343_raw_events.retain(|_| true); }\n"
                 b"    fn p1343_project_dto(&mut self) -> Vec<Event> { self.p1343_change_for_output(); self.p1343_raw_events.clone() }"),
            None,
        ),
        (
            "R2A15-shadowed-wrong-raw-snapshot",
            "Retain the exact clone witness, then shadow the snapshot with a clone of the wrong object before projection use.",
            edit(checker, manifest, "P1343-H16-RAW-FREEZE",
                 cfg + b"let raw_snapshot = &p1343_raw_snapshot;",
                 cfg + b"let p1343_raw_snapshot = self.other_events.clone();\n" + cfg + b"let raw_snapshot = &p1343_raw_snapshot;"),
            None,
        ),
        (
            "R2A16-exact-clone-only-in-dead-closure",
            "Put the exact raw clone in an uncalled closure while the live raw_snapshot references another object.",
            replace(checker, manifest, "P1343-H16-RAW-FREEZE",
                    cfg + b"let _dead = || { let p1343_raw_snapshot = self.p1343_raw_events.clone(); };\n"
                    + cfg + b"let raw_snapshot = &self.other_events;\n"
                    + cfg + b'p1343_append_raw("H16", p1343_event());\n'),
            None,
        ),
    ]

    for case_id, purpose, mutation, runtime_mutation in attacks:
        results.append(source_attack(
            case_id, purpose, mutation, checker=checker, source=source,
            manifest=manifest, binding=binding, contract=contract,
            runtime_mutate=runtime_mutation,
        ))

    base_corpus = json.loads(CORPUS_PATH.read_bytes())
    forged = forged_one_case_corpus(base_corpus)
    forged_raw = canonical_json(forged)
    accepted, payload = invoke_checker(forged_raw, sha256(forged_raw))
    results.append(record(
        "R2A17-attacker-consistent-corpus-pin", "checker-protected-input-negative",
        "Replace the closed R2 corpus with one positive and supply the matching attacker-computed digest.",
        "Violated", "Preserved" if accepted else "Violated",
        {"attacker_supplied_sha256": sha256(forged_raw), **payload},
    ))

    duplicate_prefix = canonical_json({"budget": forged["budget"]})[1:-2]
    duplicate_raw = b"{" + duplicate_prefix + b"," + forged_raw[1:]
    accepted, payload = invoke_checker(duplicate_raw, sha256(duplicate_raw))
    results.append(record(
        "R2A18-duplicate-json-key-corpus", "checker-schema-negative",
        "Use duplicate top-level budget keys; the checker uses permissive json.loads and accepts the last value.",
        "Violated", "Preserved" if accepted else "Violated",
        {"duplicate_key": "budget", "attacker_supplied_sha256": sha256(duplicate_raw), **payload},
    ))

    def symlink_source(root: Path, _runtime: dict[str, Any]) -> None:
        relative = manifest["files"][0]["path"]
        path = root / relative
        backing = root / "backing.rs"
        path.rename(backing)
        path.symlink_to(backing)

    results.append(source_attack(
        "R2A19-allowlisted-source-symlink", "Reject an allowlisted source replaced by a symlink.",
        symlink_source, checker=checker, source=source, manifest=manifest,
        binding=binding, contract=contract,
    ))

    def outside_owner_decoy(root: Path, _runtime: dict[str, Any]) -> None:
        path = root / manifest["files"][0]["path"]
        path.write_bytes(b"fn p1343_outside_decoy() {}\n" + path.read_bytes())

    results.append(source_attack(
        "R2A20-owner-decoy-outside-capsule", "Add an owner-shaped decoy outside every capsule.",
        outside_owner_decoy, checker=checker, source=source, manifest=manifest,
        binding=binding, contract=contract,
    ))

    negatives = [item for item in results if item["expected"] == "Violated" and item["validity"] == "valid"]
    violated = sum(item["actual"] == "Violated" for item in negatives)
    survivors = [item["id"] for item in negatives if item["actual"] != "Violated"]
    positive_failures = [item["id"] for item in results if item["expected"] == "Preserved" and item["actual"] != "Preserved"]
    report = {
        "schema": "p1343-adversary-run-report-r2",
        "step": 1343,
        "revision": 2,
        "role": "independent_adversary_revalidation",
        "executor": "/root/p1343_adversary_r2",
        "regime": "executado sem atestacao de isolamento",
        "protected_inputs": {label: [path.relative_to(ROOT).as_posix(), PINS[label]] for label, path in FILES.items()},
        "temporary_root": "/dev/shm",
        "full_corpus_runs": 0,
        "attacks": results,
        "summary": {
            "attacks": len(results),
            "valid_negatives": len(negatives),
            "negative_violated": violated,
            "mutation_score": violated / len(negatives) if negatives else None,
            "survivors": survivors,
            "invalid_negative_cases": [item["id"] for item in results if item["validity"] != "valid"],
            "positive_validity_failures": positive_failures,
            "verdict": "ADVERSARIAL_SURVIVORS_BLOCK_PRESEAL" if survivors or positive_failures else "ADVERSARIAL_R2_PASS_NOT_PRESEALED",
        },
    }
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 1 if survivors or positive_failures else 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(json.dumps({"schema": "p1343-adversary-run-report-r2", "fatal": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
