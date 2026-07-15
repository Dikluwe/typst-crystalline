//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout/shape_block_behaviour.md
//! @prompt-hash e6442e3f
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @layer L1
//! @updated 2026-07-15
//!
//! Atomização (ADR-0109, P377): o layout de `Shape` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.
//!
//! P767a — `Content::Shape` comporta-se como bloco no fluxo principal,
//! replicando `BlockElem::single_layouter` do vanilla.

use crate::entities::elements::shape::ShapeElem;
use crate::entities::geometry::ShapeKind;
use crate::entities::layout_types::{FrameItem, Length, Point, Pt};

use super::helpers::resolve_pt;
use super::{FontMetrics, ImageSizer, Layouter};

/// **P767a** — espaçamento por defeito de um bloco de forma, equivalente ao
/// `BlockElem::spacing` por defeito do vanilla (`Em::new(1.2)`).
const SHAPE_BLOCK_SPACING_EM: f64 = 1.2;

/// Layout de uma forma (`rect`/`circle`/`line`/…): resolve dimensões, emite
/// um `FrameItem::Shape` na página actual e avança o cursor vertical.
///
/// **P767a** — no fluxo principal as formas comportam-se como blocos:
/// quebram o parágrafo corrente e aplicam `above`/`below` de 1.2em com
/// colapso de margem, replicando `BlockElem::single_layouter` do vanilla.
/// Em sub-frames (`place`, células de grid, etc.) preserva-se o
/// comportamento anterior para não afectar posicionamento absoluto.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ShapeElem,
) {
    // **P751** — fixar a baseline inicial com o estilo activo antes de
    // posicionar a primeira forma real.
    layouter.ensure_initial_baseline();
    let (kind, width, height, fill, stroke) = (&e.kind, &e.width, &e.height, &e.fill, &e.stroke);
    let available_w = layouter.available_width();
    let (resolved_w, resolved_h) = match kind {
        // P242 — RoundedRect partilha dimensões com Rect.
        ShapeKind::Rect | ShapeKind::RoundedRect { .. } | ShapeKind::Ellipse | ShapeKind::Path(_) => {
            let w = resolve_pt(width.as_deref(), available_w);
            let h = resolve_pt(height.as_deref(), 0.0);
            (w, h)
        }
        ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
    };

    layouter.flush_line();

    // **P767a** — protocolo de bloco no fluxo principal.
    let in_main_flow = !layouter.is_sub_frame;
    if in_main_flow {
        let font = layouter.style.size.val();
        let above_pt = Length::em(SHAPE_BLOCK_SPACING_EM).resolve_pt(font);

        // Colapso de margem com o bloco anterior (P250):
        // `max(prev.below, curr.above)`; o primeiro bloco de uma Sequence
        // não leva above (`block_chain_active == false`).
        let gap = if layouter.block_chain_active {
            layouter.prev_block_below_pending.max(above_pt)
        } else {
            0.0
        };
        let advance = (gap - layouter.prev_block_below_pending).max(0.0);
        layouter.regions.current.cursor_y += Pt(advance);
        layouter.prev_block_below_pending = 0.0;

        // A forma de bloco alinha-se à margem esquerda do contentor.
        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
    }

    if layouter.regions.current.cursor_y.0 + resolved_h > layouter.regions.current.height - layouter.page_config.margin {
        layouter.new_page();
    }

    // P748/P750 — no fluxo principal o cursor_y representa a baseline do
    // texto. O topo de uma forma deve alinhar-se com o topo da linha,
    // que após P750 é medido a partir da cap-height da primeira baseline
    // (baseline − cap_height), não do ascender. Em sub-frames o cursor_y
    // já é relativo ao topo local, logo não subtraímos de novo.
    let shape_top = if layouter.is_sub_frame {
        layouter.regions.current.cursor_y
    } else {
        layouter.regions.current.cursor_y - layouter.metrics.cap_height(layouter.style.size, &layouter.style)
    };
    let pos = Point { x: layouter.regions.current.cursor_x, y: shape_top };
    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos,
        kind:   kind.clone(),
        width:  resolved_w,
        height: resolved_h,
        fill:   fill.as_ref().map(|p| p.to_color()),
        stroke: stroke.clone(),
        // P273.6 — populated by Layouter.parent_bbox (Block save/restore).
        parent_bbox_at_emit: layouter.parent_bbox,
    });

    layouter.regions.current.cursor_y += Pt(resolved_h);

    // **P767a** — below spacing + estado de bloco para colapso com o
    // elemento seguinte. Em sub-frames mantém-se o comportamento inline.
    if in_main_flow {
        let font = layouter.style.size.val();
        let below_pt = Length::em(SHAPE_BLOCK_SPACING_EM).resolve_pt(font);
        layouter.regions.current.cursor_y += Pt(below_pt);
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
    }
}
