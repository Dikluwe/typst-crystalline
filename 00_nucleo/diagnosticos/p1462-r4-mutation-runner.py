#!/usr/bin/env python3
"""Identity and strictly serial transport runner for P1462 R4.

This final local revision contains no private oracle expectations and assigns
no kills.  When an external verifier is supplied, the three required orders
are always executed serially: direct, direct_repeat, then reverse.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
from typing import Any


HERE = Path(__file__).resolve().parent
DEFAULT_FIXTURES = HERE / "p1462-r4-mutation-fixtures.json"
R3_RUNNER = HERE / "p1462-r3-mutation-runner.py"
R3_RUNNER_SHA256 = "e70b5823f59636e207d044c9e4cb04e2d1541f63ffc0352d4e5eb4f97ee6eafd"
R3_FIXTURES = HERE / "p1462-r3-mutation-fixtures.json"
R3_FIXTURES_SHA256 = "5f1b19055be43693c6dc97db0f57d3b6bec07991d6d8d2b39889c20c8e618c96"
REQUIRED_ORDERS = ("direct", "direct_repeat", "reverse")


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def load_r3_transport() -> Any:
    actual = sha256_file(R3_RUNNER)
    if actual != R3_RUNNER_SHA256:
        raise RuntimeError(
            f"frozen R3 runner identity mismatch: {actual} != {R3_RUNNER_SHA256}"
        )
    spec = importlib.util.spec_from_file_location("p1462_r3_transport", R3_RUNNER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen R3 runner")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def load_fixtures(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError("fixture root must be an object")
    return value


def expand_r4_overlay(
    overlay: dict[str, Any],
) -> tuple[dict[str, Any], dict[str, Any], Any]:
    actual = sha256_file(R3_FIXTURES)
    if actual != R3_FIXTURES_SHA256:
        raise RuntimeError(
            f"frozen R3 fixtures identity mismatch: {actual} != {R3_FIXTURES_SHA256}"
        )
    r3 = load_r3_transport()
    r3_overlay = r3.load_fixtures(R3_FIXTURES)
    r3_expanded = r3.expand_r3_overlay(r3_overlay)
    expanded = json.loads(json.dumps(r3_expanded))
    expanded["schema"] = overlay["schema"]
    expanded["revision"] = overlay["revision"]
    for key in (
        "authority",
        "provenance",
        "bundle",
        "build",
        "budgets",
        "ordering",
        "scoring",
        "public_focus",
    ):
        expanded[key] = overlay[key]

    reference = overlay["reference_replacement"]
    expanded["reference"]["identity"] = reference["identity"]
    expanded["reference"]["exact_delta"] = reference["exact_delta"]
    expanded["reference"]["r4_common_rebase"] = True

    replacements = {
        record["identity"]["id"]: record
        for record in overlay["variant_replacements"]
    }
    for record in expanded["mutants"]:
        variant_id = record["identity"]["id"]
        replacement = replacements[variant_id]
        record["identity"] = replacement["identity"]
        record["exact_delta"] = replacement["exact_delta"]
        record["r4_common_rebase"] = True
    return expanded, r3_expanded, r3


def shape_errors(overlay: dict[str, Any], expanded: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if overlay.get("schema") != "p1462-r4-mutation-fixtures/v1":
        errors.append("unexpected R4 fixture schema")
    if overlay.get("revision") != "R4":
        errors.append("fixture revision must be R4")
    expected = [f"M{number:02d}" for number in range(1, 19)]
    ids = [record.get("identity", {}).get("id") for record in expanded["mutants"]]
    if ids != expected:
        errors.append(f"mutant IDs/order {ids!r} != {expected!r}")
    replacement_ids = [
        record.get("identity", {}).get("id")
        for record in overlay.get("variant_replacements", [])
    ]
    if replacement_ids != expected:
        errors.append("R4 replacement identities must be exactly M01-M18 in order")
    if overlay["ordering"].get("execution_mode") != "strictly-serial":
        errors.append("R4 execution mode must be strictly-serial")
    if tuple(overlay["ordering"].get("required_order_sequence", [])) != REQUIRED_ORDERS:
        errors.append("R4 required order sequence differs from the frozen sequence")
    if overlay["budgets"].get("external_timeout_seconds", 0) < 360:
        errors.append("R4 external timeout must be at least 360 seconds per variant")
    if overlay["budgets"].get("final_local_revision") is not True:
        errors.append("R4 must remain marked as the final local revision")
    for relative, expected_hash in overlay["provenance"]["protected_files"].items():
        path = HERE.parent.parent / relative
        if not path.is_file():
            errors.append(f"missing protected file {relative}")
            continue
        actual_hash = sha256_file(path)
        if actual_hash != expected_hash:
            errors.append(
                f"protected file hash mismatch {relative}: "
                f"{actual_hash} != {expected_hash}"
            )
    return errors


def verify_r3_to_r4_rebase(
    overlay: dict[str, Any],
    expanded: dict[str, Any],
    r3_expanded: dict[str, Any],
    r2_transport: Any,
    r4_root: Path,
) -> list[str]:
    errors: list[str] = []
    r3_root = Path(overlay["r3_to_r4"]["r3_bundle_root"]).resolve()
    errors.extend(r2_transport.verify_bundle(r3_expanded, r3_root))

    grouped: dict[str, tuple[str, str]] = {}
    assigned: set[str] = set()
    for group in overlay["r3_to_r4"]["rules_hash_groups"]:
        pair = (group["before_sha256"], group["after_sha256"])
        for variant_id in group["ids"]:
            if variant_id in grouped:
                errors.append(f"duplicate R3->R4 hash group member {variant_id}")
            grouped[variant_id] = pair
            assigned.add(variant_id)
    expected_ids = {"P00", *(f"M{number:02d}" for number in range(1, 19))}
    if assigned != expected_ids:
        errors.append("R3->R4 hash groups do not cover exactly P00 plus M01-M18")

    for variant_id in sorted(expected_ids):
        old_index, _ = r2_transport.source_index(r3_root / variant_id)
        new_index, _ = r2_transport.source_index(r4_root / variant_id)
        actual = r2_transport.changed_files(old_index, new_index)
        before, after = grouped.get(variant_id, ("MISSING", "MISSING"))
        expected = {
            "01_core/src/compiler/eval/rules.rs": (before, after),
        }
        if actual != expected:
            errors.append(f"{variant_id}: R3->R4 delta is not the declared common rebase")

    for variant_id in sorted(expected_ids):
        source_root = r4_root / variant_id
        for path in source_root.rglob("*"):
            if path.is_symlink() or not path.is_file():
                continue
            if path.stat().st_nlink != 1:
                errors.append(f"{variant_id}: editable source hardlink: {path}")
                break
        binary = r4_root / "artifacts" / variant_id / "typst"
        if binary.stat().st_nlink != 1:
            errors.append(f"{variant_id}: frozen binary hardlink: {binary}")
    return errors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures", type=Path, default=DEFAULT_FIXTURES)
    parser.add_argument("--bundle-root", type=Path)
    parser.add_argument("--timeout", type=float)
    parser.add_argument("--report", type=Path)
    parser.add_argument(
        "command",
        nargs=argparse.REMAINDER,
        help=(
            "External verifier after '--'. Placeholders: {variant_id}, {binary}, "
            "{source_root}, {bundle_root}. All three orders run serially."
        ),
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    overlay = load_fixtures(args.fixtures)
    expanded, r3_expanded, r3 = expand_r4_overlay(overlay)
    transport = r3.load_r2_transport()
    bundle_root = transport.resolve_bundle_root(expanded, args.bundle_root)
    errors = shape_errors(overlay, expanded)
    errors.extend(transport.verify_bundle(expanded, bundle_root))
    errors.extend(
        verify_r3_to_r4_rebase(
            overlay, expanded, r3_expanded, transport, bundle_root
        )
    )
    if errors:
        for error in errors:
            print(f"identity-error: {error}", file=sys.stderr)
        return 2

    command = list(args.command)
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        print(
            json.dumps(
                {
                    "verified": True,
                    "revision": "R4",
                    "bundle_root": str(bundle_root),
                    "reference_count": 1,
                    "mutant_count": len(expanded["mutants"]),
                    "common_rebased_variant_count": 19,
                    "required_orders": list(REQUIRED_ORDERS),
                    "execution_mode": "strictly-serial",
                    "minimum_external_timeout_seconds": 360,
                    "valid_denominator": expanded["scoring"]["valid_denominator"],
                    "required_score": expanded["scoring"]["required_score"],
                    "final_local_revision": True,
                    "note": "Identity verification only; no oracle was executed.",
                },
                indent=2,
                sort_keys=True,
            )
        )
        return 0

    timeout = args.timeout or overlay["budgets"]["external_timeout_seconds"]
    if timeout < 360:
        print("execution-error: timeout must be >= 360 seconds per variant", file=sys.stderr)
        return 2

    started = datetime.now(timezone.utc).isoformat()
    reports: list[dict[str, Any]] = []
    status = "complete-unclassified"
    for order in REQUIRED_ORDERS:
        report = transport.run_external(
            expanded,
            bundle_root,
            order,
            timeout,
            command,
        )
        report["schema"] = "p1462-r4-serial-order-run/v1"
        report["revision"] = "R4"
        reports.append(report)
        if any(run["instrument_status"] != "completed" for run in report["runs"]):
            status = "inconclusive"
            break

    campaign = {
        "schema": "p1462-r4-serial-campaign/v1",
        "revision": "R4",
        "execution_mode": "strictly-serial",
        "required_orders": list(REQUIRED_ORDERS),
        "completed_orders": [report["order"] for report in reports],
        "timeout_seconds_per_variant": timeout,
        "started_at_utc": started,
        "finished_at_utc": datetime.now(timezone.utc).isoformat(),
        "status": status,
        "classification_authority": "external-preseal-verifier",
        "unknown_receives_credit": False,
        "final_local_revision": True,
        "failure_disposition": "inconclusive; no further local mutation revision",
        "note": "Transport only; this runner contains no private expectations and assigns no kills.",
        "orders": reports,
    }
    serialized = json.dumps(campaign, indent=2, sort_keys=True) + "\n"
    if args.report:
        args.report.write_text(serialized, encoding="utf-8")
    else:
        sys.stdout.write(serialized)
    return 3 if status == "inconclusive" else 0


if __name__ == "__main__":
    raise SystemExit(main())
