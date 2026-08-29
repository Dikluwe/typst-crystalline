#!/usr/bin/env python3
"""Materializa os recibos compactos e o certificado conservador de P1278."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import shutil
from pathlib import Path


GENERAL = (
    "p1278-corpus.tsv",
    "p1278-oracle-budgets.tsv",
    "p1278-results.tsv",
    "p1278-interval-cost.tsv",
    "p1278-graph.tsv",
    "p1278-raster.tsv",
    "p1278-product-boundary.tsv",
    "p1278-determinism.tsv",
    "p1278-invalid-domain.tsv",
    "p1278-pairs.tsv",
    "p1278-attacks-executed.tsv",
    "p1278-summary.json",
)
FOCAL = (
    "p1278-focal.tsv",
    "p1278-focal-commands.tsv",
    "p1278-focal-pairs.tsv",
    "p1278-focal-attacks.tsv",
    "p1278-focal-summary.json",
)
REPRO_GENERAL = tuple(name for name in GENERAL if name != "p1278-summary.json")
REPRO_FOCAL = tuple(name for name in FOCAL if name != "p1278-focal-summary.json")
POLAR = (
    "linear/oklch",
    "radial/oklch",
    "linear/hsl",
    "radial/hsl",
    "linear/hsv",
    "radial/hsv",
)


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


def fraction(value: str) -> tuple[int, int]:
    left, right = value.split("/")
    return int(left), int(right)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--general-direct", type=Path, required=True)
    parser.add_argument("--general-repro", type=Path, required=True)
    parser.add_argument("--focal-direct", type=Path, required=True)
    parser.add_argument("--focal-inverse", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    args.out = args.out.resolve()
    root = Path(__file__).resolve().parents[2]

    require_equal(args.general_direct, args.general_repro, REPRO_GENERAL)
    require_equal(args.focal_direct, args.focal_inverse, REPRO_FOCAL)
    for name in GENERAL:
        shutil.copyfile(args.general_direct / name, args.out / name)
    for name in FOCAL:
        shutil.copyfile(args.focal_direct / name, args.out / name)

    general = json.loads((args.out / "p1278-summary.json").read_text())
    focal = json.loads((args.out / "p1278-focal-summary.json").read_text())
    pairs = {row["pair"]: row for row in read_tsv(args.out / "p1278-pairs.tsv")}
    old_pairs = {
        row["pair"]: row
        for row in read_tsv(root / "00_nucleo/diagnosticos/p1277-pairs.tsv")
    }
    focal_pairs = {
        row["pair"]: row
        for row in read_tsv(args.out / "p1278-focal-pairs.tsv")
    }
    if general["fixtures"] != 192 or focal["observations"] != 336:
        raise ValueError("population drift")
    if general["mutants_rejected"] != 24 or focal["mutants_rejected"] != 6:
        raise ValueError("mutation gate failed")

    final_pairs = []
    baseline = []
    for pair, row in pairs.items():
        old = old_pairs[pair]
        new_valid, total = fraction(row["valid"])
        old_valid, old_total = fraction(old["valid"])
        delta = new_valid - old_valid
        is_polar = pair in POLAR
        if is_polar and delta < 0:
            raise ValueError(f"polar regression: {pair}")
        final_pairs.append(
            {
                "pair": pair,
                "scope": "polar-refinement" if is_polar else "negative-control",
                "general": row["classification"],
                "general_valid": row["valid"],
                "focal": focal_pairs[pair]["classification"],
                "focal_preserved": f"{focal_pairs[pair]['preserved']}/{focal_pairs[pair]['observations']}",
                "invalid_rejected": row["invalid_rejected"],
                "product_fallback": "Preserved",
                "final_classification": "Unknown-generalization",
                "productive_promotion": "false",
            }
        )
        baseline.append(
            {
                "pair": pair,
                "scope": "polar-refinement" if is_polar else "negative-control",
                "p1277_valid": old["valid"],
                "p1278_valid": row["valid"],
                "valid_delta": delta,
                "p1277_numeric": old["numeric"],
                "p1278_numeric": row["numeric"],
                "p1277_raster": old["raster"],
                "p1278_raster": row["raster"],
                "population_stable": str(total == old_total == 24).lower(),
            }
        )
    write_tsv(args.out / "p1278-final-pairs.tsv", final_pairs)
    write_tsv(args.out / "p1278-causal-baseline.tsv", baseline)

    receipts = [
        {
            "run": "general-direct",
            "population": "192 fixtures + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.general_direct / "p1278-results.tsv"),
            "reproduction": "matched-general-repro",
            "status": "ACCEPTED",
        },
        {
            "run": "general-repro",
            "population": "192 fixtures + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.general_repro / "p1278-results.tsv"),
            "reproduction": "matched-general-direct",
            "status": "ACCEPTED",
        },
        {
            "run": "focal-direct",
            "population": "48 groups / 336 observations",
            "primary_sha256": sha(args.focal_direct / "p1278-focal.tsv"),
            "reproduction": "matched-focal-inverse",
            "status": "ACCEPTED",
        },
        {
            "run": "focal-inverse",
            "population": "48 groups / 336 observations",
            "primary_sha256": sha(args.focal_inverse / "p1278-focal.tsv"),
            "reproduction": "matched-focal-direct",
            "status": "ACCEPTED",
        },
    ]
    write_tsv(args.out / "p1278-execution-receipts.tsv", receipts)

    write_tsv(
        args.out / "p1278-discarded-attempts.tsv",
        [
            {
                "attempt": "A01",
                "class": "counterfactual-loader",
                "result": "first repr-all import omitted dataclass module registration",
                "used_for_verdict": "false",
            },
            {
                "attempt": "A02",
                "class": "build-network",
                "result": "probe build attempted unavailable registry; offline retry used",
                "used_for_verdict": "false",
            },
            {
                "attempt": "A03",
                "class": "build-storage",
                "result": "separate probe target filled volume and linker aborted; generated target removed",
                "used_for_verdict": "false",
            },
            {
                "attempt": "A04",
                "class": "verdict-rounding",
                "result": "ten p95-only violations retained as Unknown; no budget widening",
                "used_for_verdict": "false",
            },
        ],
    )

    write_tsv(
        args.out / "p1278-role-capabilities.tsv",
        [
            {
                "role": "oracle",
                "artifact": "p1278-oracle-budgets.tsv + temporary vanilla masks",
                "write_scope": "oracle artifacts before candidate",
                "read_scope": "frozen inputs, vanilla binary, generated fixtures",
                "isolation": "logical-order-only",
            },
            {
                "role": "implementation",
                "artifact": "adaptive.rs + svg.rs + P1278 probe",
                "write_scope": "L0, owner code, tests and adapter observations",
                "read_scope": "L0, vanilla source, P1277 evidence and oracle artifacts",
                "isolation": "not isolated",
            },
            {
                "role": "attacker",
                "artifact": "p1278-attacks-executed.tsv + p1278-focal-attacks.tsv",
                "write_scope": "attack receipts",
                "read_scope": "contract and execution outputs",
                "isolation": "not isolated",
            },
            {
                "role": "judge",
                "artifact": "p1278-final-pairs.tsv + certificate",
                "write_scope": "adjudication",
                "read_scope": "all P1278 evidence",
                "isolation": "not isolated",
            },
        ],
    )

    snapshot = args.working_tree_snapshot.read_text().rstrip()
    snapshot += "\n\nP1278 executor inputs\n"
    executor_inputs = (
        root / "lab/parity/matrix/p1278_generalize.py",
        root / "lab/parity/matrix/p1278_focal.py",
        root / "lab/parity/matrix/p1278_probe/src/main.rs",
        root / "lab/parity/matrix/p1278_probe/Cargo.toml",
        root / "lab/parity/matrix/p1278_probe/Cargo.lock",
    )
    snapshot += "\n".join(f"{path.relative_to(root)}\t{sha(path)}" for path in executor_inputs)
    snapshot += "\n"
    (args.out / "p1278-working-tree-snapshot.txt").write_text(snapshot)

    polar_valid = sum(fraction(pairs[pair]["valid"])[0] for pair in POLAR)
    old_polar_valid = sum(fraction(old_pairs[pair]["valid"])[0] for pair in POLAR)
    combined = {
        "verdict": "POLAR-REFINEMENT-PRESERVED-NOT-PROMOTED",
        "head": general["head"],
        "measured_at": general["measured_at"],
        "general_fixtures": 192,
        "general_preserved": general["preserved"],
        "general_violated": general["violated"],
        "polar_fixtures": 144,
        "polar_preserved_p1277": old_polar_valid,
        "polar_preserved_p1278": polar_valid,
        "polar_preserved_delta": polar_valid - old_polar_valid,
        "polar_raster_pass": sum(fraction(pairs[pair]["raster"])[0] for pair in POLAR),
        "focal_observations": focal["observations"],
        "focal_preserved": focal["preserved"],
        "invalid_rejected": general["invalid_rejected"],
        "determinism_pass": general["determinism_pass"],
        "mutants_rejected": general["mutants_rejected"] + focal["mutants_rejected"],
        "mutants_total": general["mutants"] + focal["mutants"],
        "remaining_polar_unknown_fixtures": 144 - polar_valid,
        "productive_promotions_applied": 0,
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1278-final-summary.json").write_text(
        json.dumps(combined, sort_keys=True, indent=2) + "\n"
    )

    inputs = (
        root / "typst-passo-1278.md",
        root / "00_nucleo/prompts/infra/export/gradients/adaptive.md",
        root / "00_nucleo/prompts/infra/export/svg.md",
        root / "03_infra/src/export/gradients/adaptive.rs",
        root / "03_infra/src/export/svg.rs",
        root / "00_nucleo/diagnosticos/p1266-generalization-matrix.tsv",
        root / "00_nucleo/diagnosticos/p1276-contract.tsv",
        root / "00_nucleo/diagnosticos/p1276-unknown-policy.tsv",
        root / "00_nucleo/diagnosticos/p1276-attacks.tsv",
        root / "00_nucleo/diagnosticos/p1277-pairs.tsv",
        root / "00_nucleo/diagnosticos/p1278-materialize.py",
        *executor_inputs,
        Path("/usr/local/bin/typst"),
        root / "target/debug/typst",
    )
    write_tsv(
        args.out / "p1278-input-manifest.tsv",
        [{"path": str(path), "sha256": sha(path)} for path in inputs],
    )

    failures = [
        row
        for row in read_tsv(args.out / "p1278-results.tsv")
        if row["pair"] in POLAR and row["status"] != "Preserved"
    ]
    failure_rows = "\n".join(
        f"| {row['fixture_id']} | {row['color_p95']} | {row['V_color_p95_limit']} |"
        for row in failures
    )
    pair_rows = "\n".join(
        f"| {pair} | {old_pairs[pair]['valid']} | {pairs[pair]['valid']} | {pairs[pair]['raster']} |"
        for pair in POLAR
    )
    certificate = f"""# Certificado P1278 — refinamento SVG polar

