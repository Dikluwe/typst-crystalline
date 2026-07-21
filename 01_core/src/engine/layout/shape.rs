//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout/shape_block_behaviour.md
//! @prompt-hash a54abe5a
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @layer L1
//! @updated 2026-07-15
//!
//! Atomização (ADR-0109, P377): o layout de `Shape` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.
//!
//! P767a — `Content::Shape` comporta-se como bloco no fluxo principal,
//! replicando `BlockElem::single_layouter` do vanilla.
//! P767c — ancoramento vertical corrigido quando a forma sucede texto não-bloco.

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
    e: &ShapeElem,
) {
    // **P751** — fixar a baseline inicial com o estilo activo antes de
    // posicionar a primeira forma real.
    layouter.ensure_initial_baseline();
    let (kind, width, height, fill, stroke) =
        (&e.kind, &e.width, &e.height, &e.fill, &e.stroke);
    let available_w = layouter.available_width();
    let (resolved_w, resolved_h) = match kind {
        // P242 — RoundedRect partilha dimensões com Rect.
        ShapeKind::Rect
        | ShapeKind::RoundedRect { .. }
        | ShapeKind::Ellipse
        | ShapeKind::Path(_) => {
            let w = resolve_pt(width.as_deref(), available_w);
            let h = resolve_pt(height.as_deref(), 0.0);
            (w, h)
        }
        ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
    };

    // **P767c** — guardar a baseline da linha que vai ser descarregada.
    // Após `flush_line`, o cursor aponta para a baseline da *próxima* linha;
    // para ancorar a forma na grelha de linhas do parágrafo actual, usamos
    // a baseline *antes* do avanço.
    let baseline_before_flush = layouter.regions.current.cursor_y;
    layouter.flush_line();
    let cursor_after_flush = layouter.regions.current.cursor_y;
    let had_text_line = cursor_after_flush.0 > baseline_before_flush.0 + 1e-6;

    // **P767a** — protocolo de bloco no fluxo principal.
    let in_main_flow = !layouter.is_sub_frame;
    let font = layouter.style.size.val();
    let above_pt = Length::em(SHAPE_BLOCK_SPACING_EM).resolve_pt(font);
    let below_pt = Length::em(SHAPE_BLOCK_SPACING_EM).resolve_pt(font);
    let cap_height = layouter.metrics.cap_height(layouter.style.size, &layouter.style);

    if in_main_flow {
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

    if layouter.regions.current.cursor_y.0 + resolved_h
        > layouter.regions.current.height - layouter.page_config.margin
    {
        layouter.new_page();
    }

    // P748/P750 — no fluxo principal o cursor_y representa a baseline do
    // texto. O `pos.y` de `FrameItem::Shape` é a base da forma em
    // coordenadas do Layouter (Y crescente para cima a partir da base da
    // página); a forma estende-se para cima pela sua altura.
    //
    // **P767c** — quando a forma é a primeira depois de texto não-bloco,
    // o vanilla ancora a *base* da forma em `baseline + above`; o topo da
    // forma fica em `baseline + above + height`. Quando a forma sucede
    // outro bloco, ou quando é a primeira de uma Sequence sem texto antes,
    // mantém-se o modelo P767a: base da forma em `baseline − cap_height`
    // (topo da linha anterior, estendendo-se para cima).
    let shape_base = if !in_main_flow {
        layouter.regions.current.cursor_y
    } else if layouter.block_chain_active {
        layouter.regions.current.cursor_y - cap_height
    } else if had_text_line {
        baseline_before_flush + Pt(above_pt)
    } else {
        layouter.regions.current.cursor_y - cap_height
    };
    let pos = Point {
        x: layouter.regions.current.cursor_x,
        y: shape_base,
    };
    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos,
        kind: kind.clone(),
        width: resolved_w,
        height: resolved_h,
        fill: fill.as_ref().map(|p| p.to_color()),
        stroke: stroke.clone(),
        // P273.6 — populated by Layouter.parent_bbox (Block save/restore).
        parent_bbox_at_emit: layouter.parent_bbox,
    });

    // **P767c** — avanço do cursor conforme o tipo de ancoragem. Após
    // forma-a-seguir-a-texto, a próxima baseline fica no topo da forma +
    // `below + cap_height`; após forma-a-seguir-a-bloco, ou forma isolada,
    // mantém-se o comportamento P767a (base da forma + `below`).
    if in_main_flow {
        if layouter.block_chain_active {
            layouter.regions.current.cursor_y = shape_base + Pt(resolved_h + below_pt);
        } else if had_text_line {
            layouter.regions.current.cursor_y =
                shape_base + Pt(resolved_h + below_pt + cap_height.0);
        } else {
            layouter.regions.current.cursor_y = shape_base + Pt(resolved_h + below_pt);
        }
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
    } else {
        layouter.regions.current.cursor_y += Pt(resolved_h);
    }
}
