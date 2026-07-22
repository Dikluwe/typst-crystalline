# Prompt L0 — `infra/export/stream` — PageContext + emit unificado
Hash do Código: 2a3a98ff

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream.rs`
**Criado em**: 2026-05-19 (P307c)
**Atualizado em**: 2026-07-03 (P548 — correção do sinal do delta TJ)
**ADRs**: ADR-0098 (SSoT helpers unificados pós-P281)

---

## Contexto

Cluster central de emit PDF — implementação do "single source of
truth" pós-P281: helpers unificados invocados tanto pelo caminho
top-level (`build_page_stream`) como pelo caminho local em Group
(`draw_item_local`).

- `FontScenario` enum dispatch para `emit_text_pdf` / `emit_glyph_pdf`.
- `PageContext` agregador de `ptr_to_idx`, `img_refs`, `pat_ptr_to_idx`, `pat_refs`, `font_scenario`.
- `build_page_stream` orquestra emit page-level.
- `draw_item_local` emit recursivo em Groups (clip masks, transforms).
- Shape primitives: `emit_shape_path_local` + `emit_rounded_rect_ops` + `emit_stroke_paint` + `line_rg_prefix`.

## Restrições estruturais

- L3. Usa formatadores `format!`/`String` — não toca FS.
- `FontScenario` e `PageContext` são `pub(crate)` — instanciados em `super::builder` mas atravessam tipo nas chamadas.
- Helpers de emit são `pub(super)` — chamados por `super::builder` e self.
- `FrameItem::Image` inclui `orientation` (P776); o emit compõe a matriz `cm`
  com a transformação EXIF correspondente, preservando os bytes originais do
  JPEG/PNG (paridade `typst-pdf/src/image.rs::exif_transform`). A matriz é emitida
  com precisão de 5 casas decimais (P777), replicando o vanilla e evitando desvios
  de sub-pixel nas bordas em orientações com flip/rotate.
- Excede limite 800 LOC ADR-0037 Regra 2 (~685 LOC). Sub-divisão futura em `stream/{page,text,shape,draw}.rs` em P-stream-decomp dedicado se justificado.

## Interface

```rust
pub(crate) enum FontScenario<'a> {
    Type1,
    Cidfont { char_to_gid, glyph_mapping, glyph_to_nominal },
    Multifont { fonts, per_font_char_to_gid, per_font_glyph_mapping, per_font_glyph_to_nominal },
}
pub(crate) struct PageContext<'a> { /* ptr_to_idx, img_refs, pat_*, font_scenario */ }
impl<'a> PageContext<'a> {
    pub(crate) fn type1(...) -> Self;
    pub(crate) fn cidfont(..., char_to_gid, glyph_mapping, glyph_to_nominal) -> Self;
    pub(crate) fn multifont(..., fonts, per_font_char_to_gid, per_font_glyph_mapping, per_font_glyph_to_nominal) -> Self;
}

