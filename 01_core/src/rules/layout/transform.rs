//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 1d932b14
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P377): o layout de `Transform` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

use crate::entities::elements::transform::TransformElem;
use crate::entities::layout_types::{FrameItem, Point, Pt, TransformMatrix};

use super::helpers::{collect_sub_items, measure_content};
use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma transformação afim (`rotate`/`scale`/…): mede a AABB do body,
/// projecta os cantos pela matriz, compensa origem negativa e emite um
/// `FrameItem::Group` com a matriz final.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &TransformElem,
) {
    let matrix = &e.matrix;
    let body = &e.body;
    let (orig_w, orig_h) = measure_content(body, layouter.available_width());

    // Projectar os quatro cantos da AABB original através da matriz.
    let corners = [
        matrix.apply(0.0, 0.0),
        matrix.apply(orig_w, 0.0),
        matrix.apply(0.0, orig_h),
        matrix.apply(orig_w, orig_h),
    ];
    let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,     f64::min);
    let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,     f64::min);
    let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let _new_w = max_x - min_x;
    let new_h = max_y - min_y;

    if layouter.regions.current.cursor_y.0 + new_h > layouter.regions.current.height - layouter.page_config.margin {
        layouter.new_page();
    }
    layouter.flush_line();

    let pos = Point { x: layouter.regions.current.cursor_x, y: layouter.regions.current.cursor_y };

    // Compensação de origem negativa: garante que o canto mais à esquerda/acima
    // da forma transformada coincide com pos.
    let align        = TransformMatrix::translate(-min_x, -min_y);
    let final_matrix = align.concat(matrix);

    let available_w  = layouter.available_width();
    let sub_items    = collect_sub_items(body, available_w);

    layouter.regions.current.current_items.push(FrameItem::Group {
        pos,
        matrix:       final_matrix,
        clip_mask:    None,
        inner_width:  orig_w,
        inner_height: orig_h,
        items:        sub_items,
    });

    layouter.regions.current.cursor_y += Pt(new_h);
}
