//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/cancel.md
//! @prompt-hash a3d47fa0
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_cancel` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P296).

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// **P296** — Layout cancel: body + linha diagonal sobre bbox.
    /// Heurística minimal per ADR-0054 graded:
    /// - Diagonal default (bottom-left → top-right; "rising"
    ///   per vanilla angle padrão).
    /// - Sem `inverted`/`cross`/`angle`/`stroke` cosméticos —
    ///   scope-out frente futura P296.X.
    pub(super) fn layout_cancel(&self, body: &Content, style: &TextStyle) -> MathBox {
        let body_box = self.layout_node(body, style);
        // **P986/P1132q** — convenção baseline-relativa (ADR-0123): `y=0`
        // é a baseline própria do MathBox, negativo para cima. A parcela
        // de 100% vai de canto a canto; o default da linguagem soma 0.3em
        // ao comprimento da diagonal, dividido simetricamente entre as
        // pontas e projetado pela direção dinâmica da própria diagonal.
        // A espessura default também vem da linguagem: 0.05em.
        // A versão anterior usava coords
        // topo-relativas `(0, h)`→`(width, 0)` — quarto caso da família de
        // erro de convenção (P901/P906/P919/P972): o risco virava
        // sublinhado (achado §7.3). Ver `math/layout/cancel.md` §P986.
        let height = body_box.ascent + body_box.descent;
        let diagonal = body_box.width.hypot(height);
        let extra = 0.3 * style.size.val();
        let (half_x, half_y) = if diagonal > f64::EPSILON {
            (extra * body_box.width / diagonal / 2.0, extra * height / diagonal / 2.0)
        } else {
            (0.0, 0.0)
        };
        let line = FrameItem::Line {
            start: Point { x: Pt(-half_x), y: Pt(body_box.descent + half_y) },
            end: Point {
                x: Pt(body_box.width + half_x),
                y: Pt(-body_box.ascent - half_y),
            },
            // ref: lab/typst-original/crates/typst-library/src/math/cancel.rs:93
            thickness: 0.05 * style.size.val(),
            color: None,
        };
        let mut items = body_box.items;
        items.push(line);
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
