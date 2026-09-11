#!/usr/bin/env python3
"""Non-judging deterministic P1350 R2 execution-delivery composer."""

from __future__ import annotations

import hashlib
import json
import os
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CONTRACT_RECEIPT = ["00_nucleo/diagnosticos/p1350-contract-receipt-r2.json", "667e268009baf59cc7fe9f43df2dca55bfa69fbe2484ee06e7175d8165994d7f"]
CONTRACT_ROOT_SHA256 = "a07ef8d9f088b25aa6184ff0bbc8193c7c55151797a75da7139588fe4e2a3a7e"
LANGUAGE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
BOUNDARY_SHA256 = "d2e85d5936557936120754b647d41e88349ab19169ee4dd3fc967151bb68d8d8"
INPUT_KEYS = ["adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "boundary_attack_manifest", "checker", "focal_operation_route_map", "journal_parser", "language_id_sequence", "oracle_authorship_receipt", "oracle_caller", "supervisor"]
DELIVERY_KEYS = ["adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "authority_root_sha256", "boundary_attack_manifest", "checker", "focal_operation_route_map", "journal_parser", "language_id_sequence", "oracle_authorship_receipt", "oracle_caller", "schema", "supervisor"]


class ComposerFailure(RuntimeError):
    pass


def canonical(value: Any, lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if lf else b"")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result: raise ComposerFailure(f"duplicate key {key}")
        result[key] = value
    return result


def strict_json(raw: bytes, label: str) -> Any:
    try: value = json.loads(raw, object_pairs_hook=_duplicates, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    except ComposerFailure: raise
    except Exception as exc: raise ComposerFailure(f"{label} JSON {type(exc).__name__}") from None
    if canonical(value) != raw: raise ComposerFailure(f"{label} noncanonical")
    return value


def read_pin(pin: Any) -> tuple[str, str, bytes]:
    if type(pin) is not list or len(pin) != 2 or any(type(item) is not str for item in pin) or len(pin[1]) != 64:
        raise ComposerFailure("pin schema")
    relative, digest = pin
    path = ROOT / relative
    try: path.resolve().relative_to(ROOT.resolve())
    except ValueError: raise ComposerFailure("pin outside repository") from None
    if path.is_symlink() or not path.is_file(): raise ComposerFailure(f"pin missing {relative}")
    raw = path.read_bytes()
    if sha256(raw) != digest: raise ComposerFailure(f"pin drift {relative}")
    return relative, digest, raw


def _language(pin: Any) -> dict[str, Any]:
    relative, digest, raw = read_pin(pin); value = strict_json(raw, "language sequence")
    if type(value) is not dict or list(value) != ["closed_world", "count", "ids", "schema", "sequence_sha256"] or value["schema"] != "p1350-language-id-sequence-r2" or value["count"] != 122 or len(value["ids"]) != 122 or len(set(value["ids"])) != 122 or sha256(canonical(value["ids"], False)) != LANGUAGE_SHA256 or value["sequence_sha256"] != LANGUAGE_SHA256:
        raise ComposerFailure("language domain")
    return {"artifact": [relative, digest], "count": 122, "sequence_sha256": LANGUAGE_SHA256}


def _route(pin: Any) -> dict[str, Any]:
    relative, digest, raw = read_pin(pin); value = strict_json(raw, "route map")
    keys = ["closed_world", "entries", "inherited_count", "language_id_sequence_sha256", "local_count", "operation_ids_sha256", "schema", "total_count"]
    if type(value) is not dict or list(value) != keys or value["schema"] != "p1350-focal-operation-route-map-r2" or value["total_count"] != 60 or value["local_count"] != 12 or value["inherited_count"] != 48 or value["language_id_sequence_sha256"] != LANGUAGE_SHA256:
        raise ComposerFailure("route domain")
    ids = [row.get("operation_id") if type(row) is dict else None for row in value["entries"]]
    if len(ids) != 60 or len(set(ids)) != 60 or ids != sorted(ids, key=lambda item: item.encode("ascii")) or any(list(row) != ["operation_id", "route"] for row in value["entries"]):
        raise ComposerFailure("route entries")
    operation_sha = sha256(canonical(["p1350-focal-operation-ids-r2", ids], False))
    route_sha = sha256(canonical(["p1350-focal-operation-route-map-r2", LANGUAGE_SHA256, value["entries"]], False))
    if operation_sha != value["operation_ids_sha256"]: raise ComposerFailure("operation digest")
    return {"artifact": [relative, digest], "focal_operation_route_map_sha256": route_sha, "inherited_count": 48, "local_count": 12, "operation_ids_sha256": operation_sha, "total_count": 60}


def _boundary(pin: Any) -> dict[str, Any]:
    relative, digest, raw = read_pin(pin); value = strict_json(raw, "boundary manifest")
    counts = {"DYNAMIC_SUPERVISOR": 21, "FRESHNESS": 3, "JOURNAL": 8, "STATIC_BYPASS": 4}
    if type(value) is not dict or list(value) != ["category_counts", "closed_world", "entries", "manifest_sha256", "schema", "total_count"] or value["schema"] != "p1350-boundary-attack-manifest-r2" or value["total_count"] != 36 or value["category_counts"] != counts or sha256(canonical(["p1350-boundary-attack-manifest-r2", value["entries"]], False)) != BOUNDARY_SHA256 or value["manifest_sha256"] != BOUNDARY_SHA256:
        raise ComposerFailure("boundary domain")
    return {"artifact": [relative, digest], "category_counts": counts, "manifest_sha256": BOUNDARY_SHA256, "total_count": 36}


def compose(input_path: Path, delivery_path: Path, receipt_path: Path) -> tuple[str, str]:
    if input_path.is_symlink() or not input_path.is_file(): raise ComposerFailure("input path")
    inputs = strict_json(input_path.read_bytes(), "composer input")
    if type(inputs) is not dict or list(inputs) != INPUT_KEYS: raise ComposerFailure("composer input keys")
    read_pin(CONTRACT_RECEIPT)
    plain: dict[str, list[str]] = {}
    for key in INPUT_KEYS:
        if key not in ("boundary_attack_manifest", "focal_operation_route_map", "language_id_sequence"):
            relative, digest, _ = read_pin(inputs[key]); plain[key] = [relative, digest]
    language = _language(inputs["language_id_sequence"]); route = _route(inputs["focal_operation_route_map"]); boundary = _boundary(inputs["boundary_attack_manifest"])
    oracle_hashes = [plain[key][1] for key in ("checker", "oracle_caller", "supervisor", "journal_parser")]
    adversary_hashes = [plain["adversary"][1]]
    root_values = ["p1350-execution-authority-root-r2", CONTRACT_ROOT_SHA256, oracle_hashes, plain["oracle_authorship_receipt"][1], adversary_hashes, plain["adversary_authorship_receipt"][1], plain["adapter"][1], route["artifact"][1], plain["adapter_authorship_receipt"][1], LANGUAGE_SHA256, route["focal_operation_route_map_sha256"], BOUNDARY_SHA256]
    authority_root = sha256(canonical(root_values, False))
    delivery = {"adapter": plain["adapter"], "adapter_authorship_receipt": plain["adapter_authorship_receipt"], "adversary": plain["adversary"], "adversary_authorship_receipt": plain["adversary_authorship_receipt"], "authority_root_sha256": authority_root, "boundary_attack_manifest": boundary, "checker": plain["checker"], "focal_operation_route_map": route, "journal_parser": plain["journal_parser"], "language_id_sequence": language, "oracle_authorship_receipt": plain["oracle_authorship_receipt"], "oracle_caller": plain["oracle_caller"], "schema": "p1350-execution-delivery-receipt-r2", "supervisor": plain["supervisor"]}
    if list(delivery) != DELIVERY_KEYS: raise ComposerFailure("delivery construction")
    delivery_raw = canonical(delivery); delivery_sha = sha256(delivery_raw)
    receipt = {"closed_world": {"rule": "Non-judging composer receipt follows and pins execution delivery; execution delivery never pins this receipt."}, "composer_input_sha256": sha256(input_path.read_bytes()), "contract_receipt": CONTRACT_RECEIPT, "execution_delivery": [str(delivery_path.relative_to(ROOT)), delivery_sha], "inputs": inputs, "schema": "p1350-delivery-composer-receipt-r2"}
    for path, raw in ((delivery_path, delivery_raw), (receipt_path, canonical(receipt))):
        try: fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_CLOEXEC, 0o600)
        except OSError as exc: raise ComposerFailure(f"exclusive output {path.name}: {exc.errno}") from None
        try:
            offset = 0
            while offset < len(raw): offset += os.write(fd, raw[offset:])
            os.fsync(fd)
        finally: os.close(fd)
    return delivery_sha, sha256(canonical(receipt))


def main() -> int:
    args = sys.argv[1:]
    if len(args) != 6 or args[0] != "--input" or args[2] != "--delivery" or args[4] != "--receipt":
        sys.stderr.write("AUTHORITY_ROOT: expected --input FILE --delivery FILE --receipt FILE\n"); return 2
    try:
        delivery = (ROOT / args[3]).resolve(); receipt = (ROOT / args[5]).resolve()
        delivery.relative_to(ROOT.resolve()); receipt.relative_to(ROOT.resolve())
        dsha, rsha = compose((ROOT / args[1]).resolve(), delivery, receipt)
        sys.stdout.buffer.write(canonical({"delivery_sha256": dsha, "receipt_sha256": rsha, "schema": "p1350-delivery-composer-result-r2"})); return 0
    except Exception as exc:
        sys.stderr.write(f"AUTHORITY_ROOT: P1350 composer {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
