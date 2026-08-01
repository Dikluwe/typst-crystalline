//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/frac.md
//! @prompt-hash 6b15f650
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_frac` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, MathSize, Point, Pt, TextStyle},
};

use super::{offset_item, MathBox};

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
        // **P945** — `math_size` mantido honesto: num/den descem um nível
        // (`style_for_numerator`, `style.rs:343-350`: Display→Text,
        // Text→Script, Script|ScriptScript→ScriptScript). Só o campo — o
        // factor de tamanho actual NÃO muda neste passo (scope-out
        // registado em `frac.md` §P945).
        let num_math_size = match style.math_size {
            MathSize::Display => MathSize::Text,
            MathSize::Text => MathSize::Script,
            MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
        };
        let num_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            math_size: num_math_size,
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

        // **P920** — `gap` deixa de ser `fraction_num_gap` usado directamente
        // (mesmo valor duplicado nos dois lados) e passa a ser calculado pela
        // fórmula real do vanilla (`fraction.rs:30-53`), a partir da tinta
        // real do numerador/denominador e da posição do eixo matemático;
        // `fraction_num_gap`/`fraction_denom_gap` servem só de PISO MÍNIMO.
        // Ver `frac.md` §P920.
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let shift_up_pt = self
            .constants
            .to_pt(self.constants.fraction_numerator_shift_up, style.size)
            .val();
        let shift_down_pt = self
            .constants
            .to_pt(self.constants.fraction_denominator_shift_down, style.size)
            .val();
        let num_gap_floor = self
            .constants
            .to_pt(self.constants.fraction_num_gap, style.size)
            .val();
        let denom_gap_floor = self
            .constants
            .to_pt(self.constants.fraction_denom_gap, style.size)
            .val();

        let num_gap =
            (shift_up_pt - axis_pt - rule_thickness / 2.0 - num_box.descent).max(num_gap_floor);
        let denom_gap = (shift_down_pt + axis_pt - rule_thickness / 2.0 - den_box.ascent)
            .max(denom_gap_floor);

        // ascent cobre todo o numerador + espaço + metade da linha
        let ascent = num_box.height() + num_gap + rule_thickness / 2.0;
        // descent cobre metade da linha + espaço + todo o denominador
        let descent = den_box.height() + denom_gap + rule_thickness / 2.0;

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
        // descent do numerador pare `num_gap` acima do topo da linha.
        let num_y = -(num_box.descent + num_gap + rule_thickness / 2.0);
        // Denominador: a sua baseline própria desce o suficiente para que
        // o ascent do denominador pare `denom_gap` abaixo do fundo da linha.
        let den_y = denom_gap + rule_thickness / 2.0 + den_box.ascent;

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

        // **P919** — a barra fica fixa a `axis_height` acima da baseline,
        // por construção (vanilla `fraction.rs:66,69`), não "no meio do
        // ascent/descent" (`apply_axis_offset` genérico não se aplica aqui
        // — ver `frac.md` §P919). Desloca todo o box (numerador, barra,
        // denominador) em bloco por `-axis_pt`, mesmo padrão de
        // `layout_stretchy_delimiter::shift_y`. `axis_pt` já calculado acima
        // (reutilizado pela fórmula de `num_gap`/`denom_gap`, P920).
        let items: Vec<FrameItem> = items
            .into_iter()
            .map(|item| offset_item(item, Pt(0.0), Pt(-axis_pt)))
            .collect();

        MathBox {
            width,
            ascent: ascent + axis_pt,
            descent: descent - axis_pt,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
