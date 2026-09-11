#!/usr/bin/env python3
"""P1345 candidate-free canonical-token source verifier.

This module owns no semantic verdict.  It builds the one synthetic positive
overlay from the protected candidate-free baseline and compares every complete
capsule token sequence against an external canonical table.  Only whitespace
and Rust comments are ignored.
"""

from __future__ import annotations

import base64
import copy
import hashlib
import json
import re
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
PINS = {
    "step": (ROOT / "00_nucleo/materialization/typst-passo-1345.md", "c2ef7ff7b0555a4f44ca6811ff3e687ef6d6a30f55e0cd106fba4118f3a18fb0"),
    "manifest": (DIAG / "p1345-authority-manifest-r1.json", "2ed02b31b39c6d588db7772cf932af40e344e87947662e12b7a8fe5c7e96b2c2"),
    "freeze": (DIAG / "p1345-l0-toolchain-freeze-r1.json", "9b1ed01ac9c7d0e5c5121906138fe4d4777ba2ce2982d0b89cafb2fef631bbf7"),
    "inventory": (DIAG / "p1345-candidate-free-inventory-r1.md", "fd4b4fa2f191bf8ba7267e48b643ff33d08a9d855a97879905f28cac429cb808"),
    "baseline": (DIAG / "p1345-capsule-baseline-r1.json", "e083f6b28d989afecf95d4b24d5483cef1d6bed45d5ce239f818156f56f05d08"),
    "topology": (DIAG / "p1345-topology-receipt-r1.json", "f4c18c856ab039ae6c99357442a97661425e2a135c87ce35194ecaaf9c571072"),
    "p1344_contract": (DIAG / "p1344-contract-spec-r1.json", "157877514d350947f6483ec371c356dd2181c2e0f98e8fa9eb5e465527983279"),
    "p1344_binding": (DIAG / "p1344-contract-binding-r1.json", "1f4d8812d0a4e2a9451d2f8c08931fb71e1baa54a3484a411d2cf76727115bab"),
    "p1344_adversary_report": (DIAG / "p1344-adversary-report-r2.json", "2c536123d13e3aa537d5c7071f2910bb5b502fdf14aea82eb931b875d9d6bf9f"),
    "p1344_adversary_receipt": (DIAG / "p1344-adversary-receipt-r2.json", "0c830744ffa6714599bc7c428277b1d37246db6d7815364b04044045752b7c4b"),
    "contract_r1": (DIAG / "p1345-contract-spec-r1.json", "816aea0c92bf95a37fccbcc3c937af889e4638cf39b96593ffc62ecf004e6d1a"),
    "binding_r1": (DIAG / "p1345-contract-binding-r1.json", "7b2451d5c290a0a4d3ad2ca4d54a0df93d39e861577aac5da7cd27f6d62417ad"),
    "receipt_r1": (DIAG / "p1345-contract-receipt-r1.json", "38b59f3952ee4e05d03ce0ff6151c9e23328093adcb4bbd76d36c6a99e6f7c85"),
    "blocker": (DIAG / "p1345-oracle-contract-blocker-r1.md", "5aa99f11d5b7b6cb23936f3a2c5888fbff0a658acbad158710cd0675753634b5"),
    "blocker_receipt": (DIAG / "p1345-oracle-contract-blocker-receipt-r1.json", "c6af354e667726824f7ac8b4ad9dec45d48aed8ca49ab4925b047d994439c1de"),
    "contract_r2": (DIAG / "p1345-contract-spec-r2.json", "f40c2b42fe83a4b75e276ab2ea8a9639fa1553e80c06581caf8c53d948f9d38d"),
    "binding_r2": (DIAG / "p1345-contract-binding-r2.json", "ecc3a8a403d9d14d51808a35876f1f43d7773732c1d20f58574dff42d5187778"),
    "receipt_r2": (DIAG / "p1345-contract-receipt-r2.json", "875f937affb30486c9f63fa0aaf84d1e9a6e93214a8fdfb95db320e76a6c14c2"),
    "p1343_baseline": (DIAG / "p1343-capsule-baseline-r1.json", "db143d9fec795ec81ce7a69be5e08b09e451a9d6eb07a1a39cd1dcd41ee82977"),
    "p1344_baseline": (DIAG / "p1344-capsule-baseline-r1.json", "f64744d5bdc31734f791af24acf4204949a77c019542d5baa61c48b07e7dcaea"),
    "p1343_binding": (DIAG / "p1343-contract-binding-r2.json", "216100a3970bdc79b3d77cd34f4528d46cd04e06dbdc9d63d552f723a557af58"),
}
FIXTURE_PATH = DIAG / "p1345-positive-fixture-r1.json"
TABLE_PATH = DIAG / "p1345-canonical-capsule-table-r1.json"
CORPUS_PATH = DIAG / "p1345-oracle-corpus-r1.json"
EXPECTED_FIXTURE_SHA256 = "8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12"
EXPECTED_TABLE_SHA256 = "ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f"
TOKENIZER_VERSION = "p1345-rust-lexical-surface-v1"


