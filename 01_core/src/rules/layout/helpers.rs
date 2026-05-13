//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
//! @layer L1
//! @updated 2026-04-23
//!
//! Helpers livres usados pelo Layouter: manipulação de `FrameItem`,
//! medição aproximada de `Content`, recolha de items para sub-frames.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    content::Content,
    geometry::ShapeKind,
    layout_types::{FrameItem, Point, Pt},
};

/// Extrai a posição primária de um FrameItem (posição do canto superior esquerdo).
pub(super) fn item_pos(item: &FrameItem) -> (f64, f64) {
    match item {
        FrameItem::Text  { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Line  { start, .. } => (start.x.0, start.y.0),
        FrameItem::Glyph { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Image { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Shape { pos, .. } => (pos.x.0, pos.y.0),
        FrameItem::Group { pos, .. } => (pos.x.0, pos.y.0),
    }
}

/// Cria um FrameItem com a posição substituída por `(new_x, new_y)`.
pub(super) fn translate_frame_item(item: FrameItem, new_x: Pt, new_y: Pt) -> FrameItem {
    match item {
        FrameItem::Text { text, style, .. } =>
            FrameItem::Text { pos: Point { x: new_x, y: new_y }, text, style },
        FrameItem::Line { start, end, thickness } => {
            let dx = end.x.0 - start.x.0;
            let dy = end.y.0 - start.y.0;
            FrameItem::Line {
                start:     Point { x: new_x, y: new_y },
                end:       Point { x: Pt(new_x.0 + dx), y: Pt(new_y.0 + dy) },
                thickness,
            }
        }
        FrameItem::Glyph { glyph_id, x_advance, size, .. } =>
            FrameItem::Glyph { pos: Point { x: new_x, y: new_y }, glyph_id, x_advance, size },
        FrameItem::Image { data, width, height, intrinsic_width, intrinsic_height, .. } =>
            FrameItem::Image { pos: Point { x: new_x, y: new_y }, data, width, height, intrinsic_width, intrinsic_height },
        FrameItem::Shape { kind, width, height, fill, stroke, .. } =>
            FrameItem::Shape { pos: Point { x: new_x, y: new_y }, kind, width, height, fill, stroke },
        FrameItem::Group { matrix, clip_mask, inner_width, inner_height, items, .. } =>
            FrameItem::Group { pos: Point { x: new_x, y: new_y }, matrix, clip_mask, inner_width, inner_height, items },
    }
}

pub(super) fn heading_scale(level: u8) -> f64 {
    match level { 1 => 2.0, 2 => 1.667, 3 => 1.333, 4 => 1.167, _ => 1.0 }
}

/// Extrai o valor em pontos de um `Option<&Value>`, com fallback.
///
/// Suporta `Value::Length` (abs em pt), `Value::Float`, `Value::Int`.
/// `Value::Auto` e `None` → `fallback`.
pub(super) fn resolve_pt(val: Option<&crate::entities::value::Value>, fallback: f64) -> f64 {
    use crate::entities::value::Value;
    match val {
        None => fallback,
        Some(Value::Length(l)) => l.abs.to_pt(),
        Some(Value::Float(f))  => *f,
        Some(Value::Int(i))    => *i as f64,
        Some(Value::Auto)      => fallback,
        Some(_)                => fallback,
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
        Content::Shape { kind, width, height, .. } => {
            match kind {
                ShapeKind::Rect | ShapeKind::Ellipse | ShapeKind::Path(_) => (
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
        _ => (0.0, 0.0),
    }
}

/// Recolhe `FrameItem`s do conteúdo em coordenadas locais (Y-down, origem (0,0)).
///
/// Usado por `Content::Transform` para construir `FrameItem::Group.items`.
pub(super) fn collect_sub_items(content: &Content, available_w: f64) -> Vec<FrameItem> {
    let mut items = Vec::new();
    collect_items_at(content, &mut items, Pt(0.0), Pt(0.0), available_w);
    items
}

fn collect_items_at(content: &Content, items: &mut Vec<FrameItem>, x: Pt, y: Pt, available_w: f64) {
    match content {
        Content::Shape { kind, width, height, fill, stroke } => {
            let (w, h) = match kind {
                ShapeKind::Rect | ShapeKind::Ellipse | ShapeKind::Path(_) => (
                    resolve_pt(width.as_deref(), available_w),
                    resolve_pt(height.as_deref(), 0.0),
                ),
                ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
            };
            items.push(FrameItem::Shape {
                pos: Point { x, y },
                kind: kind.clone(),
                width: w,
                height: h,
                fill: *fill,
                stroke: stroke.clone(),
            });
        }
        Content::Sequence(seq) => {
            for part in seq.iter() {
                collect_items_at(part, items, x, y, available_w);
            }
        }
        _ => {}
    }
}
