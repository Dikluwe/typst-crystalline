#!/usr/bin/env python3
"""Identity/transport runner for the focal P1462 R3 mutation bundle.

The R3 runner reuses only the deterministic identity and transport primitives
of the frozen R2 runner. It contains no oracle expectations and assigns no
mutation kills.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
from typing import Any


HERE = Path(__file__).resolve().parent
DEFAULT_FIXTURES = HERE / "p1462-r3-mutation-fixtures.json"
R2_RUNNER = HERE / "p1462-r2-mutation-runner.py"
R2_RUNNER_SHA256 = "3080707215bfc80afcdf644948c2aad82b0af39979621e0bdaca29c9d23b2a6e"
R2_FIXTURES = HERE / "p1462-r2-mutation-fixtures.json"
R2_FIXTURES_SHA256 = "cabf0dbadb420efc163ba1167d676455de8275fc40dca30b9508bdcf95cc08a4"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def load_r2_transport() -> Any:
    actual = sha256_file(R2_RUNNER)
    if actual != R2_RUNNER_SHA256:
        raise RuntimeError(
            f"frozen R2 runner identity mismatch: {actual} != {R2_RUNNER_SHA256}"
        )
    spec = importlib.util.spec_from_file_location("p1462_r2_transport", R2_RUNNER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load frozen R2 runner")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def load_fixtures(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError("fixture root must be an object")
    return value


def expand_r3_overlay(overlay: dict[str, Any]) -> dict[str, Any]:
    actual = sha256_file(R2_FIXTURES)
    if actual != R2_FIXTURES_SHA256:
        raise RuntimeError(
            f"frozen R2 fixtures identity mismatch: {actual} != {R2_FIXTURES_SHA256}"
        )
    base = load_fixtures(R2_FIXTURES)
    expanded = json.loads(json.dumps(base))
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

    inherited_ids = overlay["inheritance"]["mutant_ids"]
    for index, record in enumerate(expanded["mutants"]):
        variant_id = record["identity"]["id"]
        if variant_id == "M12":
            expanded["mutants"][index] = overlay["replacement"]
        else:
            if variant_id not in inherited_ids:
                raise ValueError(f"missing inherited mutant {variant_id}")
            record["inherited_byte_identical_from_r2"] = True
            record["r2_fixture_sha256"] = R2_FIXTURES_SHA256
    expanded["reference"]["inherited_byte_identical_from_r2"] = True
    expanded["baseline"]["inherited_byte_identical_from_r2"] = True
    return expanded


def r3_shape_errors(fixtures: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if fixtures.get("schema") != "p1462-r3-mutation-fixtures/v1":
        errors.append("unexpected R3 fixture schema")
    if fixtures.get("revision") != "R3":
        errors.append("fixture revision must be R3")
    records = fixtures.get("mutants", [])
    ids = [record.get("identity", {}).get("id") for record in records]
    expected = [f"M{number:02d}" for number in range(1, 19)]
    if ids != expected:
        errors.append(f"mutant IDs/order {ids!r} != {expected!r}")
    replacements = [
        record["identity"]["id"]
        for record in records
        if record.get("r3_replacement", False)
    ]
    if replacements != ["M12"]:
        errors.append(f"R3 replacement set {replacements!r} != ['M12']")
    inherited = [
        record["identity"]["id"]
        for record in records
        if record.get("inherited_byte_identical_from_r2", False)
    ]
    expected_inherited = [item for item in expected if item != "M12"]
    if inherited != expected_inherited:
        errors.append("R3 inherited set is not exactly M01-M11 plus M13-M18")
    return errors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures", type=Path, default=DEFAULT_FIXTURES)
    parser.add_argument("--bundle-root", type=Path)
    parser.add_argument(
        "--order",
        choices=("direct", "direct_repeat", "reverse"),
        default="direct",
    )
    parser.add_argument("--timeout", type=float)
    parser.add_argument("--report", type=Path)
    parser.add_argument(
        "command",
        nargs=argparse.REMAINDER,
        help=(
            "External verifier after '--'. Placeholders: {variant_id}, {binary}, "
            "{source_root}, {bundle_root}."
        ),
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    overlay = load_fixtures(args.fixtures)
    fixtures = expand_r3_overlay(overlay)
    transport = load_r2_transport()
    bundle_root = transport.resolve_bundle_root(fixtures, args.bundle_root)
    errors = r3_shape_errors(fixtures)
    errors.extend(transport.verify_bundle(fixtures, bundle_root))
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
                    "revision": "R3",
                    "bundle_root": str(bundle_root),
                    "reference_count": 1,
                    "mutant_count": len(fixtures["mutants"]),
                    "r3_replacement": "M12",
                    "inherited_byte_identical_count": 17,
                    "valid_denominator": fixtures["scoring"]["valid_denominator"],
                    "required_score": fixtures["scoring"]["required_score"],
                    "note": "Identity verification only; no oracle was executed.",
                },
                indent=2,
                sort_keys=True,
            )
        )
        return 0

    timeout = args.timeout or fixtures["budgets"]["external_timeout_seconds"]
    report = transport.run_external(
        fixtures,
        bundle_root,
        args.order,
        timeout,
        command,
    )
    report["schema"] = "p1462-r3-transport-run/v1"
    report["revision"] = "R3"
    report["r3_replacement"] = "M12"
    serialized = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.report:
        args.report.write_text(serialized, encoding="utf-8")
    else:
        sys.stdout.write(serialized)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
