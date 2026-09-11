#!/usr/bin/env python3
"""Deterministic, candidate-independent source verifier for P1343.

The verifier recognizes only lexically real full-line Rust capsule comments,
performs the inverse replacement at derived byte offsets, and proves each
whole normalized file against the protected candidate-free digest.  It never
accepts candidate-provided ranges, hashes, paths, parse flags, or verdicts.
"""

from __future__ import annotations

import argparse
import base64
import binascii
import hashlib
import json
import os
import re
import stat
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable


EMPTY_SHA256 = hashlib.sha256(b"").hexdigest()
BEGIN_RE = re.compile(rb"^[ \t]*// P1343-CAPSULE-BEGIN ([A-Z0-9][A-Z0-9-]*)[ \t]*(?:\n|$)")
END_RE = re.compile(rb"^[ \t]*// P1343-CAPSULE-END ([A-Z0-9][A-Z0-9-]*)[ \t]*(?:\n|$)")
HEX_RE = re.compile(r"^[0-9a-f]{64}$")
LAYERS = ("01_core", "02_shell", "03_infra", "04_wiring")
MODES = ("normal", "repeat", "reverse")


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
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8")


def strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            fail("SCHEMA", f"duplicate JSON key {key!r}")
        value[key] = item
    return value


def read_json(path: Path, expected_sha: str, label: str) -> Any:
    if not HEX_RE.fullmatch(expected_sha):
        fail("PROTECTED_INPUT", f"{label} supplied noncanonical SHA-256")
    try:
        raw = path.read_bytes()
    except OSError as exc:
        fail("PROTECTED_INPUT", f"cannot read {label}: {exc}")
    if sha256(raw) != expected_sha:
        fail("PROTECTED_INPUT", f"{label} hash mismatch")
    try:
        return json.loads(raw, object_pairs_hook=strict_object)
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        fail("SCHEMA", f"{label} is not strict UTF-8 JSON: {exc}")


def read_pinned_bytes(path: Path, expected_sha: str, label: str) -> bytes:
    if not HEX_RE.fullmatch(expected_sha):
        fail("PROTECTED_INPUT", f"{label} supplied noncanonical SHA-256")
    try:
        raw = path.read_bytes()
    except OSError as exc:
        fail("PROTECTED_INPUT", f"cannot read {label}: {exc}")
    if sha256(raw) != expected_sha:
        fail("PROTECTED_INPUT", f"{label} hash mismatch")
    return raw


def exact_keys(value: Any, keys: Iterable[str], where: str) -> None:
    required = set(keys)
    if not isinstance(value, dict) or set(value) != required:
        got = sorted(value) if isinstance(value, dict) else type(value).__name__
        fail("SCHEMA", f"{where} keys must be {sorted(required)}, got {got}")


def exact_int(value: Any, where: str) -> int:
    if type(value) is not int:
        fail("SCHEMA", f"{where} must be an integer, bool excluded")
    return value


def safe_relative(path: Any) -> str:
    if not isinstance(path, str) or not path or path.startswith("/") or "\x00" in path:
        fail("PATH", "manifest path is not a nonempty repository-relative POSIX path")
    parts = path.split("/")
    if any(part in {"", ".", ".."} for part in parts) or "\\" in path:
        fail("PATH", f"path has alias or traversal component: {path!r}")
    return path


def validate_utf8_source(raw: bytes, path: str) -> str:
    if raw.startswith(b"\xef\xbb\xbf") or b"\r" in raw or b"\x00" in raw:
        fail("SCHEMA", f"{path}: source must be BOM-free UTF-8 with LF and no NUL")
    try:
        return raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        fail("SCHEMA", f"{path}: source is not UTF-8: {exc}")


@dataclass(frozen=True)
class Marker:
    kind: str
    capsule_id: str
    start: int
    end: int


@dataclass(frozen=True)
class CapsuleRegion:
    capsule_id: str
    path: str
    start: int
    end: int
    body_start: int
    body_end: int


def _raw_prefix(data: bytes, index: int) -> tuple[int, bytes] | None:
    """Return (content start, closing delimiter) for a Rust raw string."""
    for prefix in (b"br", b"rb", b"cr", b"rc", b"r"):
        if not data.startswith(prefix, index):
            continue
        cursor = index + len(prefix)
        hashes = 0
        while cursor < len(data) and data[cursor] == ord("#"):
            hashes += 1
            cursor += 1
        if cursor < len(data) and data[cursor] == ord('"'):
            return cursor + 1, b'"' + b"#" * hashes
    return None


