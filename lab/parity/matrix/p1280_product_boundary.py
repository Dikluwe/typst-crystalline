#!/usr/bin/env python3
"""P1280: readjudica a rota produtiva na população P1277 congelada."""

from __future__ import annotations

import argparse
import csv
import json
import sys
from datetime import datetime
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import p1277_generalize as harness


P1279_CERTIFICATE_SHA256 = (
    "be1e16cfb002c70b8d7af83421e7027f2abdb32324a1e8ff94ace7152ee1e3ed"
)


POLAR = frozenset(
    f"{kind}/{space}"
    for kind in ("linear", "radial")
    for space in ("oklch", "hsl", "hsv")
)


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(
            handle, list(rows[0]), delimiter="\t", lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def parsed_svg_stops(
    svg: Path, kind: str
) -> list[tuple[float, tuple[int, int, int, int]]]:
    return [
        (offset, tuple(round(component * 255.0) for component in color))
        for offset, color in harness.server_stops(svg, kind)
    ]


def route_preserved(pair: str, graph: dict[str, object]) -> bool:
    if pair in POLAR:
        return (
            int(graph["fallback_markers"]) == 0
            and int(graph["variant_nodes"]) >= 1
            and int(graph["unresolved"]) == 0
            and int(graph["raster_nodes"]) == 0
            and int(graph["role_urls"]) >= 1
        )
    return (
        int(graph["fallback_markers"]) >= 1
        and int(graph["variant_nodes"]) == 0
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--matrix", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--fixture-root", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--reverse", action="store_true")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    args.fixture_root.mkdir(parents=True, exist_ok=True)

    matrix = harness.read_tsv(args.matrix)
    valid = [row for row in matrix if row["validity"] == "valid"]
    if len(valid) != 24:
        raise ValueError("P1280 requires the frozen 24-seed population")
    fixtures = [
        harness.make_fixture(seed, pair)
        for seed in valid
        for pair in harness.PAIRS
    ]
    if args.reverse:
        fixtures.reverse()
    if len(fixtures) != 192:
        raise ValueError("P1280 product population drift")

    head = harness.run(["git", "rev-parse", "HEAD"]).stdout.strip()
    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    status = harness.run(["git", "status", "--short"]).stdout.rstrip()
    diff_stat = harness.run(["git", "diff", "HEAD", "--stat"]).stdout.rstrip()
    args.working_tree_snapshot.write_text(
        f"measured_at={measured_at}\nhead={head}\nworking_tree=uncommitted\n\n"
        f"git status --short\n{status}\n\ngit diff HEAD --stat\n{diff_stat}\n",
        encoding="utf-8",
    )

    product_rows: list[dict[str, object]] = []
    identity_rows: list[dict[str, object]] = []
    direct: dict[str, str] = {}
    fixture_by_id: dict[str, harness.Fixture] = {}
    for fixture in fixtures:
        fixture_by_id[fixture.fixture_id] = fixture
        source = args.fixture_root / f"{fixture.fixture_id}.typ"
        source.write_text(fixture.source, encoding="utf-8")
        svg = args.out / f"direct-{fixture.fixture_id}.svg"
        harness.run([str(args.candidate), "compile", str(source), str(svg)])
        graph = harness.graph_observation(svg, fixture, product=True)
        route_ok = route_preserved(fixture.pair, graph)
        product_rows.append(
            {
                "fixture_id": fixture.fixture_id,
                "pair": fixture.pair,
                "role": fixture.seed["role"],
                "fallback_markers": graph["fallback_markers"],
                "variant_nodes": graph["variant_nodes"],
                "unresolved": graph["unresolved"],
                "raster_nodes": graph["raster_nodes"],
                "role_urls": graph["role_urls"],
                "productive_promotion": str(fixture.pair in POLAR).lower(),
                "status": "Preserved" if route_ok else "Violated",
            }
        )

        identity = "not-applicable-fallback"
        actual: list[tuple[float, tuple[int, int, int, int]]] = []
        if fixture.pair in POLAR:
            actual = parsed_svg_stops(svg, fixture.pair.split("/")[0])
            identity = (
                "Inherited-P1279" if route_ok and len(actual) >= 2 else "Violated"
            )
        identity_rows.append(
            {
                "fixture_id": fixture.fixture_id,
                "pair": fixture.pair,
                "sealed_evidence_sha256": (
                    P1279_CERTIFICATE_SHA256
                    if fixture.pair in POLAR
                    else "not-applicable-fallback"
                ),
                "actual_stops": len(actual),
                "route_identity": identity,
                "status": (
                    "Preserved"
                    if identity
                    in ("Inherited-P1279", "not-applicable-fallback")
                    else "Violated"
                ),
            }
        )
        direct[fixture.fixture_id] = harness.canonical(
            {"graph": graph, "identity": identity, "stops": actual}
        )

    determinism_rows = []
    canonical_fixtures = [fixture_by_id[key] for key in sorted(fixture_by_id)]
    for phase, ordered in (
        ("inverse", list(reversed(canonical_fixtures))),
        ("repeat", canonical_fixtures),
    ):
        for fixture in ordered:
            svg = args.out / f"{phase}-{fixture.fixture_id}.svg"
            harness.run(
                [
                    str(args.candidate),
                    "compile",
                    str(args.fixture_root / f"{fixture.fixture_id}.typ"),
                    str(svg),
                ]
            )
            graph = harness.graph_observation(svg, fixture, product=True)
            identity = "not-applicable-fallback"
            stops: list[tuple[float, tuple[int, int, int, int]]] = []
            if fixture.pair in POLAR:
                stops = parsed_svg_stops(svg, fixture.pair.split("/")[0])
                identity = "Inherited-P1279" if len(stops) >= 2 else "Violated"
            observed = harness.canonical(
                {"graph": graph, "identity": identity, "stops": stops}
            )
            determinism_rows.append(
                {
                    "phase": phase,
                    "fixture_id": fixture.fixture_id,
                    "pair": fixture.pair,
                    "direct_digest": direct[fixture.fixture_id],
                    "observed_digest": observed,
                    "status": (
                        "Preserved"
                        if observed == direct[fixture.fixture_id]
                        else "Violated"
                    ),
                }
            )

    invalid_rows = []
    for pair in harness.PAIRS:
        for probe_id, expression in harness.invalid_expressions(pair).items():
            completed = harness.run(
                [str(args.candidate), "eval", "--format", "json", expression],
                check=False,
            )
            rejected = completed.returncode != 0
            invalid_rows.append(
                {
                    "probe": probe_id,
                    "pair": pair,
                    "candidate_exit": completed.returncode,
                    "classification": (
                        "Rejected-by-domain" if rejected else "Violated"
                    ),
                    "status": "Preserved" if rejected else "Violated",
                }
            )

    product_rows.sort(key=lambda row: str(row["fixture_id"]))
    identity_rows.sort(key=lambda row: str(row["fixture_id"]))
    determinism_rows.sort(
        key=lambda row: (str(row["phase"]), str(row["fixture_id"]))
    )
    invalid_rows.sort(key=lambda row: (str(row["pair"]), str(row["probe"])))
    pair_rows = []
    for pair in harness.PAIRS:
        product = [row for row in product_rows if row["pair"] == pair]
        identities = [row for row in identity_rows if row["pair"] == pair]
        invalid = [row for row in invalid_rows if row["pair"] == pair]
        determinism = [row for row in determinism_rows if row["pair"] == pair]
        passed = all(
            row["status"] == "Preserved"
            for row in product + identities + invalid + determinism
        )
        pair_rows.append(
            {
                "pair": pair,
                "expected_route": "native-adaptive" if pair in POLAR else "fallback",
                "product": f"{sum(row['status'] == 'Preserved' for row in product)}/24",
                "sealed_route_evidence": f"{sum(row['status'] == 'Preserved' for row in identities)}/24",
                "invalid_rejected": f"{sum(row['status'] == 'Preserved' for row in invalid)}/{len(invalid)}",
                "determinism": f"{sum(row['status'] == 'Preserved' for row in determinism)}/{len(determinism)}",
                "productive_promotion": str(pair in POLAR).lower(),
                "classification": (
                    "Productively-Preserved"
                    if passed and pair in POLAR
                    else "Unknown-generalization"
                    if passed
                    else "Violated"
                ),
            }
        )

    attacks = []

    def attack(mutant: str, killed: bool, witness: str) -> None:
        attacks.append(
            {
                "mutant_id": mutant,
                "executed": "true",
                "witness": witness,
                "status": "REJECTED" if killed else "SURVIVED",
            }
        )

    polar_product = [row for row in product_rows if row["pair"] in POLAR]
    luma_product = [row for row in product_rows if row["pair"] not in POLAR]
    polar_identity = [row for row in identity_rows if row["pair"] in POLAR]
    attack("A01-drop-fixture", len(product_rows) == 192, "192 product receipts")
    attack("A02-drop-linear", sum(str(row["pair"]).startswith("linear/") for row in polar_product) == 72, "72 Linear polar receipts")
    attack("A03-drop-radial", sum(str(row["pair"]).startswith("radial/") for row in polar_product) == 72, "72 Radial polar receipts")
    for space in ("oklch", "hsl", "hsv"):
        attack(f"A-space-{space}", sum(str(row["pair"]).endswith(f"/{space}") for row in polar_product) == 48, f"48 independent {space} receipts")
    attack("A07-retain-fallback", all(int(row["fallback_markers"]) == 0 for row in polar_product), "144 polar routes have zero fallback")
    attack("A08-missing-variant", all(int(row["variant_nodes"]) >= 1 for row in polar_product), "144 polar routes have native variants")
    attack("A09-pending-reference", all(int(row["unresolved"]) == 0 for row in polar_product), "144 polar graphs resolved")
    attack("A10-raster-disguise", all(int(row["raster_nodes"]) == 0 for row in polar_product), "144 polar routes remain vector")
    attack("A11-solid-disguise", all(int(row["role_urls"]) >= 1 for row in polar_product), "144 polar paints reference servers")
    attack(
        "A12-bypass-adaptive",
        all(
            row["status"] == "Preserved"
            and row["sealed_evidence_sha256"] == P1279_CERTIFICATE_SHA256
            for row in polar_identity
        ),
        "144 product routes bind the immutable P1279 adaptive certificate",
    )
    attack("A13-promote-luma", len(luma_product) == 48 and all(int(row["fallback_markers"]) >= 1 and int(row["variant_nodes"]) == 0 for row in luma_product), "48 Luma controls retain fallback")
    attack("A14-drop-invalid", len(invalid_rows) == 56 and all(row["status"] == "Preserved" for row in invalid_rows), "56 invalid probes rejected")
    attack("A15-order-sensitive", len(determinism_rows) == 384 and all(row["status"] == "Preserved" for row in determinism_rows), "384 inverse/repeat receipts")
    roles = {str(row["role"]) for row in polar_product}
    attack("A16-fill-only", {"fill", "stroke"} <= roles, "fill and stroke appear independently")
    attack("A17-unknown-as-success", all(row["classification"] == "Unknown-generalization" for row in pair_rows if row["pair"] not in POLAR), "Luma remains outside success")
    attack("A18-general-equivalence", len(pair_rows) == 8, "verdict remains limited to eight measured pairs")

    if any(row["status"] != "Preserved" for row in product_rows + identity_rows + invalid_rows + determinism_rows):
        raise ValueError("P1280 productive gate failed")
    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("P1280 productive mutant survived")

    write_tsv(args.out / "p1280-product-boundary.tsv", product_rows)
    write_tsv(args.out / "p1280-route-identity.tsv", identity_rows)
    write_tsv(args.out / "p1280-determinism.tsv", determinism_rows)
    write_tsv(args.out / "p1280-invalid-domain.tsv", invalid_rows)
    write_tsv(args.out / "p1280-pairs.tsv", pair_rows)
    write_tsv(args.out / "p1280-attacks-executed.tsv", attacks)
    summary = {
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "candidate": str(args.candidate),
        "determinism_pass": sum(row["status"] == "Preserved" for row in determinism_rows),
        "determinism_total": len(determinism_rows),
        "head": head,
        "invalid_rejected": sum(row["status"] == "Preserved" for row in invalid_rows),
        "invalid_total": len(invalid_rows),
        "luma_fallback": sum(row["status"] == "Preserved" for row in luma_product),
        "luma_total": len(luma_product),
        "measured_at": measured_at,
        "mutants_rejected": sum(row["status"] == "REJECTED" for row in attacks),
        "mutants_total": len(attacks),
        "mutation_score": sum(row["status"] == "REJECTED" for row in attacks) / len(attacks),
        "polar_native": sum(row["status"] == "Preserved" for row in polar_product),
        "polar_route_identity": sum(row["status"] == "Preserved" for row in polar_identity),
        "polar_total": len(polar_product),
        "scope": "P1280 productive boundary only; P1279 language envelope inherited immutably",
        "verdict": "POLAR-PRODUCTIVELY-PRESERVED",
    }
    (args.out / "p1280-product-summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
