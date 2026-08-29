#!/usr/bin/env python3
"""Materialize or apply P1268 declarative mutants without judging outcomes."""

from __future__ import annotations

import argparse
import copy
import csv
import json
import sys
from pathlib import Path
from typing import Any


class MutationError(RuntimeError):
    pass


MISSING = object()
NO_DEFAULT = object()


def canonical_json(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def parse_cell(value: str) -> Any:
    try:
        return json.loads(value)
    except json.JSONDecodeError:
        return value


def load_suite(path: Path, id_column: str) -> dict[str, dict[str, str]]:
    with path.open("r", encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle, delimiter="\t"))
    if not rows or id_column not in rows[0]:
        raise MutationError(f"{path}: missing suite column {id_column!r}")
    suite: dict[str, dict[str, str]] = {}
    for row in rows:
        case_id = row[id_column]
        if not case_id or case_id in suite:
            raise MutationError(f"{path}: empty or duplicate id {case_id!r}")
        suite[case_id] = row
    return suite


def decode_pointer(path: str) -> list[str]:
    if path == "":
        return []
    if not path.startswith("/"):
        raise MutationError(f"not an RFC 6901 pointer: {path!r}")
    return [part.replace("~1", "/").replace("~0", "~") for part in path[1:].split("/")]


def child(container: Any, token: str, path: str) -> Any:
    if isinstance(container, list):
        try:
            return container[int(token)]
        except (ValueError, IndexError) as exc:
            raise MutationError(f"invalid list index {token!r} in {path!r}") from exc
    if isinstance(container, dict) and token in container:
        return container[token]
    raise MutationError(f"missing path component {token!r} in {path!r}")


def locate_parent(root: Any, path: str) -> tuple[Any, str]:
    tokens = decode_pointer(path)
    if not tokens:
        raise MutationError("root replacement/removal is intentionally unsupported")
    current = root
    for token in tokens[:-1]:
        current = child(current, token, path)
    return current, tokens[-1]


def get_value(root: Any, path: str, default: Any = NO_DEFAULT) -> Any:
    current = root
    try:
        for token in decode_pointer(path):
            current = child(current, token, path)
        return current
    except MutationError:
        if default is not NO_DEFAULT:
            return default
        raise


def set_value(parent: Any, token: str, value: Any, *, add: bool) -> None:
    if isinstance(parent, list):
        if token == "-" and add:
            parent.append(value)
            return
        try:
            index = int(token)
        except ValueError as exc:
            raise MutationError(f"invalid list index {token!r}") from exc
        if add:
            if not 0 <= index <= len(parent):
                raise MutationError(f"add index {index} outside list")
            parent.insert(index, value)
        else:
            if not 0 <= index < len(parent):
                raise MutationError(f"replace index {index} outside list")
            parent[index] = value
        return
    if not isinstance(parent, dict):
        raise MutationError("operation parent is neither object nor array")
    if add and token in parent:
        raise MutationError(f"add target {token!r} already exists")
    if not add and token not in parent:
        raise MutationError(f"replace target {token!r} does not exist")
    parent[token] = value


def remove_value(parent: Any, token: str) -> Any:
    if isinstance(parent, list):
        try:
            return parent.pop(int(token))
        except (ValueError, IndexError) as exc:
            raise MutationError(f"invalid removal index {token!r}") from exc
    if isinstance(parent, dict) and token in parent:
        return parent.pop(token)
    raise MutationError(f"remove target {token!r} does not exist")


def apply_operations(record: Any, operations: list[dict[str, Any]]) -> Any:
    mutated = copy.deepcopy(record)
    for index, operation in enumerate(operations, start=1):
        op = operation.get("op")
        path = operation.get("path")
        if op not in {"replace", "remove", "add"} or not isinstance(path, str):
            raise MutationError(f"operation {index}: unsupported shape {operation!r}")
        parent, token = locate_parent(mutated, path)
        if op == "add":
            if get_value(mutated, path, MISSING) is not MISSING:
                raise MutationError(f"operation {index}: add precondition failed at {path}")
            set_value(parent, token, copy.deepcopy(operation.get("value")), add=True)
            continue
        actual = get_value(mutated, path)
        if "before" not in operation or actual != operation["before"]:
            raise MutationError(
                f"operation {index}: precondition failed at {path}: "
                f"expected {canonical_json(operation.get('before'))}, got {canonical_json(actual)}"
            )
        if op == "replace":
            set_value(parent, token, copy.deepcopy(operation.get("value")), add=False)
        else:
            removed = remove_value(parent, token)
            if removed != operation["before"]:
                raise MutationError(f"operation {index}: removal changed after precondition")
    return mutated