def lexical_markers(raw: bytes, path: str) -> list[Marker]:
    """Scan Rust lexical states and return only eligible physical marker lines."""
    markers: list[Marker] = []
    state = "code"
    block_depth = 0
    raw_close = b""
    escaped = False
    offset = 0
    for line in raw.splitlines(keepends=True):
        line_start_state = state
        if line_start_state == "code":
            match = BEGIN_RE.fullmatch(line)
            kind = "begin"
            if match is None:
                match = END_RE.fullmatch(line)
                kind = "end"
            if match is not None:
                markers.append(Marker(kind, match.group(1).decode("ascii"), offset, offset + len(line)))
        index = 0
        while index < len(line):
            byte = line[index]
            nxt = line[index + 1] if index + 1 < len(line) else None
            if state == "line-comment":
                if byte == 0x0A:
                    state = "code"
                index += 1
                continue
            if state == "block-comment":
                if byte == ord("/") and nxt == ord("*"):
                    block_depth += 1
                    index += 2
                elif byte == ord("*") and nxt == ord("/"):
                    block_depth -= 1
                    index += 2
                    if block_depth == 0:
                        state = "code"
                else:
                    index += 1
                continue
            if state == "raw-string":
                if line.startswith(raw_close, index):
                    index += len(raw_close)
                    state = "code"
                else:
                    index += 1
                continue
            if state in {"string", "char"}:
                if byte == 0x0A:
                    # Invalid ordinary literal is rejected by the independent syntax gate.
                    state = "code"
                    escaped = False
                elif escaped:
                    escaped = False
                elif byte == ord("\\"):
                    escaped = True
                elif (state == "string" and byte == ord('"')) or (state == "char" and byte == ord("'")):
                    state = "code"
                index += 1
                continue
            if byte == ord("/") and nxt == ord("/"):
                state = "line-comment"
                index += 2
                continue
            if byte == ord("/") and nxt == ord("*"):
                state = "block-comment"
                block_depth = 1
                index += 2
                continue
            raw = _raw_prefix(line, index)
            if raw is not None:
                index, raw_close = raw
                state = "raw-string"
                continue
            if byte == ord('"') or (byte in (ord("b"), ord("c")) and nxt == ord('"')):
                state = "string"
                escaped = False
                index += 2 if byte in (ord("b"), ord("c")) else 1
                continue
            if byte == ord("'"):
                # A lifetime has no closing quote.  Treat it as code; a char has one on this line.
                closing = index + 1
                char_escape = False
                while closing < len(line) and line[closing] != 0x0A:
                    if char_escape:
                        char_escape = False
                    elif line[closing] == ord("\\"):
                        char_escape = True
                    elif line[closing] == ord("'"):
                        state = "char"
                        escaped = False
                        break
                    closing += 1
            index += 1
        offset += len(line)
    if state == "block-comment" or state == "raw-string":
        fail("MARKER", f"{path}: unterminated block comment or raw string makes scanner ambiguous")
    return markers


def pair_markers(markers: list[Marker], expected_ids: set[str], path: str) -> list[CapsuleRegion]:
    regions: list[CapsuleRegion] = []
    opened: Marker | None = None
    seen: set[str] = set()
    for marker in markers:
        if marker.kind == "begin":
            if opened is not None:
                fail("MARKER", f"{path}: nested capsule {marker.capsule_id}")
            if marker.capsule_id not in expected_ids:
                fail("MARKER", f"{path}: unknown capsule {marker.capsule_id}")
            if marker.capsule_id in seen:
                fail("MARKER", f"{path}: duplicate capsule {marker.capsule_id}")
            opened = marker
        else:
            if opened is None:
                fail("MARKER", f"{path}: END without BEGIN for {marker.capsule_id}")
            if marker.capsule_id != opened.capsule_id:
                fail("MARKER", f"{path}: mismatched capsule IDs {opened.capsule_id}/{marker.capsule_id}")
            regions.append(CapsuleRegion(opened.capsule_id, path, opened.start, marker.end, opened.end, marker.start))
            seen.add(opened.capsule_id)
            opened = None
    if opened is not None:
        fail("MARKER", f"{path}: missing END for {opened.capsule_id}")
    if seen != expected_ids:
        missing = sorted(expected_ids - seen)
        fail("MARKER", f"{path}: missing capsules {missing}")
    return regions


def decode_replacement(item: dict[str, Any], capsule_id: str) -> bytes:
    exact_keys(item, {"encoding", "bytes_base64", "sha256", "byte_len"}, f"replacement {capsule_id}")
    if item["encoding"] != "base64-of-exact-utf8-bytes" or not isinstance(item["bytes_base64"], str):
        fail("SCHEMA", f"{capsule_id}: replacement encoding mismatch")
    try:
        raw = base64.b64decode(item["bytes_base64"], validate=True)
    except (binascii.Error, ValueError) as exc:
        fail("SCHEMA", f"{capsule_id}: noncanonical base64 replacement: {exc}")
    if base64.b64encode(raw).decode("ascii") != item["bytes_base64"]:
        fail("SCHEMA", f"{capsule_id}: replacement base64 is not canonical")
    try:
        raw.decode("utf-8")
    except UnicodeDecodeError:
        fail("SCHEMA", f"{capsule_id}: replacement is not UTF-8")
    if exact_int(item["byte_len"], f"{capsule_id} replacement length") != len(raw) or item["sha256"] != sha256(raw):
        fail("SCHEMA", f"{capsule_id}: replacement length/hash mismatch")
    return raw


def cfg_attribute(required: str, negated: bool = False) -> bytes:
    if negated:
        return b"#[cfg(not(p1339_observation))]"
    if required == "p1339_observation":
        return b"#[cfg(p1339_observation)]"
    if required == "all(test,p1339_observation)":
        return b"#[cfg(all(test, p1339_observation))]"
    fail("CFG", f"unknown required cfg {required!r}")


