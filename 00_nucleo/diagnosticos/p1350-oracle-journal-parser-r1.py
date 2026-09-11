#!/usr/bin/env python3
"""Independent P1350 causal-journal parser; never derives semantic answers."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


SCHEMA = "p1350-causal-journal-line-r1"
GENESIS = "0" * 64
ENVELOPE_KEYS = ["closed_world", "current_sha256", "line_core", "previous_sha256", "schema"]
CORE_KEYS = ["capability_sha256", "case_id", "deadline_ns", "domain", "event", "event_ordinal", "operation", "order", "pgid", "pid", "pidfd_available", "pidfd_fallback_errno", "result_kind", "run_id", "sid", "signal_result", "starttime_ticks", "wait_result", "writer_pid"]
EVENTS = ["case_start", "executor_bound", "deadline_armed", "frame_received", "term_sent", "kill_sent", "terminal_wait", "group_absent", "case_result", "run_complete"]
NEXT = {
    "case_start": {"executor_bound"},
    "executor_bound": {"deadline_armed"},
    "deadline_armed": {"frame_received", "term_sent", "terminal_wait"},
    "frame_received": {"term_sent", "terminal_wait"},
    "term_sent": {"kill_sent", "terminal_wait"},
    "kill_sent": {"terminal_wait"},
    "terminal_wait": {"group_absent"},
    "group_absent": {"case_result"},
}


class JournalFailure(RuntimeError):
    def __init__(self, detail: str):
        self.code = "JOURNAL_AUTHORITY"
        self.detail = detail
        super().__init__(f"{self.code}: {detail}")


def canonical(value: Any, lf: bool = False) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if lf else b"")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _reject_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise JournalFailure(f"duplicate key {key}")
        value[key] = item
    return value


def _strict_line(raw: bytes, ordinal: int) -> dict[str, Any]:
    if len(raw) > 4096 or not raw.endswith(b"\n"):
        raise JournalFailure(f"line {ordinal} framing")
    try:
        value = json.loads(raw, object_pairs_hook=_reject_duplicates, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    except JournalFailure:
        raise
    except Exception as exc:
        raise JournalFailure(f"line {ordinal} JSON {type(exc).__name__}") from None
    if canonical(value, True) != raw:
        raise JournalFailure(f"line {ordinal} noncanonical")
    if type(value) is not dict or list(value) != ENVELOPE_KEYS:
        raise JournalFailure(f"line {ordinal} envelope keys")
    if value["schema"] != SCHEMA:
        raise JournalFailure(f"line {ordinal} schema")
    if value["closed_world"] != {"keys": ENVELOPE_KEYS, "rule": "Parent-authored causal evidence only; no semantic classification."}:
        raise JournalFailure(f"line {ordinal} closed world")
    core = value["line_core"]
    if type(core) is not dict or list(core) != CORE_KEYS:
        raise JournalFailure(f"line {ordinal} core keys")
    return value


def parse_journal_bytes(raw: bytes, expected_writer_pid: int, expected_run_id: str, expected_cases: int | None = None) -> dict[str, Any]:
    if type(expected_writer_pid) is not int or expected_writer_pid <= 0 or type(expected_run_id) is not str or not expected_run_id:
        raise JournalFailure("parser authority arguments")
    if not raw or not raw.endswith(b"\n"):
        raise JournalFailure("empty or truncated journal")
    chunks = raw.splitlines(keepends=True)
    previous = GENESIS
    active: dict[str, Any] | None = None
    prior_event: str | None = None
    next_order = 0
    starts = results = groups = completes = 0
    final_line = GENESIS
    for ordinal, chunk in enumerate(chunks):
        envelope = _strict_line(chunk, ordinal)
        core = envelope["line_core"]
        if core["event_ordinal"] != ordinal or core["writer_pid"] != expected_writer_pid or core["run_id"] != expected_run_id:
            raise JournalFailure(f"line {ordinal} authority or ordinal")
        if core["event"] not in EVENTS:
            raise JournalFailure(f"line {ordinal} event")
        if envelope["previous_sha256"] != previous:
            raise JournalFailure(f"line {ordinal} previous hash")
        computed = sha256(canonical(["p1350-journal-line-r1", previous, core]))
        if envelope["current_sha256"] != computed:
            raise JournalFailure(f"line {ordinal} current hash")
        previous = final_line = computed
        event = core["event"]
        if event == "run_complete":
            if active is not None or prior_event != "case_result" or completes or core["order"] != next_order:
                raise JournalFailure(f"line {ordinal} premature run_complete")
            for key in ("case_id", "operation", "pgid", "pid", "sid", "starttime_ticks"):
                if core[key] is not None:
                    raise JournalFailure(f"line {ordinal} run_complete process field")
            completes += 1
            prior_event = event
            continue
        if completes:
            raise JournalFailure(f"line {ordinal} after run_complete")
        for key in ("case_id", "operation", "pgid", "pid", "sid", "starttime_ticks"):
            if core[key] is None:
                raise JournalFailure(f"line {ordinal} missing {key}")
        if event == "case_start":
            if active is not None or core["order"] != next_order:
                raise JournalFailure(f"line {ordinal} overlapping or unordered case")
            active = {"case_id": core["case_id"], "operation": core["operation"], "order": core["order"]}
            prior_event = event
            starts += 1
            continue
        if active is None or any(core[key] != active[key] for key in ("case_id", "operation", "order")):
            raise JournalFailure(f"line {ordinal} inactive case")
        if prior_event not in NEXT or event not in NEXT[prior_event]:
            raise JournalFailure(f"line {ordinal} transition {prior_event}->{event}")
        if event == "group_absent":
            groups += 1
        if event == "case_result":
            results += 1
            active = None
            next_order += 1
        prior_event = event
    if active is not None or completes != 1 or prior_event != "run_complete" or starts != results or groups != results:
        raise JournalFailure("journal not closed")
    if expected_cases is not None and (type(expected_cases) is not int or starts != expected_cases):
        raise JournalFailure("case cardinality")
    return {
        "active_case": None,
        "case_result_count": results,
        "case_start_count": starts,
        "final_line_sha256": final_line,
        "journal_sha256": sha256(raw),
        "line_count": len(chunks),
        "process_groups_absent": groups,
        "run_complete_count": completes,
        "run_id": expected_run_id,
        "writer_pid": expected_writer_pid,
    }


def main() -> int:
    args = sys.argv[1:]
    if len(args) != 8 or args[0] != "--journal" or args[2] != "--writer-pid" or args[4] != "--run-id" or args[6] != "--expected-cases":
        sys.stderr.write("JOURNAL_AUTHORITY: expected --journal PATH --writer-pid PID --run-id ID --expected-cases N\n")
        return 2
    try:
        path = Path(args[1])
        if path.is_symlink() or not path.is_file():
            raise JournalFailure("journal path")
        report = parse_journal_bytes(path.read_bytes(), int(args[3]), args[5], int(args[7]))
        sys.stdout.buffer.write(canonical(report, True))
        return 0
    except Exception as exc:
        detail = str(exc) if isinstance(exc, JournalFailure) else f"JOURNAL_AUTHORITY: {type(exc).__name__}"
        sys.stderr.write(detail + "\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
