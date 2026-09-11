#!/usr/bin/env python3
"""Candidate-independent P1344 source verifier.

This verifier composes the protected P1343 capsule baseline with the P1344
owner-local overlay, discovers only lexical full-line markers, performs exact
inverse normalization, and recognizes the three split helper capsules as
associated methods in their physical inherent owners.  It emits source-only
evidence.  Runtime reachability, freshness and cardinality are deliberately
outside this program and require the independent compiled P1344 Rust receipt.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import math
import os
import re
import stat
import sys
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
R1_PATH = DIAG / "p1343-source-verifier-r1.py"
R1_SHA256 = "07c5827a578ad298ba5f777d56395d9bc66eab156a9500a6730a5cdc335bf146"
EXPECTED_CORPUS_SHA256 = "7ce12848ce375ab21f5ca9e112e5b653d750fdd9e4a7daf93c9d8d1e7d0aa0d6"
PINS = {
    "step": (ROOT / "00_nucleo/materialization/typst-passo-1344.md", "1fff15c6f17800ca293379e17d8d2ed63b2fe9043a0aebb0baf16a9b9b0ea6f3"),
    "authority_manifest": (DIAG / "p1344-authority-manifest.json", "9a3cafa60d0dcffb395bb1bda5c823ed5b1881ee8239fe06d8c6cf7614d342d1"),
    "l0_freeze": (DIAG / "p1344-l0-freeze-r1.json", "6cc3a61f2d5177b3ede1d7805f8db8f7edfcd0a2f9ea55aec50405db7832c75d"),
    "capsule_baseline": (DIAG / "p1344-capsule-baseline-r1.json", "f64744d5bdc31734f791af24acf4204949a77c019542d5baa61c48b07e7dcaea"),
    "topology_receipt": (DIAG / "p1344-topology-receipt-r1.json", "b6055f728f15f0ea21b830e3af8be85cd07af22104d947ea6453c7ed23ad4194"),
    "contract": (DIAG / "p1344-contract-spec-r1.json", "157877514d350947f6483ec371c356dd2181c2e0f98e8fa9eb5e465527983279"),
    "binding": (DIAG / "p1344-contract-binding-r1.json", "1f4d8812d0a4e2a9451d2f8c08931fb71e1baa54a3484a411d2cf76727115bab"),
    "contract_receipt": (DIAG / "p1344-contract-receipt-r1.json", "17af4ae7978cd1801256522e18c578fb7520b1ae98d0e0b3b7cde6d47e56b249"),
    "fixture": (DIAG / "p1342-contract-fixture-r1.typ", "98159f5ac529520590a197521dfb23383cec0ba6373b431b8b33f426cfc3a714"),
    "p1343_capsule_baseline": (DIAG / "p1343-capsule-baseline-r1.json", "db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977"),
    "p1343_contract_final": (DIAG / "p1343-contract-spec-r2.json", "d4e444f6966a2be34018854c2b3857452447af1741e799e4cc2cfd6f31c48967"),
    "p1343_binding_final": (DIAG / "p1343-contract-binding-r2.json", "216100a3970bdc79b3d77cd34f4528d46cd04e06dbdc9d63d552f723a557af58"),
}
LAYERS = ("01_core", "02_shell", "03_infra", "04_wiring")
EXPRESSION_WRAPPERS = {
    "P1343-H06-BOUND-UPDATE", "P1343-FUNC-CTOR-CLOSURE",
    "P1343-FUNC-CTOR-NATIVE", "P1343-FUNC-CTOR-NATIVE-NS",
    "P1343-FUNC-CTOR-NATIVE-ENGINE", "P1343-FUNC-CTOR-NATIVE-ENGINE-NS",
    "P1343-FUNC-CTOR-ELEMENT", "P1343-FUNC-CTOR-PLUGIN",
    "P1343-H15-SYNTAX-BODY",
}


class VerificationFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        super().__init__(f"{code}: {detail}")
        self.code = code
        self.detail = detail


def fail(code: str, detail: str) -> None:
    raise VerificationFailure(code, detail)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    out: dict[str, Any] = {}
    for key, value in pairs:
        if key in out:
            fail("DUPLICATE_KEY", f"duplicate decoded JSON key {key!r}")
        out[key] = value
    return out


def reject_constant(value: str) -> None:
    fail("SCHEMA", f"non-finite JSON number {value}")


def strict_json(raw: bytes, label: str) -> Any:
    try:
        return json.loads(raw, object_pairs_hook=strict_object, parse_constant=reject_constant)
    except UnicodeDecodeError as exc:
        fail("SCHEMA", f"{label}: non-UTF-8 JSON: {exc}")
    except json.JSONDecodeError as exc:
        fail("SCHEMA", f"{label}: malformed JSON: {exc}")


def exact_keys(value: Any, expected: Iterable[str], where: str) -> None:
    keys = set(expected)
    if not isinstance(value, dict) or set(value) != keys:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        fail("SCHEMA", f"{where}: keys {got!r}, expected {sorted(keys)!r}")


def load_module(path: Path, name: str, expected: str) -> Any:
    raw = path.read_bytes()
    if sha256(raw) != expected:
        fail("PROTECTED_INPUT", f"{path.name}: hash mismatch")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        fail("PROTECTED_INPUT", f"cannot import {path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


R1 = load_module(R1_PATH, "p1343_source_verifier_r1_for_p1344", R1_SHA256)


def load_protected() -> dict[str, Any]:
    loaded: dict[str, Any] = {}
    for label, (path, expected) in PINS.items():
        raw = path.read_bytes()
        if sha256(raw) != expected:
            fail("PROTECTED_INPUT", f"{label}: {path} drift")
        if path.suffix == ".json":
            loaded[label] = strict_json(raw, label)
        else:
            loaded[label] = raw
    if len(loaded["fixture"]) != 178 or not loaded["fixture"].endswith(b"\n"):
        fail("PROTECTED_INPUT", "fixture must be the exact 178-byte LF-terminated input")
    return loaded


def compose_manifest(base: dict[str, Any], overlay: dict[str, Any]) -> dict[str, Any]:
    if base.get("schema") != "p1343-capsule-baseline-r1" or len(base.get("capsules", [])) != 36:
        fail("SCHEMA", "protected P1343 base is not the closed 36-capsule baseline")
    if overlay.get("schema") != "p1344-capsule-baseline-overlay-r1":
        fail("SCHEMA", "P1344 overlay identity mismatch")
    composition = overlay.get("composition", {})
    if (composition.get("inherited_capsule_count"), composition.get("added_capsule_count"), composition.get("result_capsule_count"), composition.get("result_file_count")) != (36, 2, 38, 12):
        fail("SCHEMA", "P1344 overlay cardinalities are not 36+2=38 over 12 files")
    inherited_ids = [item["capsule_id"] for item in base["capsules"]]
    if overlay.get("inherited_capsule_ids") != inherited_ids:
        fail("SCHEMA", "overlay inherited capsule order differs from protected base")
    capsules = copy.deepcopy(base["capsules"])
    by_id = {item["capsule_id"]: item for item in capsules}
    overrides = overlay.get("inherited_capsule_overrides")
    if not isinstance(overrides, list) or len(overrides) != 1 or overrides[0].get("capsule_id") != "P1343-FUNC-CARRIER-HELPERS":
        fail("SCHEMA", "only the Func helper override is allowed")
    override = overrides[0]
    if override.get("overridden_fields") != ["covers", "candidate_body_rule"]:
        fail("SCHEMA", "Func helper override fields are not closed")
    by_id[override["capsule_id"]]["covers"] = override["covers"]
    by_id[override["capsule_id"]]["candidate_body_rule"] = override["candidate_body_rule"]
    additions = copy.deepcopy(overlay.get("added_capsules"))
    if not isinstance(additions, list) or [item.get("capsule_id") for item in additions] != ["P1344-CONTENT-CARRIER-HELPERS", "P1344-COUNTER-UPDATE-CARRIER-HELPERS"]:
        fail("SCHEMA", "owner-local additions are not the exact two ordered capsules")
    capsules.extend(additions)
    files = copy.deepcopy(overlay.get("files"))
    if not isinstance(files, list) or len(files) != 12:
        fail("SCHEMA", "overlay must contain twelve files")
    file_ids = [capsule_id for item in files for capsule_id in item.get("capsule_ids", [])]
    ids = [item["capsule_id"] for item in capsules]
    if file_ids != ids or len(set(ids)) != 38:
        fail("SCHEMA", "file/capsule ordered join is not exact")
    paths = [item["path"] for item in files]
    if len(set(paths)) != 12:
        fail("SCHEMA", "consumer paths are not twelve unique owners")
    for item in capsules:
        if item["capsule_id"] not in file_ids:
            fail("SCHEMA", "unjoined capsule")
        owner_path = next(file["path"] for file in files if item["capsule_id"] in file["capsule_ids"])
        if item["path"] != owner_path:
            fail("SCHEMA", f"{item['capsule_id']}: owner path mismatch")
    return {"files": files, "capsules": capsules}


def rust_tokens(data: bytes) -> list[str]:
    """Closed lexical token stream; comments/literal payloads never become code."""
    masked = R1.code_mask(data)
    text = masked.decode("utf-8", errors="strict")
    token_re = re.compile(
        r"::|->|=>|\.\.|&&|\|\||==|!=|<=|>=|\+=|-=|\*=|/=|%=|&=|\|=|\^=|<<|>>|"
        r"[A-Za-z_][A-Za-z0-9_]*|[0-9]+|['\"][^'\"]*['\"]|[{}()\[\];,.:#?!<>+=*/&|^-]"
    )
    tokens = token_re.findall(text)
    residual = token_re.sub("", text)
    if residual.strip():
        fail("CFG_GRAMMAR", f"unsupported lexical bytes {residual.strip()[:32]!r}")
    return tokens


def validate_cfg_body(body: bytes, capsule: dict[str, Any], replacement: bytes) -> None:
    """Exact cfg selection with multiple same-guard associated/support items."""
    capsule_id = capsule["capsule_id"]
    R1.validate_balanced_body(body, capsule_id)
    tokens = rust_tokens(body)
    if "cfg_attr" in tokens or any(tokens[index:index + 2] == ["cfg", "!"] for index in range(len(tokens) - 1)):
        fail("CFG_GRAMMAR", f"{capsule_id}: cfg indirection is forbidden")
    observed = R1.cfg_attribute(capsule["required_cfg"])
    normal = R1.cfg_attribute(capsule["required_cfg"], negated=True)
    masked = R1.code_mask(body)
    attrs = list(re.finditer(rb"#\s*\[\s*cfg\s*\([^\]]*\)\s*\]", masked))
    canonical_observed = re.sub(rb"\s+", b"", observed)
    canonical_normal = re.sub(rb"\s+", b"", normal)
    canonical_attrs = [re.sub(rb"\s+", b"", match.group()) for match in attrs]
    if capsule["kind"] == "insert":
        if replacement or not attrs or any(item != canonical_observed for item in canonical_attrs):
            fail("CFG_GRAMMAR", f"{capsule_id}: insert guard/replacement mismatch")
        if R1._strip_ws_comments_prefix(body[:attrs[0].start()]):
            fail("CFG_GRAMMAR", f"{capsule_id}: syntax before first exact guard")
        # Each guard owns exactly one complete top-level syntax unit.
        for index, match in enumerate(attrs):
            end = attrs[index + 1].start() if index + 1 < len(attrs) else len(body)
            R1.require_single_guarded_unit(body[match.end():end], capsule_id)
        return
    if canonical_attrs != [canonical_normal, canonical_observed]:
        fail("CFG_GRAMMAR", f"{capsule_id}: replace branches are not exact normal/observed siblings")
    normal_at = body.find(normal)
    observed_at = body.find(observed)
    replacement_at = body.find(replacement, normal_at + len(normal))
    if min(normal_at, observed_at, replacement_at) < 0 or not (normal_at < replacement_at < observed_at):
        fail("CFG_GRAMMAR", f"{capsule_id}: exact normal replacement absent or reordered")
    prefix = R1._strip_ws_comments_prefix(body[:normal_at])
    expected_prefix = b"{" if capsule_id in EXPRESSION_WRAPPERS else b""
    if prefix.strip() != expected_prefix:
        fail("CFG_GRAMMAR", f"{capsule_id}: unauthorized outer replacement wrapper")
    before_replacement = body[normal_at + len(normal):replacement_at]
    expected_before = b"\n{\n" if capsule_id in EXPRESSION_WRAPPERS else b"\n"
    if before_replacement != expected_before:
        fail("CFG_GRAMMAR", f"{capsule_id}: normal payload prefix is not canonical")
    between = R1._strip_ws_comments_prefix(body[replacement_at + len(replacement):observed_at]).strip()
    expected_between = b"}" if capsule_id in EXPRESSION_WRAPPERS else b""
    if between != expected_between:
        fail("CFG_GRAMMAR", f"{capsule_id}: effective token alters normal branch")
    observed_payload = body[observed_at + len(observed):]
    if capsule_id in EXPRESSION_WRAPPERS:
        stripped = R1._strip_ws_comments_prefix(observed_payload)
        if not stripped.startswith(b"{"):
            fail("CFG_GRAMMAR", f"{capsule_id}: observed expression block absent")
        tail = stripped.rstrip()
        if not tail.endswith(b"}\n}") and not tail.endswith(b"}}"):
            fail("CFG_GRAMMAR", f"{capsule_id}: outer expression wrapper not closed")
    else:
        R1.require_single_guarded_unit(observed_payload, capsule_id)


def brace_spans(raw: bytes, owner: str) -> list[tuple[int, int]]:
    masked = R1.code_mask(raw)
    pattern = re.compile(rb"\bimpl\s+" + re.escape(owner.encode()) + rb"\s*\{")
    spans: list[tuple[int, int]] = []
    for match in pattern.finditer(masked):
        opening = masked.find(b"{", match.start(), match.end())
        depth = 1
        cursor = opening + 1
        while cursor < len(masked) and depth:
            if masked[cursor] == ord("{"):
                depth += 1
            elif masked[cursor] == ord("}"):
                depth -= 1
            cursor += 1
        if depth == 0:
            spans.append((opening + 1, cursor - 1))
    return spans


def require_physical_owner(raw: bytes, region: Any, owner: str, capsule_id: str) -> None:
    spans = brace_spans(raw, owner)
    containing = [(start, end) for start, end in spans if start <= region.start and region.end <= end]
    if len(containing) != 1:
        fail("OWNER_LOCAL", f"{capsule_id}: not inside exactly one existing impl {owner}")
    # A nested impl would add another `impl` token inside the capsule and is
    # separately rejected by the associated-method grammar.


EXPECTED_METHODS = {
    "P1343-FUNC-CARRIER-HELPERS": [
        "p1343_carrier", "p1343_attach_carrier", "p1343_observe_h09",
        "p1343_observe_h12a", "p1343_observe_h12b_result",
        "p1343_observe_h12b_exit", "p1343_observe_h13",
        "p1343_observe_h14", "p1343_observe_h15",
    ],
    "P1344-CONTENT-CARRIER-HELPERS": ["p1343_observe_h11"],
    "P1344-COUNTER-UPDATE-CARRIER-HELPERS": ["p1343_observe_h10"],
}
EXPECTED_OWNER = {
    "P1343-FUNC-CARRIER-HELPERS": "Func",
    "P1344-CONTENT-CARRIER-HELPERS": "Content",
    "P1344-COUNTER-UPDATE-CARRIER-HELPERS": "CounterUpdate",
}


def split_top_level_methods(tokens: list[str], capsule_id: str) -> list[tuple[str, list[str]]]:
    cursor = 0
    methods: list[tuple[str, list[str]]] = []
    guard = ["#", "[", "cfg", "(", "p1339_observation", ")", "]"]
    forbidden = {"fn", "impl", "trait", "macro_rules", "const", "static", "use", "mod", "type", "extern"}
    while cursor < len(tokens):
        if tokens[cursor:cursor + len(guard)] != guard:
            fail("CFG_GRAMMAR", f"{capsule_id}: associated method lacks exact cfg guard")
        cursor += len(guard)
        if tokens[cursor:cursor + 4] != ["pub", "(", "crate", ")"]:
            fail("CFG_GRAMMAR", f"{capsule_id}: helper visibility must be pub(crate)")
        cursor += 4
        if cursor >= len(tokens) or tokens[cursor] != "fn" or cursor + 1 >= len(tokens):
            fail("CFG_GRAMMAR", f"{capsule_id}: expected associated fn")
        name = tokens[cursor + 1]
        start = cursor
        cursor += 2
        if "{" not in tokens[cursor:]:
            fail("CFG_GRAMMAR", f"{capsule_id}/{name}: missing body")
        opening = tokens.index("{", cursor)
        depth = 1
        cursor = opening + 1
        while cursor < len(tokens) and depth:
            if tokens[cursor] == "{": depth += 1
            elif tokens[cursor] == "}": depth -= 1
            cursor += 1
        if depth:
            fail("CFG_GRAMMAR", f"{capsule_id}/{name}: unclosed method")
        method = tokens[start:cursor]
        if any(token in forbidden for token in method[method.index("{") + 1:-1]):
            fail("OWNER_LOCAL", f"{capsule_id}/{name}: nested/container/indirection item")
        if "macro_rules" in method or "!" in method:
            fail("CFG_GRAMMAR", f"{capsule_id}/{name}: macro is outside the closed grammar")
        methods.append((name, method))
    return methods


def validate_owner_helpers(capsule_id: str, body: bytes, raw: bytes, region: Any) -> dict[str, Any]:
    owner = EXPECTED_OWNER[capsule_id]
    require_physical_owner(raw, region, owner, capsule_id)
    tokens = rust_tokens(body)
    methods = split_top_level_methods(tokens, capsule_id)
    names = [name for name, _ in methods]
    if names != EXPECTED_METHODS[capsule_id]:
        fail("OWNER_LOCAL", f"{capsule_id}: method order/set {names!r} is not exact")
    joined = [token for _, method in methods for token in method]
    if capsule_id == "P1343-FUNC-CARRIER-HELPERS":
        if "Content" in joined or "CounterUpdate" in joined:
            fail("OWNER_LOCAL", "Func helper capsule names a foreign owner")
    elif capsule_id == "P1344-CONTENT-CARRIER-HELPERS":
        required = ["Content", "::", "CounterUpdate"]
        if not any(joined[index:index + 3] == required for index in range(len(joined) - 2)):
            fail("OWNER_LOCAL", "Content helper must match the real Content::CounterUpdate variant")
        if any(token in joined for token in ("append", "push", "insert", "extend")):
            fail("CARRIER_SKELETON", "Content helper is not read-only")
    else:
        required = ["CounterUpdate", "::", "Func"]
        if not any(joined[index:index + 3] == required for index in range(len(joined) - 2)):
            fail("OWNER_LOCAL", "CounterUpdate helper must match CounterUpdate::Func")
        for variant in ("Set", "Step"):
            if not any(joined[index:index + 3] == ["CounterUpdate", "::", variant] for index in range(len(joined) - 2)):
                fail("OWNER_LOCAL", f"CounterUpdate helper lacks explicit {variant} absence arm")
        if any(token in joined for token in ("append", "push", "insert", "extend", "new")):
            fail("CARRIER_SKELETON", "CounterUpdate helper constructs or writes a carrier")
    return {"capsule_id": capsule_id, "owner": owner, "methods": names, "result": "PASS"}


def validate_direct_hook(body: bytes, syntax: dict[str, Any]) -> None:
    capsule_id = syntax["capsule_id"]
    tokens = rust_tokens(body)
    expected_cfg = ["#", "[", "cfg", "("]
    if syntax["exact_cfg"] == "all(test,p1339_observation)":
        expected_cfg += ["all", "(", "test", ",", "p1339_observation", ")"]
    else:
        expected_cfg += ["p1339_observation"]
    expected_cfg += [")", "]"]
    if tokens[:len(expected_cfg)] != expected_cfg:
        fail("CFG_GRAMMAR", f"{capsule_id}: direct hook cfg is not exact")
    tail = tokens[len(expected_cfg):]
    callee = syntax["canonical_callee"].split(".")
    if len(tail) < 6 or tail[0:3] != [callee[0], ".", callee[1]] or tail[3] != "(" or tail[-2:] != [")", ";"]:
        fail("CFG_GRAMMAR", f"{capsule_id}: not the exact direct receiver.method(args); production")
    forbidden = {"::", "=>", "if", "match", "loop", "while", "for", "fn", "impl", "trait", "macro_rules", "let", "return", "break", "continue", "async", "await", "unsafe", "!", "|"}
    if any(token in forbidden for token in tail[4:-2]):
        fail("CFG_GRAMMAR", f"{capsule_id}: forbidden effective token in direct-hook arguments")
    depth = 0
    for token in tail[3:-1]:
        if token in ("(", "[", "{"): depth += 1
        elif token in (")", "]", "}"): depth -= 1
        if depth < 0: fail("CFG_GRAMMAR", f"{capsule_id}: unbalanced direct-hook arguments")
    if depth != 0:
        fail("CFG_GRAMMAR", f"{capsule_id}: unbalanced direct-hook arguments")


def validate_carrier_closed(body: bytes) -> dict[str, Any]:
    tokens = rust_tokens(body)
    forbidden = {"static", "trait", "macro_rules", "unsafe", "Deref", "DerefMut", "AsMut", "IndexMut", "retain", "drain", "splice", "extend", "clear", "truncate", "insert", "get_mut", "swap", "replace", "take"}
    found = sorted(set(tokens) & forbidden)
    if found or "!" in tokens:
        fail("CARRIER_SKELETON", f"carrier contains forbidden grammar tokens {found!r}")
    required_types = ["P1343Hook", "P1343RawEvent", "P1343AppendReceipt", "P1343LedgerState", "P1343Carrier", "P1343RawSnapshot", "P1343Projection"]
    for name in required_types:
        if tokens.count(name) < 1:
            fail("CARRIER_SKELETON", f"carrier lacks {name}")
    hooks = ["H00D", "H00S", "H01", "H02", "H03", "H04", "H05", "H06", "H07", "H08", "H09", "H10", "H11", "H12A", "H12B", "H13", "H14", "H15", "H16"]
    for hook in hooks:
        if tokens.count(hook) != 1:
            fail("CARRIER_SKELETON", f"hook variant {hook} is absent or duplicated")
    methods = [tokens[index + 1] for index, token in enumerate(tokens[:-1]) if token == "fn"]
    for name in ("new", "append", "snapshot", "project"):
        if methods.count(name) != 1:
            fail("CARRIER_SKELETON", f"storage method {name} is absent or duplicated")
    # Only append may contain push. Both event and receipt vectors are appended
    # once, so the closed skeleton requires exactly two direct push tokens.
    if tokens.count("push") != 2:
        fail("CARRIER_SKELETON", "append must own exactly the two raw Vec::push calls")
    for constructed in ("P1343RawEvent", "P1343AppendReceipt"):
        literals = sum(tokens[index:index + 2] == [constructed, "{"] for index in range(len(tokens) - 1))
        if literals != 1:
            fail("CARRIER_SKELETON", f"{constructed} may have only its type declaration in the carrier capsule")
    return {"types": required_types, "hooks": hooks, "storage_methods": ["new", "append", "snapshot", "project"], "result": "PASS"}


def validate_facade_closed(body: bytes) -> None:
    tokens = rust_tokens(body)
    if tokens.count("p1342_run_fixture_for_test") != 1 or tokens.count("run_context_stabilization_once") != 1 or tokens.count("project") != 1:
        fail("CFG_GRAMMAR", "facade must contain one named function, one product delegation and one projection")
    for parameter in ("world", "source", "challenge", "mode"):
        if tokens.count(parameter) < 2:
            fail("CFG_GRAMMAR", f"facade does not receive and forward {parameter}")
    forbidden = {"if", "match", "loop", "while", "for", "clone", "macro_rules", "unsafe", "static", "thread", "spawn", "format"}
    found = sorted(set(tokens) & forbidden)
    if found or "!" in tokens:
        fail("CFG_GRAMMAR", f"facade contains control/creation/indirection tokens {found!r}")


def enumerate_global_markers(root: Path, allowlisted: set[str]) -> None:
    for layer in LAYERS:
        base = root / layer
        if not base.exists():
            continue
        for current, dirnames, filenames in os.walk(base, followlinks=False):
            dirnames.sort(); filenames.sort()
            current_path = Path(current)
            for dirname in dirnames:
                if (current_path / dirname).is_symlink():
                    fail("PATH", "symlink directory under global marker scan")
            for filename in filenames:
                if not filename.endswith(".rs"):
                    continue
                path = current_path / filename
                relative = path.relative_to(root).as_posix()
                if path.is_symlink():
                    fail("PATH", f"symlink source {relative}")
                if relative not in allowlisted and R1.lexical_markers(path.read_bytes(), relative):
                    fail("PATH", f"P1343/P1344 capsule marker outside twelve consumers: {relative}")


def open_source(root: Path, relative: str) -> bytes:
    target = root.joinpath(*relative.split("/"))
    try:
        info = target.lstat()
    except OSError as exc:
        fail("PATH", f"cannot stat {relative}: {exc}")
    if stat.S_ISLNK(info.st_mode) or not stat.S_ISREG(info.st_mode):
        fail("PATH", f"{relative}: symlink/non-regular source")
    try:
        target.resolve(strict=True).relative_to(root)
    except (OSError, ValueError) as exc:
        fail("PATH", f"{relative}: escapes candidate root: {exc}")
    return target.read_bytes()


def verify_candidate(candidate_root: Path) -> dict[str, Any]:
    protected = load_protected()
    manifest = compose_manifest(protected["p1343_capsule_baseline"], protected["capsule_baseline"])
    try:
        root = candidate_root.resolve(strict=True)
    except OSError as exc:
        fail("PATH", f"cannot resolve candidate root: {exc}")
    if not root.is_dir():
        fail("PATH", "candidate root must be a directory")
    allowlisted = {item["path"] for item in manifest["files"]}
    enumerate_global_markers(root, allowlisted)
    syntax_base = protected["p1343_binding_final"]["capsule_syntax_table"]
    syntax_by_id = {item["capsule_id"]: item for item in syntax_base}
    grammar_overrides = {item["capsule_id"]: item for item in protected["binding"]["capsule_syntax_overrides"]}
    files_out = []
    capsules_out = []
    owner_out = []
    carrier_out: dict[str, Any] | None = None
    by_capsule = {item["capsule_id"]: item for item in manifest["capsules"]}
    for file_item in manifest["files"]:
        path = file_item["path"]
        raw = open_source(root, path)
        R1.validate_utf8_source(raw, path)
        expected_ids = set(file_item["capsule_ids"])
        regions = R1.pair_markers(R1.lexical_markers(raw, path), expected_ids, path)
        region_by_id = {item.capsule_id: item for item in regions}
        normalized = raw
        for region in sorted(regions, key=lambda item: item.start, reverse=True):
            capsule = by_capsule[region.capsule_id]
            before = capsule["anchor_before"]["text"].encode()
            after = capsule["anchor_after"]["text"].encode()
            if raw[max(0, region.start - len(before)):region.start] != before:
                fail("ANCHOR", f"{region.capsule_id}: before anchor not adjacent")
            if raw[region.end:region.end + len(after)] != after:
                fail("ANCHOR", f"{region.capsule_id}: after anchor not adjacent")
            replacement = R1.decode_replacement(capsule["baseline_replacement"], region.capsule_id)
            body = raw[region.body_start:region.body_end]
            validate_cfg_body(body, capsule, replacement)
            if region.capsule_id in EXPECTED_METHODS:
                owner_out.append(validate_owner_helpers(region.capsule_id, body, raw, region))
            else:
                syntax = syntax_by_id.get(region.capsule_id)
                if syntax is None:
                    fail("SCHEMA", f"{region.capsule_id}: missing inherited syntax production")
                if syntax["grammar_category"] == "direct_hook_call":
                    validate_direct_hook(body, syntax)
                if region.capsule_id == "P1343-FUNC-CARRIER-TYPES":
                    carrier_out = validate_carrier_closed(body)
                if region.capsule_id == "P1343-TEST-FACADE":
                    validate_facade_closed(body)
                tokens = rust_tokens(body)
                if "cfg_attr" in tokens or any(tokens[index:index + 2] == ["cfg", "!"] for index in range(len(tokens) - 1)):
                    fail("CFG_GRAMMAR", f"{region.capsule_id}: cfg macro/attribute indirection")
            normalized = normalized[:region.start] + replacement + normalized[region.end:]
            capsules_out.append({
                "capsule_id": region.capsule_id,
                "path": path,
                "grammar_category": grammar_overrides.get(region.capsule_id, syntax_by_id.get(region.capsule_id, {})).get("grammar_category"),
                "body_sha256": sha256(body),
                "replacement_sha256": sha256(replacement),
                "result": "PASS",
            })
        if sha256(normalized) != file_item["baseline_file_sha256"]:
            fail("NORMALIZATION", f"{path}: inverse normalization hash mismatch")
        files_out.append({"path": path, "live_sha256": sha256(raw), "normalized_sha256": sha256(normalized), "capsule_ids": file_item["capsule_ids"], "result": "PASS"})
    if [item["capsule_id"] for item in owner_out] != list(EXPECTED_METHODS):
        fail("OWNER_LOCAL", "owner-local helper capsules absent or reordered")
    if carrier_out is None:
        fail("CARRIER_SKELETON", "carrier capsule not validated")
    order = [item["capsule_id"] for item in manifest["capsules"]]
    indexed = {item["capsule_id"]: item for item in capsules_out}
    capsules_out = [indexed[item] for item in order]
    transcript = [[item["path"], item["normalized_sha256"]] for item in files_out]
    return {
        "schema": "p1344-candidate-source-evidence-r1",
        "step": 1344,
        "classification": "Preserved",
        "source_only": True,
        "runtime_claim": "NOT_EVALUATED_REQUIRES_EXTERNAL_COMPILED_P1344_RECEIPT",
        "protected": {**{label: digest for label, (_, digest) in PINS.items()}, "oracle_corpus": EXPECTED_CORPUS_SHA256},
        "topology": {"files": 12, "capsules": 38, "owners": 12, "hook_rows": 19},
        "files": files_out,
        "capsules": capsules_out,
        "owner_local": owner_out,
        "carrier_skeleton": carrier_out,
        "normalization_transcript_sha256": sha256(canonical_json(transcript)),
        "candidate_root_sha256": sha256(str(root).encode()),
        "verifier_sha256": sha256(Path(__file__).read_bytes()),
    }


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    value.add_argument("--candidate-root", required=True, type=Path)
    value.add_argument("--output", required=True, type=Path)
    value.add_argument("--runtime-evidence", type=Path, help="rejected here: runtime is a separate authority")
    return value


def main() -> int:
    args = parser().parse_args()
    if args.runtime_evidence is not None:
        fail("RUNTIME_AUTHORITY", "source verifier never accepts runtime evidence")
    evidence = verify_candidate(args.candidate_root)
    args.output.write_bytes(canonical_json(evidence))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (VerificationFailure, R1.VerificationFailure) as exc:
        print(json.dumps({"schema": "p1344-source-verifier-error-r1", "classification": "Violated", "reason_code": exc.code, "witness": exc.detail}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
