#!/usr/bin/env python3
"""Executable P1430 certification-policy boundary for the M21 refinement.

The module is intentionally independent of the product implementation.  It
turns public case/mutation records into the decision fields frozen by the R2
contract.  Unknown is fail-closed and never receives acceptance or mutation
credit.
"""

from __future__ import annotations

import argparse
import json
import sys
from typing import Any


CLASSIFICATIONS = frozenset({"Preserved", "Violated", "Unknown"})


class PolicyInputError(ValueError):
    """The supplied public record is malformed or outside the policy domain."""


def _classification(record: dict[str, Any], field: str) -> str:
    value = record.get(field)
    if value not in CLASSIFICATIONS:
        raise PolicyInputError(f"{field} must be Preserved, Violated, or Unknown")
    return value


def evaluate_case(record: dict[str, Any]) -> dict[str, Any]:
    """Classify one public P/N/U case and expose every certification decision."""

    case_id = str(record.get("case_id", ""))
    category = str(record.get("category", ""))
    mandatory = bool(record.get("mandatory", False))
    expected = _classification(record, "expected_classification")
    actual = _classification(record, "actual_classification")
    reason_code = str(record.get("reason_code", ""))

    classification_matches = actual == expected
    acceptance_credit = int(classification_matches and actual != "Unknown")
    blocks_gate = mandatory and (actual == "Unknown" or not classification_matches)

    return {
        "record_kind": "case",
        "record_id": case_id,
        "category": category,
        "actual_classification": actual,
        "classification_matches": classification_matches,
        "acceptance_credit": acceptance_credit,
        "mutation_credit": 0,
        "score_numerator_delta": 0,
        "score_denominator_delta": 0,
        "blocks_gate": blocks_gate,
        "reason_code": reason_code,
    }


def evaluate_mutation(record: dict[str, Any]) -> dict[str, Any]:
    """Classify one mutation result for the mutation-score boundary."""

    mutation_id = str(record.get("mutation_id", ""))
    valid = bool(record.get("valid", False))
    classification = _classification(record, "classification")
    attributable = bool(record.get("attributable", False))
    reason_code = str(record.get("reason_code", ""))

    # M21's protected executed branch: valid Unknown remains in the
    # denominator, receives no credit/numerator membership and blocks the gate.
    if classification == "Unknown":
        mutation_credit = 0
        score_numerator_delta = 0
        score_denominator_delta = int(valid)
        blocks_gate = valid
        classification_matches = False
    else:
        killed = valid and classification == "Violated" and attributable
        mutation_credit = int(killed)
        score_numerator_delta = int(killed)
        score_denominator_delta = int(valid)
        blocks_gate = valid and not killed
        classification_matches = killed

    return {
        "record_kind": "mutation",
        "record_id": mutation_id,
        "actual_classification": classification,
        "classification_matches": classification_matches,
        "acceptance_credit": 0,
        "mutation_credit": mutation_credit,
        "score_numerator_delta": score_numerator_delta,
        "score_denominator_delta": score_denominator_delta,
        "blocks_gate": blocks_gate,
        "reason_code": reason_code,
    }


def score_mutations(records: list[dict[str, Any]]) -> dict[str, Any]:
    """Aggregate only executed mutation decisions; valid Unknown blocks."""

    decisions = [evaluate_mutation(record) for record in records]
    killed = sum(item["score_numerator_delta"] for item in decisions)
    valid = sum(item["score_denominator_delta"] for item in decisions)
    return {
        "killed": killed,
        "valid": valid,
        "mutation_score": (killed / valid) if valid else None,
        "blocks_gate": any(item["blocks_gate"] for item in decisions),
        "mutation_credit": sum(item["mutation_credit"] for item in decisions),
        "decisions": decisions,
    }


def _read_payload(raw: str | None) -> Any:
    text = raw if raw is not None else sys.stdin.read()
    try:
        return json.loads(text)
    except json.JSONDecodeError as error:
        raise PolicyInputError(f"invalid JSON: {error}") from error


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("case", "mutation", "score"))
    parser.add_argument("--json", dest="payload")
    args = parser.parse_args(argv)
    try:
        payload = _read_payload(args.payload)
        if args.mode == "case":
            result = evaluate_case(payload)
        elif args.mode == "mutation":
            result = evaluate_mutation(payload)
        else:
            if not isinstance(payload, list):
                raise PolicyInputError("score payload must be a JSON array")
            result = score_mutations(payload)
    except (PolicyInputError, TypeError) as error:
        print(json.dumps({"status": "UNKNOWN", "error": str(error)}, sort_keys=True))
        return 2
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

