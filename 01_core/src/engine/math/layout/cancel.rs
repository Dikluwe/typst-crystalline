//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/cancel.md
//! @prompt-hash eccb5b3e
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
        // **P986** — convenção baseline-relativa (ADR-0123): `y=0` é a
        // baseline própria do MathBox, negativo para cima. A linha vai do
        // canto inferior-esquerdo da tinta `(0, descent)` ao canto
        // superior-direito `(width, −ascent)` — paridade com o vanilla
        // (`cancel.rs:43-45,108-115`: linha centrada no centro do frame do
        // corpo, de canto a canto). A versão anterior usava coords
        // topo-relativas `(0, h)`→`(width, 0)` — quarto caso da família de
        // erro de convenção (P901/P906/P919/P972): o risco virava
        // sublinhado (achado §7.3). Ver `math/layout/cancel.md` §P986.
        let line = FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(body_box.descent) },
            end: Point { x: Pt(body_box.width), y: Pt(-body_box.ascent) },
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
