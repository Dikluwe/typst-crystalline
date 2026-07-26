//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/frac.md
//! @prompt-hash 831519a4
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_frac` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_frac(
        &self,
        num: &Content,
        den: &Content,
        style: &TextStyle,
    ) -> MathBox {
        // **P915** — denominador é cramped, numerador não — achado do
        // vanilla (`style_for_denominator` = `[style_for_numerator,
        // style_cramped()]`, `style.rs:362`, vs. `style_for_numerator` sem
        // `style_cramped()`). `num_style` herda `cramped` do estilo
        // recebido (sem forçar); `den_style` força `true`. Ver `frac.md`
        // §P915.
        let num_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            ..style.clone()
        };
        let den_style = TextStyle { cramped: true, ..num_style.clone() };

        let num_box = self.layout_node(num, &num_style);
        let den_box = self.layout_node(den, &den_style);

        let width = num_box.width.max(den_box.width);

        let rule_thickness = self
            .constants
            .to_pt(self.constants.fraction_rule_thickness, style.size)
            .val();
        let gap = self
            .constants
            .to_pt(self.constants.fraction_num_gap, style.size)
            .val();

        // ascent cobre todo o numerador + espaço + metade da linha
        let ascent = num_box.height() + gap + rule_thickness / 2.0;
        // descent cobre metade da linha + espaço + todo o denominador
        let descent = den_box.height() + gap + rule_thickness / 2.0;

        // Centrar horizontalmente
        let num_x = (width - num_box.width) / 2.0;
        let den_x = (width - den_box.width) / 2.0;

        // **P905** — convenção baseline-relativa (confirmada em P901 via
        // `attach.rs`/`root.rs`/`layout_equation`): `y=0` é a BASELINE
        // PRÓPRIA desta `MathBox`, y cresce para baixo. A versão anterior
        // assumia (incorrectamente, como `root.rs` antes de P901) que
        // `local_y=0` era o TOPO do box: `num_y=0.0` deixava a baseline do
        // numerador coincidir com a baseline da fracção (não sobe), e
        // `den_y`/`rule_local_y` eram calculados a partir de
        // `num_box.height()` (topo-relativo) em vez de relativos à
        // baseline própria de cada caixa — a linha acabava a meio do
        // denominador em vez de entre os dois (achado visual: a barra
        // "corta" o denominador, ver typst-passo-905-relatorio.md).
        //
        // A linha de fracção fica exactamente na baseline desta MathBox
        // (`rule_local_y = 0`) — por construção, `ascent`/`descent` acima
        // medem a partir daí.
        let rule_local_y = 0.0_f64;
        // Numerador: a sua baseline própria sobe o suficiente para que o
        // descent do numerador pare `gap` acima do topo da linha.
        let num_y = -(num_box.descent + gap + rule_thickness / 2.0);
        // Denominador: a sua baseline própria desce o suficiente para que
        // o ascent do denominador pare `gap` abaixo do fundo da linha.
        let den_y = gap + rule_thickness / 2.0 + den_box.ascent;

        let mut items = Vec::new();

        for mut item in num_box.items {
            match item {
                FrameItem::Text { ref mut pos, .. }
                | FrameItem::TextShaped { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + num_x);
                    pos.y = Pt(pos.y.val() + num_y);
                }
                _ => {}
            }
            items.push(item);
        }

        // Linha de fracção posicionada entre numerador e denominador.
        items.push(FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(rule_local_y) },
            end: Point { x: Pt(width), y: Pt(rule_local_y) },
            thickness: rule_thickness,
            // P285: math frac sem stroke explícito — preserva preto bit-exact.
            color: None,
        });

        for mut item in den_box.items {
            match item {
                FrameItem::Text { ref mut pos, .. }
                | FrameItem::TextShaped { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + den_x);
                    pos.y = Pt(pos.y.val() + den_y);
                }
                _ => {}
            }
            items.push(item);
        }

        self.apply_axis_offset(MathBox { width, ascent, descent, items }, style.size)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
