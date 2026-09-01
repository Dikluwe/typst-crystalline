#!/usr/bin/env python3
"""Fecha P1283 por família e entrega o residual, sem percentagem global."""

from __future__ import annotations

import argparse
import collections
import datetime
import hashlib
import json
import pathlib
import subprocess


SELECTED = {"math", "sym", "emoji"}
CLASSES = (
    "MATCH",
    "MISSING_BINDING",
    "MISSING_MEMBER",
    "WRONG_KIND",
    "UNVERIFIED_METADATA",
    "EXTRA_BINDING",
    "UNKNOWN",
)


def load(path: pathlib.Path) -> dict:
    return json.loads(path.read_text())


def sha256(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def class_counts(entries: list[dict]) -> dict[str, int]:
    counts = collections.Counter(entry["classification"] for entry in entries)
    return {name: counts.get(name, 0) for name in CLASSES}


def family_counts(payload: dict, family: str) -> dict:
    entries = [entry for entry in payload["entries"] if entry["family"] == family]
    blocked = [
        entry
        for entry in payload.get("blocked_by_ancestor", [])
        if entry["family"] == family
    ]
    return {
        "classified": class_counts(entries),
        "blocked_by_ancestor": len(blocked),
    }


def run_text(*argv: str) -> str:
    return subprocess.run(
        argv, check=True, text=True, stdout=subprocess.PIPE
    ).stdout.rstrip()


def main() -> None:
    parser = argparse.ArgumentParser()
    for phase in ("before", "after"):
        for profile in ("default", "html"):
            parser.add_argument(
                f"--{phase}-{profile}", type=pathlib.Path, required=True
            )
    parser.add_argument("--probes-default", type=pathlib.Path, required=True)
    parser.add_argument("--probes-html", type=pathlib.Path, required=True)
    parser.add_argument("--summary", type=pathlib.Path, required=True)
    parser.add_argument("--residual", type=pathlib.Path, required=True)
    args = parser.parse_args()

    paths = {
        phase: {
            profile: getattr(args, f"{phase}_{profile}")
            for profile in ("default", "html")
        }
        for phase in ("before", "after")
    }
    data = {
        phase: {profile: load(path) for profile, path in profiles.items()}
        for phase, profiles in paths.items()
    }
    probes = {
        "default": load(args.probes_default),
        "html": load(args.probes_html),
    }

    head = run_text("git", "rev-parse", "HEAD")
    diff_stat = run_text("git", "diff", "HEAD", "--stat")
    status_short = run_text("git", "status", "--short")
    provenance = {
        "measured_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "head": head,
        "working_tree": "uncommitted" if status_short else "clean",
        "git_diff_head_stat": diff_stat.splitlines(),
        "git_status_short": status_short.splitlines(),
        "inputs": {
            str(path): {"sha256": sha256(path)}
            for profiles in paths.values()
            for path in profiles.values()
        }
        | {
            str(path): {"sha256": sha256(path)}
            for path in (args.probes_default, args.probes_html)
        },
    }

    selected = {
        profile: {
            family: {
                "before": family_counts(data["before"][profile], family),
                "after": family_counts(data["after"][profile], family),
            }
            for family in sorted(SELECTED)
        }
        for profile in ("default", "html")
    }

    summary = {
        "schema_version": "p1283-summary-v1",
        "measurement": "path inventory counts by selected family; not a parity percentage",
        "provenance": provenance,
        "selected_families": selected,
        "focal_probes": {
            profile: probes[profile]["counts"] for profile in ("default", "html")
        },
        "decisions": {
            "sym.registered": "preserved crystalline extension; no vanilla parity credit",
            "math.registered": "preserved mirrored crystalline extension; no vanilla parity credit",
            "variant_deprecations": "IntentionalVariantDeprecationDivergence",
            "math_non_declarative_functions": "outside C-P1283-v3; not credited as closed",
        },
    }

    residual_profiles = {}
    for profile in ("default", "html"):
        payload = data["after"][profile]
        entries = [
            entry
            for entry in payload["entries"] + payload.get("blocked_by_ancestor", [])
            if entry["family"] not in SELECTED
            and entry["classification"] != "MATCH"
        ]
        residual_profiles[profile] = {
            "features": [] if profile == "default" else ["html"],
            "counts": class_counts(entries),
            "entries": entries,
        }
    residual = {
        "schema_version": "p1283-residual-p1284-v1",
        "measurement": "outstanding non-match paths; math, sym and emoji excluded by P1283 delivery rule",
        "excluded_families": sorted(SELECTED),
        "provenance": provenance,
        "profiles": residual_profiles,
    }

    args.summary.write_text(
        json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    )
    args.residual.write_text(
        json.dumps(residual, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    )
    print(json.dumps({"selected_families": selected}, sort_keys=True))
    print(
        json.dumps(
            {
                profile: residual_profiles[profile]["counts"]
                for profile in ("default", "html")
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
