# Correções do lote — seletores de show

| Função | Arquivo | Situação anterior |
|---|---|---|
| `native_element_base` | [01_core/src/compiler/eval/selector_matching.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/selector_matching.rs) | V16 emitido |
| `is_text_rule` | [01_core/src/compiler/eval/selector_matching/textual.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/selector_matching/textual.rs) | predicado latente sem V16 |
| `textual_selector_matches` | [01_core/src/compiler/eval/selector_matching/textual.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/selector_matching/textual.rs) | V16 emitido |
| `textual_match_ranges` | [01_core/src/compiler/eval/selector_matching/textual.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/selector_matching/textual.rs) | V16 emitido |
| `captures_equation_rule` | [01_core/src/compiler/realize/bare_math.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/realize/bare_math.rs) | V16 emitido |
| `captures_rule` | [01_core/src/compiler/realize/table_cells.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/realize/table_cells.rs) | V16 emitido |

Os seis pontos exigem revisão em compilação quando o enum cresce.
Os grupos com o mesmo resultado compartilham corpo e conservam as notas V19.
