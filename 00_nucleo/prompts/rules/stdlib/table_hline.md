# Prompt L0 — `stdlib/table_hline` — função `table.hline()`
**P739A** — `stroke: none` aceite (paridade vanilla, medido: compila, linha não desenhada). A entidade guarda `Option<Stroke>`; o render salta a linha quando `None`. Zero-thickness proibido (hairline em PDF — P726).

**Camada**: L1 · **Alvo**: `01_core/src/rules/stdlib/structural.rs` (ou módulo grid/table)
**Origem**: Passo 512.

---

## Assinatura

```typst
table.hline(
  start: int = 0,
  end: int = auto,
  stroke: stroke = 1pt,
  position: auto | top | bottom = auto,
) -> content
```

## Semântica

Cria uma linha horizontal na tabela. Semântica idêntica a `grid.hline()`, mas devolve `Content::TableHLine` para ser reconhecido pelo construtor `table()`.

## Implementação

- `native_table_hline(args)` extrai `start`, `end`, `stroke`, `position`.
- Valida `position` em `"top"`, `"bottom"`, `"auto"`.
- `start >= 0`; `end` opcional.
- Devolve `Value::Content(Content::table_hline(...))`.
- Erros via `SourceDiagnostic::error(Span::detached(), ...)`.

## Casos de Aceitação

- `table.hline()` → linha horizontal no topo da primeira linha de células.
- `table.hline(stroke: 2pt, position: bottom)` → linha grossa na base.
- `table.hline(start: 0, end: 2)` → linha de 0 até 2 colunas.

## Nota de implementação

O `row` efectivo é calculado por `native_table()` durante a separação dos children, com base na ordem de aparecimento e no número de colunas.
