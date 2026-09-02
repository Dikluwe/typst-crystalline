//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/helpers.md
//! @prompt-hash ecaf33a5
//! @layer L1
//! @updated 2026-04-23
//!
//! Helpers livres usados pelo Layouter: manipulação de `FrameItem`,
//! medição aproximada de `Content`, recolha de items para sub-frames.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    content::Content,
    geometry::ShapeKind,
    layout_types::{FrameItem, Point, Pt},
};

/// Extrai a posição primária de um FrameItem (posição do canto superior esquerdo).
pub(crate) fn item_pos(item: &FrameItem) -> (f64, f64) {
    match item {
        FrameItem::Text { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::TextShaped { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Line { start, .. } => (start.x.0, start.y.0),
        FrameItem::Glyph { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Image { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Shape { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Group { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Link { .. } => (0.0, 0.0),
        FrameItem::Semantic { items, .. } => {
            let Some(first) = items.first() else {
                return (0.0, 0.0);
            };
            let (_, first_y) = item_pos(first);
            let left = items
                .iter()
                .map(|child| item_pos(child).0)
                .fold(f64::INFINITY, f64::min);
            (if left.is_finite() { left } else { 0.0 }, first_y)
        }
    }
}

/// **P592/P593** — largura horizontal de um FrameItem para cálculo da
/// extensão real da linha. Usado por `align_current_line_rtl` e
/// `reorder_bidi_line` para ignorar espaços finais (cursor_x pode incluir
/// avanço de `Content::Space` sem item real).
pub(crate) fn item_width(item: &FrameItem, metrics: &dyn super::FontMetrics) -> f64 {
    match item {
        // **P593** — delegar para `FontMetrics::text_width`, a fonte única do
        // nível palavra (shaping + tracking).
        FrameItem::Text { text, style, .. } => {
            if style.math {
                metrics.advance(text.as_str(), style.size, style).0
            } else {
                metrics.text_width(text.as_str(), style.size, style).0
            }
        }
        FrameItem::TextShaped { glyphs, style, units_per_em, .. } => {
            let size_pt = style.size.val();
            let upem = *units_per_em as f64;
            glyphs.iter().map(|g| g.x_advance as f64 / upem * size_pt).sum()
        }
        FrameItem::Line { start, end, .. } => (end.x.0 - start.x.0).abs(),
        FrameItem::Glyph { x_advance, .. } => x_advance.0,
        FrameItem::Image { width, .. } => width.0,
        FrameItem::Shape { width, .. } => *width,
        FrameItem::Group { inner_width, .. } => *inner_width,
        FrameItem::Link { size, .. } => size.width.0,
        FrameItem::Semantic { items, .. } => {
            let left = items
                .iter()
                .map(|item| item_pos(item).0)
                .fold(f64::INFINITY, f64::min);
            if !left.is_finite() {
                0.0
            } else {
                line_content_right(items, metrics) - left
            }
        }
    }
}

/// **P593** — limite direito real de uma linha, calculado a partir das
/// bounding boxes dos items desenhados. Usado em vez de `cursor_x` para que
/// espaços finais (sem item visual) não afectem o alinhamento RTL.
pub(super) fn line_content_right<'a>(
    items: impl IntoIterator<Item = &'a FrameItem>,
    metrics: &dyn super::FontMetrics,
) -> f64 {
    items
        .into_iter()
        .map(|item| {
            let (x, _) = item_pos(item);
            x + item_width(item, metrics)
        })
        .fold(0.0, f64::max)
}

/// **P1096** — limite inferior real de tinta do conteúdo de uma página/linha,
/// calculado a partir das bounding boxes verticais dos items desenhados.
pub(super) fn line_content_bottom<'a>(
    items: impl IntoIterator<Item = &'a FrameItem>,
    metrics: &dyn super::FontMetrics,
) -> f64 {
    items
        .into_iter()
        .map(|item| match item {
            FrameItem::Text { pos, text, style } => {
                let (_, ink_down) = metrics.text_ink_bounds(text, style.size, style);
                pos.y.val() + ink_down.val()
            }
            FrameItem::TextShaped { pos, text, style, .. } => {
                let (_, ink_down) = metrics.text_ink_bounds(text, style.size, style);
                pos.y.val() + ink_down.val()
            }
            FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
                let (_, ink_down) = metrics.glyph_ink_bounds(*glyph_id, *size, style);
                pos.y.val() + ink_down.val()
            }
            FrameItem::Line { start, end, thickness, .. } => {
                start.y.val().max(end.y.val()) + thickness / 2.0
            }
            FrameItem::Shape { pos, height, .. } => pos.y.val() + height,
            FrameItem::Group { pos, inner_height, .. } => pos.y.val() + inner_height,
            FrameItem::Image { pos, height, .. } => pos.y.val() + height.val(),
            FrameItem::Link { .. } => 0.0,
            FrameItem::Semantic { items, .. } => line_content_bottom(items, metrics),
        })
        .fold(0.0, f64::max)
}

/// Cria um FrameItem com a posição substituída por `(new_x, new_y)`.
pub(super) fn translate_frame_item(item: FrameItem, new_x: Pt, new_y: Pt) -> FrameItem {
    match item {
        FrameItem::Text { text, style, .. } => {
            FrameItem::Text { pos: Point { x: new_x, y: new_y }, text, style }
        }
        FrameItem::TextShaped { glyphs, style, text, units_per_em, .. } => {
            FrameItem::TextShaped {
                pos: Point { x: new_x, y: new_y },
                glyphs,
                style,
                text,
                units_per_em,
            }
        }
        FrameItem::Line { start, end, thickness, color } => {
            let dx = end.x.0 - start.x.0;
            let dy = end.y.0 - start.y.0;
            FrameItem::Line {
                start: Point { x: new_x, y: new_y },
                end: Point { x: Pt(new_x.0 + dx), y: Pt(new_y.0 + dy) },
                thickness,
                // P285: translate preserva cor.
                color,
            }
        }
        FrameItem::Glyph { glyph_id, x_advance, size, style, base_char, .. } => {
            FrameItem::Glyph {
                pos: Point { x: new_x, y: new_y },
                glyph_id,
                x_advance,
                size,
                style,
                base_char,
            }
        }
        FrameItem::Image {
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            orientation,
            ..
        } => FrameItem::Image {
            pos: Point { x: new_x, y: new_y },
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            clip_rect: None,
            orientation,
        },
        FrameItem::Shape {
            kind,
            width,
            height,
            fill,
            stroke,
            fill_rule,
            parent_bbox_at_emit,
            ..
        } => FrameItem::Shape {
            pos: Point { x: new_x, y: new_y },
            kind,
            width,
            height,
            fill,
            stroke,
            fill_rule,
            parent_bbox_at_emit,
        },
        FrameItem::Group {
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
            ..
        } => FrameItem::Group {
            pos: Point { x: new_x, y: new_y },
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        },
        FrameItem::Link { target, items, pos, size } => {
            let items = items
                .into_iter()
                .map(|child| {
                    let (ix, iy) = item_pos(&child);
                    translate_frame_item(child, Pt(new_x.0 + ix), Pt(new_y.0 + iy))
                })
                .collect();
            FrameItem::Link {
                target,
                items,
                pos: Point { x: Pt(new_x.0 + pos.x.0), y: Pt(new_y.0 + pos.y.0) },
                size,
            }
        }
        FrameItem::Semantic { kind, placement, alt, mut items } => {
            let (old_x, old_y) =
                items.first().map(item_pos).unwrap_or((new_x.0, new_y.0));
            for child in &mut items {
                offset_frame_item(child, new_x.0 - old_x, new_y.0 - old_y);
            }
            FrameItem::Semantic { kind, placement, alt, items }
        }
    }
}

/// **P896** — desloca a coordenada x de um `FrameItem` **in-place** por
/// `dx` (mantém y). Usado para corrigir a posição de equações de bloco
/// centradas sob `width: auto`, depois da largura final da página ser
/// conhecida (`Layouter::apply_pending_equation_fixups`) — os items já
/// existem no `Vec` da página, só a posição x precisa de ajuste, não uma
/// reconstrução via `translate_frame_item` (que exige mover o item por
/// valor e recalcular y também).
pub(super) fn shift_frame_item_x(item: &mut FrameItem, dx: f64) {
    match item {
        FrameItem::Text { pos, .. } => pos.x = Pt(pos.x.0 + dx),
        FrameItem::TextShaped { pos, .. } => pos.x = Pt(pos.x.0 + dx),
        FrameItem::Line { start, end, .. } => {
            start.x = Pt(start.x.0 + dx);
            end.x = Pt(end.x.0 + dx);
        }
        FrameItem::Glyph { pos, .. } => pos.x = Pt(pos.x.0 + dx),
        FrameItem::Image { pos, .. } => pos.x = Pt(pos.x.0 + dx),
        FrameItem::Shape { pos, .. } => pos.x = Pt(pos.x.0 + dx),
        FrameItem::Group { pos, .. } => pos.x = Pt(pos.x.0 + dx),
        FrameItem::Link { pos, items, .. } => {
            pos.x = Pt(pos.x.0 + dx);
            for child in items {
                shift_frame_item_x(child, dx);
            }
        }
        FrameItem::Semantic { items, .. } => {
            for child in items {
                shift_frame_item_x(child, dx);
            }
        }
    }
}

/// **P898** — simétrico de `shift_frame_item_x`, eixo Y. Usado por
/// `apply_pending_align_v_fixups` (`mod.rs`) para corrigir, in-place, a
/// posição vertical de items posicionados com um fallback provisório
/// (`available_height()`/`page_bottom_limit()` infinitos sob `height:
/// auto`), depois de a altura final da página ser conhecida.
pub(super) fn shift_frame_item_y(item: &mut FrameItem, dy: f64) {
    match item {
        FrameItem::Text { pos, .. } => pos.y = Pt(pos.y.0 + dy),
        FrameItem::TextShaped { pos, .. } => pos.y = Pt(pos.y.0 + dy),
        FrameItem::Line { start, end, .. } => {
            start.y = Pt(start.y.0 + dy);
            end.y = Pt(end.y.0 + dy);
        }
        FrameItem::Glyph { pos, .. } => pos.y = Pt(pos.y.0 + dy),
        FrameItem::Image { pos, .. } => pos.y = Pt(pos.y.0 + dy),
        FrameItem::Shape { pos, .. } => pos.y = Pt(pos.y.0 + dy),
        FrameItem::Group { pos, .. } => pos.y = Pt(pos.y.0 + dy),
        FrameItem::Link { pos, items, .. } => {
            pos.y = Pt(pos.y.0 + dy);
            for child in items {
                shift_frame_item_y(child, dy);
            }
        }
        FrameItem::Semantic { items, .. } => {
            for child in items {
                shift_frame_item_y(child, dy);
            }
        }
    }
}

/// **P908** — resolve um `path: &[usize]` (entrada `pending_align_*`) numa
/// slice mutável de `FrameItem`s. Todos os elementos de `path` excepto o
/// último descem por `FrameItem::Group.items`/`Link.items` sucessivamente;
/// o último é o índice inicial (`start_idx` efectivo) na lista alcançada.
/// `None` quando o `path` já não é resolúvel (item removido/substituído
/// entretanto, ex: `DeferredCellTail` descartado ao fim de 3 forwardings,
/// P251) — degradação graciosa, a entrada correspondente é simplesmente
/// ignorada por `apply_pending_align_(v_)fixups`, mesmo princípio dos
/// outros mecanismos `pending_*`.
pub(super) fn resolve_path_slice<'a>(
    items: &'a mut [FrameItem],
    path: &[usize],
    count: usize,
) -> Option<&'a mut [FrameItem]> {
    match path {
        [] => None,
        [idx] => items.get_mut(*idx..idx.checked_add(count)?),
        [idx, rest @ ..] => match items.get_mut(*idx)? {
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } => resolve_path_slice(items, rest, count),
            _ => None,
        },
    }
}

