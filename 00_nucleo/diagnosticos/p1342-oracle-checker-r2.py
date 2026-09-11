#!/usr/bin/env python3
"""P1342 R2 composite oracle.

The judged DTO and the independently pinned source/runtime evidence are separate
arguments to ``judge``. A candidate DTO cannot attest its own hook coverage,
source placement, challenge issuance, or raw pre-projection observation.

The canonicalizer implements the RFC 8785 subset used by this contract: objects,
arrays, UTF-8 strings, booleans, null, and exact integers. Floats are forbidden.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
ZERO32 = bytes(32)
RECEIPT_PREFIX = b"P1342-APPEND-V2\0"
MODES = ["normal", "repeat", "reverse"]
MANIFEST_HOOKS = [
    "H00D-attempt-kind-discovery", "H00S-attempt-kind-selected",
    "H01-attempt-open", "H02-context-dispatch", "H03-attempt-result",
    "H04-eval-carrier-transport", "H05-dict-production",
    "H06-counter-update-span", "H07-occurrence-created",
    "H08-func-carrier-slot", "H09-with-object",
    "H10-action-carrier-clone", "H11-occurrence-walked",
    "H12A-replay-enter", "H12B-replay-exit", "H13-func-dispatch",
    "H14-with-edge", "H15-syntax-body", "H16-raw-freeze-before-projection",
]
EVENT_HOOK = {
    "attempt-open": "H01-attempt-open",
    "context-dispatch": "H02-context-dispatch",
    "dict-prebound-produced": "H05-dict-production",
    "counter-occurrence-created": "H07-occurrence-created",
    "attempt-result": "H03-attempt-result",
    "counter-occurrence-walked": "H11-occurrence-walked",
    "callback-replay-enter": "H12A-replay-enter",
    "func-dispatch-with": "H13-func-dispatch",
    "with-edge": "H14-with-edge",
    "func-dispatch-inner": "H13-func-dispatch",
    "syntax-body-enter": "H15-syntax-body",
    "dict-witness-produced": "H05-dict-production",
    "syntax-body-exit": "H15-syntax-body",
    "callback-replay-exit": "H12B-replay-exit",
    "attempt-close": "H16-raw-freeze-before-projection",
}
REFS = {
    "attempt-open": ["run", "ledger", "origin_cell", "session_carrier"],
    "context-dispatch": ["run", "ledger", "origin_cell", "session_carrier", "context_closure"],
    "dict-prebound-produced": ["run", "ledger", "origin_cell", "session_carrier", "dict_prebound"],
    "counter-occurrence-created": ["run", "ledger", "origin_cell", "session_carrier", "occurrence_carrier", "callback_with", "func_callback"],
    "attempt-result": ["run", "ledger", "origin_cell", "session_carrier"],
    "counter-occurrence-walked": ["run", "ledger", "origin_cell", "session_carrier", "occurrence_carrier", "counter_content", "location"],
    "callback-replay-enter": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "callback_with", "location", "snapshot_pre"],
    "func-dispatch-with": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "callback_with"],
    "with-edge": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "callback_with", "func_callback"],
    "func-dispatch-inner": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "func_callback"],
    "syntax-body-enter": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "func_callback", "syntax_body"],
    "dict-witness-produced": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "syntax_body", "dict_witness"],
    "syntax-body-exit": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "func_callback", "syntax_body"],
    "callback-replay-exit": ["run", "ledger", "origin_cell", "consumer_cell", "session_carrier", "occurrence_carrier", "location", "snapshot_pre", "snapshot_post"],
    "attempt-close": ["run", "ledger", "origin_cell", "session_carrier"],
}
DATA = {
    "attempt-open": ["attempt_kind", "attempt_index"],
    "context-dispatch": ["context_body_span"],
    "dict-prebound-produced": ["span", "typed_value"],
    "counter-occurrence-created": ["source_span", "counter_key", "action"],
    "attempt-result": ["result_kind", "incorporation_state"],
    "counter-occurrence-walked": ["counter_key", "action"],
    "callback-replay-enter": ["snapshot_pre_value"],
    "func-dispatch-with": ["repr_kind"],
    "with-edge": ["prebound_args_span"],
    "func-dispatch-inner": ["repr_kind"],
    "syntax-body-enter": ["syntax_kind", "span"],
    "dict-witness-produced": ["span", "typed_value"],
    "syntax-body-exit": ["outcome"],
    "callback-replay-exit": ["snapshot_pre_value", "snapshot_post_value", "outcome"],
    "attempt-close": ["attempt_kind", "final_event_count"],
}
ID_PREFIX = {
    "run": "run", "ledger": "ledger", "origin_cell": "cell",
    "consumer_cell": "cell", "session_carrier": "session-carrier",
    "context_closure": "context-closure", "dict_prebound": "dict-prebound",
    "occurrence_carrier": "occurrence-carrier", "callback_with": "callback-with",
    "func_callback": "func-callback", "counter_content": "counter-content",
    "location": "location", "snapshot_pre": "snapshot-pre",
    "snapshot_post": "snapshot-post", "syntax_body": "syntax-node",
    "dict_witness": "dict-witness",
}
HEX64 = re.compile(r"[0-9a-f]{64}\Z")


class Failure(Exception):
    pass


def exact_keys(value: Any, expected: set[str] | list[str], where: str) -> None:
    expected_set = set(expected)
    if not isinstance(value, dict) or set(value) != expected_set:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        raise Failure(f"{where}: closed keys {sorted(expected_set)} required, got {got}")


def integer(value: Any, where: str) -> int:
    if type(value) is not int:
        raise Failure(f"{where}: exact integer required; bool is not int")
    return value


def boolean(value: Any, where: str) -> bool:
    if type(value) is not bool:
        raise Failure(f"{where}: exact bool required")
    return value


def canonical(value: Any) -> bytes:
    def inspect(item: Any) -> None:
        if isinstance(item, float):
            raise Failure("float is outside the exact-integer canonical subset")
        if isinstance(item, dict):
            if not all(isinstance(key, str) for key in item):
                raise Failure("canonical object keys must be strings")
            for child in item.values(): inspect(child)
        elif isinstance(item, list):
            for child in item: inspect(child)
        elif item is not None and type(item) not in {str, int, bool}:
            raise Failure(f"unsupported canonical type {type(item).__name__}")
    inspect(value)
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest(value: Any) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def span(value: Any, expected: tuple[int, int], where: str) -> None:
    if not isinstance(value, list) or len(value) != 2:
        raise Failure(f"{where}: [offset,end_exclusive] required")
    got = (integer(value[0], where), integer(value[1], where))
    if got != expected:
        raise Failure(f"{where}: expected {expected[0]}..{expected[1]}, got {got[0]}..{got[1]}")


def typed_value(value: Any, where: str, opaque_allowed: bool = False) -> tuple[Any, bool]:
    if not isinstance(value, dict) or "type" not in value:
        raise Failure(f"{where}: closed typed value required")
    if value["type"] == "int":
        exact_keys(value, {"type", "value"}, where)
        return ("int", integer(value["value"], where)), False
    if value["type"] == "selector":
        exact_keys(value, {"type", "shape", "ordered_fields"}, where)
        if value["shape"] != "heading.where" or not isinstance(value["ordered_fields"], list):
            raise Failure(f"{where}: selector shape/fields mismatch")
        pairs = []
        for index, pair in enumerate(value["ordered_fields"]):
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str):
                raise Failure(f"{where}[{index}]: ordered [name,value] required")
            child, child_opaque = typed_value(pair[1], f"{where}[{index}]")
            if child_opaque: raise Failure(f"{where}: selector opacity forbidden")
            pairs.append((pair[0], child))
        return ("selector", value["shape"], tuple(pairs)), False
    if value["type"] == "dict":
        if value.get("visibility") == "opaque":
            exact_keys(value, {"type", "visibility", "commitment"}, where)
            if not opaque_allowed or not isinstance(value["commitment"], str) or not HEX64.fullmatch(value["commitment"]):
                raise Failure(f"{where}: deliberate opaque Dict not authorized")
            return ("dict", "opaque"), True
        exact_keys(value, {"type", "visibility", "entries"}, where)
        if value["visibility"] != "inspectable" or not isinstance(value["entries"], list):
            raise Failure(f"{where}: ordered Dict pair list required")
        pairs = []
        for index, pair in enumerate(value["entries"]):
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str):
                raise Failure(f"{where}[{index}]: ordered [name,value] required")
            child, child_opaque = typed_value(pair[1], f"{where}[{index}]")
            if child_opaque: raise Failure(f"{where}: nested opacity forbidden")
            pairs.append((pair[0], child))
        return ("dict", tuple(pairs)), False
    raise Failure(f"{where}: unknown typed value variant")


def make_id(domain: str, commitment: str, label: str) -> str:
    return f"{domain}:{commitment[:16]}:{label}"


def validate_id(value: Any, domain: str, commitment: str, where: str) -> str:
    expected_prefix = f"{domain}:{commitment[:16]}:"
    if not isinstance(value, str) or not value.startswith(expected_prefix) or len(value) == len(expected_prefix):
        raise Failure(f"{where}: challenge-bound {domain} identity required")
    return value


def receipt_hash(challenge: bytes, seq: int, previous: str | None, hook: str, event_digest: str) -> str:
    previous_bytes = ZERO32 if previous is None else bytes.fromhex(previous)
    payload = RECEIPT_PREFIX + challenge + seq.to_bytes(8, "big") + previous_bytes + hook.encode() + bytes.fromhex(event_digest)
    return hashlib.sha256(payload).hexdigest()


def expected_hook_counts(attempt_kind: str, r_value: int, incorporated: bool) -> dict[str, int]:
    counts = {hook: 0 for hook in MANIFEST_HOOKS}
    counts["H00D-attempt-kind-discovery" if attempt_kind == "discovery" else "H00S-attempt-kind-selected"] = 1
    counts.update({
        "H01-attempt-open": 1, "H02-context-dispatch": 1,
        "H03-attempt-result": 1, "H04-eval-carrier-transport": 1 + r_value,
        "H05-dict-production": 1 + r_value, "H06-counter-update-span": 1,
        "H07-occurrence-created": 1, "H08-func-carrier-slot": 1,
        "H09-with-object": 1, "H10-action-carrier-clone": 1,
        "H11-occurrence-walked": 1 if incorporated else 0,
        "H12A-replay-enter": r_value, "H12B-replay-exit": r_value,
        "H13-func-dispatch": 2 * r_value, "H14-with-edge": r_value,
        "H15-syntax-body": 2 * r_value, "H16-raw-freeze-before-projection": 1,
    })
    return counts


def validate_manifest(manifest: Any, expected_hash: str, contract: dict[str, Any]) -> None:
    if digest(manifest) == "":  # force exact canonical type validation before field checks
        raise AssertionError
    manifest_path = ROOT / contract["protected_inputs"]["binding_manifest_r2"][0]
    if file_sha(manifest_path) != expected_hash:
        raise Failure("externally pinned R2 manifest bytes changed")
    if manifest != json.loads(manifest_path.read_text(encoding="utf-8")):
        raise Failure("supplied manifest differs from the exact externally pinned R2 bytes")
    expected_top = manifest.get("closed_metadata", {}).get("top_level_keys", [])
    exact_keys(manifest, expected_top, "manifest")
    if manifest["schema"] != "p1342-static-binding-manifest-v2" or integer(manifest["revision"], "manifest.revision") != 2:
        raise Failure("manifest identity/revision mismatch")
    exact_keys(manifest["baseline"], {"head", "tracked_diff_binary_sha256", "candidate_free"}, "manifest.baseline")
    if boolean(manifest["baseline"]["candidate_free"], "manifest.baseline.candidate_free") is not True:
        raise Failure("manifest baseline is not candidate-free")
    exact_keys(manifest["closed_metadata"], {"top_level_keys", "hook_keys", "anchor_scope_enum", "binding_kind_enum"}, "manifest.closed_metadata")
    if manifest["closed_metadata"]["anchor_scope_enum"] != ["rust-symbol-branch"]:
        raise Failure("manifest anchor scope enum adulterated")
    if manifest["closed_metadata"]["binding_kind_enum"] != ["attempt-kind-source", "event-hook", "transport-boundary", "object-constructor", "projection-boundary"]:
        raise Failure("manifest binding kind enum adulterated")
    if [row.get("hook") for row in manifest["hooks"]] != MANIFEST_HOOKS:
        raise Failure("manifest must contain the exact 19 binding rows in frozen order")
    hook_keys = manifest["closed_metadata"]["hook_keys"]
    role_enum, species_enum = set(manifest["role_enum"]), set(manifest["species_enum"])
    for row in manifest["hooks"]:
        exact_keys(row, hook_keys, f"manifest.{row.get('hook', '?')}")
        if row["anchor_scope"] != "rust-symbol-branch" or row["binding_kind"] not in manifest["closed_metadata"]["binding_kind_enum"]:
            raise Failure(f"{row['hook']}: binding enum mismatch")
        integer(row["baseline_line"], f"{row['hook']}.baseline_line")
        if integer(row["expected_matches_in_symbol"], f"{row['hook']}.expected_matches") != 1:
            raise Failure(f"{row['hook']}: expected one symbol/branch match")
        if not isinstance(row["stable_role"], list) or not set(row["stable_role"]) <= role_enum:
            raise Failure(f"{row['hook']}: role outside closed enum")
        if not isinstance(row["species"], list) or not set(row["species"]) <= species_enum:
            raise Failure(f"{row['hook']}: species outside closed enum")
        if row["consumer"] != row["path"] or file_sha(ROOT / row["path"]) != row["baseline_file_sha256"]:
            raise Failure(f"{row['hook']}: candidate-free consumer pin mismatch")
        if file_sha(ROOT / row["l0"]) != row["l0_sha256"]:
            raise Failure(f"{row['hook']}: L0 pin mismatch")
        text = (ROOT / row["path"]).read_text(encoding="utf-8")
        if text.count(row["anchor_needle"]) < 1:
            raise Failure(f"{row['hook']}: baseline anchor absent")
    if not isinstance(manifest["forbidden_bindings"], list) or len(manifest["forbidden_bindings"]) < 10:
        raise Failure("manifest forbidden binding policy adulterated")


def validate_source_evidence(source: Any, manifest: dict[str, Any]) -> None:
    exact_keys(source, {"schema", "manifest_sha256", "candidate_tree_sha256", "inspector_receipt_sha256", "single_writer", "projection_read_only", "cfg_test_decoys_rejected", "anchors"}, "evidence.source")
    if source["schema"] != "p1342-source-inspection-evidence-v2": raise Failure("source evidence schema mismatch")
    for name in ["manifest_sha256", "candidate_tree_sha256", "inspector_receipt_sha256"]:
        if not isinstance(source[name], str) or not HEX64.fullmatch(source[name]): raise Failure(f"source evidence {name} invalid")
    if source["manifest_sha256"] != file_sha(ROOT / "00_nucleo/diagnosticos/p1342-contract-binding-r2.json"):
        raise Failure("source evidence manifest pin mismatch")
    for name in ["single_writer", "projection_read_only", "cfg_test_decoys_rejected"]:
        if boolean(source[name], f"source.{name}") is not True: raise Failure(f"source evidence {name} failed")
    if not isinstance(source["anchors"], list) or len(source["anchors"]) != 19:
        raise Failure("source evidence requires all 19 manifest rows")
    by_hook = {item.get("hook"): item for item in source["anchors"] if isinstance(item, dict)}
    if set(by_hook) != set(MANIFEST_HOOKS): raise Failure("source evidence hook set mismatch")
    for row in manifest["hooks"]:
        item = by_hook[row["hook"]]
        exact_keys(item, {"hook", "manifest_row_sha256", "symbol", "branch", "status", "fixture_reached", "candidate_adjacent_cfg"}, f"source anchor {row['hook']}")
        if item["manifest_row_sha256"] != digest(row) or item["symbol"] != row["symbol"] or item["branch"] != row["branch"]:
            raise Failure(f"source anchor {row['hook']} not pinned to exact row")
        if item["status"] != "rust-symbol-branch-resolved-once" or item["candidate_adjacent_cfg"] != "p1339_observation-verified":
            raise Failure(f"source anchor {row['hook']} not productively inspected")
        if boolean(item["fixture_reached"], f"source anchor {row['hook']}.fixture_reached") is not True:
            raise Failure(f"source anchor {row['hook']} not reached")


def validate_event(event: Any, commitment: str, where: str, ids: set[str]) -> tuple[Any, bool]:
    exact_keys(event, {"seq", "kind", "hook", "refs", "data"}, where)
    integer(event["seq"], f"{where}.seq")
    kind = event["kind"]
    if kind not in EVENT_HOOK or event["hook"] != EVENT_HOOK[kind]: raise Failure(f"{where}: closed kind/hook mismatch")
    exact_keys(event["refs"], REFS[kind], f"{where}.refs")
    exact_keys(event["data"], DATA[kind], f"{where}.data")
    for name, value in event["refs"].items():
        ids.add(validate_id(value, ID_PREFIX[name], commitment, f"{where}.refs.{name}"))
    opaque = False
    data = event["data"]
    if kind == "attempt-open":
        if data["attempt_kind"] not in {"discovery", "selected"}: raise Failure(f"{where}: attempt kind enum")
        integer(data["attempt_index"], f"{where}.attempt_index")
    elif kind == "context-dispatch": span(data["context_body_span"], (131, 177), where)
    elif kind == "dict-prebound-produced":
        span(data["span"], (155, 162), where)
        value, opaque = typed_value(data["typed_value"], where)
        if value != ("dict", (("a", ("int", 1)),)): raise Failure(f"{where}: ordered prebound Dict mismatch")
    elif kind == "counter-occurrence-created":
        span(data["source_span"], (144, 164), where)
        value, _ = typed_value(data["counter_key"], where)
        if value != ("selector", "heading.where", (("level", ("int", 1)),)) or data["action"] != "Func": raise Failure(f"{where}: counter key/action mismatch")
    elif kind == "attempt-result":
        if data["result_kind"] != "success" or data["incorporation_state"] not in {"retired", "incorporated", "not-incorporated"}: raise Failure(f"{where}: closed result mismatch")
    elif kind == "counter-occurrence-walked":
        value, _ = typed_value(data["counter_key"], where)
        if value != ("selector", "heading.where", (("level", ("int", 1)),)) or data["action"] != "Func": raise Failure(f"{where}: walked key/action mismatch")
    elif kind == "callback-replay-enter":
        value, _ = typed_value(data["snapshot_pre_value"], where)
        if value != ("int", 0): raise Failure(f"{where}: snapshot pre mismatch")
    elif kind == "func-dispatch-with" and data["repr_kind"] != "With": raise Failure(f"{where}: wrapper repr mismatch")
    elif kind == "with-edge": span(data["prebound_args_span"], (155, 162), where)
    elif kind == "func-dispatch-inner" and data["repr_kind"] != "Closure": raise Failure(f"{where}: inner repr mismatch")
    elif kind == "syntax-body-enter":
        if data["syntax_kind"] != "CodeBlock": raise Failure(f"{where}: syntax-body must be SyntaxNode CodeBlock, never Func")
        span(data["span"], (64, 121), where)
    elif kind == "dict-witness-produced":
        span(data["span"], (82, 97), where)
        value, opaque = typed_value(data["typed_value"], where, opaque_allowed=True)
        if not opaque and value != ("dict", (("outer", ("dict", (("a", ("int", 1)),))),)): raise Failure(f"{where}: ordered witness Dict mismatch")
    elif kind == "syntax-body-exit":
        value, _ = typed_value(data["outcome"], where)
        if value != ("int", 1): raise Failure(f"{where}: body outcome mismatch")
    elif kind == "callback-replay-exit":
        pre, _ = typed_value(data["snapshot_pre_value"], where)
        post, _ = typed_value(data["snapshot_post_value"], where)
        outcome, _ = typed_value(data["outcome"], where)
        if (pre, post, outcome) != (("int", 0), ("int", 1), ("int", 1)): raise Failure(f"{where}: real pre/post/outcome mismatch")
    elif kind == "attempt-close":
        if data["attempt_kind"] not in {"discovery", "selected"}: raise Failure(f"{where}: close kind mismatch")
        integer(data["final_event_count"], f"{where}.final_event_count")
    return event, opaque


def cell_semantics(cell: dict[str, Any], runtime_hits: dict[str, int], commitment: str) -> bool:
    events = cell["events"]
    kinds = [event["kind"] for event in events]
    open_event = events[0]
    attempt_kind = open_event["data"]["attempt_kind"]
    h00d, h00s = runtime_hits.get(MANIFEST_HOOKS[0], 0), runtime_hits.get(MANIFEST_HOOKS[1], 0)
    derived_kind = "discovery" if (h00d, h00s) == (1, 0) else "selected" if (h00d, h00s) == (0, 1) else None
    if derived_kind != attempt_kind: raise Failure("attempt-kind must derive exclusively from mutually exclusive H00 receipt")
    result = next((e for e in events if e["kind"] == "attempt-result"), None)
    if result is None: raise Failure("attempt-result missing")
    incorporated = result["data"]["incorporation_state"] == "incorporated"
    r_value = runtime_hits.get("H12A-replay-enter", 0)
    expected_counts = expected_hook_counts(attempt_kind, r_value, incorporated)
    applicable = {hook for hook in MANIFEST_HOOKS if hook != (MANIFEST_HOOKS[1] if attempt_kind == "discovery" else MANIFEST_HOOKS[0])}
    if len(applicable) != 18 or set(runtime_hits) != applicable:
        raise Failure("each origin cell requires exactly 18 applicable hook receipts")
    if any(runtime_hits[hook] != expected_counts[hook] for hook in applicable):
        raise Failure("runtime hook cardinality disagrees with attempt-kind/R relations")
    counts = Counter(kinds)
    base = {"attempt-open":1, "context-dispatch":1, "dict-prebound-produced":1, "counter-occurrence-created":1, "attempt-result":1, "attempt-close":1}
    for kind, count in base.items():
        if counts[kind] != count: raise Failure(f"{kind}: expected {count}")
    relational = {"counter-occurrence-walked":1 if incorporated else 0, "callback-replay-enter":r_value, "func-dispatch-with":r_value, "with-edge":r_value, "func-dispatch-inner":r_value, "syntax-body-enter":r_value, "dict-witness-produced":r_value, "syntax-body-exit":r_value, "callback-replay-exit":r_value}
    for kind, count in relational.items():
        if counts[kind] != count: raise Failure(f"{kind}: relational cardinality expected {count}")
    if attempt_kind == "discovery" and (incorporated or r_value != 0 or counts["counter-occurrence-walked"]):
        raise Failure("discovery must have H11=0 and R=0")
    if events[-1]["kind"] != "attempt-close" or events[-1]["data"]["final_event_count"] != len(events):
        raise Failure("H16 close must be last with exact event count")
    if result["seq"] >= events[-1]["seq"]: raise Failure("H03 result is not an early result boundary")
    refs0 = open_event["refs"]
    session = refs0["session_carrier"]
    occurrence = next(e for e in events if e["kind"] == "counter-occurrence-created")["refs"]
    for event in events:
        if event["refs"]["origin_cell"] != cell["origin_cell"] or event["refs"]["session_carrier"] != session:
            raise Failure("origin/session carrier discontinuity")
        if "occurrence_carrier" in event["refs"] and event["refs"]["occurrence_carrier"] != occurrence["occurrence_carrier"]:
            raise Failure("occurrence carrier discontinuity")
    if r_value:
        by_kind = {kind: [e for e in events if e["kind"] == kind] for kind in kinds}
        walk, enter, with_d, edge, inner, body_in, witness, body_out, replay_out = [by_kind[k][0] for k in ["counter-occurrence-walked","callback-replay-enter","func-dispatch-with","with-edge","func-dispatch-inner","syntax-body-enter","dict-witness-produced","syntax-body-exit","callback-replay-exit"]]
        order = [walk, enter, with_d, edge, inner, body_in, witness, body_out, replay_out]
        if [e["seq"] for e in order] != sorted(e["seq"] for e in order): raise Failure("causal order inversion")
        if not all(e["refs"]["consumer_cell"] == cell["origin_cell"] for e in order[1:]): raise Failure("consumer cell mismatch")
        if not (walk["refs"]["location"] == enter["refs"]["location"] == replay_out["refs"]["location"]): raise Failure("Location continuity lost")
        if enter["refs"]["snapshot_pre"] != replay_out["refs"]["snapshot_pre"]: raise Failure("snapshot-pre identity continuity lost")
        validate_id(replay_out["refs"]["snapshot_post"], "snapshot-post", commitment, "snapshot-post")
        if edge["refs"]["callback_with"] != with_d["refs"]["callback_with"] or edge["refs"]["func_callback"] != inner["refs"]["func_callback"]:
            raise Failure("With endpoints disconnected/inverted")
        if body_in["refs"]["func_callback"] != edge["refs"]["func_callback"] or body_out["refs"]["func_callback"] != edge["refs"]["func_callback"]:
            raise Failure("Func callback to syntax-body edge lost")
        if not (body_in["refs"]["syntax_body"] == witness["refs"]["syntax_body"] == body_out["refs"]["syntax_body"]):
            raise Failure("SyntaxNode identity continuity lost")
    return any(e["kind"] == "dict-witness-produced" and e["data"]["typed_value"].get("visibility") == "opaque" for e in events)


def validate_run(run: Any, runtime: Any, challenge_sets: dict[str, set[str]]) -> bool:
    exact_keys(run, {"mode", "challenge", "challenge_commitment", "raw_snapshot", "projection", "classification"}, "run")
    if run["mode"] not in MODES or not isinstance(run["challenge"], str) or not re.fullmatch(r"[0-9a-f]{64}", run["challenge"]): raise Failure("run challenge must be 32-byte lowercase hex")
    challenge = bytes.fromhex(run["challenge"])
    commitment = hashlib.sha256(challenge).hexdigest()
    if run["challenge_commitment"] != commitment: raise Failure("challenge commitment mismatch")
    raw = run["raw_snapshot"]
    exact_keys(raw, {"schema", "challenge_commitment", "frozen_before_projection", "append_receipts", "cells", "raw_digest"}, "raw snapshot")
    if raw["schema"] != "p1342-raw-ledger-snapshot-v2" or raw["challenge_commitment"] != commitment or boolean(raw["frozen_before_projection"], "raw frozen") is not True: raise Failure("raw snapshot header mismatch")
    if not isinstance(raw["cells"], list) or not isinstance(raw["append_receipts"], list): raise Failure("raw cells/receipts arrays required")
    ids: set[str] = set()
    flattened = []
    opaque = False
    for c_index, cell in enumerate(raw["cells"]):
        exact_keys(cell, {"origin_cell", "events"}, f"cell[{c_index}]")
        validate_id(cell["origin_cell"], "cell", commitment, f"cell[{c_index}].origin_cell")
        if not isinstance(cell["events"], list): raise Failure("cell events array required")
        for e_index, event in enumerate(cell["events"]):
            _, event_opaque = validate_event(event, commitment, f"cell[{c_index}].event[{e_index}]", ids)
            opaque = opaque or event_opaque
            flattened.append((cell["origin_cell"], event))
    flattened.sort(key=lambda pair: pair[1]["seq"])
    if [event["seq"] for _, event in flattened] != list(range(len(flattened))): raise Failure("raw global seq is not append-tail contiguous")
    if len(raw["append_receipts"]) != len(flattened): raise Failure("one append receipt per raw event required")
    previous = None
    hits: dict[tuple[str,str], int] = defaultdict(int)
    for index, ((origin, event), receipt) in enumerate(zip(flattened, raw["append_receipts"])):
        exact_keys(receipt, {"seq", "hook", "hook_hit", "origin_cell", "event_digest", "prev_receipt", "receipt"}, f"receipt[{index}]")
        hits[(origin, event["hook"])] += 1
        event_hash = digest(event)
        expected_receipt = receipt_hash(challenge, index, previous, event["hook"], event_hash)
        if receipt != {"seq":index, "hook":event["hook"], "hook_hit":hits[(origin,event["hook"])], "origin_cell":origin, "event_digest":event_hash, "prev_receipt":previous, "receipt":expected_receipt}:
            raise Failure(f"receipt[{index}]: SHA-256 append chain mismatch")
        previous = expected_receipt
    raw_without_digest = {key: value for key, value in raw.items() if key != "raw_digest"}
    if raw["raw_digest"] != digest(raw_without_digest): raise Failure("raw preprojection digest mismatch")
    projection = run["projection"]
    exact_keys(projection, {"schema", "challenge_commitment", "raw_digest_before", "raw_digest_after", "cells"}, "projection")
    if projection != {"schema":"p1342-ledger-projection-v2", "challenge_commitment":commitment, "raw_digest_before":raw["raw_digest"], "raw_digest_after":raw["raw_digest"], "cells":raw["cells"]}:
        raise Failure("projection is not a read-only image of the frozen raw snapshot")
    exact_keys(runtime, {"mode", "challenge_commitment", "challenge_issuer_receipt_sha256", "raw_object_id", "raw_digest_before", "raw_digest_after", "callback_apply_count", "observation_callback_count", "hook_hits", "observed_selected_endpoints"}, f"runtime evidence {run['mode']}")
    if runtime["mode"] != run["mode"] or runtime["challenge_commitment"] != commitment: raise Failure("runtime evidence run/challenge mismatch")
    for field in ["challenge_issuer_receipt_sha256"]:
        if not isinstance(runtime[field], str) or not HEX64.fullmatch(runtime[field]): raise Failure("runtime issuer receipt invalid")
    ids.add(validate_id(runtime["raw_object_id"], "raw-object", commitment, "runtime raw object"))
    if runtime["raw_digest_before"] != raw["raw_digest"] or runtime["raw_digest_after"] != raw["raw_digest"]: raise Failure("independent raw before/after digest mismatch")
    if integer(runtime["observation_callback_count"], "observation callback count") != 0: raise Failure("observation executed an extra callback")
    if not isinstance(runtime["hook_hits"], list): raise Failure("runtime hook hits array required")
    hit_by_cell: dict[str, dict[str,int]] = defaultdict(dict)
    for h_index, hit in enumerate(runtime["hook_hits"]):
        exact_keys(hit, {"hook", "origin_cell", "count", "receipt_tail"}, f"runtime hit[{h_index}]")
        if hit["hook"] not in MANIFEST_HOOKS: raise Failure("runtime evidence unknown hook")
        validate_id(hit["origin_cell"], "cell", commitment, "runtime hit origin")
        count = integer(hit["count"], "runtime hit count")
        if count < 0 or hit["hook"] in hit_by_cell[hit["origin_cell"]]: raise Failure("runtime hook duplicate/negative")
        if (count == 0 and hit["receipt_tail"] is not None) or (count > 0 and (not isinstance(hit["receipt_tail"], str) or not HEX64.fullmatch(hit["receipt_tail"]))): raise Failure("runtime receipt tail/count mismatch")
        hit_by_cell[hit["origin_cell"]][hit["hook"]] = count
    total_r = 0
    for cell in raw["cells"]:
        if cell["origin_cell"] not in hit_by_cell: raise Failure("runtime hook evidence missing cell")
        opaque = cell_semantics(cell, hit_by_cell[cell["origin_cell"]], commitment) or opaque
        total_r += hit_by_cell[cell["origin_cell"]].get("H12A-replay-enter", 0)
    exact_keys(runtime["observed_selected_endpoints"], {"location", "snapshot_pre", "snapshot_post"}, "runtime observed selected endpoints")
    selected_cell = next(cell for cell in raw["cells"] if cell["events"][0]["data"]["attempt_kind"] == "selected")
    replay_exit = next(event for event in selected_cell["events"] if event["kind"] == "callback-replay-exit")
    if runtime["observed_selected_endpoints"] != {
        "location": replay_exit["refs"]["location"],
        "snapshot_pre": replay_exit["refs"]["snapshot_pre"],
        "snapshot_post": make_id("snapshot-post", commitment, "selected"),
    }:
        raise Failure("independent runtime Location/snapshot endpoint evidence mismatch")
    if replay_exit["refs"]["snapshot_post"] != runtime["observed_selected_endpoints"]["snapshot_post"]:
        raise Failure("raw snapshot-post is not the independently observed productive endpoint")
    if integer(runtime["callback_apply_count"], "callback apply count") != 2 * total_r: raise Failure("callback apply count must equal 2*R")
    if ids & set().union(*(values for key, values in challenge_sets.items() if key != run["mode"])):
        raise Failure("challenge-bound identity sets overlap across normal/repeat/reverse")
    challenge_sets[run["mode"]] = ids
    return opaque


def judge(dto: Any, evidence: Any, trusted_evidence_sha256: str, contract: dict[str, Any], manifest: dict[str, Any], pins: dict[str,str]) -> str:
    # Precedence 1: externally supplied immutable pins and manifest metadata.
    if digest(evidence) != trusted_evidence_sha256: raise Failure("external evidence pin mismatch")
    validate_manifest(manifest, pins["binding"], contract)
    exact_keys(dto, {"schema", "contract_sha256", "binding_manifest_sha256", "fixture_sha256", "runs"}, "DTO")
    if dto["schema"] != "p1342-composite-ledger-dto-v2" or dto["contract_sha256"] != pins["contract"] or dto["binding_manifest_sha256"] != pins["binding"] or dto["fixture_sha256"] != pins["fixture"]:
        raise Failure("DTO external pins mismatch")
    exact_keys(evidence, {"schema", "contract_sha256", "binding_manifest_sha256", "fixture_sha256", "l0_freeze_sha256", "source_inspection", "runtime_runs"}, "evidence")
    if evidence["schema"] != "p1342-independent-evidence-v2" or [evidence["contract_sha256"], evidence["binding_manifest_sha256"], evidence["fixture_sha256"], evidence["l0_freeze_sha256"]] != [pins["contract"], pins["binding"], pins["fixture"], pins["l0_freeze"]]:
        raise Failure("evidence external pins mismatch")
    validate_source_evidence(evidence["source_inspection"], manifest)
    # Precedence 2-4: challenges, raw/receipts/projection, schemas and semantics.
    if not isinstance(dto["runs"], list) or [run.get("mode") for run in dto["runs"]] != MODES: raise Failure("DTO requires normal/repeat/reverse in closed order")
    if not isinstance(evidence["runtime_runs"], list) or [run.get("mode") for run in evidence["runtime_runs"]] != MODES: raise Failure("evidence requires normal/repeat/reverse")
    commitments = [run.get("challenge_commitment") for run in dto["runs"]]
    challenges = [run.get("challenge") for run in dto["runs"]]
    if len(set(commitments)) != 3 or len(set(challenges)) != 3: raise Failure("normal/repeat/reverse challenges must be pairwise fresh")
    identity_sets: dict[str,set[str]] = {}
    opaque_flags = [validate_run(run, evidence["runtime_runs"][index], identity_sets) for index, run in enumerate(dto["runs"])]
    if len(set(opaque_flags)) != 1: raise Failure("payload opacity differs across run modes")
    computed = "Unknown" if opaque_flags[0] else "Preserved"
    if any(run["classification"] != computed for run in dto["runs"]): raise Failure("DTO classification field disagrees with computed classification")
    return computed


def event(kind: str, refs: dict[str,str], data: dict[str,Any]) -> dict[str,Any]:
    return {"seq": -1, "kind": kind, "hook": EVENT_HOOK[kind], "refs": refs, "data": data}


def build_events(mode: str, challenge: str, attempt_kind: str) -> tuple[str,list[dict[str,Any]]]:
    commitment = hashlib.sha256(bytes.fromhex(challenge)).hexdigest()
    ident = lambda domain, label: make_id(domain, commitment, label)
    cell = ident("cell", attempt_kind)
    base = {"run":ident("run",mode), "ledger":ident("ledger","focal"), "origin_cell":cell, "session_carrier":ident("session-carrier",attempt_kind)}
    prebound = {"type":"dict", "visibility":"inspectable", "entries":[["a", {"type":"int","value":1}]]}
    key = {"type":"selector", "shape":"heading.where", "ordered_fields":[["level", {"type":"int","value":1}]]}
    events = [
        event("attempt-open", dict(base), {"attempt_kind":attempt_kind,"attempt_index":0 if attempt_kind=="discovery" else 1}),
        event("context-dispatch", {**base,"context_closure":ident("context-closure",attempt_kind)}, {"context_body_span":[131,177]}),
        event("dict-prebound-produced", {**base,"dict_prebound":ident("dict-prebound",attempt_kind)}, {"span":[155,162],"typed_value":prebound}),
        event("counter-occurrence-created", {**base,"occurrence_carrier":ident("occurrence-carrier",attempt_kind),"callback_with":ident("callback-with",attempt_kind),"func_callback":ident("func-callback",attempt_kind)}, {"source_span":[144,164],"counter_key":key,"action":"Func"}),
        event("attempt-result", dict(base), {"result_kind":"success","incorporation_state":"retired" if attempt_kind=="discovery" else "incorporated"}),
    ]
    if attempt_kind == "selected":
        common = {**base,"consumer_cell":cell,"occurrence_carrier":ident("occurrence-carrier",attempt_kind)}
        events.extend([
            event("counter-occurrence-walked", {**base,"occurrence_carrier":common["occurrence_carrier"],"counter_content":ident("counter-content",attempt_kind),"location":ident("location",attempt_kind)}, {"counter_key":key,"action":"Func"}),
            event("callback-replay-enter", {**common,"callback_with":ident("callback-with",attempt_kind),"location":ident("location",attempt_kind),"snapshot_pre":ident("snapshot-pre",attempt_kind)}, {"snapshot_pre_value":{"type":"int","value":0}}),
            event("func-dispatch-with", {**common,"callback_with":ident("callback-with",attempt_kind)}, {"repr_kind":"With"}),
            event("with-edge", {**common,"callback_with":ident("callback-with",attempt_kind),"func_callback":ident("func-callback",attempt_kind)}, {"prebound_args_span":[155,162]}),
            event("func-dispatch-inner", {**common,"func_callback":ident("func-callback",attempt_kind)}, {"repr_kind":"Closure"}),
            event("syntax-body-enter", {**common,"func_callback":ident("func-callback",attempt_kind),"syntax_body":ident("syntax-node",attempt_kind)}, {"syntax_kind":"CodeBlock","span":[64,121]}),
            event("dict-witness-produced", {**common,"syntax_body":ident("syntax-node",attempt_kind),"dict_witness":ident("dict-witness",attempt_kind)}, {"span":[82,97],"typed_value":{"type":"dict","visibility":"inspectable","entries":[["outer",prebound]]}}),
            event("syntax-body-exit", {**common,"func_callback":ident("func-callback",attempt_kind),"syntax_body":ident("syntax-node",attempt_kind)}, {"outcome":{"type":"int","value":1}}),
            event("callback-replay-exit", {**common,"location":ident("location",attempt_kind),"snapshot_pre":ident("snapshot-pre",attempt_kind),"snapshot_post":ident("snapshot-post",attempt_kind)}, {"snapshot_pre_value":{"type":"int","value":0},"snapshot_post_value":{"type":"int","value":1},"outcome":{"type":"int","value":1}}),
        ])
    events.append(event("attempt-close", dict(base), {"attempt_kind":attempt_kind,"final_event_count":len(events)+1}))
    return cell, events


def mutate_pre_finalize(case_id: str, runs: list[dict[str,Any]], manifest: dict[str,Any]) -> None:
    def selected(run: dict[str,Any]) -> list[dict[str,Any]]: return run["cells"][1]["events"]
    if case_id == "A01-dead-h06-branch-anchor":
        row = next(x for x in manifest["hooks"] if x["hook"].startswith("H06")); row["branch"]="display dead branch"; row["anchor_needle"]='"display" => {'
    elif case_id == "A02-dead-h15-else-anchor":
        row = next(x for x in manifest["hooks"] if x["hook"].startswith("H15")); row["branch"]="Value::None else"; row["anchor_needle"]="Value::None"
    elif case_id == "A03-fabricated-h08-test-anchor":
        row = next(x for x in manifest["hooks"] if x["hook"].startswith("H08")); row["symbol"]="cfg_test_decoy"; row["branch"]="#[cfg(test)]"
    elif case_id == "A04-binding-metadata-adulterated":
        manifest["revision"]=True; manifest["baseline"]={"forged":True}; manifest["forbidden_bindings"]=[]
    elif case_id == "A05-invalid-anchor-scope-enum":
        next(x for x in manifest["hooks"] if x["hook"].startswith("H09"))["anchor_scope"]="fabricated-scope"
    for run in runs:
        events = selected(run)
        by = {event["kind"]:event for event in events}
        if case_id == "A08-carrier-lost-outer-dispatch": by["func-dispatch-with"]["refs"]["occurrence_carrier"] = make_id("occurrence-carrier", run["commitment"], "lost-outer")
        elif case_id == "A09-carrier-lost-with-edge": by["with-edge"]["refs"]["occurrence_carrier"] = make_id("occurrence-carrier", run["commitment"], "lost-edge")
        elif case_id == "A10-carrier-lost-inner-dispatch": by["func-dispatch-inner"]["refs"]["occurrence_carrier"] = make_id("occurrence-carrier", run["commitment"], "lost-inner")
        elif case_id == "A11-carrier-lost-closure-dispatch": by["syntax-body-enter"]["refs"]["func_callback"] = make_id("func-callback", run["commitment"], "lost-closure")
        elif case_id == "A12-carrier-lost-body-enter": by["syntax-body-enter"]["refs"]["occurrence_carrier"] = make_id("occurrence-carrier", run["commitment"], "lost-enter")
        elif case_id == "A13-carrier-lost-body-exit": by["syntax-body-exit"]["refs"]["occurrence_carrier"] = make_id("occurrence-carrier", run["commitment"], "lost-exit")
        elif case_id == "A14-location-lost-replay-exit": by["callback-replay-exit"]["refs"]["location"] = make_id("location", run["commitment"], "other")
        elif case_id == "A15-snapshot-post-fabricated": by["callback-replay-exit"]["refs"]["snapshot_post"] = make_id("snapshot-post", run["commitment"], "fabricated")
        elif case_id == "A16-extra-event-inserted":
            extra=copy.deepcopy(by["dict-prebound-produced"]); extra["kind"]="dict-decoy-produced"; events.insert(-1,extra)
        elif case_id == "A17-extra-event-duplicated": events.insert(-1,copy.deepcopy(by["dict-witness-produced"]))
        elif case_id == "A18-bool-as-int-in-unvalidated-event":
            extra=copy.deepcopy(by["dict-prebound-produced"]); extra["data"]["typed_value"]["entries"][0][1]["value"]=True; events.insert(-1,extra)
        elif case_id == "A19-swapped-span-in-unvalidated-event":
            extra=copy.deepcopy(by["dict-prebound-produced"]); extra["data"]["span"]=[64,121]; events.insert(-1,extra)
        elif case_id == "A20-r-self-declared-by-fabricated-events":
            by["attempt-open"]["data"]["attempt_kind"]="discovery"; by["attempt-close"]["data"]["attempt_kind"]="discovery"
        elif case_id == "A21-unknown-before-carrier-proof":
            by["syntax-body-enter"]["refs"]["occurrence_carrier"] = make_id("occurrence-carrier", run["commitment"], "lost-before-unknown")
            by["dict-witness-produced"]["data"]["typed_value"]={"type":"dict","visibility":"opaque","commitment":"3f5b8f852dd5bba8fa19de766caf42ef54f0e060ca9faf9e0a82911926d86eb6"}
        elif case_id == "P02-opaque-composite":
            by["dict-witness-produced"]["data"]["typed_value"]={"type":"dict","visibility":"opaque","commitment":"3f5b8f852dd5bba8fa19de766caf42ef54f0e060ca9faf9e0a82911926d86eb6"}


def fixed_ids(item: Any) -> Any:
    if isinstance(item, str) and re.match(r"^[a-z-]+:[0-9a-f]{16}:", item):
        parts=item.split(":",2); return f"{parts[0]}:fixedfixedfixedfix:{parts[2]}"
    if isinstance(item,list): return [fixed_ids(x) for x in item]
    if isinstance(item,dict): return {k:fixed_ids(v) for k,v in item.items()}
    return item


def finalize_run(run_blueprint: dict[str,Any]) -> dict[str,Any]:
    challenge = bytes.fromhex(run_blueprint["challenge"])
    commitment = run_blueprint["commitment"]
    seq=0
    flattened=[]
    cells=[]
    for cell in run_blueprint["cells"]:
        events=[]
        for value in cell["events"]:
            value=copy.deepcopy(value); value["seq"]=seq; seq+=1; events.append(value); flattened.append((cell["origin_cell"],value))
        cells.append({"origin_cell":cell["origin_cell"],"events":events})
    receipts=[]; previous=None; hits=defaultdict(int)
    for index,(origin,value) in enumerate(flattened):
        hits[(origin,value["hook"])]+=1; event_hash=digest(value); current=receipt_hash(challenge,index,previous,value["hook"],event_hash)
        receipts.append({"seq":index,"hook":value["hook"],"hook_hit":hits[(origin,value["hook"])],"origin_cell":origin,"event_digest":event_hash,"prev_receipt":previous,"receipt":current}); previous=current
    raw_base={"schema":"p1342-raw-ledger-snapshot-v2","challenge_commitment":commitment,"frozen_before_projection":True,"append_receipts":receipts,"cells":cells}
    raw={**raw_base,"raw_digest":digest(raw_base)}
    opaque=any(e["kind"]=="dict-witness-produced" and e["data"]["typed_value"].get("visibility")=="opaque" for _,e in flattened)
    return {"mode":run_blueprint["mode"],"challenge":run_blueprint["challenge"],"challenge_commitment":commitment,"raw_snapshot":raw,"projection":{"schema":"p1342-ledger-projection-v2","challenge_commitment":commitment,"raw_digest_before":raw["raw_digest"],"raw_digest_after":raw["raw_digest"],"cells":copy.deepcopy(cells)},"classification":"Unknown" if opaque else "Preserved"}


def build_evidence(dto: dict[str,Any], manifest: dict[str,Any], pins: dict[str,str]) -> dict[str,Any]:
    anchors=[{"hook":row["hook"],"manifest_row_sha256":digest(row),"symbol":row["symbol"],"branch":row["branch"],"status":"rust-symbol-branch-resolved-once","fixture_reached":True,"candidate_adjacent_cfg":"p1339_observation-verified"} for row in manifest["hooks"]]
    source={"schema":"p1342-source-inspection-evidence-v2","manifest_sha256":pins["binding"],"candidate_tree_sha256":hashlib.sha256(b"oracle-positive-source-evidence").hexdigest(),"inspector_receipt_sha256":hashlib.sha256(b"oracle-independent-inspector").hexdigest(),"single_writer":True,"projection_read_only":True,"cfg_test_decoys_rejected":True,"anchors":anchors}
    runtime=[]
    for run in dto["runs"]:
        hits=[]
        raw=run["raw_snapshot"]
        for cell in raw["cells"]:
            kind=cell["events"][0]["data"]["attempt_kind"]
            incorporated=next(e for e in cell["events"] if e["kind"]=="attempt-result")["data"]["incorporation_state"]=="incorporated"
            r_value=sum(e["kind"]=="callback-replay-enter" for e in cell["events"])
            counts=expected_hook_counts(kind,r_value,incorporated)
            excluded="H00S-attempt-kind-selected" if kind=="discovery" else "H00D-attempt-kind-discovery"
            for hook_name in MANIFEST_HOOKS:
                if hook_name==excluded: continue
                count=counts[hook_name]
                tail=hashlib.sha256(f"{run['challenge_commitment']}:{cell['origin_cell']}:{hook_name}:{count}".encode()).hexdigest() if count else None
                hits.append({"hook":hook_name,"origin_cell":cell["origin_cell"],"count":count,"receipt_tail":tail})
        total_r=sum(hit["count"] for hit in hits if hit["hook"]=="H12A-replay-enter")
        selected_cell=raw["cells"][1]
        replay_exit=next(event for event in selected_cell["events"] if event["kind"]=="callback-replay-exit")
        runtime.append({"mode":run["mode"],"challenge_commitment":run["challenge_commitment"],"challenge_issuer_receipt_sha256":hashlib.sha256(("issuer:"+run["challenge"]).encode()).hexdigest(),"raw_object_id":make_id("raw-object",run["challenge_commitment"],"frozen"),"raw_digest_before":raw["raw_digest"],"raw_digest_after":raw["raw_digest"],"callback_apply_count":2*total_r,"observation_callback_count":0,"hook_hits":hits,"observed_selected_endpoints":{"location":replay_exit["refs"]["location"],"snapshot_pre":replay_exit["refs"]["snapshot_pre"],"snapshot_post":make_id("snapshot-post",run["challenge_commitment"],"selected")}})
    return {"schema":"p1342-independent-evidence-v2","contract_sha256":pins["contract"],"binding_manifest_sha256":pins["binding"],"fixture_sha256":pins["fixture"],"l0_freeze_sha256":pins["l0_freeze"],"source_inspection":source,"runtime_runs":runtime}


def build_case(case_id: str, corpus: dict[str,Any], base_manifest: dict[str,Any], pins: dict[str,str]) -> tuple[dict[str,Any],dict[str,Any],dict[str,Any],str]:
    manifest=copy.deepcopy(base_manifest)
    blueprints=[]
    for mode in MODES:
        challenge=corpus["challenges"][mode]; commitment=hashlib.sha256(bytes.fromhex(challenge)).hexdigest()
        d_cell,d_events=build_events(mode,challenge,"discovery"); s_cell,s_events=build_events(mode,challenge,"selected")
        blueprints.append({"mode":mode,"challenge":challenge,"commitment":commitment,"cells":[{"origin_cell":d_cell,"events":d_events},{"origin_cell":s_cell,"events":s_events}]})
    mutate_pre_finalize(case_id,blueprints,manifest)
    if case_id=="A06-fixed-runtime-ids-across-runs": blueprints=fixed_ids(blueprints)
    runs=[finalize_run(run) for run in blueprints]
    dto={"schema":"p1342-composite-ledger-dto-v2","contract_sha256":pins["contract"],"binding_manifest_sha256":pins["binding"],"fixture_sha256":pins["fixture"],"runs":runs}
    evidence=build_evidence(dto,manifest,pins)
    trusted=digest(evidence)
    if case_id=="A07-posthoc-event-ids-and-ordinals":
        for run in dto["runs"]:
            events=run["projection"]["cells"][1]["events"]
            for index,value in enumerate(events): value["seq"]=1000+index
    return dto,evidence,manifest,trusted


def main() -> int:
    parser=argparse.ArgumentParser()
    parser.add_argument("--contract",required=True,type=Path); parser.add_argument("--manifest",required=True,type=Path); parser.add_argument("--corpus",required=True,type=Path)
    args=parser.parse_args()
    contract=json.loads(args.contract.read_text()); manifest=json.loads(args.manifest.read_text()); corpus=json.loads(args.corpus.read_text())
    exact_keys(corpus,{"schema","step","revision","role","executor","regime","status","protected_inputs","challenges","composite_schema","evidence_schema","cases","closed_case_order","scope_exclusions"},"corpus")
    if corpus["schema"]!="p1342-oracle-corpus-v2" or integer(corpus["revision"],"corpus.revision")!=2: raise Failure("corpus identity mismatch")
    exact_keys(corpus["challenges"], set(MODES), "corpus.challenges")
    if any(not isinstance(value,str) or not re.fullmatch(r"[0-9a-f]{64}",value) for value in corpus["challenges"].values()) or len(set(corpus["challenges"].values()))!=3:
        raise Failure("corpus requires three distinct 32-byte challenge vectors")
    exact_keys(corpus["composite_schema"], {"dto_keys","run_keys","raw_keys","receipt_keys","cell_keys","event_keys","projection_keys","receipt_formula","raw_digest_formula","projection_rule"}, "corpus.composite_schema")
    if corpus["composite_schema"]["dto_keys"] != ["schema","contract_sha256","binding_manifest_sha256","fixture_sha256","runs"] or corpus["composite_schema"]["event_keys"] != ["seq","kind","hook","refs","data"]:
        raise Failure("corpus composite schema declaration adulterated")
    exact_keys(corpus["evidence_schema"], {"separate_from_dto","pin_rule","source_keys","anchor_keys","runtime_keys","hook_hit_keys","static_binding_rows","applicable_hook_rows_per_origin_cell","mutual_exclusion"}, "corpus.evidence_schema")
    if boolean(corpus["evidence_schema"]["separate_from_dto"], "evidence_schema.separate_from_dto") is not True or integer(corpus["evidence_schema"]["static_binding_rows"], "evidence_schema.static_binding_rows")!=19 or integer(corpus["evidence_schema"]["applicable_hook_rows_per_origin_cell"], "evidence_schema.applicable")!=18:
        raise Failure("corpus evidence/static-hook schema declaration mismatch")
    for pin in corpus["protected_inputs"].values():
        if not isinstance(pin,list) or len(pin)!=2 or file_sha(ROOT/pin[0])!=pin[1]: raise Failure("protected input changed")
    cases=corpus["cases"]
    if corpus["closed_case_order"] != [case["id"] for case in cases]: raise Failure("closed case order mismatch")
    for case in cases:
        exact_keys(case,{"id","class","mutation","expected","validity","purpose"},f"case {case.get('id','?')}")
        if case["validity"]!="valid" or case["expected"] not in {"Preserved","Violated","Unknown"}: raise Failure("case validity/classification mismatch")
    pins={"contract":file_sha(args.contract),"binding":file_sha(args.manifest),"fixture":corpus["protected_inputs"]["fixture"][1],"l0_freeze":corpus["protected_inputs"]["l0_freeze"][1]}
    orders={"normal":cases,"repeat":cases,"reverse":list(reversed(cases))}; report={"schema":"p1342-oracle-run-report-v2","runs":{},"agreement":True}
    classifications=defaultdict(list)
    for batch,ordered in orders.items():
        results=[]
        for case in ordered:
            dto,evidence,local_manifest,trusted=build_case(case["id"],corpus,manifest,pins)
            try: actual=judge(dto,evidence,trusted,contract,local_manifest,pins); witness="all composite predicates passed" if actual=="Preserved" else "deliberate witness opacity after every non-payload predicate"
            except Failure as exc: actual="Violated"; witness=str(exc)
            classifications[case["id"]].append(actual)
            if actual!=case["expected"]: report["agreement"]=False
            results.append({"id":case["id"],"expected":case["expected"],"actual":actual,"witness":witness})
        report["runs"][batch]=results
    if any(len(set(values))!=1 for values in classifications.values()): report["agreement"]=False
    negatives=[case for case in cases if case["class"]=="negative"]
    rejected=sum(classifications[case["id"]][0]=="Violated" for case in negatives)
    report["summary"]={"cases":len(cases),"valid_negatives":len(negatives),"negative_violated":rejected,"mutation_score":rejected/len(negatives),"positive_preserved":sum(case["class"]=="positive" and classifications[case["id"]][0]=="Preserved" for case in cases),"opaque_unknown":sum(case["class"]=="opaque" and classifications[case["id"]][0]=="Unknown" for case in cases),"applicable_hooks_per_origin":18,"static_binding_rows":19}
    print(json.dumps(report,indent=2,sort_keys=True)); return 0 if report["agreement"] and report["summary"]["mutation_score"]==1.0 else 1


if __name__=="__main__":
    try: raise SystemExit(main())
    except Failure as exc: print(json.dumps({"schema":"p1342-oracle-run-report-v2","fatal":str(exc)},indent=2),file=sys.stderr); raise SystemExit(2)
