#!/usr/bin/env python3
"""Classificador bilateral P1282 para catálogos runtime frescos."""

from __future__ import annotations

import argparse
import json
import pathlib
from collections import Counter


SCHEMA_VERSION = "p1282-v1"
CATALOG_SCHEMA_VERSION = "p1282-catalog-v1"
EXPECTED_VANILLA_SHA256 = (
    "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f"
)
EXPECTED_CRYSTALLINE_SHA256 = (
    "0a4d71415b4e3ba47ed9c50f08b612a61fd14e1e538790ea4e51245d3019ce3f"
)
EXPECTED_VANILLA_REVISION = "586e1bd43"
CLASSES = (
    "MATCH",
    "MISSING_BINDING",
    "MISSING_MEMBER",
    "WRONG_KIND",
    "UNVERIFIED_METADATA",
    "EXTRA_BINDING",
    "UNKNOWN",
)


def absent_side() -> dict:
    return {
        "present": False,
        "availability": "absent",
        "kind": None,
        "params": None,
        "source": None,
        "owner_path": None,
        "owner_kind": None,
        "slot_kind": None,
        "access_form": None,
        "structural_verified": False,
        "symbol": None,
    }


def normalize_side(raw: dict | None) -> dict:
    if not raw:
        return absent_side()
    side = absent_side()
    side.update(raw)
    side["availability"] = raw.get(
        "availability", "active" if raw.get("present") else "absent"
    )
    return side


def owner_is_compatible(owner_path: str | None, owners: dict) -> bool:
    if not owner_path or owner_path not in owners:
        return False
    vanilla, crystalline = owners[owner_path]
    return (
        vanilla["present"]
        and crystalline["present"]
        and vanilla["kind"] == crystalline["kind"]
    )


def metadata_result(vanilla: dict, crystalline: dict) -> tuple[str | None, str | None]:
    if not vanilla.get("structural_verified", False) or not crystalline.get(
        "structural_verified", False
    ):
        return "MISSING_OBSERVATION", "access form is not independently observed"

    if vanilla["kind"] == "function":
        if vanilla.get("params") is None or crystalline.get("params") is None:
            return "MISSING_OBSERVATION", "function parameters are not observed bilaterally"
        if vanilla["params"] != crystalline["params"]:
            return "VERIFIED_DIFFERENCE", "function parameter metadata differs"

    if vanilla["kind"] == "symbol":
        if vanilla.get("symbol") is None or crystalline.get("symbol") is None:
            return "MISSING_OBSERVATION", "symbol identity or variants are not observed"
        if vanilla["symbol"] != crystalline["symbol"]:
            return "VERIFIED_DIFFERENCE", "symbol value or variants differ"

    return None, None


