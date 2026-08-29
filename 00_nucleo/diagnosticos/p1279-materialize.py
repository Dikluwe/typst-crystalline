#!/usr/bin/env python3
"""Materializa recibos e certificado do diagnóstico causal P1279."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import shutil
from pathlib import Path


GENERAL = (
    "p1279-corpus.tsv",
    "p1279-oracle-budgets.tsv",
    "p1279-results.tsv",
    "p1279-interval-cost.tsv",
    "p1279-graph.tsv",
    "p1279-raster.tsv",
    "p1279-product-boundary.tsv",
    "p1279-determinism.tsv",
    "p1279-invalid-domain.tsv",
    "p1279-pairs.tsv",
    "p1279-attacks-executed.tsv",
    "p1279-summary.json",
)
OFFSET = (
    "p1279-offset-witnesses.tsv",
    "p1279-offset-attacks.tsv",
    "p1279-offset-summary.json",
)
REPRO_GENERAL = tuple(name for name in GENERAL if name != "p1279-summary.json")
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
    parser.add_argument("--offset-direct", type=Path, required=True)
    parser.add_argument("--offset-inverse", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    args.out = args.out.resolve()
    root = Path(__file__).resolve().parents[2]

    require_equal(args.general_direct, args.general_repro, REPRO_GENERAL)
    require_equal(args.offset_direct, args.offset_inverse, OFFSET)
    for name in GENERAL:
        shutil.copyfile(args.general_direct / name, args.out / name)
    for name in OFFSET:
        shutil.copyfile(args.offset_direct / name, args.out / name)

    general = json.loads((args.out / "p1279-summary.json").read_text())
    offset = json.loads((args.out / "p1279-offset-summary.json").read_text())
    focal = json.loads(
        (root / "00_nucleo/diagnosticos/p1278-focal-summary.json").read_text()
    )
    pairs = {row["pair"]: row for row in read_tsv(args.out / "p1279-pairs.tsv")}
    old_pairs = {
        row["pair"]: row
        for row in read_tsv(root / "00_nucleo/diagnosticos/p1278-pairs.tsv")
    }
    product = read_tsv(args.out / "p1279-product-boundary.tsv")
    invalid = read_tsv(args.out / "p1279-invalid-domain.tsv")
    determinism = read_tsv(args.out / "p1279-determinism.tsv")
    general_attacks = read_tsv(args.out / "p1279-attacks-executed.tsv")
    offset_attacks = read_tsv(args.out / "p1279-offset-attacks.tsv")

    if general["fixtures"] != 192 or general["preserved"] != 158:
        raise ValueError("general population or expected Luma control drift")
    if any(pairs[pair]["classification"] != "Generalization-Preserved" for pair in POLAR):
        raise ValueError("polar pair did not close")
    if any(row["status"] != "Preserved" for row in product + invalid + determinism):
        raise ValueError("product, invalid or determinism gate failed")
    if any(row["status"] != "REJECTED" for row in general_attacks + offset_attacks):
        raise ValueError("mutant survived")

    final_pairs = []
    causal = []
    for pair, row in pairs.items():
        old = old_pairs[pair]
        before, total = fraction(old["valid"])
        after, after_total = fraction(row["valid"])
        polar = pair in POLAR
        final_pairs.append(
            {
                "pair": pair,
                "scope": "polar" if polar else "luma-negative-control",
                "p1278": old["classification"],
                "p1279": row["classification"],
                "valid": row["valid"],
                "numeric": row["numeric"],
                "raster": row["raster"],
                "invalid_rejected": row["invalid_rejected"],
                "product_fallback": "Preserved",
                "final_classification": row["classification"],
                "productive_promotion": "false",
            }
        )
        causal.append(
            {
                "pair": pair,
                "scope": "polar" if polar else "luma-negative-control",
                "p1278_valid": old["valid"],
                "p1279_valid": row["valid"],
                "valid_delta": after - before,
                "population_stable": str(total == after_total == 24).lower(),
                "cause": (
                    "diagnostic-f64-transport"
                    if polar and after > before
                    else "unchanged-control"
                ),
            }
        )
    write_tsv(args.out / "p1279-final-pairs.tsv", final_pairs)
    write_tsv(args.out / "p1279-causal-adjudication.tsv", causal)

    receipts = [
        {
            "run": "general-direct",
            "population": "192 fixtures + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.general_direct / "p1279-results.tsv"),
            "reproduction": "matched-general-repro",
            "status": "ACCEPTED",
        },
        {
            "run": "general-repro",
            "population": "192 fixtures + 56 invalid + 384 determinism",
            "primary_sha256": sha(args.general_repro / "p1279-results.tsv"),
            "reproduction": "matched-general-direct",
            "status": "ACCEPTED",
        },
        {
            "run": "offset-direct",
            "population": "5 stopsets / 6 mutants",
            "primary_sha256": sha(args.offset_direct / "p1279-offset-witnesses.tsv"),
            "reproduction": "matched-offset-inverse",
            "status": "ACCEPTED",
        },
        {
            "run": "offset-inverse",
            "population": "5 stopsets / 6 mutants",
            "primary_sha256": sha(args.offset_inverse / "p1279-offset-witnesses.tsv"),
            "reproduction": "matched-offset-direct",
            "status": "ACCEPTED",
        },
        {
            "run": "focal-inherited-P1278",
            "population": "48 groups / 336 observations",
            "primary_sha256": sha(root / "00_nucleo/diagnosticos/p1278-focal.tsv"),
            "reproduction": "immutable predecessor evidence",
            "status": "ACCEPTED-NO-PRODUCT-CODE-CHANGE",
        },
    ]
    write_tsv(args.out / "p1279-execution-receipts.tsv", receipts)

    write_tsv(
        args.out / "p1279-discarded-attempts.tsv",
        [
            {
                "attempt": "D01",
                "class": "discovery",
                "result": "numeric substring matches were not treated as an explicit P1279 spec",
                "used_for_verdict": "false",
            },
            {
                "attempt": "D02",
                "class": "budget-rounding",
                "result": "P1278 p95 excess was not rounded into success; harness cause measured instead",
                "used_for_verdict": "false",
            },
        ],
    )

    write_tsv(
        args.out / "p1279-role-capabilities.tsv",
        [
            {
                "role": "oracle",
                "artifact": "p1279-oracle-budgets.tsv + temporary vanilla SVG/masks",
                "write_scope": "oracle before candidate",
                "read_scope": "frozen P1278 inputs and vanilla binary",
                "isolation": "logical-order-only",
            },
            {
                "role": "harness-implementation",
                "artifact": "p1279_generalize.py",
                "write_scope": "diagnostic transport only",
                "read_scope": "P1278 harness, public metadata and frozen contract",
                "isolation": "not isolated",
            },
            {
                "role": "adversary",
                "artifact": "p1279-offset-attacks.tsv",
                "write_scope": "attack receipts",
                "read_scope": "five oracle stopsets and product boundary",
                "isolation": "not isolated",
            },
            {
                "role": "judge",
                "artifact": "p1279-final-pairs.tsv + certificate",
                "write_scope": "adjudication only",
                "read_scope": "all P1279 receipts",
                "isolation": "not isolated",
            },
        ],
    )

    snapshot = args.working_tree_snapshot.read_text().rstrip()
    executor_inputs = (
        root / "typst-passo-1279.md",
        root / "lab/parity/matrix/p1279_generalize.py",
        root / "lab/parity/matrix/p1279_offset_attacks.py",
        root / "00_nucleo/diagnosticos/p1279-materialize.py",
    )
    snapshot += "\n\nP1279 executor inputs at materialization\n"
    snapshot += "\n".join(f"{path.relative_to(root)}\t{sha(path)}" for path in executor_inputs)
    snapshot += "\n"
    (args.out / "p1279-working-tree-snapshot.txt").write_text(snapshot)

    polar_before = sum(fraction(old_pairs[pair]["valid"])[0] for pair in POLAR)
    polar_after = sum(fraction(pairs[pair]["valid"])[0] for pair in POLAR)
    final = {
        "verdict": "POLAR-GENERALIZATION-PRESERVED-DIAGNOSTIC-ONLY",
        "head": general["head"],
        "measured_at": general["measured_at"],
        "general_fixtures": general["fixtures"],
        "general_preserved": general["preserved"],
        "polar_fixtures": 144,
        "polar_preserved_p1278": polar_before,
        "polar_preserved_p1279": polar_after,
        "polar_delta": polar_after - polar_before,
        "luma_control_preserved": 14,
        "lossless_stopsets": offset["lossless_exact"],
        "lossless_stopsets_total": offset["fixtures"],
        "p15_offset_diffs": offset["p15_offset_diffs"],
        "pre_eval_offset_diffs": offset["pre_eval_offset_diffs"],
        "f32_offset_diffs": offset["f32_offset_diffs"],
        "invalid_rejected": general["invalid_rejected"],
        "determinism_pass": general["determinism_pass"],
        "p1279_mutants_rejected": len(general_attacks) + len(offset_attacks),
        "p1279_mutants_total": len(general_attacks) + len(offset_attacks),
        "focal_inherited_preserved": focal["preserved"],
        "focal_inherited_total": focal["observations"],
        "productive_promotions_applied": 0,
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1279-final-summary.json").write_text(
        json.dumps(final, sort_keys=True, indent=2) + "\n"
    )

    inputs = (
        *executor_inputs,
        root / "00_nucleo/diagnosticos/p1266-generalization-matrix.tsv",
        root / "00_nucleo/diagnosticos/p1276-contract.tsv",
        root / "00_nucleo/diagnosticos/p1276-unknown-policy.tsv",
        root / "00_nucleo/diagnosticos/p1276-attacks.tsv",
        root / "00_nucleo/diagnosticos/p1278-pairs.tsv",
        root / "00_nucleo/diagnosticos/p1278-results.tsv",
        root / "00_nucleo/diagnosticos/p1278-focal.tsv",
        root / "00_nucleo/diagnosticos/p1278-focal-summary.json",
        root / "lab/parity/matrix/p1278_probe/src/main.rs",
        root / "lab/parity/matrix/p1278_probe/Cargo.toml",
        root / "lab/parity/matrix/p1278_probe/Cargo.lock",
        Path("/usr/local/bin/typst"),
        root / "target/debug/typst",
    )
    write_tsv(
        args.out / "p1279-input-manifest.tsv",
        [{"path": str(path), "sha256": sha(path)} for path in inputs],
    )

    pair_lines = "\n".join(
        f"| {pair} | {old_pairs[pair]['valid']} | {pairs[pair]['valid']} | {pairs[pair]['classification']} |"
        for pair in POLAR
    )
    certificate = f"""# Certificado P1279 — identidade de offsets do harness polar

