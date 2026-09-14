#!/usr/bin/env python3
"""Observador binding-free e fail-closed do vetor S02 de PDF tagueado."""

from __future__ import annotations

from typing import Any


Observation = dict[str, Any]


def _unknown(reason_code: str) -> Observation:
    return {"complete": False, "value": None, "reason_code": reason_code}


def parse_tagged_field(stdout: bytes) -> Observation:
    """Normalize exactly one public ``pdfinfo`` Tagged field."""

    try:
        text = stdout.decode("utf-8", errors="strict")
    except UnicodeDecodeError:
        return _unknown("INVALID_UTF8")

    values: list[str] = []
    for line in text.splitlines():
        key, separator, value = line.strip().partition(":")
        if separator and key.strip().casefold() == "tagged":
            values.append(value.strip().casefold())

    if len(values) != 1:
        return _unknown("TAGGED_FIELD_MISSING_OR_DUPLICATE")
    if values[0] == "yes":
        return {"complete": True, "value": True, "reason_code": None}
    if values[0] == "no":
        return {"complete": True, "value": False, "reason_code": None}
    return _unknown("TAGGED_VALUE_UNSUPPORTED")


def classify_s02(
    top_level: Observation,
    document_over_cli: Observation,
    cli_only: Observation,
) -> dict[str, Any]:
    """Classify the isolated S02 precedence vector without implicit success."""

    observations = (top_level, document_over_cli, cli_only)
    if any(item.get("complete") is not True for item in observations):
        return {
            "classification": "Unknown",
            "vector": None,
            "reason_code": "INCOMPLETE_OBSERVATION",
        }

    vector = [item.get("value") for item in observations]
    if any(type(value) is not bool for value in vector):
        return {
            "classification": "Unknown",
            "vector": None,
            "reason_code": "NON_BOOLEAN_OBSERVATION",
        }

    expected = [True, False, True]
    return {
        "classification": "Preserved" if vector == expected else "Violated",
        "vector": vector,
        "reason_code": None if vector == expected else "S02_VECTOR_MISMATCH",
    }