def load_records(path: Path) -> tuple[Any, str, list[str] | None]:
    if path.suffix.lower() == ".json":
        return json.loads(path.read_text(encoding="utf-8")), "json", None
    with path.open("r", encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        rows = list(reader)
        fields = reader.fieldnames
    if not fields:
        raise MutationError(f"{path}: TSV has no header")
    if fields == ["field", "value"]:
        return {row["field"]: parse_cell(row["value"]) for row in rows}, "kv-tsv", fields
    return [{key: parse_cell(value) for key, value in row.items()} for row in rows], "table-tsv", fields


def write_records(path: Path, data: Any, style: str, fields: list[str] | None) -> None:
    if path.suffix.lower() == ".json" or style == "json":
        path.write_text(canonical_json(data) + "\n", encoding="utf-8")
        return
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle, delimiter="\t", lineterminator="\n")
        if style == "kv-tsv":
            if not isinstance(data, dict):
                raise MutationError("key/value TSV root stopped being an object")
            writer.writerow(["field", "value"])
            for key in sorted(data):
                value = data[key]
                writer.writerow([key, canonical_json(value) if not isinstance(value, str) else value])
            return
        if not isinstance(data, list) or not all(isinstance(item, dict) for item in data):
            raise MutationError("table TSV root stopped being an array of objects")
        columns = list(fields or [])
        extras = sorted({key for row in data for key in row} - set(columns))
        columns.extend(extras)
        dict_writer = csv.DictWriter(handle, fieldnames=columns, delimiter="\t", lineterminator="\n")
        dict_writer.writeheader()
        for row in data:
            dict_writer.writerow({key: encode_cell(row.get(key)) for key in columns})


def encode_cell(value: Any) -> str:
    if isinstance(value, str):
        return value
    return canonical_json(value)


def selector_pairs(selector: dict[str, Any]) -> list[tuple[str, Any]]:
    pairs = []
    for key, expected in selector.items():
        pointer = key if key.startswith("/") else "/" + key.replace("~", "~0").replace("/", "~1")
        pairs.append((pointer, expected))
    return pairs


def matches(record: Any, selector: dict[str, Any]) -> bool:
    return all(get_value(record, pointer, MISSING) == expected for pointer, expected in selector_pairs(selector))


def select_record(root: Any, selector: dict[str, Any]) -> tuple[Any, int | None]:
    if isinstance(root, list):
        indices = [index for index, record in enumerate(root) if matches(record, selector)]
        if len(indices) != 1:
            raise MutationError(f"selector matched {len(indices)} records; exactly one required")
        return root[indices[0]], indices[0]
    if not matches(root, selector):
        raise MutationError("selector does not match canonical root")
    return root, None


def row_payload(row: dict[str, str], opaque: bool) -> tuple[Any, list[dict[str, Any]], Any | None]:
    if opaque:
        before = json.loads(row["canonical_state_json"])
        operations = json.loads(row["deterministic_operation_json"])
        return before, operations, None
    return json.loads(row["before_json"]), json.loads(row["operation_json"]), json.loads(row["after_json"])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite", type=Path, required=True, help="attacks.tsv or opaque-cases.tsv")
    parser.add_argument("--opaque", action="store_true", help="use opaque-cases.tsv column schema")
    parser.add_argument("--case", required=True, help="attack_id or case_id")
    subparsers = parser.add_subparsers(dest="command", required=True)
    materialize = subparsers.add_parser("materialize", help="emit the canonical before or after state")
    materialize.add_argument("--state", choices=("before", "after"), default="after")
    materialize.add_argument("--output", type=Path, required=True)
    apply_parser = subparsers.add_parser("apply", help="apply the mutant to a caller-supplied canonical JSON/TSV record")
    apply_parser.add_argument("--input", type=Path, required=True)
    apply_parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    id_column = "case_id" if args.opaque else "attack_id"
    suite = load_suite(args.suite, id_column)
    if args.case not in suite:
        raise MutationError(f"unknown case {args.case!r}")
    row = suite[args.case]
    before, operations, declared_after = row_payload(row, args.opaque)

    if args.command == "materialize":
        after = apply_operations(before, operations)
        if declared_after is not None and after != declared_after:
            raise MutationError("declared after_json does not equal deterministic operation result")
        output = before if args.state == "before" else after
        write_records(args.output, output, "json" if args.output.suffix.lower() == ".json" else "kv-tsv", None)
        return 0

    root, style, fields = load_records(args.input)
    selector = json.loads(row["selector_json"])
    selected, index = select_record(root, selector)
    mutated = apply_operations(selected, operations)
    if index is None:
        root = mutated
    else:
        root[index] = mutated
    write_records(args.output, root, style, fields)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (MutationError, OSError, json.JSONDecodeError, csv.Error) as error:
        print(f"attack_runner: {error}", file=sys.stderr)
        raise SystemExit(2)
