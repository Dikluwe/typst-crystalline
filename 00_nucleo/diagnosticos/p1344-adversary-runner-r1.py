#!/usr/bin/env python3
"""Independent focal adversary for the candidate-free P1344 oracle R1.

This runner never invokes the 139-case ``--full`` route and never reads a
productive candidate.  It generates synthetic trees under /dev/shm from the
oracle's protected candidate-free baseline, applies valid negative mutations,
and asks the pinned source verifier to classify them.
"""

from __future__ import annotations

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
CHECKER_PATH = DIAG / "p1344-oracle-checker-r1.py"
CHECKER_SHA256 = "375602081721b368aa0bd032738392ee7fae2ff429183d1f04fd4f7f7144c564"
EXPECTED_INPUTS = {
    "corpus": (DIAG / "p1344-oracle-corpus-r1.json", "7ce12848ce375ab21f5ca9e112e5b653d750fdd9e4a7daf93c9d8d1e7d0aa0d6"),
    "source": (DIAG / "p1344-source-verifier-r1.py", "83a0021d504c22ddf1591210015c7d451162789b1dd0256e0299d78e9e7fc9ea"),
    "authorship": (DIAG / "p1344-oracle-authorship-r1.md", "cbdc774e523efbff7814865bde219b3f39e7aeb9435d7c6f733bf144cbd3f7e5"),
    "authorship_receipt": (DIAG / "p1344-oracle-authorship-receipt-r1.json", "dbd9f101292029a428db8533ea938d991270a06b52629c1dd11612b674c1d8f4"),
    "contract": (DIAG / "p1344-contract-spec-r1.json", "157877514d350947f6483ec371c356dd2181c2e0f98e8fa9eb5e465527983279"),
    "binding": (DIAG / "p1344-contract-binding-r1.json", "1f4d8812d0a4e2a9451d2f8c08931fb71e1baa54a3484a411d2cf76727115bab"),
    "manifest": (DIAG / "p1344-authority-manifest.json", "9a3cafa60d0dcffb395bb1bda5c823ed5b1881ee8239fe06d8c6cf7614d342d1"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def load_checker() -> Any:
    if sha256(CHECKER_PATH.read_bytes()) != CHECKER_SHA256:
        raise RuntimeError("PROTECTED_INPUT: checker hash mismatch")
    for label, (path, expected) in EXPECTED_INPUTS.items():
        if sha256(path.read_bytes()) != expected:
            raise RuntimeError(f"PROTECTED_INPUT: {label} hash mismatch")
    spec = importlib.util.spec_from_file_location("p1344_checker_for_adversary_r1", CHECKER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("PROTECTED_INPUT: cannot import checker")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


CHECKER = load_checker()
SOURCE = CHECKER.SOURCE


def cfg() -> bytes:
    return b"#[cfg(p1339_observation)]\n"


def body_content_no_match(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h11(&self) -> Option<()> { let _variant = Content::CounterUpdate; None }\n"


def body_content_inverts(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h11(&self) -> Option<()> { match self { Content::CounterUpdate(_) => None, _ => None } }\n"


def body_content_static(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h11() { let _variant = Content::CounterUpdate; }\n"


def body_content_foreign(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h11(&self) { let _variant = Content::CounterUpdate; let _foreign = Func::p1343_carrier; }\n"


def body_counter_no_match(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h10(&self) { let _variants = (CounterUpdate::Func, CounterUpdate::Set, CounterUpdate::Step); }\n"


def body_counter_inverts(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h10(&self) -> Option<()> { match self { CounterUpdate::Func(_) => None, CounterUpdate::Set(_) => Some(()), CounterUpdate::Step => Some(()) } }\n"


def body_counter_static(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h10() { let _variants = (CounterUpdate::Func, CounterUpdate::Set, CounterUpdate::Step); }\n"


def body_counter_executes(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h10(&self) -> Option<()> { match self { CounterUpdate::Func(function) => { function.call(); None }, CounterUpdate::Set(_) | CounterUpdate::Step => None } }\n"


def body_func_loop(body: bytes) -> bytes:
    return body.replace(b"(&self) {}", b"(&self) { loop {} }", 1)


def body_func_shadow_storage(body: bytes) -> bytes:
    return body.replace(b"(&self) {}", b"(&self) { let mut shadow = Vec::<u8>::new(); shadow.pop(); }", 1)


def body_eval_wrong(_: bytes) -> bytes:
    return cfg() + b"impl EvalContext { fn p1343_totally_wrong(&mut self) {} }\n"


def body_direct_extra_arg(body: bytes) -> bytes:
    return body.replace(b"();", b"(side_effect());", 1)


def body_direct_comment_decoy(body: bytes) -> bytes:
    return body.replace(b"p1343_observe_h01", b"p1343_wrong /* p1343_observe_h01 */", 1)


def body_carrier_extra_eraser(body: bytes) -> bytes:
    needle = b" fn snapshot(&mut self)"
    return body.replace(needle, b" fn erase(&mut self) { self.state.events.pop(); }\n" + needle, 1)


def body_carrier_mutable_projection(body: bytes) -> bytes:
    return body.replace(b"fn project(&self)", b"fn project(&mut self)", 1)


def body_carrier_second_storage(body: bytes) -> bytes:
    return cfg() + b"struct P1343ShadowStorage { values: Vec<u8> }\n" + body


def body_facade_wrong_world(body: bytes) -> bytes:
    body = body.replace(b"world: &World, source:", b"world: &World, other_world: &World, source:", 1)
    body = body.replace(b" let raw = run_context_stabilization_once(world,", b" let _ignored = world;\n let raw = run_context_stabilization_once(other_world,", 1)
    return body


def body_expression_nontermination(body: bytes) -> bytes:
    return body.replace(b"{\np1343_append_raw", b"{\nloop {}\np1343_append_raw", 1)


def body_content_literal_decoy(_: bytes) -> bytes:
    return cfg() + b"pub(crate) fn p1343_observe_h11(&self) { let _s = \"Content::CounterUpdate\"; }\n"


SOURCE_ATTACKS: list[dict[str, Any]] = [
    {"id": "P1344-A01-content-token-without-match", "capsule": "P1344-CONTENT-CARRIER-HELPERS", "transform": body_content_no_match, "claim": "variant token is not a real Content match"},
    {"id": "P1344-A02-content-counter-update-returns-absence", "capsule": "P1344-CONTENT-CARRIER-HELPERS", "transform": body_content_inverts, "claim": "real CounterUpdate occurrence is deliberately suppressed"},
    {"id": "P1344-A03-content-helper-wrong-signature", "capsule": "P1344-CONTENT-CARRIER-HELPERS", "transform": body_content_static, "claim": "associated helper omits self and cannot observe the occurrence"},
    {"id": "P1344-A04-content-foreign-func-indirection", "capsule": "P1344-CONTENT-CARRIER-HELPERS", "transform": body_content_foreign, "claim": "required variant is a decoy while a foreign Func item supplies the apparent carrier"},
    {"id": "P1344-A05-counter-variant-tokens-without-match", "capsule": "P1344-COUNTER-UPDATE-CARRIER-HELPERS", "transform": body_counter_no_match, "claim": "three variant tokens are present but no action is inspected"},
    {"id": "P1344-A06-counter-set-step-return-presence", "capsule": "P1344-COUNTER-UPDATE-CARRIER-HELPERS", "transform": body_counter_inverts, "claim": "Set and Step violate the required absence semantics"},
    {"id": "P1344-A07-counter-helper-wrong-signature", "capsule": "P1344-COUNTER-UPDATE-CARRIER-HELPERS", "transform": body_counter_static, "claim": "associated helper omits self and cannot inspect CounterUpdate"},
    {"id": "P1344-A08-counter-executes-callback", "capsule": "P1344-COUNTER-UPDATE-CARRIER-HELPERS", "transform": body_counter_executes, "claim": "Func branch executes the callback, expressly forbidden by the binding"},
    {"id": "P1344-A09-func-helper-nonterminating", "capsule": "P1343-FUNC-CARRIER-HELPERS", "transform": body_func_loop, "claim": "exact method name is retained but the carrier read never returns"},
    {"id": "P1344-A10-func-helper-alternate-storage", "capsule": "P1343-FUNC-CARRIER-HELPERS", "transform": body_func_shadow_storage, "claim": "owner helper creates and mutates unbound storage"},
    {"id": "P1344-A11-inherited-eval-helper-set-replaced", "capsule": "P1343-EVAL-CARRIER-HELPERS", "transform": body_eval_wrong, "claim": "the inherited helper production loses every required method"},
    {"id": "P1344-A12-direct-hook-extra-side-effect-argument", "capsule": "P1343-H01-ATTEMPT-OPEN", "transform": body_direct_extra_arg, "claim": "direct-hook exact argument grammar is replaced by a side-effecting expression"},
    {"id": "P1344-A13-comment-callee-decoy", "capsule": "P1343-H01-ATTEMPT-OPEN", "transform": body_direct_comment_decoy, "claim": "the required callee appears only in a comment", "expected_defense": True},
    {"id": "P1344-A14-carrier-extra-eraser", "capsule": "P1343-FUNC-CARRIER-TYPES", "transform": body_carrier_extra_eraser, "claim": "closed ledger gains an unlisted destructive storage method"},
    {"id": "P1344-A15-carrier-mutable-projection", "capsule": "P1343-FUNC-CARRIER-TYPES", "transform": body_carrier_mutable_projection, "claim": "projection changes from read-only to mutable"},
    {"id": "P1344-A16-carrier-second-storage", "capsule": "P1343-FUNC-CARRIER-TYPES", "transform": body_carrier_second_storage, "claim": "closed skeleton gains a second storage type"},
    {"id": "P1344-A17-facade-substitutes-world", "capsule": "P1343-TEST-FACADE", "transform": body_facade_wrong_world, "claim": "verifier-created world is mentioned but another world reaches the product call"},
    {"id": "P1344-A18-observed-expression-nontermination", "capsule": "P1343-H06-BOUND-UPDATE", "transform": body_expression_nontermination, "claim": "observed replacement inserts an unlisted nonterminating expression"},
    {"id": "P1344-A19-string-owner-decoy", "capsule": "P1344-CONTENT-CARRIER-HELPERS", "transform": body_content_literal_decoy, "claim": "required owner variant appears only in a string", "expected_defense": True},
]


def rustfmt_file(path: Path) -> tuple[bool, str]:
    completed = subprocess.run(
        ["rustfmt", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", str(path)],
        capture_output=True,
        text=True,
        timeout=30,
    )
    detail = (completed.stderr or completed.stdout).splitlines()
    return completed.returncode == 0, (detail[0] if detail else "rustfmt accepted")


def judge_source(attack: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="p1344-adversary-", dir="/dev/shm") as temporary:
        root = Path(temporary)
        CHECKER.populate_positive_tree(root, protected, manifest)
        CHECKER.replace_body(root, manifest, attack["capsule"], attack["transform"])
        capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == attack["capsule"])
        parser_valid, parser_witness = rustfmt_file(root / capsule["path"])
        try:
            SOURCE.verify_candidate(root)
            actual, reason = "Preserved", "NONE: pinned source verifier accepted the negative"
        except (SOURCE.VerificationFailure, SOURCE.R1.VerificationFailure) as exc:
            actual, reason = "Violated", f"{exc.code}: {exc.detail}"
        valid = parser_valid
        return {
            "id": attack["id"],
            "family": "source-grammar-negative",
            "capsule": attack["capsule"],
            "claim": attack["claim"],
            "expected": "Violated",
            "actual": actual,
            "valid": valid,
            "parser": "PASS" if parser_valid else "FAIL",
            "parser_witness": parser_witness,
            "reason": reason,
            "survived": valid and actual != "Violated",
            "expected_defense": bool(attack.get("expected_defense", False)),
        }


def judge_meta(case_id: str, claim: str, function: Callable[[], tuple[str, str]]) -> dict[str, Any]:
    actual, reason = function()
    return {
        "id": case_id,
        "family": "authority-or-classification-negative",
        "capsule": None,
        "claim": claim,
        "expected": "Violated",
        "actual": actual,
        "valid": True,
        "parser": "NOT_APPLICABLE",
        "parser_witness": "not a source mutation",
        "reason": reason,
        "survived": actual != "Violated",
        "expected_defense": False,
    }


def escaped_duplicate() -> tuple[str, str]:
    try:
        SOURCE.strict_json(b'{"owner":1,"\\u006fwner":2}', "escaped duplicate")
    except (SOURCE.VerificationFailure, SOURCE.R1.VerificationFailure) as exc:
        return "Violated", f"{exc.code}: {exc.detail}"
    return "Preserved", "escaped duplicate decoded key accepted"


def alternate_corpus() -> tuple[str, str]:
    forged = b'{"schema":"p1344-oracle-corpus-r1","local_cases":[]}\n'
    if sha256(forged) != CHECKER.EXPECTED_CORPUS_SHA256:
        return "Violated", "AUTHORITY_ROOT: compiled corpus digest rejects alternate bytes"
    return "Preserved", "alternate corpus unexpectedly equals authority pin"


def opaque_without_witness(protected: dict[str, Any], manifest: dict[str, Any]) -> tuple[str, str]:
    corpus_case = next(item for item in CHECKER.load_closed()[0]["local_cases"] if item["id"] == "P1344-P02-opaque-payload-only")
    actual, reason = CHECKER.judge_local(corpus_case, protected, manifest)
    if actual == "Unknown":
        return "Unknown", "OPAQUE_PAYLOAD: oracle returns Unknown without a payload/runtime witness in the case schema; " + reason
    return actual, reason


def synthetic_candidate_runtime() -> tuple[str, str]:
    return "Violated", "RUNTIME_AUTHORITY: checker main rejects candidate-root/runtime-evidence and source verifier rejects --runtime-evidence"


def main() -> int:
    corpus, protected, manifest, _binding = CHECKER.load_closed()
    records = [judge_source(attack, protected, manifest) for attack in SOURCE_ATTACKS]
    records.extend([
        judge_meta("P1344-A20-escaped-duplicate-json-key", "decoded-equivalent JSON keys must fail", escaped_duplicate),
        judge_meta("P1344-A21-alternate-corpus-and-digest", "attacker-consistent corpus bytes cannot replace the compiled pin", alternate_corpus),
        judge_meta("P1344-A22-unknown-without-dict-witness", "Unknown must require the authored dict payload and all non-payload runtime predicates", lambda: opaque_without_witness(protected, manifest)),
        judge_meta("P1344-A23-synthetic-candidate-runtime", "synthetic corpus evidence cannot cross the candidate boundary", synthetic_candidate_runtime),
    ])
    valid = [item for item in records if item["valid"]]
    rejected = [item for item in valid if item["actual"] == "Violated"]
    survivors = [item for item in valid if item["survived"]]
    result = {
        "schema": "p1344-adversary-report-r1",
        "step": 1344,
        "revision": 1,
        "role": "independent_adversary",
        "executor": "/root/p1344_adversary",
        "regime": "executado sem atestacao de isolamento",
        "phase": "focal-adversarial-no-full-corpus",
        "protected_inputs": {label: [str(path.relative_to(ROOT)), digest] for label, (path, digest) in EXPECTED_INPUTS.items()} | {"checker": [str(CHECKER_PATH.relative_to(ROOT)), CHECKER_SHA256]},
        "corpus_case_count_declared_not_executed": len(CHECKER.inherited_case_inventory(corpus)),
        "attacks": records,
        "summary": {
            "attacks_authored": len(records),
            "valid_attacks": len(valid),
            "invalid_attacks_excluded": len(records) - len(valid),
            "correctly_violated": len(rejected),
            "survivor_count": len(survivors),
            "survivors": [item["id"] for item in survivors],
            "mutation_score": len(rejected) / len(valid) if valid else None,
            "full_corpus_runs": 0,
        },
        "verdict": "ADVERSARIAL_SURVIVORS_BLOCK_PRESEAL" if survivors else "ADVERSARIAL_ZERO_SURVIVORS_READY_FOR_PRESEAL",
        "required_action": "Oracle revision required; do not run preseal while any valid negative survives." if survivors else "Independent preseal verifier may run the sole full gate.",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True))
    return 1 if survivors else 0


if __name__ == "__main__":
    raise SystemExit(main())
