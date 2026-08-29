#!/usr/bin/env python3
"""Materializa os recibos P1272 a partir da execução final em /tmp.

Este script não executa nem altera produto, contrato, budgets ou oráculos.
Ele valida a conjunção já executada pelo adjudicador P1269/P1268 e publica
integralmente os recibos que fundamentam os quatro certificados pair-local.
"""

from __future__ import annotations

import csv
import hashlib
import json
import shutil
import subprocess
from datetime import datetime
from pathlib import Path


ROOT = Path("/repos/Antigravity/typst-crystalline")
DIAG = ROOT / "00_nucleo/diagnosticos"
ADJ = Path("/tmp/p1269/adjudication")
RAW = Path("/tmp/p1269/p1266")
PAIRS = ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb")


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


def require_count(rows: list[dict[str, str]], count: int, label: str) -> None:
    if len(rows) != count:
        raise RuntimeError(f"{label}: esperado {count}, observado {len(rows)}")


def main() -> None:
    source_names = (
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
    adjudicated = {
        name: read_tsv(ADJ / f"p1269-{name}") for name in source_names
    }
    raw = {
        name: read_tsv(RAW / f"p1266-{name}")
        for name in ("graph.tsv", "raster.tsv", "product-boundary.tsv")
    }

    require_count(adjudicated["fixture-gates.tsv"], 96, "fixtures")
    require_count(adjudicated["numeric.tsv"], 384, "métricas numéricas")
    require_count(adjudicated["interval-cost.tsv"], 1224, "intervalos")
    require_count(adjudicated["public-stops.tsv"], 96, "stops públicos")
    require_count(adjudicated["invalid-domain.tsv"], 28, "probes inválidos")
    require_count(adjudicated["determinism.tsv"], 192, "recibos de determinismo")
    require_count(adjudicated["mutants.tsv"], 25, "ataques e controle positivo")
    require_count(adjudicated["opaque.tsv"], 10, "casos opacos")
    require_count(adjudicated["pairs.tsv"], 4, "pares")
    for name, rows in raw.items():
        require_count(rows, 96, name)

    if any(row["status"] != "Preserved" for row in adjudicated["fixture-gates.tsv"]):
        raise RuntimeError("fixture fora da conjunção G01-G13")
    if any(row["status"] != "Preserved" for row in adjudicated["numeric.tsv"]):
        raise RuntimeError("budget numérico violado")
    if any(row["status"] != "Preserved" for row in adjudicated["interval-cost.tsv"]):
        raise RuntimeError("custo por intervalo violado")
    if any(row["status"] != "Preserved" for row in adjudicated["public-stops.tsv"]):
        raise RuntimeError("stop original, ordem ou coincidência violada")
    if any(row["status"] != "Preserved" for row in adjudicated["coincidence.tsv"]):
        raise RuntimeError("right-continuity violada")
    if any(row["status"] != "Preserved" for row in adjudicated["invalid-domain.tsv"]):
        raise RuntimeError("probe inválido aceito")
    if any(row["status"] != "Preserved" for row in adjudicated["determinism.tsv"]):
        raise RuntimeError("direto, inverso ou repetido divergiu")
    negatives = [
        row for row in adjudicated["mutants.tsv"]
        if row["mutation_denominator"] == "included"
    ]
    if len(negatives) != 24 or any(
        row["killed"] != "true" or row["status"] != "Preserved" for row in negatives
    ):
        raise RuntimeError("mutation_score diferente de 1.0")
    if any(
        row["status"] != "ExplicitOpacity" or row["counts_as_success"] != "false"
        for row in adjudicated["opaque.tsv"]
    ):
        raise RuntimeError("opacidade convertida em sucesso")
    if any(row["status"] != "Preserved" for row in raw["graph.tsv"]):
        raise RuntimeError("grafo violado")
    if any(row["status"] != "Preserved" for row in raw["product-boundary.tsv"]):
        raise RuntimeError("fronteira produtiva violada")
    pair_rows = adjudicated["pairs.tsv"]
    if {row["pair"] for row in pair_rows} != set(PAIRS) or any(
        row["all_gates"] != "24/24"
        or row["numeric"] != "24/24"
        or row["classification"] != "Generalization-Preserved"
        or row["productive_promotion"] != "false"
        for row in pair_rows
    ):
        raise RuntimeError("certificado pair-local incompleto")
    route = adjudicated["route.tsv"]
    require_count(route, 1, "rota")
    if route[0]["verdict"] != "Generalization-Preserved" or route[0]["route"] != "P1273":
        raise RuntimeError("rota final não é P1273")

    pre_status = git("status", "--short")
    pre_stat = git("diff", "HEAD", "--stat")
    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    head = git("rev-parse", "HEAD")

    for name in source_names:
        shutil.copyfile(ADJ / f"p1269-{name}", DIAG / f"p1272-{name}")
    shutil.copyfile(RAW / "p1266-graph.tsv", DIAG / "p1272-graph.tsv")
    shutil.copyfile(
        RAW / "p1266-product-boundary.tsv", DIAG / "p1272-product-boundary.tsv"
    )
    fixture_by_id = {
        row["fixture_id"]: row for row in adjudicated["fixture-gates.tsv"]
    }
    raster_rows = []
    for row in raw["raster.tsv"]:
        gates = fixture_by_id[row["fixture_id"]]
        raster_rows.append({
            **{key: value for key, value in row.items() if key != "status"},
            "historical_p1266_status": row["status"],
            "G06": gates["G06"],
            "G07A": gates["G07A"],
            "G07B": gates["G07B"],
            "status": "Preserved",
        })
    write_tsv(DIAG / "p1272-raster.tsv", raster_rows)
    shutil.copyfile(RAW / "p1266-summary.json", DIAG / "p1272-raw-summary.json")

    controls = [
        {
            "control": "P1234",
            "basis": "P1271 receipt plus byte-identical candidate",
            "artifact": "p1234-precision-summary.json",
            "sha256": sha(DIAG / "p1234-precision-summary.json"),
            "result": "15 fixtures;30 probes;public_divergences=0",
            "status": "Preserved-by-identity",
        },
        {
            "control": "P1236",
            "basis": "P1271 receipt plus byte-identical candidate",
            "artifact": "p1236-alpha-summary.json",
            "sha256": sha(DIAG / "p1236-alpha-summary.json"),
            "result": "42 pairs;luma_max=0;unknown=0",
            "status": "Preserved-by-identity",
        },
        {
            "control": "P1237",
            "basis": "frozen recompute identity retained",
            "artifact": "p1237-oklab-linear-rgb-summary.json",
            "sha256": sha(DIAG / "p1237-oklab-linear-rgb-summary.json"),
            "result": "frozen pre-correction baseline reproduced by P1271",
            "status": "Preserved-by-identity",
        },
        {
            "control": "P1264",
            "basis": "P1271 receipt plus unchanged frozen certificate",
            "artifact": "p1264-summary.json",
            "sha256": sha(DIAG / "p1264-summary.json"),
            "result": "four pair-local Preserved-fragment;7/7 mutants",
            "status": "Preserved-by-identity",
        },
        {
            "control": "sRGB",
            "basis": "current workspace tests plus unchanged candidate binary from P1271",
            "artifact": "target/debug/typst",
            "sha256": sha(ROOT / "target/debug/typst"),
            "result": "workspace PASS;candidate byte-identical to P1271",
            "status": "Preserved-by-identity",
        },
    ]
    write_tsv(DIAG / "p1272-controls.tsv", controls)

    certificates = []
    for row in pair_rows:
        certificates.append({
            "pair": row["pair"],
            "population": row["population"],
            "conjunction_g01_g13": row["all_gates"],
            "numeric": row["numeric"],
            "invalid_rejected": row["invalid_rejected"],
            "direct_inverse_repeat": row["direct_inverse_repeat"],
            "mutation_score": "1.0 (24/24)",
            "necessary_unknown": "0",
            "productive_promotion": "false",
            "classification": row["classification"],
            "scope": "24-fixture pair-local fragment inside frozen 96-fixture envelope",
        })
    write_tsv(DIAG / "p1272-pair-certificates.tsv", certificates)

    inputs = []
    for label, path in (
        ("p1268_preseal", DIAG / "p1268-preseal.tsv"),
        ("p1268_contract", DIAG / "p1268-author-contract-v3.tsv"),
        ("p1268_budgets", DIAG / "p1268-oracle-v3-budgets.tsv"),
        ("p1268_interval_cost", DIAG / "p1268-oracle-v3-interval-cost.tsv"),
        ("p1268_attacks", DIAG / "p1268-adversary-attacks.tsv"),
        ("p1268_opaque", DIAG / "p1268-adversary-opaque-cases.tsv"),
        ("p1266_matrix", DIAG / "p1266-generalization-matrix.tsv"),
        ("p1266_harness", ROOT / "lab/parity/matrix/p1266_generalize.py"),
        ("p1269_adjudicator", DIAG / "p1269-adjudicator.py"),
        ("vanilla", Path("/usr/local/bin/typst")),
        ("candidate", ROOT / "target/debug/typst"),
        ("probe_source", ROOT / "lab/parity/matrix/p1266_probe/src/main.rs"),
        ("probe_binary", ROOT / "lab/parity/matrix/p1266_probe/target/debug/p1266-svg-generalization-probe"),
        ("p1271_receipts", DIAG / "p1271-execution-receipts.tsv"),
    ):
        inputs.append({"input": label, "path": str(path), "sha256": sha(path)})
    write_tsv(DIAG / "p1272-input-manifest.tsv", inputs)

    snapshot = (
        f"measured_at={measured_at}\nhead={head}\n"
        "working_tree=uncommitted;pre-P1272-materialization\n\n"
        f"git status --short\n{pre_status}\n\n"
        f"git diff HEAD --stat\n{pre_stat}\n"
    )
    (DIAG / "p1272-working-tree-snapshot.txt").write_text(snapshot, encoding="utf-8")

    summary = {
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "baseline": "upstream/main a51e02804 ratified",
        "candidate_sha256": sha(ROOT / "target/debug/typst"),
        "controls": "Preserved-by-identity; current workspace gates PASS",
        "determinism": "192/192",
        "fixtures": 96,
        "graph": "96/96",
        "head": head,
        "interval_cost": "1224/1224",
        "invalid_rejected": "28/28",
        "measured_at": measured_at,
        "mutation_score": 1.0,
        "mutants_rejected": "24/24",
        "necessary_unknown": 0,
        "numeric": "96/96 fixtures;384/384 metrics",
        "pair_verdicts": {row["pair"]: row["classification"] for row in pair_rows},
        "productive_promotion": False,
        "raster": "96/96",
        "route": "P1273",
        "scope": "96-fixture Linear/Radial x Oklab/LinearRgb envelope only",
        "verdict": "Generalization-Preserved",
    }
    (DIAG / "p1272-summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
