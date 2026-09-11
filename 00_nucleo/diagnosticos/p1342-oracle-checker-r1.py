#!/usr/bin/env python3
"""Independent P1342 oracle over supplied binding-free ledger DTOs.

This diagnostic checker does not accept a DTO coverage declaration. Static coverage
is derived from the external binding manifest and its pinned files; dynamic counts,
R, edges and order are derived from the supplied event stream.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
EXPECTED_HOOKS = [f"H{i:02d}" for i in range(1, 16)]
TOP_KEYS = {"schema", "fixture", "binding_manifest", "payload_visibility", "attempts"}
ATTEMPT_KEYS = {"attempt_id", "kind", "incorporated", "events"}
EVENT_KEYS = {"seq", "event_id", "prev_event_id", "kind", "hook", "refs", "data"}
EVENT_KINDS = {
    "attempt-open", "context-dispatch", "dict-produced",
    "counter-occurrence-created", "counter-occurrence-walked",
    "callback-replay-enter", "func-dispatch", "with-unwrapped",
    "closure-dispatch", "syntax-body-enter", "syntax-body-exit",
    "callback-replay-exit", "attempt-close",
}
EXPECTED_EVENT_HOOK = {
    "attempt-open": "H01", "context-dispatch": "H02",
    "dict-produced": "H05", "counter-occurrence-created": "H07",
    "counter-occurrence-walked": "H11", "callback-replay-enter": "H12",
    "func-dispatch": "H13", "with-unwrapped": "H14",
    "closure-dispatch": "H13", "syntax-body-enter": "H15",
    "syntax-body-exit": "H15", "callback-replay-exit": "H12",
    "attempt-close": "H03",
}
REF_KEYS = {
    "attempt-open": {"attempt"},
    "context-dispatch": {"attempt", "context_closure", "context_block"},
    "dict-produced": {"attempt", "carrier", "syntax_body"},
    "counter-occurrence-created": {"attempt", "occurrence", "carrier", "callback_with"},
    "counter-occurrence-walked": {"attempt", "occurrence", "carrier", "content", "location", "snapshot_pre", "callback_with"},
    "callback-replay-enter": {"attempt", "occurrence", "carrier", "content", "location", "snapshot_pre", "callback_with"},
    "func-dispatch": {"attempt", "carrier", "func"},
    "with-unwrapped": {"attempt", "carrier", "outer", "inner"},
    "closure-dispatch": {"attempt", "carrier", "func_callback", "syntax_body"},
    "syntax-body-enter": {"attempt", "carrier", "func_callback", "syntax_body"},
    "syntax-body-exit": {"attempt", "carrier", "func_callback", "syntax_body"},
    "callback-replay-exit": {"attempt", "occurrence", "carrier", "location", "snapshot_pre", "snapshot_post", "callback_with"},
    "attempt-close": {"attempt"},
}
DATA_KEYS = {
    "attempt-open": set(), "context-dispatch": set(),
    "dict-produced": {"role", "span", "value"},
    "counter-occurrence-created": {"source_span", "key", "action"},
    "counter-occurrence-walked": {"key", "action"},
    "callback-replay-enter": set(),
    "func-dispatch": {"role"},
    "with-unwrapped": set(),
    "closure-dispatch": set(),
    "syntax-body-enter": {"syntax_kind", "span"},
    "syntax-body-exit": {"syntax_kind", "span", "result"},
    "callback-replay-exit": {"result"},
    "attempt-close": {"result"},
}
PREFIX = {
    "attempt": "attempt:", "context_closure": "context-closure:",
    "context_block": "context-block:", "occurrence": "counter-occurrence:",
    "carrier": "carrier:", "callback_with": "callback-with:",
    "func": None, "outer": "callback-with:", "inner": "func-callback:",
    "content": "counter-content:", "location": "location:",
    "snapshot_pre": "snapshot:", "snapshot_post": "snapshot:",
    "func_callback": "func-callback:", "syntax_body": "syntax-node:",
}
ID_PREFIXES = tuple(sorted({p for p in PREFIX.values() if p} | {"event:"}, key=len, reverse=True))


class Failure(Exception):
    pass


def exact_keys(value: Any, keys: set[str], where: str) -> None:
    if not isinstance(value, dict) or set(value) != keys:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        raise Failure(f"{where}: closed schema expected {sorted(keys)}, got {got}")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def integer(value: Any, where: str) -> int:
    if type(value) is not int:
        raise Failure(f"{where}: expected int (bool is not int)")
    return value


def span(value: Any, expected: tuple[int, int], where: str) -> None:
    exact_keys(value, {"offset", "end_exclusive"}, where)
    got = (integer(value["offset"], where), integer(value["end_exclusive"], where))
    if got != expected:
        raise Failure(f"{where}: expected {expected[0]}..{expected[1]}, got {got[0]}..{got[1]}")


def typed_value(value: Any, where: str, allow_opaque: bool = False) -> tuple[Any, bool]:
    if not isinstance(value, dict) or "type" not in value:
        raise Failure(f"{where}: typed value object required")
    kind = value["type"]
    if kind == "int":
        exact_keys(value, {"type", "value"}, where)
        return ("int", integer(value["value"], where)), False
    if kind == "selector":
        exact_keys(value, {"type", "shape", "ordered_fields"}, where)
        if value["shape"] != "heading.where" or not isinstance(value["ordered_fields"], list):
            raise Failure(f"{where}: invalid selector")
        pairs = []
        for i, pair in enumerate(value["ordered_fields"]):
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str):
                raise Failure(f"{where}.ordered_fields[{i}]: ordered [name,value] required")
            child, opaque = typed_value(pair[1], f"{where}.ordered_fields[{i}]")
            if opaque:
                raise Failure(f"{where}: selector fields cannot be opaque")
            pairs.append((pair[0], child))
        return ("selector", value["shape"], tuple(pairs)), False
    if kind == "dict":
        visibility = value.get("visibility")
        if visibility == "opaque":
            exact_keys(value, {"type", "visibility", "commitment"}, where)
            if not allow_opaque:
                raise Failure(f"{where}: opacity not authorized")
            if not isinstance(value["commitment"], str) or not re.fullmatch(r"[0-9a-f]{64}", value["commitment"]):
                raise Failure(f"{where}: opaque commitment must be 64 lowercase hex")
            return ("dict", "opaque"), True
        exact_keys(value, {"type", "visibility", "entries"}, where)
        if visibility != "inspectable" or not isinstance(value["entries"], list):
            raise Failure(f"{where}: ordered inspectable Dict entries required")
        pairs = []
        for i, pair in enumerate(value["entries"]):
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str):
                raise Failure(f"{where}.entries[{i}]: ordered [name,value] required")
            child, opaque = typed_value(pair[1], f"{where}.entries[{i}]", allow_opaque=False)
            if opaque:
                raise Failure(f"{where}: nested opacity not authorized")
            pairs.append((pair[0], child))
        return ("dict", tuple(pairs)), False
    raise Failure(f"{where}: variant {kind!r} is outside the closed typed-value schema")


def symbol_region(text: str, symbol: str) -> str:
    name = symbol.split("::")[-1]
    match = re.search(rf"\bfn\s+{re.escape(name)}\b", text)
    if not match:
        raise Failure(f"binding symbol {symbol!r} not found")
    opening = text.find("{", match.end())
    if opening < 0:
        raise Failure(f"binding symbol {symbol!r} has no body")
    depth = 0
    for index in range(opening, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[match.start():index + 1]
    raise Failure(f"binding symbol {symbol!r} has unbalanced body")


def validate_static_manifest(manifest: Any, contract_hash: str, fixture_hash: str) -> None:
    expected_top = {"schema", "step", "revision", "regime", "contract", "fixture", "baseline", "anchor_rule", "cardinality_rule", "hooks", "forbidden_bindings"}
    exact_keys(manifest, expected_top, "binding manifest")
    if manifest["schema"] != "p1342-static-binding-manifest-v1" or manifest["step"] != 1342:
        raise Failure("binding manifest identity mismatch")
    if manifest["contract"][1] != contract_hash or manifest["fixture"][1] != fixture_hash:
        raise Failure("binding manifest contract/fixture pin mismatch")
    hooks = manifest["hooks"]
    if not isinstance(hooks, list) or len(hooks) != 15:
        raise Failure("binding manifest must contain exactly 15 hooks")
    short = [h.get("hook", "")[:3] for h in hooks if isinstance(h, dict)]
    if sorted(short) != EXPECTED_HOOKS or len(set(h.get("hook") for h in hooks if isinstance(h, dict))) != 15:
        raise Failure("binding manifest has a missing or duplicated hook")
    hook_keys = {"hook", "stable_role", "l0", "l0_sha256", "consumer", "consumer_sha256", "path", "symbol", "anchor_scope", "anchor_needle", "anchor_matches", "species", "fixture_reach", "cardinality_per_attempt"}
    for h in hooks:
        exact_keys(h, hook_keys, f"hook {h.get('hook', '?')}")
        if type(h["anchor_matches"]) is not int or h["anchor_matches"] != 1:
            raise Failure(f"{h['hook']}: anchor_matches must be integer 1")
        l0_path, consumer_path, bound_path = ROOT / h["l0"], ROOT / h["consumer"], ROOT / h["path"]
        if consumer_path != bound_path:
            raise Failure(f"{h['hook']}: consumer/path mismatch")
        if sha256(l0_path) != h["l0_sha256"] or sha256(consumer_path) != h["consumer_sha256"]:
            raise Failure(f"{h['hook']}: frozen L0/consumer hash mismatch")
        text = bound_path.read_text(encoding="utf-8")
        scope = text if h["anchor_scope"] == "file" else symbol_region(text, h["symbol"])
        if scope.count(h["anchor_needle"]) != 1:
            raise Failure(f"{h['hook']}: anchor does not resolve exactly once in declared scope")


def validate_ref(name: str, value: Any, where: str) -> None:
    if value is None and name in {"carrier", "syntax_body"}:
        return
    if not isinstance(value, str) or not value:
        raise Failure(f"{where}.{name}: nonempty local identity required")
    prefix = PREFIX[name]
    if name == "func":
        if not (value.startswith("callback-with:") or value.startswith("func-callback:")):
            raise Failure(f"{where}.{name}: invalid Func identity domain")
    elif prefix and not value.startswith(prefix):
        raise Failure(f"{where}.{name}: expected identity domain {prefix}")


def events_of(events: list[dict[str, Any]], kind: str, role: str | None = None) -> list[dict[str, Any]]:
    values = [event for event in events if event["kind"] == kind]
    if role is not None:
        values = [event for event in values if event["data"].get("role") == role]
    return values


def only(values: list[Any], where: str) -> Any:
    if len(values) != 1:
        raise Failure(f"{where}: expected exactly 1, got {len(values)}")
    return values[0]


def validate_dto(dto: Any, manifest_hash: str, fixture: dict[str, Any], manifest: dict[str, Any]) -> str:
    exact_keys(dto, TOP_KEYS, "DTO")
    if dto["schema"] != "p1342-ledger-dto-v1":
        raise Failure("DTO schema mismatch")
    exact_keys(dto["fixture"], {"path", "sha256", "bytes"}, "DTO.fixture")
    exact_keys(dto["binding_manifest"], {"path", "sha256"}, "DTO.binding_manifest")
    if dto["fixture"] != {"path": fixture["path"], "sha256": fixture["sha256"], "bytes": fixture["bytes"]}:
        raise Failure("DTO fixture pin mismatch")
    if dto["binding_manifest"] != {"path": "00_nucleo/diagnosticos/p1342-contract-binding-manifest-r1.json", "sha256": manifest_hash}:
        raise Failure("DTO external binding manifest pin mismatch")
    if dto["payload_visibility"] not in {"inspectable", "opaque"}:
        raise Failure("DTO payload_visibility mismatch")
    if not isinstance(dto["attempts"], list) or len(dto["attempts"]) != 1:
        raise Failure("DTO requires exactly one attempt cell")
    attempt = dto["attempts"][0]
    exact_keys(attempt, ATTEMPT_KEYS, "attempt")
    if attempt["kind"] not in {"discovery", "selected"} or type(attempt["incorporated"]) is not bool:
        raise Failure("attempt kind/incorporated type mismatch")
    validate_ref("attempt", attempt["attempt_id"], "attempt")
    events = attempt["events"]
    if not isinstance(events, list) or not events:
        raise Failure("attempt events must be nonempty")
    opaque_seen = False
    hook_by_short = {hook["hook"][:3]: hook["hook"] for hook in manifest["hooks"]}
    for index, event in enumerate(events):
        exact_keys(event, EVENT_KEYS, f"event[{index}]")
        if integer(event["seq"], f"event[{index}].seq") != index:
            raise Failure("append-only stream has non-tail ordinal or reordered event")
        if not isinstance(event["event_id"], str) or not event["event_id"].startswith("event:"):
            raise Failure("event identity domain mismatch")
        expected_prev = None if index == 0 else events[index - 1]["event_id"]
        if event["prev_event_id"] != expected_prev:
            raise Failure("append-only predecessor chain mismatch")
        kind = event["kind"]
        if kind not in EVENT_KINDS:
            raise Failure(f"event[{index}]: unknown event kind")
        if event["hook"] != hook_by_short.get(EXPECTED_EVENT_HOOK[kind]):
            raise Failure(f"event[{index}]: wrong productive hook for {kind}")
        exact_keys(event["refs"], REF_KEYS[kind], f"event[{index}].refs")
        exact_keys(event["data"], DATA_KEYS[kind], f"event[{index}].data")
        for name, value in event["refs"].items():
            validate_ref(name, value, f"event[{index}].refs")
        if event["refs"].get("attempt") != attempt["attempt_id"]:
            raise Failure(f"event[{index}]: cross-cell attempt reference")
    if len({e["event_id"] for e in events}) != len(events):
        raise Failure("event identities must be unique")

    open_event = only(events_of(events, "attempt-open"), "attempt-open")
    context = only(events_of(events, "context-dispatch"), "context-dispatch")
    prebound = only(events_of(events, "dict-produced", "dict-prebound"), "dict-prebound")
    created = only(events_of(events, "counter-occurrence-created"), "counter-occurrence-created")
    close = only(events_of(events, "attempt-close"), "attempt-close")
    span(prebound["data"]["span"], (155, 162), "dict-prebound span")
    prebound_value, prebound_opaque = typed_value(prebound["data"]["value"], "dict-prebound value")
    if prebound_opaque or prebound_value != ("dict", (("a", ("int", 1)),)):
        raise Failure("ordered typed prebound Dict mismatch")
    span(created["data"]["source_span"], (144, 164), "counter occurrence source span")
    key, key_opaque = typed_value(created["data"]["key"], "counter key")
    expected_key = ("selector", "heading.where", (("level", ("int", 1)),))
    if key_opaque or key != expected_key or created["data"]["action"] != "Func":
        raise Failure("counter key/action mismatch")
    if created["refs"]["callback_with"] == context["refs"]["context_closure"]:
        raise Failure("context closure and callback-with must not alias")

    walked = events_of(events, "counter-occurrence-walked")
    expected_walk = 1 if attempt["incorporated"] else 0
    if len(walked) != expected_walk:
        raise Failure("incorporation/walk cardinality mismatch")
    replay_enter = events_of(events, "callback-replay-enter")
    r_value = len(replay_enter)
    if r_value not in {0, 1}:
        raise Failure("derived R must be 0 or 1")
    expected_counts = {
        "callback-replay-exit": r_value, "with-unwrapped": r_value,
        "closure-dispatch": r_value, "syntax-body-enter": r_value,
        "syntax-body-exit": r_value,
    }
    for kind, expected in expected_counts.items():
        if len(events_of(events, kind)) != expected:
            raise Failure(f"{kind}: relational cardinality expected {expected}")
    dispatches = events_of(events, "func-dispatch")
    if len(dispatches) != 2 * r_value:
        raise Failure("func-dispatch must equal 2*R")
    roles = [event["data"]["role"] for event in dispatches]
    if roles != ["callback-with", "func-callback"] * r_value:
        raise Failure("Func dispatch role order mismatch")
    witness = events_of(events, "dict-produced", "dict-witness")
    if len(witness) != r_value:
        raise Failure("dict-witness must equal R")

    if r_value == 0:
        if attempt["incorporated"] or close["data"]["result"] != "not-incorporated":
            raise Failure("R=0 case must be explicitly non-incorporated")
    else:
        if not attempt["incorporated"] or close["data"]["result"] != "incorporated":
            raise Failure("R=1 case must be incorporated")
        walk = walked[0]
        enter = replay_enter[0]
        outer_dispatch, inner_dispatch = dispatches
        unwrap = only(events_of(events, "with-unwrapped"), "with-unwrapped")
        closure = only(events_of(events, "closure-dispatch"), "closure-dispatch")
        body_enter = only(events_of(events, "syntax-body-enter"), "syntax-body-enter")
        body_exit = only(events_of(events, "syntax-body-exit"), "syntax-body-exit")
        replay_exit = only(events_of(events, "callback-replay-exit"), "callback-replay-exit")
        witness_event = witness[0]
        canonical = {
            "occurrence": created["refs"]["occurrence"], "carrier": created["refs"]["carrier"],
            "callback_with": created["refs"]["callback_with"],
        }
        for event in [walk, enter, replay_exit]:
            for name in ["occurrence", "carrier", "callback_with"]:
                if event["refs"].get(name) != canonical[name]:
                    raise Failure(f"carrier/occurrence topology disconnected at {event['kind']}")
        walked_key, walked_key_opaque = typed_value(walk["data"]["key"], "walked counter key")
        if walked_key_opaque or walked_key != key or walk["data"]["action"] != created["data"]["action"]:
            raise Failure("walked key/action is not the created occurrence payload")
        if walk["refs"]["content"] != enter["refs"]["content"] or walk["refs"]["location"] != enter["refs"]["location"]:
            raise Failure("walk/replay Content or Location edge disconnected")
        if walk["refs"]["snapshot_pre"] != enter["refs"]["snapshot_pre"] or enter["refs"]["snapshot_pre"] != replay_exit["refs"]["snapshot_pre"]:
            raise Failure("snapshot-pre edge disconnected")
        if replay_exit["refs"]["snapshot_post"] == replay_exit["refs"]["snapshot_pre"]:
            raise Failure("snapshot-pre and snapshot-post identities must be distinct")
        if outer_dispatch["refs"]["func"] != canonical["callback_with"] or unwrap["refs"]["outer"] != canonical["callback_with"]:
            raise Failure("With outer edge disconnected or inverted")
        inner = unwrap["refs"]["inner"]
        if inner_dispatch["refs"]["func"] != inner or closure["refs"]["func_callback"] != inner:
            raise Failure("With inner/recursive dispatch edge disconnected")
        body_id = closure["refs"]["syntax_body"]
        for event in [body_enter, body_exit]:
            if event["refs"]["func_callback"] != inner or event["refs"]["syntax_body"] != body_id:
                raise Failure("closure/SyntaxNode edge disconnected")
            if event["data"]["syntax_kind"] != "CodeBlock":
                raise Failure("syntax-body must be a CodeBlock SyntaxNode, never Func")
            span(event["data"]["span"], (64, 121), f"{event['kind']} span")
        if witness_event["refs"]["syntax_body"] != body_id or witness_event["refs"]["carrier"] != canonical["carrier"]:
            raise Failure("witness Dict is not causally inside syntax-body")
        span(witness_event["data"]["span"], (82, 97), "dict-witness span")
        witness_value, witness_opaque = typed_value(
            witness_event["data"]["value"], "dict-witness value",
            allow_opaque=dto["payload_visibility"] == "opaque",
        )
        if witness_opaque:
            opaque_seen = True
        elif witness_value != ("dict", (("outer", ("dict", (("a", ("int", 1)),))),)):
            raise Failure("ordered typed witness Dict mismatch")
        for event in [body_exit, replay_exit]:
            result, result_opaque = typed_value(event["data"]["result"], f"{event['kind']} result")
            if result_opaque or result != ("int", 1):
                raise Failure("callback result must be Int(1)")
        order = [open_event, context, prebound, created, walk, enter, outer_dispatch, unwrap, inner_dispatch, closure, body_enter, witness_event, body_exit, replay_exit, close]
        if [event["seq"] for event in order] != sorted(event["seq"] for event in order):
            raise Failure("causal order inversion")

    if close["seq"] != len(events) - 1:
        raise Failure("attempt-close must follow every attributable event")
    if dto["payload_visibility"] == "opaque" and not opaque_seen:
        raise Failure("opaque mode without a deliberately opaque payload")
    if dto["payload_visibility"] == "inspectable" and opaque_seen:
        raise Failure("inspectable mode contains opaque payload")
    return "Unknown" if opaque_seen else "Preserved"


def pointer_parent(document: Any, pointer: str) -> tuple[Any, str]:
    parts = pointer.lstrip("/").split("/") if pointer else []
    current = document
    for raw in parts[:-1]:
        part = raw.replace("~1", "/").replace("~0", "~")
        current = current[int(part)] if isinstance(current, list) else current[part]
    return current, parts[-1].replace("~1", "/").replace("~0", "~")


def apply_mutations(template: Any, mutations: list[dict[str, Any]]) -> Any:
    value = copy.deepcopy(template)
    for mutation in mutations:
        if mutation["target"] != "dto":
            continue
        parent, key = pointer_parent(value, mutation.get("path", ""))
        op = mutation["op"]
        if op == "replace":
            if isinstance(parent, list):
                parent[int(key)] = copy.deepcopy(mutation["value"])
            else:
                parent[key] = copy.deepcopy(mutation["value"])
        elif op == "add":
            if isinstance(parent, list):
                parent.insert(int(key), copy.deepcopy(mutation["value"]))
            else:
                parent[key] = copy.deepcopy(mutation["value"])
        elif op == "remove":
            parent.pop(int(key)) if isinstance(parent, list) else parent.pop(key)
        elif op in {"swap", "swap-rechain"}:
            other_parent, other_key = pointer_parent(value, mutation["other_path"])
            left = parent[int(key)] if isinstance(parent, list) else parent[key]
            right = other_parent[int(other_key)] if isinstance(other_parent, list) else other_parent[other_key]
            if isinstance(parent, list): parent[int(key)] = right
            else: parent[key] = right
            if isinstance(other_parent, list): other_parent[int(other_key)] = left
            else: other_parent[other_key] = left
            if op == "swap-rechain":
                rechain(parent)
        elif op in {"insert-copy", "insert-copy-rechain"}:
            source_parent, source_key = pointer_parent(value, mutation["source_path"])
            source = source_parent[int(source_key)] if isinstance(source_parent, list) else source_parent[source_key]
            parent.insert(int(key), copy.deepcopy(source))
            if op == "insert-copy-rechain":
                rechain(parent)
        else:
            raise Failure(f"unsupported DTO mutation {op}")
    return value


def rechain(events: list[dict[str, Any]]) -> None:
    seen: set[str] = set()
    previous = None
    for index, event in enumerate(events):
        event["seq"] = index
        event_id = event["event_id"]
        if event_id in seen:
            event_id = f"{event_id}-copy-{index}"
            event["event_id"] = event_id
        seen.add(event_id)
        event["prev_event_id"] = previous
        previous = event_id


def mutate_manifest(manifest: Any, mutations: list[dict[str, Any]]) -> Any:
    value = copy.deepcopy(manifest)
    for mutation in mutations:
        if mutation["target"] != "manifest":
            continue
        index = next((i for i, hook in enumerate(value["hooks"]) if hook["hook"].startswith(mutation["hook"])), None)
        if index is None:
            raise Failure(f"manifest mutation target {mutation['hook']} absent")
        if mutation["op"] == "remove-hook":
            value["hooks"].pop(index)
        elif mutation["op"] == "duplicate-hook":
            value["hooks"].insert(index + 1, copy.deepcopy(value["hooks"][index]))
        else:
            raise Failure(f"unsupported manifest mutation {mutation['op']}")
    return value


def remap_local_ids(value: Any, nonce: str) -> Any:
    mapping: dict[str, str] = {}
    def visit(item: Any) -> Any:
        if isinstance(item, str) and item.startswith(ID_PREFIXES):
            return mapping.setdefault(item, f"{item}@{nonce}")
        if isinstance(item, list):
            return [visit(child) for child in item]
        if isinstance(item, dict):
            return {key: visit(child) for key, child in item.items()}
        return item
    return visit(value)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--contract", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--corpus", required=True, type=Path)
    args = parser.parse_args()
    contract = json.loads(args.contract.read_text(encoding="utf-8"))
    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    corpus = json.loads(args.corpus.read_text(encoding="utf-8"))
    exact_keys(corpus, {"schema", "step", "revision", "role", "executor", "regime", "status", "protected_inputs", "closed_case_order", "run_semantics", "templates", "cases", "scope_exclusions"}, "corpus")
    if corpus["schema"] != "p1342-oracle-corpus-v1" or corpus["step"] != 1342:
        raise Failure("corpus identity mismatch")
    exact_keys(corpus["run_semantics"], {"normal", "repeat", "reverse", "expected_relation"}, "corpus.run_semantics")
    if set(corpus["templates"]) != {"selected-r1", "nonincorporated-r0"}:
        raise Failure("corpus template set is not closed")
    if not isinstance(corpus["cases"], list) or not isinstance(corpus["closed_case_order"], list):
        raise Failure("corpus cases/order must be arrays")
    for case in corpus["cases"]:
        exact_keys(case, {"id", "class", "template", "expected", "purpose", "mutations"}, f"case {case.get('id', '?')}")
        if case["class"] not in {"positive", "negative", "opaque"} or case["expected"] not in {"Preserved", "Violated", "Unknown"}:
            raise Failure(f"case {case['id']}: invalid classification")
        if case["template"] not in corpus["templates"] or not isinstance(case["mutations"], list):
            raise Failure(f"case {case['id']}: invalid template/mutations")
    case_ids = [case["id"] for case in corpus["cases"]]
    if len(case_ids) != len(set(case_ids)) or corpus["closed_case_order"] != case_ids:
        raise Failure("closed_case_order must list every case exactly once in authored order")
    contract_hash, manifest_hash = sha256(args.contract), sha256(args.manifest)
    if corpus["protected_inputs"]["contract"][1] != contract_hash or corpus["protected_inputs"]["binding_manifest"][1] != manifest_hash:
        raise Failure("corpus protected pin mismatch")
    for label, pin in corpus["protected_inputs"].items():
        if not isinstance(pin, list) or len(pin) != 2 or sha256(ROOT / pin[0]) != pin[1]:
            raise Failure(f"protected input {label} changed")
    fixture = contract["fixture"]
    if sha256(ROOT / fixture["path"]) != fixture["sha256"]:
        raise Failure("fixture bytes changed")
    modes = [("normal", corpus["closed_case_order"]), ("repeat", corpus["closed_case_order"]), ("reverse", list(reversed(corpus["closed_case_order"])))]
    case_by_id = {case["id"]: case for case in corpus["cases"]}
    report = {"schema": "p1342-oracle-run-report-v1", "runs": {}, "agreement": True}
    per_case: dict[str, list[str]] = {case_id: [] for case_id in corpus["closed_case_order"]}
    for run_index, (mode, order) in enumerate(modes):
        results = []
        for case_index, case_id in enumerate(order):
            case = case_by_id[case_id]
            dto = apply_mutations(corpus["templates"][case["template"]], case["mutations"])
            dto = remap_local_ids(dto, f"{mode}-{run_index}-{case_index}")
            local_manifest = mutate_manifest(manifest, case["mutations"])
            try:
                validate_static_manifest(local_manifest, contract_hash, fixture["sha256"])
                actual = validate_dto(dto, manifest_hash, fixture, local_manifest)
            except Failure as exc:
                actual = "Violated"
                witness = str(exc)
            else:
                witness = "all predicates passed" if actual == "Preserved" else "deliberately opaque payload after non-payload predicates"
            if actual != case["expected"]:
                report["agreement"] = False
            per_case[case_id].append(actual)
            results.append({"id": case_id, "expected": case["expected"], "actual": actual, "witness": witness})
        report["runs"][mode] = results
    for case_id, classifications in per_case.items():
        if len(set(classifications)) != 1:
            report["agreement"] = False
    report["summary"] = {
        "cases": len(corpus["closed_case_order"]),
        "positive_preserved": sum(case["class"] == "positive" and per_case[case["id"]][0] == "Preserved" for case in corpus["cases"]),
        "negative_violated": sum(case["class"] == "negative" and per_case[case["id"]][0] == "Violated" for case in corpus["cases"]),
        "opaque_unknown": sum(case["class"] == "opaque" and per_case[case["id"]][0] == "Unknown" for case in corpus["cases"]),
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["agreement"] else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Failure as exc:
        print(json.dumps({"schema": "p1342-oracle-run-report-v1", "fatal": str(exc)}, indent=2), file=sys.stderr)
        raise SystemExit(2)
