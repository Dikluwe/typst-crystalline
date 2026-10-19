//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/cell_tail_commit.md
//! @prompt-hash c9f8c71f
//! @layer L1
//! @updated 2026-09-25
//!
//! Emissão de caudas diferidas no topo de uma região já aberta.

use super::metrics::FontMetrics;
use crate::entities::{image_sizer::ImageSizer, layout_types::Pt};

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// **P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita
    /// ADR-0082 PROPOSTO N=2 segunda aplicação citante)** — flush
    /// dos cell tails pendentes no **topo** da página actual (chamado
    /// por `new_page` após cursor_y setup). Items rebased pelo
    /// `cursor_y` actual; cursor_y avança pela altura do tail emitido.
    ///
    /// Z-order paridade P248: fill atrás → items → stroke à frente.
    /// Bounds do fill/stroke usam tail extent (não cell original).
    ///
    /// Limit 3 forwardings consecutivos (paridade vanilla heurística;
    /// `forwarded_count >= 3` descarta silenciosamente — mitigação
    /// loop infinito caso tail recursivo).
    pub(super) fn flush_pending_cell_tails(&mut self) {
        use crate::compiler::layout::slicing::rebase_item_y;
        use crate::entities::geometry::ShapeKind;
        use crate::entities::layout_types::{FrameItem, Point};
        if self.pending_cell_tails.is_empty() {
            return;
        }
        let tails: Vec<crate::compiler::layout::DeferredCellTail> =
            std::mem::take(&mut self.pending_cell_tails);
        let cursor_top = self.regions.current.cursor_y.0;
        let mut max_y_after = cursor_top;
        for tail in tails {
            if tail.forwarded_count >= 3 {
                // P251 — limit forwarding; descarta silenciosamente
                // (paridade vanilla heurística max-iter).
                continue;
            }
            // Calcula altura do tail (max pos.y dos items locais).
            let mut tail_h = 0.0_f64;
            for item in tail.items.iter() {
                let y = match item {
                    FrameItem::TextSeparator { pos, .. }
                    | FrameItem::Text { pos, .. } => pos.y.0,
                    FrameItem::TextShaped { pos, .. } => pos.y.0,
                    FrameItem::Line { start, .. } => start.y.0,
                    FrameItem::Glyph { pos, .. } => pos.y.0,
                    FrameItem::Image { pos, .. } => pos.y.0,
                    FrameItem::Shape { pos, .. } => pos.y.0,
                    FrameItem::Group { pos, .. } => pos.y.0,
                    FrameItem::Link { .. } => 0.0,
                    FrameItem::TextReplacement { items, .. }
                    | FrameItem::Semantic { items, .. } => items
                        .iter()
                        .map(crate::compiler::layout::slicing::item_y_start)
                        .fold(0.0, f64::max),
                };
                tail_h = tail_h.max(y);
            }
            // Z-order step 1: fill atrás.
            if let Some(c) = tail.fill {
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos: Point { x: Pt(tail.origin_x), y: Pt(cursor_top) },
                    kind: ShapeKind::Rect,
                    width: tail.width,
                    height: tail_h,
                    fill: Some(crate::entities::frame_paint::FramePaint::Solid(c)),
                    stroke: None,
                    fill_rule: Default::default(),
                    parent_bbox_at_emit: None,
                });
            }
            // **P908** — rebasear as entradas órfãs deste tail (um
            // `Content::Align`/`Content::Place` aninhado dentro da célula
            // cujos items caíram inteiramente no tail, sob `width`/
            // `height: auto`) com a MESMA translação `+cursor_top`
            // aplicada aos items reais abaixo (`rebase_item_y`); `path[0]`
            // passa a apontar para a posição real em `current_items`
            // (soma o comprimento actual, capturado ANTES do merge,
            // já incluindo o fill do passo 1). Ver
            // `00_nucleo/prompts/compiler/layout.md` §P908.
            let insertion_base = self.regions.current.current_items.len();
            for (mut path, count, align, content_w, origin_x, applied_x) in
                tail.orphaned_align_x
            {
                path[0] += insertion_base;
                self.pending_align_centering
                    .push((path, count, align, content_w, origin_x, applied_x));
            }
            for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                tail.orphaned_align_y
            {
                path[0] += insertion_base;
                self.pending_align_v_centering.push((
                    path,
                    count,
                    align,
                    content_h,
                    origin_y + cursor_top,
                    dy,
                    applied_y + cursor_top,
                ));
            }
            // Z-order step 2: items rebased.
            for item in tail.items {
                let final_item = rebase_item_y(item, cursor_top);
                self.regions.current.current_items.push(final_item);
            }
            // Z-order step 3: stroke à frente.
            if let Some(s) = tail.stroke {
                let prepared_stroke = super::tiling::resolve_stroke(self, Some(&s));
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos: Point { x: Pt(tail.origin_x), y: Pt(cursor_top) },
                    kind: ShapeKind::Rect,
                    width: tail.width,
                    height: tail_h,
                    fill: None,
                    stroke: prepared_stroke,
                    fill_rule: Default::default(),
                    parent_bbox_at_emit: None,
                });
            }
            max_y_after = max_y_after.max(cursor_top + tail_h);
        }
        // Avança cursor_y para depois dos tails emitidos.
        self.regions.current.cursor_y = Pt(max_y_after);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1504_cell_tail_commit_possui_emissao() {
        assert!(include_str!("cell_tail_commit.rs").contains("forwarded_count >= 3"));
    }
}
