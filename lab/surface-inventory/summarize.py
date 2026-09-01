#!/usr/bin/env python3
"""Produz o ledger canônico e as listas P1282 para entrega ao P1283."""

from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import pathlib
import subprocess
from collections import defaultdict

import merge


def load(path: pathlib.Path):
    return json.loads(path.read_text())


def sha256(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_output(*args: str) -> str:
    run = subprocess.run(
        ["git", *args],
        cwd=pathlib.Path(__file__).resolve().parents[2],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    return run.stdout.rstrip()


def entry_index(payload: dict) -> dict[str, dict]:
    entries = payload["entries"] + payload.get("blocked_by_ancestor", [])
    return {entry["display_path"]: entry for entry in entries}


def aliases(entries: list[dict], side: str) -> list[dict]:
    groups = defaultdict(list)
    for entry in entries:
        symbol = entry[side].get("symbol")
        if symbol:
            groups[json.dumps(symbol, ensure_ascii=False, sort_keys=True)].append(
                entry["display_path"]
            )
    return [
        {"paths": sorted(paths), "identity": json.loads(identity)}
        for identity, paths in sorted(groups.items())
        if len(paths) > 1
    ]


def family_payload(payload: dict, family: str) -> dict:
    entries = [
        entry
        for entry in payload["entries"]
        if entry["family"] == family
    ]
    categories = {name: [] for name in ("modules", "functions", "symbols", "others")}
    variants = []
    for entry in entries:
        kinds = {entry[side]["kind"] for side in ("vanilla", "crystalline")}
        if "module" in kinds:
            category = "modules"
        elif "function" in kinds:
            category = "functions"
        elif "symbol" in kinds:
            category = "symbols"
        else:
            category = "others"
        categories[category].append(
            {
                "path": entry["display_path"],
                "classification": entry["classification"],
                "vanilla_kind": entry["vanilla"]["kind"],
                "crystalline_kind": entry["crystalline"]["kind"],
            }
        )
        if category == "symbols":
            variants.append(
                {
                    "path": entry["display_path"],
                    "classification": entry["classification"],
                    "vanilla": (entry["vanilla"].get("symbol") or {}).get(
                        "variants", []
                    ),
                    "crystalline": (entry["crystalline"].get("symbol") or {}).get(
                        "variants", []
                    ),
                }
            )
    return {
        **categories,
        "aliases": {
            "vanilla": aliases(entries, "vanilla"),
            "crystalline": aliases(entries, "crystalline"),
        },
        "variants": variants,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanilla-default", type=pathlib.Path, required=True)
    parser.add_argument("--crystalline-default", type=pathlib.Path, required=True)
    parser.add_argument("--vanilla-html", type=pathlib.Path, required=True)
    parser.add_argument("--crystalline-html", type=pathlib.Path, required=True)
    parser.add_argument("--inventory-default", type=pathlib.Path, required=True)
    parser.add_argument("--inventory-html", type=pathlib.Path, required=True)
    parser.add_argument("--probes-default", type=pathlib.Path, required=True)
    parser.add_argument("--probes-html", type=pathlib.Path, required=True)
    parser.add_argument("--seeds", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    args = parser.parse_args()

    vd, cd = load(args.vanilla_default), load(args.crystalline_default)
    vh, ch = load(args.vanilla_html), load(args.crystalline_html)
    default, html = load(args.inventory_default), load(args.inventory_html)
    probes_default, probes_html = load(args.probes_default), load(args.probes_html)
    seeds = load(args.seeds)
    default_index, html_index = entry_index(default), entry_index(html)

    ledger = merge.feature_ledger(
        default_vanilla=vd["entries"],
        default_crystalline=cd["entries"],
        html_vanilla=vh["entries"],
        html_crystalline=ch["entries"],
    )
    extra_adjudication = []
    for seed in sorted(set(seeds)):
        extra_adjudication.append(
            {
                "path": seed,
                "default": (
                    default_index.get(seed, {}).get("classification", "BILATERALLY_ABSENT")
                ),
                "html": html_index.get(seed, {}).get(
                    "classification", "BILATERALLY_ABSENT"
                ),
                "automatic_credit": False,
                "automatic_removal": False,
            }
        )

    inputs = {
        name: {"sha256": sha256(path)}
        for name, path in {
            "vanilla_default": args.vanilla_default,
            "crystalline_default": args.crystalline_default,
            "vanilla_html": args.vanilla_html,
            "crystalline_html": args.crystalline_html,
            "inventory_default": args.inventory_default,
            "inventory_html": args.inventory_html,
            "probes_default": args.probes_default,
            "probes_html": args.probes_html,
            "extra_seeds": args.seeds,
        }.items()
    }
    payload = {
        "schema_version": "p1282-summary-v1",
        "measurement": "path inventory counts; not a language-parity percentage",
        "inputs": inputs,
        "provenance": {
            "measured_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "git_head": git_output("rev-parse", "HEAD"),
            "git_status_short": git_output("status", "--short"),
            "git_diff_head_stat": git_output("diff", "HEAD", "--stat"),
            "vanilla_product_sha256": vd["product_sha256"],
            "crystalline_product_sha256": cd["product_sha256"],
        },
        "profiles": {
            "default": {
                "features": [],
                "counts": default["counts"],
                "blocked_by_ancestor_count": default["blocked_by_ancestor_count"],
                "probe_counts": probes_default["counts"],
            },
            "html": {
                "features": ["html"],
                "counts": html["counts"],
                "blocked_by_ancestor_count": html["blocked_by_ancestor_count"],
                "probe_counts": probes_html["counts"],
            },
        },
        "feature_ledger": ledger,
        "historical_extra_seed_adjudication": extra_adjudication,
        "p1283_families": {
            family: family_payload(html, family) for family in ("math", "sym", "emoji")
        },
    }
    args.output.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    )
    print(json.dumps(payload["profiles"], ensure_ascii=False, sort_keys=True))


if __name__ == "__main__":
    main()
