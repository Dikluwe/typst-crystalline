//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash cedb58ca
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P378): o layout das decorações de texto
//! (`Underline`/`Strike`/`Overline`) movido do monólito `layout_content` para
//! o arquivo da feature (forma B). Primeiro arm AGRUPADO atomizado — a free
//! function recebe `&Content` e re-match interno. Content-preserving.

use crate::entities::content::Content;
use crate::entities::layout_types::{FrameItem, Point, Pt};

use super::{DecoSegment, FontMetrics, ImageSizer, Layouter};

/// Layout de uma decoração de texto (`underline`/`strike`/`overline`):
/// colecciona os segmentos de linha do body e emite um `FrameItem::Line` por
/// segmento, ao offset vertical próprio de cada tipo. `content` é o nó
/// agrupado (`Underline`/`Strike`/`Overline`).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    content:  &Content,
) {
    // Modelo D (Lote 4 P319): destructure de Arc<Elem> + kind_em
    // num só match (os 3 são tipos `Arc` distintos — sem `|`).
    let (body, stroke, offset, extent, kind_em) = match content {
        Content::Underline(e) => (&e.body, e.stroke, e.offset, e.extent,  0.10_f64),
        Content::Strike(e)    => (&e.body, e.stroke, e.offset, e.extent, -0.25),
        Content::Overline(e)  => (&e.body, e.stroke, e.offset, e.extent, -0.80),
        _ => unreachable!("arm gates Underline/Strike/Overline"),
    };
    let font_pt = layouter.font_size_pt.val();
    let offset_pt = offset
        .map(|l| l.resolve_pt(font_pt))
        .unwrap_or(kind_em * font_pt);
    let extent_pt = extent.map_or(0.0, |l| l.resolve_pt(font_pt));
    let thickness = (font_pt * 0.05).max(0.4);
    // P285 §A.3: utilizador explícito > herança do texto > default.
    let color = stroke.or(layouter.style.fill);

    // P286 — snapshot inicial + activa collector.
    let start_x_initial    = layouter.regions.current.cursor_x;
    let baseline_y_initial = layouter.regions.current.cursor_y;
    let prev_collector     = layouter.decoration_lines_collector.take();
    layouter.decoration_lines_collector = Some(Vec::new());

    layouter.layout_content(body);

    let mut segments = layouter.decoration_lines_collector
        .take().unwrap_or_default();
    // Restaurar collector outer (suporta decorações aninhadas
    // hipotéticas; LIFO save/restore standard).
    layouter.decoration_lines_collector = prev_collector;

    // P286 — patch do primeiro segment: o `start_x` real é
    // o snapshot inicial (não line_start_x), porque o body
    // pode começar a meio de uma linha já em curso.
    if let Some(first) = segments.first_mut() {
        first.start_x    = start_x_initial;
        first.baseline_y = baseline_y_initial;
    }
    // P286 — acrescentar segment "final não-flushed" (a linha
    // onde o body terminou sem causar wrap final).
    let final_end_x      = layouter.regions.current.cursor_x;
    let final_baseline_y = layouter.regions.current.cursor_y;
    let final_start_x    = if segments.is_empty() {
        start_x_initial
    } else {
        layouter.regions.current.line_start_x
    };
    if final_end_x.val() > final_start_x.val() {
        segments.push(DecoSegment {
            start_x:    final_start_x,
            end_x:      final_end_x,
            baseline_y: final_baseline_y,
        });
    }

    // P286 — emite 1 `FrameItem::Line` por segment. Extent
    // aplicado simetricamente em cada linha (§A.3 opção α).
    for seg in &segments {
        let line_y = Pt(seg.baseline_y.val() + offset_pt);
        // Cada Line vai para `current_line` da página actual.
        // Para segments do meio (já flushed), o seu baseline_y
        // pertence a uma linha já em `current_items`; ainda
        // assim push em current_line é válido — o Layouter
        // não reordena por Y, apenas concatena no flush
        // seguinte (paridade vanilla painter pós-frame).
        layouter.regions.current.current_line.push(FrameItem::Line {
            start:     Point { x: Pt(seg.start_x.val() - extent_pt), y: line_y },
            end:       Point { x: Pt(seg.end_x.val()   + extent_pt), y: line_y },
            thickness,
            color,
        });
    }
}
