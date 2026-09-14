#!/usr/bin/env python3
"""Oraculo CLI segregado P1399 para a morfologia de ContextBlock.

Emite um recibo JSON canonico em stdout. Nao le implementacao produtiva, nao
constroi o projeto e nao adapta expectativas ao binario observado.
"""

from __future__ import annotations

import argparse
import base64
import datetime as dt
import hashlib
import json
import os
import resource
import selectors
import signal
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
RLIMIT_AS_BYTES = 512 * 1024 * 1024
PROCESS_TIMEOUT_SECONDS = 10.0
BATCH_TIMEOUT_SECONDS = 60.0
STREAM_LIMIT_BYTES = 64 * 1024
EXPECTED_HEAD = "bdec21bf935cbf7f61070756cd0f218e441b7e07"

INPUTS = {
    "00_nucleo/prompts/compiler/eval/repr.md":
        "6b3633fce169a0e99d91ee542d53340b73d8d87ec1fcd877405696eb56f6f073",
    "00_nucleo/materialization/typst-passo-1399.md":
        "146ef5d573ac42f3c62089ecad5cce2ac698edb7d2812fbe32a0632a304b04c3",
    "00_nucleo/diagnosticos/p1399-manifest.json":
        "cf832bbd4a0b149aa409423c27174265ab1173951d95a0497d4740917dae2851",
    "00_nucleo/diagnosticos/p1399-contract.json":
        "14139f75129f2a8445ef90fa2e50f317ba67f4c4bd6ede7a6eca733249b20431",
}

BINARIES = (
    {
        "label": "crystalline-baseline",
        "path": "target/release/typst",
        "sha256": "3ff85cea7a721adcc9be76807e04d8aeeb03e8290d60f796b850632575b7b2b4",
        "expected_vector": {
            "context-text-x": "Violated",
            "context-text-different": "Violated",
            "context-code": "Violated",
            "context-access": "Violated",
            "control-function": "Preserved",
            "control-content": "Preserved",
        },
        "expected_observed": {
            "context-text-x": "context(...)",
            "context-text-different": "context(...)",
            "context-code": "context(...)",
            "context-access": "context(...)",
            "control-function": "(..) => ..",
            "control-content": "[x]",
        },
    },
    {
        "label": "vanila-ratificado",
        "path": "/usr/local/bin/typst",
        "sha256": "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f",
        "revision": "586e1bd43fae6c9a973218163d3165c53ab8d16d",
        "expected_vector": {
            "context-text-x": "Preserved",
            "context-text-different": "Preserved",
            "context-code": "Preserved",
            "context-access": "Preserved",
            "control-function": "Preserved",
            "control-content": "Preserved",
        },
        "expected_observed": {
            "context-text-x": "context()",
            "context-text-different": "context()",
            "context-code": "context()",
            "context-access": "context()",
            "control-function": "(..) => ..",
            "control-content": "[x]",
        },
    },
)

CASES = (
    ("context-text-x", "repr(context [x])", "context()"),
    ("context-text-different", "repr(context [different])", "context()"),
    ("context-code", "repr(context {1 + 2})", "context()"),
    ("context-access", "repr(context text.lang)", "context()"),
    ("control-function", "repr((x) => x + 1)", "(..) => .."),
    ("control-content", "repr([x])", "[x]"),
)


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="microseconds")


def codepoints(text: str) -> list[str]:
    return [f"U+{ord(char):04X}" for char in text]


def swap_snapshot() -> dict[str, int]:
    values: dict[str, int] = {}
    with Path("/proc/meminfo").open("r", encoding="ascii") as stream:
        for line in stream:
            key, _, rest = line.partition(":")
            if key in {"SwapTotal", "SwapFree"}:
                values[key] = int(rest.strip().split()[0])
    total = values.get("SwapTotal", -1)
    free = values.get("SwapFree", -1)
    return {
        "total_kib": total,
        "free_kib": free,
        "used_kib": total - free if total >= 0 and free >= 0 else -1,
    }


def child_limits() -> None:
    os.setsid()
    resource.setrlimit(resource.RLIMIT_AS, (RLIMIT_AS_BYTES, RLIMIT_AS_BYTES))