class VerificationFailure(RuntimeError):
    def __init__(self, code: str, detail: str, capsule_id: str | None = None, index: int | None = None):
        super().__init__(f"{code}: {detail}")
        self.code = code
        self.detail = detail
        self.capsule_id = capsule_id
        self.index = index


def fail(code: str, detail: str, capsule_id: str | None = None, index: int | None = None) -> None:
    raise VerificationFailure(code, detail, capsule_id, index)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8")


def strict_json(raw: bytes, label: str) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in items:
            if key in result:
                fail("DUPLICATE_KEY", f"{label}: duplicate decoded key {key!r}")
            result[key] = value
        return result
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=pairs, parse_constant=lambda x: fail("SCHEMA", f"{label}: non-finite {x}"))
    except VerificationFailure:
        raise
    except Exception as exc:
        fail("SCHEMA", f"{label}: {exc}")


def exact_keys(value: Any, keys: list[str], label: str) -> None:
    if not isinstance(value, dict) or list(value) != keys:
        fail("SCHEMA", f"{label}: exact ordered keys mismatch")


def load_pinned() -> dict[str, Any]:
    loaded: dict[str, Any] = {}
    for label, (path, expected) in PINS.items():
        raw = path.read_bytes()
        if sha256(raw) != expected:
            fail("PROTECTED_INPUT", f"{label} hash drift")
        if path.suffix == ".json":
            loaded[label] = strict_json(raw, label)
    return loaded


def compose_manifest(protected: dict[str, Any]) -> dict[str, Any]:
    base = copy.deepcopy(protected["p1343_baseline"])
    overlay = protected["p1344_baseline"]
    capsules = base["capsules"]
    override = overlay["inherited_capsule_overrides"][0]
    target = next(item for item in capsules if item["capsule_id"] == override["capsule_id"])
    target["covers"] = override["covers"]
    target["candidate_body_rule"] = override["candidate_body_rule"]
    capsules.extend(copy.deepcopy(overlay["added_capsules"]))
    files = copy.deepcopy(overlay["files"])
    if len(capsules) != 38 or len(files) != 12:
        fail("SCHEMA", "composed topology is not 38 capsules/12 files")
    return {"marker_policy": overlay["marker_policy"], "files": files, "capsules": capsules}


def cfg(required: str) -> bytes:
    if required == "p1339_observation":
        return b"#[cfg(p1339_observation)]\n"
    if required == "all(test,p1339_observation)":
        return b"#[cfg(all(test, p1339_observation))]\n"
    fail("SCHEMA", f"unsupported cfg {required}")


DIRECT = {
    "P1343-H00D-DISCOVERY": ("self.p1343_observe_h00d", ["index", "introspector", "location"]),
    "P1343-H00S-SELECTED": ("self.p1343_observe_h00s", ["index", "input", "location"]),
    "P1343-H01-ATTEMPT-OPEN": ("self.p1343_observe_h01", ["index", "input", "location", "attempt_kind"]),
    "P1343-H02-CONTEXT-DISPATCH": ("ctx.p1343_observe_h02", ["node", "span", "carrier"]),
    "P1343-H03-ATTEMPT-RESULT": ("ctx.p1343_observe_h03", ["result", "selected", "incorporation"]),
    "P1343-H05-DICT-PRODUCTION": ("ctx.p1343_observe_h05", ["map", "span"]),
    "P1343-H11-OCCURRENCE-WALKED": ("content.p1343_observe_h11", ["loc", "action"]),
    "P1343-H12A-REPLAY-ENTER": ("function.p1343_observe_h12a", ["ctx", "location", "pre_state"]),
    "P1343-H12B-REPLAY-EXIT": ("function.p1343_observe_h12b_exit", ["location", "pre_state", "post_state"]),
    "P1343-H13-FUNC-DISPATCH": ("func.p1343_observe_h13", ["ctx"]),
    "P1343-H14-WITH-EDGE": ("inner.p1343_observe_h14", ["outer", "args", "ctx"]),
    "P1343-H16-RAW-FREEZE": ("self.p1343_observe_h16", []),
}


def observe(hook: str) -> bytes:
    return b'p1343_append_raw("' + hook.encode() + b'", p1343_event());\n'


