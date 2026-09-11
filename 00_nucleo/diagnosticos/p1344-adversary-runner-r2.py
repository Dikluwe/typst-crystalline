#!/usr/bin/env python3
"""Independent focal adversary for the pinned P1344 oracle R2.

Replays the 23 protected R1 attacks through the R2 judge and adds new
parser-valid negatives.  It never calls the 168-case full route and creates
synthetic source trees only under /dev/shm.
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
CHECKER_PATH = DIAG / "p1344-oracle-checker-r2.py"
CHECKER_SHA256 = "62a9ebb7241fc6b2ff1495417b5a5f8b3d4ad547dc2e770cba6da8c60e118d5f"
EXPECTED_INPUTS = {
    "corpus": (DIAG / "p1344-oracle-corpus-r2.json", "1246920352da5678603dadcd07efd2f1e2364e2e397d2d26fdd36b8b7ef757e5"),
    "source": (DIAG / "p1344-source-verifier-r2.py", "62824f399b34659b81a2ab6ff538ad0747b4bb76aed6c5af4cfc78fd17a20e82"),
    "authorship": (DIAG / "p1344-oracle-authorship-r2.md", "446ff183bf9531a563360395b7a27002968045f2fb9ccaf8d636e7a8b3f0c419"),
    "authorship_receipt": (DIAG / "p1344-oracle-authorship-receipt-r2.json", "74dd492dce9b950edab222b19cee8038240cce095dead726ce1dd8f91fce0ab4"),
    "contract": (DIAG / "p1344-contract-spec-r1.json", "157877514d350947f6483ec371c356dd2181c2e0f98e8fa9eb5e465527983279"),
    "binding": (DIAG / "p1344-contract-binding-r1.json", "1f4d8812d0a4e2a9451d2f8c08931fb71e1baa54a3484a411d2cf76727115bab"),
    "manifest": (DIAG / "p1344-authority-manifest.json", "9a3cafa60d0dcffb395bb1bda5c823ed5b1881ee8239fe06d8c6cf7614d342d1"),
    "r1_adversary_receipt": (DIAG / "p1344-adversary-receipt-r1.json", "430fd1ba9bdae3adcac49e3b775c4540c2ad106627c2172c1008d34afdc982b9"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_checker() -> Any:
    if sha256(CHECKER_PATH.read_bytes()) != CHECKER_SHA256:
        raise RuntimeError("PROTECTED_INPUT: R2 checker hash mismatch")
    for label, (path, expected) in EXPECTED_INPUTS.items():
        if sha256(path.read_bytes()) != expected:
            raise RuntimeError(f"PROTECTED_INPUT: {label} hash mismatch")
    spec = importlib.util.spec_from_file_location("p1344_checker_for_adversary_r2", CHECKER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("PROTECTED_INPUT: cannot import R2 checker")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


CHECKER = load_checker()
SOURCE = CHECKER.SOURCE
R1 = CHECKER.R1


def func_observer_decoy(body: bytes) -> bytes:
    old = b"{ if let Some(carrier) = self.p1343_carrier() { carrier.append(event); } }"
    new = b"{ let (p1343_carrier, append, _) = (0, 0, event); }"
    return body.replace(old, new, 1)


def func_foreign_receiver(body: bytes) -> bytes:
    return body.replace(b"self.p1343_carrier()", b"other.p1343_carrier()", 1)


def eval_static_signatures(body: bytes) -> bytes:
    return body.replace(b"(&mut self)", b"()")


def eval_shadow_storage(body: bytes) -> bytes:
    return body.replace(b"(&mut self) {}", b"(&mut self) { let mut shadow = Vec::<u8>::new(); shadow.pop(); }", 1)


def carrier_alias_storage(body: bytes) -> bytes:
    body = b"#[cfg(p1339_observation)]\ntype P1343ShadowStorage = Option<Box<[u8]>>;\n" + body
    body = body.replace(
        b"struct P1343Carrier { state: P1343LedgerState }",
        b"struct P1343Carrier { state: P1343LedgerState, shadow: P1343ShadowStorage }",
        1,
    )
    return body.replace(b"Self { state: P1343LedgerState", b"Self { shadow: None, state: P1343LedgerState", 1)


def carrier_snapshot_remove(body: bytes) -> bytes:
    return body.replace(b"self.state.frozen = true;", b"self.state.events.remove(0); self.state.frozen = true;", 1)


def carrier_dead_writer(body: bytes) -> bytes:
    old = b"self.state.events.push(event); self.state.receipts.push(receipt);"
    new = b"let _dead = || { self.state.events.push(event); self.state.receipts.push(receipt); };"
    return body.replace(old, new, 1)


def carrier_conditional_event(body: bytes) -> bytes:
    return body.replace(b"self.state.events.push(event);", b"if false { self.state.events.push(event); }", 1)


def carrier_swap_remove(body: bytes) -> bytes:
    return body.replace(b"self.state.frozen = true;", b"self.state.events.swap_remove(0); self.state.frozen = true;", 1)


def facade_lateral_call(body: bytes) -> bytes:
    return body.replace(b" let raw =", b" side_effect();\n let raw =", 1)


def facade_fabricated_projection(body: bytes) -> bytes:
    return body.replace(b"let projection = raw.project();\n (raw, projection)", b"let _ignored = raw.project();\n (raw, P1343Projection)", 1)


def facade_dead_delegate(_: bytes) -> bytes:
    return CHECKER.cfg("all(test,p1339_observation)") + (
        b"fn p1342_run_fixture_for_test(world: &World, source: Source, challenge: [u8; 32], mode: P1343Mode) -> (P1343RawSnapshot, P1343Projection) {\n"
        b" let _dead = || run_context_stabilization_once(world, source, challenge, mode);\n"
        b" let raw = P1343RawSnapshot { events: Vec::new(), receipts: Vec::new() };\n"
        b" let projection = raw.project();\n"
        b" (raw, projection)\n}\n"
    )


def replacement_dead_observation(body: bytes) -> bytes:
    old = b'p1343_append_raw("H06", p1343_event());'
    new = b'if false { p1343_append_raw("H06", p1343_event()); }'
    return body.replace(old, new, 1)


def owner_parenthesized_equivalent(body: bytes) -> bytes:
    return body.replace(b"match self", b"side_effect(); (match self", 1).replace(b"_ => None, }", b"_ => None, })", 1)


NEW_SOURCE_ATTACKS: list[dict[str, Any]] = [
    {"id":"P1344-R2A01-func-observer-token-decoys","capsule":"P1343-FUNC-CARRIER-HELPERS","transform":func_observer_decoy,"cause":"Func observation effect is inferred from three identifier counts rather than a call AST"},
    {"id":"P1344-R2A02-func-observer-foreign-receiver","capsule":"P1343-FUNC-CARRIER-HELPERS","transform":func_foreign_receiver,"cause":"p1343_carrier is called once on an unbound foreign receiver"},
    {"id":"P1344-R2A03-eval-exact-names-static-signatures","capsule":"P1343-EVAL-CARRIER-HELPERS","transform":eval_static_signatures,"cause":"Eval helper validation checks names but not receiver/signature"},
    {"id":"P1344-R2A04-eval-exact-names-shadow-storage","capsule":"P1343-EVAL-CARRIER-HELPERS","transform":eval_shadow_storage,"cause":"Eval helper validation omits lateral storage and pop from its production"},
    {"id":"P1344-R2A05-carrier-aliased-non-vec-storage","capsule":"P1343-FUNC-CARRIER-TYPES","transform":carrier_alias_storage,"cause":"carrier storage cardinality counts only literal colon-Vec fields and ignores an aliased field"},
    {"id":"P1344-R2A06-carrier-remove-outside-append","capsule":"P1343-FUNC-CARRIER-TYPES","transform":carrier_snapshot_remove,"cause":"snapshot mutates raw events through remove, an unlisted writer absent from the blacklist"},
    {"id":"P1344-R2A07-carrier-pushes-in-dead-closure","capsule":"P1343-FUNC-CARRIER-TYPES","transform":carrier_dead_writer,"cause":"two push tokens occur only in a never-called closure"},
    {"id":"P1344-R2A08-carrier-conditional-event-loss","capsule":"P1343-FUNC-CARRIER-TYPES","transform":carrier_conditional_event,"cause":"the event push is guarded by false while token cardinality stays exact"},
    {"id":"P1344-R2A09-carrier-swap-remove-spelling","capsule":"P1343-FUNC-CARRIER-TYPES","transform":carrier_swap_remove,"cause":"swap_remove bypasses the exact-token blacklist entry swap"},
    {"id":"P1344-R2A10-facade-unlisted-lateral-call","capsule":"P1343-TEST-FACADE","transform":facade_lateral_call,"cause":"facade forwards the four inputs once but performs an extra call"},
    {"id":"P1344-R2A11-facade-fabricated-return-projection","capsule":"P1343-TEST-FACADE","transform":facade_fabricated_projection,"cause":"raw.project is called once but its result is discarded and a fresh unit projection is returned"},
    {"id":"P1344-R2A12-facade-product-call-dead-closure","capsule":"P1343-TEST-FACADE","transform":facade_dead_delegate,"cause":"the exact product call and arguments occur once inside a never-called closure"},
    {"id":"P1344-R2A13-replacement-hook-under-if-false","capsule":"P1343-H06-BOUND-UPDATE","transform":replacement_dead_observation,"cause":"replacement control rejects loops but permits a dead if-false observation"},
    {"id":"P1344-R2A14-owner-parenthesized-with-lateral-call","capsule":"P1344-CONTENT-CARRIER-HELPERS","transform":owner_parenthesized_equivalent,"cause":"parenthesized equivalent match plus an unlisted lateral call must not evade the closed production","expected_defense":True}
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


def judge_new_source(attack: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="p1344-adversary-r2-", dir="/dev/shm") as temporary:
        root = Path(temporary)
        CHECKER.populate_positive_tree(root, protected, manifest)
        R1.replace_body(root, manifest, attack["capsule"], attack["transform"])
        capsule = next(item for item in manifest["capsules"] if item["capsule_id"] == attack["capsule"])
        parser_valid, parser_witness = rustfmt_file(root / capsule["path"])
        try:
            SOURCE.verify_candidate(root)
            actual, reason = "Preserved", "NONE: pinned R2 source verifier accepted the valid negative"
        except (SOURCE.VerificationFailure, SOURCE.R1.R1.VerificationFailure) as exc:
            actual, reason = "Violated", f"{exc.code}: {exc.detail}"
        return {
            "id":attack["id"], "origin":"new-r2", "family":"source-grammar-negative",
            "capsule":attack["capsule"], "claim":attack["cause"], "expected":"Violated",
            "actual":actual, "valid":parser_valid, "parser":"PASS" if parser_valid else "FAIL",
            "parser_witness":parser_witness, "reason":reason,
            "survived":parser_valid and actual != "Violated",
            "expected_defense":bool(attack.get("expected_defense",False)),
        }


def replay_r1(case: dict[str, Any], corpus: dict[str, Any], protected: dict[str, Any], manifest: dict[str, Any]) -> dict[str, Any]:
    actual, reason = CHECKER.judge(case, corpus, protected, manifest)
    return {
        "id":case["id"], "origin":"r1-replay", "family":case["class"], "capsule":None,
        "claim":"protected R1 adversarial replay", "expected":"Violated", "actual":actual,
        "valid":actual != "InvalidNegative", "parser":"CHECKER_R2", "parser_witness":"R2 replay harness",
        "reason":reason, "survived":actual not in ("Violated","InvalidNegative"), "expected_defense":False,
    }


def unknown_self_attestation(corpus: dict[str, Any]) -> tuple[str, str]:
    keys = corpus["unknown_policy"]["required_witness_keys"]
    values = corpus["unknown_policy"]["exact_values"]
    forged = {key:value for key,value in zip(keys,values)}
    actual, reason = CHECKER.validate_opaque({"id":"P1344-R2A15-unknown-canonical-self-attestation","opaque_witness":forged}, corpus)
    return actual, "No source/runtime predicate was executed by the caller; " + reason


def escaped_duplicate() -> tuple[str, str]:
    try:
        SOURCE.R1.strict_json(b'{"root":1,"\\u0072oot":2}', "R2 escaped duplicate")
    except SOURCE.VerificationFailure as exc:
        return "Violated", f"{exc.code}: {exc.detail}"
    return "Preserved", "decoded-equivalent duplicate key accepted"


def alternate_corpus() -> tuple[str, str]:
    forged = b'{"schema":"p1344-oracle-corpus-r2","controls":[],"attacks":[]}\n'
    return ("Violated", "AUTHORITY_ROOT: compiled R2 corpus rejects alternate bytes") if sha256(forged) != CHECKER.EXPECTED_CORPUS_SHA256 else ("Preserved", "alternate corpus equals compiled root")


def meta_record(case_id: str, claim: str, result: tuple[str,str]) -> dict[str, Any]:
    actual, reason = result
    return {
        "id":case_id,"origin":"new-r2","family":"authority-or-classification-negative","capsule":None,
        "claim":claim,"expected":"Violated","actual":actual,"valid":True,"parser":"NOT_APPLICABLE",
        "parser_witness":"not a source mutation","reason":reason,"survived":actual!="Violated","expected_defense":False,
    }


def main() -> int:
    corpus, protected, manifest = CHECKER.load_closed()
    records = [replay_r1(case, corpus, protected, manifest) for case in corpus["attacks"]]
    records.extend(judge_new_source(attack, protected, manifest) for attack in NEW_SOURCE_ATTACKS)
    records.extend([
        meta_record("P1344-R2A15-unknown-canonical-self-attestation", "canonical witness values assert Preserved without independent source/runtime evidence", unknown_self_attestation(corpus)),
        meta_record("P1344-R2A16-escaped-duplicate-json-key", "decoded-equivalent duplicate keys must fail", escaped_duplicate()),
        meta_record("P1344-R2A17-alternate-corpus-root", "attacker corpus/hash cannot replace the compiled R2 root", alternate_corpus()),
    ])
    valid = [item for item in records if item["valid"]]
    rejected = [item for item in valid if item["actual"]=="Violated"]
    survivors = [item for item in valid if item["survived"]]
    new_records = [item for item in records if item["origin"]=="new-r2"]
    result = {
        "schema":"p1344-adversary-report-r2","step":1344,"revision":2,
        "role":"independent_adversary","executor":"/root/p1344_adversary",
        "regime":"executado sem atestacao de isolamento","phase":"focal-adversarial-r2-no-full-corpus",
        "protected_inputs":{label:[str(path.relative_to(ROOT)),digest] for label,(path,digest) in EXPECTED_INPUTS.items()}|{"checker":[str(CHECKER_PATH.relative_to(ROOT)),CHECKER_SHA256]},
        "corpus_case_count_declared_not_executed":168,"attacks":records,
        "summary":{
            "r1_attacks_replayed":23,"new_r2_attacks":len(new_records),"attacks_total":len(records),
            "valid_attacks":len(valid),"invalid_attacks_excluded":len(records)-len(valid),
            "correctly_violated":len(rejected),"survivor_count":len(survivors),
            "survivors":[item["id"] for item in survivors],
            "mutation_score":len(rejected)/len(valid) if valid else None,"full_corpus_runs":0,
        },
        "verdict":"ADVERSARIAL_R2_SURVIVORS_BLOCK_PRESEAL" if survivors else "ADVERSARIAL_R2_ZERO_SURVIVORS_READY_FOR_PRESEAL",
        "required_action":"A distinct oracle authority must address the public R2 causes before preseal." if survivors else "A distinct preseal authority may run the sole full gate.",
    }
    print(json.dumps(result,ensure_ascii=False,indent=2,sort_keys=True))
    return 1 if survivors else 0


if __name__=="__main__":
    raise SystemExit(main())