def run_capped(argv: list[str], batch_started: float) -> dict[str, Any]:
    started_at = utc_now()
    started = time.monotonic()
    result: dict[str, Any] = {
        "instant": started_at,
        "argv": argv,
        "cwd": str(ROOT),
        "rlimit_as_kib": RLIMIT_AS_BYTES // 1024,
        "timeout_seconds": PROCESS_TIMEOUT_SECONDS,
    }
    try:
        process = subprocess.Popen(
            argv,
            cwd=ROOT,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            close_fds=True,
            preexec_fn=child_limits,
        )
    except (OSError, subprocess.SubprocessError) as exc:
        result.update({
            "exit": None,
            "duration_seconds": round(time.monotonic() - started, 6),
            "max_rss_kib": None,
            "stdout": b"",
            "stderr": str(exc).encode("utf-8", "backslashreplace"),
            "reason_code": "spawn-failure",
        })
        return result

    assert process.stdout is not None and process.stderr is not None
    streams = {process.stdout.fileno(): ("stdout", bytearray()),
               process.stderr.fileno(): ("stderr", bytearray())}
    selector = selectors.DefaultSelector()
    for fd in streams:
        os.set_blocking(fd, False)
        selector.register(fd, selectors.EVENT_READ)

    status = None
    usage = None
    reason = None
    while status is None or selector.get_map():
        elapsed = time.monotonic() - started
        batch_elapsed = time.monotonic() - batch_started
        if reason is None and elapsed > PROCESS_TIMEOUT_SECONDS:
            reason = "per-process-timeout"
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
        if reason is None and batch_elapsed > BATCH_TIMEOUT_SECONDS:
            reason = "external-batch-timeout"
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass

        for key, _ in selector.select(0.025):
            fd = key.fd
            name, buffer = streams[fd]
            try:
                chunk = os.read(fd, 8192)
            except BlockingIOError:
                continue
            if not chunk:
                selector.unregister(fd)
                continue
            buffer.extend(chunk)
            if len(buffer) > STREAM_LIMIT_BYTES and reason is None:
                reason = f"{name}-limit-exceeded"
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass

        if status is None:
            waited, wait_status, wait_usage = os.wait4(process.pid, os.WNOHANG)
            if waited == process.pid:
                status = wait_status
                usage = wait_usage

    if status is None:
        _, status, usage = os.wait4(process.pid, 0)
    process.returncode = os.waitstatus_to_exitcode(status)
    stdout = bytes(streams[process.stdout.fileno()][1])
    stderr = bytes(streams[process.stderr.fileno()][1])
    process.stdout.close()
    process.stderr.close()
    result.update({
        "exit": process.returncode,
        "duration_seconds": round(time.monotonic() - started, 6),
        "max_rss_kib": int(usage.ru_maxrss) if usage is not None else None,
        "stdout": stdout,
        "stderr": stderr,
        "stdout_bytes": len(stdout),
        "stderr_bytes": len(stderr),
        "reason_code": reason,
    })
    return result


def public_stream_fields(raw: bytes, prefix: str) -> dict[str, Any]:
    fields: dict[str, Any] = {
        f"{prefix}_bytes": len(raw),
        f"{prefix}_hex": raw.hex(),
        f"{prefix}_base64": base64.b64encode(raw).decode("ascii"),
    }
    try:
        decoded = raw.decode("utf-8", "strict")
    except UnicodeDecodeError:
        fields[f"{prefix}_utf8"] = None
        fields[f"{prefix}_codepoints"] = None
    else:
        fields[f"{prefix}_utf8"] = decoded
        fields[f"{prefix}_codepoints"] = codepoints(decoded)
    return fields


def classify_probe(run: dict[str, Any], expected: str) -> tuple[str, str | None]:
    if run.get("reason_code") is not None:
        return "Unknown", str(run["reason_code"])
    if run.get("exit") != 0:
        return "Unknown", "unexpected-exit"
    stdout = run["stdout"]
    stderr = run["stderr"]
    if not stdout:
        return "Unknown", "missing-output"
    if stderr:
        return "Unknown", "unexpected-stderr"
    try:
        observed = stdout.decode("utf-8", "strict")
    except UnicodeDecodeError:
        return "Unknown", "unreadable-stdout"
    if observed == expected:
        return "Preserved", None
    return "Violated", "exact-bytes-mismatch"


def identity(binary: Path, expected: str) -> dict[str, Any]:
    observed = sha256_path(binary) if binary.is_file() else None
    ok = observed == expected
    return {
        "path": str(binary),
        "expected_sha256": expected,
        "observed_sha256": observed,
        "classification": "Preserved" if ok else "Unknown",
        "reason_code": None if ok else "ambiguous-binary-identity",
    }


