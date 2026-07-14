# Prompt L0 — `stdlib/grid_vline` — função `grid.vline()`
**P739A** — `stroke: none` aceite (paridade vanilla, medido: compila, linha não desenhada). A entidade guarda `Option<Stroke>`; o render salta a linha quando `None`. Zero-thickness proibido (hairline em PDF — P726).

**Camada**: L1 · **Alvo**: `01_core/src/rules/stdlib/structural.rs` (ou módulo grid/table)
**Origem**: Passo 512.

---

## Assinatura

```typst
grid.vline(
  start: int = 0,
  end: int = auto,
  stroke: stroke = 1pt,
  position: auto | left | right = auto,
) -> content
```

## Semântica

Cria uma linha vertical no grid. A linha é posicionada automaticamente consoante a ordem dos children: cada `grid.vline()` é renderizada à esquerda (position: left) ou à direita (position: right) da coluna de células atual. Quando `position: auto`, assume `left`.

## Implementação

- `native_grid_vline(args)` extrai `start`, `end`, `stroke`, `position`.
- Valida `position` em `"left"`, `"right"`, `"auto"`.
- `start >= 0`; `end` opcional.
- Devolve `Value::Content(Content::grid_vline(...))`.
- Erros via `SourceDiagnostic::error(Span::detached(), ...)`.

## Casos de Aceitação

- `grid.vline()` → linha vertical à esquerda da primeira coluna.
- `grid.vline(stroke: 2pt, position: right)` → linha grossa à direita.
- `grid.vline(start: 0, end: 2)` → linha de 0 até 2 linhas.

## Nota de implementação

O `col` efectivo é calculado por `native_grid()` durante a separação dos children, com base na ordem de aparecimento e no número de colunas.
