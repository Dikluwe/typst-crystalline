#!/usr/bin/env python3
"""Materializa a readjudicação produtiva do Passo 1280."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import shutil
from pathlib import Path


PRODUCT_FILES = (
    "p1280-product-boundary.tsv",
    "p1280-route-identity.tsv",
    "p1280-determinism.tsv",
    "p1280-invalid-domain.tsv",
    "p1280-pairs.tsv",
    "p1280-attacks-executed.tsv",
)
P1279_HASHES = {
    "00_nucleo/diagnosticos/typst-p1279-offset-identity-certificate.md":
        "be1e16cfb002c70b8d7af83421e7027f2abdb32324a1e8ff94ace7152ee1e3ed",
    "00_nucleo/diagnosticos/p1279-final-summary.json":
        "c0628bc0e2723e664c587a6b5077c8c9a59bea70d2ffe93909a44efdb5b5c80a",
    "00_nucleo/diagnosticos/p1279-evidence-manifest.tsv":
        "8f44e3ac8da2b9f75ee075f0656a7917f8039c39734e1cef12951ca21916c1a3",
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle, list(rows[0]), delimiter="\t", lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def require_equal(left: Path, right: Path) -> None:
    divergent = [name for name in PRODUCT_FILES if sha(left / name) != sha(right / name)]
    if divergent:
        raise ValueError(f"non-reproducible P1280 artifacts: {divergent}")


def verify_manifest(path: Path, root: Path) -> None:
    for row in read_tsv(path):
        candidate = Path(row["path"])
        if not candidate.is_absolute():
            candidate = root / candidate
        if sha(candidate) != row["sha256"]:
            raise ValueError(f"sealed input drift: {candidate}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--direct", type=Path, required=True)
    parser.add_argument("--repro", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    args.out = args.out.resolve()
    root = Path(__file__).resolve().parents[2]

    require_equal(args.direct, args.repro)
    verify_manifest(root / "00_nucleo/diagnosticos/p1280-preseal-manifest.tsv", root)
    verify_manifest(root / "00_nucleo/diagnosticos/p1280-preseal-inputs.tsv", root)
    for relative, expected in P1279_HASHES.items():
        if sha(root / relative) != expected:
            raise ValueError(f"immutable P1279 evidence drift: {relative}")

    for name in PRODUCT_FILES:
        shutil.copyfile(args.direct / name, args.out / name)
    shutil.copyfile(
        args.direct / "p1280-product-summary.json",
        args.out / "p1280-product-summary.json",
    )

    summary = json.loads((args.out / "p1280-product-summary.json").read_text())
    attacks = read_tsv(args.out / "p1280-attacks-executed.tsv")
    preseal_attacks = read_tsv(args.out / "p1280-preseal-attacks.tsv")
    product = read_tsv(args.out / "p1280-product-boundary.tsv")
    identities = read_tsv(args.out / "p1280-route-identity.tsv")
    invalid = read_tsv(args.out / "p1280-invalid-domain.tsv")
    determinism = read_tsv(args.out / "p1280-determinism.tsv")
    if (
        summary["polar_native"] != 144
        or summary["luma_fallback"] != 48
        or summary["invalid_rejected"] != 56
        or summary["determinism_pass"] != 384
    ):
        raise ValueError("P1280 population or productive verdict drift")
    if any(row["status"] != "Preserved" for row in product + identities + invalid + determinism):
        raise ValueError("P1280 productive evidence failed")
    if any(row["status"] != "REJECTED" for row in attacks + preseal_attacks):
        raise ValueError("P1280 mutant survived")

    p1279_pairs = {
        row["pair"]: row
        for row in read_tsv(root / "00_nucleo/diagnosticos/p1279-final-pairs.tsv")
    }
    product_pairs = read_tsv(args.out / "p1280-pairs.tsv")
    final_pairs = []
    for row in product_pairs:
        pair = row["pair"]
        promoted = row["productive_promotion"] == "true"
        final_pairs.append(
            {
                "pair": pair,
                "p1279_language_classification": p1279_pairs[pair]["final_classification"],
                "p1280_expected_route": row["expected_route"],
                "p1280_product": row["product"],
                "p1280_sealed_route_evidence": row["sealed_route_evidence"],
                "p1280_invalid_rejected": row["invalid_rejected"],
                "p1280_determinism": row["determinism"],
                "productive_promotion": str(promoted).lower(),
                "final_classification": (
                    "Productively-Preserved" if promoted else "Unknown-generalization"
                ),
            }
        )
    write_tsv(args.out / "p1280-final-pairs.tsv", final_pairs)

    receipts = [
        {
            "run": "preseal",
            "population": "14 contract claims / 18 attacks",
            "primary_sha256": sha(args.out / "p1280-preseal-attacks.tsv"),
            "reproduction": "sealed inputs reverified at materialization",
            "status": "ACCEPTED",
        },
        {
            "run": "product-direct",
            "population": "192 product + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.direct / "p1280-product-boundary.tsv"),
            "reproduction": "matched product-repro",
            "status": "ACCEPTED",
        },
        {
            "run": "product-repro-reverse",
            "population": "192 product + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.repro / "p1280-product-boundary.tsv"),
            "reproduction": "matched product-direct",
            "status": "ACCEPTED",
        },
        {
            "run": "language-envelope-inherited-P1279",
            "population": "144 polar fixtures / numeric + raster + graph + cost",
            "primary_sha256": P1279_HASHES["00_nucleo/diagnosticos/p1279-evidence-manifest.tsv"],
            "reproduction": "immutable predecessor manifest and certificate reverified",
            "status": "ACCEPTED-ROUTING-DELTA-ONLY",
        },
    ]
    write_tsv(args.out / "p1280-execution-receipts.tsv", receipts)
    write_tsv(
        args.out / "p1280-discarded-attempts.tsv",
        [
            {
                "attempt": "D01",
                "class": "auxiliary-probe-build",
                "result": "discarded after target-dir build exhausted disk; exact residue removed",
                "used_for_verdict": "false",
            },
            {
                "attempt": "D02",
                "class": "auxiliary-probe-build-retry",
                "result": "discarded after shared target retry exhausted disk; exact residue removed",
                "used_for_verdict": "false",
            },
            {
                "attempt": "D03",
                "class": "evidence-substitution",
                "result": "no substitution: immutable P1279 numeric/raster certificate was reverified by SHA-256",
                "used_for_verdict": "true",
            },
        ],
    )

    snapshot = args.working_tree_snapshot.read_text().rstrip()
    executor_inputs = (
        root / "typst-passo-1280.md",
        root / "lab/parity/matrix/p1280_product_boundary.py",
        root / "00_nucleo/diagnosticos/p1280-preseal.py",
        root / "00_nucleo/diagnosticos/p1280-materialize.py",
    )
    snapshot += "\n\nP1280 executor inputs at materialization\n"
    snapshot += "\n".join(
        f"{path.relative_to(root)}\t{sha(path)}" for path in executor_inputs
    )
    snapshot += "\n"
    (args.out / "p1280-working-tree-snapshot.txt").write_text(snapshot)

    final = {
        "verdict": "POLAR-PRODUCTIVELY-PRESERVED",
        "head": summary["head"],
        "measured_at": summary["measured_at"],
        "product_fixtures": len(product),
        "polar_native": summary["polar_native"],
        "polar_total": summary["polar_total"],
        "luma_fallback": summary["luma_fallback"],
        "luma_total": summary["luma_total"],
        "invalid_rejected": summary["invalid_rejected"],
        "invalid_total": summary["invalid_total"],
        "determinism_pass": summary["determinism_pass"],
        "determinism_total": summary["determinism_total"],
        "p1280_mutants_rejected": len(attacks) + len(preseal_attacks),
        "p1280_mutants_total": len(attacks) + len(preseal_attacks),
        "p1279_mutants_inherited": 30,
        "productive_promotions_applied": 6,
        "unchanged_unknown_pairs": 2,
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1280-final-summary.json").write_text(
        json.dumps(final, sort_keys=True, indent=2) + "\n"
    )

    inputs = (
        *executor_inputs,
        root / "00_nucleo/prompts/infra/export/svg.md",
        root / "03_infra/src/export/svg.rs",
        root / "00_nucleo/diagnosticos/p1266-generalization-matrix.tsv",
        root / "00_nucleo/diagnosticos/p1280-contract.tsv",
        root / "00_nucleo/diagnosticos/p1280-unknown-policy.tsv",
        root / "00_nucleo/diagnosticos/p1280-policy.tsv",
        root / "00_nucleo/diagnosticos/p1280-red-receipt.tsv",
        root / "00_nucleo/diagnosticos/p1280-green-receipt.tsv",
        *(root / relative for relative in P1279_HASHES),
        root / "target/debug/typst",
    )
    write_tsv(
        args.out / "p1280-input-manifest.tsv",
        [{"path": str(path), "sha256": sha(path)} for path in inputs],
    )

    polar_lines = "\n".join(
        f"| {row['pair']} | {row['p1280_product']} | {row['p1280_determinism']} | {row['final_classification']} |"
        for row in final_pairs
        if row["productive_promotion"] == "true"
    )
    report = f"""# Diagnóstico P1280 — promoção produtiva dos gradientes polares