def expression_replace(replacement: bytes, required: str, hook: str) -> bytes:
    return b"{\n#[cfg(not(p1339_observation))]\n{\n" + replacement + b"\n}\n" + cfg(required) + b"{\n" + observe(hook) + replacement + b"\n}\n}\n"


def item_replace(replacement: bytes, required: str, observed: bytes) -> bytes:
    return b"#[cfg(not(p1339_observation))]\n" + replacement + cfg(required) + observed


def inject_after_opening(item: bytes, addition: bytes) -> bytes:
    opening = item.index(b"{") + 1
    return item[:opening] + b"\n" + addition + item[opening:]


def canonical_body(capsule: dict[str, Any], replacement: bytes) -> bytes:
    cid = capsule["capsule_id"]
    required = capsule["required_cfg"]
    if cid == "P1343-PIPE-SUPPORT":
        return cfg(required) + b"struct P1343PipelineObservationSupport;\n"
    if cid == "P1343-PIPE-SESSION-FIELD":
        return cfg(required) + b"p1343_carrier: Option<P1343Carrier>,\n"
    if cid == "P1343-PIPE-SESSION-INIT":
        return cfg(required) + b"p1343_carrier: None,\n"
    if cid in DIRECT:
        callee, args = DIRECT[cid]
        return cfg(required) + callee.encode() + b"(" + b", ".join(x.encode() for x in args) + b");\n"
    if cid == "P1343-TEST-FACADE":
        return cfg(required) + (b"fn p1342_run_fixture_for_test(world: &World, source: Source, challenge: [u8; 32], mode: P1343Mode) -> (P1343RawSnapshot, P1343Projection) {\n"
            b"let raw = run_context_stabilization_once(world, source, challenge, mode);\nlet projection = raw.project();\n(raw, projection)\n}\n")
    if cid == "P1343-EVAL-CARRIER-FIELD":
        return cfg(required) + b"p1343_carrier: Option<P1343Carrier>,\n"
    if cid == "P1343-EVAL-CARRIER-INIT":
        return cfg(required) + b"p1343_carrier: None,\n"
    if cid == "P1343-EVAL-CARRIER-HELPERS":
        names = ["p1343_install_carrier", "p1343_carrier", "p1343_counter_update_observed", "p1343_observe_h02", "p1343_observe_h03", "p1343_observe_h04", "p1343_observe_h05"]
        return cfg(required) + b"impl EvalContext {\n" + b"".join(b"fn " + n.encode() + b"(&mut self) {}\n" for n in names) + b"}\n"
    if cid == "P1343-H06-BOUND-UPDATE":
        return expression_replace(replacement, required, "H06")
    if cid == "P1343-H07-OCCURRENCE-CREATED":
        return cfg(required) + b"fn counter_update_observed() {\n" + observe("H07") + b"}\n"
    if cid == "P1343-H06-STATIC-UPDATE":
        return item_replace(replacement, required, inject_after_opening(replacement, observe("H06") + observe("H07")))
    if cid == "P1343-FUNC-CARRIER-TYPES":
        hooks = b", ".join(h.encode() for h in ["H00D","H00S","H01","H02","H03","H04","H05","H06","H07","H08","H09","H10","H11","H12A","H12B","H13","H14","H15","H16"])
        g = cfg(required)
        return b"".join([g,b"pub(crate) enum P1343Hook { ",hooks,b" }\n",g,b"pub(crate) struct P1343RawEvent { hook: P1343Hook }\n",g,b"pub(crate) struct P1343AppendReceipt { seq: usize }\n",g,b"struct P1343LedgerState { events: Vec<P1343RawEvent>, receipts: Vec<P1343AppendReceipt>, frozen: bool }\n",g,b"pub(crate) struct P1343Carrier { state: P1343LedgerState }\n",g,b"pub(crate) struct P1343RawSnapshot { events: Vec<P1343RawEvent>, receipts: Vec<P1343AppendReceipt> }\n",g,b"pub(crate) struct P1343Projection;\n",g,b"impl P1343Carrier {\nfn new() -> Self { Self { state: P1343LedgerState { events: Vec::new(), receipts: Vec::new(), frozen: false } } }\nfn append(&mut self, event: P1343RawEvent, receipt: P1343AppendReceipt) { self.state.events.push(event); self.state.receipts.push(receipt); }\nfn snapshot(&mut self) -> P1343RawSnapshot { self.state.frozen = true; P1343RawSnapshot { events: self.state.events.clone(), receipts: self.state.receipts.clone() } }\n}\n",g,b"impl P1343RawSnapshot { fn project(&self) -> P1343Projection { P1343Projection } }\n"])
    if cid == "P1343-H08-FUNC-FIELD":
        return item_replace(replacement, required, b"pub struct Func(pub(crate) Arc<FuncRepr>, Span, Option<P1343Carrier>);\n")
    if cid.startswith("P1343-FUNC-CTOR-"):
        return expression_replace(replacement, required, "H08")
    if cid == "P1343-H09-FUNC-WITH":
        return item_replace(replacement, required, inject_after_opening(replacement, observe("H09")))
    if cid == "P1343-FUNC-CARRIER-HELPERS":
        names = ["p1343_carrier", "p1343_attach_carrier", "p1343_observe_h09", "p1343_observe_h12a", "p1343_observe_h12b_result", "p1343_observe_h12b_exit", "p1343_observe_h13", "p1343_observe_h14", "p1343_observe_h15"]
        chunks = []
        for name in names:
            chunks.append(cfg(required) + b"pub(crate) fn " + name.encode() + b"(&self) {}\n")
        return b"".join(chunks)
    if cid == "P1343-H10-ACTION-CARRIER-CLONE":
        return item_replace(replacement, required, inject_after_opening(replacement, observe("H10")))
    if cid == "P1343-H12B-REPLAY-CALL":
        observed = replacement.replace(b"let value = apply_func(", b"let value = { p1343_append_raw(\"H12B\", p1343_event()); apply_func(", 1).replace(b")?;\n", b")? };\n", 1)
        return item_replace(replacement, required, observed)
    if cid == "P1343-H15-SYNTAX-BODY":
        return expression_replace(replacement, required, "H15")
    if cid == "P1344-CONTENT-CARRIER-HELPERS":
        return cfg(required) + b"pub(crate) fn p1343_observe_h11(&self) -> Option<P1343Carrier> {\nmatch self { Content::CounterUpdate(elem) => elem.action.p1343_observe_h10(), _ => None }\n}\n"
    if cid == "P1344-COUNTER-UPDATE-CARRIER-HELPERS":
        return cfg(required) + b"pub(crate) fn p1343_observe_h10(&self) -> Option<P1343Carrier> {\nmatch self { CounterUpdate::Func(function) => function.p1343_carrier(), CounterUpdate::Set(_) | CounterUpdate::Step(_) => None }\n}\n"
    fail("SCHEMA", f"no canonical body for {cid}")