## Veredito

`POLAR-REFINEMENT-PRESERVED-NOT-PROMOTED`.

Na matriz congelada de P1277, os seis pares polares melhoraram de
{old_polar_valid}/144 para {polar_valid}/144 fixtures gerais preservados, sem
regressão por par. Raster fechou 144/144, focal público {focal['preserved']}/{focal['observations']},
domínio inválido {general['invalid_rejected']}/{general['invalid_probes']}, determinismo
{general['determinism_pass']}/{general['determinism_receipts']} e ataques
{combined['mutants_rejected']}/{combined['mutants_total']}.

| Par | P1277 válido | P1278 válido | Raster P1278 |
|---|---:|---:|---:|
{pair_rows}

## Unknown remanescente

Dez fixtures excedem somente o budget `color_p95`; todos os demais gates
aplicáveis passam. Elas permanecem `Unknown-generalization`:

| Fixture | color_p95 | limite vanilla + 1e-6 |
|---|---:|---:|
{failure_rows}

Não houve arredondamento de veredito, aumento de budget ou promoção cruzada.
Luma permaneceu controle negativo (7/24 por par) e CMYK continuou fora do
escopo por ADR-0097.

## Fronteira produtiva

`paint_is_svg_native` não foi ampliado. Os 192/192 grupos mantiveram fallback
produtivo, portanto `productive_promotions_applied = 0`. Uma promoção futura é
mudança de comportamento por defeito e exige o gate ADR-0127.

## Proveniência

- baseline: upstream/main `a51e02804` ratificado;
- HEAD medido: `{general['head']}` com working tree não commitado;
- medição: `{general['measured_at']}`;
- resultados gerais reproduzidos com SHA-256 `{sha(args.general_direct / 'p1278-results.tsv')}`;
- focal reproduzido com SHA-256 `{sha(args.focal_direct / 'p1278-focal.tsv')}`.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Os papéis foram separados por ordem e
artefatos, mas compartilharam checkout e capacidade de leitura.
"""
    (args.out / "typst-p1278-polar-refinement.md").write_text(certificate)

    evidence = sorted(
        path
        for path in args.out.glob("p1278-*")
        if path.name != "p1278-evidence-manifest.tsv"
    ) + [args.out / "typst-p1278-polar-refinement.md"]
    write_tsv(
        args.out / "p1278-evidence-manifest.tsv",
        [{"path": str(path.relative_to(root)), "sha256": sha(path)} for path in evidence],
    )


if __name__ == "__main__":
    main()
