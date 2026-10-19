# Correções do lote — projeções de Value

| Função | Arquivo | Situação anterior |
|---|---|---|
| `is_none` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `truthy` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | V16 emitido |
| `cast_bool` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_int` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_float` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | V16 emitido |
| `cast_str` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_array` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_dict` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_align` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_bytes` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | projeção latente sem V16 |
| `cast_decimal` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | V16 emitido |
| `cast_duration` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | V16 emitido |
| `cast_version` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | V16 emitido |
| `cast_regex` | [01_core/src/entities/value.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/value.rs) | V16 emitido |
| `element_or_type_with_name` | [01_core/src/compiler/eval/bindings/field_access/callee_error.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/bindings/field_access/callee_error.rs) | V16 emitido |
| `typst_type_name` | [01_core/src/compiler/layout/format_options.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/format_options.rs) | V16 emitido |
| `diagnostic_type_name` | [01_core/src/compiler/stdlib/structural/document.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/structural/document.rs) | V16 emitido |

Os dezessete pontos exigem revisão em compilação quando o enum cresce.
Os grupos com o mesmo resultado compartilham corpo e conservam as notas V19.
