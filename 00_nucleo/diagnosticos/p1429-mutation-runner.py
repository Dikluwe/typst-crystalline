#!/usr/bin/env python3
"""Executa mutantes físicos P1429 em clones locais isolados.

Cada operador é um patch versionado aplicado sobre o commit candidato exato.
O runner nunca altera o checkout chamador. Saída JSON vai para o caminho
fornecido, tipicamente sob /tmp durante a execução adversarial.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

CANDIDATE = "925f63ba06d85050fc00d9926b63a370d1e501b5"
MUTANTS = [
    ("M01", "p1429_binding_normalizacao_casts_e_presenca"),
    ("M02", "p1429_assinatura_fechada_rejeita_casos_atribuiveis"),
    ("M03", "p1429_binding_normalizacao_casts_e_presenca"),
    ("M04", "p1429_assinatura_fechada_rejeita_casos_atribuiveis"),
    ("M05", "p1429_carrier_discrimina_valores_e_presenca_independentes"),
    ("M06", "p1429_size_resolve_na_largura_e_precedencia_independente"),
    ("M07", "p1429_rotas_bare_qualificada_e_wrappers_convergem"),
    ("M08", "p1429_carrier_discrimina_valores_e_presenca_independentes"),
    ("M09", "p1429_orientacao_superior_inferior_e_dotless"),
    ("M10", "p1429_size_resolve_na_largura_e_precedencia_independente"),
    ("M11", "p1429_orientacao_superior_inferior_e_dotless"),
    ("M12", "p1429_unknown_u01_u06_falha_fechado_sem_credito"),
]


def run(command: list[str], cwd: Path, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE,
                          stderr=subprocess.STDOUT, check=False, timeout=240)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--mutant", action="append", choices=[m[0] for m in MUTANTS])
    args = parser.parse_args()
    repo = args.repo.resolve()
    patch_dir = repo / "00_nucleo/diagnosticos/p1429-mutation-patches"
    selected = set(args.mutant or [m[0] for m in MUTANTS])
    root = Path(tempfile.mkdtemp(prefix="p1429-r4-"))
    # Reutiliza somente artefatos Cargo do checkout principal; o código-fonte
    # e o patch de cada mutante continuam isolados no clone próprio.
    target = repo / "target"
    results = []
    try:
        for mutant, killer in MUTANTS:
            if mutant not in selected:
                continue
            clone = root / mutant.lower()
            patch_path = patch_dir / f"{mutant}.patch"
            patch_sha = hashlib.sha256(patch_path.read_bytes()).hexdigest()
            started = time.monotonic()
            clone_result = run(
                ["git", "clone", "--quiet", "--shared", "--no-checkout", str(repo), str(clone)],
                repo,
            )
            if clone_result.returncode == 0:
                checkout = run(["git", "checkout", "--quiet", "--detach", CANDIDATE], clone)
            else:
                checkout = clone_result
            applied = checkout
            if checkout.returncode == 0:
                applied = run(["git", "apply", "--check", str(patch_path)], clone)
            if applied.returncode == 0:
                applied = run(["git", "apply", str(patch_path)], clone)
            command = ["cargo", "test", "-p", "typst-core", killer, "--", "--nocapture"]
            tested = applied
            if applied.returncode == 0:
                env = os.environ.copy()
                env["CARGO_TARGET_DIR"] = str(target)
                env["CARGO_INCREMENTAL"] = "0"
                tested = run(command, clone, env)
            infrastructure_ok = clone_result.returncode == checkout.returncode == applied.returncode == 0
            test_failed = "running 1 test" in tested.stdout and "test result: FAILED" in tested.stdout
            test_passed = "running 1 test" in tested.stdout and "test result: ok" in tested.stdout
            killed = infrastructure_ok and tested.returncode != 0 and test_failed
            classification = (
                "Violated" if killed else
                "Preserved" if infrastructure_ok and tested.returncode == 0 and test_passed else
                "Unknown"
            )
            results.append({
                "mutant": mutant,
                "patch": str(patch_path.relative_to(repo)),
                "patch_sha256": patch_sha,
                "killer": killer,
                "command": " ".join(command),
                "patch_applied": applied.returncode == 0,
                "test_exit_code": tested.returncode,
                "classification": classification,
                "killed": killed,
                "duration_seconds": round(time.monotonic() - started, 3),
                "output_tail": tested.stdout[-4000:],
            })
            shutil.rmtree(clone, ignore_errors=True)
    finally:
        payload = {
            "schema": "tekt.materialization.physical-mutation-run/v1",
            "candidate": CANDIDATE,
            "isolation": "separate local shared clone per mutant; shared Cargo target is cache only",
            "mutants": results,
            "valid": len(results),
            "violated": sum(r["classification"] == "Violated" for r in results),
            "preserved": sum(r["classification"] == "Preserved" for r in results),
            "unknown": sum(r["classification"] == "Unknown" for r in results),
        }
        payload["mutation_score"] = payload["violated"] / payload["valid"] if payload["valid"] else 0.0
        args.output.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")
        shutil.rmtree(root, ignore_errors=True)
    return 0 if results and all(r["killed"] for r in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