## Veredito

`POLAR-PRODUCTIVELY-PRESERVED`.

Os seis pares Linear/Radial × Oklch/Hsl/Hsv foram promovidos da rota de
fallback para o servidor SVG adaptativo já certificado pelo P1279. A matriz
produtiva fechou {summary['polar_native']}/{summary['polar_total']} fixtures
polares nativas; Luma conservou fallback em
{summary['luma_fallback']}/{summary['luma_total']} controles.

| Par | Produto | Determinismo | Classificação final |
|---|---:|---:|---|
{polar_lines}

## Evidência e ataques

- domínio inválido rejeitado: {summary['invalid_rejected']}/{summary['invalid_total']};
- recomposições inversa/repetida: {summary['determinism_pass']}/{summary['determinism_total']};
- ataques P1280 rejeitados: {len(attacks) + len(preseal_attacks)}/{len(attacks) + len(preseal_attacks)};
- envelope numérico/raster/grafo/custo P1279 herdado por manifesto imutável: `{P1279_HASHES['00_nucleo/diagnosticos/p1279-evidence-manifest.tsv']}`.

O RED observou o fallback anterior em Linear/Oklch; o GREEN cobriu os seis
pares em fill e stroke. O delta produtivo limita-se ao predicado de
elegibilidade; o adaptador certificado não foi alterado.

## Limites

O veredito não afirma equivalência SVG geral. Linear/Radial × Luma e × CMYK
continuam `Unknown-generalization`; Conic e Tiling não foram promovidos.

## Proveniência

- baseline: upstream/main `a51e02804` ratificado;
- HEAD medido: `{summary['head']}` com working tree não commitado;
- medição direta: `{summary['measured_at']}`;
- resultado produtivo reproduzido: `{sha(args.direct / 'p1280-product-boundary.tsv')}`.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Os papéis, entradas e artefatos foram
segregados logicamente, mas compartilharam checkout e capacidade de leitura.
"""
    report_path = args.out / "typst-p1280-polar-product-promotion.md"
    report_path.write_text(report)

    evidence = sorted(
        path
        for path in args.out.glob("p1280-*")
        if path.name != "p1280-evidence-manifest.tsv"
    ) + [report_path]
    write_tsv(
        args.out / "p1280-evidence-manifest.tsv",
        [{"path": str(path.relative_to(root)), "sha256": sha(path)} for path in evidence],
    )


if __name__ == "__main__":
    main()
