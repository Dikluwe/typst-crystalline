#!/usr/bin/env python3
"""Validate final metadata-only P1461 R4 fixtures and invoke an opaque harness."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess
import sys
from typing import Any


DEFAULT_FIXTURES = Path(__file__).with_name("p1461-mutation-fixtures.json")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def command_output(argv: list[str], cwd: Path | None = None) -> str:
    process = subprocess.run(
        argv, cwd=cwd, check=False, stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT, text=True, timeout=30,
    )
    if process.returncode != 0:
        raise ValueError(
            f"identity command failed ({process.returncode}): {argv!r}: "
            f"{process.stdout.strip()}"
        )
    return process.stdout.strip()


def load_fixtures(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as stream:
        fixtures = json.load(stream)
    if fixtures.get("artifact") != "p1461-mutation-fixtures":
        raise ValueError(f"unexpected fixture artifact in {path}")
    if fixtures.get("campaign", {}).get("revision") != 4:
        raise ValueError(f"fixture is not R4: {path}")
    return fixtures


def source_bundle(files: list[dict[str, str]]) -> str:
    digest = hashlib.sha256()
    for source in sorted(files, key=lambda item: item["path"]):
        digest.update(source["path"].encode())
        digest.update(b"\0")
        digest.update(source["sha256"].encode())
        digest.update(b"\n")
    return digest.hexdigest()


def validate_variant(variant: dict[str, Any], causal_commit: str) -> dict[str, Any]:
    errors: list[str] = []
    worktree = Path(variant["worktree"])
    if variant.get("materialization_status") != "ready":
        errors.append(f"materialization not ready: {variant.get('materialization_status')}")
    if not worktree.is_dir():
        errors.append(f"missing worktree: {worktree}")
    else:
        try:
            head = command_output(["git", "rev-parse", "HEAD"], cwd=worktree)
            if head != causal_commit:
                errors.append(f"worktree HEAD {head} != {causal_commit}")
        except (OSError, ValueError, subprocess.TimeoutExpired) as error:
            errors.append(str(error))

    for source in variant["transformed_files"]:
        path = worktree / source["path"]
        if not path.is_file():
            errors.append(f"missing transformed source: {path}")
        elif sha256(path) != source["sha256"]:
            errors.append(f"source hash mismatch: {source['path']}")
    bundle = source_bundle(variant["transformed_files"])
    if bundle != variant["source_bundle_sha256"]:
        errors.append(f"source bundle mismatch: {bundle}")

    build = variant["build"]
    if build.get("exit_code") != 0 or build.get("status") != "success":
        errors.append("recorded build did not succeed")
    target = Path(build["target_dir"])
    if not target.is_dir():
        errors.append(f"missing exclusive target directory: {target}")
    if not build.get("target_dir_was_empty") or not build.get("target_dir_unique"):
        errors.append("exclusive empty target attestation is false")
    build_log = Path(build["log_path"])
    if not build_log.is_file():
        errors.append(f"missing build log: {build_log}")
    elif sha256(build_log) != build["log_sha256"]:
        errors.append(f"build log hash mismatch: {build_log}")

    binary = Path(variant["binary"]["path"])
    if not binary.is_file():
        errors.append(f"missing binary: {binary}")
    else:
        actual = sha256(binary)
        if actual != variant["binary"]["sha256"]:
            errors.append(f"binary hash mismatch: {actual}")
        mode = stat.S_IMODE(binary.stat().st_mode)
        if mode != 0o555:
            errors.append(f"copied binary mode is not 0555: {oct(mode)}")
        try:
            version = command_output([str(binary), "--version"])
            if version != variant["binary"]["version_output"]:
                errors.append(f"version mismatch: {version!r}")
        except (OSError, ValueError, subprocess.TimeoutExpired) as error:
            errors.append(str(error))
    return {
        "errors": errors,
        "id": variant["id"],
        "status": "valid" if not errors else "invalid",
    }


def validate_all(fixtures: dict[str, Any]) -> list[dict[str, Any]]:
    variants = fixtures["variants"]
    expected = ["P00", "E01", "E02", "E03", "E04", "E05", "E06"] + [
        f"M{index:02d}" for index in range(1, 9)
    ]
    campaign_errors: list[str] = []
    if [variant["id"] for variant in variants] != expected:
        campaign_errors.append("variant order or set differs from frozen R2 set")
    targets = [variant["build"]["target_dir"] for variant in variants]
    if len(set(targets)) != len(targets):
        campaign_errors.append("target directories are not unique")
    by_id = {variant["id"]: variant for variant in variants}
    if "P00" in by_id and "E05" in by_id:
        p00, e05 = by_id["P00"], by_id["E05"]
        if p00["source_bundle_sha256"] != e05["source_bundle_sha256"]:
            campaign_errors.append("P00 and E05 source bundles differ")
        if p00["binary"]["sha256"] == e05["binary"]["sha256"]:
            campaign_errors.append("P00 and E05 binary hashes are equal")
        observations = lambda item: (item["exit_code"], item["stdout"], item["stderr"])
        if [observations(item) for item in p00["smoke"]] != [
            observations(item) for item in e05["smoke"]
        ]:
            campaign_errors.append("P00 and E05 language smoke observations differ")
    validations = [
        validate_variant(variant, fixtures["baseline"]["causal_commit"])
        for variant in variants
    ]
    if campaign_errors:
        validations.insert(0, {"errors": campaign_errors, "id": "campaign", "status": "invalid"})
    return validations


def expand_command(
    template: list[str], variant: dict[str, Any], fixtures_path: Path,
    order: str, index: int,
) -> list[str]:
    replacements = {
        "{binary}": variant["binary"]["path"],
        "{fixtures}": str(fixtures_path),
        "{index}": str(index),
        "{order}": order,
        "{variant}": variant["id"],
        "{worktree}": variant["worktree"],
    }
    expanded: list[str] = []
    for argument in template:
        for marker, value in replacements.items():
            argument = argument.replace(marker, value)
        expanded.append(argument)
    return expanded


def run_harness(
    fixtures: dict[str, Any], fixtures_path: Path, order: str, template: list[str]
) -> int:
    by_id = {variant["id"]: variant for variant in fixtures["variants"]}
    receipts: list[dict[str, Any]] = []
    failed = False
    for index, variant_id in enumerate(fixtures["campaign_orders"][order]):
        variant = by_id[variant_id]
        command = expand_command(template, variant, fixtures_path, order, index)
        environment = os.environ.copy()
        environment.update({
            "P1461_FIXTURES": str(fixtures_path),
            "P1461_ORDER": order,
            "P1461_SEQUENCE_INDEX": str(index),
            "P1461_VARIANT_BINARY": variant["binary"]["path"],
            "P1461_VARIANT_ID": variant_id,
            "P1461_VARIANT_KIND": variant["kind"],
            "P1461_VARIANT_WORKTREE": variant["worktree"],
        })
        try:
            process = subprocess.run(
                command, check=False, stdout=subprocess.PIPE,
                stderr=subprocess.PIPE, text=True, env=environment,
            )
            failed = failed or process.returncode != 0
            receipts.append({
                "command": command, "exit_code": process.returncode,
                "index": index, "order": order, "stderr": process.stderr,
                "stdout": process.stdout, "variant": variant_id,
            })
        except OSError as error:
            failed = True
            receipts.append({
                "command": command, "error": str(error), "exit_code": None,
                "index": index, "order": order, "variant": variant_id,
            })
    json.dump(
        {"artifact": "p1461-mutation-run", "order": order, "receipts": receipts},
        sys.stdout, ensure_ascii=False, indent=2, sort_keys=True,
    )
    sys.stdout.write("\n")
    return 1 if failed else 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, default=DEFAULT_FIXTURES)
    subparsers = parser.add_subparsers(dest="action", required=True)
    subparsers.add_parser("validate")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument(
        "--order", choices=("direct", "direct_repeat", "reverse"), required=True
    )
    run_parser.add_argument("harness_command", nargs=argparse.REMAINDER)
    arguments = parser.parse_args()
    fixtures_path = arguments.fixtures.resolve()
    fixtures = load_fixtures(fixtures_path)
    validations = validate_all(fixtures)
    invalid = [item for item in validations if item["status"] != "valid"]
    if arguments.action == "validate":
        json.dump(
            {"artifact": "p1461-mutation-validation", "variants": validations},
            sys.stdout, ensure_ascii=False, indent=2, sort_keys=True,
        )
        sys.stdout.write("\n")
        return 1 if invalid else 0
    if invalid:
        json.dump(
            {"artifact": "p1461-mutation-validation", "variants": validations},
            sys.stderr, ensure_ascii=False, indent=2, sort_keys=True,
        )
        sys.stderr.write("\n")
        return 1
    command = arguments.harness_command
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        parser.error("run requires a verifier-owned harness command after --")
    return run_harness(fixtures, fixtures_path, arguments.order, command)


if __name__ == "__main__":
    raise SystemExit(main())