def identity_only(args: argparse.Namespace) -> int:
    resource.setrlimit(resource.RLIMIT_AS, (RLIMIT_AS_BYTES, RLIMIT_AS_BYTES))
    result = identity(Path(args.binary).resolve(), args.expected_sha256)
    print(json.dumps(result, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
    return 0 if result["classification"] == "Preserved" else 2


def repository_state(batch_started: float) -> dict[str, Any]:
    head_run = run_capped(["git", "rev-parse", "HEAD"], batch_started)
    status_run = run_capped(
        ["git", "-c", "core.preloadIndex=false", "status", "--short"],
        batch_started,
    )
    head = head_run["stdout"].decode("ascii", "strict").strip() if head_run.get("exit") == 0 else None
    status = status_run["stdout"].decode("utf-8", "strict") if status_run.get("exit") == 0 else None
    return {
        "causal_base_head": EXPECTED_HEAD,
        "observed_head": head,
        "head_matches": head == EXPECTED_HEAD,
        "working_tree_status_short": status,
        "git_diff_head_stat": "not-collected: oracle role forbids reading productive diffs",
        "head_probe": {
            "exit": head_run.get("exit"),
            "duration_seconds": head_run.get("duration_seconds"),
            "max_rss_kib": head_run.get("max_rss_kib"),
        },
        "status_probe": {
            "exit": status_run.get("exit"),
            "duration_seconds": status_run.get("duration_seconds"),
            "max_rss_kib": status_run.get("max_rss_kib"),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--identity-only", action="store_true")
    parser.add_argument("--binary")
    parser.add_argument("--expected-sha256")
    args = parser.parse_args()
    if args.identity_only:
        if not args.binary or not args.expected_sha256:
            parser.error("--identity-only exige --binary e --expected-sha256")
        return identity_only(args)

    resource.setrlimit(resource.RLIMIT_AS, (RLIMIT_AS_BYTES, RLIMIT_AS_BYTES))
    batch_started = time.monotonic()
    receipt: dict[str, Any] = {
        "schema": "tekt-oracle-receipt/v1",
        "protocol_version": "tekt-materializacao-segregada/1",
        "step": "P1399",
        "status": "oracle-calibration",
        "regime": "full",
        "attestation": "executado sem atestação de isolamento",
        "role": "oracle-author",
        "executor": "/root/oraculos_p1399",
        "started_at": utc_now(),
        "oracle_artifact": {
            "path": "lab/parity/p1399_context_repr.py",
            "sha256": sha256_path(Path(__file__).resolve()),
        },
        "limits": {
            "parallelism": 1,
            "coordinator_rlimit_as_kib": RLIMIT_AS_BYTES // 1024,
            "child_rlimit_as_kib": RLIMIT_AS_BYTES // 1024,
            "per_process_timeout_seconds": PROCESS_TIMEOUT_SECONDS,
            "external_batch_timeout_seconds": BATCH_TIMEOUT_SECONDS,
            "max_bytes_per_stream": STREAM_LIMIT_BYTES,
        },
        "capabilities": {
            "read_allowlist": ["AGENTS.md", "skill and direct references", *INPUTS.keys()],
            "write_allowlist": [
                "lab/parity/p1399_context_repr.py",
                "00_nucleo/diagnosticos/p1399-oracles.json",
            ],
            "forbidden": [
                "01_core/src/compiler/eval/repr.rs",
                "productive git diff",
                "candidate implementation",
                "candidate-adapted expectations",
                "seal",
                "verdict",
            ],
            "context_inherited": "Role assignment, frozen hashes, required cases, limits, baseline/vanila expectations, and quarantine prohibition.",
        },
        "frozen_inputs": {},
        "repository": {},
        "swap": {"before": swap_snapshot()},
        "binaries": [],
        "invalid_hash_attack": {},
    }

    preflight_unknowns: list[str] = []
    for relative, expected_hash in INPUTS.items():
        path = ROOT / relative
        observed_hash = sha256_path(path) if path.is_file() else None
        state = {
            "expected_sha256": expected_hash,
            "observed_sha256": observed_hash,
            "classification": "Preserved" if observed_hash == expected_hash else "Unknown",
        }
        receipt["frozen_inputs"][relative] = state
        if state["classification"] == "Unknown":
            preflight_unknowns.append(f"input-identity:{relative}")

    receipt["repository"] = repository_state(batch_started)
    if not receipt["repository"]["head_matches"]:
        preflight_unknowns.append("head-identity")
    if receipt["repository"]["working_tree_status_short"] is None:
        preflight_unknowns.append("working-tree-status-unavailable")
    if receipt["swap"]["before"]["used_kib"] != 0:
        preflight_unknowns.append("nonzero-swap-before")

    if not preflight_unknowns:
        schedules = (
            ("direct", CASES),
            ("repeated", CASES),
            ("inverse", tuple(reversed(CASES))),
        )
        for binary_spec in BINARIES:
            binary_path = (ROOT / binary_spec["path"]).resolve() if not str(binary_spec["path"]).startswith("/") else Path(binary_spec["path"])
            binary_result: dict[str, Any] = {
                "label": binary_spec["label"],
                "revision": binary_spec.get("revision"),
                "identity": identity(binary_path, binary_spec["sha256"]),
                "runs": [],
            }
            if binary_result["identity"]["classification"] == "Unknown":
                binary_result["classification"] = "Unknown"
                binary_result["reason_code"] = "ambiguous-binary-identity"
                receipt["binaries"].append(binary_result)
                continue

            observed_by_schedule: dict[str, dict[str, str]] = {}
            for schedule_name, schedule_cases in schedules:
                observed_by_schedule[schedule_name] = {}
                for case_id, expression, expected in schedule_cases:
                    run = run_capped([str(binary_path), "eval", "--format", "raw", expression], batch_started)
                    classification, reason = classify_probe(run, expected)
                    stdout = run.pop("stdout")
                    stderr = run.pop("stderr")
                    record = {
                        "schedule": schedule_name,
                        "case_id": case_id,
                        "expression": expression,
                        "expected_utf8": expected,
                        "expected_hex": expected.encode("utf-8").hex(),
                        "expected_codepoints": codepoints(expected),
                        **run,
                        **public_stream_fields(stdout, "stdout"),
                        **public_stream_fields(stderr, "stderr"),
                        "classification": classification,
                        "reason_code": reason,
                    }
                    if record["stdout_utf8"] is not None:
                        observed_by_schedule[schedule_name][case_id] = record["stdout_utf8"]
                    binary_result["runs"].append(record)

            vector = {
                case_id: next(
                    record["classification"] for record in binary_result["runs"]
                    if record["schedule"] == "direct" and record["case_id"] == case_id
                ) for case_id, _, _ in CASES
            }
            stable = all(observed_by_schedule[name] == observed_by_schedule["direct"] for name in ("repeated", "inverse"))
            outputs_expected = observed_by_schedule["direct"] == binary_spec["expected_observed"]
            vector_expected = vector == binary_spec["expected_vector"]
            no_unknown = all(record["classification"] != "Unknown" for record in binary_result["runs"])
            binary_result.update({
                "direct_vector": vector,
                "expected_vector": binary_spec["expected_vector"],
                "repeat_and_reverse_stable": stable,
                "expected_observed_outputs_match": outputs_expected,
                "classification": "Preserved" if stable and outputs_expected and vector_expected and no_unknown else "Unknown",
                "reason_code": None if stable and outputs_expected and vector_expected and no_unknown else "unexpected-calibration-vector",
            })
            receipt["binaries"].append(binary_result)

        wrong_hash = "0" * 64
        attack_run = run_capped([
            sys.executable,
            str(Path(__file__).resolve()),
            "--identity-only",
            "--binary", "/usr/local/bin/typst",
            "--expected-sha256", wrong_hash,
        ], batch_started)
        attack_stdout = attack_run.pop("stdout")
        attack_stderr = attack_run.pop("stderr")
        try:
            attack_payload = json.loads(attack_stdout.decode("utf-8", "strict"))
        except (UnicodeDecodeError, json.JSONDecodeError):
            attack_payload = None
        attack_ok = (
            attack_run.get("reason_code") is None
            and attack_run.get("exit") not in (None, 0)
            and isinstance(attack_payload, dict)
            and attack_payload.get("classification") == "Unknown"
            and attack_payload.get("reason_code") == "ambiguous-binary-identity"
            and not attack_stderr
        )
        receipt["invalid_hash_attack"] = {
            **attack_run,
            **public_stream_fields(attack_stdout, "stdout"),
            **public_stream_fields(attack_stderr, "stderr"),
            "supplied_sha256": wrong_hash,
            "parsed_result": attack_payload,
            "expected": {"classification": "Unknown", "exit_nonzero": True},
            "classification": "Preserved" if attack_ok else "Unknown",
            "reason_code": None if attack_ok else "invalid-hash-attack-failed",
        }

    receipt["swap"]["after"] = swap_snapshot()
    if receipt["swap"]["after"]["used_kib"] != 0:
        preflight_unknowns.append("nonzero-swap-after")
    receipt["preflight_unknowns"] = preflight_unknowns
    receipt["duration_seconds"] = round(time.monotonic() - batch_started, 6)
    receipt["finished_at"] = utc_now()
    binary_ok = len(receipt["binaries"]) == len(BINARIES) and all(
        item.get("classification") == "Preserved" for item in receipt["binaries"]
    )
    attack_ok = receipt["invalid_hash_attack"].get("classification") == "Preserved"
    within_batch = receipt["duration_seconds"] <= BATCH_TIMEOUT_SECONDS
    receipt["classification"] = "Preserved" if not preflight_unknowns and binary_ok and attack_ok and within_batch else "Unknown"
    receipt["reason_code"] = None if receipt["classification"] == "Preserved" else "mandatory-oracle-gate-unresolved"
    receipt["claim_boundary"] = "Only the frozen P1399 ContextBlock repr morphology fragment; no general functional equivalence."
    print(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2))
    return 0 if receipt["classification"] == "Preserved" else 2


if __name__ == "__main__":
    raise SystemExit(main())
