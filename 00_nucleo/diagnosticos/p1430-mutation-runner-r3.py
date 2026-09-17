#!/usr/bin/env python3
"""Campanha física P1430 contra um commit candidato imutável.

Cada mutante é aplicado num clone local fresco. Os testes e entradas protegidas
nunca são editados. Um exit 101 com testemunha do teste focal é KILLED; exit 0
é SURVIVED; qualquer outra condição é UNKNOWN.
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


CANDIDATE = "f7dcbe86ca7618bf1e6d840e51ea168f7b1c3c8e"


def replacement(path: str, old: str, new: str, occurrence: int = 1):
    return {"path": path, "old": old, "new": new, "occurrence": occurrence}


MUTANTS = [
    ("M01-drop-or-alias-binding", "p1430_binding_aridade_variadica_e_casts_de_children", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''    dict.insert(
        "cases".into(),
        Value::Func(crate::entities::func::Func::native("cases", native_math_cases)),
    );
''',
        '''    // P1430 physical mutant: binding omitted.
''')),
    ("M02-break-variadic-cardinality", "p1430_binding_aridade_variadica_e_casts_de_children", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''    let children = args
        .items
        .iter()
        .map(|value| math_cases_child(value, args.span))
''',
        '''    let children = args
        .items
        .iter()
        .rev()
        .map(|value| math_cases_child(value, args.span))
''')),
    ("M03-relax-positional-content-cast", "p1430_binding_aridade_variadica_e_casts_de_children", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
''',
        '''        Value::None => Ok(Content::Empty),
        Value::Int(_) => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
''', 3)),
    ("M04-open-named-signature", "p1430_rejeicoes_atribuiveis_e_duplicados", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )])
        }
''',
        '''        _other => return Ok(()),
''', 2)),
    ("M05-relax-delimiter-cast", "p1430_delimitadores_malformados_sao_rejeitados", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''    if !matches!(
        crate::entities::math_class::default_math_class(character),
        Some(
            crate::entities::math_class::MathClass::Opening
                | crate::entities::math_class::MathClass::Closing
                | crate::entities::math_class::MathClass::Fence
        )
    ) {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("invalid delimiter: \\"{character}\\""),
        )]);
    }
''',
        '''    // P1430 physical mutant: arbitrary one-codepoint glyph accepted.
''')),
    ("M06-corrupt-delimiter-normalization", "p1430_delimitadores_opcionais_gap_e_explicitude", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs", "        '[' => ']',\n", "        '[' => '[',\n")),
    ("M07-relax-reverse-cast", "p1430_rejeicoes_atribuiveis_e_duplicados", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''        "reverse" => {
            let Value::Bool(reverse) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected boolean, found {}", vanilla_type_name_class(value)),
                )]);
            };
            options.reverse = *reverse;
            options.explicit.reverse |= explicit;
        }
''',
        '''        "reverse" => {
            options.reverse = match value {
                Value::Bool(reverse) => *reverse,
                Value::Int(value) => *value != 0,
                _ => false,
            };
            options.explicit.reverse |= explicit;
        }
''')),
    ("M08-relax-or-flatten-gap-cast", "p1430_rejeicoes_atribuiveis_e_duplicados", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''                Value::Length(gap) => Rel { rel: 0.0, abs: *gap },
                other => {
''',
        '''                Value::Length(gap) => Rel { rel: 0.0, abs: *gap },
                Value::Int(_) => Rel { rel: 0.0, abs: Length::ZERO },
                other => {
''', 2)),
    ("M09-collapse-explicit-presence", "p1430_delimitadores_opcionais_gap_e_explicitude", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        "            options.explicit.delim |= explicit;\n",
        "            options.explicit.delim = false;\n", 2)),
    ("M10-break-cascade-precedence", "p1430_gap_relativo_e_precedencia_de_cascata", replacement(
        "01_core/src/compiler/math/layout/cases.rs",
        '''        let gap = if elem.explicit.gap {
            elem.gap
        } else {
            cascaded_gap.unwrap_or(Rel { rel: 0.0, abs: Length::em(0.2) })
        };
''',
        '''        let gap = cascaded_gap.unwrap_or(elem.gap);
''')),
    ("M11-fabricate-presence-from-cascade", "p1430_cascata_nao_muta_carrier_e_explicito_prevalece", replacement(
        "01_core/src/compiler/eval/math.rs",
        '''                "cases" => {
                    let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
                    apply_p1293_math_constructor("cases", args, scopes, ctx, engine)
                }
''',
        '''                "cases" => {
                    let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
                    let content = apply_p1293_math_constructor("cases", args, scopes, ctx, engine)?;
                    let Content::MathCases(elem) = content else { return Ok(content) };
                    let mut explicit = elem.explicit;
                    explicit.delim |= engine.styles.custom("math.cases.delim").is_some();
                    explicit.reverse |= engine.styles.custom("math.cases.reverse").is_some();
                    explicit.gap |= engine.styles.custom("math.cases.gap").is_some();
                    Ok(Content::math_cases_full(
                        elem.children.clone(), elem.delim, elem.reverse, elem.gap, explicit,
                    ))
                }
''')),
    ("M12-diverge-bare-and-qualified-routes", "p1430_rotas_bare_e_qualificada_convergem", replacement(
        "01_core/src/compiler/eval/math.rs",
        '''                    apply_p1293_math_constructor("cases", args, scopes, ctx, engine)
''',
        '''                    let content = apply_p1293_math_constructor("cases", args, scopes, ctx, engine)?;
                    let Content::MathCases(elem) = content else { return Ok(content) };
                    let mut children = elem.children.clone();
                    children.pop();
                    Ok(Content::math_cases_full(children, elem.delim, elem.reverse, elem.gap, elem.explicit))
''')),
    ("M13-pre-resolve-or-reevaluate-children", "p1430_bare_converge_e_preserva_alignpoint_no_carrier", replacement(
        "01_core/src/compiler/stdlib/structural/math.rs",
        '''        Value::Content(Content::MathIdent(text)) => Ok(Content::MathText(text.clone())),
        Value::Content(content) => Ok(content.clone()),
''',
        '''        Value::Content(Content::MathIdent(text)) => Ok(Content::MathText(text.clone())),
        Value::Content(Content::MathSequence(nodes)) => Ok(Content::MathSequence(
            std::sync::Arc::from(nodes.iter().filter(|node| !matches!(node, Content::MathAlignPoint(_))).cloned().collect::<Vec<_>>())
        )),
        Value::Content(content) => Ok(content.clone()),
''')),
    ("M14-drop-carrier-field-in-reconstruction", "p1430_reconstrucao_preserva_campos_causais", replacement(
        "01_core/src/compiler/math/layout/mod.rs",
        '''        Content::MathCases(e) => Content::math_cases_full(
            e.children.iter().map(apply_math_default).collect(),
            e.delim,
            e.reverse,
            e.gap,
            e.explicit,
        ),
''',
        '''        Content::MathCases(e) => Content::math_cases_full(
            e.children.iter().map(apply_math_default).collect(),
            e.delim,
            e.reverse,
            crate::entities::rel::Rel {
                rel: 0.0,
                abs: crate::entities::layout_types::Length::em(0.2),
            },
            e.explicit,
        ),
''')),
    ("M15-corrupt-repr-morphology", "p1430_binding_aridade_variadica_e_casts_de_children", replacement(
        "01_core/src/compiler/eval/repr.rs",
        "                pretty_array_like(&children, children.len() == 1)\n",
        "                pretty_array_like(&children, false)\n", 1)),
    ("M16-split-align-points-as-global-columns", "p1430_alignpoint_resolve_tarde_e_ramos_usam_denominador", replacement(
        "01_core/src/compiler/math/layout/cases.rs",
        '''        let grid_box = if any_align {
''',
        '''        let any_align = false;
        let grid_box = if any_align {
''')),
    ("M17-use-wrong-delimiter-side-or-direction", "p1430_delimitador_respeita_direcao_e_none", replacement(
        "01_core/src/compiler/math/layout/cases.rs",
        "        let delim_char = if reverse { delim.1 } else { delim.0 };\n",
        "        let delim_char = delim.0;\n")),
    ("M18-resolve-gap-against-wrong-basis", "p1430_gap_relativo_e_precedencia_de_cascata", replacement(
        "01_core/src/compiler/math/layout/cases.rs",
        '''        let row_gap =
            Pt(gap.resolve(Length::pt(style.size.val())).resolve_pt(style.size.val()));
''',
        '''        let row_gap = Pt(0.2 * style.size.val());
''')),
    ("M19-regress-established-cases-geometry", "p1430_extents_incluem_delimitador_sem_padding_redundante", replacement(
        "01_core/src/compiler/math/layout/cases.rs",
        "        let mut x = Pt(0.0);\n",
        "        let mut x = Pt(style.size.val() * 0.1);\n")),
    ("M20-break-legacy-wrapper", "p1430_wrapper_legado_preserva_geometria_de_rows", replacement(
        "01_core/src/entities/content.rs",
        '''                for cell in cells {
                    items.push(Content::math_align_point());
                    items.push(cell);
                }
''',
        '''                for cell in cells {
                    items.push(cell);
                }
''')),
    ("M21-credit-unknown", "V01-V05", replacement(
        "00_nucleo/diagnosticos/p1430-certification-policy.py",
        '''    if classification == "Unknown":
        mutation_credit = 0
        score_numerator_delta = 0
        score_denominator_delta = int(valid)
        blocks_gate = valid
        classification_matches = False
''',
        '''    if classification == "Unknown":
        mutation_credit = int(valid)
        score_numerator_delta = int(valid)
        score_denominator_delta = int(valid)
        blocks_gate = valid
        classification_matches = False
''')),
]


def run(cmd, cwd: Path, env=None, timeout=180):
    started = time.monotonic()
    proc = subprocess.run(
        cmd,
        cwd=cwd,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=timeout,
    )
    return proc.returncode, proc.stdout[-65536:], round(time.monotonic() - started, 3)


def apply_mutation(root: Path, spec):
    path = root / spec["path"]
    data = path.read_text()
    count = data.count(spec["old"])
    occurrence = spec.get("occurrence", 1)
    if count < occurrence:
        raise RuntimeError(
            f"expected occurrence {occurrence} in {spec['path']}, found {count}"
        )
    before, match, after = data.rpartition(spec["old"]) if occurrence == count else (None, None, None)
    if occurrence == count:
        path.write_text(before + spec["new"] + after)
        return
    parts = data.split(spec["old"])
    path.write_text(
        spec["old"].join(parts[:occurrence])
        + spec["new"]
        + spec["old"].join(parts[occurrence:])
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", default=".")
    parser.add_argument("--output", required=True)
    parser.add_argument("--only")
    args = parser.parse_args()
    repo = Path(args.repo).resolve()
    output = Path(args.output).resolve()
    patch_dir = output.parent / "p1430-mutation-patches-r3"
    patch_dir.mkdir(parents=True, exist_ok=True)
    campaign_root = Path(tempfile.mkdtemp(prefix="p1430-mutants-", dir="/tmp"))
    env = os.environ.copy()
    env["RUSTFLAGS"] = "-Awarnings"
    env["CARGO_TARGET_DIR"] = "/tmp/p1430-mutation-target-r3"
    results = []
    try:
        selected = [m for m in MUTANTS if args.only is None or m[0] == args.only]
        for index, (mid, test, spec) in enumerate(selected, 1):
            clone = campaign_root / f"{index:02d}-{mid}"
            code, clone_out, clone_secs = run(
                ["git", "clone", "--shared", "--no-checkout", str(repo), str(clone)],
                campaign_root,
                timeout=120,
            )
            if code != 0:
                results.append({"id": mid, "classification": "UNKNOWN", "reason": "clone_failed", "output": clone_out, "seconds": clone_secs})
                continue
            code, checkout_out, checkout_secs = run(["git", "checkout", "--detach", CANDIDATE], clone, timeout=120)
            if code != 0:
                results.append({"id": mid, "classification": "UNKNOWN", "reason": "checkout_failed", "output": checkout_out, "seconds": checkout_secs})
                continue
            try:
                apply_mutation(clone, spec)
            except Exception as exc:
                results.append({"id": mid, "classification": "UNKNOWN", "reason": "mutation_site", "output": str(exc)})
                continue
            patch = subprocess.run(["git", "diff", "--binary"], cwd=clone, text=True, stdout=subprocess.PIPE, check=True).stdout
            patch_path = patch_dir / f"{mid}.patch"
            patch_path.write_text(patch)
            patch_sha = hashlib.sha256(patch.encode()).hexdigest()
            if mid == "M21-credit-unknown":
                command = [
                    "python3",
                    "00_nucleo/diagnosticos/p1430-certification-policy-oracle.py",
                    "--policy",
                    "00_nucleo/diagnosticos/p1430-certification-policy.py",
                ]
            else:
                command = ["cargo", "test", "-p", "typst-core", test, "--", "--nocapture"]
            try:
                code, test_out, seconds = run(command, clone, env=env, timeout=240)
                if mid == "M21-credit-unknown" and code == 1 and '"status":"FAIL"' in test_out:
                    classification, reason = "KILLED", "V03_V04_policy_oracle_failed_attributably"
                elif code == 101 and "test result: FAILED" in test_out:
                    classification, reason = "KILLED", "focused_test_failed_attributably"
                elif code == 0:
                    classification, reason = "SURVIVED", "focused_test_passed"
                else:
                    classification, reason = "UNKNOWN", "non_attributable_execution"
            except subprocess.TimeoutExpired as exc:
                code, seconds = None, 240.0
                test_out = (exc.stdout or "")[-65536:] if isinstance(exc.stdout, str) else ""
                classification, reason = "UNKNOWN", "timeout"
            results.append({
                "id": mid,
                "test": test,
                "patch": str(patch_path.relative_to(output.parent)),
                "patch_sha256": patch_sha,
                "command": " ".join(command),
                "exit_code": code,
                "classification": classification,
                "reason": reason,
                "seconds": seconds,
                "output_tail": test_out[-8000:],
            })
    finally:
        shutil.rmtree(campaign_root, ignore_errors=True)

    killed = sum(item.get("classification") == "KILLED" for item in results)
    survived = sum(item.get("classification") == "SURVIVED" for item in results)
    unknown = sum(item.get("classification") == "UNKNOWN" for item in results)
    receipt = {
        "schema": "tekt.materialization.physical-mutation-results/v2",
        "revision": "R3-complete-after-M21-refinement",
        "step": 1430,
        "candidate_commit": CANDIDATE,
        "attestation": "executado sem atestacao de isolamento",
        "executor": "agent:/root/p1430_adversary",
        "baseline_green": "recorded separately before this runner",
        "policy": {"unknown_credit": 0, "required_score": 1.0},
        "summary": {
            "total": len(results),
            "killed": killed,
            "survived": survived,
            "unknown": unknown,
            "mutation_score": killed / len(results) if results else 0.0,
        },
        "results": results,
        "final_verdict": "NOT_EMITTED_BY_ADVERSARY",
    }
    output.write_text(json.dumps(receipt, indent=2, ensure_ascii=False) + "\n")
    print(json.dumps(receipt["summary"], sort_keys=True))


if __name__ == "__main__":
    main()
