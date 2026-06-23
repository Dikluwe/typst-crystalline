# Prompt L0 — `rules/layout/link` — Layout de `Content::Link`
Hash do Código: fd6e07c2

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/link.rs`
**Origem**: P422 (S) — atomização do layout de hiperligações; **P424** — bbox.

---

## Decisão arquitetural (ADR-0109 forma B)

- `entities/elements/link.rs` — `LinkElem` struct puro (`url`, `body`).
- `rules/layout/link.rs` — free function `layout_link(layouter, &LinkElem)` que
  renderiza o body, calcula a bounding-box acumulada e envolve os itens
  resultantes em `FrameItem::Link`.
- `entities/layout_types.rs` — `FrameItem::Link { url, items, pos, size }`.

## Algoritmo

1. Renderizar `body` normalmente através do `Layouter`.
2. Coletar os `FrameItem`s produzidos (tanto em `current_line` quanto em
   `current_items`, caso tenha havido flush).
3. Calcular `pos`/`size` da bbox acumulada dos items:
   - `Text`: medir avanço via `FontMetrics::advance` e altura via
     `FontMetrics::vertical_metrics`.
   - `Glyph`: `x_advance` × `size`.
   - `Shape`/`Image`: `width` × `height`.
   - `Line`: bounding box entre `start` e `end`.
   - `Group`: aproximar por `pos` + `inner_width`/`inner_height`.
   - `Link` aninhado: reutilizar `pos`/`size` já calculados.
4. Emitir um único `FrameItem::Link { url, items, pos, size }` no frame atual.

## Scope-out

- Cor azul / sublinhado — aguarda `FrameItem::Decoration`.
- Links internos (`<label>`) e inter-página.
- `QuadPoints` para áreas clicáveis multi-linha.
