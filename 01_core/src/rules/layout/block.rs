//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 720bdf3e
//! @layer L1
//! @updated 2026-06-18
//!
//! Atomização (ADR-0109, P376): o layout de `Block` movido do monólito
//! `layout_content` para o arquivo da feature. Content-preserving — a lógica
//! é idêntica; só o endereço mudou. O `match` no núcleo delega numa linha.

use crate::entities::elements::block::BlockElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout do container `Block` (P156G/P242/P243/P247/P248/P250/P252/P273.6).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &BlockElem,
) {
    let (body, width, height, inset, breakable, outset, radius, clip, fill, stroke, spacing, above, below) =
        (&e.body, &e.width, &e.height, &e.inset, &e.breakable, &e.outset, &e.radius, &e.clip, &e.fill, &e.stroke, &e.spacing, &e.above, &e.below);
    let font = layouter.font_size_pt.val();
    let inset_left   = inset.left.resolve_pt(font);
    let inset_top    = inset.top.resolve_pt(font);
    let inset_bottom = inset.bottom.resolve_pt(font);
    // inset.right é scope-out (mesma razão que Pad.right
    // em P156C — refino com refactor multi-region).

    // P247 — outset semantic real activação (cenário A audit:
    // outset zero-uso pré-P247). Margem externa visual que
    // expande bounds Shape mas NÃO afecta cursor interno do
    // body (paralelo a margin CSS).
    let outset_left   = outset.left.resolve_pt(font);
    let outset_right  = outset.right.resolve_pt(font);
    let outset_top    = outset.top.resolve_pt(font);
    let outset_bottom = outset.bottom.resolve_pt(font);
    let has_shape = fill.is_some() || stroke.is_some();
    let has_outset = outset_left != 0.0 || outset_right != 0.0
                     || outset_top != 0.0 || outset_bottom != 0.0;

    // 1. Termina linha em curso.
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    // P250 — spacing/above collapse (paridade vanilla CSS margin
    // collapse `max(prev.below, curr.above)` entre Blocks
    // consecutivos; `above` suprimido no primeiro Block dum
    // Sequence — sinalizado via `block_chain_active == false`).
    let above_pt = above
        .or(*spacing)
        .map(|l| l.resolve_pt(font))
        .unwrap_or(0.0);
    let gap = if layouter.block_chain_active {
        layouter.prev_block_below_pending.max(above_pt)
    } else {
        0.0
    };
    let advance = (gap - layouter.prev_block_below_pending).max(0.0);
    layouter.regions.current.cursor_y += Pt(advance);
    layouter.prev_block_below_pending = 0.0;

    // P248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa) —
    // Block.breakable semantic real activação: medição
    // antecipada via `measure_content_constrained` (puro;
    // §2.4 audit). Quando `breakable: false` E o bloco não
    // cabe no espaço restante mas cabe numa página inteira,
    // `new_page()` antecipa o break. Quando o bloco excede a
    // página inteira, emit normal (paridade vanilla
    // "overlong atómico"). Default `breakable: true` preserva
    // comportamento P156G literal.
    if !*breakable {
        let avail_w = match width {
            Some(w) => w.resolve_pt(font),
            None    => layouter.available_width(),
        };
        let (_, body_h) = layouter.measure_content_constrained(body, avail_w);
        let height_min = height.map(|h| h.resolve_pt(font)).unwrap_or(0.0);
        let inner_h = body_h.max(height_min);
        let block_total_h = outset_top + inset_top + inner_h
                           + inset_bottom + outset_bottom;
        let page_usable_h = layouter.available_height();
        let remaining_h = layouter.page_bottom_limit()
                        - layouter.regions.current.cursor_y.0;
        if block_total_h <= page_usable_h && block_total_h > remaining_h {
            // Cabe em página nova mas não na actual → break antecipado.
            layouter.new_page();
        }
        // else: cabe na actual OU overlong (emit normal — paridade vanilla).
    }

    // P247 — snapshot items_before para inserir Shape ANTES
    // do body (Z-order: fill+stroke atrás do conteúdo).
    let items_before = layouter.regions.current.current_items.len();

    // 2. Captura cursor.y para verificar height mínimo no fim.
    let start_y = layouter.regions.current.cursor_y.0;

    // P247 — outset.top avança cursor antes do inset.top
    // (margem externa precede margem interna).
    layouter.regions.current.cursor_y += Pt(outset_top);

    // 3. Aplica inset top.
    layouter.regions.current.cursor_y += Pt(inset_top);

    // 4. Aplica inset left (e width se especificado).
    let saved_line_start = layouter.regions.current.line_start_x;
    let saved_width      = layouter.regions.current.width;
    layouter.regions.current.line_start_x = saved_line_start + Pt(inset_left);
    layouter.regions.current.cursor_x     = layouter.regions.current.line_start_x;

    // P243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
    // — promoção real `Block.width`: quando `Some(w)`,
    // clampa `regions.current.width` ao valor user-provided
    // durante body layout via save/restore. `width: None`
    // preserva largura herdada.
    if let Some(w) = width {
        let w_pt = w.resolve_pt(font);
        let line_start_pt = layouter.regions.current.line_start_x.0;
        // Width efectiva = line_start + w_pt (ponto onde wrap deve ocorrer).
        layouter.regions.current.width = (line_start_pt + w_pt).max(0.0);
    }

    // P242 (M9d / M7+5) — clip=true emite FrameItem::Group com
    // clip_mask `RoundedRect{radii}` (radius non-zero) ou `Rect`
    // (radius zero; paridade DEBT-30 P79). Layouter restringe
    // body output em mask. Quando clip=false, comportamento
    // inline original preservado (semantic adiada graded mas
    // estrutura inline mantida).
    let radius_is_zero =
        radius.top_left == crate::entities::layout_types::Length::ZERO
     && radius.top_right == crate::entities::layout_types::Length::ZERO
     && radius.bottom_right == crate::entities::layout_types::Length::ZERO
     && radius.bottom_left == crate::entities::layout_types::Length::ZERO;

    if *clip {
        // P242 wrap em Group via snapshot-and-extract.
        // Layout body normalmente; snapshots de items count
        // antes/depois; extrai items emitidos pelo body e
        // re-emite como Group com clip_mask.
        let pos_block   = crate::entities::layout_types::Point {
            x: layouter.regions.current.line_start_x,
            y: layouter.regions.current.cursor_y,
        };
        let items_before = layouter.regions.current.current_items.len();
        let y_before     = layouter.regions.current.cursor_y;

        // P273.6 — save/restore parent_bbox (Decisão 3γ.2.γ:
        // popular apenas quando width+height literais).
        let saved_parent_bbox = layouter.parent_bbox;
        if let (Some(w), Some(h)) = (width, height) {
            let w_pt = w.resolve_pt(font);
            let h_pt = h.resolve_pt(font);
            layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
                x: layouter.regions.current.cursor_x,
                y: layouter.regions.current.cursor_y,
                w: Pt(w_pt),
                h: Pt(h_pt),
            });
        }

        // Layout body (acumula items em current_items).
        layouter.layout_content(body);
        layouter.flush_line();

        // P273.6 — restore parent_bbox (LIFO).
        layouter.parent_bbox = saved_parent_bbox;

        // Extrair items adicionados pelo body.
        let body_items: Vec<FrameItem> = layouter.regions.current
            .current_items.drain(items_before..).collect();
        let inner_h  = (layouter.regions.current.cursor_y - y_before).0;
        let inner_w  = layouter.available_width();
        let clip_shape = if radius_is_zero {
            crate::entities::geometry::ShapeKind::Rect
        } else {
            crate::entities::geometry::ShapeKind::RoundedRect {
                radii: *radius,
            }
        };
        // Re-emit como Group envolvendo os items extraídos.
        layouter.regions.current.current_items.push(FrameItem::Group {
            pos:          pos_block,
            matrix:       crate::entities::layout_types::TransformMatrix::identity(),
            clip_mask:    Some(clip_shape),
            inner_width:  inner_w,
            inner_height: inner_h,
            items:        body_items,
        });
    } else {
        // P273.6 — save/restore parent_bbox (Decisão 3γ.2.γ).
        let saved_parent_bbox = layouter.parent_bbox;
        if let (Some(w), Some(h)) = (width, height) {
            let w_pt = w.resolve_pt(font);
            let h_pt = h.resolve_pt(font);
            layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
                x: layouter.regions.current.cursor_x,
                y: layouter.regions.current.cursor_y,
                w: Pt(w_pt),
                h: Pt(h_pt),
            });
        }

        // 5. Layout do body (caminho original inline).
        layouter.layout_content(body);
        layouter.flush_line();

        // P273.6 — restore parent_bbox (LIFO).
        layouter.parent_bbox = saved_parent_bbox;
    }

    // 6. Aplica inset bottom.
    layouter.regions.current.cursor_y += Pt(inset_bottom);

    // 7. Se height: Some(h), garantir que avançámos pelo menos h.
    //    h é altura INTERNA do bloco (sem outset); por isso
    //    compara com `cursor_y - start_y - outset_top` que é a
    //    altura consumida desde o topo interno.
    if let Some(h) = height {
        let h_pt = h.resolve_pt(font);
        let consumed_inner = layouter.regions.current.cursor_y.0 - start_y - outset_top;
        if consumed_inner < h_pt {
            layouter.regions.current.cursor_y += Pt(h_pt - consumed_inner);
        }
    }

    // P247 — outset.bottom avança cursor após inset.bottom +
    // height min (margem externa fecha o bloco).
    layouter.regions.current.cursor_y += Pt(outset_bottom);

    // P247 — emissão FrameItem::Shape (fill/stroke/outset).
    // Z-order: insere ANTES dos items do body (índice
    // items_before) para que fill+stroke renderizem por baixo
    // do conteúdo. Bounds outer incluem outset+inset+body.
    if has_shape || has_outset {
        let block_outer_w = match width {
            Some(w) => w.resolve_pt(font) + inset_left,
            None    => saved_width - saved_line_start.0,
        };
        let mut outer_w = block_outer_w + outset_left + outset_right;
        let mut outer_h = layouter.regions.current.cursor_y.0 - start_y;
        let mut pos = crate::entities::layout_types::Point {
            x: saved_line_start - Pt(outset_left),
            y: Pt(start_y),
        };
        // P252 — stroke-overhang real activação Block.
        // Bounds Shape expandidos por thickness/2 em todos
        // os lados quando `stroke.overhang == true` (paridade
        // vanilla). Default Rust construtor `overhang: false`
        // preserva bounds literais (backward compat literal
        // pré-P252; sentinela p252).
        if let Some(ref s) = stroke {
            if s.overhang {
                let ov = s.thickness / 2.0;
                pos.x = pos.x - Pt(ov);
                pos.y = pos.y - Pt(ov);
                outer_w += 2.0 * ov;
                outer_h += 2.0 * ov;
            }
        }
        let radius_is_zero_p247 =
            radius.top_left == crate::entities::layout_types::Length::ZERO
         && radius.top_right == crate::entities::layout_types::Length::ZERO
         && radius.bottom_right == crate::entities::layout_types::Length::ZERO
         && radius.bottom_left == crate::entities::layout_types::Length::ZERO;
        let shape_kind = if radius_is_zero_p247 {
            crate::entities::geometry::ShapeKind::Rect
        } else {
            crate::entities::geometry::ShapeKind::RoundedRect {
                radii: *radius,
            }
        };
        layouter.regions.current.current_items.insert(items_before, FrameItem::Shape {
            pos,
            kind:   shape_kind,
            width:  outer_w,
            height: outer_h,
            fill:   *fill,
            stroke: stroke.clone(),
            // P273.6 — Block's own shape; gradient relative=parent
            // resolve para contentor outer (saved_parent_bbox foi
            // restaurado em parent_bbox antes desta emissão).
            parent_bbox_at_emit: layouter.parent_bbox,
        });
    }

    // 8. Restaura line_start_x e width (P243).
    layouter.regions.current.line_start_x = saved_line_start;
    layouter.regions.current.cursor_x     = saved_line_start;
    layouter.regions.current.width        = saved_width;

    // P250 — below cursor.y advance + state update para
    // collapse com próximo Block consecutivo.
    let below_pt = below
        .or(*spacing)
        .map(|l| l.resolve_pt(font))
        .unwrap_or(0.0);
    layouter.regions.current.cursor_y += Pt(below_pt);
    layouter.prev_block_below_pending = below_pt;
    layouter.block_chain_active       = true;
}
