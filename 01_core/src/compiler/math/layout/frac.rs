//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/frac.md
//! @prompt-hash 8be48a2a
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_frac` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::compiler::layout::FontMetrics;
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
        // **P952** — `num_style`/`den_style` passam a usar a descida por
        // NÍVEL de `MathSize` (`style_for_numerator`,
        // `style_for_denominator` = numerator + `style_cramped()`,
        // vanilla `style.rs:343-363`), via os helpers partilhados
        // `numerator_style` (cramped herdado do ambiente — sem forçar) e
        // `denominator_style` (cramped: true sempre). Tabela: Display→Text
        // ×1.0, Text→Script ×`script_percent_scale_down`,
        // Script→ScriptScript ×`sscript/script`,
        // ScriptScript→ScriptScript ×1.0. Substitui o
        // ×`script_percent_scale_down` incondicional de P915, medido
        // correcto só em inline (Text→Script). Ver `frac.md` §P952 e
        // `_comum.md` §"numerator_style (novo helper, P952)" + §P945.
        let num_style = self.numerator_style(style);
        let den_style = self.denominator_style(style);

        let num_box = self.layout_node(num, &num_style);
        let den_box = self.layout_node(den, &den_style);

        // **P990-B** — `FRAC_PADDING = 0.1em` (vanilla `math/frac.rs:9`,
        // `ir/resolve.rs:748`): a largura da fracção não é
        // `max(num, den)` nua — ganha `2 × padding` (0.1em de cada lado,
        // resolvido ao `style.size` do CONTEXTO da fracção, não do
        // numerador/denominador). A barra desenha-se só com
        // `line_width = max(num, den)`, centrada em `width` — não de
        // margem a margem da caixa. Ver `frac.md` §P990-B.
        let line_width = num_box.width.max(den_box.width);
        let padding = 0.1 * style.size.val();
        // rationale: padding simétrico dos dois lados da barra de fração — paridade literal com o vanilla (fraction.rs:56, 104). P1066.
        let width = line_width + 2.0 * padding;

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
        //
        // **P990-A** — em estilo Display (`style.math_size ==
        // MathSize::Display`), os shifts/pisos são as 4 constantes Display
        // dedicadas (`entities/math_constants.md` §P990), não as de texto
        // (vanilla `fraction.rs:33-52` selecciona por `MathSize`). Inline
        // (Text) mantém as constantes de P920, inalteradas. Ver `frac.md`
        // §P990-A.
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let is_display = style.math_size == MathSize::Display;
        let shift_up_pt = self
            .constants
            .to_pt(
                if is_display {
                    self.constants.fraction_numerator_display_style_shift_up
                } else {
                    self.constants.fraction_numerator_shift_up
                },
                style.size,
            )
            .val();
        let shift_down_pt = self
            .constants
            .to_pt(
                if is_display {
                    self.constants.fraction_denominator_display_style_shift_down
                } else {
                    self.constants.fraction_denominator_shift_down
                },
                style.size,
            )
            .val();
        let num_gap_floor = self
            .constants
            .to_pt(
                if is_display {
                    self.constants.fraction_num_display_style_gap_min
                } else {
                    self.constants.fraction_num_gap
                },
                style.size,
            )
            .val();
        let denom_gap_floor = self
            .constants
            .to_pt(
                if is_display {
                    self.constants.fraction_denom_display_style_gap_min
                } else {
                    self.constants.fraction_denom_gap
                },
                style.size,
            )
            .val();

        let num_gap =
            // rationale: P1064 Classe 1B — semi-espessura de traço de fração (rule_thickness / 2.0)
            (shift_up_pt - axis_pt - rule_thickness / 2.0 - num_box.descent).max(num_gap_floor);
        // rationale: P1064 Classe 1B — semi-espessura de traço de fração (rule_thickness / 2.0)
        let denom_gap = (shift_down_pt + axis_pt - rule_thickness / 2.0 - den_box.ascent)
            .max(denom_gap_floor);

        // ascent cobre todo o numerador + espaço + metade da linha
        // rationale: P1064 Classe 1B — semi-espessura de traço de fração (rule_thickness / 2.0)
        let ascent = num_box.height() + num_gap + rule_thickness / 2.0;
        // descent cobre metade da linha + espaço + todo o denominador
        // rationale: P1064 Classe 1B — semi-espessura de traço de fração (rule_thickness / 2.0)
        let descent = den_box.height() + denom_gap + rule_thickness / 2.0;

        // Centrar horizontalmente
        // rationale: P1064 Classe 1A — centragem do numerador ((width - num_box.width) / 2.0)
        let num_x = (width - num_box.width) / 2.0;
        // rationale: P1064 Classe 1A — centragem do denominador ((width - den_box.width) / 2.0)
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
        // rationale: P1064 Classe 1B — semi-espessura de traço de fração (rule_thickness / 2.0)
        let num_y = -(num_box.descent + num_gap + rule_thickness / 2.0);
        // Denominador: a sua baseline própria desce o suficiente para que
        // o ascent do denominador pare `denom_gap` abaixo do fundo da linha.
        // rationale: P1064 Classe 1B — semi-espessura de traço de fração (rule_thickness / 2.0)
        let den_y = denom_gap + rule_thickness / 2.0 + den_box.ascent;

        let mut items = Vec::new();

        // **P972** — `offset_item` (todos os tipos) em vez de um `match`
        // só sobre Text/TextShaped: o braço `_ => {}` deixava
        // `FrameItem::Glyph` (delimitadores stretchy sem mapeamento Unicode,
        // P906/P952b — só com a fonte real) e `FrameItem::Line` (overline de
        // sqrt aninhado) sem o offset, 3.52pt fora da baseline no numerador
        // (achado 9.3). Ver `frac.md` §P972.
        for item in num_box.items {
            items.push(offset_item(item, Pt(num_x), Pt(num_y)));
        }

        // Linha de fracção posicionada entre numerador e denominador.
        // **P990-B** — a barra desenha-se só com `line_width` (não
        // margem-a-margem de `width`), centrada: `(width−line_width)/2` a
        // `(width+line_width)/2` (vanilla lab/typst-original/crates/typst-layout/src/math/fraction.rs:60).
        // rationale: P1064 Classe 1A — centragem da linha de fração ((width - line_width) / 2.0)
        let rule_x0 = (width - line_width) / 2.0;
        items.push(FrameItem::Line {
            start: Point { x: Pt(rule_x0), y: Pt(rule_local_y) },
            end: Point { x: Pt(rule_x0 + line_width), y: Pt(rule_local_y) },
            thickness: rule_thickness,
            // P285: math frac sem stroke explícito — preserva preto bit-exact.
            color: None,
        });

        for item in den_box.items {
            items.push(offset_item(item, Pt(den_x), Pt(den_y)));
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
