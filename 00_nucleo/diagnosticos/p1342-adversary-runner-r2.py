#!/usr/bin/env python3
"""Independent adversarial replay and boundary attacks for P1342 R2.

Only deep copies are mutated. The R2 contract, binding manifest, checker, corpus,
authorship artifacts and receipts remain read-only. The checker composite is
invoked through its unchanged ``judge`` function.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
from collections import defaultdict
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
R1_CASES = [f"A{i:02d}" for i in range(1, 22)]


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_checker(path: Path) -> Any:
    spec = importlib.util.spec_from_file_location("p1342_oracle_checker_r2", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load R2 checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def first_case_id(corpus: dict[str, Any], prefix: str) -> str:
    values = [case["id"] for case in corpus["cases"] if case["id"].startswith(prefix)]
    if len(values) != 1:
        raise RuntimeError(f"case prefix {prefix} does not resolve once")
    return values[0]


def runtime_for(evidence: dict[str, Any], mode: str) -> dict[str, Any]:
    return next(item for item in evidence["runtime_runs"] if item["mode"] == mode)


def selected_events(run: dict[str, Any]) -> list[dict[str, Any]]:
    return next(
        cell["events"]
        for cell in run["raw_snapshot"]["cells"]
        if cell["events"][0]["data"]["attempt_kind"] == "selected"
    )


def event_of(events: list[dict[str, Any]], kind: str) -> dict[str, Any]:
    values = [event for event in events if event["kind"] == kind]
    if len(values) != 1:
        raise RuntimeError(f"event {kind} does not resolve once in attack fixture")
    return values[0]


def rebuild_run(checker: Any, run: dict[str, Any]) -> None:
    """Recompute the complete raw receipt chain/digest and exact projection."""
    challenge = bytes.fromhex(run["challenge"])
    flattened: list[tuple[str, dict[str, Any]]] = []
    seq = 0
    for cell in run["raw_snapshot"]["cells"]:
        for event in cell["events"]:
            event["seq"] = seq
            seq += 1
            flattened.append((cell["origin_cell"], event))
        event_of(cell["events"], "attempt-close")["data"]["final_event_count"] = len(cell["events"])
    receipts = []
    previous = None
    hits: dict[tuple[str, str], int] = defaultdict(int)
    for index, (origin, event) in enumerate(flattened):
        hits[(origin, event["hook"])] += 1
        event_hash = checker.digest(event)
        current = checker.receipt_hash(challenge, index, previous, event["hook"], event_hash)
        receipts.append({
            "seq": index,
            "hook": event["hook"],
            "hook_hit": hits[(origin, event["hook"])],
            "origin_cell": origin,
            "event_digest": event_hash,
            "prev_receipt": previous,
            "receipt": current,
        })
        previous = current
    raw = run["raw_snapshot"]
    raw["append_receipts"] = receipts
    raw_without = {key: value for key, value in raw.items() if key != "raw_digest"}
    raw["raw_digest"] = checker.digest(raw_without)
    run["projection"] = {
        "schema": "p1342-ledger-projection-v2",
        "challenge_commitment": run["challenge_commitment"],
        "raw_digest_before": raw["raw_digest"],
        "raw_digest_after": raw["raw_digest"],
        "cells": copy.deepcopy(raw["cells"]),
    }


def refresh_runtime_digests(dto: dict[str, Any], evidence: dict[str, Any]) -> None:
    for run in dto["runs"]:
        runtime = runtime_for(evidence, run["mode"])
        runtime["raw_digest_before"] = run["raw_snapshot"]["raw_digest"]
        runtime["raw_digest_after"] = run["raw_snapshot"]["raw_digest"]


Mutation = Callable[[Any, dict[str, Any], dict[str, Any], dict[str, Any]], tuple[bool, str]]


def evidence_hash_mismatch(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del dto, manifest
    trusted = checker.digest(evidence)
    evidence["source_inspection"]["candidate_tree_sha256"] = "0" * 64
    return False, trusted


def evidence_open_member(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del dto, manifest
    evidence["forged"] = True
    return True, ""


def anchor_fixture_false(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    evidence["source_inspection"]["anchors"][5]["fixture_reached"] = False
    return True, ""


def anchor_status_false(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    evidence["source_inspection"]["anchors"][14]["status"] = "text-only"
    return True, ""


def runtime_tail_fabricated(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    for runtime in evidence["runtime_runs"]:
        hit = next(item for item in runtime["hook_hits"] if item["hook"] == "H12A-replay-enter" and item["count"] == 1)
        hit["receipt_tail"] = "0" * 64
    return True, ""


def h00_tail_fabricated(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    for runtime in evidence["runtime_runs"]:
        hit = next(item for item in runtime["hook_hits"] if item["hook"] == "H00D-attempt-kind-discovery")
        hit["receipt_tail"] = "f" * 64
    return True, ""


def h00_both_live(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    for runtime in evidence["runtime_runs"]:
        discovery = next(item for item in runtime["hook_hits"] if item["hook"] == "H00D-attempt-kind-discovery")
        runtime["hook_hits"].append({
            "hook": "H00S-attempt-kind-selected",
            "origin_cell": discovery["origin_cell"],
            "count": 1,
            "receipt_tail": "e" * 64,
        })
    return True, ""


def duplicate_challenge(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, evidence, manifest
    dto["runs"][1]["challenge"] = dto["runs"][0]["challenge"]
    return False, ""


def issuer_receipt_fabricated(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    for runtime in evidence["runtime_runs"]:
        runtime["challenge_issuer_receipt_sha256"] = "0" * 64
    return True, ""


def receipt_digest_corrupt(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, evidence, manifest
    dto["runs"][0]["raw_snapshot"]["append_receipts"][3]["event_digest"] = "0" * 64
    return False, ""


def causal_base_reorder_resealed(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del manifest
    for run in dto["runs"]:
        events = selected_events(run)
        events[1], events[2] = events[2], events[1]
        rebuild_run(checker, run)
    refresh_runtime_digests(dto, evidence)
    return True, ""


def raw_not_frozen(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, evidence, manifest
    dto["runs"][0]["raw_snapshot"]["frozen_before_projection"] = False
    return False, ""


def runtime_after_changed(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    evidence["runtime_runs"][0]["raw_digest_after"] = "0" * 64
    return True, ""


def projection_changed(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, evidence, manifest
    selected_events({"raw_snapshot": {"cells": dto["runs"][0]["projection"]["cells"]}})[0]["data"]["attempt_index"] = 99
    return False, ""


def created_endpoint_disconnected(field: str, domain: str) -> Mutation:
    def mutate(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
        del manifest
        for run in dto["runs"]:
            event = event_of(selected_events(run), "counter-occurrence-created")
            event["refs"][field] = checker.make_id(domain, run["challenge_commitment"], "disconnected-at-creation")
            rebuild_run(checker, run)
        refresh_runtime_digests(dto, evidence)
        return True, ""
    return mutate


def ref_disconnected(kind: str, field: str, domain: str) -> Mutation:
    def mutate(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
        del manifest
        for run in dto["runs"]:
            event = event_of(selected_events(run), kind)
            event["refs"][field] = checker.make_id(domain, run["challenge_commitment"], "disconnected")
            rebuild_run(checker, run)
        refresh_runtime_digests(dto, evidence)
        return True, ""
    return mutate


def r_count_false(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    for runtime in evidence["runtime_runs"]:
        hit = next(item for item in runtime["hook_hits"] if item["hook"] == "H12A-replay-enter" and item["count"] == 1)
        hit["count"] = 0
        hit["receipt_tail"] = None
    return True, ""


def extra_known_event(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del manifest
    for run in dto["runs"]:
        events = selected_events(run)
        events.insert(-1, copy.deepcopy(event_of(events, "dict-witness-produced")))
        rebuild_run(checker, run)
    refresh_runtime_digests(dto, evidence)
    return True, ""


def bool_runtime_count(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del checker, dto, manifest
    evidence["runtime_runs"][0]["hook_hits"][0]["count"] = True
    return True, ""


def unknown_with_created_endpoint_loss(checker: Any, dto: dict[str, Any], evidence: dict[str, Any], manifest: dict[str, Any]) -> tuple[bool, str]:
    del manifest
    for run in dto["runs"]:
        events = selected_events(run)
        created = event_of(events, "counter-occurrence-created")
        created["refs"]["func_callback"] = checker.make_id("func-callback", run["challenge_commitment"], "lost-before-opacity")
        witness = event_of(events, "dict-witness-produced")
        witness["data"]["typed_value"] = {
            "type": "dict",
            "visibility": "opaque",
            "commitment": "3f5b8f852dd5bba8fa19de766caf42ef54f0e060ca9faf9e0a82911926d86eb6",
        }
        rebuild_run(checker, run)
        run["classification"] = "Unknown"
    refresh_runtime_digests(dto, evidence)
    return True, ""


EXTRA: list[tuple[str, str, Mutation]] = [
    ("X01-evidence-hash-out-of-band-mismatch", "evidence changes while trusted hash remains frozen", evidence_hash_mismatch),
    ("X02-evidence-closed-schema-extra", "authenticated evidence has an unknown top-level member", evidence_open_member),
    ("X03-anchor-fixture-coverage-false", "source evidence marks a required anchor unreached", anchor_fixture_false),
    ("X04-anchor-inspection-status-false", "source evidence downgrades structural inspection to text-only", anchor_status_false),
    ("X05-runtime-receipt-tail-fabricated", "H12A runtime coverage tail is unrelated all-zero digest", runtime_tail_fabricated),
    ("X06-h00d-receipt-tail-fabricated", "H00D runtime coverage tail is unrelated all-f digest", h00_tail_fabricated),
    ("X07-h00d-h00s-both-live", "discovery origin claims both mutually exclusive attempt-kind hooks", h00_both_live),
    ("X08-challenge-not-pairwise-fresh", "repeat reuses the normal challenge", duplicate_challenge),
    ("X09-challenge-issuer-receipt-fabricated", "every issuer receipt reference is replaced by all-zero digest", issuer_receipt_fabricated),
    ("X10-append-event-digest-corrupt", "raw receipt event digest no longer matches canonical event", receipt_digest_corrupt),
    ("X11-base-causal-order-resealed", "context dispatch and prebound Dict are swapped, then raw receipts/digests/projection are rebuilt", causal_base_reorder_resealed),
    ("X12-raw-freeze-flag-false", "raw snapshot declares it was not frozen before projection", raw_not_frozen),
    ("X13-runtime-raw-after-differs", "independent after-projection digest differs", runtime_after_changed),
    ("X14-projection-not-equal-raw", "projection mutates a raw event", projection_changed),
    ("X15-created-callback-with-disconnected", "created outer Func endpoint differs from dispatch/With endpoint", created_endpoint_disconnected("callback_with", "callback-with")),
    ("X16-created-func-callback-disconnected", "created inner Func endpoint differs from With/body endpoint", created_endpoint_disconnected("func_callback", "func-callback")),
    ("X17-run-endpoint-disconnected", "syntax-body enter uses a different in-domain run identity", ref_disconnected("syntax-body-enter", "run", "run")),
    ("X18-ledger-endpoint-disconnected", "syntax-body exit uses a different in-domain ledger identity", ref_disconnected("syntax-body-exit", "ledger", "ledger")),
    ("X19-r-external-count-false", "external H12A count is zero while raw has one replay", r_count_false),
    ("X20-extra-known-event", "second witness event inserted with fully rebuilt receipts", extra_known_event),
    ("X21-bool-as-runtime-int", "runtime hook count is JSON true", bool_runtime_count),
    ("X22-unknown-before-created-endpoint-proof", "witness is opaque while created inner Func endpoint is disconnected", unknown_with_created_endpoint_loss),
]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--checker", required=True, type=Path)
    parser.add_argument("--contract", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    args = parser.parse_args()
    checker = load_checker(args.checker)
    contract = json.loads(args.contract.read_text(encoding="utf-8"))
    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    corpus = json.loads(args.corpus.read_text(encoding="utf-8"))
    pins = {
        "contract": file_sha(args.contract),
        "binding": file_sha(args.manifest),
        "fixture": corpus["protected_inputs"]["fixture"][1],
        "l0_freeze": corpus["protected_inputs"]["l0_freeze"][1],
    }

    attacks: list[tuple[str, str, str, Mutation | None]] = []
    for prefix in R1_CASES:
        case_id = first_case_id(corpus, prefix)
        purpose = next(case["purpose"] for case in corpus["cases"] if case["id"] == case_id)
        attacks.append((case_id, "R1-replay", purpose, None))
    attacks.extend((case_id, "R2-boundary", vector, mutation) for case_id, vector, mutation in EXTRA)

    orders = {"normal": attacks, "repeat": attacks, "reverse": list(reversed(attacks))}
    per_case: dict[str, list[str]] = defaultdict(list)
    report: dict[str, Any] = {
        "schema": "p1342-adversary-run-report-v2",
        "regime": "executado sem atestacao de isolamento",
        "protected_hashes": {
            "checker": file_sha(args.checker),
            "contract": pins["contract"],
            "binding": pins["binding"],
            "corpus": file_sha(args.corpus),
        },
        "runs": {},
    }
    for batch, ordered in orders.items():
        results = []
        for case_id, family, vector, mutation in ordered:
            if mutation is None:
                dto, evidence, local_manifest, trusted = checker.build_case(case_id, corpus, manifest, pins)
            else:
                dto, evidence, local_manifest, trusted = checker.build_case("P01-inspectable-composite", corpus, manifest, pins)
                recompute, explicit_trusted = mutation(checker, dto, evidence, local_manifest)
                if explicit_trusted:
                    trusted = explicit_trusted
                elif recompute:
                    trusted = checker.digest(evidence)
            try:
                actual = checker.judge(dto, evidence, trusted, contract, local_manifest, pins)
                witness = "checker accepted every implemented composite predicate"
            except checker.Failure as exc:
                actual = "Violated"
                witness = str(exc)
            per_case[case_id].append(actual)
            results.append({"id": case_id, "family": family, "expected": "Violated", "actual": actual, "vector": vector, "witness": witness})
        report["runs"][batch] = results

    stable = all(len(set(values)) == 1 for values in per_case.values())
    total = len(attacks)
    rejected = sum(values[0] == "Violated" for values in per_case.values())
    preserved = sum(values[0] == "Preserved" for values in per_case.values())
    unknown = sum(values[0] == "Unknown" for values in per_case.values())
    r1_rejected = sum(per_case[first_case_id(corpus, prefix)][0] == "Violated" for prefix in R1_CASES)
    survivors = [case_id for case_id, values in per_case.items() if values[0] != "Violated"]
    report["summary"] = {
        "valid_negative_mutations": total,
        "r1_negatives_replayed": 21,
        "r1_negatives_violated": r1_rejected,
        "r2_boundary_negatives": len(EXTRA),
        "rejected_as_violated": rejected,
        "survived_as_preserved": preserved,
        "survived_as_unknown": unknown,
        "mutation_score": rejected / total,
        "stable_normal_repeat_reverse": stable,
        "survivors": survivors,
        "verdict": "BLOCKER_NOT_SEALED" if survivors else "NOT_SEALED",
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 1 if survivors or not stable else 0


if __name__ == "__main__":
    raise SystemExit(main())
