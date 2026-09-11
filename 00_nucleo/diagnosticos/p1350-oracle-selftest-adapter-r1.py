#!/usr/bin/env python3
"""Non-adversarial adapter used only by the P1350 oracle self-focal."""

from __future__ import annotations

from typing import Any


KEYS = ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"]


def execute_in_executor(case: dict[str, Any], route: dict[str, Any], context: dict[str, Any]) -> dict[str, Any]:
    if route != {"case_id": case["case_id"], "operation": case["operation"], "route": route["route"]}:
        raise RuntimeError("route disagreement")
    if type(context.get("authority_root_sha256")) is not str:
        raise RuntimeError("authority root absent")
    if case["operation"] == "raise_exception":
        raise RuntimeError("self-focal adapter exception")
    if case["operation"] != "canonical":
        raise RuntimeError("self-focal operation")
    return {"case_id": case["case_id"], "classification": "Preserved", "closed_world": {"keys": KEYS, "rule": "P1349 semantic classification preserved through P1350 transport."}, "evidence": {"oracle_selftest": True, "route": route["route"]}, "reason_code": "PRESERVED", "schema": "p1349-classification-result-r1"}