def classify_entry(
    profile: str,
    path: str,
    vanilla_raw: dict,
    crystalline_raw: dict,
    *,
    owners: dict | None = None,
) -> dict:
    vanilla = normalize_side(vanilla_raw)
    crystalline = normalize_side(crystalline_raw)
    owners = owners or {}
    result = {
        "profile": profile,
        "path_segments": path.split("."),
        "display_path": path,
        "owner_path": vanilla.get("owner_path") or crystalline.get("owner_path"),
        "slot_kind": vanilla.get("slot_kind") or crystalline.get("slot_kind"),
        "access_form": vanilla.get("access_form") or crystalline.get("access_form"),
        "family": path.split(".", 1)[0],
        "vanilla": vanilla,
        "crystalline": crystalline,
        "classification": None,
        "metadata_state": None,
        "unknown_reason": None,
        "extra_subtype": None,
        "inference": None,
        "refutation": None,
    }

    if vanilla["availability"] == "unknown" or crystalline["availability"] == "unknown":
        result["classification"] = "UNKNOWN"
        result["unknown_reason"] = "enumerator_failure"
        return result

    if not vanilla["present"] and not crystalline["present"]:
        result["classification"] = "UNKNOWN"
        result["unknown_reason"] = "bilateral_absence_without_feature_evidence"
        return result

    if vanilla["present"] and not crystalline["present"]:
        if vanilla.get("slot_kind") == "global_binding":
            result["classification"] = "MISSING_BINDING"
        elif owner_is_compatible(vanilla.get("owner_path"), owners):
            result["classification"] = "MISSING_MEMBER"
        else:
            result["classification"] = "UNKNOWN"
            result["unknown_reason"] = "blocked_by_ancestor"
        return result

    if crystalline["present"] and not vanilla["present"]:
        result["classification"] = "EXTRA_BINDING"
        result["extra_subtype"] = crystalline.get("slot_kind") or "unknown"
        return result

    vanilla_key = (
        vanilla.get("kind"),
        vanilla.get("owner_kind"),
        vanilla.get("slot_kind"),
        vanilla.get("access_form"),
    )
    crystalline_key = (
        crystalline.get("kind"),
        crystalline.get("owner_kind"),
        crystalline.get("slot_kind"),
        crystalline.get("access_form"),
    )
    if vanilla_key != crystalline_key:
        if not vanilla.get("structural_verified", False) or not crystalline.get(
            "structural_verified", False
        ):
            result["classification"] = "UNVERIFIED_METADATA"
            result["metadata_state"] = "MISSING_OBSERVATION"
            result["inference"] = (
                "kind/access-form candidate differs without bilateral structural proof"
            )
            result["refutation"] = (
                "an independent receiver/static probe establishing the access form"
            )
        else:
            result["classification"] = "WRONG_KIND"
        return result

    metadata_state, reason = metadata_result(vanilla, crystalline)
    if metadata_state:
        result["classification"] = "UNVERIFIED_METADATA"
        result["metadata_state"] = metadata_state
        result["inference"] = reason
        result["refutation"] = "bilateral metadata receipts proving equality"
        return result

    result["classification"] = "MATCH"
    return result


def merge_catalogs(profile: str, vanilla_raw: dict, crystalline_raw: dict) -> dict:
    vanilla = {path: normalize_side(value) for path, value in vanilla_raw.items()}
    crystalline = {
        path: normalize_side(value) for path, value in crystalline_raw.items()
    }
    owners = {
        path: (vanilla.get(path, absent_side()), crystalline.get(path, absent_side()))
        for path in sorted(set(vanilla) | set(crystalline))
    }
    classified = [
        classify_entry(
            profile,
            path,
            vanilla.get(path, absent_side()),
            crystalline.get(path, absent_side()),
            owners=owners,
        )
        for path in sorted(set(vanilla) | set(crystalline))
    ]
    blocked = [
        entry
        for entry in classified
        if entry["classification"] == "UNKNOWN"
        and entry["unknown_reason"] == "blocked_by_ancestor"
    ]
    entries = [entry for entry in classified if entry not in blocked]
    counts = Counter(entry["classification"] for entry in entries)
    return {
        "schema_version": SCHEMA_VERSION,
        "profile": profile,
        "features": [] if profile == "default" else ["html"],
        "counts": {name: counts.get(name, 0) for name in CLASSES},
        "blocked_by_ancestor_count": len(blocked),
        "blocked_by_ancestor": blocked,
        "entries": entries,
    }


def provenance_errors(
    profile: str,
    vanilla: dict,
    crystalline: dict,
    *,
    expected_crystalline_sha256: str = EXPECTED_CRYSTALLINE_SHA256,
) -> list[str]:
    expected_features = [] if profile == "default" else ["html"]
    errors = []
    for name, payload in (("vanilla", vanilla), ("crystalline", crystalline)):
        if payload.get("schema_version") != CATALOG_SCHEMA_VERSION:
            errors.append(f"{name} catalog schema")
        if payload.get("side") != name:
            errors.append(f"{name} side identity")
        if payload.get("profile") != profile:
            errors.append(f"{name} profile")
        if payload.get("features") != expected_features:
            errors.append(f"{name} feature set")
    if vanilla.get("product_sha256") != EXPECTED_VANILLA_SHA256:
        errors.append("vanilla product SHA")
    if crystalline.get("product_sha256") != expected_crystalline_sha256:
        errors.append("crystalline product SHA")
    if vanilla.get("vanilla_revision") != EXPECTED_VANILLA_REVISION:
        errors.append("vanilla revision")
    return errors


