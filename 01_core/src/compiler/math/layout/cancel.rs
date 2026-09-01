//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/cancel.md
//! @prompt-hash aef83f0a
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_cancel` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P296).

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    elements::math_cancel::{MathCancelAngle, MathCancelElem},
    layout_types::{Angle, FrameItem, Point, Pt, TextStyle},
};

use super::{callbacks, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// Compatibility surface for the pre-P1291 internal tests and callers.
    pub(super) fn layout_cancel(
        &self,
        body: &crate::entities::content::Content,
        style: &TextStyle,
    ) -> MathBox {
        let elem = MathCancelElem {
            body: body.clone(),
            length: crate::entities::rel::Rel::from_percent(100.0)
                + crate::entities::layout_types::Length::em(0.3),
            inverted: false,
            cross: false,
            angle: MathCancelAngle::Auto,
            stroke: None,
            background: false,
            span: crate::entities::span::Span::detached(),
        };
        self.layout_cancel_elem(&elem, style, 0)
    }

    /// **P296** — Layout cancel: body + linha diagonal sobre bbox.
    /// Heurística minimal per ADR-0054 graded:
    /// - Diagonal default (bottom-left → top-right; "rising"
    ///   per vanilla angle padrão).
    /// - Sem `inverted`/`cross`/`angle`/`stroke` cosméticos —
    ///   scope-out frente futura P296.X.
    pub(super) fn layout_cancel_elem(
        &self,
        elem: &MathCancelElem,
        style: &TextStyle,
        occurrence: usize,
    ) -> MathBox {
        // The occurrence was reserved by the dispatcher before this descent.
        let body_box = self.layout_node(&elem.body, style);
        let height = body_box.ascent + body_box.descent;
        let diagonal = body_box.width.hypot(height);
        let line_length =
            elem.length.rel * diagonal + elem.length.abs.resolve_pt(style.size.val());
        let auto = Angle::rad(body_box.width.atan2(height));
        let angle_for = |line: u8| {
            let requested_default = auto;
            match &elem.angle {
                MathCancelAngle::Auto => requested_default,
                MathCancelAngle::Angle(angle) => *angle,
                MathCancelAngle::Func(func) => {
                    let resolved =
                        match (self.equation, self.lexical_styles, self.callback_pass) {
                            (Some(equation), Some(chain), Some(pass)) => pass
                                .resolve_or_record(callbacks::MathCancelRequest {
                                    id: callbacks::MathCancelRequestId {
                                        equation,
                                        occurrence,
                                        line,
                                    },
                                    func: func.clone(),
                                    default: requested_default,
                                    span: elem.span,
                                    styles: callbacks::snapshot_styles(chain, style),
                                }),
                            _ => None,
                        };
                    resolved.unwrap_or(requested_default)
                }
            }
        };
        let make_line = |line: u8| {
            let mut angle = angle_for(line);
            let invert = if elem.cross { line == 1 } else { elem.inverted };
            if invert {
                angle = Angle::rad(-angle.to_rad());
            }
            let half_x = angle.to_rad().sin() * line_length / 2.0;
            let half_y = angle.to_rad().cos() * line_length / 2.0;
            // rationale: the cancel stroke is centered on the body box, so the
            // horizontal midpoint is exactly half of its measured width.
            let center_x = body_box.width / 2.0;
            let center_y = (body_box.descent - body_box.ascent) / 2.0;
            let (thickness, color) = match &elem.stroke {
                Some(stroke) => (stroke.thickness, Some(stroke.paint.to_color())),
                None => (0.05 * style.size.val(), style.fill),
            };
            FrameItem::Line {
                start: Point { x: Pt(center_x - half_x), y: Pt(center_y + half_y) },
                end: Point { x: Pt(center_x + half_x), y: Pt(center_y - half_y) },
                thickness,
                color,
            }
        };
        let mut items = body_box.items;
        let mut lines = vec![make_line(0)];
        if elem.cross {
            lines.push(make_line(1));
        }
        if elem.background {
            lines.extend(items);
            items = lines;
        } else {
            items.extend(lines);
        }
        MathBox {
            width: body_box.width,
            ascent: body_box.ascent,
            descent: body_box.descent,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
