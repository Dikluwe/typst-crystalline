//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash bf8f0b19
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
    let (body_h, sub_items, deco_segments, orphaned_align_x, orphaned_align_y) = layouter
        .layout_sub_frame(
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

    // **P908** — família "envolvimento em `Group`": os `sub_items`
    // (coordenadas LOCAIS, pré-matriz) tornam-se `Group.items` — a
    // correcção diferida de um `Align`/`Place` aninhado aqui dentro
    // precisa de um nível extra de `path` (o índice deste `Group` na
    // lista onde vai ser inserido, capturado ANTES do `push` abaixo).
    //
    // **Diferença crucial face à família "merge imediato"
    // (`layout_align`/`layout_place`)**: essas duas SEMPRE compõem os
    // items reais via `iy - sub_origin_y`/`y_offset` — uma subtracção do
    // ascender que, quando o conteúdo veio de um `Place` aninhado
    // (`in_sub_frame`, `y_offset=0`, ascender NÃO cancelado na emissão
    // própria do Place), acaba por cancelar esse ascender residual "de
    // borla" durante a composição. `Content::Transform` empurra
    // `sub_items` para `Group.items` SEM qualquer composição — o
    // ascender residual permanece no item físico.
    //
    // **`origin_y` vs `applied_y` — tratamento ASSIMÉTRICO** (confirmado
    // por teste exploratório, `p908_place_aninhado_em_transform_sob_
    // height_auto_...`): `origin_y` é o ANCORA lógico usado por
    // `apply_pending_align_v_fixups` para calcular `page_height - margin
    // - origin_y` — só leva `pos.y` (a origem do `Group`). Somar também o
    // ascender a `origin_y` empurra-o para além de `page_height - margin`
    // sempre que o `Group` já ocupa a maior parte da página, disparando o
    // clamp `max(0.0, …)` e quebrando o cancelamento algébrico do termo
    // `origin_y` na fórmula de `resolve_alignment` (Bottom/Horizon) —
    // sintoma observado: erro residual de exactamente 1 ascender.
    // `applied_y` (a posição já "aplicada", contra a qual o delta final é
    // medido) tem de bater com a posição FÍSICA actual do item
    // (`item.pos.y_original = applied_y_original + ascender`, porque o
    // ascender não foi cancelado na emissão do Place aninhado) — por isso
    // `applied_y`, ao contrário de `origin_y`, leva o termo extra do
    // ascender. Eixo X não tem esta assimetria nenhuma (`layout_place`
    // traduz X sempre da mesma forma, independente de `in_sub_frame`) —
    // só `pos.x`, sem termo extra em nenhum dos dois campos. Ver
    // `00_nucleo/prompts/compiler/layout.md` §P908.
    let (ascender_local, _) =
        layouter.metrics.vertical_metrics(layouter.style.size, &layouter.style);
    let group_idx = layouter.regions.current.current_items.len();
    for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_align_x {
        path.insert(0, group_idx);
        layouter.pending_align_centering.push((
            path,
            count,
            align,
            content_w,
            origin_x + pos.x.val(),
            applied_x + pos.x.val(),
        ));
    }
    for (mut path, count, align, content_h, origin_y, dy, applied_y) in orphaned_align_y {
        path.insert(0, group_idx);
        layouter.pending_align_v_centering.push((
            path,
            count,
            align,
            content_h,
            // `origin_y` é o ANCORA lógico (o "topo" contra o qual o
            // espaço restante é medido, `page_height - margin - origin_y`
            // em `apply_pending_align_v_fixups`) — sem ascender, só
            // `pos.y`; somar o ascender aqui empurraria `origin_y` para lá
            // de `page_height - margin`, disparando o clamp `max(0.0, …)`
            // e quebrando o cancelamento algébrico (confirmado por teste
            // exploratório).
            origin_y + pos.y.val(),
            dy,
            // `applied_y` tem de bater com a posição FÍSICA actual do
            // item (`item.pos.y_original = applied_y_original + ascender`,
            // porque o Place aninhado emitiu com `y_offset=0` — ascender
            // não cancelado, ver nota acima) — por isso leva o termo extra
            // do ascender que `origin_y` não leva.
            applied_y + pos.y.val() + ascender_local.val(),
        ));
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
