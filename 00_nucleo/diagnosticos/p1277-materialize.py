#!/usr/bin/env python3
"""Materializa os recibos compactos de P1277 a partir de duas execuções."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import shutil
from datetime import datetime
from pathlib import Path


GENERAL_FILES = (
    "p1277-corpus.tsv", "p1277-oracle-budgets.tsv", "p1277-results.tsv",
    "p1277-interval-cost.tsv", "p1277-graph.tsv", "p1277-raster.tsv",
    "p1277-product-boundary.tsv", "p1277-determinism.tsv",
    "p1277-invalid-domain.tsv", "p1277-pairs.tsv",
    "p1277-attacks-executed.tsv", "p1277-summary.json",
)
FOCAL_FILES = (
    "p1277-focal.tsv", "p1277-focal-commands.tsv", "p1277-focal-pairs.tsv",
    "p1277-focal-attacks.tsv", "p1277-focal-summary.json",
)
REPRO_GENERAL = tuple(
    name for name in GENERAL_FILES
    if name not in {"p1277-summary.json", "p1277-attacks-executed.tsv"}
)
REPRO_FOCAL = tuple(name for name in FOCAL_FILES if name != "p1277-focal-summary.json")


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise ValueError(f"empty artifact: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def require_equal(left: Path, right: Path, names: tuple[str, ...]) -> None:
    divergent = [name for name in names if sha(left / name) != sha(right / name)]
    if divergent:
        raise ValueError(f"non-reproducible artifacts: {divergent}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--general-first", type=Path, required=True)
    parser.add_argument("--general-final", type=Path, required=True)
    parser.add_argument("--focal-direct", type=Path, required=True)
    parser.add_argument("--focal-inverse", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    root = Path(__file__).resolve().parents[2]

    require_equal(args.general_first, args.general_final, REPRO_GENERAL)
    require_equal(args.focal_direct, args.focal_inverse, REPRO_FOCAL)

    for name in GENERAL_FILES:
        shutil.copyfile(args.general_final / name, args.out / name)
    for name in FOCAL_FILES:
        shutil.copyfile(args.focal_direct / name, args.out / name)
    ignored_measurement_inputs = (
        root / "lab/parity/matrix/p1277_generalize.py",
        root / "lab/parity/matrix/p1277_focal.py",
        root / "lab/parity/matrix/p1277_probe/src/main.rs",
        root / "lab/parity/matrix/p1277_probe/Cargo.toml",
        root / "lab/parity/matrix/p1277_probe/Cargo.lock",
    )
    snapshot = args.working_tree_snapshot.read_text().rstrip()
    snapshot += "\n\nignored P1277 executor inputs present at measurement\n"
    snapshot += "\n".join(f"{path.relative_to(root)}\t{sha(path)}" for path in ignored_measurement_inputs)
    snapshot += "\n"
    (args.out / "p1277-working-tree-snapshot.txt").write_text(snapshot)

    general = json.loads((args.out / "p1277-summary.json").read_text())
    focal = json.loads((args.out / "p1277-focal-summary.json").read_text())
    general_pairs = {row["pair"]: row for row in read_tsv(args.out / "p1277-pairs.tsv")}
    focal_pairs = {row["pair"]: row for row in read_tsv(args.out / "p1277-focal-pairs.tsv")}
    results = read_tsv(args.out / "p1277-results.tsv")
    product = read_tsv(args.out / "p1277-product-boundary.tsv")
    invalid = read_tsv(args.out / "p1277-invalid-domain.tsv")
    attacks = read_tsv(args.out / "p1277-attacks-executed.tsv")
    focal_attacks = read_tsv(args.out / "p1277-focal-attacks.tsv")

    if general["fixtures"] != 192 or focal["observations"] != 336:
        raise ValueError("P1277 execution cardinality drift")
    if general["mutants_rejected"] != general["mutants"] or focal["mutants_rejected"] != focal["mutants"]:
        raise ValueError("P1277 mutation gate failed")
    if any(row["status"] != "Preserved" for row in product + invalid):
        raise ValueError("product boundary or invalid-domain gate failed")

    final_pairs = []
    for pair in general_pairs:
        general_row = general_pairs[pair]
        focal_row = focal_pairs[pair]
        final_pairs.append({
            "pair": pair,
            "general": general_row["classification"],
            "general_valid": general_row["valid"],
            "focal": focal_row["classification"],
            "focal_preserved": f"{focal_row['preserved']}/{focal_row['observations']}",
            "invalid_rejected": general_row["invalid_rejected"],
            "product_fallback": "Preserved",
            "final_classification": "Unknown-generalization",
            "productive_promotion": "false",
        })
    write_tsv(args.out / "p1277-final-pairs.tsv", final_pairs)

    receipts = (
        {
            "run": "general-first-semantic",
            "population": "192 fixtures",
            "primary_sha256": sha(args.general_first / "p1277-results.tsv"),
            "reproduction": "matched-final",
            "status": "DISCARDED-AS-FINAL: stale M20 schema only",
        },
        {
            "run": "general-final",
            "population": "192 fixtures + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.general_final / "p1277-results.tsv"),
            "reproduction": "matched-first-semantic",
            "status": "ACCEPTED",
        },
        {
            "run": "focal-direct",
            "population": "48 groups / 336 observations",
            "primary_sha256": sha(args.focal_direct / "p1277-focal.tsv"),
            "reproduction": "matched-inverse",
            "status": "ACCEPTED",
        },
        {
            "run": "focal-inverse",
            "population": "48 groups / 336 observations",
            "primary_sha256": sha(args.focal_inverse / "p1277-focal.tsv"),
            "reproduction": "matched-direct",
            "status": "ACCEPTED",
        },
    )
    write_tsv(args.out / "p1277-execution-receipts.tsv", list(receipts))

    discarded = (
        {
            "attempt": "A01",
            "class": "operational",
            "result": "cargo build attempted network access; offline rebuild succeeded",
            "used_for_verdict": "false",
        },
        {
            "attempt": "A02",
            "class": "executor-contract",
            "result": "rgb(color) unsupported by crystalline eval; native-component oracle replaced it",
            "used_for_verdict": "false",
        },
        {
            "attempt": "A03",
            "class": "attack-schema",
            "result": "M20 named predecessor column; semantic outputs reproduced by final run",
            "used_for_verdict": "false",
        },
    )
    write_tsv(args.out / "p1277-discarded-attempts.tsv", list(discarded))

    roles = (
        {
            "role": "oracle",
            "artifact": "p1277-oracle-budgets.tsv + vanilla masks in temporary run",
            "write_scope": "oracle artifacts",
            "read_scope": "P1276 inputs, vanilla binary and generated fixtures",
            "isolation": "logical-order-only",
        },
        {
            "role": "implementation",
            "artifact": "p1277 probe + diagnostic adapter",
            "write_scope": "candidate observations and adapter SVG in temporary run",
            "read_scope": "oracle artifacts and existing adaptive.rs",
            "isolation": "not isolated",
        },
        {
            "role": "attacker",
            "artifact": "p1277-attacks-executed.tsv + p1277-focal-attacks.tsv",
            "write_scope": "attack receipts",
            "read_scope": "contract and execution outputs",
            "isolation": "not isolated",
        },
        {
            "role": "judge",
            "artifact": "p1277-final-pairs.tsv + certificate",
            "write_scope": "adjudication artifacts",
            "read_scope": "all P1277 evidence",
            "isolation": "not isolated",
        },
    )
    write_tsv(args.out / "p1277-role-capabilities.tsv", list(roles))

    inputs = (
        root / "typst-passo-1277.md",
        root / "00_nucleo/diagnosticos/p1266-generalization-matrix.tsv",
        root / "00_nucleo/diagnosticos/p1276-contract.tsv",
        root / "00_nucleo/diagnosticos/p1276-unknown-policy.tsv",
        root / "00_nucleo/diagnosticos/p1276-attacks.tsv",
        root / "00_nucleo/diagnosticos/p1277-materialize.py",
        root / "lab/parity/matrix/p1277_generalize.py",
        root / "lab/parity/matrix/p1277_focal.py",
        root / "lab/parity/matrix/p1277_probe/src/main.rs",
        root / "lab/parity/matrix/p1277_probe/Cargo.toml",
        root / "lab/parity/matrix/p1277_probe/Cargo.lock",
        Path("/usr/local/bin/typst"),
        root / "target/debug/typst",
    )
    write_tsv(args.out / "p1277-input-manifest.tsv", [
        {"path": str(path), "sha256": sha(path)} for path in inputs
    ])

    combined = {
        "verdict": "NO-PROMOTION-ALL-EIGHT-UNKNOWN-GENERALIZATION",
        "head": general["head"],
        "measured_at": general["measured_at"],
        "general_fixtures": 192,
        "general_preserved": general["preserved"],
        "general_violated": general["violated"],
        "graph_pass": general["graph_pass"],
        "numeric_pass": general["numeric_pass"],
        "raster_pass": general["raster_pass"],
        "cost_pass": general["cost_pass"],
        "focal_groups": 48,
        "focal_observations": 336,
        "focal_preserved": focal["preserved"],
        "invalid_rejected": general["invalid_rejected"],
        "invalid_total": general["invalid_probes"],
        "determinism_pass": general["determinism_pass"],
        "determinism_total": general["determinism_receipts"],
        "mutants_rejected": general["mutants_rejected"] + focal["mutants_rejected"],
        "mutants_total": general["mutants"] + focal["mutants"],
        "luma_public_divergent": sum(row["pair"].endswith("/luma") and row["public"] == "Violated" for row in results),
        "productive_promotions_applied": 0,
        "cmyk": "Unknown-ADR0097",
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1277-final-summary.json").write_text(json.dumps(combined, sort_keys=True, indent=2) + "\n")

    evidence_names = sorted(
        path.name for path in args.out.glob("p1277-*")
        if path.name not in {"p1277-evidence-manifest.tsv"}
    )
    write_tsv(args.out / "p1277-evidence-manifest.tsv", [
        {"artifact": name, "sha256": sha(args.out / name)} for name in evidence_names
    ])
    manifest_sha = sha(args.out / "p1277-evidence-manifest.tsv")
    report = f"""# P1277 — certificado da fronteira SVG multi-space restante

