//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/cancel.md
//! @prompt-hash b31450dd
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_cancel` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P296).

use crate::engine::layout::FontMetrics;
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
        let h = body_box.height();
        // Linha diagonal de canto inferior-esquerdo (0, h) a canto
        // superior-direito (width, 0). Local coords relativos a topo
        // do MathBox.
        let line = FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(h) },
            end: Point { x: Pt(body_box.width), y: Pt(0.0) },
            thickness: 0.5,
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
