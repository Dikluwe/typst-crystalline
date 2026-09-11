#!/usr/bin/env python3
"""Final focal P1342 oracle revision over the R2 composite.

R3 preserves the R2 implementation and adds only the nine missing boundaries:
runtime-tail reconciliation, challenge-issuer resolution, full base order, and
run/ledger/Func endpoint continuity before the opacity boundary.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
R2_CHECKER = ROOT / "00_nucleo/diagnosticos/p1342-oracle-checker-r2.py"
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
ISSUER_PREFIX = b"P1342-CHALLENGE-ISSUER-V3\0"
HOOK_PREFIX = b"P1342-HOOK-RECEIPT-V3\0"
MODES = ["normal", "repeat", "reverse"]


def load_r2() -> Any:
    spec = importlib.util.spec_from_file_location("p1342_oracle_checker_r2_frozen", R2_CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen R2 checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


R2 = load_r2()
Failure = R2.Failure
digest = R2.digest
make_id = R2.make_id
receipt_hash = R2.receipt_hash


def exact_keys(value: Any, keys: set[str], where: str) -> None:
    if not isinstance(value, dict) or set(value) != keys:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        raise Failure(f"{where}: closed keys {sorted(keys)} required, got {got}")


def integer(value: Any, where: str) -> int:
    if type(value) is not int:
        raise Failure(f"{where}: exact integer required; bool is not int")
    return value


def issuer_receipt(mode: str, challenge_hex: str, commitment: str, nonce_hex: str, previous: str | None) -> str:
    if mode not in MODES or not HEX64.fullmatch(challenge_hex) or not HEX64.fullmatch(commitment) or not HEX64.fullmatch(nonce_hex):
        raise Failure("issuer receipt input schema mismatch")
    previous_bytes = bytes(32) if previous is None else bytes.fromhex(previous)
    payload = ISSUER_PREFIX + mode.encode() + bytes.fromhex(challenge_hex) + bytes.fromhex(commitment) + bytes.fromhex(nonce_hex) + previous_bytes
    return hashlib.sha256(payload).hexdigest()


def hook_receipt(challenge_hex: str, origin_cell: str, hook: str, count: int, raw_tail: str | None, raw_digest: str) -> str:
    if not HEX64.fullmatch(challenge_hex) or not HEX64.fullmatch(raw_digest):
        raise Failure("hook receipt challenge/raw digest malformed")
    raw_tail_bytes = bytes(32) if raw_tail is None else bytes.fromhex(raw_tail)
    payload = HOOK_PREFIX + bytes.fromhex(challenge_hex) + origin_cell.encode() + b"\0" + hook.encode() + count.to_bytes(8, "big") + raw_tail_bytes + bytes.fromhex(raw_digest)
    return hashlib.sha256(payload).hexdigest()


def raw_tail_map(run: dict[str, Any]) -> dict[tuple[str, str], str]:
    values: dict[tuple[str, str], str] = {}
    for receipt in run["raw_snapshot"]["append_receipts"]:
        values[(receipt["origin_cell"], receipt["hook"])] = receipt["receipt"]
    return values


def enhance_evidence(dto: dict[str, Any], evidence: dict[str, Any]) -> None:
    previous_issuer = None
    runtime_by_mode = {item["mode"]: item for item in evidence["runtime_runs"]}
    for run in dto["runs"]:
        runtime = runtime_by_mode[run["mode"]]
        nonce = hashlib.sha256(("p1342-r3-driver-nonce:" + run["mode"]).encode()).hexdigest()
        current = issuer_receipt(run["mode"], run["challenge"], run["challenge_commitment"], nonce, previous_issuer)
        issuer = {
            "schema": "p1342-challenge-issuer-receipt-v3",
            "mode": run["mode"],
            "challenge_sha256": hashlib.sha256(bytes.fromhex(run["challenge"])).hexdigest(),
            "challenge_commitment": run["challenge_commitment"],
            "driver_nonce": nonce,
            "previous_issuer_receipt": previous_issuer,
            "receipt": current,
        }
        runtime["challenge_issuer"] = issuer
        runtime["challenge_issuer_receipt_sha256"] = digest(issuer)
        tails = raw_tail_map(run)
        hook_receipts = []
        for hit in runtime["hook_hits"]:
            raw_tail = tails.get((hit["origin_cell"], hit["hook"]))
            resolved = hook_receipt(run["challenge"], hit["origin_cell"], hit["hook"], hit["count"], raw_tail, run["raw_snapshot"]["raw_digest"])
            hook_receipts.append({
                "hook": hit["hook"],
                "origin_cell": hit["origin_cell"],
                "count": hit["count"],
                "raw_event_receipt_tail": raw_tail,
                "receipt": resolved,
            })
            hit["receipt_tail"] = resolved if hit["count"] > 0 else None
        runtime["hook_receipts"] = hook_receipts
        previous_issuer = current


def stripped_r2_evidence(evidence: dict[str, Any]) -> dict[str, Any]:
    value = copy.deepcopy(evidence)
    for runtime in value["runtime_runs"]:
        runtime.pop("challenge_issuer")
        runtime.pop("hook_receipts")
    return value


def validate_issuer_chain(dto: dict[str, Any], evidence: dict[str, Any]) -> None:
    runtime_by_mode = {item["mode"]: item for item in evidence["runtime_runs"]}
    previous = None
    seen_nonces: set[str] = set()
    for run in dto["runs"]:
        runtime = runtime_by_mode[run["mode"]]
        issuer = runtime["challenge_issuer"]
        exact_keys(issuer, {"schema", "mode", "challenge_sha256", "challenge_commitment", "driver_nonce", "previous_issuer_receipt", "receipt"}, "challenge issuer receipt")
        if issuer["schema"] != "p1342-challenge-issuer-receipt-v3" or issuer["mode"] != run["mode"]:
            raise Failure("challenge issuer receipt schema/mode mismatch")
        if issuer["challenge_sha256"] != hashlib.sha256(bytes.fromhex(run["challenge"])).hexdigest() or issuer["challenge_commitment"] != run["challenge_commitment"]:
            raise Failure("challenge issuer receipt does not resolve the supplied challenge")
        if not isinstance(issuer["driver_nonce"], str) or not HEX64.fullmatch(issuer["driver_nonce"]) or issuer["driver_nonce"] in seen_nonces:
            raise Failure("challenge issuer nonce must be closed, fresh and externally supplied")
        seen_nonces.add(issuer["driver_nonce"])
        expected = issuer_receipt(run["mode"], run["challenge"], run["challenge_commitment"], issuer["driver_nonce"], previous)
        if issuer["previous_issuer_receipt"] != previous or issuer["receipt"] != expected:
            raise Failure("challenge issuer receipt formula/chain mismatch")
        if runtime["challenge_issuer_receipt_sha256"] != digest(issuer):
            raise Failure("challenge issuer receipt reference is unresolved")
        previous = expected


def validate_hook_receipts(dto: dict[str, Any], evidence: dict[str, Any]) -> None:
    run_by_mode = {run["mode"]: run for run in dto["runs"]}
    for runtime in evidence["runtime_runs"]:
        run = run_by_mode[runtime["mode"]]
        if not isinstance(runtime["hook_receipts"], list) or len(runtime["hook_receipts"]) != len(runtime["hook_hits"]):
            raise Failure("one closed hook receipt per runtime hook hit is required")
        tails = raw_tail_map(run)
        receipts = {(item.get("origin_cell"), item.get("hook")): item for item in runtime["hook_receipts"] if isinstance(item, dict)}
        hits = {(item.get("origin_cell"), item.get("hook")): item for item in runtime["hook_hits"] if isinstance(item, dict)}
        if set(receipts) != set(hits) or len(receipts) != len(runtime["hook_receipts"]):
            raise Failure("runtime hook receipts/hits do not reconcile one-to-one by hook and cell")
        for key, hit in hits.items():
            item = receipts[key]
            exact_keys(item, {"hook", "origin_cell", "count", "raw_event_receipt_tail", "receipt"}, "runtime hook receipt")
            count = integer(hit["count"], "runtime hook hit count")
            if integer(item["count"], "runtime hook receipt count") != count:
                raise Failure("runtime hook receipt count differs from hook hit count")
            raw_tail = tails.get(key)
            if item["raw_event_receipt_tail"] != raw_tail:
                raise Failure("runtime hook receipt tail is not reconciled with raw hook/cell chain")
            expected = hook_receipt(run["challenge"], key[0], key[1], count, raw_tail, run["raw_snapshot"]["raw_digest"])
            if item["receipt"] != expected:
                raise Failure("runtime hook receipt formula mismatch")
            expected_tail = expected if count > 0 else None
            if hit["receipt_tail"] != expected_tail:
                raise Failure("runtime receipt_tail is not the resolved hook/cell receipt")


DISCOVERY_ORDER = [
    "attempt-open", "context-dispatch", "dict-prebound-produced",
    "counter-occurrence-created", "attempt-result", "attempt-close",
]
SELECTED_ORDER = [
    "attempt-open", "context-dispatch", "dict-prebound-produced",
    "counter-occurrence-created", "attempt-result", "counter-occurrence-walked",
    "callback-replay-enter", "func-dispatch-with", "with-edge",
    "func-dispatch-inner", "syntax-body-enter", "dict-witness-produced",
    "syntax-body-exit", "callback-replay-exit", "attempt-close",
]


def one(events: list[dict[str, Any]], kind: str) -> dict[str, Any]:
    values = [event for event in events if event.get("kind") == kind]
    if len(values) != 1:
        raise Failure(f"{kind}: expected exactly one event")
    return values[0]


def validate_full_order_and_endpoints(dto: dict[str, Any]) -> None:
    for run in dto["runs"]:
        for cell in run["raw_snapshot"]["cells"]:
            events = cell["events"]
            attempt_kind = one(events, "attempt-open")["data"]["attempt_kind"]
            expected_order = DISCOVERY_ORDER if attempt_kind == "discovery" else SELECTED_ORDER
            if [event.get("kind") for event in events] != expected_order:
                raise Failure("complete base causal order mismatch")
            open_refs = events[0]["refs"]
            for event in events:
                refs = event.get("refs", {})
                if refs.get("run") != open_refs["run"] or refs.get("ledger") != open_refs["ledger"]:
                    raise Failure("run/ledger endpoint continuity lost inside origin cell")
            created = one(events, "counter-occurrence-created")["refs"]
            if attempt_kind == "selected":
                dispatch_with = one(events, "func-dispatch-with")["refs"]
                edge = one(events, "with-edge")["refs"]
                dispatch_inner = one(events, "func-dispatch-inner")["refs"]
                body_enter = one(events, "syntax-body-enter")["refs"]
                body_exit = one(events, "syntax-body-exit")["refs"]
                if not (created["callback_with"] == dispatch_with["callback_with"] == edge["callback_with"]):
                    raise Failure("created callback_with endpoint is disconnected from dispatch/With")
                if not (created["func_callback"] == edge["func_callback"] == dispatch_inner["func_callback"] == body_enter["func_callback"] == body_exit["func_callback"]):
                    raise Failure("created func_callback endpoint is disconnected from dispatch/body")


def judge(dto: Any, evidence: Any, trusted_evidence_sha256: str, contract: dict[str, Any], manifest: dict[str, Any], pins: dict[str, str]) -> str:
    # Exact external evidence pin precedes every candidate-provided statement.
    if digest(evidence) != trusted_evidence_sha256:
        raise Failure("external evidence pin mismatch")
    try:
        validate_issuer_chain(dto, evidence)
        validate_hook_receipts(dto, evidence)
        validate_full_order_and_endpoints(dto)
    except (KeyError, IndexError, TypeError, StopIteration) as exc:
        raise Failure(f"R3 closed composite evidence/endpoint schema mismatch: {type(exc).__name__}") from exc
    # R2 then validates pins, all closed raw/projection/event schemas, challenge/ID
    # disjointness, receipt chain, carriers, Location/snapshots, R and values.
    stripped = stripped_r2_evidence(evidence)
    return R2.judge(dto, stripped, digest(stripped), contract, manifest, pins)


def selected_events(run: dict[str, Any]) -> list[dict[str, Any]]:
    return next(cell["events"] for cell in run["raw_snapshot"]["cells"] if cell["events"][0]["data"]["attempt_kind"] == "selected")


def event_of(events: list[dict[str, Any]], kind: str) -> dict[str, Any]:
    return one(events, kind)


def rebuild_run(run: dict[str, Any]) -> None:
    challenge = bytes.fromhex(run["challenge"])
    flattened = []
    seq = 0
    for cell in run["raw_snapshot"]["cells"]:
        for value in cell["events"]:
            value["seq"] = seq
            seq += 1
            flattened.append((cell["origin_cell"], value))
        event_of(cell["events"], "attempt-close")["data"]["final_event_count"] = len(cell["events"])
    receipts = []
    previous = None
    hits: dict[tuple[str, str], int] = defaultdict(int)
    for index, (origin, value) in enumerate(flattened):
        hits[(origin, value["hook"])] += 1
        event_hash = digest(value)
        current = receipt_hash(challenge, index, previous, value["hook"], event_hash)
        receipts.append({"seq": index, "hook": value["hook"], "hook_hit": hits[(origin, value["hook"])], "origin_cell": origin, "event_digest": event_hash, "prev_receipt": previous, "receipt": current})
        previous = current
    raw = run["raw_snapshot"]
    raw["append_receipts"] = receipts
    raw["raw_digest"] = digest({key: value for key, value in raw.items() if key != "raw_digest"})
    run["projection"] = {"schema": "p1342-ledger-projection-v2", "challenge_commitment": run["challenge_commitment"], "raw_digest_before": raw["raw_digest"], "raw_digest_after": raw["raw_digest"], "cells": copy.deepcopy(raw["cells"])}


def refresh_evidence(dto: dict[str, Any], evidence: dict[str, Any]) -> None:
    runtime_by_mode = {item["mode"]: item for item in evidence["runtime_runs"]}
    for run in dto["runs"]:
        runtime = runtime_by_mode[run["mode"]]
        runtime["raw_digest_before"] = run["raw_snapshot"]["raw_digest"]
        runtime["raw_digest_after"] = run["raw_snapshot"]["raw_digest"]
        runtime.pop("challenge_issuer", None)
        runtime.pop("hook_receipts", None)
    enhance_evidence(dto, evidence)


def apply_x(case_id: str, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any], trusted: str) -> tuple[str, bool]:
    del manifest
    recompute = True
    if case_id == "X01-evidence-hash-out-of-band-mismatch":
        evidence["source_inspection"]["candidate_tree_sha256"] = "0" * 64
        return trusted, False
    if case_id == "X02-evidence-closed-schema-extra": evidence["forged"] = True
    elif case_id == "X03-anchor-fixture-coverage-false": evidence["source_inspection"]["anchors"][5]["fixture_reached"] = False
    elif case_id == "X04-anchor-inspection-status-false": evidence["source_inspection"]["anchors"][14]["status"] = "text-only"
    elif case_id == "X05-runtime-receipt-tail-fabricated":
        for runtime in evidence["runtime_runs"]: next(x for x in runtime["hook_hits"] if x["hook"] == "H12A-replay-enter" and x["count"] == 1)["receipt_tail"] = "0" * 64
    elif case_id == "X06-h00d-receipt-tail-fabricated":
        for runtime in evidence["runtime_runs"]: next(x for x in runtime["hook_hits"] if x["hook"] == "H00D-attempt-kind-discovery")["receipt_tail"] = "f" * 64
    elif case_id == "X07-h00d-h00s-both-live":
        for runtime in evidence["runtime_runs"]:
            item = next(x for x in runtime["hook_hits"] if x["hook"] == "H00D-attempt-kind-discovery")
            runtime["hook_hits"].append({"hook": "H00S-attempt-kind-selected", "origin_cell": item["origin_cell"], "count": 1, "receipt_tail": "e" * 64})
    elif case_id == "X08-challenge-not-pairwise-fresh": dto["runs"][1]["challenge"] = dto["runs"][0]["challenge"]
    elif case_id == "X09-challenge-issuer-receipt-fabricated":
        for runtime in evidence["runtime_runs"]: runtime["challenge_issuer_receipt_sha256"] = "0" * 64
    elif case_id == "X10-append-event-digest-corrupt": dto["runs"][0]["raw_snapshot"]["append_receipts"][3]["event_digest"] = "0" * 64
    elif case_id == "X11-base-causal-order-resealed":
        for run in dto["runs"]:
            events = selected_events(run); events[1], events[2] = events[2], events[1]; rebuild_run(run)
        refresh_evidence(dto, evidence)
    elif case_id == "X12-raw-freeze-flag-false": dto["runs"][0]["raw_snapshot"]["frozen_before_projection"] = False
    elif case_id == "X13-runtime-raw-after-differs": evidence["runtime_runs"][0]["raw_digest_after"] = "0" * 64
    elif case_id == "X14-projection-not-equal-raw": selected_events({"raw_snapshot": {"cells": dto["runs"][0]["projection"]["cells"]}})[0]["data"]["attempt_index"] = 99
    elif case_id in {"X15-created-callback-with-disconnected", "X16-created-func-callback-disconnected", "X17-run-endpoint-disconnected", "X18-ledger-endpoint-disconnected", "X22-unknown-before-created-endpoint-proof"}:
        for run in dto["runs"]:
            events = selected_events(run)
            if case_id == "X15-created-callback-with-disconnected": event_of(events, "counter-occurrence-created")["refs"]["callback_with"] = make_id("callback-with", run["challenge_commitment"], "disconnected-at-creation")
            elif case_id in {"X16-created-func-callback-disconnected", "X22-unknown-before-created-endpoint-proof"}: event_of(events, "counter-occurrence-created")["refs"]["func_callback"] = make_id("func-callback", run["challenge_commitment"], "disconnected-at-creation")
            elif case_id == "X17-run-endpoint-disconnected": event_of(events, "syntax-body-enter")["refs"]["run"] = make_id("run", run["challenge_commitment"], "disconnected")
            elif case_id == "X18-ledger-endpoint-disconnected": event_of(events, "syntax-body-exit")["refs"]["ledger"] = make_id("ledger", run["challenge_commitment"], "disconnected")
            if case_id == "X22-unknown-before-created-endpoint-proof":
                event_of(events, "dict-witness-produced")["data"]["typed_value"] = {"type": "dict", "visibility": "opaque", "commitment": "3f5b8f852dd5bba8fa19de766caf42ef54f0e060ca9faf9e0a82911926d86eb6"}
                run["classification"] = "Unknown"
            rebuild_run(run)
        refresh_evidence(dto, evidence)
    elif case_id == "X19-r-external-count-false":
        for runtime in evidence["runtime_runs"]:
            item = next(x for x in runtime["hook_hits"] if x["hook"] == "H12A-replay-enter" and x["count"] == 1); item["count"] = 0; item["receipt_tail"] = None
    elif case_id == "X20-extra-known-event":
        for run in dto["runs"]:
            events = selected_events(run); events.insert(-1, copy.deepcopy(event_of(events, "dict-witness-produced"))); rebuild_run(run)
        refresh_evidence(dto, evidence)
    elif case_id == "X21-bool-as-runtime-int": evidence["runtime_runs"][0]["hook_hits"][0]["count"] = True
    else: raise Failure(f"unknown R3 X case {case_id}")
    return digest(evidence) if recompute else trusted, True


def build_case(case_id: str, corpus: dict[str, Any], manifest: dict[str, Any], pins: dict[str, str]) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any], str]:
    base_id = case_id if case_id.startswith("A") or case_id.startswith("P") else "P01-inspectable-composite"
    dto, evidence, local_manifest, _ = R2.build_case(base_id, corpus, manifest, pins)
    enhance_evidence(dto, evidence)
    trusted = digest(evidence)
    if case_id.startswith("X"):
        trusted, _ = apply_x(case_id, dto, evidence, local_manifest, trusted)
    return dto, evidence, local_manifest, trusted


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--contract", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    args = parser.parse_args()
    contract = json.loads(args.contract.read_text(encoding="utf-8"))
    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    corpus = json.loads(args.corpus.read_text(encoding="utf-8"))
    exact_keys(corpus, {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "challenges", "r3_closure", "cases", "closed_case_order", "scope_exclusions"}, "R3 corpus")
    if corpus["schema"] != "p1342-oracle-corpus-v3" or integer(corpus["revision"], "corpus revision") != 3:
        raise Failure("R3 corpus identity mismatch")
    for pin in corpus["protected_inputs"].values():
        if not isinstance(pin, list) or len(pin) != 2 or hashlib.sha256((ROOT / pin[0]).read_bytes()).hexdigest() != pin[1]:
            raise Failure("R3 protected input changed")
    exact_keys(corpus["challenges"], set(MODES), "R3 challenges")
    cases = corpus["cases"]
    if corpus["closed_case_order"] != [case["id"] for case in cases]:
        raise Failure("R3 closed case order mismatch")
    for case in cases:
        exact_keys(case, {"id", "family", "expected", "validity", "purpose"}, f"case {case.get('id', '?')}")
        if case["validity"] != "valid": raise Failure("no invalid vector is permitted in R3 corpus")
    pins = {"contract": hashlib.sha256(args.contract.read_bytes()).hexdigest(), "binding": hashlib.sha256(args.manifest.read_bytes()).hexdigest(), "fixture": corpus["protected_inputs"]["fixture"][1], "l0_freeze": corpus["protected_inputs"]["l0_freeze"][1]}
    orders = {"normal": cases, "repeat": cases, "reverse": list(reversed(cases))}
    per_case: dict[str, list[str]] = defaultdict(list)
    report: dict[str, Any] = {"schema": "p1342-oracle-run-report-v3", "runs": {}, "agreement": True}
    for batch, ordered in orders.items():
        results = []
        for case in ordered:
            dto, evidence, local_manifest, trusted = build_case(case["id"], corpus, manifest, pins)
            try:
                actual = judge(dto, evidence, trusted, contract, local_manifest, pins)
                witness = "all R3 composite predicates passed" if actual == "Preserved" else "deliberate witness opacity after all R3 non-payload predicates"
            except Failure as exc:
                actual = "Violated"
                witness = str(exc)
            per_case[case["id"]].append(actual)
            if actual != case["expected"]: report["agreement"] = False
            results.append({"id": case["id"], "expected": case["expected"], "actual": actual, "witness": witness})
        report["runs"][batch] = results
    if any(len(set(values)) != 1 for values in per_case.values()): report["agreement"] = False
    negatives = [case for case in cases if case["family"] in {"A", "X"}]
    rejected = sum(per_case[case["id"]][0] == "Violated" for case in negatives)
    survivors = [case["id"] for case in negatives if per_case[case["id"]][0] != "Violated"]
    report["summary"] = {"cases": len(cases), "valid_negatives": len(negatives), "negative_violated": rejected, "mutation_score": rejected / len(negatives), "positive_preserved": sum(case["family"] == "positive" and per_case[case["id"]][0] == "Preserved" for case in cases), "opaque_unknown": sum(case["family"] == "opaque" and per_case[case["id"]][0] == "Unknown" for case in cases), "survivors": survivors}
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["agreement"] and not survivors else 1


if __name__ == "__main__":
    try: raise SystemExit(main())
    except Failure as exc: print(json.dumps({"schema": "p1342-oracle-run-report-v3", "fatal": str(exc)}, indent=2), file=sys.stderr); raise SystemExit(2)
