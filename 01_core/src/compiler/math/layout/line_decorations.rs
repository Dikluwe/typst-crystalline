//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/line_decorations.md
//! @prompt-hash 9e0c2065
//! @layer L1
//! @updated 2026-09-30
//!
//! Linhas decorativas matemáticas em coordenadas de baseline.

use super::*;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    /// **P990-C** — `Content::Strike` em contexto math: layout do corpo
    /// (já com itálico por defeito, `apply_math_default`) + uma
    /// `FrameItem::Line` horizontal sobre a caixa. Mesma geometria do lado
    /// de texto (`compiler/layout/decorations.rs::layout`): offset por
    /// omissão `-0.25em`, thickness `max(0.05em, 0.4pt)`, extent simétrico
    /// nos dois lados. Convenção baseline-relativa (ADR-0123: `y=0` =
    /// baseline própria do `MathBox`, positivo para baixo) — `offset_pt`
    /// já vem negativo (acima da baseline) e aplica-se directamente, sem
    /// somar a nenhum `y` de base (ao contrário do lado de texto, que soma
    /// a `baseline_y` de cada linha). Ver `_comum.md` §P990-C.
    pub(super) fn layout_overline(
        &self,
        body: &Content,
        stroke: Option<Color>,
        _offset: Option<Length>,
        _extent: Option<Length>,
        style: &TextStyle,
    ) -> MathBox {
        let cramped_style = TextStyle { cramped: true, ..style.clone() };
        let body_box = self.layout_node(body, &cramped_style);

        let size = style.size;
        let sep = self
            .constants
            .to_pt(self.constants.overbar_extra_ascender, size)
            .val();
        let thickness = self
            .constants
            .to_pt(self.constants.overbar_rule_thickness, size)
            .val();
        let gap = self.constants.to_pt(self.constants.overbar_vertical_gap, size).val();
        let extra_height = sep + thickness + gap;

        let width = body_box.width;
        let ascent = body_box.ascent + extra_height;
        let descent = body_box.descent;

        let line_y = -(body_box.ascent + gap + thickness / 2.0);
        let color = stroke.or(style.fill.clone());

        let mut items = body_box.items;
        items.push(FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(line_y) },
            end: Point { x: Pt(width), y: Pt(line_y) },
            thickness,
            color,
            cap: crate::entities::geometry::LineCap::Butt,
        });

        MathBox { width, ascent, descent, items }
    }

    pub(super) fn layout_underline(
        &self,
        body: &Content,
        stroke: Option<Color>,
        _offset: Option<Length>,
        _extent: Option<Length>,
        style: &TextStyle,
    ) -> MathBox {
        let body_box = self.layout_node(body, style);

        let size = style.size;
        let sep = self
            .constants
            .to_pt(self.constants.underbar_extra_descender, size)
            .val();
        let thickness = self
            .constants
            .to_pt(self.constants.underbar_rule_thickness, size)
            .val();
        let gap = self.constants.to_pt(self.constants.underbar_vertical_gap, size).val();
        let extra_height = sep + thickness + gap;

        let width = body_box.width;
        let ascent = body_box.ascent;
        let descent = body_box.descent + extra_height;

        let line_y = body_box.descent + gap + thickness / 2.0;
        let color = stroke.or(style.fill.clone());

        let mut items = body_box.items;
        items.push(FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(line_y) },
            end: Point { x: Pt(width), y: Pt(line_y) },
            thickness,
            color,
            cap: crate::entities::geometry::LineCap::Butt,
        });

        MathBox { width, ascent, descent, items }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
