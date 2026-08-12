//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/stretchy.md
//! @prompt-hash 65510a2f
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_stretchy_delimiter` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::layout_types::{FrameItem, Point, TextStyle};

use super::MathBox;

/// **P918** — Subtrai `DELIM_SHORT_FALL = 0.1em` (P912) da dimensão alvo
/// antes de consultar `select_variant`. Partilhada por
/// `layout_stretchy_delimiter` e `layout_stretchy_glyph_horizontal` — só
/// estas 2 linhas são idênticas byte-a-byte entre os dois métodos; o resto
/// diverge de propósito (eixo vertical centra em `axis_height`, horizontal
/// assenta na baseline) e não foi unificado (ver `stretchy.md` §P918).
fn apply_delim_short_fall(target_du: f64, upem: f64) -> f64 {
    let short_fall_du = 0.1 * upem;
    (target_du - short_fall_du).max(0.0)
}

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_stretchy_delimiter(
        &self,
        c: char,
        min_height_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        self.layout_stretchy_delimiter_impl(c, min_height_du, style, true)
    }

    /// **P974** — o símbolo do radical (`√`) estica com **short_fall = 0**
    /// no vanilla (`resolve.rs:1246`: `StretchInfo::new(Rel::one(),
    /// Em::zero())`) — ao contrário dos delimitadores de `lr`/matrizes
    /// (DELIM_SHORT_FALL = 0.1em, P912). Ver `root.md` §P974.
    pub(super) fn layout_radical_symbol(
        &self,
        min_height_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        self.layout_stretchy_delimiter_impl('√', min_height_du, style, false)
    }

    fn layout_stretchy_delimiter_impl(
        &self,
        c: char,
        min_height_du: f64,
        style: &TextStyle,
        apply_short_fall: bool,
    ) -> MathBox {
        let variants = self.metrics.vertical_glyph_variants(c, style);

        // P912: subtrair DELIM_SHORT_FALL = 0.1em (0.1 * upem em design units) da dimensão alvo
        // (só para delimitadores — o radical tem short_fall=0 no vanilla, P974)
        let target_du = if apply_short_fall {
            apply_delim_short_fall(min_height_du, self.constants.upem)
        } else {
            min_height_du
        };

        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();

        if let Some(picked) = variants.select_variant(target_du) {
            let glyph_id = picked.glyph_id;
            // **P917** — `picked.advance` é a medida do eixo de esticamento
            // (altura, para construções verticais) — correcto aqui, só para
            // ascent/descent/shift_y. `picked.hor_advance` (avanço nativo do
            // glifo) é usado abaixo para x_advance/width — nunca `advance`.
            let height_pt = style.size.val() * (picked.advance / self.constants.upem);
            let half_h = height_pt / 2.0;
            let ascent = axis_pt + half_h;
            let descent = (half_h - axis_pt).max(0.0);
            let shift_y = axis_pt - half_h;

            // Variante encontrada
            if let Some(mapped_char) = self.metrics.glyph_to_char(glyph_id) {
                // Mapeamento Unicode disponível — emitir como Text
                let text: ecow::EcoString = mapped_char.to_string().into();
                let mut b = self.layout_text_node(&text, style);
                b.ascent = ascent;
                b.descent = descent;
                b.items = b
                    .items
                    .into_iter()
                    .map(|item| super::offset_item(item, crate::entities::layout_types::Pt(0.0), crate::entities::layout_types::Pt(shift_y)))
                    .collect();
                return b;
            } else {
                // Sem mapeamento — emitir como Glyph. **P952b** — a tinta da
                // variante é assimétrica (NewCMMath: mais acima que abaixo);
                // centra-se a tinta real no eixo (vanilla `update_glyph` +
                // `center_on_axis`), não a metade simétrica do advance.
                let (ink_up, ink_down) =
                    self.metrics.glyph_ink_bounds(glyph_id, style.size, style);
                let ink_center = (ink_up.val() - ink_down.val()) / 2.0;
                let shift_y_ink = ink_center - axis_pt;
                let half_ink = (ink_up.val() + ink_down.val()) / 2.0;
                let x_advance = style.size * (picked.hor_advance / self.constants.upem);
                return MathBox {
                    width: x_advance.val(),
                    ascent: half_ink + axis_pt,
                    descent: (half_ink - axis_pt).max(0.0),
                    items: vec![FrameItem::Glyph {
                        pos: Point {
                            x: crate::entities::layout_types::Pt(0.0),
                            y: crate::entities::layout_types::Pt(shift_y_ink),
                        },
                        glyph_id,
                        x_advance,
                        size: style.size,
                        style: style.clone(),
                        base_char: c,
                    }],
                };
            }
        }

        // Nenhuma variante suficiente — tentar GlyphAssembly
        let assembly = self.metrics.vertical_glyph_assembly(c, style);
        if !assembly.is_empty() {
            return self.layout_assembly(c, assembly, min_height_du, style);
        }

        // Fallback: glifo base
        let text: ecow::EcoString = c.to_string().into();
        self.layout_text_node(&text, style)
    }

    /// **P906** — esticamento no eixo X (`underbrace`/`overbrace`/
    /// `underbracket`/`overbracket`, acentos largos). Espelha
    /// `layout_stretchy_delimiter` acima, trocando `vertical_glyph_variants`/
    /// `vertical_glyph_assembly` por `horizontal_glyph_variants`/
    /// `horizontal_glyph_assembly` e `min_height_du` por `min_width_du` — ver
    /// `math/layout/stretchy.md` §P906.
    ///
    /// **P984** — `short_fall_em` depende do chamador (vanilla
    /// `ir/resolve.rs:389`/`:1430`): acentos 0.5em (`ACCENT_SHORT_FALL`),
    /// spreaders 0.0. Ver `stretchy.md` §P984.
    pub(super) fn layout_stretchy_glyph_horizontal(
        &self,
        c: char,
        min_width_du: f64,
        style: &TextStyle,
        short_fall_em: f64,
    ) -> MathBox {
        let variants = self.metrics.horizontal_glyph_variants(c, style);

        // **P984** — short_fall por chamador (antes: DELIM_SHORT_FALL = 0.1em
        // fixo para todo o eixo X — errado para acentos e spreaders).
        let target_du =
            (min_width_du - short_fall_em * self.constants.upem).max(0.0);

        // **P984 (keep-base)** — vanilla `fragment/glyph.rs:267-271`: se
        // `short_target ≤ advance hmtx do glifo base`, mantém o base (não
        // estica). A comparação é com o advance shaped/hmtx (500du para o
        // hat de NewCMMath), NÃO com o `AdvanceMeasurement` da tabela MATH
        // (307du) — confundir os dois sobre-esticava bases estreitas.
        let size_pt = style.size.val().max(0.001);
        let base_advance_du =
            self.metrics.advance(&c.to_string(), style.size, style).val()
                * self.constants.upem
                / size_pt;
        if target_du <= base_advance_du {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        if let Some(picked) = variants.select_variant(target_du) {
            return self.emit_horizontal_variant(c, picked, style);
        }

        // Nenhuma variante suficiente — tentar GlyphAssembly
        let assembly = self.metrics.horizontal_glyph_assembly(c, style);
        if !assembly.is_empty() {
            return self.layout_assembly_horizontal(c, assembly, min_width_du, style);
        }

        // **P984 (keep-largest)** — vanilla `fragment/glyph.rs:278-298`: sem
        // variante suficiente e SEM assembly, fica com a MAIOR variante (o
        // loop do vanilla fica sempre com a última), não com o glifo base.
        if let Some(largest) = variants.variants.last() {
            return self.emit_horizontal_variant(c, largest, style);
        }

        // Fallback: glifo base
        let text: ecow::EcoString = c.to_string().into();
        self.layout_text_node(&text, style)
    }

    /// **P984** — emissão de uma variante horizontal seleccionada (corpo
    /// partilhado pelos ramos "primeira suficiente" e "maior variante" de
    /// `layout_stretchy_glyph_horizontal` — idênticos byte-a-byte).
    fn emit_horizontal_variant(
        &self,
        c: char,
        picked: &crate::entities::glyph_variants::GlyphVariant,
        style: &TextStyle,
    ) -> MathBox {
        let glyph_id = picked.glyph_id;
        // Variante encontrada
        if let Some(mapped_char) = self.metrics.glyph_to_char(glyph_id) {
            // Mapeamento Unicode disponível — emitir como Text
            let text: ecow::EcoString = mapped_char.to_string().into();
            self.layout_text_node(&text, style)
        } else {
            // Sem mapeamento — emitir como Glyph. **P917** — hor_advance
            // (avanço nativo), nunca `advance` (medida do eixo de
            // esticamento) — ver `stretchy.md` §P917.
            let x_advance = style.size * (picked.hor_advance / self.constants.upem);
            // **P985** — ascent/descent da TINTA real da variante
            // (`glyph_ink_bounds`, P952b), não do `vertical_metrics` da
            // fonte: a tinta de ⏟ está toda abaixo da baseline e a de ⏞
            // toda acima (NewCMMath) — com métricas da fonte a caixa
            // mentia nos dois sentidos (ver `underover.md` §P985).
            let (ink_up, ink_down) =
                self.metrics.glyph_ink_bounds(glyph_id, style.size, style);
            MathBox {
                width: x_advance.val(),
                ascent: ink_up.val(),
                descent: ink_down.val(),
                items: vec![FrameItem::Glyph {
                    pos: Point::ZERO,
                    glyph_id,
                    x_advance,
                    size: style.size,
                    style: style.clone(),
                    base_char: c,
                }],
            }
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
