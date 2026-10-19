# Passo 395 — relatório: modelagem de `Value::Tiling`

**Tipo:** modelagem de tipos primitivos (L1 — pureza; expande enum `Value` fechado per ADR-0017; zero I/O).
**Data:** 2026-06-22. **HEAD:** pós-`1129da547`.
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente
em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Modelou-se o tipo `Tiling` (padrão de azulejos / pattern fill) em L1, abrindo o portão
ADR-0017 para futuros tipos visuais.

- `01_core/src/entities/tiling.rs` — tipo L1 puro:
  - `Tiling { body, size, relative, spacing }`.
  - `TilingBody { Image(ImageElem), Gradient(Gradient), Color(Color) }`.
  - `TilingRelative { Itself, Parent }` (variante `Self` renomeada para `Itself` por ser
    palavra reservada em Rust).
  - `Tiling::new(body)` e `Tiling::to_color()` fallback.
- `01_core/src/entities/mod.rs` — regista o módulo `tiling`.
- `01_core/src/entities/value.rs` — adiciona `Value::Tiling(Arc<Tiling>)`, `type_name()`
  e `From<Tiling>`.
- `01_core/src/entities/paint.rs` — activa `Paint::Tiling(Tiling)`, construtores,
  `From<Tiling>` e `to_color()` fallback.
- `03_infra/src/export/stream.rs` — braço `Paint::Tiling` em `emit_stroke_paint` com
  fallback a cor representativa (scope-out ADR-0054 graded).
- `00_nucleo/prompts/entities/tiling.md` — L0 novo.
- `00_nucleo/prompts/entities/value.md` — secção aditiva P395.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` — `Tiling` tipo
  reclassificado de `ausente` para `implementado`; `tiling()` função permanece `ausente`
  (consumer P396).

`cargo test --workspace` + `crystalline-lint .` verdes; **13** testes novos passam
(8 em `entities/tiling.rs` + 2 em `entities/value.rs` + 3 em `entities/paint.rs`).

## Protocolo de Nucleação cumprido

1. Redigiu-se o L0 (`tiling.md`) e actualizou-se `value.md`; hashes propagados.
2. Modelou-se o tipo com TDD (construção, igualdade, clone, fallback de cor).
3. Linhagem `@prompt`/`@prompt-hash` actualizada via `crystalline-lint --fix-hashes`.

## Decisão de engenharia

A paridade (ADR-0107) é morfológica: `Tiling` existe no pipeline como valor de runtime e
como fonte de preenchimento (`Paint::Tiling`). Não se implementou `tiling()` (P396) nem
render PDF de pattern fill — ambos scope-out.

`TilingBody::Gradient` existe como placeholder, mas o consumer real fica para quando
`Gradient` (P262) e pattern fill forem integrados. `TilingBody::Image` também é
placeholder consumer-wise; o fallback de cor para imagem é preto (graded).

A variante `TilingRelative::Self` do plano foi renomeada para `Itself` porque `Self` é
palavra reservada em Rust. A semântica mantém-se.

A integração em `Paint` foi escolhida em vez de criar um enum `Fill` separado, porque o
cristalino já usa `Paint` como wrapper de fontes de cor/preenchimento (`Solid`,
`Gradient`). `Paint::Tiling` é portanto o equivalente a `Fill::Tiling` previsto no plano.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `Tiling::new(Color)` | construção com defaults | ✓ |
| `TilingBody::Color` → `to_color()` | mesma cor | ✓ |
| `TilingBody::Image` → `to_color()` | preto (fallback) | ✓ |
| `Value::Tiling(...).type_name()` | `"tiling"` | ✓ |
| `Value::Tiling` PartialEq | struct equality | ✓ |
| `Paint::Tiling` → `to_color()` | fallback | ✓ |
| Export stroke com `Paint::Tiling` | emite cor fallback | ✓ |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Value::Tiling(Arc<Tiling>)` compila e participa de matches | ✓ |
| 2 | `Tiling` struct com `TilingBody`/`TilingRelative`/size/spacing | ✓ |
| 3 | `Paint::Tiling` existe; consumers usam fallback Color | ✓ |
| 4 | Export emite Color fallback para Tiling | ✓ |
| 5 | Tests verdes; lint zero; hashes propagados | ✓ 13/13; `✓ No violations` |
| 6 | Inventário 148 actualizado | ✓ |
| 7 | L0 salvo e hashado antes do código | ✓ `tiling.md` + `value.md` |

## Artefactos

- Código: `01_core/src/entities/tiling.rs`, `entities/mod.rs`, `entities/value.rs`,
  `entities/paint.rs`, `03_infra/src/export/stream.rs`.
- L0: `00_nucleo/prompts/entities/tiling.md`, `00_nucleo/prompts/entities/value.md`.
- Testes: `entities/tiling.rs` (8), `entities/value.rs` (2), `entities/paint.rs` (3).
- Inventário 148 — `Tiling` tipo implementado; `tiling()` função ainda ausente.
- este relatório.

## Nota sobre o Tekt

Este passo é puramente modelagem — nenhum consumer real foi adicionado. A intrusão ficou
contida: um novo ficheiro entity, duas variantes de enum (`Value` e `Paint`) e um braço
no export. O portão ADR-0017 está agora aberto; tipos S futuros (`Bytes`, `Decimal`,
`Duration`, `Version`) podem fluir com o mesmo padrão.