def invalid_provenance_payload(profile: str, errors: list[str]) -> dict:
    entry = classify_entry(
        profile,
        "__p1282_invalid_provenance__",
        {"availability": "unknown", "present": False},
        {"availability": "unknown", "present": False},
    )
    entry["unknown_reason"] = "invalid_provenance"
    return {
        "schema_version": SCHEMA_VERSION,
        "profile": profile,
        "features": [] if profile == "default" else ["html"],
        "counts": {name: int(name == "UNKNOWN") for name in CLASSES},
        "blocked_by_ancestor_count": 0,
        "blocked_by_ancestor": [],
        "provenance_errors": errors,
        "entries": [entry],
    }


def merge_catalog_payloads(
    profile: str,
    vanilla: dict,
    crystalline: dict,
    *,
    expected_crystalline_sha256: str = EXPECTED_CRYSTALLINE_SHA256,
) -> dict:
    errors = provenance_errors(
        profile,
        vanilla,
        crystalline,
        expected_crystalline_sha256=expected_crystalline_sha256,
    )
    if errors:
        return invalid_provenance_payload(profile, errors)
    payload = merge_catalogs(profile, vanilla["entries"], crystalline["entries"])
    payload["catalog_provenance"] = {
        "vanilla": {key: value for key, value in vanilla.items() if key != "entries"},
        "crystalline": {
            key: value for key, value in crystalline.items() if key != "entries"
        },
    }
    payload["provenance_errors"] = []
    return payload


def validate_profiles(vanilla: dict, crystalline: dict) -> None:
    if vanilla.get("profile") != crystalline.get("profile"):
        raise ValueError("profile names must be symmetric")
    if vanilla.get("features", []) != crystalline.get("features", []):
        raise ValueError("HTML profile must be symmetric")


def feature_ledger(
    *,
    default_vanilla: dict,
    default_crystalline: dict,
    html_vanilla: dict,
    html_crystalline: dict,
) -> dict:
    paths = sorted(
        set(default_vanilla)
        | set(default_crystalline)
        | set(html_vanilla)
        | set(html_crystalline)
    )
    ledger = {}
    for path in paths:
        default_active = path in default_vanilla or path in default_crystalline
        html_bilateral = path in html_vanilla and path in html_crystalline
        if not default_active and html_bilateral:
            default_state = "disabled_by_profile"
        elif path in default_vanilla and path in default_crystalline:
            default_state = "active_bilateral"
        else:
            default_state = "active_asymmetric"
        ledger[path] = {
            "default": default_state,
            "html": "active_bilateral" if html_bilateral else "active_asymmetric",
        }
    return ledger


def load_catalog(path: pathlib.Path) -> dict:
    return json.loads(path.read_text())


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("vanilla", type=pathlib.Path)
    parser.add_argument("crystalline", type=pathlib.Path)
    parser.add_argument("output", type=pathlib.Path)
    parser.add_argument("--profile", choices=("default", "html"), required=True)
    parser.add_argument(
        "--expected-crystalline-sha256",
        default=EXPECTED_CRYSTALLINE_SHA256,
        help="produto cristalino pinado para esta execução (default: baseline P1282)",
    )
    args = parser.parse_args()

    payload = merge_catalog_payloads(
        args.profile,
        load_catalog(args.vanilla),
        load_catalog(args.crystalline),
        expected_crystalline_sha256=args.expected_crystalline_sha256,
    )
    args.output.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    )
    print(json.dumps(payload["counts"], ensure_ascii=False, sort_keys=True))


if __name__ == "__main__":
    main()
