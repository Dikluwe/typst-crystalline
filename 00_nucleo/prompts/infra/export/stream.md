# Prompt L0 — `infra/export/stream` — PageContext + emit unificado
Hash do Código: 30ad603b

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/stream.rs`
**Criado em**: 2026-05-19 (P307c)
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
- Excede limite 800 LOC ADR-0037 Regra 2 (~685 LOC). Sub-divisão futura em `stream/{page,text,shape,draw}.rs` em P-stream-decomp dedicado se justificado.

## Interface

```rust
pub(crate) enum FontScenario<'a> { Type1, Cidfont { char_to_gid }, Multifont { fonts, per_font_char_to_gid } }
pub(crate) struct PageContext<'a> { /* ptr_to_idx, img_refs, pat_*, font_scenario */ }
impl<'a> PageContext<'a> {
    pub(crate) fn type1(...) -> Self;
    pub(crate) fn cidfont(..., char_to_gid) -> Self;
    pub(crate) fn multifont(..., fonts, per_font_char_to_gid) -> Self;
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
[ {-x_offset_tu} <GID> {advance_tu + x_offset_tu} ... ] TJ
```

Onde:
- `x_offset_tu = -(x_offset / upm * 1000)` — pré-glifo: desloca cursor à direita
- `advance_tu = -(x_advance / upm * 1000)` — idêntico ao P485
- Post-glifo = `advance_tu + x_offset_tu` — cancela o desvio pré-glifo

Para `x_offset = 0`: output idêntico ao P485 (zero regressão).

### `y_offset` — scope-out confirmado

`y_offset` requer sequências `Td` (saída do array TJ) e não há corpus LTR com
`y_offset != 0`. Scope-out declarado — não implementado em P486.

### Testes adicionados P486

- `p486_emit_x_offset_zero_equivale_p485`: x_offset=0 → sem número antes do GID
- `p486_emit_x_offset_nonzero_aplica_ajuste`: x_offset=-50, upm=1000 → "50 " antes do GID
- `p486_emit_x_offset_positivo`: x_offset=30, upm=1000 → "-30 " antes do GID
- `p486_parity_73_73_mantido`: sentinela parity (lab/parity)
