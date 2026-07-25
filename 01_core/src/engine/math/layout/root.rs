//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/root.md
//! @prompt-hash d91669fc
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_root` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
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
        let rad_box = self.layout_node(radicand, style);

        // 3. Geometria da overline (calculada antes do radical para computar min_height)
        let line_thickness = self
            .constants
            .to_pt(self.constants.radical_rule_thickness, style.size)
            .val();
        let gap = self
            .constants
            .to_pt(self.constants.radical_vertical_gap, style.size)
            .val();

        // 2. Símbolo √ extensível — altura cobre radicando + gap + overline
        let rad_height_pt = rad_box.ascent + rad_box.descent + gap + line_thickness;
        let min_height_du = if style.size.val() > 0.0 {
            rad_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        let radical_box = self.layout_stretchy_delimiter('√', min_height_du, style);
        let radical_width = radical_box.width;

        // 4. Dimensões totais
        //    ascent cobre: ascent do radicando + gap + espessura da linha
        let total_ascent = rad_box.ascent + gap + line_thickness;
        let total_descent = rad_box.descent;
        let total_width = radical_width + rad_box.width;

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

        // 5a. Símbolo √ — a `radical_box` devolvida por
        //     `layout_stretchy_delimiter` já está na convenção baseline=0
        //     (a sua própria baseline local); desloca-se para que o TOPO do
        //     glifo esticado (`radical_box.ascent` acima da sua baseline)
        //     coincida com o topo da barra (`y = -total_ascent`):
        //     `dy = radical_box.ascent - total_ascent` (negativo — sobe).
        let sym_dy = radical_box.ascent - total_ascent;
        for item in radical_box.items {
            items.push(offset_item(item, Pt(0.0), Pt(sym_dy)));
        }

        // 5b. Overline — acima do topo da tinta do radicando
        //     (`-rad_box.ascent`) por `gap`, com a barra centrada na sua
        //     própria espessura (negativo — acima da baseline).
        let overline_y = -(rad_box.ascent + gap + line_thickness / 2.0);
        items.push(FrameItem::Line {
            start: Point { x: Pt(radical_width), y: Pt(overline_y) },
            end: Point {
                x: Pt(radical_width + rad_box.width),
                y: Pt(overline_y),
            },
            thickness: line_thickness,
            // P285: sqrt overline preto default (paridade matemática).
            color: None,
        });

        // 5c. Radicando — à direita do símbolo; SEM deslocamento vertical
        //     (a sua própria baseline, já baseline=0, já é a baseline do
        //     composto — `total_descent = rad_box.descent` já assumia isto).
        for item in rad_box.items {
            items.push(offset_item(item, Pt(radical_width), Pt(0.0)));
        }

        // 6. Índice opcional (para root(n, x))
        if let Some(idx_content) = index {
            let script_style = TextStyle {
                size: style.size * self.constants.script_percent_scale_down,
                ..style.clone()
            };
            let idx_box = self.layout_node(idx_content, &script_style);
            // Posicionar acima e à esquerda do símbolo: x=20% da largura do
            // radical; y = topo do composto (`-total_ascent`) — mesma
            // intenção original ("topo"), corrigida para a convenção
            // baseline=0 confirmada em P901 (antes: `y=0.0`, que sob a
            // convenção correcta colocaria o índice na baseline, não no
            // topo — regressão evitada pela revisão do orquestrador, não
            // coberta pelos 2 testes do Agente A, que só verificam a
            // relação overline-vs-radicando).
            let idx_x = radical_width * 0.2;
            let idx_dy = -total_ascent;
            for item in idx_box.items {
                items.push(offset_item(item, Pt(idx_x), Pt(idx_dy)));
            }
        }

        let result = MathBox {
            width: total_width,
            ascent: total_ascent,
            descent: total_descent,
            items,
        };
        self.apply_axis_offset(result, style.size)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
