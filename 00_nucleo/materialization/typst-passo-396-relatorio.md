# Passo 396 — relatório: constructor `tiling(...)` e consumers `Paint::Tiling`

**Tipo:** implementação de stdlib + integração layout/export (L1/L3; consumer real do tipo modelado em P395).  
**Data:** 2026-06-22. **HEAD:** pós-`4fd2c0f3d`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Materializou-se o constructor user-facing `tiling(...)` e abriu-se o caminho do tipo
`Tiling` pelo pipeline até ao layout/export, com fallback Color (scope-out ADR-0054).

- `00_nucleo/prompts/rules/stdlib/tiling-stdlib.md` — L0 do constructor.
- `01_core/src/rules/stdlib/visualize.rs`:
  - `native_tiling(...)` constrói `Value::Tiling` a partir de `Color`, `Content::Image`,
    `Str` (I/O via `world.read_bytes`), `Tiling` (identidade); rejeita `Gradient` com erro
    ADR-0054.
  - Helpers `extract_size`, `parse_relative`.
  - 7 testes unitários (`tiling_color_body`, `tiling_size_length_uniform`,
    `tiling_size_array`, `tiling_relative_parent`, `tiling_identity`,
    `tiling_invalid_body_errors`, `tiling_gradient_body_errors`).
  - Registo em `make_stdlib`: `scope.define("tiling", Value::Func(Func::native(...)))`.
- `01_core/src/rules/stdlib/shapes.rs`:
  - Refactor do fill de `Option<Color>` para `Option<Paint>` (`parse_paint`).
  - `rect`, `square`, `ellipse`, `circle`, `polygon`, `curve` agora aceitam
    `Color`, `Gradient` e `Tiling` como fill.
- `01_core/src/entities/elements/shape.rs` + `01_core/src/entities/content.rs`:
  - `ShapeElem.fill` e `Content::shape(...)` passam a `Option<Paint>`.
- `01_core/src/rules/layout/shape.rs` + `01_core/src/rules/layout/helpers.rs`:
  - `FrameItem::Shape.fill` continua `Option<Color>`; conversão via `paint.to_color()`.
- `03_infra/src/integration_tests.rs`:
  - Teste E2E `rect_fill_tiling_cai_no_fallback_color_no_pdf` — `#rect(fill: tiling(rgb(255,0,0)))`
    compila e o PDF emite fill vermelho via fallback.
- `03_infra/fixtures/p307b/reference/{05-gradient-linear,06-gradient-conic,07-multi-feature}.pdf`:
  - Snapshots regenerados porque o refactor de fill faz com que `gradient(...)` no fill seja
    agora reconhecido como `Paint::Gradient` e convertido para cor de fallback, em vez de
    ser ignorado e cair no stroke preto por omissão.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - `tiling(...)` reclassificado de `ausente` para `implementado`.
  - Entrada da Tabela C marcada como resolvida.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`.

## Protocolo de Nucleação cumprido

1. L0 (`tiling-stdlib.md`) escrito e hash propagado via `crystalline-lint --fix-hashes`.
2. TDD: testes unitários + E2E escritos antes/paralelamente à implementação.
3. Refactor type-safe de `Color` → `Paint` no fill dos shapes.
4. Snapshots afectados regenerados e justificados.

## Decisão de engenharia

A integração do `Tiling` no pipeline exigiu alargar o fill dos shapes de `Color` para `Paint`.
Essa mudança é mínima e type-safe: `FrameItem::Shape.fill` mantém `Option<Color>` (o contrato
com o PDF), e a conversão `Paint → Color` acontece exclusivamente nos dois pontos de layout
que constroem `FrameItem::Shape`. Assim, `Gradient` e `Tiling` como fill fluem até ao layout,
mas o exportador PDF continua a ver apenas cores (fallback ADR-0054).

A semântica real de `relative: "parent"` continua scope-out; o valor é armazenado e utilizado
como `"self"` no layout. `Gradient` como body de `tiling` é rejeitado explicitamente, uma vez
que `TilingBody::Gradient` é placeholder (P262 ainda não materializado como body válido).

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `tiling(rgb(255,0,0))` | `Value::Tiling` com `TilingBody::Color` | ✓ |
| `tiling(..., size: 50pt)` | `size` uniforme | ✓ |
| `tiling(..., size: (50pt, 30pt))` | `Size` diferenciado | ✓ |
| `tiling(..., relative: "parent")` | `TilingRelative::Parent` | ✓ |
| `tiling(tiling(...))` | identidade | ✓ |
| `tiling(123)` | erro de tipo | ✓ |
| `tiling(gradient.linear(...))` | erro ADR-0054 | ✓ |
| `#rect(fill: tiling(rgb(255,0,0)))` | PDF com fill vermelho (fallback) | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `native_tiling` implementado e registado em `make_stdlib` | ✓ |
| 2 | Refactor `ShapeElem.fill`/`Content::shape` para `Option<Paint>` | ✓ |
| 3 | Layout converte `Paint` → `Color` ao emitir `FrameItem::Shape` | ✓ |
| 4 | `Gradient` como body de `tiling` rejeitado com erro educacional | ✓ |
| 5 | Testes unitários + E2E passam | ✓ 8 unit + 1 E2E |
| 6 | Snapshots P307b afectados regenerados | ✓ |
| 7 | `cargo test --workspace` verde; `crystalline-lint .` zero | ✓ |
| 8 | Inventário 148 actualizado | ✓ |
| 9 | L0 salvo e hashado | ✓ `tiling-stdlib.md` |

## Artefactos

- Código:
  - `01_core/src/rules/stdlib/visualize.rs`
  - `01_core/src/rules/stdlib/shapes.rs`
  - `01_core/src/entities/elements/shape.rs`
  - `01_core/src/entities/content.rs`
  - `01_core/src/rules/layout/shape.rs`
  - `01_core/src/rules/layout/helpers.rs`
  - `03_infra/src/integration_tests.rs`
- L0: `00_nucleo/prompts/rules/stdlib/tiling-stdlib.md`.
- Snapshots: `03_infra/fixtures/p307b/reference/{05-gradient-linear,06-gradient-conic,07-multi-feature}.pdf`.
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- este relatório.

## Nota sobre o Tekt

Este passo termina a linha P395→P396: o tipo `Tiling` já existia, e agora tem constructor e
consumer de layout. A intrusão no pipeline foi pequena — um refactor local de `Color` para
`Paint` no fill dos shapes — mas permitiu que `Tiling` (e `Gradient`) sejam passados como
fill sem panic. O render PDF real de pattern fill e gradient fill continua scope-out,
mantendo a disciplina ADR-0054.
