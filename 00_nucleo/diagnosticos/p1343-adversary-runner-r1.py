#!/usr/bin/env python3
"""Independent focal adversarial runner for the P1343 R1 oracle.

This runner never executes the protected full corpus.  It builds the oracle
author's synthetic positive tree in /dev/shm, applies one bounded negative at
a time, and asks the pinned source verifier to derive evidence again.  It also
probes the checker with a one-case, unpinned corpus and checks whether the
authored positive source is parseable by rustfmt.
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
SOURCE_PATH = DIAG / "p1343-source-verifier-r1.py"
CHECKER_PATH = DIAG / "p1343-oracle-checker-r1.py"
CORPUS_PATH = DIAG / "p1343-oracle-corpus-r1.json"

PINS = {
    "authority_manifest": "d41aff08708b9fa72ac1d130ee71543f5048ebf2dc6944c25aba9a42b866dc3b",
    "capsule_baseline": "db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977",
    "contract": "67107164ddb631e2c1539ca88c90658041811dc389bbae47cb2738312a5f3467",
    "binding": "fbd5ff5f9f32594f95b6b76ea11b562fd1772172dde3a80d4083c9e401ccea85",
    "contract_receipt": "dfccfd81e7254c5c22d3de9530a3fd55b267e906e503de62c27da91539026e15",
    "source_verifier": "07c5827a578ad298ba5f777d56395d9bc66eab156a9500a6730a5cdc335bf146",
    "checker": "ee8f8b5fa212753d1e90294ee6f6d1559ec9d717a667169f596e33c8b2b576e8",
    "corpus": "294e83c280086f64466651e9f5971e55968710dae639fdbaa6eb984cd2442ea1",
    "l0_freeze": "2fb962c3edd8cdd83848d2cd7c9158c39a5d0218f510e81bb5e8011a540e8c0e",
    "fixture": "98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714",
    "p1342_contract": "07689c204f8741cdcfcc23bef9fa4af969dba6d991caaa93afaf117eb7ac73e7",
    "p1342_binding": "8630a376350d374f854345c37282ac8fbb657f2f9a8506b7de47bda7b8276bf5",
}

FILES = {
    "authority_manifest": DIAG / "p1343-authority-manifest.json",
    "capsule_baseline": DIAG / "p1343-capsule-baseline-r1.json",
    "contract": DIAG / "p1343-contract-spec-r1.json",
    "binding": DIAG / "p1343-contract-binding-r1.json",
    "contract_receipt": DIAG / "p1343-contract-receipt-r1.json",
    "source_verifier": SOURCE_PATH,
    "checker": CHECKER_PATH,
    "corpus": CORPUS_PATH,
    "l0_freeze": DIAG / "p1342-l0-freeze-r1.json",
    "fixture": DIAG / "p1342-contract-fixture-r1.typ",
    "p1342_contract": DIAG / "p1342-contract-spec-r3.json",
    "p1342_binding": DIAG / "p1342-contract-binding-r3.json",
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def check_pins() -> None:
    for label, path in FILES.items():
        actual = sha256(path.read_bytes())
        if actual != PINS[label]:
            raise RuntimeError(f"protected pin drift: {label}: {actual}")


def runtime(binding: dict[str, Any], fixture_sha: str, duplicate_refs: bool = False) -> dict[str, Any]:
    rows = []
    for row in binding["row_capsule_bindings"]:
        if duplicate_refs:
            refs = {mode: "candidate-self-attested:one-receipt" for mode in ("normal", "repeat", "reverse")}
        else:
            refs = {mode: f"receipt:{mode}:{row['row_index']}" for mode in ("normal", "repeat", "reverse")}
        rows.append({"row_index": row["row_index"], "hook": row["hook"], **refs})
    raw = (
        {mode: "candidate-self-attested:one-raw-snapshot" for mode in ("normal", "repeat", "reverse")}
        if duplicate_refs
        else {mode: f"raw:{mode}:fixture" for mode in ("normal", "repeat", "reverse")}
    )
    return {
        "schema": "p1343-runtime-evidence-v1",
        "fixture_sha256": fixture_sha,
        "modes": ["normal", "repeat", "reverse"],
        "row_coverage": rows,
        "raw_snapshot_refs": raw,
        "result": "PASS",
    }


def replace_capsule_body(root: Path, manifest: dict[str, Any], oracle: Any, capsule_id: str, body: bytes) -> None:
    capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == capsule_id)
    path = root / capsule["path"]
    raw = path.read_bytes()
    _, body_start, body_end, _ = oracle.find_capsule(raw, capsule_id)
    path.write_bytes(raw[:body_start] + body + raw[body_end:])


def edit_capsule_body(root: Path, manifest: dict[str, Any], oracle: Any, capsule_id: str, old: bytes, new: bytes) -> None:
    capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == capsule_id)
    path = root / capsule["path"]
    raw = path.read_bytes()
    _, body_start, body_end, _ = oracle.find_capsule(raw, capsule_id)
    body = raw[body_start:body_end]
    if body.count(old) != 1:
        raise RuntimeError(f"mutation needle not unique in {capsule_id}: {old!r}")
    body = body.replace(old, new, 1)
    path.write_bytes(raw[:body_start] + body + raw[body_end:])


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
            source_verifier_sha=PINS["source_verifier"],
            invocation_sha=sha256(b"p1343-independent-adversary-r1"),
        )
    except source.VerificationFailure as exc:
        return "Violated", f"{exc.code}: {exc.detail}"
    return "Preserved", "source verifier emitted PASS evidence"


def run_source_attack(
    attack_id: str,
    purpose: str,
    mutate: Callable[[Path], None],
    *,
    oracle: Any,
    source: Any,
    manifest: dict[str, Any],
    binding: dict[str, Any],
    contract: dict[str, Any],
    runtime_evidence: dict[str, Any],
    expected: str = "Violated",
) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix=f"p1343-adv-{attack_id.lower()}-", dir="/dev/shm") as temp:
        root = Path(temp)
        oracle.populate_positive_tree(root, manifest, ROOT)
        mutate(root)
        actual, witness = derive(root, source, manifest, binding, contract, runtime_evidence)
    return {
        "id": attack_id,
        "family": "independent-source-negative",
        "purpose": purpose,
        "expected": expected,
        "actual": actual,
        "survived": expected == "Violated" and actual != "Violated",
        "witness": witness,
    }


def checker_args(corpus: Path) -> list[str]:
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
        ("source-verifier", "source_verifier"),
    )
    for option, label in mapping:
        args.extend((f"--{option}", str(FILES[label]), f"--{option}-sha256", PINS[label]))
    args.extend(("--corpus", str(corpus), "--focus"))
    return args


def run_forged_corpus_attack(corpus: dict[str, Any]) -> dict[str, Any]:
    forged = copy.deepcopy(corpus)
    positive = next(case for case in forged["cases"] if case["id"] == "P01-inspectable-composite")
    forged["cases"] = [positive]
    forged["closed_case_order"] = [positive["id"]]
    forged["budget"]["focal_case_ids"] = [positive["id"]]
    with tempfile.TemporaryDirectory(prefix="p1343-adv-corpus-", dir="/dev/shm") as temp:
        path = Path(temp) / "attacker-corpus.json"
        path.write_bytes(canonical_json(forged))
        completed = subprocess.run(checker_args(path), cwd=ROOT, capture_output=True, text=True, timeout=60)
        try:
            result = json.loads(completed.stdout)
        except json.JSONDecodeError:
            result = {"stdout": completed.stdout[-1000:], "stderr": completed.stderr[-1000:]}
    accepted = completed.returncode == 0 and result.get("agreement") is True
    return {
        "id": "ADV10-unpinned-forged-corpus",
        "family": "checker-protected-input-negative",
        "purpose": "Replace the authored 85-case corpus with one attacker-selected positive while retaining valid embedded pins.",
        "expected": "Violated",
        "actual": "Preserved" if accepted else "Violated",
        "survived": accepted,
        "witness": {
            "exit_code": completed.returncode,
            "agreement": result.get("agreement"),
            "summary": result.get("summary"),
            "checker_has_corpus_sha256_option": False,
        },
    }


def syntax_probe(oracle: Any, manifest: dict[str, Any]) -> dict[str, Any]:
    failures = []
    with tempfile.TemporaryDirectory(prefix="p1343-adv-syntax-", dir="/dev/shm") as temp:
        root = Path(temp)
        oracle.populate_positive_tree(root, manifest, ROOT)
        for item in manifest["files"]:
            path = root / item["path"]
            completed = subprocess.run(
                ["rustfmt", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", str(path)],
                capture_output=True,
                text=True,
                timeout=30,
            )
            if completed.returncode != 0:
                diagnostic = (completed.stderr or completed.stdout).strip().splitlines()
                failures.append({"path": item["path"], "first_diagnostic": diagnostic[0] if diagnostic else "rustfmt failed"})
    return {
        "id": "ADV11-authored-positive-not-rust-parseable",
        "family": "positive-validity-control",
        "purpose": "The contract requires a syntactically valid positive; parse the exact synthetic positive accepted by the focal oracle.",
        "expected": "Preserved",
        "actual": "InvalidPositive" if failures else "Preserved",
        "survived": False,
        "blocks_seal": bool(failures),
        "witness": {"rustfmt_parse_failures": failures, "failed_files": len(failures)},
    }


def main() -> int:
    check_pins()
    source = load_module(SOURCE_PATH, "p1343_source_adversary_r1")
    oracle = load_module(CHECKER_PATH, "p1343_oracle_adversary_r1")
    manifest = json.loads(FILES["capsule_baseline"].read_bytes())
    binding = json.loads(FILES["binding"].read_bytes())
    contract = json.loads(FILES["contract"].read_bytes())
    corpus = json.loads(FILES["corpus"].read_bytes())
    normal_runtime = runtime(binding, PINS["fixture"])

    attacks: list[dict[str, Any]] = []

    def edit(capsule_id: str, old: bytes, new: bytes) -> Callable[[Path], None]:
        return lambda root: edit_capsule_body(root, manifest, oracle, capsule_id, old, new)

    attacks.append(run_source_attack(
        "ADV01-wrong-hook-payload",
        "A row-bound capsule calls the sole writer with the wrong hook/event identity.",
        edit("P1343-H01-ATTEMPT-OPEN", b'p1343_append_raw("H01")', b'p1343_append_raw("NOT-H01")'),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV02-dead-owner-symbol-decoy",
        "Replace a real H01 observation with an uncalled nested function carrying the matching writer text.",
        lambda root: replace_capsule_body(
            root, manifest, oracle, "P1343-H01-ATTEMPT-OPEN",
            b'#[cfg(all(test, p1339_observation))]\nfn p1343_dead_h01() { p1343_append_raw("H01"); }\n',
        ),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV03-writer-pushes-wrong-object",
        "Keep one syntactic .push in the writer but redirect it from the raw ledger to an unrelated object.",
        edit("P1343-PIPE-SUPPORT", b"ledger.push(event)", b"scratch.push(event)"),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV04-alternate-writer-via-extend",
        "Add a second direct raw-ledger mutator using extend, which the .push-only scan does not enumerate.",
        edit(
            "P1343-PIPE-SUPPORT",
            b"    fn p1343_project_dto(raw: &[Event]) -> Dto { project(raw) }\n",
            b"    fn p1343_alternate_writer(event: Event) { ledger.extend([event]); }\n"
            b"    fn p1343_project_dto(raw: &[Event]) -> Dto { project(raw) }\n",
        ),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV05-projection-mutates-ledger-via-extend",
        "Make the raw-to-DTO projection mutate the ledger through extend rather than .push or the named writer.",
        edit(
            "P1343-PIPE-SUPPORT",
            b"fn p1343_project_dto(raw: &[Event]) -> Dto { project(raw) }",
            b"fn p1343_project_dto(raw: &[Event]) -> Dto { ledger.extend(raw.iter().cloned()); project(raw) }",
        ),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV06-posthoc-constructor-unscanned",
        "Add an explicit post-hoc Event constructor; the verifier emits an empty constructor list without scanning it.",
        edit(
            "P1343-PIPE-SUPPORT",
            b"    fn p1343_project_dto(raw: &[Event]) -> Dto { project(raw) }\n",
            b"    fn p1343_posthoc() -> Event { Event::forged_after_execution() }\n"
            b"    fn p1343_project_dto(raw: &[Event]) -> Dto { project(raw) }\n",
        ),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV07-fake-raw-freeze-token",
        "Retain only the identifier raw_freeze while removing the snapshot clone required before projection.",
        edit("P1343-H16-RAW-FREEZE", b"let raw_freeze = ledger.clone()", b"let raw_freeze = ()"),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=normal_runtime,
    ))
    attacks.append(run_source_attack(
        "ADV08-empty-facade-with-forged-runtime",
        "The exact facade exists but executes no worlds, sources or challenges; detached runtime strings claim all coverage.",
        lambda root: replace_capsule_body(
            root, manifest, oracle, "P1343-TEST-FACADE",
            b"#[cfg(all(test, p1339_observation))]\nfn p1342_run_fixture_for_test() {}\n",
        ),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=runtime(binding, PINS["fixture"], duplicate_refs=True),
    ))
    attacks.append(run_source_attack(
        "ADV09-combined-dead-hook-fake-freeze-runtime",
        "Combine wrong/dead H01, fake H16 freeze and one duplicated self-attested runtime receipt.",
        lambda root: (
            replace_capsule_body(
                root, manifest, oracle, "P1343-H01-ATTEMPT-OPEN",
                b'#[cfg(all(test, p1339_observation))]\nfn p1343_dead_wrong() { p1343_append_raw("NOT-H01"); }\n',
            ),
            edit_capsule_body(root, manifest, oracle, "P1343-H16-RAW-FREEZE", b"let raw_freeze = ledger.clone()", b"let raw_freeze = ()"),
        ),
        oracle=oracle, source=source, manifest=manifest, binding=binding, contract=contract,
        runtime_evidence=runtime(binding, PINS["fixture"], duplicate_refs=True),
    ))
    attacks.append(run_forged_corpus_attack(corpus))
    attacks.append(syntax_probe(oracle, manifest))

    survivors = [item["id"] for item in attacks if item.get("survived")]
    blockers = [item["id"] for item in attacks if item.get("blocks_seal")]
    negative_count = sum(item["expected"] == "Violated" for item in attacks)
    rejected = sum(item["expected"] == "Violated" and item["actual"] == "Violated" for item in attacks)
    report = {
        "schema": "p1343-adversary-run-report-r1",
        "step": 1343,
        "revision": 1,
        "role": "independent_adversary",
        "executor": "/root/p1343_adversary",
        "regime": "executado sem atestacao de isolamento",
        "full_corpus_runs": 0,
        "temporary_root": "/dev/shm",
        "protected_inputs": {label: [str(FILES[label].relative_to(ROOT)), PINS[label]] for label in PINS},
        "attacks": attacks,
        "summary": {
            "attacks": len(attacks),
            "valid_negatives": negative_count,
            "negative_violated": rejected,
            "mutation_score": rejected / negative_count if negative_count else None,
            "survivors": survivors,
            "positive_validity_blockers": blockers,
            "verdict": "ADVERSARIAL_SURVIVORS_BLOCK_SEAL" if survivors or blockers else "ADVERSARIAL_PASS",
        },
    }
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 1 if survivors or blockers else 0


if __name__ == "__main__":
    raise SystemExit(main())
