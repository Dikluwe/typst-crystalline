#!/usr/bin/env python3
"""Transport runner for the frozen P1462 R2 mutation bundle.

This runner verifies artifact/source identity and executes an external verifier
in a declared order.  It intentionally contains no private expected values and
never classifies a mutant as killed or preserved.
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
import time
from datetime import datetime, timezone
from typing import Any


HERE = Path(__file__).resolve().parent
DEFAULT_FIXTURES = HERE / "p1462-r2-mutation-fixtures.json"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def source_index(root: Path) -> tuple[dict[str, str], str]:
    files: dict[str, str] = {}
    for path in root.rglob("*"):
        if path.is_symlink() or not path.is_file():
            continue
        relative = path.relative_to(root).as_posix()
        files[relative] = sha256_file(path)

    aggregate = hashlib.sha256()
    for relative in sorted(files):
        aggregate.update(f"{files[relative]}  ./{relative}\n".encode("utf-8"))
    return files, aggregate.hexdigest()


def changed_files(before: dict[str, str], after: dict[str, str]) -> dict[str, tuple[str, str]]:
    changed: dict[str, tuple[str, str]] = {}
    for relative in sorted(before.keys() | after.keys()):
        old = before.get(relative, "ABSENT")
        new = after.get(relative, "ABSENT")
        if old != new:
            changed[relative] = (old, new)
    return changed


def load_fixtures(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise ValueError("fixture root must be an object")
    return value


def resolve_bundle_root(fixtures: dict[str, Any], override: Path | None) -> Path:
    if override is not None:
        return override.resolve()
    return Path(fixtures["bundle"]["root"]).resolve()


def verify_record(
    record: dict[str, Any],
    bundle_root: Path,
    base_index: dict[str, str],
    cache: dict[Path, tuple[dict[str, str], str]],
) -> list[str]:
    errors: list[str] = []
    identity = record["identity"]
    variant_id = identity["id"]
    source_root = bundle_root / identity["source_root"]
    binary_path = bundle_root / identity["binary"]

    if not source_root.is_dir():
        errors.append(f"{variant_id}: missing source root {source_root}")
        return errors
    if not binary_path.is_file():
        errors.append(f"{variant_id}: missing binary {binary_path}")
        return errors

    if source_root not in cache:
        cache[source_root] = source_index(source_root)
    current_index, tree_hash = cache[source_root]
    if tree_hash != identity["source_tree_sha256"]:
        errors.append(
            f"{variant_id}: source tree SHA-256 {tree_hash} != "
            f"{identity['source_tree_sha256']}"
        )

    actual_binary_hash = sha256_file(binary_path)
    if actual_binary_hash != identity["binary_sha256"]:
        errors.append(
            f"{variant_id}: binary SHA-256 {actual_binary_hash} != "
            f"{identity['binary_sha256']}"
        )
    actual_size = binary_path.stat().st_size
    if actual_size != identity["binary_size_bytes"]:
        errors.append(
            f"{variant_id}: binary size {actual_size} != "
            f"{identity['binary_size_bytes']}"
        )
    actual_mode = stat.S_IMODE(binary_path.stat().st_mode)
    expected_mode = int(identity["binary_mode"], 8)
    if actual_mode != expected_mode:
        errors.append(
            f"{variant_id}: binary mode {actual_mode:04o} != {expected_mode:04o}"
        )

    declared = {
        item["path"]: (item["before_sha256"], item["after_sha256"])
        for item in record["exact_delta"]
    }
    actual = changed_files(base_index, current_index)
    if actual != declared:
        errors.append(f"{variant_id}: exact delta differs from fixture declaration")
        missing = sorted(declared.keys() - actual.keys())
        extra = sorted(actual.keys() - declared.keys())
        wrong = sorted(
            key for key in declared.keys() & actual.keys() if declared[key] != actual[key]
        )
        if missing:
            errors.append(f"{variant_id}: missing declared changes: {missing}")
        if extra:
            errors.append(f"{variant_id}: undeclared changes: {extra}")
        if wrong:
            errors.append(f"{variant_id}: changed hashes disagree: {wrong}")
    return errors


def verify_bundle(fixtures: dict[str, Any], bundle_root: Path) -> list[str]:
    errors: list[str] = []
    expected_ids = [f"M{number:02d}" for number in range(1, 19)]
    records = fixtures.get("mutants", [])
    ids = [record.get("identity", {}).get("id") for record in records]
    if ids != expected_ids:
        errors.append(f"mutant IDs/order {ids!r} != {expected_ids!r}")
    if fixtures["scoring"]["valid_denominator"] != 18:
        errors.append("valid denominator must be 18")
    if fixtures["scoring"]["required_score"] != 1.0:
        errors.append("required score must be 1.0")
    if fixtures["scoring"]["unknown_receives_credit"] is not False:
        errors.append("Unknown must not receive credit")

    baseline_root = bundle_root / fixtures["baseline"]["source_root"]
    if not baseline_root.is_dir():
        return errors + [f"missing baseline source root {baseline_root}"]
    cache: dict[Path, tuple[dict[str, str], str]] = {}
    baseline_index, baseline_tree_hash = source_index(baseline_root)
    cache[baseline_root] = (baseline_index, baseline_tree_hash)
    if baseline_tree_hash != fixtures["baseline"]["source_tree_sha256"]:
        errors.append(
            f"baseline source tree SHA-256 {baseline_tree_hash} != "
            f"{fixtures['baseline']['source_tree_sha256']}"
        )

    reference = fixtures["reference"]
    errors.extend(verify_record(reference, bundle_root, baseline_index, cache))
    reference_root = bundle_root / reference["identity"]["source_root"]
    reference_index = cache[reference_root][0]
    for record in records:
        errors.extend(verify_record(record, bundle_root, reference_index, cache))
    return errors


def ordered_records(fixtures: dict[str, Any], order: str) -> list[dict[str, Any]]:
    reference = fixtures["reference"]
    mutants = list(fixtures["mutants"])
    direct = [reference, *mutants]
    if order == "direct":
        return direct
    if order == "direct_repeat":
        return [*direct, *direct]
    if order == "reverse":
        return [reference, *reversed(mutants)]
    raise ValueError(f"unsupported order: {order}")


def expand_command(template: list[str], context: dict[str, str]) -> list[str]:
    return [argument.format_map(context) for argument in template]


def run_external(
    fixtures: dict[str, Any],
    bundle_root: Path,
    order: str,
    timeout: float,
    command_template: list[str],
) -> dict[str, Any]:
    runs: list[dict[str, Any]] = []
    for index, record in enumerate(ordered_records(fixtures, order), start=1):
        identity = record["identity"]
        context = {
            "variant_id": identity["id"],
            "binary": str(bundle_root / identity["binary"]),
            "source_root": str(bundle_root / identity["source_root"]),
            "bundle_root": str(bundle_root),
        }
        command = expand_command(command_template, context)
        environment = os.environ.copy()
        environment.update(
            {
                "P1462_R2_VARIANT_ID": context["variant_id"],
                "P1462_R2_BINARY": context["binary"],
                "P1462_R2_SOURCE_ROOT": context["source_root"],
                "P1462_R2_BUNDLE_ROOT": context["bundle_root"],
            }
        )
        started = datetime.now(timezone.utc).isoformat()
        monotonic_start = time.monotonic()
        result: dict[str, Any] = {
            "invocation": index,
            "variant_id": context["variant_id"],
            "command": command,
            "started_at_utc": started,
            "classification": "unclassified",
            "credit": None,
        }
        try:
            completed = subprocess.run(
                command,
                env=environment,
                capture_output=True,
                check=False,
                timeout=timeout,
            )
            result.update(
                {
                    "instrument_status": "completed",
                    "exit_code": completed.returncode,
                    "stdout_sha256": hashlib.sha256(completed.stdout).hexdigest(),
                    "stderr_sha256": hashlib.sha256(completed.stderr).hexdigest(),
                    "stdout": completed.stdout.decode("utf-8", errors="replace"),
                    "stderr": completed.stderr.decode("utf-8", errors="replace"),
                }
            )
        except subprocess.TimeoutExpired as error:
            stdout = error.stdout or b""
            stderr = error.stderr or b""
            result.update(
                {
                    "instrument_status": "unknown-timeout",
                    "exit_code": None,
                    "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
                    "stderr_sha256": hashlib.sha256(stderr).hexdigest(),
                    "stdout": stdout.decode("utf-8", errors="replace"),
                    "stderr": stderr.decode("utf-8", errors="replace"),
                    "credit": False,
                }
            )
        except OSError as error:
            result.update(
                {
                    "instrument_status": "unknown-launch-error",
                    "exit_code": None,
                    "stdout_sha256": hashlib.sha256(b"").hexdigest(),
                    "stderr_sha256": hashlib.sha256(str(error).encode()).hexdigest(),
                    "stdout": "",
                    "stderr": str(error),
                    "credit": False,
                }
            )
        result["duration_ms"] = round((time.monotonic() - monotonic_start) * 1000, 3)
        runs.append(result)

    repeat_consistent: bool | None = None
    if order == "direct_repeat":
        half = len(runs) // 2
        first = runs[:half]
        second = runs[half:]
        repeat_consistent = all(
            (
                left["variant_id"],
                left.get("instrument_status"),
                left.get("exit_code"),
                left.get("stdout_sha256"),
                left.get("stderr_sha256"),
            )
            == (
                right["variant_id"],
                right.get("instrument_status"),
                right.get("exit_code"),
                right.get("stdout_sha256"),
                right.get("stderr_sha256"),
            )
            for left, right in zip(first, second)
        )
    return {
        "schema": "p1462-r2-transport-run/v1",
        "order": order,
        "timeout_seconds": timeout,
        "repeat_consistent": repeat_consistent,
        "classification_authority": "external-preseal-verifier",
        "note": "Transport evidence only; this runner assigns no kills.",
        "runs": runs,
    }


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
    fixtures = load_fixtures(args.fixtures)
    bundle_root = resolve_bundle_root(fixtures, args.bundle_root)
    errors = verify_bundle(fixtures, bundle_root)
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
                    "bundle_root": str(bundle_root),
                    "reference_count": 1,
                    "mutant_count": len(fixtures["mutants"]),
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
    report = run_external(fixtures, bundle_root, args.order, timeout, command)
    serialized = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.report:
        args.report.write_text(serialized, encoding="utf-8")
    else:
        sys.stdout.write(serialized)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
