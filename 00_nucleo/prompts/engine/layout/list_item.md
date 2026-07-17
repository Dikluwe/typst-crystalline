# Prompt L0 — `engine/layout/list_item` — layout de `ListItemElem`
Hash do Código: 885183a4

**Camada**: L1 · **Alvo**: `01_core/src/engine/layout/list_item.rs`
**Origem**: atomização (ADR-0109, P380); campos `indent`/`body_indent`/`tight`
adicionados em **P505**.
**Prompts relacionados**: `entities/elements/list_item.md`,
`entities/list_marker.md`, `rules/stdlib/structural.md`.

---

## Assinatura

```rust
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ListItemElem,
)
```

## Semântica

Renderiza um item de lista não ordenada num fluxo de bloco:

1. **Quebra de linha** se o cursor já estiver além de `line_start_x`
   (`cursor_x.0 > line_start_x.0`).
2. **Espaçamento entre itens soltos** (P505):
   - Se `e.tight == Some(false)` e o item anterior na sequência também foi um
     `ListItem`/`EnumItem` solto (`layouter.last_was_loose_item == true`),
     avança `cursor_y` por um `line_height` antes de posicionar o marker.
   - Actualiza `layouter.last_was_loose_item = (e.tight == Some(false))`.
3. **Resolve indentação**:
   - `indent` (Length) → deslocamento horizontal do marker em relação à margem.
     Default `0pt`.
   - `body_indent` (Length) → deslocamento horizontal do corpo em relação ao
     fim do marker. Default `0pt`.
   - `tight` (bool) → `true` (default) não adiciona espaço extra entre itens;
     `false` adiciona um espaçamento vertical equivalente a uma linha *entre*
     itens consecutivos soltos.
4. **Posiciona o marker** em:
   ```text
   marker_x = margin + indent
   ```
   O marcador é emitido como `FrameItem::Text` na posição corrente `cursor_y`.
5. **Mede a largura do marker** via `layouter.metrics.advance(marker_str, font_size_pt)`.
6. **Posiciona o corpo** alterando temporariamente `line_start_x` e `cursor_x`
   para:
   ```text
   body_x = margin + indent + marker_width + body_indent
   ```
   O corpo é renderizado com `layouter.layout_content(&e.body)`. A alteração de
   `line_start_x` garante que quebras de linha dentro do body mantenham a
   indentação.
7. **Restaura** `line_start_x` para o valor original após `flush_line`.
8. **Preservação de `marker_align`** (P504): campo é lido e preservado; o efeito
   visual de alinhamento do marker é scope-out.

## Validação

- `list(indent: 1.5em, body-indent: 0.5em, tight: false, [A], [B])` produz
  itens cujo marker está deslocado de `1.5em` e cujo corpo está deslocado de
  `1.5em + marker_width + 0.5em` em relação à margem.
- `list(tight: true, [A], [B])` mantém `cursor_y` inalterado entre itens.
- `list(tight: false, [A], [B])` adiciona `line_height` de espaço entre itens.
- Itens com texto longo mantêm a indentação do corpo nas linhas quebradas.

## Scope-outs explícitos

- Alinhamento visual real de `marker_align` (P504) — campo aceite e propagado,
  sem efeito de render.
- Suporte a marcadores por nível (`marker: Array`) — parse aceite em eval,
  mas layout usa apenas o primeiro marcador.