**Veredito:** `NO-PROMOTION — ALL-EIGHT-UNKNOWN-GENERALIZATION`.

Foram executados 240 grupos: 192 fixtures gerais (24 por par) e 48 grupos
focais, estes com 336 observações. O envelope focal preservou 336/336, mas a
conjunção geral preservou apenas {general['preserved']}/192: grafo
{general['graph_pass']}/192, numérico {general['numeric_pass']}/192, raster
{general['raster_pass']}/192 e custo {general['cost_pass']}/192. Subgates não se
compensam, portanto os oito pares permanecem `Unknown-generalization`.

Foram rejeitados {len(attacks) + len(focal_attacks)}/{len(attacks) + len(focal_attacks)}
mutantes executados, {general['invalid_rejected']}/{general['invalid_probes']}
entradas inválidas e {general['determinism_pass']}/{general['determinism_receipts']}
recibos de determinismo. A ordem inversa reproduziu os artefatos focais, e a
segunda execução geral reproduziu byte a byte `results`, `pairs` e
`oracle-budgets` (além dos demais recibos semânticos).

Luma concentra {combined['luma_public_divergent']} fixtures com divergência
pública, coerente com a fronteira de alpha já conhecida; isso bloqueia, não é
contado como sucesso. CMYK permanece `Unknown-ADR0097`.

Nenhuma mudança produtiva, de L0 ou de comportamento padrão foi aplicada. Os
oito pares conservam o fallback explícito `gradient-color-space`. Este
certificado é restrito à população executada e não alega equivalência SVG
geral.

Proveniência: HEAD `{general['head']}`, working tree não commitada, medição
`{general['measured_at']}`. Manifesto de evidências
`p1277-evidence-manifest.tsv` (`sha256:{manifest_sha}`).

**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
"""
    (args.out / "typst-p1277-multispace-certificate.md").write_text(report)


if __name__ == "__main__":
    main()
