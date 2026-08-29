#!/usr/bin/env python3
"""Publica recibos P1274 sem alterar os artefatos selados P1272."""

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
    args = parser.parse_args()
    raw = args.run_root / "raw"
    adjudication = args.run_root / "adjudication"

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

    require(receipts["fixture-gates.tsv"], 96, "fixtures")
    require(receipts["numeric.tsv"], 384, "métricas")
    require(receipts["interval-cost.tsv"], 1224, "intervalos")
    require(receipts["public-stops.tsv"], 96, "stops públicos")
    require(receipts["coincidence.tsv"], 180, "coincidências")
    require(receipts["invalid-domain.tsv"], 28, "domínio inválido")
    require(receipts["determinism.tsv"], 192, "determinismo")
    require(receipts["mutants.tsv"], 25, "ataques P1268")
    require(receipts["opaque.tsv"], 10, "opacidade")
    require(receipts["pairs.tsv"], 4, "pares")
    require(receipts["route.tsv"], 1, "rota")

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
    if route["verdict"] != "Generalization-Preserved" or route["route"] != "P1274":
        raise RuntimeError("rota adjudicada não fecha em P1274")

    raw_product = read_tsv(raw / "p1266-product-boundary.tsv")
    require(raw_product, 96, "fronteira produtiva")
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
            "productive_promotion": "true",
            "status": "Preserved",
        })

    for name in names:
        shutil.copyfile(adjudication / f"p1269-{name}", DIAG / f"p1274-{name}")
    write_tsv(DIAG / "p1274-product-boundary.tsv", product_rows)

    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    inputs = []
    for label, path in (
        ("l0_svg", DIAG.parent / "prompts/infra/export/svg.md"),
        ("consumer_svg", ROOT / "03_infra/src/export/svg.rs"),
        ("p1272_certificate", DIAG / "typst-p1272-generalization-certificate.md"),
        ("p1272_manifest", DIAG / "p1272-final-manifest.tsv"),
        ("p1266_runner", ROOT / "lab/parity/matrix/p1266_generalize.py"),
        ("p1269_adjudicator", DIAG / "p1269-adjudicator.py"),
        ("p1274_adjudicator", args.run_root / "p1274-adjudicator.py"),
        ("candidate", ROOT / "target/debug/typst"),
        ("vanilla", Path("/usr/local/bin/typst")),
        ("probe", ROOT / "lab/parity/matrix/p1266_probe/target/debug/p1266-svg-generalization-probe"),
        ("run_snapshot", args.run_root / "working-tree-snapshot.txt"),
    ):
        inputs.append({"input": label, "path": str(path), "sha256": sha(path)})
    write_tsv(DIAG / "p1274-input-manifest.tsv", inputs)

    snapshot = (
        f"measured_at={measured_at}\nhead={git('rev-parse', 'HEAD')}\n"
        "working_tree=uncommitted;post-P1274-materialization\n\n"
        f"git status --short\n{git('status', '--short')}\n\n"
        f"git diff HEAD --stat\n{git('diff', 'HEAD', '--stat')}\n"
    )
    (DIAG / "p1274-working-tree-snapshot.txt").write_text(snapshot, encoding="utf-8")

    summary = {
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "baseline_commit": git("rev-parse", "HEAD"),
        "candidate_sha256": sha(ROOT / "target/debug/typst"),
        "determinism": "192/192",
        "fixtures": "96/96",
        "fixtures_per_pair": 24,
        "interval_cost": "1224/1224",
        "invalid_rejected": "28/28",
        "mutation_score_p1268": "24/24",
        "necessary_unknown": 0,
        "numeric": "384/384",
        "opaque": "10/10 excluded from success",
        "pairs": {row["pair"]: row["classification"] for row in receipts["pairs.tsv"]},
        "productive_boundary": "96/96 native;zero fallback",
        "scope": "96-fixture Linear/Radial x Oklab/LinearRgb envelope only;no general SVG equivalence",
        "implementation_result": "four approved pairs productively promoted",
        "independent_final_verdict": False,
    }
    (DIAG / "p1274-summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
