# Prompt L0 — `stdlib/table_vline` — função `table.vline()`
**P739A** — `stroke: none` aceite (paridade vanilla, medido: compila, linha não desenhada). A entidade guarda `Option<Stroke>`; o render salta a linha quando `None`. Zero-thickness proibido (hairline em PDF — P726).
Hash do Código: dc48f596

**Camada**: L1 · **Alvo**: `01_core/src/rules/stdlib/structural.rs` (ou módulo grid/table)
**Origem**: Passo 512.

---

## Assinatura

```typst
table.vline(
  start: int = 0,
  end: int = auto,
  stroke: stroke = 1pt,
  position: auto | left | right = auto,
) -> content
```

## Semântica

Cria uma linha vertical na tabela. Semântica idêntica a `grid.vline()`, mas devolve `Content::TableVLine` para ser reconhecido pelo construtor `table()`.

## Implementação

- `native_table_vline(args)` extrai `start`, `end`, `stroke`, `position`.
- Valida `position` em `"left"`, `"right"`, `"auto"`.
- `start >= 0`; `end` opcional.
- Devolve `Value::Content(Content::table_vline(...))`.
- Erros via `SourceDiagnostic::error(Span::detached(), ...)`.

## Casos de Aceitação

- `table.vline()` → linha vertical à esquerda da primeira coluna.
- `table.vline(stroke: 2pt, position: right)` → linha grossa à direita.
- `table.vline(start: 0, end: 2)` → linha de 0 até 2 linhas.

## Nota de implementação

O `col` efectivo é calculado por `native_table()` durante a separação dos children, com base na ordem de aparecimento e no número de colunas.
