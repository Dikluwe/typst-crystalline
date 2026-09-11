#!/usr/bin/env python3
"""Independent adversarial harness for the frozen P1342 R1 checker.

This file never edits the contract, manifest, oracle corpus, checker, L0, product,
or tests.  It imports the exact public validation functions from the protected
checker, mutates only deep copies in memory, and prints a deterministic report.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_checker(path: Path) -> Any:
    spec = importlib.util.spec_from_file_location("p1342_oracle_checker_r1", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def hook(manifest: dict[str, Any], short: str) -> dict[str, Any]:
    values = [value for value in manifest["hooks"] if value["hook"].startswith(short)]
    if len(values) != 1:
        raise RuntimeError(f"hook {short} does not resolve once in attack harness")
    return values[0]


def selected(corpus: dict[str, Any]) -> dict[str, Any]:
    return copy.deepcopy(corpus["templates"]["selected-r1"])


def replace(dto: dict[str, Any], pointer: str, value: Any) -> None:
    parts = pointer.strip("/").split("/")
    current: Any = dto
    for part in parts[:-1]:
        current = current[int(part)] if isinstance(current, list) else current[part]
    key = parts[-1]
    if isinstance(current, list):
        current[int(key)] = copy.deepcopy(value)
    else:
        current[key] = copy.deepcopy(value)


def swap(dto: dict[str, Any], left: str, right: str) -> None:
    def parent(pointer: str) -> tuple[Any, str]:
        parts = pointer.strip("/").split("/")
        current: Any = dto
        for part in parts[:-1]:
            current = current[int(part)] if isinstance(current, list) else current[part]
        return current, parts[-1]
    lp, lk = parent(left)
    rp, rk = parent(right)
    lv = lp[int(lk)] if isinstance(lp, list) else lp[lk]
    rv = rp[int(rk)] if isinstance(rp, list) else rp[rk]
    if isinstance(lp, list):
        lp[int(lk)] = rv
    else:
        lp[lk] = rv
    if isinstance(rp, list):
        rp[int(rk)] = lv
    else:
        rp[rk] = lv


def rechain(events: list[dict[str, Any]], prefix: str = "event:attack") -> None:
    previous = None
    for index, event in enumerate(events):
        event["seq"] = index
        event["event_id"] = f"{prefix}-{index:02d}"
        event["prev_event_id"] = previous
        previous = event["event_id"]


Mutation = Callable[[dict[str, Any], dict[str, Any]], None]


def nothing(_dto: dict[str, Any], _manifest: dict[str, Any]) -> None:
    pass


def dead_h06(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del dto
    value = hook(manifest, "H06")
    value["anchor_needle"] = '"display" => {'
    value["fixture_reach"] = "fabricated: focal fixture never takes display branch"
    value["species"] = "dead-branch-fabricated-source-span"


def dead_h15(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del dto
    value = hook(manifest, "H15")
    value["anchor_scope"] = "symbol"
    value["anchor_needle"] = "Value::None"
    value["fixture_reach"] = "fabricated: parsed CodeBlock never takes body conversion failure"
    value["species"] = "dead-else-branch"


def fabricated_h08(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del dto
    value = hook(manifest, "H08")
    value["anchor_needle"] = "fn with_native_fn_addr_e_none() {"
    value["stable_role"] = "location"
    value["species"] = "fabricated-test-function-hook"
    value["fixture_reach"] = "false"
    value["cardinality_per_attempt"] = "exactly 0"


def adulterate_manifest(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del dto
    manifest["revision"] = True
    manifest["baseline"] = {"forged": True}
    manifest["forbidden_bindings"] = []
    value = hook(manifest, "H10")
    value["stable_role"] = "snapshot-post"
    value["fixture_reach"] = "self-attested without evidence"
    value["cardinality_per_attempt"] = "never"
    value["species"] = "unrelated"


def invalid_anchor_scope(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del dto
    value = hook(manifest, "H09")
    value["anchor_scope"] = "fabricated-scope-enum"


def posthoc_rechain(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    rechain(dto["attempts"][0]["events"], "event:posthoc")


def carrier_at(event_index: int) -> Mutation:
    def mutate(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
        del manifest
        dto["attempts"][0]["events"][event_index]["refs"]["carrier"] = f"carrier:lost-{event_index}"
    return mutate


def location_at_replay_exit(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][13]["refs"]["location"] = "location:posthoc-other"


def fabricated_snapshot_post(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][13]["refs"]["snapshot_post"] = "snapshot:fabricated-posthoc"


def insert_decoy(dto: dict[str, Any], manifest: dict[str, Any], variant: str) -> None:
    del manifest
    events = dto["attempts"][0]["events"]
    event = copy.deepcopy(events[2] if variant != "witness" else events[11])
    event["data"]["role"] = f"dict-{variant}"
    event["event_id"] = f"event:{variant}"
    if variant == "bool":
        event["data"]["span"] = {"offset": True, "end_exclusive": False}
        event["data"]["value"] = {"type": "int", "value": True}
    elif variant == "swapped-span":
        event["data"]["span"] = {"offset": 64, "end_exclusive": 121}
    events.insert(len(events) - 1, event)
    rechain(events, f"event:insert-{variant}")


def decoy(which: str) -> Mutation:
    return lambda dto, manifest: insert_decoy(dto, manifest, which)


def forged_r(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["kind"] = "discovery"
    rechain(dto["attempts"][0]["events"], "event:r-self-declared")


def opaque_witness(dto: dict[str, Any]) -> None:
    dto["payload_visibility"] = "opaque"
    dto["attempts"][0]["events"][11]["data"]["value"] = {
        "type": "dict",
        "visibility": "opaque",
        "commitment": "3f5b8f852dd5bba8fa19de766caf42ef54f0e060ca9faf9e0a82911926d86eb6",
    }


def early_unknown(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    opaque_witness(dto)
    dto["attempts"][0]["events"][10]["refs"]["carrier"] = "carrier:lost-before-unknown"


def source_span_swapped(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    replace(dto, "/attempts/0/events/3/data/source_span", {"offset": 64, "end_exclusive": 121})


def body_span_swapped(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    replace(dto, "/attempts/0/events/10/data/span", {"offset": 144, "end_exclusive": 164})


def dicts_swapped(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    swap(dto, "/attempts/0/events/2/data/value", "/attempts/0/events/11/data/value")


def dict_reordered(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    entries = dto["attempts"][0]["events"][11]["data"]["value"]["entries"]
    entries.insert(0, ["extra", {"type": "int", "value": 0}])


def fictitious_func_body(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][10]["data"]["syntax_kind"] = "Func"


def cross_domain_alias(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][4]["refs"]["location"] = "snapshot:selected-pre"


def remove_required(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"].pop(12)
    rechain(dto["attempts"][0]["events"], "event:removed")


def duplicate_required(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    events = dto["attempts"][0]["events"]
    events.insert(9, copy.deepcopy(events[8]))
    rechain(events, "event:duplicated")


def reorder_required(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    events = dto["attempts"][0]["events"]
    events[10], events[11] = events[11], events[10]
    rechain(events, "event:reordered")


def edge_inverted(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    refs = dto["attempts"][0]["events"][7]["refs"]
    refs["outer"], refs["inner"] = refs["inner"], refs["outer"]


def edge_disconnected(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][7]["refs"]["inner"] = "func-callback:orphan"


def carrier_lost_walk(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][4]["refs"]["carrier"] = "carrier:lost-at-walk"


def add_r_field(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["R"] = 1


def main_bool(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["attempts"][0]["events"][2]["data"]["value"]["entries"][0][1]["value"] = True


def prebound_opaque(dto: dict[str, Any], manifest: dict[str, Any]) -> None:
    del manifest
    dto["payload_visibility"] = "opaque"
    dto["attempts"][0]["events"][2]["data"]["value"] = {
        "type": "dict", "visibility": "opaque", "commitment": "0" * 64,
    }


ATTACKS: list[tuple[str, str, Mutation, bool]] = [
    ("A01-dead-h06-branch-anchor", "dead hook accepted by textual uniqueness", dead_h06, True),
    ("A02-dead-h15-else-anchor", "fixture-unreachable body branch accepted", dead_h15, True),
    ("A03-fabricated-h08-test-anchor", "test-only function fabricated as productive hook", fabricated_h08, True),
    ("A04-binding-metadata-adulterated", "baseline, forbidden policy, role, species, reach and cardinality adulterated", adulterate_manifest, True),
    ("A05-invalid-anchor-scope-enum", "unknown anchor_scope treated as symbol scope", invalid_anchor_scope, True),
    ("A06-fixed-runtime-ids-across-runs", "same concrete IDs reused in normal/repeat/reverse", nothing, False),
    ("A07-posthoc-event-ids-and-ordinals", "whole predecessor chain and ordinals regenerated post hoc", posthoc_rechain, True),
    ("A08-carrier-lost-outer-dispatch", "carrier discontinuity at outer Func dispatch", carrier_at(6), True),
    ("A09-carrier-lost-with-edge", "carrier discontinuity at With edge", carrier_at(7), True),
    ("A10-carrier-lost-inner-dispatch", "carrier discontinuity at inner Func dispatch", carrier_at(8), True),
    ("A11-carrier-lost-closure-dispatch", "carrier discontinuity at closure dispatch", carrier_at(9), True),
    ("A12-carrier-lost-body-enter", "carrier discontinuity at SyntaxNode body enter", carrier_at(10), True),
    ("A13-carrier-lost-body-exit", "carrier discontinuity at SyntaxNode body exit", carrier_at(12), True),
    ("A14-location-lost-replay-exit", "Location endpoint disconnected at replay exit", location_at_replay_exit, True),
    ("A15-snapshot-post-fabricated", "post snapshot identity accepted without productive provenance", fabricated_snapshot_post, True),
    ("A16-extra-event-inserted", "closed fixture accepts extra Dict event with unknown role", decoy("decoy"), True),
    ("A17-extra-event-duplicated", "duplicated Dict payload hidden under unknown role", decoy("witness"), True),
    ("A18-bool-as-int-in-unvalidated-event", "bool accepted in nominal int of ignored Dict event", decoy("bool"), True),
    ("A19-swapped-span-in-unvalidated-event", "body span accepted on ignored prebound-like Dict", decoy("swapped-span"), True),
    ("A20-r-self-declared-by-fabricated-events", "R=1 accepted from DTO events even when attempt relabelled discovery", forged_r, True),
    ("A21-unknown-before-carrier-proof", "Unknown returned despite carrier loss at body enter", early_unknown, True),
    ("C01-source-span-swapped", "protected occurrence span swap", source_span_swapped, True),
    ("C02-body-span-swapped", "protected SyntaxNode span swap", body_span_swapped, True),
    ("C03-dicts-swapped", "prebound and witness Dict payloads swapped", dicts_swapped, True),
    ("C04-dict-reordered", "typed witness Dict order/content altered", dict_reordered, True),
    ("C05-fictitious-func-body", "SyntaxNode body falsely typed as Func", fictitious_func_body, True),
    ("C06-cross-domain-alias", "Location aliases snapshot domain", cross_domain_alias, True),
    ("C07-required-event-removed", "required body exit removed and chain repaired", remove_required, True),
    ("C08-required-event-duplicated", "required inner dispatch duplicated and chain repaired", duplicate_required, True),
    ("C09-required-event-reordered", "body enter and Dict witness reordered and chain repaired", reorder_required, True),
    ("C10-with-edge-inverted", "With edge direction inverted", edge_inverted, True),
    ("C11-with-edge-disconnected", "With inner endpoint disconnected", edge_disconnected, True),
    ("C12-carrier-lost-at-walk", "carrier discontinuity at checked walk boundary", carrier_lost_walk, True),
    ("C13-r-field-self-declared", "explicit R field added to closed DTO", add_r_field, True),
    ("C14-main-bool-is-not-int", "bool substituted in protected typed Int", main_bool, True),
    ("C15-prebound-opacity-too-early", "opacity introduced before witness payload", prebound_opaque, True),
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
    contract_hash = sha256(args.contract)
    manifest_hash = sha256(args.manifest)
    fixture = contract["fixture"]

    modes = ["normal", "repeat", "reverse"]
    orders = {
        "normal": ATTACKS,
        "repeat": ATTACKS,
        "reverse": list(reversed(ATTACKS)),
    }
    report: dict[str, Any] = {
        "schema": "p1342-adversary-run-report-v1",
        "regime": "executado sem atestacao de isolamento",
        "protected_hashes": {
            str(args.checker): sha256(args.checker),
            str(args.contract): contract_hash,
            str(args.manifest): manifest_hash,
            str(args.corpus): sha256(args.corpus),
        },
        "runs": {},
    }
    classifications: dict[str, list[str]] = {attack[0]: [] for attack in ATTACKS}
    for mode_index, mode in enumerate(modes):
        results = []
        for case_index, (case_id, vector, mutation, remap) in enumerate(orders[mode]):
            dto = selected(corpus)
            local_manifest = copy.deepcopy(manifest)
            mutation(dto, local_manifest)
            if remap:
                dto = checker.remap_local_ids(dto, f"{mode}-{mode_index}-{case_index}")
            try:
                checker.validate_static_manifest(local_manifest, contract_hash, fixture["sha256"])
                actual = checker.validate_dto(dto, manifest_hash, fixture, local_manifest)
            except checker.Failure as exc:
                actual = "Violated"
                witness = str(exc)
            else:
                witness = "checker accepted every implemented predicate"
            classifications[case_id].append(actual)
            results.append({"id": case_id, "expected": "Violated", "actual": actual, "vector": vector, "witness": witness})
        report["runs"][mode] = results

    stable = all(len(set(values)) == 1 for values in classifications.values())
    rejected = sum(values[0] == "Violated" for values in classifications.values())
    preserved = sum(values[0] == "Preserved" for values in classifications.values())
    unknown = sum(values[0] == "Unknown" for values in classifications.values())
    total = len(ATTACKS)
    survivors = [case_id for case_id, values in classifications.items() if values[0] != "Violated"]
    report["summary"] = {
        "valid_negative_mutations": total,
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
