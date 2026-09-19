#!/usr/bin/env python3
"""Validate P1461 R1 mutation fixtures and invoke an opaque frozen harness.

This runner deliberately knows no oracle expectations.  It validates the
producer-owned source/binary identities recorded in the fixture manifest and
then invokes a verifier-supplied command once per variant in a frozen order.
"""

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


def load_fixtures(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as stream:
        data = json.load(stream)
    if data.get("artifact") != "p1461-mutation-fixtures":
        raise ValueError(f"unexpected fixture artifact in {path}")
    return data


def command_output(argv: list[str], cwd: Path | None = None) -> str:
    process = subprocess.run(
        argv,
        cwd=cwd,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        timeout=30,
    )
    if process.returncode != 0:
        raise ValueError(
            f"identity command failed ({process.returncode}): {argv!r}: "
            f"{process.stdout.strip()}"
        )
    return process.stdout.strip()


def validate_variant(variant: dict[str, Any], causal_commit: str) -> dict[str, Any]:
    variant_id = variant["id"]
    errors: list[str] = []
    worktree = Path(variant["worktree"])

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
            continue
        actual = sha256(path)
        if actual != source["sha256"]:
            errors.append(
                f"source hash mismatch {source['path']}: {actual} != {source['sha256']}"
            )

    if variant["materialization_status"] != "ready":
        errors.append(
            "fixture is fail-closed: "
            f"{variant['materialization_status']}: {variant['reason_code']}"
        )
        return {"id": variant_id, "status": "invalid", "errors": errors}

    binary = Path(variant["binary"]["path"])

    if not binary.is_file():
        errors.append(f"missing binary: {binary}")
    else:
        actual_binary_hash = sha256(binary)
        if actual_binary_hash != variant["binary"]["sha256"]:
            errors.append(
                f"binary hash mismatch: {actual_binary_hash} != "
                f"{variant['binary']['sha256']}"
            )
        mode = binary.stat().st_mode
        if mode & (stat.S_IWUSR | stat.S_IWGRP | stat.S_IWOTH):
            errors.append(f"binary is not immutable by mode: {oct(mode)}")
        try:
            version = command_output([str(binary), "--version"])
            if version != variant["binary"]["version_output"]:
                errors.append(
                    f"version mismatch: {version!r} != "
                    f"{variant['binary']['version_output']!r}"
                )
        except (OSError, ValueError, subprocess.TimeoutExpired) as error:
            errors.append(str(error))

    return {
        "id": variant_id,
        "status": "valid" if not errors else "invalid",
        "errors": errors,
    }


def validate_all(fixtures: dict[str, Any]) -> list[dict[str, Any]]:
    causal_commit = fixtures["baseline"]["causal_commit"]
    return [
        validate_variant(variant, causal_commit)
        for variant in fixtures["variants"]
    ]


def expand_command(
    template: list[str],
    variant: dict[str, Any],
    fixtures_path: Path,
    order: str,
    index: int,
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
    fixtures: dict[str, Any],
    fixtures_path: Path,
    order: str,
    template: list[str],
) -> int:
    by_id = {variant["id"]: variant for variant in fixtures["variants"]}
    sequence = fixtures["campaign_orders"][order]
    receipts: list[dict[str, Any]] = []
    for index, variant_id in enumerate(sequence):
        variant = by_id[variant_id]
        command = expand_command(template, variant, fixtures_path, order, index)
        environment = os.environ.copy()
        environment.update(
            {
                "P1461_FIXTURES": str(fixtures_path),
                "P1461_ORDER": order,
                "P1461_SEQUENCE_INDEX": str(index),
                "P1461_VARIANT_BINARY": variant["binary"]["path"],
                "P1461_VARIANT_ID": variant_id,
                "P1461_VARIANT_KIND": variant["kind"],
                "P1461_VARIANT_WORKTREE": variant["worktree"],
            }
        )
        try:
            process = subprocess.run(
                command,
                check=False,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                env=environment,
            )
            receipts.append(
                {
                    "command": command,
                    "exit_code": process.returncode,
                    "index": index,
                    "order": order,
                    "stderr": process.stderr,
                    "stdout": process.stdout,
                    "variant": variant_id,
                }
            )
        except OSError as error:
            receipts.append(
                {
                    "command": command,
                    "error": str(error),
                    "exit_code": None,
                    "index": index,
                    "order": order,
                    "variant": variant_id,
                }
            )
    json.dump(
        {"artifact": "p1461-mutation-run", "order": order, "receipts": receipts},
        sys.stdout,
        ensure_ascii=False,
        indent=2,
        sort_keys=True,
    )
    sys.stdout.write("\n")
    return 0


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
            sys.stdout,
            ensure_ascii=False,
            indent=2,
            sort_keys=True,
        )
        sys.stdout.write("\n")
        return 1 if invalid else 0

    if invalid:
        json.dump(
            {"artifact": "p1461-mutation-validation", "variants": validations},
            sys.stderr,
            ensure_ascii=False,
            indent=2,
            sort_keys=True,
        )
        sys.stderr.write("\n")
        return 1
    command = arguments.harness_command
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        parser.error("run requires a verifier-supplied harness command after --")
    return run_harness(fixtures, fixtures_path, arguments.order, command)


if __name__ == "__main__":
    raise SystemExit(main())
