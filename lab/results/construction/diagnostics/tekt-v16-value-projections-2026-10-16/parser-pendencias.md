# Emissões sobre None explícito — correção do parser pendente

A fixture mínima compilou sem wildcard, mas o linter denunciou None; uma
variante futura já produz E0004. A causa foi localizada em check_is_catchall
e get_catchall_ident do parser Rust. Não houve edição do linter nesta etapa.

| Consumer | Linha | Função |
|---|---:|---|
| [01_core/src/compiler/eval/control_flow.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/control_flow.rs:80) | 80 | `eval_while` |
| [01_core/src/compiler/eval/control_flow.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/control_flow.rs:178) | 178 | `run_for_loop` |
| [01_core/src/compiler/eval/rules/set.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/rules/set.rs:862) | 862 | `eval_set_rule` |
| [01_core/src/compiler/layout/helpers.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/helpers.rs:433) | 433 | `resolve_pt` |
| [01_core/src/compiler/layout/measurement.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/measurement.rs:376) | 376 | `measure_content_constrained` |
| [01_core/src/compiler/layout/transform.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/transform.rs:464) | 464 | `layout_native` |
| [01_core/src/compiler/stdlib/layout/grid.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/layout/grid.rs:32) | 32 | `extract_tracks` |
| [03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs](/repos/Antigravity/typst-crystalline/03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs:117) | 117 | `expected_af_relationship_name` |
| [03_infra/src/font_metrics/basic.rs](/repos/Antigravity/typst-crystalline/03_infra/src/font_metrics/basic.rs:65) | 65 | `edge_offset_pt` |

O defeito confirmado é a classificação de None como binding genérico.
Isso não certifica a exaustividade de braços internos como Some(_): eles
precisam de análise própria. Não isentar todos os identificadores maiúsculos,
pois um identificador incomum também pode ser binding verdadeiro.

[Reprodução](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16/none-probe-result.json) · [Análise](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16/none-parser-analysis.json).
