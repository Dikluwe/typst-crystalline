#!/usr/bin/env python3
"""Materializa o certificado P1275 a partir de uma execução já concluída."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import shutil
import subprocess
from datetime import datetime
from pathlib import Path


ROOT = Path("/repos/Antigravity/typst-crystalline")
DIAG = ROOT / "00_nucleo/diagnosticos"


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise RuntimeError(f"recibo vazio: {path}")
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, text=True, capture_output=True
    ).stdout.rstrip()


def require(rows: list[dict[str, str]], count: int, label: str) -> None:
    if len(rows) != count:
        raise RuntimeError(f"{label}: esperado {count}, observado {len(rows)}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--preseal", type=Path, required=True)
    args = parser.parse_args()
    raw = args.run_root / "raw"
    adjudication = args.run_root / "adjudication"
    negative = args.run_root / "negative"

    names = (
        "public-stops.tsv",
        "numeric.tsv",
        "interval-cost.tsv",
        "coincidence.tsv",
        "fixture-gates.tsv",
        "invalid-domain.tsv",
        "determinism.tsv",
        "mutants.tsv",
        "opaque.tsv",
        "pairs.tsv",
        "route.tsv",
    )
    receipts = {
        name: read_tsv(adjudication / f"p1269-{name}") for name in names
    }

    expected = {
        "fixture-gates.tsv": 96,
        "numeric.tsv": 384,
        "interval-cost.tsv": 1224,
        "public-stops.tsv": 96,
        "coincidence.tsv": 180,
        "invalid-domain.tsv": 28,
        "determinism.tsv": 192,
        "mutants.tsv": 25,
        "opaque.tsv": 10,
        "pairs.tsv": 4,
        "route.tsv": 1,
    }
    for name, count in expected.items():
        require(receipts[name], count, name)

    for name in (
        "fixture-gates.tsv",
        "numeric.tsv",
        "interval-cost.tsv",
        "public-stops.tsv",
        "coincidence.tsv",
        "invalid-domain.tsv",
        "determinism.tsv",
    ):
        if any(row["status"] != "Preserved" for row in receipts[name]):
            raise RuntimeError(f"gate não preservado: {name}")

    negatives = [
        row for row in receipts["mutants.tsv"]
        if row["mutation_denominator"] == "included"
    ]
    if len(negatives) != 24 or any(
        row["killed"] != "true" or row["status"] != "Preserved"
        for row in negatives
    ):
        raise RuntimeError("mutation_score P1268 diferente de 1.0")
    if any(
        row["status"] != "ExplicitOpacity" or row["counts_as_success"] != "false"
        for row in receipts["opaque.tsv"]
    ):
        raise RuntimeError("Unknown/opacidade convertido em sucesso")
    if any(
        row["all_gates"] != "24/24"
        or row["numeric"] != "24/24"
        or row["classification"] != "Generalization-Preserved"
        or row["productive_promotion"] != "true"
        for row in receipts["pairs.tsv"]
    ):
        raise RuntimeError("par promovido sem certificado completo")
    route = receipts["route.tsv"][0]
    if route["verdict"] != "Generalization-Preserved" or route["route"] != "P1275":
        raise RuntimeError("rota adjudicada não fecha em P1275")

    raw_product = read_tsv(raw / "p1266-product-boundary.tsv")
    require(raw_product, 96, "fronteira produtiva")
    raw_graph = read_tsv(raw / "p1266-graph.tsv")
    raw_raster = read_tsv(raw / "p1266-raster.tsv")
    require(raw_graph, 96, "grafo produtivo")
    require(raw_raster, 96, "raster")
    if any(row["status"] != "Preserved" for row in raw_graph):
        raise RuntimeError("grafo produtivo não preservado")
    gates_by_fixture = {
        row["fixture_id"]: row for row in receipts["fixture-gates.tsv"]
    }
    raster_rows = []
    for row in raw_raster:
        gates = gates_by_fixture[row["fixture_id"]]
        required = [gates["G06"]]
        if row["fixture_id"].startswith("S20-"):
            required.append(gates["G07A"])
        if row["fixture_id"].startswith("S21-"):
            required.append(gates["G07B"])
        adjudicated = "Preserved" if all(value == "Preserved" for value in required) else "Violated"
        if adjudicated != "Preserved":
            raise RuntimeError(f"raster adjudicado não preservado: {row['fixture_id']}")
        raster_rows.append({
            **{key: value for key, value in row.items() if key != "status"},
            "raw_status": row["status"],
            "status": adjudicated,
        })
    product_rows = []
    for row in raw_product:
        preserved = (
            int(row["fallback_markers"]) == 0
            and int(row["variant_nodes"]) >= 1
            and row["productive_promotion"] == "unexpected"
        )
        if not preserved:
            raise RuntimeError(f"promoção produtiva ausente: {row['fixture_id']}")
        product_rows.append({
            "fixture_id": row["fixture_id"],
            "pair": row["pair"],
            "fallback_markers": row["fallback_markers"],
            "variant_nodes": row["variant_nodes"],
            "status": "PROMOTED-PRESERVED",
        })

    negative_rows = []
    for source in sorted(negative.glob("*.typ")):
        svg = source.with_suffix(".svg")
        if not svg.is_file():
            raise RuntimeError(f"SVG negativo ausente: {svg}")
        serialized = svg.read_text(encoding="utf-8")
        marker_count = serialized.count(
            'data-crystalline-fill-fallback="gradient-color-space"'
        )
        server_count = serialized.count("<linearGradient") + serialized.count("<radialGradient")
        if marker_count != 1 or server_count != 0:
            raise RuntimeError(f"fronteira negativa violada: {source.stem}")
        kind, space = source.stem.split("-", 1)
        negative_rows.append({
            "pair": f"{kind}/{space}",
            "source_sha256": sha(source),
            "svg_sha256": sha(svg),
            "fallback_markers": marker_count,
            "variant_nodes": server_count,
            "status": "FALLBACK-PRESERVED",
        })
    require(negative_rows, 10, "pares não aprovados")

    for name in names:
        shutil.copyfile(adjudication / f"p1269-{name}", DIAG / f"p1275-{name}")
    shutil.copyfile(raw / "p1266-graph.tsv", DIAG / "p1275-graph.tsv")
    write_tsv(DIAG / "p1275-raster.tsv", raster_rows)
    write_tsv(DIAG / "p1275-product-boundary.tsv", product_rows)
    write_tsv(DIAG / "p1275-negative-pairs.tsv", negative_rows)

    pair_rows = []
    for row in receipts["pairs.tsv"]:
        pair_rows.append({
            "pair": row["pair"],
            "population": row["population"],
            "all_gates": row["all_gates"],
            "numeric": row["numeric"],
            "productive_boundary": "24/24 native;zero fallback",
            "verdict": "PROMOTED-PRESERVED",
        })
    write_tsv(DIAG / "p1275-pair-certificates.tsv", pair_rows)

    input_rows = []
    for label, path in (
        ("p1274_manifest", DIAG / "p1274-final-manifest.tsv"),
        ("p1272_manifest", DIAG / "p1272-final-manifest.tsv"),
        ("p1272_certificate", DIAG / "typst-p1272-generalization-certificate.md"),
        ("l0_svg", DIAG.parent / "prompts/infra/export/svg.md"),
        ("consumer_svg", ROOT / "03_infra/src/export/svg.rs"),
        ("matrix", DIAG / "p1266-generalization-matrix.tsv"),
        ("contract", DIAG / "p1266-generalization-contract.tsv"),
        ("unknown_policy", DIAG / "p1266-unknown-policy.tsv"),
        ("attacks", DIAG / "p1266-generalization-attacks.tsv"),
        ("runner", ROOT / "lab/parity/matrix/p1266_generalize.py"),
        ("base_adjudicator", DIAG / "p1269-adjudicator.py"),
        ("p1275_adjudicator", args.run_root / "p1275-adjudicator.py"),
        ("candidate", ROOT / "target/debug/typst"),
        ("vanilla", Path("/usr/local/bin/typst")),
        ("probe", ROOT / "lab/parity/matrix/p1266_probe/target/debug/p1266-svg-generalization-probe"),
        ("preseal", args.preseal),
        ("runner_snapshot", args.run_root / "working-tree-snapshot.txt"),
    ):
        input_rows.append({"input": label, "path": str(path), "sha256": sha(path)})
    write_tsv(DIAG / "p1275-input-manifest.tsv", input_rows)

    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    snapshot = (
        f"measured_at={measured_at}\nhead={git('rev-parse', 'HEAD')}\n"
        "working_tree=uncommitted;P1275-receipts-publishing\n\n"
        f"git status --short\n{git('status', '--short')}\n\n"
        f"git diff HEAD --stat\n{git('diff', 'HEAD', '--stat')}\n"
    )
    (DIAG / "p1275-working-tree-snapshot.txt").write_text(snapshot, encoding="utf-8")

    summary = {
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "baseline_commit": git("rev-parse", "HEAD"),
        "candidate_sha256": sha(ROOT / "target/debug/typst"),
        "determinism": "192/192",
        "discarded_run": "first run invalidated because runner overwrote the file misclassified as preseal",
        "fixtures": "96/96",
        "interval_cost": "1224/1224",
        "invalid_rejected": "28/28",
        "mutation_score": "24/24",
        "negative_pairs": "10/10 FALLBACK-PRESERVED",
        "numeric": "384/384",
        "opaque": "10/10 excluded from success",
        "pair_verdict": "4/4 PROMOTED-PRESERVED",
        "preseal_sha256": sha(args.preseal),
        "scope": "96-fixture Linear/Radial x Oklab/LinearRgb envelope plus 10 non-approved pair boundary fixtures;no general SVG equivalence",
        "verdict": "PROMOTED-PRESERVED",
    }
    (DIAG / "p1275-summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