## Veredito

`POLAR-GENERALIZATION-PRESERVED-DIAGNOSTIC-ONLY`.

O transporte lossless de offsets removeu os 10 falsos negativos de P1278. Os
seis pares Linear/Radial × Oklch/Hsl/Hsv fecharam 144/144 fixtures gerais sem
mudar algoritmo produtivo, L0, budget ou fronteira SVG.

| Par | P1278 válido | P1279 válido | Classificação P1279 |
|---|---:|---:|---|
{pair_lines}

## Causa medida

- stopsets lossless idênticos ao vanilla: {offset['lossless_exact']}/{offset['fixtures']};
- mutante `.15g`: {offset['p15_offset_diffs']} offsets divergentes;
- mutante pré-avaliação: {offset['pre_eval_offset_diffs']} offset divergente;
- mutante via `f32`: {offset['f32_offset_diffs']} offsets divergentes;
- ataques P1279: {len(general_attacks) + len(offset_attacks)}/{len(general_attacks) + len(offset_attacks)} rejeitados.

Luma permaneceu controle negativo em 14/48. O focal público P1278 foi herdado
imutavelmente em {focal['preserved']}/{focal['observations']} porque P1279 não
alterou código do produto.

## Fronteira produtiva

Os 192/192 grupos conservaram fallback e
`productive_promotions_applied = 0`. O certificado cobre somente o fragmento
executado; promover os seis pares muda comportamento por defeito e exige uma
etapa ADR-0127 explícita.

## Proveniência

- baseline: upstream/main `a51e02804` ratificado;
- HEAD medido: `{general['head']}` com working tree não commitado;
- medição: `{general['measured_at']}`;
- resultados reproduzidos: `{sha(args.general_direct / 'p1279-results.tsv')}`;
- testemunhos de offset reproduzidos: `{sha(args.offset_direct / 'p1279-offset-witnesses.tsv')}`.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Houve separação lógica de entradas,
ordem e artefatos, mas os papéis compartilharam checkout e leitura.
"""
    (args.out / "typst-p1279-offset-identity-certificate.md").write_text(certificate)

    evidence = sorted(
        path
        for path in args.out.glob("p1279-*")
        if path.name != "p1279-evidence-manifest.tsv"
    ) + [args.out / "typst-p1279-offset-identity-certificate.md"]
    write_tsv(
        args.out / "p1279-evidence-manifest.tsv",
        [{"path": str(path.relative_to(root)), "sha256": sha(path)} for path in evidence],
    )


if __name__ == "__main__":
    main()
