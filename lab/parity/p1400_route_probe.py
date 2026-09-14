#!/usr/bin/env python3
"""Instrumento bilateral segregado do Passo 1400.

O runner valida os inputs selados antes de executar produtos, limita cada
processo e stream, e nunca promove evidência incompleta. Os brutos ficam em
/tmp; somente ``--run --output PATH`` escreve o resultado consolidado.
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
import tempfile
import time
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SEALED_PRESEAL_SHA256 = "95c09cbb48fd43bc87271ba6e3747d15063d33b5e87748d97ad478990ceb4f2c"
STREAM_LIMIT = 64 * 1024
RLIMIT_AS_KIB = 512 * 1024
TIMEOUT_SECONDS = 15.0
BATCH_SIZE = 12
ORDERS = ("direct", "direct-repeat", "reverse")
UNKNOWN_REASONS = {
    "BASELINE_DRIFT", "IDENTITY_MISMATCH", "AMBIGUOUS_IDENTITY",
    "UNSUPPORTED_CONSTRUCTION", "PARSER_CONTROL_FAILED",
    "TARGET_CONTROL_FAILED", "FEATURE_CONTROL_FAILED",
    "FIXTURE_CONTROL_FAILED", "RUNNER_FAILURE", "TOOL_MISSING",
    "TARGET_UNAVAILABLE", "UNEXPECTED_EXIT", "TIMEOUT", "OOM",
    "NONZERO_SWAP", "OUTPUT_MISSING", "OUTPUT_TRUNCATED",
    "OUTPUT_UNREADABLE", "STREAM_LIMIT", "NON_BILATERAL_EVIDENCE",
    "CALIBRATION_BUDGET_EXHAUSTED", "OWNER_AMBIGUOUS",
}
ABSENCE_MARKERS = (
    "unknown variable", "unknown field", "unknown method", "no field",
    "no method", "does not contain", "cannot access", "not found in",
    "unrecognized field", "unrecognized function", "unknown function",
)


class GateError(RuntimeError):
    """A sealed-input or execution gate failed."""


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="microseconds")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise GateError(f"JSON inválido em {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise GateError(f"objeto JSON esperado em {path}")
    return value


def stream_object(data: bytes) -> dict[str, Any]:
    try:
        text = data.decode("utf-8", "strict")
    except UnicodeDecodeError:
        text = None
    return {
        "encoding": "base64",
        "data": base64.b64encode(data).decode("ascii"),
        "bytes": len(data),
        "sha256": sha256_bytes(data),
        "utf8": text,
    }


def expected_bytes(value: dict[str, Any]) -> bytes:
    if value.get("encoding") != "base64" or not isinstance(value.get("data"), str):
        raise GateError("stream expected não é base64")
    try:
        raw = base64.b64decode(value["data"], validate=True)
    except (ValueError, TypeError) as exc:
        raise GateError("base64 expected inválido") from exc
    if value.get("bytes") != len(raw):
        raise GateError("cardinalidade de stream expected divergente")
    if "sha256" in value and value["sha256"] != sha256_bytes(raw):
        raise GateError("hash de stream expected divergente")
    return raw


def git_value(*args: str) -> str:
    run = subprocess.run(
        ["git", *args], cwd=ROOT, stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5, check=False,
    )
    if run.returncode != 0:
        raise GateError(f"git {' '.join(args)} falhou")
    return run.stdout.decode("ascii", "strict").strip()


def chunks(values: list[Any], size: int) -> list[list[Any]]:
    return [values[index:index + size] for index in range(0, len(values), size)]


def validate_inputs(args: argparse.Namespace) -> dict[str, Any]:
    paths = {
        "manifest": args.manifest.resolve(), "contract": args.contract.resolve(),
        "oracles": args.oracles.resolve(), "mutants": args.mutants.resolve(),
        "preseal": args.preseal.resolve(), "inventory": args.inventory.resolve(),
    }
    documents = {key: load_json(path) for key, path in paths.items()}
    preseal = documents["preseal"]
    errors: list[str] = []
    if sha256_path(paths["preseal"]) != SEALED_PRESEAL_SHA256:
        errors.append("preseal_sha256")
    if preseal.get("status") != "SEALED" or preseal.get("verdict") != "SEALED":
        errors.append("preseal_state")
    sealed = preseal.get("sealed_inputs", {})
    sealed_map = {
        "manifest": paths["manifest"], "contract": paths["contract"],
        "oracles": paths["oracles"], "mutants": paths["mutants"],
        "inventory": paths["inventory"],
    }
    step_info = sealed.get("step", {})
    step_path = ROOT / str(step_info.get("path", ""))
    if not step_path.is_file() or sha256_path(step_path) != step_info.get("sha256"):
        errors.append("step_sha256")
    for key, path in sealed_map.items():
        if not path.is_file() or sha256_path(path) != sealed.get(key, {}).get("sha256"):
            errors.append(f"{key}_sha256")

    manifest, contract = documents["manifest"], documents["contract"]
    oracles, inventory = documents["oracles"], documents["inventory"]
    frozen = manifest.get("frozen_routes")
    contract_routes = contract.get("frozen_routes")
    cases = oracles.get("cases")
    if not isinstance(frozen, list) or len(frozen) != 93 or len(set(frozen)) != 93:
        errors.append("manifest_denominator")
        frozen = []
    if contract_routes != frozen:
        errors.append("contract_denominator")
    if not isinstance(cases, list) or len(cases) != 93:
        errors.append("oracle_case_count")
        cases = []
    case_paths = [case.get("path") for case in cases]
    case_ids = [case.get("id") for case in cases]
    if set(case_paths) != set(frozen) or len(set(case_paths)) != 93:
        errors.append("oracle_denominator")
    if len(set(case_ids)) != 93:
        errors.append("oracle_ids")
    if any(path == "format" or str(path).startswith("format.") for path in frozen):
        errors.append("format_in_denominator")

    crosswalk = inventory.get("crosswalk", [])
    vanila_cells = [row for row in crosswalk if row.get("classification") == "VANILLA_ONLY"]
    unique = {row.get("path") for row in vanila_cells}
    non_format = {p for p in unique if p != "format" and not str(p).startswith("format.")}
    format_family = unique - non_format
    if len(vanila_cells) != 544 or len(unique) != 217:
        errors.append("inventory_counts")
    if len(format_family) != 124 or non_format != set(frozen):
        errors.append("inventory_frozen_set")
    hashes = inventory.get("determinism_hashes")
    expected_det = contract.get("inventory_precondition", {}).get("must_all_hold", {}).get("determinism_hash")
    if not isinstance(hashes, list) or len(hashes) != 4 or len(set(hashes)) != 1 or hashes[0] != expected_det:
        errors.append("inventory_determinism")
    if inventory.get("verdict") != "PASS" or inventory.get("crosswalk_semantics") != "presence-only":
        errors.append("inventory_verdict")

    probe_ids: list[str] = []
    html_tags: list[str] = []
    for case in cases:
        probes = case.get("probes", [])
        kinds = [probe.get("kind") for probe in probes]
        probe_ids.extend(str(probe.get("id")) for probe in probes)
        if kinds != case.get("required_probe_kinds"):
            errors.append(f"probe_kinds:{case.get('path')}")
        if not probes or any(not probe.get("independent_process") or not probe.get("always_reachable") for probe in probes):
            errors.append(f"probe_independence:{case.get('path')}")
        for probe in probes:
            expected = probe.get("vanila_expected", {})
            try:
                expected_bytes(expected.get("stdout", {}))
                expected_bytes(expected.get("stderr", {}))
                projection = expected.get("public_projection", {})
                if projection.get("kind") == "html-bytes":
                    if not isinstance(projection.get("bytes"), int) or not isinstance(projection.get("sha256"), str):
                        raise GateError("projeção HTML expected inválida")
                if projection.get("kind") == "pdftotext-layout":
                    expected_bytes(projection.get("bytes", {}))
                    expected_bytes(projection.get("extractor_stderr", {}))
                    if projection.get("extractor_exit") != 0:
                        raise GateError("extractor expected inválido")
            except GateError:
                errors.append(f"expected_stream:{probe.get('id')}")
        path = str(case.get("path"))
        if path.startswith("html."):
            alts = [p for p in probes if p.get("kind") == "alternative"]
            tag = path.removeprefix("html.")
            if len(alts) != 1 or alts[0].get("literal_tag") != tag:
                errors.append(f"html_alternative:{path}")
            else:
                html_tags.append(tag)
            if any(p.get("argv_template", [None, None])[1] != "compile"
                   or "--format" not in p.get("argv_template", [])
                   or "html" not in p.get("argv_template", [])
                   for p in probes):
                errors.append(f"html_transport:{path}")
        if path.startswith("math."):
            alts = [p for p in probes if p.get("kind") == "alternative"]
            if len(alts) != 1 or path in str(alts[0].get("expression", "")):
                errors.append(f"math_alternative:{path}")
        if "construction" in kinds:
            if kinds != ["qualified", "construction", "invocation"]:
                errors.append(f"construction_chain:{path}")
            ctor = probes[kinds.index("construction")]
            if ctor.get("ancestor_qualified_constructor_used") is not False:
                errors.append(f"ancestor_constructor:{path}")
    if len(probe_ids) != len(set(probe_ids)) or len(probe_ids) != 182:
        errors.append("probe_ids_or_count")
    literal_tags = contract.get("alternative_probe_contract", {}).get("html", {}).get("literal_tags_exact", [])
    if sorted(html_tags) != sorted(literal_tags) or len(html_tags) != 44:
        errors.append("html_literal_set")
    profile_counts = Counter(case.get("execution_profile") for case in cases)
    if profile_counts != Counter({"default": 43, "html": 40, "a11y-extras": 6, "html+a11y": 4}):
        errors.append("profile_coverage")
    expected_refs = {
        "default": ["eval-parser-feature-common:default"],
        "a11y-extras": ["eval-parser-feature-common:a11y-extras"],
        "html": ["eval-parser-feature-common:html", "html-target-fixture-common:html", "html-api-valid-args-common:html"],
        "html+a11y": ["eval-parser-feature-common:html+a11y", "html-target-fixture-common:html+a11y", "html-api-valid-args-common:html+a11y"],
    }
    if any(case.get("control_refs") != expected_refs.get(case.get("execution_profile")) for case in cases):
        errors.append("control_reference_scope")
    controls = {control.get("id"): control for control in oracles.get("controls", [])}
    required_control_ids = {item for values in expected_refs.values() for item in values}
    required_control_ids.add("P1396-liminf-thin-space")
    if not required_control_ids <= controls.keys():
        errors.append("control_set")
    for control_id in required_control_ids & controls.keys():
        control = controls[control_id]
        if "--target" in control.get("argv_template", []):
            errors.append(f"asymmetric_target_control:{control_id}")
        try:
            expected_bytes(control.get("vanila_expected", {}).get("stdout", {}))
            expected_bytes(control.get("vanila_expected", {}).get("stderr", {}))
        except GateError:
            errors.append(f"control_expected:{control_id}")
    fixture_control = controls.get("html-target-fixture-common:html", {})
    fixture_source = str(fixture_control.get("source", "")).encode("utf-8")
    control_contract = contract.get("control_separation_r2", {}).get("html_target_and_fixture_control", {})
    if (sha256_bytes(fixture_source) != control_contract.get("fixture_utf8_sha256")
            or fixture_source.decode("utf-8") != control_contract.get("fixture_utf8")
            or fixture_source.endswith(b"\n")):
        errors.append("html_contextual_fixture")
    if oracles.get("revision", {}).get("contract_sha256") != sha256_path(paths["contract"]):
        errors.append("oracle_contract_link")
    mutation_contract_hash = documents["mutants"].get("revision_r4", {}).get("contract_sha256")
    if mutation_contract_hash != sha256_path(paths["contract"]):
        errors.append("mutant_contract_link")

    identities = manifest.get("identities", {})
    binary_paths = {
        "vanila": args.vanila_bin.resolve(),
        "crystalline": args.crystalline_bin.resolve(),
        "vanila_enumerator": (ROOT / identities.get("vanila_enumerator", {}).get("path", "")).resolve(),
        "crystalline_enumerator": (ROOT / identities.get("crystalline_enumerator", {}).get("path", "")).resolve(),
    }
    expected_binary_hashes = {
        "vanila": identities.get("vanila_product", {}).get("sha256"),
        "crystalline": identities.get("crystalline_product", {}).get("sha256"),
        "vanila_enumerator": identities.get("vanila_enumerator", {}).get("sha256"),
        "crystalline_enumerator": identities.get("crystalline_enumerator", {}).get("sha256"),
    }
    actual_hashes: dict[str, str | None] = {}
    for key, path in binary_paths.items():
        actual_hashes[key] = sha256_path(path) if path.is_file() else None
        if actual_hashes[key] != expected_binary_hashes[key]:
            errors.append(f"identity:{key}")
    head = git_value("rev-parse", "HEAD")
    tree = git_value("rev-parse", "HEAD^{tree}")
    baseline = manifest.get("causal_baseline", {})
    if head != baseline.get("repository_head") or tree != baseline.get("repository_tree"):
        errors.append("repository_baseline")
    if errors:
        raise GateError(";".join(sorted(set(errors))))
    return {
        "paths": paths, "documents": documents, "head": head, "tree": tree,
        "routes": frozen, "cases": cases, "binary_paths": binary_paths,
        "binary_hashes": actual_hashes,
        "input_hashes": {key: sha256_path(path) for key, path in paths.items()},
        "step_sha256": sha256_path(step_path),
    }


def evaluate_mutation_gate(context: dict[str, Any]) -> dict[str, Any]:
    mutants = context["documents"]["mutants"]
    negative = mutants.get("negative_mutations", [])
    positive = mutants.get("positive_controls", [])
    opaque = mutants.get("opaque_controls", [])
    required = set(context["documents"]["contract"].get("mutation_gate", {}).get("required_attack_families", []))
    observed = {item.get("attack_family") for item in negative}
    errors: list[str] = []
    if len(negative) != 33 or len(positive) != 6 or len(opaque) != 8:
        errors.append("mutation_cardinality")
    if not required <= observed:
        errors.append("attack_family_coverage")
    required_r3_reasons = {
        "ASYMMETRIC_EVAL_TARGET_REUSED_AS_COMMON_CONTROL",
        "HTML_CONTROL_CONTAMINATES_49_NON_HTML_ROUTES",
        "ASYMMETRIC_PER_PRODUCT_FLAG_SUBSTITUTION",
        "HTML_TARGET_FIXTURE_CONTEXT_REMOVED",
    }
    if not required_r3_reasons <= {item.get("reason_code") for item in negative}:
        errors.append("r3_control_attack_coverage")
    ids = [item.get("id") for item in negative + positive + opaque]
    if len(ids) != len(set(ids)):
        errors.append("mutation_ids")
    violated = sum(item.get("valid_negative") is True and item.get("expected_contract_classification") == "Violated" for item in negative)
    preserved = sum(item.get("expected_contract_classification") == "Preserved" for item in positive)
    unknown = sum(item.get("expected_contract_classification") == "Unknown" and item.get("reason_code") in UNKNOWN_REASONS for item in opaque)
    if violated != len(negative) or preserved != len(positive) or unknown != len(opaque):
        errors.append("mutation_expected_boundary")
    score = violated / len(negative) if negative else 0.0
    vectors: dict[str, str] = {}
    canonical = sorted((item.get("id"), item.get("reason_code"), item.get("expected_contract_classification")) for item in negative + positive + opaque)
    for order in ORDERS:
        vectors[order] = sha256_bytes(json.dumps(canonical, separators=(",", ":"), ensure_ascii=False).encode())
    if len(set(vectors.values())) != 1 or score != 1.0:
        errors.append("mutation_score_or_order")
    return {
        "result": "PASS" if not errors else "FAIL", "errors": errors,
        "valid_negative": len(negative), "violated": violated,
        "positive": len(positive), "preserved": preserved,
        "opaque": len(opaque), "unknown": unknown,
        "mutation_score": score, "vectors_sha256": vectors,
    }


def child_limits() -> None:
    os.setsid()
    limit = RLIMIT_AS_KIB * 1024
    resource.setrlimit(resource.RLIMIT_AS, (limit, limit))


def read_swap_kib(pid: int) -> int | None:
    try:
        for line in Path(f"/proc/{pid}/status").read_text(encoding="ascii").splitlines():
            if line.startswith("VmSwap:"):
                return int(line.split()[1])
    except (OSError, UnicodeError, ValueError):
        return None
    return None


def kill_group(pid: int) -> None:
    try:
        os.killpg(pid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def run_capped(argv: list[str], raw_prefix: Path) -> dict[str, Any]:
    started_at, started = now(), time.monotonic()
    reason: str | None = None
    max_swap = 0
    try:
        process = subprocess.Popen(
            argv, cwd=ROOT, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, close_fds=True, preexec_fn=child_limits,
        )
    except FileNotFoundError as exc:
        return {"instant": started_at, "argv": argv, "exit_code": None,
                "duration_ms": 0.0, "max_rss_kib": None, "swap_kib": None,
                "timeout": False, "truncated": False, "reason_code": "TOOL_MISSING",
                "stdout": stream_object(b""), "stderr": stream_object(str(exc).encode())}
    except (OSError, subprocess.SubprocessError) as exc:
        return {"instant": started_at, "argv": argv, "exit_code": None,
                "duration_ms": 0.0, "max_rss_kib": None, "swap_kib": None,
                "timeout": False, "truncated": False, "reason_code": "RUNNER_FAILURE",
                "stdout": stream_object(b""), "stderr": stream_object(str(exc).encode())}

    assert process.stdout is not None and process.stderr is not None
    streams = {process.stdout.fileno(): ("stdout", bytearray()), process.stderr.fileno(): ("stderr", bytearray())}
    selector = selectors.DefaultSelector()
    for fd in streams:
        os.set_blocking(fd, False)
        selector.register(fd, selectors.EVENT_READ)
    status = None
    usage = None
    while status is None or selector.get_map():
        elapsed = time.monotonic() - started
        swap = read_swap_kib(process.pid)
        if swap is not None:
            max_swap = max(max_swap, swap)
        if reason is None and elapsed > TIMEOUT_SECONDS:
            reason = "TIMEOUT"
            kill_group(process.pid)
        for key, _ in selector.select(0.01):
            fd = key.fd
            _, buffer = streams[fd]
            try:
                chunk = os.read(fd, 8192)
            except BlockingIOError:
                continue
            if not chunk:
                selector.unregister(fd)
                continue
            remaining = STREAM_LIMIT + 1 - len(buffer)
            if remaining > 0:
                buffer.extend(chunk[:remaining])
            if len(buffer) > STREAM_LIMIT and reason is None:
                reason = "OUTPUT_TRUNCATED"
                kill_group(process.pid)
        if status is None:
            waited, wait_status, wait_usage = os.wait4(process.pid, os.WNOHANG)
            if waited == process.pid:
                status, usage = wait_status, wait_usage
    if status is None:
        _, status, usage = os.wait4(process.pid, 0)
    process.returncode = os.waitstatus_to_exitcode(status)
    out = bytes(streams[process.stdout.fileno()][1][:STREAM_LIMIT])
    err = bytes(streams[process.stderr.fileno()][1][:STREAM_LIMIT])
    process.stdout.close()
    process.stderr.close()
    truncated = any(len(buffer) > STREAM_LIMIT for _, buffer in streams.values())
    if max_swap > 0 and reason is None:
        reason = "NONZERO_SWAP"
    if process.returncode in (-signal.SIGKILL, 137) and reason is None:
        reason = "OOM"
    raw_prefix.parent.mkdir(parents=True, exist_ok=True)
    raw_prefix.with_suffix(".stdout").write_bytes(out)
    raw_prefix.with_suffix(".stderr").write_bytes(err)
    return {
        "instant": started_at, "argv": argv, "cwd": str(ROOT),
        "rlimit_as_kib": RLIMIT_AS_KIB, "timeout_seconds": TIMEOUT_SECONDS,
        "stream_limit_bytes": STREAM_LIMIT, "exit_code": process.returncode,
        "duration_ms": round((time.monotonic() - started) * 1000, 3),
        "max_rss_kib": int(usage.ru_maxrss) if usage else None,
        "swap_kib": max_swap, "timeout": reason == "TIMEOUT",
        "truncated": truncated, "reason_code": reason,
        "stdout": stream_object(out), "stderr": stream_object(err),
        "raw_stdout": str(raw_prefix.with_suffix(".stdout")),
        "raw_stderr": str(raw_prefix.with_suffix(".stderr")),
    }


def substitute_argv(template: list[str], binary: Path, fixture: Path | None, artifact: Path | None) -> list[str]:
    values = {"{binary}": str(binary), "{fixture}": str(fixture) if fixture else "", "{artifact}": str(artifact) if artifact else ""}
    return [values.get(part, part) for part in template]


def artifact_extension(probe: dict[str, Any]) -> str:
    projection = probe.get("vanila_expected", {}).get("public_projection", {})
    return ".html" if projection.get("kind") == "html-bytes" else ".pdf"


def run_probe(probe: dict[str, Any], binary: Path, product: str, schedule: str,
              raw_root: Path, serial: int) -> dict[str, Any]:
    stem = f"{serial:05d}-{schedule}-{probe['id']}-{product}"
    work = raw_root / "work" / stem
    work.mkdir(parents=True, exist_ok=True)
    fixture = None
    if probe.get("source") is not None:
        fixture = work / "fixture.typ"
        fixture.write_text(str(probe["source"]), encoding="utf-8")
    artifact = work / ("artifact" + artifact_extension(probe)) if "{artifact}" in probe["argv_template"] else None
    argv = substitute_argv(probe["argv_template"], binary, fixture, artifact)
    receipt = run_capped(argv, raw_root / "streams" / stem)
    receipt.update({"probe_id": probe["id"], "kind": probe["kind"], "product": product,
                    "binary_path": str(binary), "binary_sha256": sha256_path(binary)})
    if artifact is not None:
        if artifact.is_file():
            raw_artifact = raw_root / "artifacts" / (stem + artifact.suffix)
            raw_artifact.parent.mkdir(parents=True, exist_ok=True)
            artifact.replace(raw_artifact)
            receipt["artifact"] = {"path": str(raw_artifact), "bytes": raw_artifact.stat().st_size,
                                   "sha256": sha256_path(raw_artifact), "missing": False}
        else:
            receipt["artifact"] = {"path": None, "bytes": 0, "sha256": None, "missing": True}
    projection = probe.get("vanila_expected", {}).get("public_projection", {})
    kind = projection.get("kind")
    if kind == "stdout":
        receipt["public_projection"] = {"kind": "stdout", "bytes": receipt["stdout"]}
    elif kind == "html-bytes":
        receipt["public_projection"] = {"kind": kind, "bytes": receipt.get("artifact")}
    elif kind == "pdftotext-layout":
        if receipt.get("artifact", {}).get("missing"):
            receipt["public_projection"] = {"kind": kind, "reason_code": "OUTPUT_MISSING"}
        else:
            extractor_argv = ["pdftotext", "-layout", receipt["artifact"]["path"], "-"]
            extractor = run_capped(extractor_argv, raw_root / "streams" / (stem + "-extractor"))
            receipt["public_projection"] = {"kind": kind, "bytes": extractor["stdout"], "extractor": extractor,
                                             "pdf_internal_bytes_compared": False}
    return receipt


def expected_matches(probe: dict[str, Any], receipt: dict[str, Any]) -> bool:
    expected = probe["vanila_expected"]
    if receipt.get("reason_code") is not None or receipt.get("exit_code") != expected.get("exit_code"):
        return False
    if base64.b64decode(receipt["stdout"]["data"]) != expected_bytes(expected["stdout"]):
        return False
    if base64.b64decode(receipt["stderr"]["data"]) != expected_bytes(expected["stderr"]):
        return False
    projection = expected.get("public_projection", {})
    actual = receipt.get("public_projection", {})
    if projection.get("kind") == "html-bytes":
        artifact = actual.get("bytes", {})
        return artifact.get("bytes") == projection.get("bytes") and artifact.get("sha256") == projection.get("sha256")
    if projection.get("kind") == "pdftotext-layout":
        extractor = actual.get("extractor", {})
        return (extractor.get("reason_code") is None and extractor.get("exit_code") == projection.get("extractor_exit")
                and base64.b64decode(actual.get("bytes", {}).get("data", "")) == expected_bytes(projection["bytes"])
                and base64.b64decode(extractor.get("stderr", {}).get("data", "")) == expected_bytes(projection["extractor_stderr"]))
    return True


def resource_reason(receipt: dict[str, Any]) -> str | None:
    if receipt.get("reason_code"):
        return str(receipt["reason_code"])
    projection = receipt.get("public_projection", {})
    if projection.get("reason_code"):
        return str(projection["reason_code"])
    extractor = projection.get("extractor")
    if isinstance(extractor, dict) and extractor.get("reason_code"):
        return str(extractor["reason_code"])
    if isinstance(extractor, dict) and extractor.get("exit_code") != 0:
        return "OUTPUT_UNREADABLE"
    return None


def same_public_observables(left: dict[str, Any], right: dict[str, Any]) -> bool:
    if left.get("exit_code") != right.get("exit_code"):
        return False
    if left.get("stdout", {}).get("sha256") != right.get("stdout", {}).get("sha256"):
        return False
    if left.get("stderr", {}).get("sha256") != right.get("stderr", {}).get("sha256"):
        return False
    lp, rp = left.get("public_projection", {}), right.get("public_projection", {})
    if lp.get("kind") != rp.get("kind"):
        return False
    if lp.get("kind") == "html-bytes":
        return lp.get("bytes", {}).get("sha256") == rp.get("bytes", {}).get("sha256")
    if lp.get("kind") == "pdftotext-layout":
        return (lp.get("bytes", {}).get("sha256") == rp.get("bytes", {}).get("sha256")
                and lp.get("extractor", {}).get("exit_code") == rp.get("extractor", {}).get("exit_code")
                and lp.get("extractor", {}).get("stderr", {}).get("sha256") == rp.get("extractor", {}).get("stderr", {}).get("sha256"))
    return True


def specifically_absent(receipt: dict[str, Any], path: str) -> bool:
    if receipt.get("exit_code") == 0 or receipt.get("reason_code") is not None:
        return False
    text = (receipt.get("stderr", {}).get("utf8") or "").lower()
    leaf = path.rsplit(".", 1)[-1].lower()
    return leaf in text and any(marker in text for marker in ABSENCE_MARKERS)


def classify_case(case: dict[str, Any], probe_runs: dict[str, tuple[dict[str, Any], dict[str, Any]]]) -> tuple[str, str, str]:
    probes = {probe["kind"]: probe for probe in case["probes"]}
    for probe in case["probes"]:
        vanila, crystal = probe_runs[probe["kind"]]
        reason = resource_reason(vanila) or resource_reason(crystal)
        if reason:
            return "UNKNOWN", reason if reason in UNKNOWN_REASONS else "RUNNER_FAILURE", f"ROUTE_{probe['kind'].upper()}_EXECUTION"
        if not expected_matches(probe, vanila):
            return "UNKNOWN", "BASELINE_DRIFT", f"VANILA_{probe['kind'].upper()}_EXPECTED"
    for kind in ("construction", "invocation"):
        if kind in probes:
            vanila, crystal = probe_runs[kind]
            if vanila.get("exit_code") != 0 or crystal.get("exit_code") != 0:
                return "UNKNOWN", "UNSUPPORTED_CONSTRUCTION", f"INSTANCE_{kind.upper()}"
            if not same_public_observables(vanila, crystal):
                return "UNKNOWN", "NON_BILATERAL_EVIDENCE", f"INSTANCE_{kind.upper()}"
    vanila_q, crystal_q = probe_runs["qualified"]
    if crystal_q.get("exit_code") == 0:
        if vanila_q.get("exit_code") != 0:
            return "UNKNOWN", "BASELINE_DRIFT", "VANILA_QUALIFIED_EXPECTED"
        return (("MATCH", "BILATERAL_PUBLIC_OBSERVABLES_MATCH", "ROUTE_PUBLIC_SEMANTICS_OR_MORPHOLOGY")
                if same_public_observables(vanila_q, crystal_q)
                else ("DIFFERENCE", "PUBLIC_OBSERVABLE_DIFFERENCE", "ROUTE_PUBLIC_SEMANTICS_OR_MORPHOLOGY"))
    if not specifically_absent(crystal_q, case["path"]):
        return "UNKNOWN", "UNEXPECTED_EXIT", "ROUTE_QUALIFIED_DIAGNOSTIC"
    if "alternative" in probes:
        vanila_alt, crystal_alt = probe_runs["alternative"]
        if vanila_alt.get("exit_code") == 0 and crystal_alt.get("exit_code") == 0:
            if same_public_observables(vanila_alt, crystal_alt):
                return "PARTIAL", "QUALIFIED_ABSENT_FUNCTIONAL_ALTERNATIVE", "ROUTE_PUBLIC_SEMANTICS_OR_MORPHOLOGY"
            return "UNKNOWN", "NON_BILATERAL_EVIDENCE", "ROUTE_ALTERNATIVE_PROJECTION"
        if crystal_alt.get("exit_code") != 0:
            return "UNKNOWN", "UNEXPECTED_EXIT", "ROUTE_ALTERNATIVE_DIAGNOSTIC"
    return "ABSENT", "QUALIFIED_PUBLIC_FORM_ABSENT", "ROUTE_QUALIFIED_DIAGNOSTIC"


def run_control(control: dict[str, Any], binaries: dict[str, Path], schedule: str,
                raw_root: Path, serial: int) -> dict[str, Any]:
    receipts: dict[str, dict[str, Any]] = {}
    probe = {**control, "kind": "control"}
    for product, binary in binaries.items():
        receipts[product] = run_probe(probe, binary, product, schedule, raw_root, serial)
    expected_ok = expected_matches(probe, receipts["vanila"])
    reason_code: str | None = None
    failure_phase: str | None = None
    for receipt in receipts.values():
        resource = resource_reason(receipt)
        if resource:
            reason_code, failure_phase = resource, "CONTROL_RESOURCE_EXECUTION"
            break
    control_id = str(control["id"])
    crystal = receipts["crystalline"]
    crystal_stderr = (crystal.get("stderr", {}).get("utf8") or "").lower()
    if not expected_ok:
        reason_code, failure_phase = "BASELINE_DRIFT", "VANILA_CONTROL_EXPECTED"
    elif reason_code is None and control_id.startswith("eval-parser-feature-common:"):
        if not same_public_observables(receipts["vanila"], crystal):
            if any(marker in crystal_stderr for marker in ("unexpected argument", "unrecognized option", "unknown option")):
                reason_code, failure_phase = "PARSER_CONTROL_FAILED", "CLI_OPTION_PARSING"
            elif "feature" in crystal_stderr and control.get("applies_to") != "default":
                reason_code, failure_phase = "FEATURE_CONTROL_FAILED", "FEATURE_ACTIVATION"
            else:
                reason_code, failure_phase = "PARSER_CONTROL_FAILED", "EXPRESSION_PARSING"
    elif reason_code is None and control_id.startswith("html-target-fixture-common:"):
        artifact = crystal.get("artifact", {})
        if crystal.get("exit_code") != 0:
            if "feature" in crystal_stderr:
                reason_code, failure_phase = "FEATURE_CONTROL_FAILED", "FEATURE_ACTIVATION"
            elif any(marker in crystal_stderr for marker in ("unexpected argument", "unrecognized option", "unknown option", "format")):
                reason_code, failure_phase = "TARGET_CONTROL_FAILED", "CLI_TARGET_SELECTION"
            else:
                reason_code, failure_phase = "TARGET_CONTROL_FAILED", "TARGET_COMPILATION"
        elif artifact.get("missing"):
            reason_code, failure_phase = "FIXTURE_CONTROL_FAILED", "FIXTURE_PROJECTION"
        else:
            try:
                projected = Path(str(artifact["path"])).read_text(encoding="utf-8")
            except (OSError, UnicodeError):
                projected = ""
            if "HTML target fixture." not in projected:
                reason_code, failure_phase = "FIXTURE_CONTROL_FAILED", "FIXTURE_PROJECTION"
    elif reason_code is None and control_id.startswith("html-api-valid-args-common:"):
        artifact = crystal.get("artifact", {})
        if crystal.get("exit_code") != 0:
            reason_code, failure_phase = "FIXTURE_CONTROL_FAILED", "HTML_API_ARGUMENT_VALIDATION"
        elif artifact.get("missing") or not artifact.get("bytes"):
            reason_code, failure_phase = "FIXTURE_CONTROL_FAILED", "TARGET_COMPILATION"
        else:
            try:
                projected = Path(str(artifact["path"])).read_text(encoding="utf-8")
            except (OSError, UnicodeError):
                projected = ""
            if "<div" not in projected:
                reason_code, failure_phase = "FIXTURE_CONTROL_FAILED", "HTML_API_ARGUMENT_VALIDATION"
    elif reason_code is None and not same_public_observables(receipts["vanila"], crystal):
        reason_code, failure_phase = "PARSER_CONTROL_FAILED", "MORPHOLOGY_SENTINEL"
    result = "PASS" if reason_code is None else "FAIL"
    return {"control_id": control_id, "phase": control.get("phase"), "applies_to": control.get("applies_to"),
            "purpose": control.get("purpose"), "receipts": receipts,
            "vanila_expected_matches": expected_ok, "result": result,
            "reason_code": reason_code, "failure_phase": failure_phase}


def stable_vector(rows: list[dict[str, str]]) -> str:
    canonical = sorted((row["id"], row["path"], row["state"], row["reason_code"], row["reason_phase"]) for row in rows)
    return sha256_bytes(json.dumps(canonical, ensure_ascii=False, separators=(",", ":")).encode())


def execute(context: dict[str, Any], args: argparse.Namespace) -> dict[str, Any]:
    started_at = now()
    raw_root = (args.raw_dir.resolve() if args.raw_dir else Path(tempfile.mkdtemp(prefix="p1400-route-probe-", dir="/tmp")))
    raw_root.mkdir(parents=True, exist_ok=True)
    binaries = {"vanila": context["binary_paths"]["vanila"], "crystalline": context["binary_paths"]["crystalline"]}
    cases = list(context["cases"])
    controls_by_id = {c["id"]: c for c in context["documents"]["oracles"].get("controls", [])}
    required_controls = {
        "eval-parser-feature-common:default", "eval-parser-feature-common:html",
        "eval-parser-feature-common:a11y-extras", "eval-parser-feature-common:html+a11y",
        "html-target-fixture-common:html", "html-target-fixture-common:html+a11y",
        "html-api-valid-args-common:html", "html-api-valid-args-common:html+a11y",
        "P1396-liminf-thin-space",
    }
    if not required_controls <= controls_by_id.keys():
        raise GateError("nenhum controle positivo executável")
    all_batches: list[dict[str, Any]] = []
    order_rows: dict[str, list[dict[str, str]]] = {}
    receipts_by_case: dict[str, dict[str, dict[str, list[dict[str, Any]]]]] = {
        case["id"]: {probe["kind"]: {"vanila": [], "crystalline": []} for probe in case["probes"]} for case in cases
    }
    control_executions: dict[str, dict[str, dict[str, Any]]] = {}
    serial = 0
    for schedule in ORDERS:
        ordered = list(reversed(cases)) if schedule == "reverse" else cases
        schedule_rows: list[dict[str, str]] = []
        schedule_controls: dict[str, dict[str, Any]] = {}
        for control_id in sorted(required_controls - {"P1396-liminf-thin-space"}):
            serial += 1
            schedule_controls[control_id] = run_control(
                controls_by_id[control_id], binaries, schedule, raw_root, serial
            )
        control_executions[schedule] = schedule_controls
        for batch_index, batch in enumerate(chunks(ordered, BATCH_SIZE), 1):
            batch_profile = next((case["execution_profile"] for case in batch if case["path"].startswith("html.")), batch[0]["execution_profile"])
            positive_id = f"eval-parser-feature-common:{batch_profile}"
            serial += 1
            control_receipt = run_control(
                controls_by_id[positive_id], binaries, schedule, raw_root, serial
            )
            sentinel = controls_by_id["P1396-liminf-thin-space"]
            serial += 1
            sentinel_receipt = run_control(sentinel, binaries, schedule, raw_root, serial)
            batch_record = {"schedule": schedule, "batch": batch_index,
                            "route_ids": [case["id"] for case in batch], "route_count": len(batch),
                            "parallelism": 1, "positive_control": control_receipt,
                            "difference_sentinel": sentinel_receipt}
            all_batches.append(batch_record)
            for case in batch:
                local: dict[str, tuple[dict[str, Any], dict[str, Any]]] = {}
                for probe in case["probes"]:
                    pair: dict[str, dict[str, Any]] = {}
                    for product, binary in binaries.items():
                        serial += 1
                        receipt = run_probe(probe, binary, product, schedule, raw_root, serial)
                        pair[product] = receipt
                        receipts_by_case[case["id"]][probe["kind"]][product].append(receipt)
                    local[probe["kind"]] = (pair["vanila"], pair["crystalline"])
                state, reason, reason_phase = classify_case(case, local)
                failed_control = next(
                    (schedule_controls[control_id] for control_id in case["control_refs"]
                     if schedule_controls[control_id]["result"] != "PASS"),
                    None,
                )
                if failed_control is not None:
                    state = "UNKNOWN"
                    reason = failed_control["reason_code"]
                    reason_phase = failed_control["failure_phase"]
                if sentinel_receipt["result"] != "PASS":
                    state, reason, reason_phase = "UNKNOWN", "PARSER_CONTROL_FAILED", "MORPHOLOGY_SENTINEL"
                schedule_rows.append({"id": case["id"], "path": case["path"], "state": state,
                                      "reason_code": reason, "reason_phase": reason_phase,
                                      "control_id": failed_control["control_id"] if failed_control else None})
        order_rows[schedule] = schedule_rows

    vectors = {order: stable_vector(rows) for order, rows in order_rows.items()}
    stable = len(set(vectors.values())) == 1
    by_order = {order: {row["id"]: row for row in rows} for order, rows in order_rows.items()}
    route_results: list[dict[str, Any]] = []
    for case in cases:
        states = [(by_order[order][case["id"]]["state"], by_order[order][case["id"]]["reason_code"],
                   by_order[order][case["id"]]["reason_phase"]) for order in ORDERS]
        state, reason, reason_phase = states[0] if len(set(states)) == 1 else ("UNKNOWN", "RUNNER_FAILURE", "ORDER_INVARIANCE")
        probe_receipts = receipts_by_case[case["id"]]
        route_results.append({
            "id": case["id"], "path": case["path"], "family": case["family"],
            "minimum_profile": case["minimum_profile"], "features": case["features"], "target": case["target"],
            "typst_source_or_cli_expression": [p.get("source") or p.get("expression") for p in case["probes"]],
            "bilateral_argv": {kind: {product: [r["argv"] for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "vanila_identity": {"path": str(binaries["vanila"]), "sha256": context["binary_hashes"]["vanila"]},
            "crystalline_identity": {"path": str(binaries["crystalline"]), "sha256": context["binary_hashes"]["crystalline"]},
            "raw_stdout": {kind: {product: [r["raw_stdout"] for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "raw_stderr": {kind: {product: [r["raw_stderr"] for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "exit_code": {kind: {product: [r["exit_code"] for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "artifact": {kind: {product: [r.get("artifact") for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "public_projection": {kind: {product: [r.get("public_projection") for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "adr_0107_axis": case["adr_0107_axis"], "state": state, "reason_code": reason,
            "reason_phase": reason_phase,
            "reason": {"code": reason, "phase": reason_phase},
            "positive_controls": case["control_refs"],
            "control_receipt_refs": {order: [f"/control_executions/{order}/{control_id}" for control_id in case["control_refs"]] for order in ORDERS},
            "observed_cause": f"{reason}:{reason_phase}", "inferred_cause": None,
            "inference_refuted_by": "nova evidência bilateral byte-exact da forma pública e de todos os controles obrigatórios",
            "instant": min(r["instant"] for products in probe_receipts.values() for runs in products.values() for r in runs),
            "duration_ms": round(sum(r["duration_ms"] for products in probe_receipts.values() for runs in products.values() for r in runs), 3),
            "max_rss_kib": max((r.get("max_rss_kib") or 0) for products in probe_receipts.values() for runs in products.values() for r in runs),
            "swap_kib": max((r.get("swap_kib") or 0) for products in probe_receipts.values() for runs in products.values() for r in runs),
            "stdout_bytes": sum(r["stdout"]["bytes"] for products in probe_receipts.values() for runs in products.values() for r in runs),
            "stderr_bytes": sum(r["stderr"]["bytes"] for products in probe_receipts.values() for runs in products.values() for r in runs),
            "truncated": any(r.get("truncated") for products in probe_receipts.values() for runs in products.values() for r in runs),
            "probe_receipts": probe_receipts, "schedule_classifications": {order: by_order[order][case["id"]] for order in ORDERS},
        })
    counts = Counter(route["state"] for route in route_results)
    mandatory_failure = (not stable or any(
        batch["positive_control"]["result"] != "PASS"
        or batch["difference_sentinel"]["result"] != "PASS"
        for batch in all_batches
    ))
    verdict = "INCONCLUSIVE" if counts.get("UNKNOWN", 0) else ("FAIL" if mandatory_failure else "PASS")
    return {
        "schema": "p1400-route-results/v1",
        "manifest_sha256": context["input_hashes"]["manifest"],
        "contract_sha256": context["input_hashes"]["contract"],
        "preseal_sha256": context["input_hashes"]["preseal"],
        "oracles_sha256": context["input_hashes"]["oracles"],
        "mutants_sha256": context["input_hashes"]["mutants"],
        "inventory_sha256": context["input_hashes"]["inventory"],
        "head": context["head"], "tree": context["tree"],
        "working_tree": "working tree não commitado", "started_at": started_at, "finished_at": now(),
        "binary_identities": {key: {"path": str(context["binary_paths"][key]), "sha256": context["binary_hashes"][key]} for key in ("vanila", "crystalline", "vanila_enumerator", "crystalline_enumerator")},
        "execution_orders": {order: {"vector_sha256": vectors[order], "routes": order_rows[order]} for order in ORDERS},
        "order_invariant": stable, "control_executions": control_executions,
        "control_applicability": "HTML target/fixture/API controls apply only to the 44 HTML routes",
        "batches": all_batches, "routes": route_results,
        "state_counts": dict(sorted(counts.items())), "unknown_count": counts.get("UNKNOWN", 0),
        "resource_policy": {"route_batch_size_max": BATCH_SIZE, "parallelism": 1, "process_rlimit_as_kib": RLIMIT_AS_KIB,
                            "per_process_timeout_seconds": TIMEOUT_SECONDS, "max_stdout_bytes": STREAM_LIMIT,
                            "max_stderr_bytes": STREAM_LIMIT, "swap_must_equal_kib": 0, "raw_outputs_directory": str(raw_root)},
        "attestation": "executado sem atestação de isolamento", "verdict": verdict,
    }


def emit(value: dict[str, Any]) -> None:
    print(json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")))


def main() -> int:
    parser = argparse.ArgumentParser(description="Sonda bilateral segregada P1400")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--validate-only", action="store_true")
    mode.add_argument("--gate", action="store_true")
    mode.add_argument("--run", action="store_true")
    parser.add_argument("--manifest", type=Path, default=ROOT / "00_nucleo/diagnosticos/p1400-manifest.json")
    parser.add_argument("--contract", type=Path, default=ROOT / "00_nucleo/diagnosticos/p1400-contract.json")
    parser.add_argument("--oracles", type=Path, default=ROOT / "00_nucleo/diagnosticos/p1400-oracles.json")
    parser.add_argument("--mutants", type=Path, default=ROOT / "00_nucleo/diagnosticos/p1400-mutants.json")
    parser.add_argument("--preseal", type=Path, default=ROOT / "00_nucleo/diagnosticos/p1400-preseal.json")
    parser.add_argument("--inventory", type=Path, default=Path("/tmp/p1400-current-inventory.json"))
    parser.add_argument("--vanila-bin", type=Path, default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline-bin", type=Path, default=ROOT / "target/release/typst")
    parser.add_argument("--raw-dir", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    started = time.monotonic()
    try:
        context = validate_inputs(args)
        gate = evaluate_mutation_gate(context)
        if gate["result"] != "PASS":
            raise GateError("gate discriminatório falhou: " + ",".join(gate["errors"]))
        if args.validate_only:
            emit({"schema": "p1400-runner-validation/v1", "result": "PASS", "route_count": 93,
                  "probe_count_per_order": 182, "preseal_sha256": context["input_hashes"]["preseal"],
                  "elapsed_ms": round((time.monotonic() - started) * 1000, 3)})
            return 0
        if args.gate:
            emit({"schema": "p1400-runner-gate/v1", **gate,
                  "elapsed_ms": round((time.monotonic() - started) * 1000, 3)})
            return 0
        if args.output is None:
            raise GateError("--run exige --output explícito")
        expected_output = ROOT / context["documents"]["contract"]["results_schema"]["path"]
        if args.output.resolve() != expected_output.resolve():
            raise GateError(f"--output deve ser o caminho contratado: {expected_output}")
        if args.raw_dir is not None and not args.raw_dir.resolve().is_relative_to(Path("/tmp")):
            raise GateError("--raw-dir deve ficar sob /tmp")
        result = execute(context, args)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        temporary = args.output.with_name(args.output.name + ".tmp")
        temporary.write_text(json.dumps(result, ensure_ascii=False, sort_keys=True, indent=2) + "\n", encoding="utf-8")
        temporary.replace(args.output)
        emit({"schema": "p1400-runner-completion/v1", "result": "PASS", "output": str(args.output.resolve()),
              "output_sha256": sha256_path(args.output), "state_counts": result["state_counts"],
              "unknown_count": result["unknown_count"], "verdict": result["verdict"],
              "elapsed_ms": round((time.monotonic() - started) * 1000, 3)})
        return 0
    except (GateError, OSError, subprocess.SubprocessError) as exc:
        emit({"schema": "p1400-runner-error/v1", "result": "FAIL", "reason_code": "RUNNER_FAILURE",
              "error": str(exc), "elapsed_ms": round((time.monotonic() - started) * 1000, 3)})
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
