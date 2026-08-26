# Prompt L0 — `compiler/layout/list_item` — layout de `ListItemElem`
Hash do Código: 9662cf3a

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/list_item.rs`
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
2. **Espaçamento entre grupos separados por Parbreak** (P864):
   - Se o item anterior na sequência foi um `ListItem` e ocorreu um
     `Content::Parbreak` desde então
     (`layouter.parbreak_since_last_item == true` e
     `layouter.last_seen_item_group == Some(ItemGroup::List)`), avança
     `cursor_y` por um `paragraph_advance` (P762:
     `top_edge + |bottom_edge| + leading`) antes de posicionar o marker.
   - Reseta `layouter.last_was_loose_item = false` para não acumular com o
     espaçamento de itens soltos do mesmo grupo.
3. **Espaçamento entre itens soltos** (P505):
   - Se `e.tight == Some(false)` e o item anterior na sequência também foi um
     `ListItem`/`EnumItem` solto (`layouter.last_was_loose_item == true`),
     avança `cursor_y` por um `line_height` antes de posicionar o marker.
   - Actualiza `layouter.last_was_loose_item = (e.tight == Some(false))`.
4. **Resolve indentação**:
   - `indent` (Length) → deslocamento horizontal do marker em relação à margem.
     Default `0pt`.
   - `body_indent` (Length) → deslocamento horizontal do corpo em relação ao
     fim do marker. Default `0.5em` (P1072 / paridade vanilla `list.rs:100-102`).
   - `tight` (bool) → `true` (default) não adiciona espaço extra entre itens;
     `false` adiciona um espaçamento vertical equivalente a uma linha *entre*
     itens consecutivos soltos.

> **Fonte de paridade (P1031)** — doc comments dos campos de `ListElem` no vanilla
> ratificado (`e0e8ca4d`), `crates/typst-library/src/model/list.rs`, publicados em
> `typst.app/docs/reference/model/list/`:
>
> | Campo | `file:line` | Doc comment / default |
> |---|---|---|
> | `indent` | `list.rs:97-98` | *"The indent of each item."* — sem `#[default]`, logo `0pt` ✅ confere |
> | `body_indent` | `list.rs:100-102` | *"The spacing between the marker and the body of each item."* — `#[default(Em::new(0.5).into())]` ❌ **`0.5em`, não `0pt`** |
> | `tight` | `list.rs:65-66` | `#[default(true)]` ✅ confere no valor |
> | `marker` | `list.rs:88-95` | default `('•', '‣', '–')` — `\u{2022}`, `\u{2023}`, `\u{2013}` |
>
> **ACHADO CORRIGIDO (P1072) — default de `body_indent`.** A linguagem diz `0.5em`; o cristalino usa
> `0pt`. Medição directa (2026-08-13; vanilla `/usr/local/bin/typst` = `typst 0.15.1
> (e0e8ca4d)`; cristalino `target/release/typst` da fonte em HEAD `4f64e4e69`, árvore só com
> edições em `00_nucleo/prompts/**`), documento `- Um` / `- Dois`:
>
> | Binário | `pdftotext -layout` |
> |---|---|
> | Vanilla | `• Um` / `• Dois` |
> | Cristalino | `•Um` / `•Dois` |
> | Cristalino com `#list(body-indent: 0.5em, …)` | `• Um` / `• Dois` (coincide) |
>
> A divergência está **só no default** — com o valor explícito o resultado bate. Mudança de
> comportamento por defeito → gate ADR-0127 e passo próprio. **Não implementado aqui.**
>
> **Sobre `tight` — a descrição acima não é a da linguagem.** O vanilla define `tight` em
> termos de espaçamento de parágrafo, não de "uma linha" (`list.rs:52-64`, doc comment):
> os itens usam *paragraph spacing* quando `tight` é `{false}` e *paragraph leading* quando
> é `{true}`. E acrescenta uma regra que este L0 não regista: *"In markup mode, the value of
> this parameter is determined based on whether items are separated with a blank line. If
> items directly follow each other, this is set to `{true}`; if items are separated by a
> blank line, this is set to `{false}`. **The markup-defined tightness cannot be overridden
> with set rules.**"* A fórmula "equivalente a uma linha" do cristalino é aproximação de
> implementação, não citação — assinalado como tal, sem medição de desvio neste passo.
5. **Posiciona o marker** em:
   ```text
   marker_x = margin + indent
   ```
   O marcador é emitido como `FrameItem::Text` na posição corrente `cursor_y`.
6. **Mede a largura do marker** via `layouter.metrics.advance(marker_str, font_size_pt)`.
7. **Posiciona o corpo** alterando temporariamente `line_start_x` e `cursor_x`
   para:
   ```text
   body_x = margin + indent + marker_width + body_indent
   ```
   O corpo é renderizado com `layouter.layout_content(&e.body)`. A alteração de
   `line_start_x` garante que quebras de linha dentro do body mantenham a
   indentação.
8. **Restaura** `line_start_x` para o valor original após `flush_line`.
9. **Preservação de `marker_align`** (P504): campo é lido e preservado; o efeito
   visual de alinhamento do marker é scope-out.

## Validação

- `list(indent: 1.5em, body-indent: 0.5em, tight: false, [A], [B])` produz
  itens cujo marker está deslocado de `1.5em` e cujo corpo está deslocado de
  `1.5em + marker_width + 0.5em` em relação à margem.
- `list(tight: true, [A], [B])` mantém `cursor_y` inalterado entre itens.
- `list(tight: false, [A], [B])` adiciona `line_height` de espaço entre itens.
- Itens com texto longo mantêm a indentação do corpo nas linhas quebradas.
- Dois `ListItem` separados por `Content::Parbreak` (P864) têm gap vertical de
  `2 * line_advance` e `last_was_loose_item` é resetado; sem `Parbreak`, o gap
  é `1 * line_advance`.

## Scope-outs explícitos

- Alinhamento visual real de `marker_align` (P504) — campo aceite e propagado,
  sem efeito de render.
- Suporte a marcadores por nível (`marker: Array`) — parse aceite em eval,
  mas layout usa apenas o primeiro marcador.
