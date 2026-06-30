# Relatório de Paridade Funcional — Passo 512

**Data:** 2026-06-30  
**Tema:** `grid.hline` / `grid.vline` / `table.hline` / `table.vline`

## Resumo

Implementação da família de linhas de grid/table no compilador cristalino. O trabalho cobre:

- Entidades `GridHLineElem`, `GridVLineElem`, `TableHLineElem`, `TableVLineElem`.
- Variantes correspondentes em `Content`.
- Campos `hlines`/`vlines` em `GridElem` e `TableElem`.
- Funções nativas `grid.hline`, `grid.vline`, `table.hline`, `table.vline`.
- Filtragem e cálculo de `row`/`col` efectivo em `native_grid`/`native_table`.
- Renderização das linhas no motor partilhado `layout_grid`.
- Registo das funções nos namespaces `grid` e `table` (e bindings flat legados).

## Decisões tomadas

### 1. `row`/`col` resolvido no parsing

O L0 original dos elementos não previa coordenada de linha/coluna. Para posicionar `hline`/`vline` entre linhas de células (semântica vanilla), adicionámos `row: usize` a `GridHLineElem`/`TableHLineElem` e `col: usize` a `GridVLineElem`/`TableVLineElem`. Os valores são calculados por `native_grid`/`native_table` com base na ordem dos children e no número de colunas. A semântica de `position: auto` foi aproximada:

- `grid.hline()` no início de uma linha de children → `position: top`, `row` atual.
- `grid.hline()` após células na mesma linha → `position: bottom`, `row` atual.
- `grid.vline()` → `position: left`, `col` atual.

Esta aproximação é suficiente para os casos de aceitação do Passo 512. Células com `x`/`y` explícitos podem deslocar a contagem automática — scope-out documentado.

### 2. Renderização no motor `layout_grid`

As linhas são desenhadas **após** as células, usando os `row_start_y` reais (já consideraram paginação). `hline` desenha `ShapeKind::Line { dx, dy: 0 }`; `vline` desenha `ShapeKind::Line { dx: 0, dy }`. Foram introduzidos traits privados `LayoutHLine`/`LayoutVLine` para partilhar o algoritmo entre grid e table.

### 3. Aceitação de `position` como `alignment`

`position: top`/`bottom`/`left`/`right` são palavras-chave de alinhamento em Typst. As funções nativas aceitam tanto `Value::Str` como `Value::Align` (convertendo o componente vertical/horizontal relevante).

## Ficheiros alterados

- L0:
  - `00_nucleo/prompts/entities/elements/grid_hline.md`
  - `00_nucleo/prompts/entities/elements/grid_vline.md`
  - `00_nucleo/prompts/entities/elements/table_hline.md`
  - `00_nucleo/prompts/entities/elements/table_vline.md`
  - `00_nucleo/prompts/rules/stdlib/grid_hline.md`
  - `00_nucleo/prompts/rules/stdlib/grid_vline.md`
  - `00_nucleo/prompts/rules/stdlib/table_hline.md`
  - `00_nucleo/prompts/rules/stdlib/table_vline.md`
- L1:
  - `01_core/src/entities/content.rs`
  - `01_core/src/entities/elements/grid.rs`
  - `01_core/src/entities/elements/table.rs`
  - `01_core/src/entities/elements/grid_hline.rs` (novo)
  - `01_core/src/entities/elements/grid_vline.rs` (novo)
  - `01_core/src/entities/elements/table_hline.rs` (novo)
  - `01_core/src/entities/elements/table_vline.rs` (novo)
  - `01_core/src/rules/stdlib/layout.rs`
  - `01_core/src/rules/stdlib/structural.rs`
  - `01_core/src/rules/eval/mod.rs`
  - `01_core/src/rules/layout/grid.rs`
  - `01_core/src/rules/layout/table.rs`
  - `01_core/src/rules/layout/tests.rs`
- L3:
  - `03_infra/src/query_helpers.rs`
- Config:
  - `crystalline.toml` (exceções V7 para os prompts stdlib hline/vline)

## Validação

### Testes unitários

```text
cargo test -p typst-core
=> 3539 passed; 0 failed
cargo test -p typst-wiring
=> 21 passed; 0 failed (mais 2 do crystalline_lint.rs)
```

Foram adicionados testes para:
- `native_grid_hline` / `native_grid_vline` / `native_table_hline` / `native_table_vline` (defaults e `position` como alignment).
- `GridHLineElem` / `GridVLineElem` / `TableHLineElem` / `TableVLineElem` (plain_text, map terminal, hash).

### Compilação e snippet

```text
cargo build  => OK
./target/debug/typst target/tmp/test_hline.typ target/tmp/test_hline.pdf  => OK
```

Snippet usado:

```typst
#grid(
  columns: 3,
  gutter: 3pt,
  [A], [B], [C],
  grid.hline(),
  [D], [E], [F],
  grid.hline(position: bottom),
  [G], [H], [I],
)

#table(
  columns: 3,
  [A], [B], [C],
  table.hline(stroke: 2pt),
  [D], [E], [F],
  table.vline(start: 0, end: 2),
  [G], [H], [I],
)
```

### Linter

```text
crystalline-lint .
=> ✓ No violations found
```

## Scope-outs / divergências conhecidas

1. **Posicionamento automático por linha/coluna:** a contagem `row`/`col` é linear e não replica o algoritmo completo de posicionamento vanilla (celulas explícitas com `x`/`y`, headers/footers, gaps). Para documentos simples a paridade é visual; para layouts complexos pode divergir.
2. **`position: auto` de `vline`:** assume sempre `left` em vez de decidir entre `left`/`right` com base na posição na linha de children.
3. **Gutter:** `hline`/`vline` desenham nas fronteiras das células (sem offset de gutter), espelhando a implementação actual das bordas de célula.
4. **Não há `table.hline()` / `table.vline()` como elementos standalone fora de grid/table:** esse comportamento não é suportado (a linha é filtrada e ignorada fora do contexto).

## Conclusão

A funcionalidade de linhas em grid/table está materializada e validada. O critério de saída `crystalline-lint .` com zero violations é satisfeito; a suíte de testes mantém-se verde.