/// **P978** — factores do vanilla
/// (`lab/typst-original/crates/typst-library/src/model/heading.rs:281-285`:
/// nível 1 → 1.4em, nível 2 → 1.2em, nível 3+ → 1.0em). Antes: escala
/// tipo HTML (2.0/1.667/1.333/1.167) — medida errada contra o vanilla
/// (15.4/13.2/11.0pt sobre corpo de 11pt). Ver `compiler/layout/heading.md`
/// §P978.
pub(super) fn heading_scale(level: u8) -> f64 {
    match level {
        1 => 1.4,
        2 => 1.2,
        _ => 1.0,
    }
}

/// Extrai o valor em pontos de um `Option<&Value>`, com fallback.
///
/// Suporta `Value::Length` (abs em pt), `Value::Float`, `Value::Int`.
/// `Value::Auto` e `None` → `fallback`.
pub(super) fn resolve_pt(
    val: Option<&crate::entities::value::Value>,
    fallback: f64,
) -> f64 {
    use crate::entities::value::Value;
    match val {
        None => fallback, // neutro: N16[γ] — Value::Auto e None resolvem para fallback em dimensões (fallback aberto)
        Some(Value::Length(l)) => l.abs.to_pt(),
        Some(Value::Float(f)) => *f,
        Some(Value::Int(i)) => *i as f64,
        Some(Value::Auto) => fallback,
        Some(_) => fallback,
    }
}

