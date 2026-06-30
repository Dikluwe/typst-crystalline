# Prompt L0 — `stdlib/grid_hline` — função `grid.hline()`

**Camada**: L1 · **Alvo**: `01_core/src/rules/stdlib/structural.rs` (ou módulo grid/table)
**Origem**: Passo 512.

---

## Assinatura

```typst
grid.hline(
  start: int = 0,
  end: int = auto,
  stroke: stroke = 1pt,
  position: auto | top | bottom = auto,
) -> content
```

## Semântica

Cria uma linha horizontal no grid. A linha é posicionada automaticamente consoante a ordem dos children: cada `grid.hline()` é renderizada acima (position: top) ou abaixo (position: bottom) da linha de células atual. Quando `position: auto`, assume `top` se for a primeira coisa numa linha de children, senão `bottom`.

## Implementação

- `native_grid_hline(args)` extrai `start`, `end`, `stroke`, `position`.
- Valida `position` em `"top"`, `"bottom"`, `"auto"`.
- `start >= 0`; `end` opcional.
- Devolve `Value::Content(Content::grid_hline(...))`.
- Erros via `SourceDiagnostic::error(Span::detached(), ...)`.

## Casos de Aceitação

- `grid.hline()` → linha horizontal no topo da primeira linha de células.
- `grid.hline(stroke: 2pt, position: bottom)` → linha grossa na base.
- `grid.hline(start: 0, end: 2)` → linha de 0 até 2 colunas.

## Nota de implementação

O `row` efectivo é calculado por `native_grid()` durante a separação dos children, com base na ordem de aparecimento e no número de colunas.