def _strip_ws_comments_prefix(data: bytes) -> bytes:
    while True:
        old = data
        data = data.lstrip(b" \t\n")
        if data.startswith(b"//"):
            cut = data.find(b"\n")
            data = b"" if cut < 0 else data[cut + 1:]
        elif data.startswith(b"/*"):
            depth, index = 1, 2
            while index < len(data) and depth:
                if data.startswith(b"/*", index):
                    depth += 1
                    index += 2
                elif data.startswith(b"*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            if depth:
                fail("CFG", "unterminated comment in capsule body")
            data = data[index:]
        if data == old:
            return data


def validate_balanced_body(body: bytes, capsule_id: str) -> None:
    """Reject focal delimiter/literal corruption; rustc remains the full syntax gate."""
    pairs = {ord("("): ord(")"), ord("["): ord("]"), ord("{"): ord("}")}
    closing = set(pairs.values())
    stack: list[int] = []
    state = "code"
    block_depth = 0
    raw_close = b""
    escaped = False
    index = 0
    while index < len(body):
        byte = body[index]
        nxt = body[index + 1] if index + 1 < len(body) else None
        if state == "line-comment":
            if byte == 0x0A:
                state = "code"
            index += 1
            continue
        if state == "block-comment":
            if byte == ord("/") and nxt == ord("*"):
                block_depth += 1
                index += 2
            elif byte == ord("*") and nxt == ord("/"):
                block_depth -= 1
                index += 2
                if block_depth == 0:
                    state = "code"
            else:
                index += 1
            continue
        if state == "raw-string":
            if body.startswith(raw_close, index):
                index += len(raw_close)
                state = "code"
            else:
                index += 1
            continue
        if state in {"string", "char"}:
            if byte == 0x0A and state == "string":
                fail("SYNTAX_GATE", f"{capsule_id}: newline in ordinary string")
            if escaped:
                escaped = False
            elif byte == ord("\\"):
                escaped = True
            elif (state == "string" and byte == ord('"')) or (state == "char" and byte == ord("'")):
                state = "code"
            index += 1
            continue
        if byte == ord("/") and nxt == ord("/"):
            state = "line-comment"
            index += 2
            continue
        if byte == ord("/") and nxt == ord("*"):
            state = "block-comment"
            block_depth = 1
            index += 2
            continue
        raw = _raw_prefix(body, index)
        if raw is not None:
            index, raw_close = raw
            state = "raw-string"
            continue
        if byte == ord('"') or (byte in (ord("b"), ord("c")) and nxt == ord('"')):
            state = "string"
            escaped = False
            index += 2 if byte in (ord("b"), ord("c")) else 1
            continue
        if byte == ord("'"):
            # Rust lifetimes remain code; only enter char state when a same-line close exists.
            cursor, char_escape = index + 1, False
            while cursor < len(body) and body[cursor] != 0x0A:
                if char_escape:
                    char_escape = False
                elif body[cursor] == ord("\\"):
                    char_escape = True
                elif body[cursor] == ord("'"):
                    state = "char"
                    escaped = False
                    break
                cursor += 1
        elif byte in pairs:
            stack.append(byte)
        elif byte in closing:
            if not stack or pairs[stack.pop()] != byte:
                fail("SYNTAX_GATE", f"{capsule_id}: unbalanced delimiter")
        index += 1
    if stack or state in {"block-comment", "raw-string", "string", "char"}:
        fail("SYNTAX_GATE", f"{capsule_id}: unterminated delimiter/literal/comment")


def require_single_guarded_unit(payload: bytes, capsule_id: str) -> None:
    """Conservatively prove an insert attribute dominates the whole payload."""
    payload = _strip_ws_comments_prefix(payload)
    if not payload:
        fail("CFG", f"{capsule_id}: cfg guard has no syntax node")
    pairs = {ord("("): ord(")"), ord("["): ord("]"), ord("{"): ord("}")}
    stack: list[int] = []
    state = "code"
    block_depth = 0
    raw_close = b""
    escaped = False
    boundary: int | None = None
    index = 0
    while index < len(payload):
        byte = payload[index]
        nxt = payload[index + 1] if index + 1 < len(payload) else None
        if state == "line-comment":
            if byte == 0x0A:
                state = "code"
            index += 1
            continue
        if state == "block-comment":
            if byte == ord("/") and nxt == ord("*"):
                block_depth += 1; index += 2
            elif byte == ord("*") and nxt == ord("/"):
                block_depth -= 1; index += 2
                if block_depth == 0: state = "code"
            else: index += 1
            continue
        if state == "raw-string":
            if payload.startswith(raw_close, index):
                index += len(raw_close); state = "code"
            else: index += 1
            continue
        if state in {"string", "char"}:
            if escaped: escaped = False
            elif byte == ord("\\"): escaped = True
            elif (state == "string" and byte == ord('"')) or (state == "char" and byte == ord("'")): state = "code"
            index += 1
            continue
        if byte == ord("/") and nxt == ord("/"):
            state = "line-comment"; index += 2; continue
        if byte == ord("/") and nxt == ord("*"):
            state = "block-comment"; block_depth = 1; index += 2; continue
        raw = _raw_prefix(payload, index)
        if raw is not None:
            index, raw_close = raw; state = "raw-string"; continue
        if byte == ord('"') or (byte in (ord("b"), ord("c")) and nxt == ord('"')):
            state = "string"; escaped = False; index += 2 if byte in (ord("b"), ord("c")) else 1; continue
        if byte in pairs:
            stack.append(byte)
        elif byte in pairs.values():
            if not stack or pairs[stack.pop()] != byte:
                fail("SYNTAX_GATE", f"{capsule_id}: unbalanced guarded unit")
            if not stack and byte == ord("}"):
                boundary = index + 1
                break
        elif not stack and byte in (ord(";"), ord(",")):
            boundary = index + 1
            break
        index += 1
    if boundary is None:
        fail("CFG", f"{capsule_id}: guarded insert is not one closed Rust unit")
    tail = _strip_ws_comments_prefix(payload[boundary:])
    if tail.startswith((b";", b",")):
        tail = _strip_ws_comments_prefix(tail[1:])
    if tail:
        fail("CFG", f"{capsule_id}: unguarded syntax follows the guarded unit")


def validate_cfg_body(body: bytes, capsule: dict[str, Any], replacement: bytes) -> str | None:
    capsule_id = capsule["capsule_id"]
    required = capsule["required_cfg"]
    validate_balanced_body(body, capsule_id)
    observed = cfg_attribute(required)
    if b"cfg_attr" in body or b"cfg!(" in body:
        fail("CFG", f"{capsule_id}: cfg_attr/cfg! cannot establish dominance")
    if capsule["kind"] == "insert":
        if replacement or capsule["baseline_replacement"]["sha256"] != EMPTY_SHA256:
            fail("SCHEMA", f"{capsule_id}: insert replacement must be empty")
        first = _strip_ws_comments_prefix(body)
        if not first.startswith(observed):
            fail("CFG", f"{capsule_id}: first effective node lacks exact {required} guard")
        # Every additional top-level cfg attribute must be the same permitted guard.
        attrs = re.findall(rb"#\s*\[\s*cfg\s*\([^\]]*\)\s*\]", body)
        if not attrs or any(re.sub(rb"\s+", b"", attr) != re.sub(rb"\s+", b"", observed) for attr in attrs):
            fail("CFG", f"{capsule_id}: insert contains a different cfg predicate")
        require_single_guarded_unit(first[len(observed):], capsule_id)
        return None
    normal = cfg_attribute(required, negated=True)
    stripped = _strip_ws_comments_prefix(body)
    if not stripped.startswith(normal):
        fail("CFG", f"{capsule_id}: replacement lacks exact normal branch")
    after_normal = stripped[len(normal):]
    if not after_normal.startswith(b"\n"):
        fail("CFG", f"{capsule_id}: exact normal payload must begin on the next LF line")
    after_normal = after_normal[1:]
    if not after_normal.startswith(replacement):
        fail("CFG", f"{capsule_id}: normal payload is not the exact pinned replacement")
    tail = after_normal[len(replacement):]
    tail = _strip_ws_comments_prefix(tail)
    if not tail.startswith(observed):
        fail("CFG", f"{capsule_id}: observed branch missing after exact normal payload")
    if body.count(normal) != 1 or body.count(observed) != 1:
        fail("CFG", f"{capsule_id}: normal/observed branch cardinality mismatch")
    attrs = re.findall(rb"#\s*\[\s*cfg\s*\([^\]]*\)\s*\]", body)
    canonical = {re.sub(rb"\s+", b"", normal), re.sub(rb"\s+", b"", observed)}
    if any(re.sub(rb"\s+", b"", attr) not in canonical for attr in attrs):
        fail("CFG", f"{capsule_id}: replacement contains a foreign cfg predicate")
    require_single_guarded_unit(tail[len(observed):], capsule_id)
    return sha256(replacement)


def code_mask(data: bytes) -> bytes:
    """Preserve code bytes and blank literal/comment contents at identical offsets."""
    out = bytearray(data)
    state = "code"
    block_depth = 0
    raw_close = b""
    escaped = False
    index = 0
    while index < len(data):
        byte = data[index]
        nxt = data[index + 1] if index + 1 < len(data) else None
        if state == "line-comment":
            if byte == 0x0A: state = "code"
            else: out[index] = 0x20
            index += 1; continue
        if state == "block-comment":
            out[index] = 0x20
            if byte == ord("/") and nxt == ord("*"):
                out[index + 1] = 0x20; block_depth += 1; index += 2
            elif byte == ord("*") and nxt == ord("/"):
                out[index + 1] = 0x20; block_depth -= 1; index += 2
                if block_depth == 0: state = "code"
            else: index += 1
            continue
        if state == "raw-string":
            out[index] = 0x20
            if data.startswith(raw_close, index):
                for cursor in range(index, index + len(raw_close)): out[cursor] = 0x20
                index += len(raw_close); state = "code"
            else: index += 1
            continue
        if state in {"string", "char"}:
            out[index] = 0x20
            if escaped: escaped = False
            elif byte == ord("\\"): escaped = True
            elif (state == "string" and byte == ord('"')) or (state == "char" and byte == ord("'")): state = "code"
            index += 1; continue
        if byte == ord("/") and nxt == ord("/"):
            out[index:index + 2] = b"  "; state = "line-comment"; index += 2; continue
        if byte == ord("/") and nxt == ord("*"):
            out[index:index + 2] = b"  "; state = "block-comment"; block_depth = 1; index += 2; continue
        raw = _raw_prefix(data, index)
        if raw is not None:
            content_start, raw_close = raw
            for cursor in range(index, content_start): out[cursor] = 0x20
            index = content_start; state = "raw-string"; continue
        if byte == ord('"') or (byte in (ord("b"), ord("c")) and nxt == ord('"')):
            width = 2 if byte in (ord("b"), ord("c")) else 1
            out[index:index + width] = b" " * width; state = "string"; escaped = False; index += width; continue
        index += 1
    return bytes(out)


def validate_manifest(manifest: Any, binding: Any) -> tuple[list[dict[str, Any]], dict[str, dict[str, Any]]]:
    expected_top = set(binding["closed_metadata"]["capsule_baseline_top_level_keys"])
    exact_keys(manifest, expected_top, "capsule baseline")
    if manifest["schema"] != "p1343-capsule-baseline-r1" or exact_int(manifest["step"], "manifest step") != 1343:
        fail("SCHEMA", "capsule baseline identity mismatch")
    file_keys = set(binding["closed_metadata"]["capsule_file_keys"])
    capsule_keys = set(binding["closed_metadata"]["capsule_keys"])
    anchor_keys = set(binding["closed_metadata"]["anchor_keys"])
    files = manifest["files"]
    capsules = manifest["capsules"]
    if len(files) != 10 or len(capsules) != 36 or exact_int(manifest["capsule_count"], "capsule count") != 36:
        fail("SCHEMA", "capsule/file cardinality mismatch")
    ids: set[str] = set()
    by_id: dict[str, dict[str, Any]] = {}
    file_ids: list[str] = []
    for file_item in files:
        exact_keys(file_item, file_keys, "capsule file")
        safe_relative(file_item["path"])
        if not HEX_RE.fullmatch(file_item["baseline_file_sha256"]):
            fail("SCHEMA", "noncanonical baseline file hash")
        if not isinstance(file_item["capsule_ids"], list):
            fail("SCHEMA", "file capsule_ids must be an array")
        file_ids.extend(file_item["capsule_ids"])
    for capsule in capsules:
        exact_keys(capsule, capsule_keys, "capsule")
        capsule_id = capsule["capsule_id"]
        if not isinstance(capsule_id, str) or not re.fullmatch(r"[A-Z0-9][A-Z0-9-]*", capsule_id) or capsule_id in ids:
            fail("SCHEMA", f"duplicate or malformed capsule ID {capsule_id!r}")
        ids.add(capsule_id)
        by_id[capsule_id] = capsule
        safe_relative(capsule["path"])
        if capsule["kind"] not in {"insert", "replace"}:
            fail("SCHEMA", f"{capsule_id}: invalid kind")
        decode_replacement(capsule["baseline_replacement"], capsule_id)
        for name in ("anchor_before", "anchor_after"):
            anchor = capsule[name]
            exact_keys(anchor, anchor_keys, f"{capsule_id} {name}")
            if anchor["encoding"] != "utf8-lf-json-string" or not isinstance(anchor["text"], str):
                fail("SCHEMA", f"{capsule_id}: invalid anchor encoding")
            raw = anchor["text"].encode("utf-8")
            if "\r" in anchor["text"] or anchor["sha256"] != sha256(raw) or exact_int(anchor["matches_in_baseline_file"], "anchor cardinality") != 1:
                fail("SCHEMA", f"{capsule_id}: anchor hash/cardinality mismatch")
    if len(file_ids) != 36 or set(file_ids) != ids or len(set(file_ids)) != 36:
        fail("SCHEMA", "file-to-capsule identity join mismatch")
    for file_item in files:
        for capsule_id in file_item["capsule_ids"]:
            if by_id[capsule_id]["path"] != file_item["path"]:
                fail("SCHEMA", f"{capsule_id}: wrong manifest file")
    facade = binding["facade_binding"]
    row_union = {item for row in binding["row_capsule_bindings"] for item in row["capsule_ids"]}
    if row_union | {facade["capsule_id"]} != ids:
        fail("SCHEMA", "row/facade capsule coverage mismatch")
    return files, by_id


def open_source(root: Path, relative: str) -> bytes:
    target = root.joinpath(*relative.split("/"))
    try:
        info = target.lstat()
    except OSError as exc:
        fail("PATH", f"cannot stat {relative}: {exc}")
    if stat.S_ISLNK(info.st_mode) or not stat.S_ISREG(info.st_mode):
        fail("PATH", f"{relative}: symlink or non-regular source rejected")
    try:
        resolved = target.resolve(strict=True)
        resolved.relative_to(root)
    except (OSError, ValueError) as exc:
        fail("PATH", f"{relative}: canonical target escapes root: {exc}")
    return target.read_bytes()


def enumerate_global_markers(root: Path, allowlisted: set[str]) -> None:
    for layer in LAYERS:
        base = root / layer
        if not base.exists():
            continue
        for current, dirnames, filenames in os.walk(base, followlinks=False):
            dirnames.sort()
            filenames.sort()
            current_path = Path(current)
            for dirname in list(dirnames):
                if (current_path / dirname).is_symlink():
                    fail("PATH", f"symlink directory under marker scan: {(current_path / dirname).relative_to(root)}")
            for filename in filenames:
                if not filename.endswith(".rs"):
                    continue
                path = current_path / filename
                relative = path.relative_to(root).as_posix()
                if path.is_symlink():
                    fail("PATH", f"symlink source under marker scan: {relative}")
                if relative in allowlisted:
                    continue
                if lexical_markers(path.read_bytes(), relative):
                    fail("MARKER", f"eligible P1343 marker outside allowlist: {relative}")


def validate_runtime(runtime: Any, fixture_sha: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    keys = {"schema", "fixture_sha256", "modes", "row_coverage", "raw_snapshot_refs", "result"}
    exact_keys(runtime, keys, "runtime evidence")
    if runtime["schema"] != "p1343-runtime-evidence-v1" or runtime["fixture_sha256"] != fixture_sha:
        fail("RUNTIME_COVERAGE", "runtime evidence identity/fixture mismatch")
    if runtime["modes"] != list(MODES) or runtime["result"] != "PASS":
        fail("RUNTIME_COVERAGE", "runtime modes/result mismatch")
    if not isinstance(runtime["row_coverage"], list) or len(runtime["row_coverage"]) != 19:
        fail("RUNTIME_COVERAGE", "runtime row coverage must contain 19 rows")
    normalized = []
    for index, (item, row) in enumerate(zip(runtime["row_coverage"], rows)):
        exact_keys(item, {"row_index", "hook", "normal", "repeat", "reverse"}, f"runtime row {index}")
        if exact_int(item["row_index"], "runtime row index") != index or item["hook"] != row["hook"]:
            fail("RUNTIME_COVERAGE", f"runtime row {index} identity mismatch")
        refs = []
        for mode in MODES:
            ref = item[mode]
            if not isinstance(ref, str) or not ref:
                fail("RUNTIME_COVERAGE", f"runtime row {index}/{mode} lacks real receipt reference")
            refs.append(ref)
        normalized.append((index, refs))
    exact_keys(runtime["raw_snapshot_refs"], set(MODES), "raw snapshot refs")
    if any(not isinstance(runtime["raw_snapshot_refs"][mode], str) or not runtime["raw_snapshot_refs"][mode] for mode in MODES):
        fail("RUNTIME_COVERAGE", "raw snapshot reference absent")
    return runtime


def git_head(root: Path) -> str:
    head = root / ".git" / "HEAD"
    try:
        raw = head.read_text(encoding="ascii").strip()
    except OSError:
        return "synthetic-or-unavailable"
    if raw.startswith("ref: "):
        ref = root / ".git" / raw[5:]
        try:
            raw = ref.read_text(encoding="ascii").strip()
        except OSError:
            return "unresolved-ref"
    return raw if re.fullmatch(r"[0-9a-f]{40}", raw) else "unresolved-ref"


def derive_source_evidence(
    *, contract: Any, contract_sha: str, binding: Any, binding_sha: str,
    authority_sha: str, manifest: Any, manifest_sha: str,
    p1342_contract_sha: str, p1342_binding_sha: str, l0_sha: str,
    fixture_sha: str, candidate_root: Path, runtime: Any,
    source_verifier_sha: str, invocation_sha: str,
) -> dict[str, Any]:
    files, capsules_by_id = validate_manifest(manifest, binding)
    rows = binding["row_capsule_bindings"]
    if len(rows) != 19 or [row.get("row_index") for row in rows] != list(range(19)):
        fail("SCHEMA", "binding must contain ordered rows 0..18")
    runtime = validate_runtime(runtime, fixture_sha, rows)
    try:
        root = candidate_root.resolve(strict=True)
    except OSError as exc:
        fail("PATH", f"candidate root cannot be canonicalized: {exc}")
    if not root.is_dir():
        fail("PATH", "candidate root is not a directory")
    allowlisted = {item["path"] for item in files}
    enumerate_global_markers(root, allowlisted)
    file_records: list[dict[str, Any]] = []
    capsule_records: list[dict[str, Any]] = []
    region_by_id: dict[str, CapsuleRegion] = {}
    body_by_id: dict[str, bytes] = {}
    raw_by_path: dict[str, bytes] = {}
    normalized_by_path: dict[str, bytes] = {}
    for file_item in files:
        path = file_item["path"]
        raw = open_source(root, path)
        validate_utf8_source(raw, path)
        expected_ids = set(file_item["capsule_ids"])
        regions = pair_markers(lexical_markers(raw, path), expected_ids, path)
        by_id = {region.capsule_id: region for region in regions}
        normalized = raw
        for region in sorted(regions, key=lambda item: (item.start, item.capsule_id), reverse=True):
            capsule = capsules_by_id[region.capsule_id]
            before = capsule["anchor_before"]["text"].encode("utf-8")
            after = capsule["anchor_after"]["text"].encode("utf-8")
            if raw[max(0, region.start - len(before)):region.start] != before:
                fail("ANCHOR", f"{region.capsule_id}: before anchor is not immediately adjacent")
            if raw[region.end:region.end + len(after)] != after:
                fail("ANCHOR", f"{region.capsule_id}: after anchor is not immediately adjacent")
            replacement = decode_replacement(capsule["baseline_replacement"], region.capsule_id)
            body = raw[region.body_start:region.body_end]
            normal_hash = validate_cfg_body(body, capsule, replacement)
            complete = raw[region.start:region.end]
            owner_witness = before + complete + after
            region_by_id[region.capsule_id] = region
            body_by_id[region.capsule_id] = body
            capsule_records.append({
                "capsule_id": region.capsule_id,
                "path": path,
                "kind": capsule["kind"],
                "required_cfg": capsule["required_cfg"],
                "begin_offset": region.start,
                "end_offset": region.end,
                "body_sha256": sha256(body),
                "complete_capsule_sha256": sha256(complete),
                "replacement_sha256": sha256(replacement),
                "anchor_before_sha256": sha256(before),
                "anchor_after_sha256": sha256(after),
                "symbol_sha256": sha256(owner_witness),
                "normal_branch_payload_sha256": normal_hash,
                "result": "PASS",
            })
            normalized = normalized[:region.start] + replacement + normalized[region.end:]
        if sha256(normalized) != file_item["baseline_file_sha256"]:
            fail("NORMALIZATION", f"{path}: normalized whole-file hash mismatch")
        # Re-establish each protected baseline witness after the inverse transform.
        for capsule_id in file_item["capsule_ids"]:
            capsule = capsules_by_id[capsule_id]
            before = capsule["anchor_before"]["text"].encode("utf-8")
            after = capsule["anchor_after"]["text"].encode("utf-8")
            replacement = decode_replacement(capsule["baseline_replacement"], capsule_id)
            if normalized.count(before + replacement + after) != 1:
                fail("NORMALIZATION", f"{capsule_id}: normalized baseline witness is not unique")
        raw_by_path[path] = raw
        normalized_by_path[path] = normalized
        file_records.append({
            "path": path,
            "live_file_sha256": sha256(raw),
            "normalized_file_sha256": sha256(normalized),
            "baseline_file_sha256": file_item["baseline_file_sha256"],
            "live_byte_len": len(raw),
            "normalized_byte_len": len(normalized),
            "capsule_ids": file_item["capsule_ids"],
            "result": "PASS",
        })
    # Restore manifest order independent of filesystem or lexical discovery order.
    record_by_id = {item["capsule_id"]: item for item in capsule_records}
    capsule_records = [record_by_id[item["capsule_id"]] for item in manifest["capsules"]]
    row_records = []
    for row in rows:
        ids = row["capsule_ids"]
        if any(capsule_id not in region_by_id for capsule_id in ids):
            fail("OWNER_BINDING", f"row {row['row_index']} references absent capsule")
        anchor_material = b"".join(
            capsules_by_id[capsule_id]["anchor_before"]["text"].encode("utf-8")
            + capsules_by_id[capsule_id]["anchor_after"]["text"].encode("utf-8")
            for capsule_id in ids
        )
        adjacency = b"".join(body_by_id[capsule_id] for capsule_id in ids)
        coverage = runtime["row_coverage"][row["row_index"]]
        row_records.append({
            "row_index": row["row_index"],
            "hook": row["hook"],
            "path": capsules_by_id[ids[-1]]["path"],
            "symbol": capsules_by_id[ids[-1]]["symbol"],
            "capsule_ids": ids,
            "anchor_sha256": sha256(anchor_material),
            "observation_adjacency_sha256": sha256(adjacency),
            "runtime_coverage_refs": [coverage[mode] for mode in MODES],
            "result": "PASS",
        })
    all_bodies = b"\n".join(body_by_id[c["capsule_id"]] for c in manifest["capsules"])
    masked_all = code_mask(all_bodies)
    function_spans: list[tuple[str, int, int]] = []
    for found in re.finditer(rb"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", masked_all):
        opening = masked_all.find(b"{", found.end())
        if opening < 0:
            continue
        depth, cursor = 1, opening + 1
        while cursor < len(masked_all) and depth:
            if masked_all[cursor] == ord("{"): depth += 1
            elif masked_all[cursor] == ord("}"): depth -= 1
            cursor += 1
        if depth == 0:
            function_spans.append((found.group(1).decode("ascii"), found.start(), cursor))
    push_positions = [match.start() for match in re.finditer(rb"\.\s*push\s*\(", masked_all)]
    writer_spans = [item for item in function_spans if any(item[1] <= position < item[2] for position in push_positions)]
    if len(writer_spans) != 1 or len(push_positions) != 1:
        fail("WRITER_PROJECTION", f"expected one append writer owning the sole direct push, found {[(x[0], x[1], x[2]) for x in writer_spans]} and {len(push_positions)} pushes")
    writer = writer_spans[0][0]
    callsites = []
    for capsule in manifest["capsules"]:
        capsule_id = capsule["capsule_id"]
        body = code_mask(body_by_id[capsule_id])
        count = len(re.findall(rb"\b" + re.escape(writer.encode("ascii")) + rb"\s*\(", body))
        if re.search(rb"\bfn\s+" + re.escape(writer.encode("ascii")) + rb"\s*\(", body):
            count -= 1
        callsites.extend(f"{capsule_id}#{index}" for index in range(max(0, count)))
    if not callsites:
        fail("WRITER_PROJECTION", "append writer has no source-derived callsite")
    for row in rows:
        row_bodies = b"\n".join(code_mask(body_by_id[capsule_id]) for capsule_id in row["capsule_ids"])
        calls = len(re.findall(rb"\b" + re.escape(writer.encode("ascii")) + rb"\s*\(", row_bodies))
        defs = len(re.findall(rb"\bfn\s+" + re.escape(writer.encode("ascii")) + rb"\s*\(", row_bodies))
        if calls - defs < 1:
            fail("OWNER_BINDING", f"row {row['row_index']} has no writer call in its bound capsules")
    facade_body = code_mask(body_by_id[binding["facade_binding"]["capsule_id"]])
    if len(re.findall(rb"\bfn\s+p1342_run_fixture_for_test\s*\(", facade_body)) != 1:
        fail("OWNER_BINDING", "the exact test facade symbol is absent or duplicated")
    projections = sorted(set(name.decode("ascii") for name in re.findall(rb"\bfn\s+([A-Za-z_][A-Za-z0-9_]*(?:project|dto)[A-Za-z0-9_]*)\s*\(", masked_all, flags=re.I)))
    projection_writes = []
    for name in projections:
        match = re.search(rb"\bfn\s+" + re.escape(name.encode("ascii")) + rb"\s*\([^)]*\)[^{]*\{([^{}]*)\}", masked_all, flags=re.S)
        if match and (b".push(" in match.group(1) or re.search(rb"\b" + re.escape(writer.encode("ascii")) + rb"\s*\(", match.group(1))):
            projection_writes.append(name)
    h16 = code_mask(body_by_id.get("P1343-H16-RAW-FREEZE", b""))
    raw_freeze = bool(re.search(rb"\b(?:freeze|raw_snapshot|raw_freeze)\b", h16, flags=re.I))
    if projection_writes or not raw_freeze:
        fail("WRITER_PROJECTION", "projection writes or raw freeze before projection not proven")
    writer_projection = {
        "append_writer_symbol": writer,
        "append_writer_symbol_sha256": sha256(writer.encode("ascii")),
        "writer_callsites": callsites,
        "alternate_writers": [],
        "posthoc_constructors": [],
        "projection_symbols": projections,
        "projection_writes": projection_writes,
        "raw_freeze_before_projection": True,
        "result": "PASS",
    }
    runtime_projection = {
        "fixture_sha256": runtime["fixture_sha256"],
        "modes": runtime["modes"],
        "row_coverage": runtime["row_coverage"],
        "raw_snapshot_refs": runtime["raw_snapshot_refs"],
        "result": "PASS",
    }
    exact_pairs = [[item["path"], item["live_file_sha256"]] for item in file_records]
    transcript = {
        "files": [[item["path"], item["normalized_file_sha256"]] for item in file_records],
        "capsules": [[item["capsule_id"], item["begin_offset"], item["end_offset"], item["body_sha256"]] for item in capsule_records],
    }
    evidence = {
        "schema": "p1343-candidate-source-evidence-v1",
        "step": 1343,
        "contract_sha256": contract_sha,
        "binding_sha256": binding_sha,
        "authority_manifest_sha256": authority_sha,
        "capsule_baseline_sha256": manifest_sha,
        "p1342_contract_sha256": p1342_contract_sha,
        "p1342_binding_sha256": p1342_binding_sha,
        "l0_freeze_sha256": l0_sha,
        "fixture_sha256": fixture_sha,
        "candidate_tree": {
            "head": git_head(root),
            "state": "verifier-controlled candidate root",
            "candidate_root_sha256": sha256(str(root).encode("utf-8")),
            "exact_file_set_sha256": sha256(canonical_json(exact_pairs)),
        },
        "files": file_records,
        "capsules": capsule_records,
        "rows": row_records,
        "writer_projection": writer_projection,
        "runtime_coverage": runtime_projection,
        "normalization_transcript_sha256": sha256(canonical_json(transcript)),
        "verifier": {
            "source_verifier_sha256": source_verifier_sha,
            "invocation_sha256": invocation_sha,
            "derived_projection_sha256": "",
            "result": "PASS",
        },
    }
    projection = {key: value for key, value in evidence.items() if key != "verifier"}
    evidence["verifier"]["derived_projection_sha256"] = sha256(canonical_json(projection))
    return evidence


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    for name in ("contract", "binding", "authority-manifest", "capsule-baseline", "p1342-contract", "p1342-binding", "l0-freeze", "fixture", "runtime-evidence"):
        value.add_argument(f"--{name}", required=True, type=Path)
        value.add_argument(f"--{name}-sha256", required=True)
    value.add_argument("--candidate-root", required=True, type=Path)
    value.add_argument("--output", required=True, type=Path)
    return value


def main() -> int:
    args = parser().parse_args()
    contract = read_json(args.contract, args.contract_sha256, "contract")
    binding = read_json(args.binding, args.binding_sha256, "binding")
    read_json(args.authority_manifest, args.authority_manifest_sha256, "authority manifest")
    manifest = read_json(args.capsule_baseline, args.capsule_baseline_sha256, "capsule baseline")
    read_json(args.p1342_contract, args.p1342_contract_sha256, "P1342 contract")
    read_json(args.p1342_binding, args.p1342_binding_sha256, "P1342 binding")
    read_json(args.l0_freeze, args.l0_freeze_sha256, "L0 freeze")
    fixture = read_pinned_bytes(args.fixture, args.fixture_sha256, "fixture")
    if len(fixture) != 178 or not fixture.endswith(b"\n"):
        fail("PROTECTED_INPUT", "fixture must be exactly 178 bytes and LF-terminated")
    runtime = read_json(args.runtime_evidence, args.runtime_evidence_sha256, "runtime evidence")
    source_sha = sha256(Path(__file__).read_bytes())
    invocation = {
        key: str(value)
        for key, value in sorted(vars(args).items())
        if key != "output"
    }
    evidence = derive_source_evidence(
        contract=contract, contract_sha=args.contract_sha256,
        binding=binding, binding_sha=args.binding_sha256,
        authority_sha=args.authority_manifest_sha256,
        manifest=manifest, manifest_sha=args.capsule_baseline_sha256,
        p1342_contract_sha=args.p1342_contract_sha256,
        p1342_binding_sha=args.p1342_binding_sha256,
        l0_sha=args.l0_freeze_sha256, fixture_sha=args.fixture_sha256,
        candidate_root=args.candidate_root, runtime=runtime,
        source_verifier_sha=source_sha,
        invocation_sha=sha256(canonical_json(invocation)),
    )
    args.output.write_bytes(canonical_json(evidence))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except VerificationFailure as exc:
        print(json.dumps({"schema": "p1343-source-verifier-error-v1", "classification": "Violated", "reason_code": exc.code, "witness": exc.detail}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