/// Estima as dimensões (width, height) de conteúdo sem correr o layouter completo.
///
/// Suficiente para calcular a AABB de `Content::Transform`. Para conteúdo complexo
/// (texto multi-linha, equações), retorna (0, 0) como approximação conservadora.
///
/// Visibility promovida a `pub(crate)` em P222 para uso por `native_measure`
/// stdlib (Fase 4 Layout candidata; ADR-0066 §"Plano promoção" Bloco C
/// cross-módulo primeira materialização parcial).
pub(crate) fn measure_content(content: &Content, available_w: f64) -> (f64, f64) {
    match content {
        Content::Shape(e) => {
            let (kind, width, height) = (&e.kind, &e.width, &e.height);
            match kind {
                // P242 — RoundedRect partilha dimensões com Rect/Ellipse/Path
                // (radii não afecta bounding box per ADR-0054 graded).
                ShapeKind::Rect
                | ShapeKind::RoundedRect { .. }
                | ShapeKind::Ellipse
                | ShapeKind::Path(_) => (
                    resolve_pt(width.as_deref(), available_w),
                    resolve_pt(height.as_deref(), 0.0),
                ),
                ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
            }
        }
        Content::Sequence(seq) => {
            let mut max_w = 0.0_f64;
            let mut total_h = 0.0_f64;
            for part in seq.iter() {
                let (w, h) = measure_content(part, available_w);
                max_w = max_w.max(w);
                total_h += h;
            }
            (max_w, total_h)
        }
        _ => (0.0, 0.0), // neutro: N16[γ] — Content sem dimensão mensurável retorna (0.0, 0.0) (fallback de layout aberto)
    }
}