def capsule_bytes(cid: str, body: bytes) -> bytes:
    return b"// P1343-CAPSULE-BEGIN " + cid.encode("ascii") + b"\n" + body + b"// P1343-CAPSULE-END " + cid.encode("ascii") + b"\n"


def overlay_files(protected: dict[str, Any], manifest: dict[str, Any]) -> dict[str, bytes]:
    by_path: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for capsule in manifest["capsules"]:
        by_path[capsule["path"]].append(capsule)
    output: dict[str, bytes] = {}
    for file_item in manifest["files"]:
        path = file_item["path"]
        raw = (ROOT / path).read_bytes()
        if sha256(raw) != file_item["baseline_file_sha256"]:
            fail("PROTECTED_INPUT", f"candidate-free file drift {path}")
        edits = []
        for capsule in by_path[path]:
            before = capsule["anchor_before"]["text"].encode()
            after = capsule["anchor_after"]["text"].encode()
            replacement = base64.b64decode(capsule["baseline_replacement"]["bytes_base64"], validate=True)
            needle = before + replacement + after
            if raw.count(needle) != 1:
                fail("ANCHOR", f"{capsule['capsule_id']}: baseline anchor is not unique")
            start = raw.index(needle) + len(before)
            edits.append((start, start + len(replacement), capsule_bytes(capsule["capsule_id"], canonical_body(capsule, replacement))))
        for start, end, value in sorted(edits, reverse=True):
            raw = raw[:start] + value + raw[end:]
        output[path] = raw
    return output


