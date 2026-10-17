# Lista de falsos V16 corrigidos no candidato

Medição: mesmo working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; estado e diffstat em before.json.gz. A comparação usa o base e53de6f; o pin local após publicação é 12797073972e375d9ab078cf6750c8b30b71e2b9.

| Consumer | Linha medida | Função |
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

Todos são braços explícitos `None`, já fechados no domínio externo Option. Outros padrões internos como `Some(_)` não ficam certificados por esta correção. Nenhum consumer Typst foi reescrito para esconder o erro do parser.