/// Desloca a posição de um `FrameItem` por `(dx, dy)`.
pub(super) fn offset_frame_item(item: &mut FrameItem, dx: f64, dy: f64) {
    match item {
        FrameItem::Text { pos, .. }
        | FrameItem::TextShaped { pos, .. }
        | FrameItem::Glyph { pos, .. }
        | FrameItem::Shape { pos, .. }
        | FrameItem::Image { pos, .. }
        | FrameItem::Group { pos, .. } => {
            pos.x = Pt(pos.x.0 + dx);
            pos.y = Pt(pos.y.0 + dy);
        }
        FrameItem::Line { start, end, .. } => {
            start.x = Pt(start.x.0 + dx);
            start.y = Pt(start.y.0 + dy);
            end.x = Pt(end.x.0 + dx);
            end.y = Pt(end.y.0 + dy);
        }
        FrameItem::Link { .. } => {}
        FrameItem::Semantic { items, .. } => {
            for child in items {
                offset_frame_item(child, dx, dy);
            }
        }
    }
}

/// Extrai o limite inferior (bottom Y) de um FrameItem.
pub(crate) fn item_bottom_y(item: &FrameItem) -> f64 {
    match item {
        FrameItem::Text { pos, text, style } => {
            let has_descender = text.chars().any(|ch| "gjpqy,".contains(ch));
            pos.y.0 + if has_descender { style.size.val() * 0.25 } else { 0.0 }
        }
        FrameItem::TextShaped { pos, text, style, .. } => {
            let has_descender = text.chars().any(|ch| "gjpqy,".contains(ch));
            pos.y.0 + if has_descender { style.size.val() * 0.25 } else { 0.0 }
        }
        FrameItem::Line { start, end, .. } => start.y.0.max(end.y.0),
        FrameItem::Glyph { pos, size, base_char, .. } => {
            let has_descender = "gjpqy,".contains(*base_char);
            pos.y.0 + if has_descender { size.val() * 0.25 } else { 0.0 }
        }
        FrameItem::Image { pos, height, .. } => pos.y.0 + height.0,
        FrameItem::Shape { pos, height, .. } => pos.y.0 + *height,
        FrameItem::Group { pos, inner_height, items, .. } => {
            let mut max_y = pos.y.0 + *inner_height;
            for child in items {
                max_y = max_y.max(pos.y.0 + item_bottom_y(child));
            }
            max_y
        }
        FrameItem::Link { pos, size, .. } => pos.y.0 + size.height.0,
        FrameItem::Semantic { items, .. } => {
            items.iter().map(item_bottom_y).fold(0.0, f64::max)
        }
    }
}