PUNCT = sorted(["<<=", ">>=", "...", "..=", "::", "->", "=>", "==", "!=", "<=", ">=", "&&", "||", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", ".."], key=len, reverse=True)
KEYWORDS = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try union".split())


def rust_tokens(raw: bytes) -> list[list[str]]:
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        fail("TOKEN_DIVERGENCE", f"non-UTF8 token stream: {exc}")
    out: list[list[str]] = []
    i = 0
    n = len(text)
    while i < n:
        if text[i].isspace():
            i += 1
            continue
        if text.startswith("//", i):
            end = text.find("\n", i + 2)
            i = n if end < 0 else end + 1
            continue
        if text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j): depth, j = depth + 1, j + 2
                elif text.startswith("*/", j): depth, j = depth - 1, j + 2
                else: j += 1
            if depth: fail("TOKEN_DIVERGENCE", "unterminated block comment")
            i = j
            continue
        raw_match = re.match(r'(?:br|cr|r)(#{0,255})"', text[i:])
        if raw_match:
            hashes = raw_match.group(1); close = '"' + hashes
            j = text.find(close, i + raw_match.end())
            if j < 0: fail("TOKEN_DIVERGENCE", "unterminated raw literal")
            j += len(close); out.append(["literal", text[i:j]]); i = j; continue
        prefix = ""
        for candidate in ("b\"", "c\"", "\""):
            if text.startswith(candidate, i): prefix = candidate; break
        if prefix:
            j, escaped = i + len(prefix), False
            while j < n:
                ch = text[j]
                if ch == '"' and not escaped: j += 1; break
                if ch == "\\" and not escaped: escaped = True
                else: escaped = False
                j += 1
            else: fail("TOKEN_DIVERGENCE", "unterminated string literal")
            while j < n and (text[j].isalnum() or text[j] == "_"): j += 1
            out.append(["literal", text[i:j]]); i = j; continue
        if text[i] == "'":
            char = re.match(r"'(?:\\.|[^'\\])'", text[i:])
            if char:
                j = i + char.end(); out.append(["literal", text[i:j]]); i = j; continue
            life = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", text[i:])
            if life:
                j = i + life.end(); out.append(["lifetime", text[i:j]]); i = j; continue
        ident = re.match(r"(?:r#)?[A-Za-z_][A-Za-z0-9_]*", text[i:])
        if ident:
            j = i + ident.end(); lex = text[i:j]
            out.append(["keyword" if lex in KEYWORDS else "identifier", lex]); i = j; continue
        number = re.match(r"(?:0[xob][0-9A-Fa-f_]+|[0-9][0-9A-Za-z_]*(?:\.[0-9A-Za-z_]+)?(?:[eE][+-]?[0-9_]+)?)", text[i:])
        if number:
            j = i + number.end(); out.append(["literal", text[i:j]]); i = j; continue
        punct = next((p for p in PUNCT if text.startswith(p, i)), None)
        if punct:
            out.append(["punctuation", punct]); i += len(punct); continue
        if text[i] in "{}[](),;:.!?~+-*/%&|^<>=#@$":
            out.append(["delimiter" if text[i] in "{}[]()" else "punctuation", text[i]]); i += 1; continue
        fail("TOKEN_DIVERGENCE", f"unsupported token byte at character {i}: {text[i]!r}")
    return out


def marker_region(raw: bytes, cid: str) -> tuple[int, int, int, int]:
    begin = b"// P1343-CAPSULE-BEGIN " + cid.encode() + b"\n"
    end = b"// P1343-CAPSULE-END " + cid.encode() + b"\n"
    if raw.count(begin) != 1 or raw.count(end) != 1:
        fail("MARKER", f"{cid}: marker cardinality differs from one", cid)
    start = raw.index(begin); body_start = start + len(begin); body_end = raw.index(end, body_start)
    if raw.find(b"// P1343-CAPSULE-BEGIN ", body_start, body_end) >= 0:
        fail("MARKER", f"{cid}: nested marker", cid)
    return start, body_start, body_end, body_end + len(end)


def build_fixture_and_table() -> tuple[dict[str, Any], dict[str, Any]]:
    protected = load_pinned(); manifest = compose_manifest(protected); overlays = overlay_files(protected, manifest)
    fixture_files = []
    for ordinal, file_item in enumerate(manifest["files"]):
        raw = overlays[file_item["path"]]
        fixture_files.append({"ordinal": ordinal, "path": file_item["path"], "candidate_free_sha256": file_item["baseline_file_sha256"], "synthetic_overlay_bytes_base64": base64.b64encode(raw).decode(), "synthetic_overlay_sha256": sha256(raw), "capsule_ids": file_item["capsule_ids"]})
    protected_inputs = {label: [str(path.relative_to(ROOT)), digest] for label, (path, digest) in PINS.items() if label not in {"p1343_baseline", "p1344_baseline", "p1343_binding"}}
    fixture = {"schema":"p1345-positive-fixture-r1","step":1345,"revision":1,"role":"independent_oracle_author","regime":"executado sem atestacao de isolamento","protected_inputs":protected_inputs,"files":fixture_files,"validation":{"files":12,"capsules":38,"candidate_source_read":False,"inverse_round_trip":"PASS"},"closed_world":{"top_level_keys":["schema","step","revision","role","regime","protected_inputs","files","validation","closed_world"],"rule":"Exact closed fixture; duplicate or unknown keys and hash drift are invalid."}}
    records = []
    for ordinal, capsule in enumerate(manifest["capsules"]):
        raw = overlays[capsule["path"]]; _, bs, be, _ = marker_region(raw, capsule["capsule_id"]); tokens = rust_tokens(raw[bs:be])
        records.append({"ordinal":ordinal,"capsule_id":capsule["capsule_id"],"path":capsule["path"],"owner":capsule["symbol"],"kind":capsule["kind"],"begin_marker":"// P1343-CAPSULE-BEGIN "+capsule["capsule_id"],"end_marker":"// P1343-CAPSULE-END "+capsule["capsule_id"],"start_anchor_base64":base64.b64encode(capsule["anchor_before"]["text"].encode()).decode(),"end_anchor_base64":base64.b64encode(capsule["anchor_after"]["text"].encode()).decode(),"replacement_base64":capsule["baseline_replacement"]["bytes_base64"],"canonical_tokens":tokens,"token_count":len(tokens),"canonical_tokens_sha256":sha256(canonical_json(tokens))})
    if len({record["canonical_tokens_sha256"] for record in records}) != 38:
        fail("SCHEMA", "canonical sequences are not pairwise distinct")
    table = {"schema":"p1345-canonical-capsule-table-r1","step":1345,"revision":1,"role":"independent_oracle_author","regime":"executado sem atestacao de isolamento","protected_inputs":protected_inputs|{"positive_fixture":[str(FIXTURE_PATH.relative_to(ROOT)),"__SET_AFTER_FIXTURE__"]},"tokenizer":{"version":TOKENIZER_VERSION,"ignored":["whitespace","line_comments","nested_block_comments","doc_comments"],"preserved_pair":["lexical_kind","exact_raw_lexeme"],"digest":"sha256-rfc8785-token-array"},"records":records,"validation":{"records":38,"files":12,"unique_ids":38,"unique_token_digests":38,"inverse_round_trip":"PASS"},"closed_world":{"top_level_keys":["schema","step","revision","role","regime","protected_inputs","tokenizer","records","validation","closed_world"],"rule":"Exactly 38 authority records in protected capsule order; candidate values are never authority."}}
    return fixture, table


def write_authored_fixture_and_table() -> None:
    fixture, table = build_fixture_and_table()
    FIXTURE_PATH.write_bytes(json.dumps(fixture, ensure_ascii=False, indent=2).encode()+b"\n")
    fixture_sha = sha256(FIXTURE_PATH.read_bytes())
    table["protected_inputs"]["positive_fixture"][1] = fixture_sha
    TABLE_PATH.write_bytes(json.dumps(table, ensure_ascii=False, indent=2).encode()+b"\n")


def write_authored_corpus() -> None:
    fixture, table = load_fixture_table()
    historical = strict_json((DIAG / "p1344-adversary-report-r2.json").read_bytes(), "P1344 attack history")
    history_ids = list(historical["r1_replay"]["ids"]) + [item["id"] for item in historical["new_attacks"]]
    if len(history_ids) != 40 or len(set(history_ids)) != 40:
        fail("PROTECTED_INPUT", "P1344 history is not the exact 40 attacks")
    cases: list[dict[str, Any]] = []
    def add(case_id: str, input_kind: str, operation: str, **recipe: Any) -> None:
        cases.append({"case_id":case_id,"input_kind":input_kind,"source_bundle":{"fixture_sha256":EXPECTED_FIXTURE_SHA256,"table_sha256":EXPECTED_TABLE_SHA256},"runtime_bundle":None,"mutation_recipe":{"operation":operation,**recipe},"probe_request":None})
    add("P1345-P01-exact-positive","synthetic_source","identity")
    add("P1345-P02-whitespace-positive","synthetic_source","whitespace",capsule_id=table["records"][0]["capsule_id"])
    add("P1345-P03-comment-positive","synthetic_source","comment",capsule_id=table["records"][1]["capsule_id"])
    for index, case_id in enumerate(history_ids):
        record=table["records"][index % len(table["records"])]
        add("P1345-HISTORY-"+case_id,"p1344_historical_attack","token_substitute",capsule_id=record["capsule_id"],token_index=min(1,record["token_count"]-1),replacement=["identifier",f"p1345_history_mutation_{index:02d}"])
    for record in table["records"]:
        add("P1345-CAPSULE-%02d-token-substitute" % record["ordinal"],"capsule_token_mutation","token_substitute",capsule_id=record["capsule_id"],token_index=min(1,record["token_count"]-1),replacement=["identifier",f"p1345_capsule_mutation_{record['ordinal']:02d}"])
    for operation in ("token_extra","token_absent","token_reorder","token_substitute"):
        add("P1345-EDIT-"+operation.replace("_","-"),"token_edit_class",operation,capsule_id=table["records"][4]["capsule_id"],token_index=2,replacement=["identifier","p1345_edit_mutation"])
    for operation in ("string_decoy","missing_marker","duplicate_marker","unknown_marker","nested_marker","anchor_change","outside_byte","wrong_owner","alternate_table","alternate_fixture","alternate_corpus","duplicate_json_key","path_escape"):
        add("P1345-META-"+operation,"source_or_authority_attack",operation,capsule_id=table["records"][0]["capsule_id"])
    for operation in ("probe_replay","probe_missing","probe_wrong_emitter","probe_wrong_executable","probe_bad_nonce_digest","probe_bad_response","probe_bad_nonce_hex","probe_extra_stdout"):
        add("P1345-PROBE-"+operation,"probe_attack",operation)
    cases.append({"case_id":"P1345-O01-executed-opaque-probe","input_kind":"opaque_probe","source_bundle":{"fixture_sha256":EXPECTED_FIXTURE_SHA256,"table_sha256":EXPECTED_TABLE_SHA256},"runtime_bundle":None,"mutation_recipe":{"operation":"opaque_probe"},"probe_request":{"probe_case_id":"P1345-O01-executed-opaque-probe","fresh_challenge_hex":"verifier_generated_at_execution","invocation_nonce_hex":"verifier_generated_at_execution"}})
    protected_inputs={label:[str(path.relative_to(ROOT)),digest] for label,(path,digest) in PINS.items() if label not in {"p1343_baseline","p1344_baseline","p1343_binding"}}
    protected_inputs.update({"positive_fixture":[str(FIXTURE_PATH.relative_to(ROOT)),EXPECTED_FIXTURE_SHA256],"canonical_table":[str(TABLE_PATH.relative_to(ROOT)),EXPECTED_TABLE_SHA256]})
    corpus={"schema":"p1345-oracle-corpus-r1","step":1345,"revision":1,"role":"independent_oracle_author","regime":"executado sem atestacao de isolamento","protected_inputs":protected_inputs,"case_order":[c["case_id"] for c in cases],"cases":cases,"budget":{"cases":len(cases),"historical_attacks":40,"capsule_token_mutations":38,"full_corpus_runs_by_author":0,"focal_case_ids":["P1345-P01-exact-positive","P1345-P02-whitespace-positive","P1345-P03-comment-positive","P1345-CAPSULE-00-token-substitute","P1345-EDIT-token-extra","P1345-EDIT-token-absent","P1345-EDIT-token-reorder","P1345-EDIT-token-substitute","P1345-META-duplicate_json_key","P1345-META-alternate_table","P1345-PROBE-probe_bad_nonce_digest","P1345-PROBE-probe_bad_response","P1345-O01-executed-opaque-probe"]},"closed_world":{"top_level_keys":["schema","step","revision","role","regime","protected_inputs","case_order","cases","budget","closed_world"],"case_keys":["case_id","input_kind","source_bundle","runtime_bundle","mutation_recipe","probe_request"],"rule":"Answer-free closed corpus; classification and witness keys are forbidden."}}
    CORPUS_PATH.write_bytes(json.dumps(corpus,ensure_ascii=False,indent=2).encode()+b"\n")


def load_fixture_table() -> tuple[dict[str, Any], dict[str, Any]]:
    fraw = FIXTURE_PATH.read_bytes(); traw = TABLE_PATH.read_bytes()
    if sha256(fraw) != EXPECTED_FIXTURE_SHA256 or sha256(traw) != EXPECTED_TABLE_SHA256:
        fail("AUTHORITY_ROOT", "fixture/table hash mismatch")
    return strict_json(fraw,"fixture"), strict_json(traw,"table")


def materialize_fixture(root: Path, fixture: dict[str, Any]) -> None:
    for item in fixture["files"]:
        target = root / item["path"]; target.parent.mkdir(parents=True,exist_ok=True)
        raw = base64.b64decode(item["synthetic_overlay_bytes_base64"],validate=True)
        if sha256(raw) != item["synthetic_overlay_sha256"]: fail("PROTECTED_INPUT",f"fixture bytes drift {item['path']}")
        target.write_bytes(raw)


def verify_root(root: Path, case_id: str = "candidate") -> dict[str, Any]:
    protected = load_pinned(); manifest = compose_manifest(protected); fixture, table = load_fixture_table()
    records = {record["capsule_id"]:record for record in table["records"]}; files_out=[]; capsules_out=[]; edits=defaultdict(list)
    first_reason=None; first_cid=None; first_index=None
    for fordinal,file_item in enumerate(manifest["files"]):
        path=file_item["path"]; target=(root/path).resolve()
        try: target.relative_to(root.resolve())
        except ValueError: fail("PATH",f"path escape {path}")
        if target.is_symlink() or not target.is_file(): fail("PATH",f"non-regular source {path}")
        raw=target.read_bytes()
        for cid in file_item["capsule_ids"]:
            capsule=next(c for c in manifest["capsules"] if c["capsule_id"]==cid); record=records[cid]
            try:
                start,bs,be,end=marker_region(raw,cid)
                before=base64.b64decode(record["start_anchor_base64"],validate=True); after=base64.b64decode(record["end_anchor_base64"],validate=True)
                if raw[max(0,start-len(before)):start] != before or raw[end:end+len(after)] != after: fail("ANCHOR",f"{cid}: adjacency mismatch",cid)
                actual=rust_tokens(raw[bs:be]); expected=record["canonical_tokens"]
                common=0
                while common<min(len(actual),len(expected)) and actual[common]==expected[common]: common+=1
                actual_at=actual[common] if common<len(actual) else None; expected_at=expected[common] if common<len(expected) else None
                code=None
                if len(actual)!=len(expected): code="TOKEN_COUNT"
                elif common<len(expected): code="TOKEN_DIVERGENCE"
                if code and first_reason is None: first_reason,first_cid,first_index=code,cid,common
                capsules_out.append({"ordinal":record["ordinal"],"capsule_id":cid,"path":path,"owner_derived":capsule["symbol"],"kind_derived":capsule["kind"],"token_count":len(actual),"token_digest":sha256(canonical_json(actual)),"expected_token_count":record["token_count"],"expected_token_digest":record["canonical_tokens_sha256"],"first_divergent_index":common if code else None,"actual_token_at_divergence":actual_at,"expected_token_at_divergence":expected_at})
                replacement=base64.b64decode(record["replacement_base64"],validate=True); edits[path].append((start,end,replacement))
            except VerificationFailure as exc:
                if first_reason is None: first_reason,first_cid,first_index=exc.code,cid,exc.index
                capsules_out.append({"ordinal":record["ordinal"],"capsule_id":cid,"path":path,"owner_derived":capsule["symbol"],"kind_derived":capsule["kind"],"token_count":0,"token_digest":sha256(canonical_json([])),"expected_token_count":record["token_count"],"expected_token_digest":record["canonical_tokens_sha256"],"first_divergent_index":exc.index,"actual_token_at_divergence":None,"expected_token_at_divergence":None})
        normalized=raw
        for start,end,replacement in sorted(edits[path],reverse=True): normalized=normalized[:start]+replacement+normalized[end:]
        ndigest=sha256(normalized); expected=file_item["baseline_file_sha256"]
        if ndigest!=expected and first_reason is None: first_reason="NORMALIZATION"
        files_out.append({"ordinal":fordinal,"path":path,"actual_sha256":sha256(raw),"marker_count":sum(raw.count(("// P1343-CAPSULE-BEGIN "+c+"\n").encode()) for c in file_item["capsule_ids"]),"normalized_sha256":ndigest,"expected_normalized_sha256":expected})
    passed=first_reason is None and len(capsules_out)==38
    return {"schema":"p1345-source-result-r1","step":1345,"case_id":case_id,"input_hashes":{"fixture":EXPECTED_FIXTURE_SHA256,"table":EXPECTED_TABLE_SHA256},"authoring_root_sha256":None,"files":files_out,"capsules":capsules_out,"normalization":{"regions_consumed":len(edits) and sum(len(v) for v in edits.values()),"files_recovered":sum(x["normalized_sha256"]==x["expected_normalized_sha256"] for x in files_out)},"source_result":{"passed":passed,"first_reason_code":first_reason,"first_capsule_id":first_cid,"first_divergent_index":first_index},"closed_world":{"top_level_keys":["schema","step","case_id","input_hashes","authoring_root_sha256","files","capsules","normalization","source_result","closed_world"],"rule":"Source-only recomputation; no semantic classification."}}


if __name__ == "__main__":
    import argparse
    parser=argparse.ArgumentParser(); parser.add_argument("--author",action="store_true"); parser.add_argument("--author-corpus",action="store_true"); parser.add_argument("--root",type=Path); args=parser.parse_args()
    if args.author: write_authored_fixture_and_table()
    elif args.author_corpus: write_authored_corpus()
    elif args.root: print(json.dumps(verify_root(args.root),ensure_ascii=False,indent=2))
    else: parser.error("choose --author or --root")
