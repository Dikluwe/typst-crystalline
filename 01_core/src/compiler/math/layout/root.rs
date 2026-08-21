//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/root.md
//! @prompt-hash 0cddce97
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_root` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, MathSize, Point, Pt, TextStyle},
};

use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_root(
        &self,
        index: Option<&Content>,
        radicand: &Content,
        style: &TextStyle,
    ) -> MathBox {
        // 1. Layout do radicando
        // **P915** — radicando é cramped (achado do vanilla: "the radicand
        // is resolved in cramped style", `resolve_root`, `resolve.rs:1222-
        // 1231`). Ver `root.md` §P915.
        let radicand_style = TextStyle { cramped: true, ..style.clone() };
        let rad_box = self.layout_node(radicand, &radicand_style);

        // 3. Geometria da overline (calculada antes do radical para computar min_height)
        let line_thickness = self
            .constants
            .to_pt(self.constants.radical_rule_thickness, style.size)
            .val();
        // **P974** — o vanilla escolhe o gap por nível MathSize
        // (`radical.rs:32-36`): Display → `radical_display_style_vertical_gap`
        // (148du em NewCMMath); os restantes → `radical_vertical_gap` (50du).
        // Sem isto o alvo em Display ficava 98du curto e a variante correcta
        // do √ nunca era seleccionada (achado v5 — "altura fixa do √").
        let gap_du = if style.math_size == MathSize::Display {
            self.constants.radical_display_style_vertical_gap
        } else {
            self.constants.radical_vertical_gap
        };
        let gap = self.constants.to_pt(gap_du, style.size).val();

        // 2. Símbolo √ extensível — altura cobre radicando + gap + overline
        let rad_height_pt = rad_box.ascent + rad_box.descent + gap + line_thickness;
        let min_height_du = if style.size.val() > 0.0 {
            rad_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        // **P974** — caminho próprio para o radical: short_fall = 0 (o
        // vanilla não aplica DELIM_SHORT_FALL ao √ — `resolve.rs:1246`).
        let radical_box = self.layout_radical_symbol(min_height_du, style);
        let radical_width = radical_box.width;

        let sqrt_ascent = rad_box.ascent + gap + line_thickness;
        let descent_surd = (radical_box.ascent + radical_box.descent) - sqrt_ascent;
        let total_descent = rad_box.descent.max(descent_surd);

        let extra_asc = self
            .constants
            .to_pt(self.constants.radical_extra_ascender, style.size)
            .val();
        let inner_ascent = sqrt_ascent + extra_asc;

        let mut sqrt_x = 0.0_f64;
        let mut index_layout: Option<(super::MathBox, f64, f64)> = None; // (box, x, dy)
        let mut total_ascent = inner_ascent;
        if let Some(idx_content) = index {
            // **P915/P970** — índice cramped E scriptscript absoluto (ver §6
            // abaixo para a tabela do factor).
            let factor = match style.math_size {
                MathSize::Display | MathSize::Text => {
                    self.constants.script_script_percent_scale_down
                }
                MathSize::Script => {
                    self.constants.script_script_percent_scale_down
                        / self.constants.script_percent_scale_down
                }
                MathSize::ScriptScript => 1.0,
            };
            let script_style = TextStyle {
                size: style.size * factor,
                cramped: true,
                math_size: MathSize::ScriptScript,
                ..style.clone()
            };
            let idx_box = self.layout_node(idx_content, &script_style);

            let kern_before = self
                .constants
                .to_pt(self.constants.radical_kern_before_degree, style.size)
                .val();
            let kern_after = self
                .constants
                .to_pt(self.constants.radical_kern_after_degree, style.size)
                .val();
            let raise = self.constants.radical_degree_bottom_raise_percent;

            let sqrt_offset = kern_before + idx_box.width + kern_after;
            sqrt_x = sqrt_offset.max(0.0);
            let idx_x = -sqrt_offset.min(0.0) + kern_before;

            // `descent_surd` = profundidade do surd esticado abaixo da
            // baseline (vanilla `radical.rs:79`: sqrt.height − sqrt_ascent).
            let descent_surd =
                (radical_box.ascent + radical_box.descent) - sqrt_ascent;
            let shift_up =
                raise * (inner_ascent - descent_surd) + idx_box.descent;
            // Vanilla `radical.rs:84,95`: o ascent do composto cobre o
            // índice. Convenção baseline-relativa: a baseline do índice fica
            // a `−shift_up` (tradução de `index_pos.y = ascent −
            // index.ascent − shift_up`, ver `root.md` §P970).
            total_ascent = inner_ascent.max(shift_up + idx_box.ascent);
            index_layout = Some((idx_box, idx_x, -shift_up));
        }

        let total_width = sqrt_x + radical_width + rad_box.width;

        let mut items = Vec::new();

        // **P901** — convenção de coordenadas confirmada por leitura de
        // `hconcat_spaced`/`layout_equation`/`layout_text_node`
        // (`math/layout/mod.rs`): dentro de `MathBox.items`, `y=0` é a
        // BASELINE desta `MathBox` (não o topo), `y` cresce para baixo.
        // `hconcat_spaced` só desloca `x` ao juntar boxes irmãs — só é
        // geometricamente correcto se todas partilharem a mesma baseline
        // `y=0`; `layout_equation` chama `place()` uma única vez com
        // `baseline_y = box.ascent`, o que anula o termo `-ascent` da
        // fórmula de `place()` e faz os `y` locais sobreviverem inalterados
        // até à `Vec<FrameItem>` final. A versão anterior deste ficheiro
        // assumia (incorrectamente) `y=0` = topo da caixa, com offsets
        // positivos (para baixo) tanto para a overline como para o
        // radicando — a barra ficava perto da baseline do radicando em vez
        // de acima do topo da tinta, atravessando o glifo (achado visual de
        // P894, "a barra parece um traço/strikethrough"; ver
        // `typst-passo-901-relatorio.md`).

        // 5a. Overline — acima do topo da tinta do radicando
        //     (`-rad_box.ascent`) por `gap`, com a barra centrada na sua
        //     própria espessura (negativo — acima da baseline).
        let overline_y = -(rad_box.ascent + gap + line_thickness / 2.0);

        // 5b. Símbolo √ — conectado à overline
        let sym_dy = overline_y + radical_box.ascent;
        for item in radical_box.items {
            items.push(offset_item(item, Pt(sqrt_x), Pt(sym_dy)));
        }
        items.push(FrameItem::Line {
            start: Point { x: Pt(sqrt_x + radical_width), y: Pt(overline_y) },
            end: Point {
                x: Pt(sqrt_x + radical_width + rad_box.width),
                y: Pt(overline_y),
            },
            thickness: line_thickness,
            // P285: sqrt overline preto default (paridade matemática).
            color: None,
        });

        // 5c. Radicando — à direita do símbolo (após o deslocamento
        //     `sqrt_x` do índice, P970); SEM deslocamento vertical (a sua
        //     própria baseline, já baseline=0, já é a baseline do composto —
        //     `total_descent = rad_box.descent` já assumia isto).
        for item in rad_box.items {
            items.push(offset_item(item, Pt(sqrt_x + radical_width), Pt(0.0)));
        }

        // 6. Índice opcional (para root(n, x)) — posições já computadas
        //    acima (P970 Parte 2): `idx_x` segue os kerns de degree e a
        //    baseline fica a `−shift_up` (encaixado no vinco do √, não no
        //    topo do composto como antes de P970).
        if let Some((idx_box, idx_x, idx_dy)) = index_layout {
            for item in idx_box.items {
                items.push(offset_item(item, Pt(idx_x), Pt(idx_dy)));
            }
        }

        // **P919** — vanilla (`radical.rs:110`, confirmado por leitura): sem
        // termo de `axis_height` — a baseline do composto é simplesmente o
        // `ascent` do radicando. Ver `root.md` §P919.
        MathBox {
            width: total_width,
            ascent: inner_ascent.max(total_ascent),
            descent: total_descent,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
