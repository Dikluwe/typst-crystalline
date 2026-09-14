#!/usr/bin/env python3
"""Runner bilateral limitado para o vetor efetivo S02."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import resource
import subprocess
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


TIMEOUT_SECONDS = 45
ADDRESS_SPACE_BYTES = 2 * 1024**3
FILE_SIZE_BYTES = 16 * 1024**2
CPU_SECONDS = 45
CHANNEL_READ_BYTES = 1024**2
CHANNEL_PREVIEW_BYTES = 4096
MAX_PARALLEL_PROCESSES = 1

PRODUCTS = ("vanila", "cristalino")
PROFILES = ("default", "html", "a11y-extras", "html+a11y")
CASES = ("top_level", "document_over_cli", "cli_only")
PROFILE_FEATURES = {
    "default": (),
    "html": ("html",),
    "a11y-extras": ("a11y-extras",),
    "html+a11y": ("html", "a11y-extras"),
}
CASE_INPUTS = {
    "top_level": ("#set pdf(tagged: true)\nTop", ()),
    "document_over_cli": (
        "#set pdf(tagged: false)\nDocument",
        ("--pdf-tagged=true",),
    ),
    "cli_only": ("CLI", ("--pdf-tagged=true",)),
}

EXPECTED_HASHES = {
    "vanila": "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f",
    "cristalino": "60851811dca31074eca38f5d4e397ce04a89eccb7bf49c9d6e28f6759ba761f9",
    "pdfinfo": "fee70ade670fb025343aca2b5c3a2aacacb8ed9edce1b716b233b5de924b6bf5",
}


def _load_observer() -> Any:
    path = Path(__file__).with_name("p1368_s02_observer.py")
    spec = importlib.util.spec_from_file_location("p1368_s02_observer", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("P1368 observer loader unavailable")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


OBSERVER = _load_observer()


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def planned_jobs() -> list[dict[str, str]]:
    return [
        {"product": product, "profile": profile, "case": case}
        for product in PRODUCTS
        for profile in PROFILES
        for case in CASES
    ]


def classify_bilateral(vanila: dict[str, Any], cristalino: dict[str, Any]) -> str:
    if "Unknown" in (vanila.get("classification"), cristalino.get("classification")):
        return "Unknown"
    expected = [True, False, True]
    if (
        vanila.get("classification") == "Preserved"
        and cristalino.get("classification") == "Preserved"
        and vanila.get("vector") == expected
        and cristalino.get("vector") == expected
    ):
        return "MATCH"
    return "DIFFERENCE"


def _apply_limits() -> None:
    resource.setrlimit(resource.RLIMIT_AS, (ADDRESS_SPACE_BYTES, ADDRESS_SPACE_BYTES))
    resource.setrlimit(resource.RLIMIT_FSIZE, (FILE_SIZE_BYTES, FILE_SIZE_BYTES))
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_SECONDS, CPU_SECONDS))


def _bounded_process(argv: list[str]) -> dict[str, Any]:
    started = time.monotonic()
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        process = subprocess.Popen(
            argv,
            stdin=subprocess.DEVNULL,
            stdout=stdout,
            stderr=stderr,
            preexec_fn=_apply_limits,
        )
        reason_code = None
        try:
            exit_code = process.wait(timeout=TIMEOUT_SECONDS)
        except subprocess.TimeoutExpired:
            process.kill()
            exit_code = process.wait()
            reason_code = "TIMEOUT"

        elapsed_ms = round((time.monotonic() - started) * 1000, 3)
        stdout_size = stdout.tell()
        stderr_size = stderr.tell()
        stdout.seek(0)
        stderr.seek(0)
        stdout_bytes = stdout.read(CHANNEL_READ_BYTES + 1)
        stderr_bytes = stderr.read(CHANNEL_READ_BYTES + 1)

    if stdout_size > CHANNEL_READ_BYTES or stderr_size > CHANNEL_READ_BYTES:
        reason_code = "CHANNEL_LIMIT"
    elif exit_code < 0 and reason_code is None:
        reason_code = f"SIGNAL_{-exit_code}"

    return {
        "exit_code": exit_code,
        "reason_code": reason_code,
        "elapsed_ms": elapsed_ms,
        "stdout_size": stdout_size,
        "stderr_size": stderr_size,
        "stdout": stdout_bytes[:CHANNEL_READ_BYTES],
        "stderr": stderr_bytes[:CHANNEL_READ_BYTES],
    }


def _receipt(process: dict[str, Any]) -> dict[str, Any]:
    return {
        "exit_code": process["exit_code"],
        "reason_code": process["reason_code"],
        "elapsed_ms": process["elapsed_ms"],
        "stdout_size": process["stdout_size"],
        "stderr_size": process["stderr_size"],
        "stdout_sha256": hashlib.sha256(process["stdout"]).hexdigest(),
        "stderr_sha256": hashlib.sha256(process["stderr"]).hexdigest(),
        "stderr_preview": process["stderr"][:CHANNEL_PREVIEW_BYTES].decode(
            "utf-8", errors="replace"
        ),
    }


def _unknown(reason_code: str) -> dict[str, Any]:
    return {"complete": False, "value": None, "reason_code": reason_code}


def execute_job(
    job: dict[str, str],
    binaries: dict[str, Path],
    pdfinfo: Path,
    root: Path,
) -> dict[str, Any]:
    case_root = root / job["product"] / job["profile"] / job["case"]
    case_root.mkdir(parents=True, exist_ok=False)
    source_path = case_root / "probe.typ"
    output_path = case_root / "out.pdf"
    source, options = CASE_INPUTS[job["case"]]
    source_path.write_text(source, encoding="utf-8")

    features = PROFILE_FEATURES[job["profile"]]
    feature_args = ["--features", ",".join(features)] if features else []
    compile_argv = [
        str(binaries[job["product"]]),
        "compile",
        *feature_args,
        *options,
        str(source_path),
        str(output_path),
    ]
    compile_process = _bounded_process(compile_argv)
    compile_receipt = _receipt(compile_process)
    if compile_process["reason_code"] is not None:
        observation = _unknown(compile_process["reason_code"])
        return {"job": job, "compile": compile_receipt, "pdfinfo": None, "observation": observation}
    if compile_process["exit_code"] != 0:
        observation = _unknown(f"COMPILE_EXIT_{compile_process['exit_code']}")
        return {"job": job, "compile": compile_receipt, "pdfinfo": None, "observation": observation}
    if not output_path.is_file():
        observation = _unknown("OUTPUT_MISSING")
        return {"job": job, "compile": compile_receipt, "pdfinfo": None, "observation": observation}
    if output_path.stat().st_size > FILE_SIZE_BYTES:
        observation = _unknown("OUTPUT_FILE_LIMIT")
        return {"job": job, "compile": compile_receipt, "pdfinfo": None, "observation": observation}

    info_process = _bounded_process([str(pdfinfo), str(output_path)])
    info_receipt = _receipt(info_process)
    if info_process["reason_code"] is not None:
        observation = _unknown(info_process["reason_code"])
    elif info_process["exit_code"] != 0:
        observation = _unknown(f"PDFINFO_EXIT_{info_process['exit_code']}")
    else:
        observation = OBSERVER.parse_tagged_field(info_process["stdout"])
    return {
        "job": job,
        "compile": compile_receipt,
        "pdfinfo": info_receipt,
        "output_size": output_path.stat().st_size,
        "output_sha256": sha256_path(output_path),
        "observation": observation,
    }


def execute(args: argparse.Namespace) -> dict[str, Any]:
    binaries = {"vanila": Path(args.vanila).resolve(), "cristalino": Path(args.cristalino).resolve()}
    pdfinfo = Path(args.pdfinfo).resolve()
    identities = {
        "vanila": sha256_path(binaries["vanila"]),
        "cristalino": sha256_path(binaries["cristalino"]),
        "pdfinfo": sha256_path(pdfinfo),
    }
    if identities != EXPECTED_HASHES:
        raise ValueError(f"frozen identity mismatch: {identities}")

    with tempfile.TemporaryDirectory(prefix="p1369-s02.") as directory:
        root = Path(directory)
        rows = [execute_job(job, binaries, pdfinfo, root) for job in planned_jobs()]

    indexed = {
        (row["job"]["product"], row["job"]["profile"], row["job"]["case"]): row
        for row in rows
    }
    profiles = []
    for profile in PROFILES:
        product_verdicts = {}
        for product in PRODUCTS:
            observations = [indexed[(product, profile, case)]["observation"] for case in CASES]
            product_verdicts[product] = OBSERVER.classify_s02(*observations)
        profiles.append(
            {
                "profile": profile,
                "products": product_verdicts,
                "bilateral": classify_bilateral(
                    product_verdicts["vanila"], product_verdicts["cristalino"]
                ),
            }
        )

    counts: dict[str, int] = {}
    for item in profiles:
        counts[item["bilateral"]] = counts.get(item["bilateral"], 0) + 1
    return {
        "schema": "p1369-s02-bilateral/v1",
        "created_at": datetime.now(timezone.utc).isoformat(),
        "identities": identities,
        "limits": {
            "parallel_processes": MAX_PARALLEL_PROCESSES,
            "timeout_seconds": TIMEOUT_SECONDS,
            "address_space_bytes": ADDRESS_SPACE_BYTES,
            "file_size_bytes": FILE_SIZE_BYTES,
            "cpu_seconds": CPU_SECONDS,
            "channel_read_bytes": CHANNEL_READ_BYTES,
        },
        "job_count": len(rows),
        "pdfinfo_count": sum(row["pdfinfo"] is not None for row in rows),
        "counts": counts,
        "profiles": profiles,
        "rows": rows,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanila", required=True)
    parser.add_argument("--cristalino", required=True)
    parser.add_argument("--pdfinfo", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    result = execute(args)
    output = Path(args.output)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