#[cfg(test)]
mod p1293_attach_refutator_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::layout_types::{SemanticKind, SemanticPlacement, TextStyle};

    fn glyph(id: u16, x: f64, y: f64, width: f64) -> FrameItem {
        FrameItem::Glyph {
            pos: Point { x: Pt(x), y: Pt(y) },
            glyph_id: id,
            x_advance: Pt(width),
            size: Pt(12.0),
            style: TextStyle::regular(Pt(12.0)),
            base_char: 'x',
        }
    }

    #[test]
    fn p1293_semantic_mede_posicao_e_largura_no_mesmo_referencial() {
        let semantic = FrameItem::Semantic {
            kind: SemanticKind::Formula,
            placement: SemanticPlacement::Inline,
            alt: None,
            // O primeiro filho não é o mais à esquerda. A ordem é parte da
            // contraprova: medir não pode reordenar nem transladar filhos.
            items: vec![glyph(10, 10.0, 7.0, 4.0), glyph(20, 2.0, 11.0, 3.0)],
        };

        assert_eq!(item_pos(&semantic), (2.0, 7.0));
        assert_eq!(item_width(&semantic, &FixedMetrics), 12.0);
        assert_eq!(line_content_right([&semantic], &FixedMetrics), 14.0);

        let FrameItem::Semantic { items, .. } = semantic else { unreachable!() };
        assert!(matches!(items[0], FrameItem::Glyph { glyph_id: 10, .. }));
        assert!(matches!(items[1], FrameItem::Glyph { glyph_id: 20, .. }));
        assert_eq!(item_pos(&items[0]), (10.0, 7.0));
        assert_eq!(item_pos(&items[1]), (2.0, 11.0));
    }

    #[test]
    fn p1293_semantic_vazio_conserva_posicao_e_largura_zero() {
        let semantic = FrameItem::Semantic {
            kind: SemanticKind::Formula,
            placement: SemanticPlacement::Inline,
            alt: None,
            items: vec![],
        };
        assert_eq!(item_pos(&semantic), (0.0, 0.0));
        assert_eq!(item_width(&semantic, &FixedMetrics), 0.0);
    }
}
