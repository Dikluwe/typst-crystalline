#!/usr/bin/env python3
"""Oracle-owned non-adversarial adapter for P1350 R2 focal controls."""

from __future__ import annotations

from typing import Any


CLASSIFICATION_KEYS = ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"]
META_KEYS = ["case_id", "closed_world", "meta_evidence", "meta_status", "schema"]


def execute_in_executor(case: dict[str, Any], route: dict[str, Any], context: dict[str, Any]) -> dict[str, Any]:
    if route["operation_id"] != case["operation"] or route["route"] not in ("INHERITED_P1348_P1347_P1346", "P1350_LOCAL"):
        raise RuntimeError("route disagreement")
    return {"case_id": case["case_id"], "classification": "Preserved", "closed_world": {"keys": CLASSIFICATION_KEYS, "rule": "P1349 classification preserved through the distinct P1350 R2 focal route."}, "evidence": {"language_case_id": case["case_id"], "operation_id": route["operation_id"], "oracle_selftest": True}, "reason_code": "PRESERVED", "schema": "p1349-classification-result-r1"}


def execute_meta_in_executor(case: dict[str, Any], context: dict[str, Any]) -> dict[str, Any]:
    return {"case_id": case["case_id"], "closed_world": {"keys": META_KEYS, "rule": "P1349 meta observation preserved outside semantic score."}, "meta_evidence": {"oracle_selftest": True, "operation": case["operation"]}, "meta_status": "PASS", "schema": "p1349-meta-result-r1"}
