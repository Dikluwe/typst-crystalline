#!/usr/bin/env python3
"""Focal R2 refinement of the protected P1343 source verifier.

R2 retains R1 lexical marker discovery and inverse whole-file normalization,
then derives owner, hook payload, writer object/mutators, projection, H16 and
runtime joins from source/runtime bytes.  It is still necessary rather than
sufficient for a productive verdict: rustc/rustfmt and the independent Rust
A/B/final gates remain external.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
R1_PATH = ROOT / "00_nucleo/diagnosticos/p1343-source-verifier-r1.py"
R1_SHA256 = "07c5827a578ad298ba5f777d56395d9bc66eab156a9500a6730a5cdc335bf146"
MODES = ("normal", "repeat", "reverse")
MUTATOR_RE = re.compile(rb"\.\s*(push|extend|append|insert|splice|clear|truncate|drain)\s*\(")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_r1() -> Any:
    if sha256(R1_PATH.read_bytes()) != R1_SHA256:
        raise RuntimeError("protected R1 source verifier hash mismatch")
    spec = importlib.util.spec_from_file_location("p1343_source_verifier_r1_for_r2", R1_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load protected R1 source verifier")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


R1 = load_r1()
VerificationFailure = R1.VerificationFailure
fail = R1.fail
canonical_json = R1.canonical_json
code_mask = R1.code_mask


PRIMARY_CAPSULES = {
    "H00D-attempt-kind-discovery": ("P1343-H00D-DISCOVERY", "H00D"),
    "H00S-attempt-kind-selected": ("P1343-H00S-SELECTED", "H00S"),
    "H01-attempt-open": ("P1343-H01-ATTEMPT-OPEN", "H01"),
    "H02-context-dispatch": ("P1343-H02-CONTEXT-DISPATCH", "H02"),
    "H03-attempt-result": ("P1343-H03-ATTEMPT-RESULT", "H03"),
    "H04-eval-carrier-transport": ("P1343-EVAL-CARRIER-HELPERS", "H04"),
    "H05-dict-production": ("P1343-H05-DICT-PRODUCTION", "H05"),
    "H06-counter-update-span": ("P1343-H06-BOUND-UPDATE", "H06"),
    "H07-occurrence-created": ("P1343-H07-OCCURRENCE-CREATED", "H07"),
    "H08-func-carrier-slot": ("P1343-FUNC-CTOR-CLOSURE", "H08"),
    "H09-with-object": ("P1343-H09-FUNC-WITH", "H09"),
    "H10-action-carrier-clone": ("P1343-H10-ACTION-CARRIER-CLONE", "H10"),
    "H11-occurrence-walked": ("P1343-H11-OCCURRENCE-WALKED", "H11"),
    "H12A-replay-enter": ("P1343-H12A-REPLAY-ENTER", "H12A"),
    "H12B-replay-exit": ("P1343-H12B-REPLAY-EXIT", "H12B"),
    "H13-func-dispatch": ("P1343-H13-FUNC-DISPATCH", "H13"),
    "H14-with-edge": ("P1343-H14-WITH-EDGE", "H14"),
    "H15-syntax-body": ("P1343-H15-SYNTAX-BODY", "H15"),
    "H16-raw-freeze-before-projection": ("P1343-H16-RAW-FREEZE", "H16"),
}

STATEMENT_CAPSULES = {
    "P1343-H01-ATTEMPT-OPEN", "P1343-H02-CONTEXT-DISPATCH",
    "P1343-H03-ATTEMPT-RESULT", "P1343-H00D-DISCOVERY",
    "P1343-H00S-SELECTED", "P1343-H16-RAW-FREEZE",
    "P1343-H05-DICT-PRODUCTION", "P1343-H11-OCCURRENCE-WALKED",
    "P1343-H12A-REPLAY-ENTER", "P1343-H12B-REPLAY-EXIT",
    "P1343-H13-FUNC-DISPATCH", "P1343-H14-WITH-EDGE",
}

EXPRESSION_WRAPPER_IDS = {
    "P1343-H06-BOUND-UPDATE", "P1343-FUNC-CTOR-CLOSURE",
    "P1343-FUNC-CTOR-NATIVE", "P1343-FUNC-CTOR-NATIVE-NS",
    "P1343-FUNC-CTOR-NATIVE-ENGINE", "P1343-FUNC-CTOR-NATIVE-ENGINE-NS",
    "P1343-FUNC-CTOR-ELEMENT", "P1343-FUNC-CTOR-PLUGIN",
    "P1343-H15-SYNTAX-BODY",
}


def cfg_bytes(required: str) -> bytes:
    if required == "p1339_observation":
        return b"#[cfg(p1339_observation)]"
    if required == "all(test,p1339_observation)":
        return b"#[cfg(all(test, p1339_observation))]"
    fail("CFG", f"unsupported cfg {required!r}")


def validate_cfg_body_r2(body: bytes, capsule: dict[str, Any], replacement: bytes) -> str | None:
    """Validate exact cfg branches while allowing multiple separately guarded units."""
    capsule_id = capsule["capsule_id"]
    R1.validate_balanced_body(body, capsule_id)
    if b"cfg_attr" in body or b"cfg!(" in body:
        fail("CFG", f"{capsule_id}: cfg_attr/cfg! forbidden")
    observed = cfg_bytes(capsule["required_cfg"])
    normal = b"#[cfg(not(p1339_observation))]"
    attrs = re.findall(rb"#\s*\[\s*cfg\s*\([^\]]*\)\s*\]", body)
    allowed = {re.sub(rb"\s+", b"", observed)}
    if capsule["kind"] == "replace":
        allowed.add(re.sub(rb"\s+", b"", normal))
    if not attrs or any(re.sub(rb"\s+", b"", attr) not in allowed for attr in attrs):
        fail("CFG", f"{capsule_id}: foreign or absent cfg predicate")
    if capsule["kind"] == "insert":
        if replacement or body.count(observed) < 1:
            fail("CFG", f"{capsule_id}: insert cfg/replacement mismatch")
        first = R1._strip_ws_comments_prefix(body)
        if not first.startswith(observed):
            fail("CFG", f"{capsule_id}: first unit is not observation guarded")
        # Prove dominance unit by unit.  Counting the exact attribute in the
        # lexical code mask prevents a string/comment decoy from becoming a
        # separator, and the inherited single-unit parser rejects any tail
        # that is not independently guarded.
        masked = code_mask(body)
        positions = [match.start() for match in re.finditer(re.escape(observed), masked)]
        if len(positions) != len(attrs) or body.count(observed) != len(attrs):
            fail("CFG", f"{capsule_id}: every insert guard must use the exact canonical bytes")
        prefix = body[:positions[0]]
        if R1._strip_ws_comments_prefix(prefix):
            fail("CFG", f"{capsule_id}: effective syntax precedes the first guard")
        for index, position in enumerate(positions):
            payload_start = position + len(observed)
            payload_end = positions[index + 1] if index + 1 < len(positions) else len(body)
            R1.require_single_guarded_unit(body[payload_start:payload_end], capsule_id)
        return None
    if body.count(normal) != 1 or body.count(observed) != 1:
        fail("CFG", f"{capsule_id}: replace branch cardinality mismatch")
    normal_at = body.index(normal)
    replacement_at = body.find(replacement, normal_at + len(normal))
    observed_at = body.index(observed)
    if replacement_at < 0 or not (normal_at < replacement_at < observed_at):
        fail("CFG", f"{capsule_id}: exact normal replacement is absent or reordered")
    before_replacement = body[normal_at + len(normal):replacement_at]
    expected_before = b"\n{\n" if capsule_id in EXPRESSION_WRAPPER_IDS else b"\n"
    if before_replacement != expected_before:
        fail("CFG", f"{capsule_id}: bytes before the exact normal replacement are not canonical")
    between = body[replacement_at + len(replacement):observed_at]
    between_shape = R1._strip_ws_comments_prefix(between).strip()
    expected_between = b"}" if capsule_id in EXPRESSION_WRAPPER_IDS else b""
    if between_shape != expected_between:
        fail("CFG", f"{capsule_id}: effective tokens alter the normal branch")
    return sha256(replacement)


def body_map(candidate_root: Path, manifest: dict[str, Any]) -> tuple[dict[str, bytes], dict[str, tuple[bytes, Any]]]:
    bodies: dict[str, bytes] = {}
    sources: dict[str, tuple[bytes, Any]] = {}
    ids_by_path = {item["path"]: set(item["capsule_ids"]) for item in manifest["files"]}
    for path, ids in ids_by_path.items():
        raw = R1.open_source(candidate_root.resolve(strict=True), path)
        regions = R1.pair_markers(R1.lexical_markers(raw, path), ids, path)
        for region in regions:
            bodies[region.capsule_id] = raw[region.body_start:region.body_end]
            sources[region.capsule_id] = (raw, region)
    return bodies, sources


def brace_spans(masked: bytes, pattern: bytes) -> list[tuple[int, int]]:
    spans = []
    for found in re.finditer(pattern, masked):
        opening = masked.find(b"{", found.end())
        if opening < 0:
            continue
        depth, cursor = 1, opening + 1
        while cursor < len(masked) and depth:
            if masked[cursor] == ord("{"): depth += 1
            elif masked[cursor] == ord("}"): depth -= 1
            cursor += 1
        if depth == 0:
            spans.append((found.start(), cursor))
    return spans


def owner_is_real(capsule: dict[str, Any], raw: bytes, region: Any) -> bool:
    symbol = capsule["symbol"]
    masked = code_mask(raw)
    position = region.start
    if symbol.startswith("module::"):
        return not any(start < position < end for start, end in brace_spans(masked, rb"\bfn\s+[A-Za-z_][A-Za-z0-9_]*\b"))
    if symbol == "Func":
        return b"pub struct Func" in code_mask(raw[region.body_start:region.body_end])
    if symbol == "p1342_run_fixture_for_test":
        return len(re.findall(rb"\bfn\s+p1342_run_fixture_for_test\s*\(", code_mask(raw[region.body_start:region.body_end]))) == 1
    name = symbol.split("::")[-1]
    if name in {"Dict", "With"}:
        owner = symbol.split("::")[0]
        # The protected logical owner `eval_expr` is materialized by the
        # private `eval_expr_inner` body; no other suffix is accepted.
        owner_pattern = re.escape(owner.encode()) + (rb"(?:_inner)?" if owner == "eval_expr" else b"")
        functions = brace_spans(masked, rb"\bfn\s+" + owner_pattern + rb"\b")
        token = (b"Expr::Dict" if name == "Dict" else b"FuncRepr::With")
        return any(start < position < end and token in masked[start:position] for start, end in functions)
    if "::" in symbol and not symbol.startswith("impl "):
        method = symbol.split("::")[-1]
        return any(start < position < end for start, end in brace_spans(masked, rb"\bfn\s+" + re.escape(method.encode()) + rb"\b")) or bool(
            re.search(rb"\bfn\s+" + re.escape(method.encode()) + rb"\s*\(", code_mask(raw[region.body_start:region.body_end]))
        )
    if symbol.startswith("impl "):
        typ = symbol[5:]
        return any(start < position < end for start, end in brace_spans(masked, rb"\bimpl\s+" + re.escape(typ.encode()) + rb"\b")) or bool(
            re.search(rb"\bimpl\s+" + re.escape(typ.encode()) + rb"\b", code_mask(raw[region.body_start:region.body_end]))
        )
    # Struct field or free function owner.
    struct_spans = brace_spans(masked, rb"\bstruct\s+" + re.escape(symbol.encode()) + rb"\b")
    function_spans = brace_spans(masked, rb"\bfn\s+" + re.escape(symbol.encode()) + rb"\b")
    return any(start < position < end for start, end in struct_spans + function_spans) or bool(
        re.search(rb"\bfn\s+" + re.escape(symbol.encode()) + rb"\s*\(", code_mask(raw[region.body_start:region.body_end]))
    )


def exact_hook_call(body: bytes, writer: str, hook: str) -> bool:
    # Comments are blanked but string payload bytes remain available in the original.
    masked = code_mask(body)
    candidates = list(re.finditer(rb"\b" + re.escape(writer.encode()) + rb"\s*\(", masked))
    for call in candidates:
        window = body[call.start():call.start() + 240]
        if re.search(rb"[\(,]\s*\"" + re.escape(hook.encode()) + rb"\"\s*[,\)]", window):
            return True
    return False


def validate_runtime_r2(runtime: Any, fixture_sha: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    R1.exact_keys(runtime, {"schema", "fixture_sha256", "modes", "challenges", "run_ids", "row_coverage", "raw_snapshot_refs", "result"}, "R2 runtime evidence")
    if runtime["schema"] != "p1343-runtime-evidence-v2" or runtime["fixture_sha256"] != fixture_sha or runtime["modes"] != list(MODES) or runtime["result"] != "PASS":
        fail("RUNTIME_COVERAGE", "R2 runtime identity/modes/result mismatch")
    R1.exact_keys(runtime["challenges"], set(MODES), "runtime challenges")
    R1.exact_keys(runtime["run_ids"], set(MODES), "runtime run IDs")
    challenges = [runtime["challenges"][mode] for mode in MODES]
    run_ids = [runtime["run_ids"][mode] for mode in MODES]
    if any(not re.fullmatch(r"[0-9a-f]{64}", value or "") for value in challenges) or len(set(challenges)) != 3:
        fail("RUNTIME_COVERAGE", "three pairwise-distinct canonical challenges required")
    if any(not isinstance(value, str) or not value for value in run_ids) or len(set(run_ids)) != 3:
        fail("RUNTIME_COVERAGE", "three pairwise-distinct fresh run IDs required")
    if len(runtime["row_coverage"]) != 19:
        fail("RUNTIME_COVERAGE", "exactly 19 runtime rows required")
    all_refs = []
    for index, (item, row) in enumerate(zip(runtime["row_coverage"], rows)):
        R1.exact_keys(item, {"row_index", "hook", "normal", "repeat", "reverse"}, f"runtime row {index}")
        if item["row_index"] != index or item["hook"] != row["hook"]:
            fail("RUNTIME_COVERAGE", f"runtime row {index} binding mismatch")
        for mode in MODES:
            expected = f"receipt:{mode}:{run_ids[MODES.index(mode)]}:{challenges[MODES.index(mode)]}:{index}:{row['hook']}"
            if item[mode] != expected:
                fail("RUNTIME_COVERAGE", f"runtime receipt {mode}/{index} is not challenge/run/row bound")
            all_refs.append(item[mode])
    if len(set(all_refs)) != len(all_refs):
        fail("RUNTIME_COVERAGE", "runtime receipt references are not unique")
    R1.exact_keys(runtime["raw_snapshot_refs"], set(MODES), "runtime raw snapshots")
    snapshots = [runtime["raw_snapshot_refs"][mode] for mode in MODES]
    for mode, value, challenge, run_id in zip(MODES, snapshots, challenges, run_ids):
        if value != f"raw:{mode}:{run_id}:{challenge}":
            fail("RUNTIME_COVERAGE", "raw snapshot is not bound to mode/run/challenge")
    if len(set(snapshots)) != 3:
        fail("RUNTIME_COVERAGE", "raw snapshots are not disjoint")
    return runtime


def derive_source_evidence(**kwargs: Any) -> dict[str, Any]:
    runtime = kwargs["runtime"]
    binding = kwargs["binding"]
    manifest = kwargs["manifest"]
    validate_runtime_r2(runtime, kwargs["fixture_sha"], binding["row_capsule_bindings"])
    r1_runtime = {
        "schema": "p1343-runtime-evidence-v1",
        "fixture_sha256": runtime["fixture_sha256"],
        "modes": runtime["modes"],
        "row_coverage": runtime["row_coverage"],
        "raw_snapshot_refs": runtime["raw_snapshot_refs"],
        "result": runtime["result"],
    }
    old_cfg = R1.validate_cfg_body
    R1.validate_cfg_body = validate_cfg_body_r2
    try:
        inherited_kwargs = dict(kwargs)
        inherited_kwargs["runtime"] = r1_runtime
        evidence = R1.derive_source_evidence(**inherited_kwargs)
    finally:
        R1.validate_cfg_body = old_cfg
    root = kwargs["candidate_root"].resolve(strict=True)
    bodies, sources = body_map(root, manifest)
    manifest_by_id = {item["capsule_id"]: item for item in manifest["capsules"]}
    for capsule_id, (raw, region) in sources.items():
        if not owner_is_real(manifest_by_id[capsule_id], raw, region):
            fail("OWNER_BINDING", f"{capsule_id}: capsule is not in its resolved Rust owner")
    carrier = code_mask(bodies["P1343-FUNC-CARRIER-TYPES"])
    writer_match = re.search(rb"\bfn\s+(p1343_append_raw)\s*\([^)]*\)\s*\{([^{}]*)\}", carrier, flags=re.S)
    if writer_match is None:
        fail("WRITER_PROJECTION", "raw writer is not defined in P1343-FUNC-CARRIER-TYPES")
    writer = writer_match.group(1).decode()
    writer_body = writer_match.group(2)
    if len(MUTATOR_RE.findall(writer_body)) != 1 or not re.search(rb"\bself\.p1343_raw_events\s*\.\s*push\s*\(\s*event\s*\)", writer_body):
        fail("WRITER_PROJECTION", "writer does not append event to exact self.p1343_raw_events owner")
    all_masked = {capsule_id: code_mask(body) for capsule_id, body in bodies.items()}
    mutators = []
    assignments = []
    aliases = []
    for capsule_id, masked in all_masked.items():
        for match in MUTATOR_RE.finditer(masked):
            mutators.append((capsule_id, match.group(1).decode(), match.start()))
        if re.search(rb"\bp1343_raw_events\s*(?:\[[^\]]+\])?\s*=", masked):
            assignments.append(capsule_id)
        if re.search(rb"\blet\s+(?:mut\s+)?[A-Za-z_][A-Za-z0-9_]*\s*=\s*&\s*mut\s+(?:self\.)?p1343_raw_events", masked):
            aliases.append(capsule_id)
    if mutators != [("P1343-FUNC-CARRIER-TYPES", "push", writer_match.start(2) + writer_body.index(b".push"))] or assignments or aliases:
        fail("WRITER_PROJECTION", f"alternate raw mutator/assignment/alias found: {mutators}, {assignments}, {aliases}")
    for row in binding["row_capsule_bindings"]:
        primary, hook = PRIMARY_CAPSULES[row["hook"]]
        body = bodies[primary]
        if primary in STATEMENT_CAPSULES and re.search(rb"\bfn\s+[A-Za-z_]", code_mask(body)):
            fail("OWNER_BINDING", f"{primary}: dead nested function cannot establish hook reachability")
        if not exact_hook_call(body, writer, hook):
            fail("OWNER_BINDING", f"{primary}: exact hook payload {hook} is absent at the bound owner")
    projection_functions = []
    posthoc = []
    for capsule_id, masked in all_masked.items():
        if re.search(rb"\b(?:posthoc|forged_after_execution)\b", masked, flags=re.I):
            posthoc.append(capsule_id)
        for match in re.finditer(rb"\bfn\s+([A-Za-z_][A-Za-z0-9_]*(?:project|dto)[A-Za-z0-9_]*)\s*\([^)]*\)[^{;]*\{([^{}]*)\}", masked, flags=re.I | re.S):
            projection_functions.append((capsule_id, match.group(1).decode(), match.group(2)))
    if posthoc:
        fail("WRITER_PROJECTION", f"post-hoc event constructor/function found in {posthoc}")
    if len(projection_functions) != 1:
        fail("WRITER_PROJECTION", "exactly one raw-to-DTO projection function required")
    projection_body = projection_functions[0][2]
    if MUTATOR_RE.search(projection_body) or re.search(rb"\bp1343_append_raw\s*\(", projection_body) or re.search(rb"\bp1343_raw_events\s*=", projection_body):
        fail("WRITER_PROJECTION", "projection is not read-only")
    h16 = code_mask(bodies["P1343-H16-RAW-FREEZE"])
    freeze = re.search(rb"\blet\s+p1343_raw_snapshot\s*=\s*self\.p1343_raw_events\.clone\s*\(\s*\)\s*;", h16)
    if freeze is None or not exact_hook_call(bodies["P1343-H16-RAW-FREEZE"], writer, "H16") or freeze.start() > h16.find(b"p1343_append_raw"):
        fail("WRITER_PROJECTION", "H16 does not clone the exact raw ledger before its projection boundary")
    facade_body = bodies["P1343-TEST-FACADE"]
    facade = code_mask(facade_body)
    required_facade_tokens = [
        b"fn p1342_run_fixture_for_test", b"normal_world", b"repeat_world", b"reverse_world",
        b"normal_source", b"repeat_source", b"reverse_source",
        b"normal_challenge", b"repeat_challenge", b"reverse_challenge",
    ]
    if any(token not in facade for token in required_facade_tokens):
        fail("RUNTIME_COVERAGE", "facade does not construct and execute three explicit fresh mode/world/source/challenge paths")
    call_sites = list(re.finditer(rb"\bp1343_run_mode\s*\(", facade))
    if len(call_sites) != 3:
        fail("RUNTIME_COVERAGE", "facade must contain exactly three executable mode calls")
    for mode, call_site in zip(MODES, call_sites):
        window = facade_body[call_site.start():call_site.start() + 220]
        expected = (
            rb'^p1343_run_mode\s*\(\s*"' + mode.encode() + rb'"\s*,\s*'
            + mode.encode() + rb'_world\s*,\s*' + mode.encode() + rb'_source\s*,\s*'
            + mode.encode() + rb'_challenge\s*\)\s*;'
        )
        if re.match(expected, window) is None:
            fail("RUNTIME_COVERAGE", f"facade {mode} call is absent, reordered or decoyed")
    evidence["runtime_coverage"] = {
        "fixture_sha256": runtime["fixture_sha256"], "modes": runtime["modes"],
        "row_coverage": runtime["row_coverage"], "raw_snapshot_refs": runtime["raw_snapshot_refs"], "result": "PASS",
    }
    evidence["writer_projection"]["append_writer_symbol"] = "P1343RawCarrier::p1343_append_raw"
    evidence["writer_projection"]["append_writer_symbol_sha256"] = sha256(b"P1343RawCarrier::p1343_append_raw")
    evidence["writer_projection"]["alternate_writers"] = []
    evidence["writer_projection"]["posthoc_constructors"] = []
    evidence["writer_projection"]["projection_symbols"] = [projection_functions[0][1]]
    projection = {key: value for key, value in evidence.items() if key != "verifier"}
    evidence["verifier"]["source_verifier_sha256"] = kwargs["source_verifier_sha"]
    evidence["verifier"]["derived_projection_sha256"] = sha256(canonical_json(projection))
    return evidence


def main() -> int:
    parser = R1.parser()
    args = parser.parse_args()
    contract = R1.read_json(args.contract, args.contract_sha256, "contract")
    binding = R1.read_json(args.binding, args.binding_sha256, "binding")
    R1.read_json(args.authority_manifest, args.authority_manifest_sha256, "authority manifest")
    manifest = R1.read_json(args.capsule_baseline, args.capsule_baseline_sha256, "capsule baseline")
    R1.read_json(args.p1342_contract, args.p1342_contract_sha256, "P1342 contract")
    R1.read_json(args.p1342_binding, args.p1342_binding_sha256, "P1342 binding")
    R1.read_json(args.l0_freeze, args.l0_freeze_sha256, "L0 freeze")
    fixture = R1.read_pinned_bytes(args.fixture, args.fixture_sha256, "fixture")
    if len(fixture) != 178 or not fixture.endswith(b"\n"):
        fail("PROTECTED_INPUT", "fixture shape mismatch")
    runtime = R1.read_json(args.runtime_evidence, args.runtime_evidence_sha256, "runtime evidence")
    invocation = {key: str(value) for key, value in sorted(vars(args).items()) if key != "output"}
    evidence = derive_source_evidence(
        contract=contract, contract_sha=args.contract_sha256, binding=binding, binding_sha=args.binding_sha256,
        authority_sha=args.authority_manifest_sha256, manifest=manifest, manifest_sha=args.capsule_baseline_sha256,
        p1342_contract_sha=args.p1342_contract_sha256, p1342_binding_sha=args.p1342_binding_sha256,
        l0_sha=args.l0_freeze_sha256, fixture_sha=args.fixture_sha256, candidate_root=args.candidate_root,
        runtime=runtime, source_verifier_sha=sha256(Path(__file__).read_bytes()),
        invocation_sha=sha256(canonical_json(invocation)),
    )
    args.output.write_bytes(canonical_json(evidence))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except VerificationFailure as exc:
        print(json.dumps({"schema": "p1343-source-verifier-error-v2", "classification": "Violated", "reason_code": exc.code, "witness": exc.detail}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
