#!/usr/bin/env python3
"""Retifica e reclassifica o vetor S02 sem repetir medições externas."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any


INPUT_SHA256 = "1e6c38c2c7fe387e4877d18eb6890e11a25d8572e3a39483bbaf3261dded7b05"
MAX_BYTES = 65536
PRODUCTS = ("vanila", "cristalino")
PROFILES = ("default", "html", "a11y-extras", "html+a11y")
CASES = ("top_level", "document_over_cli", "cli_only")
EXPECTED_VECTOR = [True, True, True]
EXPECTED_IDENTITIES = {
    "vanila": "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f",
    "cristalino": "60851811dca31074eca38f5d4e397ce04a89eccb7bf49c9d6e28f6759ba761f9",
    "pdfinfo": "fee70ade670fb025343aca2b5c3a2aacacb8ed9edce1b716b233b5de924b6bf5",
}


def _unknown(reason_code: str) -> dict[str, Any]:
    return {"verdict": "Unknown", "reason_code": reason_code, "counts": {"Unknown": 4}}


def classify_vector(observations: list[dict[str, Any]]) -> dict[str, Any]:
    if len(observations) != len(CASES):
        return {"classification": "Unknown", "vector": None, "reason_code": "VECTOR_CARDINALITY"}
    if any(item.get("complete") is not True for item in observations):
        return {"classification": "Unknown", "vector": None, "reason_code": "INCOMPLETE_OBSERVATION"}

    vector = [item.get("value") for item in observations]
    if any(type(value) is not bool for value in vector):
        return {"classification": "Unknown", "vector": None, "reason_code": "NON_BOOLEAN_OBSERVATION"}
    return {
        "classification": "Preserved" if vector == EXPECTED_VECTOR else "Violated",
        "vector": vector,
        "reason_code": None if vector == EXPECTED_VECTOR else "S02_VECTOR_MISMATCH",
    }


def _bilateral(left: dict[str, Any], right: dict[str, Any]) -> str:
    if "Unknown" in (left["classification"], right["classification"]):
        return "Unknown"
    if (
        left["classification"] == "Preserved"
        and right["classification"] == "Preserved"
        and left["vector"] == EXPECTED_VECTOR
        and right["vector"] == EXPECTED_VECTOR
    ):
        return "MATCH"
    return "DIFFERENCE"


def reclassify(payload: dict[str, Any]) -> dict[str, Any]:
    if payload.get("schema") != "p1369-s02-bilateral/v1":
        return _unknown("INPUT_SCHEMA")
    if payload.get("identities") != EXPECTED_IDENTITIES:
        return _unknown("IDENTITY_MISMATCH")
    rows = payload.get("rows")
    if not isinstance(rows, list) or payload.get("job_count") != 24 or len(rows) != 24:
        return _unknown("JOB_CARDINALITY")

    expected = {
        (product, profile, case)
        for product in PRODUCTS
        for profile in PROFILES
        for case in CASES
    }
    indexed: dict[tuple[str, str, str], dict[str, Any]] = {}
    try:
        for row in rows:
            job = row["job"]
            key = (job["product"], job["profile"], job["case"])
            if key in indexed:
                return _unknown("DUPLICATE_JOB")
            observation = row["observation"]
            if not isinstance(observation, dict):
                return _unknown("MALFORMED_OBSERVATION")
            indexed[key] = observation
    except (KeyError, TypeError):
        return _unknown("MALFORMED_ROW")
    if set(indexed) != expected:
        return _unknown("JOB_IDENTITY_SET")

    profiles = []
    counts: dict[str, int] = {}
    for profile in PROFILES:
        products = {}
        for product in PRODUCTS:
            observations = [indexed[(product, profile, case)] for case in CASES]
            products[product] = classify_vector(observations)
        bilateral = _bilateral(products["vanila"], products["cristalino"])
        counts[bilateral] = counts.get(bilateral, 0) + 1
        profiles.append({"profile": profile, "products": products, "bilateral": bilateral})

    if counts == {"MATCH": 4}:
        verdict = "MATCH"
        reason_code = None
    elif counts.get("Unknown", 0):
        verdict = "Unknown"
        reason_code = "INCOMPLETE_PROFILE"
    else:
        verdict = "DIFFERENCE"
        reason_code = "BILATERAL_VECTOR_MISMATCH"
    return {
        "schema": "p1370-s02-reclassification/v1",
        "verdict": verdict,
        "reason_code": reason_code,
        "retified_expected_vector": EXPECTED_VECTOR,
        "counts": counts,
        "profiles": profiles,
        "ignored_input_fields": ["profiles"],
    }


def reclassify_file(path: Path) -> dict[str, Any]:
    try:
        size = path.stat().st_size
    except OSError:
        return _unknown("INPUT_UNREADABLE")
    if size > MAX_BYTES:
        return _unknown("INPUT_SIZE_LIMIT")
    try:
        raw = path.read_bytes()
    except OSError:
        return _unknown("INPUT_UNREADABLE")
    if hashlib.sha256(raw).hexdigest() != INPUT_SHA256:
        return _unknown("INPUT_HASH_MISMATCH")
    try:
        payload = json.loads(raw)
    except (UnicodeDecodeError, json.JSONDecodeError):
        return _unknown("INPUT_JSON_INVALID")
    if not isinstance(payload, dict):
        return _unknown("INPUT_NOT_OBJECT")
    result = reclassify(payload)
    result["input_sha256"] = INPUT_SHA256
    result["input_bytes"] = size
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    result = reclassify_file(Path(args.input))
    encoded = (json.dumps(result, ensure_ascii=False, indent=2) + "\n").encode("utf-8")
    if len(encoded) > MAX_BYTES:
        raise RuntimeError("P1370 output exceeds 64 KiB")
    Path(args.output).write_bytes(encoded)


if __name__ == "__main__":
    main()
