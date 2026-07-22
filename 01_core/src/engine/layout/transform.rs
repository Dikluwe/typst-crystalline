//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash a54abe5a
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P377): o layout de `Transform` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.
//!
//! **P832 (achado #58, GRAVE)** — o body passa a ser layoutado num sub-frame
//! real (`layout_sub_frame`, P625/P629) em vez da colecção manual
//! `collect_sub_items`, que só tratava `Shape`/`Sequence` e descartava texto
//! (e todo o resto) silenciosamente. Paridade vanilla: `transform.rs` do
//! vanilla layouta o body para uma frame e embrulha-a num grupo com a matriz.

use crate::entities::elements::transform::TransformElem;
use crate::entities::layout_types::{FrameItem, Point, Pt, TransformMatrix};

use super::helpers::measure_content;
use super::sub_frame::SubLayoutRegion;
use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma transformação afim (`move`/`rotate`/`scale`): layouta o
/// body num sub-frame isolado (cobre conteúdo arbitrariamente aninhado —
/// texto, shapes, sequências, outros transforms), mede a AABB dos items
/// produzidos, projecta os cantos pela matriz, compensa origem negativa e
/// emite um `FrameItem::Group` com a matriz final.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TransformElem,
) {
    let matrix = &e.matrix;
    let body = &e.body;
    let available_w = layouter.available_width();

    // P832 — sub-layout real do body (save/restore completo do estado do
    // layouter; não afecta o cursor do frame pai).
    let (body_h, sub_items, deco_segments) = layouter.layout_sub_frame(
        body,
        SubLayoutRegion {
            origin_x: 0.0,
            width: available_w,
            height: None,
            align_rtl: true,
            unconstrained_height: true,
        },
    );

    // AABB do body: largura = limite direito real dos items; altura = altura
    // do sub-layout. Fallback à medição aproximada quando o body não produz
    // items mensuráveis (preserva o comportamento pré-P832 para esse caso).
    let measured_w =
        super::helpers::line_content_right(&sub_items, &layouter.metrics);
    let (orig_w, orig_h) = if measured_w > 0.0 || body_h > 0.0 {
        (measured_w, body_h)
    } else {
        measure_content(body, available_w)
    };

    // Projectar os quatro cantos da AABB original através da matriz.
    let corners = [
        matrix.apply(0.0, 0.0),
        matrix.apply(orig_w, 0.0),
        matrix.apply(0.0, orig_h),
        matrix.apply(orig_w, orig_h),
    ];
    let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let _new_w = max_x - min_x;
    let new_h = max_y - min_y;

    if layouter.regions.current.cursor_y.0 + new_h
        > layouter.regions.current.height - layouter.page_config.margin
    {
        layouter.new_page();
    }
    layouter.flush_line();

    let pos = Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };

    // Compensação de origem negativa: garante que o canto mais à esquerda/acima
    // da forma transformada coincide com pos.
    let align = TransformMatrix::translate(-min_x, -min_y);
    let final_matrix = align.concat(matrix);

    // P832 — os segmentos de decoração do sub-frame (só existem se o
    // transform estiver aninhado dentro de Underline/Strike/Overline) são
    // re-inseridos no collector ambiente traduzidos pela origem do grupo.
    // Limitação registada: a matriz da transformação não é aplicada aos
    // segmentos (decoração sob rotação fica com a geometria sem rotação).
    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {
        for seg in deco_segments {
            collector.push(super::DecoSegment {
                start_x: seg.start_x + pos.x,
                end_x: seg.end_x + pos.x,
                baseline_y: seg.baseline_y + pos.y,
            });
        }
    }

    layouter.regions.current.current_items.push(FrameItem::Group {
        pos,
        matrix: final_matrix,
        clip_mask: None,
        inner_width: orig_w,
        inner_height: orig_h,
        items: sub_items,
    });

    layouter.regions.current.cursor_y += Pt(new_h);
}
