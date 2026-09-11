#!/usr/bin/env python3
"""Non-judging P1350 R3 composer for the Contract R2 execution delivery."""

from __future__ import annotations

import hashlib
import importlib.util
import os
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
BASE_REL = "00_nucleo/diagnosticos/p1350-oracle-delivery-composer-r2.py"
BASE_SHA256 = "4a4969f3da9d7851affc9ae72ab5694964975ff83e99148ce0dfd3d022d0abd2"
CONTRACT_RECEIPT = ["00_nucleo/diagnosticos/p1350-contract-receipt-r2.json", "667e268009baf59cc7fe9f43df2dca55bfa69fbe2484ee06e7175d8165994d7f"]
CONTRACT_ROOT_SHA256 = "a07ef8d9f088b25aa6184ff0bbc8193c7c55151797a75da7139588fe4e2a3a7e"
LANGUAGE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
BOUNDARY_SHA256 = "d2e85d5936557936120754b647d41e88349ab19169ee4dd3fc967151bb68d8d8"
INPUT_KEYS = ["adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "boundary_attack_manifest", "checker", "focal_operation_route_map", "journal_parser", "language_id_sequence", "oracle_authorship_receipt", "oracle_caller", "supervisor"]
DELIVERY_KEYS = ["adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "authority_root_sha256", "boundary_attack_manifest", "checker", "focal_operation_route_map", "journal_parser", "language_id_sequence", "oracle_authorship_receipt", "oracle_caller", "schema", "supervisor"]


def _base() -> Any:
    path = ROOT / BASE_REL; raw = path.read_bytes()
    if path.is_symlink() or hashlib.sha256(raw).hexdigest() != BASE_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: P1350 R2 composer drift")
    spec = importlib.util.spec_from_file_location("p1350_composer_r2_for_r3", path)
    if spec is None or spec.loader is None: raise RuntimeError("AUTHORITY_ROOT: P1350 R2 composer loader")
    module = importlib.util.module_from_spec(spec); sys.modules[spec.name] = module; spec.loader.exec_module(module); return module


BASE = _base()
ComposerFailure = BASE.ComposerFailure
canonical = BASE.canonical
sha256 = BASE.sha256
strict_json = BASE.strict_json
read_pin = BASE.read_pin


def _write_new(path: Path, raw: bytes) -> None:
    try: fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_CLOEXEC, 0o600)
    except OSError as exc: raise ComposerFailure(f"exclusive output {path.name}: {exc.errno}") from None
    try:
        offset = 0
        while offset < len(raw): offset += os.write(fd, raw[offset:])
        os.fsync(fd)
    finally: os.close(fd)


def compose(input_path: Path, delivery_path: Path, receipt_path: Path) -> tuple[str, str]:
    if input_path.is_symlink() or not input_path.is_file(): raise ComposerFailure("input path")
    inputs = strict_json(input_path.read_bytes(), "composer input")
    if type(inputs) is not dict or list(inputs) != INPUT_KEYS: raise ComposerFailure("composer input keys")
    read_pin(CONTRACT_RECEIPT)
    plain: dict[str, list[str]] = {}
    for key in INPUT_KEYS:
        if key not in ("boundary_attack_manifest", "focal_operation_route_map", "language_id_sequence"):
            relative, digest, _ = read_pin(inputs[key]); plain[key] = [relative, digest]
    language = BASE._language(inputs["language_id_sequence"])
    route = BASE._route(inputs["focal_operation_route_map"])
    boundary = BASE._boundary(inputs["boundary_attack_manifest"])
    oracle_hashes = [plain[key][1] for key in ("checker", "oracle_caller", "supervisor", "journal_parser")]
    root_values = ["p1350-execution-authority-root-r2", CONTRACT_ROOT_SHA256, oracle_hashes, plain["oracle_authorship_receipt"][1], [plain["adversary"][1]], plain["adversary_authorship_receipt"][1], plain["adapter"][1], route["artifact"][1], plain["adapter_authorship_receipt"][1], LANGUAGE_SHA256, route["focal_operation_route_map_sha256"], BOUNDARY_SHA256]
    authority_root = sha256(canonical(root_values, False))
    delivery = {"adapter": plain["adapter"], "adapter_authorship_receipt": plain["adapter_authorship_receipt"], "adversary": plain["adversary"], "adversary_authorship_receipt": plain["adversary_authorship_receipt"], "authority_root_sha256": authority_root, "boundary_attack_manifest": boundary, "checker": plain["checker"], "focal_operation_route_map": route, "journal_parser": plain["journal_parser"], "language_id_sequence": language, "oracle_authorship_receipt": plain["oracle_authorship_receipt"], "oracle_caller": plain["oracle_caller"], "schema": "p1350-execution-delivery-receipt-r2", "supervisor": plain["supervisor"]}
    if list(delivery) != DELIVERY_KEYS: raise ComposerFailure("delivery construction")
    delivery_raw = canonical(delivery); delivery_sha = sha256(delivery_raw)
    receipt = {"closed_world": {"rule": "Non-judging R3 composer receipt follows and pins Contract R2 execution delivery; delivery never pins this receipt."}, "composer_input_sha256": sha256(input_path.read_bytes()), "contract_receipt": CONTRACT_RECEIPT, "execution_delivery": [str(delivery_path.relative_to(ROOT)), delivery_sha], "inputs": inputs, "oracle_revision": 3, "schema": "p1350-delivery-composer-receipt-r3"}
    receipt_raw = canonical(receipt)
    _write_new(delivery_path, delivery_raw); _write_new(receipt_path, receipt_raw)
    return delivery_sha, sha256(receipt_raw)


def main() -> int:
    args = sys.argv[1:]
    if len(args) != 6 or args[0] != "--input" or args[2] != "--delivery" or args[4] != "--receipt":
        sys.stderr.write("AUTHORITY_ROOT: expected --input FILE --delivery FILE --receipt FILE\n"); return 2
    try:
        delivery = (ROOT / args[3]).resolve(); receipt = (ROOT / args[5]).resolve()
        delivery.relative_to(ROOT.resolve()); receipt.relative_to(ROOT.resolve())
        dsha, rsha = compose((ROOT / args[1]).resolve(), delivery, receipt)
        sys.stdout.buffer.write(canonical({"delivery_sha256": dsha, "receipt_sha256": rsha, "schema": "p1350-delivery-composer-result-r3"})); return 0
    except Exception as exc:
        sys.stderr.write(f"AUTHORITY_ROOT: P1350 R3 composer {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