pub(super) fn emit_text_pdf(ops, pos_x, base_y, text, style, scenario);
pub(super) fn emit_glyph_pdf(ops, pos_x, base_y, glyph_id, size, scenario);
pub(super) fn emit_stroke_paint(...);
pub(super) fn line_rg_prefix(color: &Option<Color>) -> String;
pub(super) fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8>;
pub(super) fn emit_shape_path_local(ops, kind, w, h);
pub(super) fn emit_rounded_rect_ops(...);
pub(super) fn draw_item_local(ops, item, ctx, ...);
```

## Invariantes (ADR-0098)

- `emit_text_pdf` / `emit_glyph_pdf` / `line_rg_prefix` chamados em ambos os caminhos top-level e local — alteração simétrica garantida por construção.
- Para `color: None` em Line, `line_rg_prefix` retorna `""` (bit-exact pré-P285).
- Hash de `stream.rs` é métrica de aderência ao padrão (P282-P306, 23 passos cumulativos).

## Critérios de verificação

- Tests P281+ em `tests.rs` validam paridade entre caminhos.
- Test `p282_line_stroke_color_simetric` valida que `RG` injecção é simétrica.
- Snapshot binário em `p307b_snapshot_tests.rs` valida invariante observable.

## §P486 — `x_offset` no TJ array (Sub-item B)

**P486** adiciona suporte a `ShapedGlyph.x_offset` no operador PDF `TJ` em
`emit_shaped_pdf`. Aplicado simetricamente em `Cidfont` e `Multifont`.

### Fórmula por glifo com x_offset != 0

```
[ {-x_offset_tu} <GID> {advance_tu} ... ] TJ
```

Onde:
- `x_offset_tu = -(x_offset / upm * 1000)` — pré-glifo: desloca cursor
- `advance_tu` — ver §P520 (delta model)

O `x_offset` é uma translação visual do glifo actual; não altera o avanço
para o glifo seguinte.

### `y_offset` — scope-out confirmado

`y_offset` requer sequências `Td` (saída do array TJ) e não há corpus LTR com
`y_offset != 0`. Scope-out declarado — não implementado em P486.

### Testes adicionados P486

- `p486_emit_x_offset_zero_equivale_p485`: x_offset=0 → sem número antes do GID
- `p486_emit_x_offset_nonzero_aplica_ajuste`: x_offset=-50, upm=1000 → "50 " antes do GID
- `p486_emit_x_offset_positivo`: x_offset=30, upm=1000 → "-30 " antes do GID

## §P520 — Kerning via delta model no TJ

**P520** corrige o avanço do operador `TJ` para reflicta kerning aplicado
pelo shaper (rustybuzz). O CIDFont declara `/W` com larguras nominais
(`hmtx`), pelo que o `TJ` deve conter apenas o *delta* entre a largura
declarada e o avanço real.

### Fórmula por glifo

```
nominal    = glyph_to_nominal.get(glyph_id).unwrap_or(x_advance)
advance_tu = (nominal - x_advance) as f64 / upm * 1000.0
[ {-x_offset_tu} <GID> {advance_tu} ... ] TJ
```

No operador PDF `TJ`, cada número é **subtraído** da coordenada horizontal
antes de desenhar o próximo glifo. Portanto:

- `advance_tu` **positivo** quando `x_advance < nominal` (kerning negativo —
  aproxima o próximo glifo).
- `advance_tu` **negativo** quando `x_advance > nominal` (kerning positivo —
  afasta o próximo glifo).
- `glyph_to_nominal` é construído em `builder.rs` a partir do `hmtx` da fonte
  original, para todos os `glyph_id` usados no documento (`collect_glyph_ids`
  + codepoints mapeados).

### Testes adicionados P520

- `p520_emit_shaped_kerning_delta`: x_advance=599, nominal=639, upm=1000 → delta = +40

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-05-19 | Criação — P307c: PageContext + emit unificado | `stream.rs` |
| 2026-07-03 | P548 — correção do sinal do delta TJ: `advance_tu = (nominal - x_advance)` em vez de `(x_advance - nominal)` | `stream.rs`, `stream.md`, `builder.md`, `tests.rs` |

---

## §P788 — `draw_item_top`: flip Y para filhos de `FrameItem::Link` ao nível da página

**Decisão:** a emissão top-level por item foi extraída de `build_page_stream`
para `draw_item_top(ops, item, page_height, ctx) -> ops` (recebe/devolve
`ops` por valor — braço `Link` chama recursivamente sem conflito de borrow).
O braço `FrameItem::Link` passa a desenhar os filhos **pelo caminho
top-level com flip Y** (`pdf_y = page_height - pos.y`).

**Causa raiz (medida, P786 A9 + `#link` genérico):** os filhos de Link ao
nível da página eram desenhados por `draw_item_local` — que NÃO aplica flip
(assume a matriz `cm` invertida de um `Group` envolvente). Sem Group, os
filhos apareciam com `pos.y` crua → fundo da página (medido: y≈754 em vez
de y≈68; vanilla: coordenadas idênticas após a correção). `draw_item_local`
mantém o seu papel dentro de `Group` (matriz já invertida) — inalterado.

**Critério de aceitação:** teste `p788_link_top_level_filho_tem_flip_y` —
filho `Text` a (70,100) em página 800 → stream contém `70.0 700.0 Td`
(nunca `70.0 100.0 Td`).

## P836 — selecção de fonte por variações

`FontScenario::Multifont` e `font_index_for_style` passam a chavear por
`(FontList, FontVariant, FontVariations)`: o índice `/F{n}` de cada
run de texto é resolvido comparando também `style.variations`
(`unwrap_or_default`), coerente com a chave da pipeline/builder.
