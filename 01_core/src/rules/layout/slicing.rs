//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
//! @layer L1
//! @updated 2026-05-14
//!
//! **P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita ADR-0082
//! PROPOSTO N=2 segunda aplicação citante)** — slicing de
//! `Vec<FrameItem>` por `pos.y` threshold para row break TableCell
//! cell-level. **γ-Items** (não γ-Content reconstruction).
//!
//! Limitações conscientes (per ADR-0054 graded; documentadas em
//! relatório P251):
//!
//! 1. Items atómicos (Group com `inner_height` grande; Shape com
//!    height grande) que começam abaixo de threshold vão **completos
//!    para tail** — não slice mid-item (paridade vanilla "atomic
//!    block can't split mid-paragraph").
//! 2. Tail items são rebased por `pos.y -= threshold` (movido para
//!    topo da próxima região).
//! 3. `FrameItem::Line` usa `start`/`end` (não `pos`); a decisão
//!    head vs tail é via `start.y`. Ambos start.y e end.y são
//!    rebased simétricamente para preservar geometria da linha.

use crate::entities::layout_types::{FrameItem, Point, Pt};

/// **P251** — divide items em `(head, tail)` por `threshold` em
/// `pos.y`. Items que **começam** abaixo (ou em) threshold vão para
/// tail (rebased: `pos.y -= threshold`); items acima vão para head.
///
/// Items que internamente cruzam threshold (ex: Group com
/// `pos.y + inner_height > threshold`) vão **completos para tail**
/// — não slice mid-item (paridade vanilla atómico).
///
/// **Função pura** (sem `&self`); reusável noutros contextos de
/// pagination overflow no futuro (column flow DEBT-56; pagination
/// generic).
pub(super) fn slice_frame_items_at_height(
    items: Vec<FrameItem>,
    threshold: f64,
) -> (Vec<FrameItem>, Vec<FrameItem>) {
    let mut head = Vec::with_capacity(items.len());
    let mut tail = Vec::new();
    for item in items {
        let y = item_y_start(&item);
        if y >= threshold {
            tail.push(rebase_item_y(item, -threshold));
        } else {
            head.push(item);
        }
    }
    (head, tail)
}

/// **P251** — y-start (topo) de um `FrameItem`. Para `Line`, é
/// `start.y`; para outros variants, é `pos.y`.
fn item_y_start(item: &FrameItem) -> f64 {
    match item {
        FrameItem::Text  { pos, .. } => pos.y.0,
        FrameItem::Line  { start, .. } => start.y.0,
        FrameItem::Glyph { pos, .. } => pos.y.0,
        FrameItem::Image { pos, .. } => pos.y.0,
        FrameItem::Shape { pos, .. } => pos.y.0,
        FrameItem::Group { pos, .. } => pos.y.0,
    }
}

/// **P251** — rebase `pos.y` de um `FrameItem` por `delta` aditivo.
/// `Line` rebase ambos `start.y` e `end.y` simétricamente para
/// preservar geometria.
///
/// `Group.items` **não** são recursivamente rebased — items locais
/// permanecem relativos à origem do Group (paridade comentário
/// `layout_types.rs`: "itens em espaço local").
pub(super) fn rebase_item_y(item: FrameItem, delta: f64) -> FrameItem {
    match item {
        FrameItem::Text { pos, text, style } =>
            FrameItem::Text {
                pos: Point { x: pos.x, y: Pt(pos.y.0 + delta) },
                text, style,
            },
        FrameItem::Line { start, end, thickness } =>
            FrameItem::Line {
                start: Point { x: start.x, y: Pt(start.y.0 + delta) },
                end:   Point { x: end.x,   y: Pt(end.y.0   + delta) },
                thickness,
            },
        FrameItem::Glyph { pos, glyph_id, x_advance, size } =>
            FrameItem::Glyph {
                pos: Point { x: pos.x, y: Pt(pos.y.0 + delta) },
                glyph_id, x_advance, size,
            },
        FrameItem::Image { pos, data, width, height, intrinsic_width, intrinsic_height } =>
            FrameItem::Image {
                pos: Point { x: pos.x, y: Pt(pos.y.0 + delta) },
                data, width, height, intrinsic_width, intrinsic_height,
            },
        FrameItem::Shape { pos, kind, width, height, fill, stroke } =>
            FrameItem::Shape {
                pos: Point { x: pos.x, y: Pt(pos.y.0 + delta) },
                kind, width, height, fill, stroke,
            },
        FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } =>
            FrameItem::Group {
                pos: Point { x: pos.x, y: Pt(pos.y.0 + delta) },
                matrix, clip_mask, inner_width, inner_height, items,
            },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::TextStyle;
    use ecow::EcoString;

    fn mk_text(y: f64) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(y) },
            text: EcoString::from("t"),
            style: TextStyle::default(),
        }
    }

    fn mk_shape(y: f64, h: f64) -> FrameItem {
        FrameItem::Shape {
            pos: Point { x: Pt(0.0), y: Pt(y) },
            kind: ShapeKind::Rect,
            width: 10.0, height: h,
            fill: None, stroke: None,
        }
    }

    #[test]
    fn p251_slice_items_vazio_retorna_vazio() {
        let (head, tail) = slice_frame_items_at_height(vec![], 10.0);
        assert!(head.is_empty());
        assert!(tail.is_empty());
    }

    #[test]
    fn p251_slice_items_todos_abaixo_threshold_va_head() {
        let items = vec![mk_text(5.0), mk_text(8.0), mk_text(9.99)];
        let (head, tail) = slice_frame_items_at_height(items, 10.0);
        assert_eq!(head.len(), 3, "items < threshold vão para head");
        assert!(tail.is_empty());
    }

    #[test]
    fn p251_slice_items_todos_acima_threshold_va_tail_rebased() {
        let items = vec![mk_text(10.0), mk_text(15.0), mk_text(20.0)];
        let (head, tail) = slice_frame_items_at_height(items, 10.0);
        assert!(head.is_empty());
        assert_eq!(tail.len(), 3, "items >= threshold vão para tail");
        // Rebased: pos.y -= 10.0.
        if let FrameItem::Text { pos, .. } = &tail[0] {
            assert_eq!(pos.y.0, 0.0, "rebase pos.y -= threshold");
        }
        if let FrameItem::Text { pos, .. } = &tail[2] {
            assert_eq!(pos.y.0, 10.0);
        }
    }

    #[test]
    fn p251_slice_items_mistos_split_correcto() {
        let items = vec![mk_text(3.0), mk_text(10.0), mk_text(7.0), mk_text(15.0)];
        let (head, tail) = slice_frame_items_at_height(items, 10.0);
        assert_eq!(head.len(), 2);
        assert_eq!(tail.len(), 2);
    }

    #[test]
    fn p251_slice_items_atomic_shape_grande_vai_completo_para_tail() {
        // Shape com pos.y == threshold + height grande: vai completo
        // para tail (atomic; não slice mid-item per paridade vanilla).
        let items = vec![mk_shape(10.0, 100.0)];
        let (head, tail) = slice_frame_items_at_height(items, 10.0);
        assert!(head.is_empty());
        assert_eq!(tail.len(), 1);
        if let FrameItem::Shape { pos, height, .. } = &tail[0] {
            assert_eq!(pos.y.0, 0.0, "Shape rebased pos.y = 0");
            assert_eq!(*height, 100.0, "Shape height preservado (atomic)");
        }
    }

    #[test]
    fn p251_slice_threshold_zero_tudo_tail() {
        let items = vec![mk_text(0.0), mk_text(5.0)];
        let (head, tail) = slice_frame_items_at_height(items, 0.0);
        assert!(head.is_empty());
        assert_eq!(tail.len(), 2);
    }

    #[test]
    fn p251_rebase_item_y_text() {
        let item = mk_text(15.0);
        let rebased = rebase_item_y(item, -10.0);
        if let FrameItem::Text { pos, .. } = rebased {
            assert_eq!(pos.y.0, 5.0);
        } else { panic!("esperado Text"); }
    }

    #[test]
    fn p251_rebase_item_y_line_start_e_end_simultaneos() {
        let item = FrameItem::Line {
            start:     Point { x: Pt(0.0), y: Pt(10.0) },
            end:       Point { x: Pt(50.0), y: Pt(10.0) },
            thickness: 1.0,
        };
        let rebased = rebase_item_y(item, -10.0);
        if let FrameItem::Line { start, end, .. } = rebased {
            assert_eq!(start.y.0, 0.0);
            assert_eq!(end.y.0, 0.0, "Line end.y também rebased");
        } else { panic!("esperado Line"); }
    }

    #[test]
    fn p251_rebase_item_y_shape() {
        let item = mk_shape(20.0, 50.0);
        let rebased = rebase_item_y(item, -5.0);
        if let FrameItem::Shape { pos, height, .. } = rebased {
            assert_eq!(pos.y.0, 15.0);
            assert_eq!(height, 50.0, "Shape height preservado");
        } else { panic!("esperado Shape"); }
    }

    #[test]
    fn p251_rebase_item_y_group_items_locais_preservados() {
        // Group.items locais (relativos a Group.pos) NÃO recursivamente
        // rebased — só Group.pos rebased.
        use crate::entities::layout_types::TransformMatrix;
        let inner = mk_text(3.0);
        let group = FrameItem::Group {
            pos: Point { x: Pt(0.0), y: Pt(20.0) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 10.0, inner_height: 50.0,
            items: vec![inner],
        };
        let rebased = rebase_item_y(group, -5.0);
        if let FrameItem::Group { pos, items, .. } = rebased {
            assert_eq!(pos.y.0, 15.0);
            if let FrameItem::Text { pos: inner_pos, .. } = &items[0] {
                assert_eq!(inner_pos.y.0, 3.0,
                    "inner items NÃO rebased (espaço local relativo a Group.pos)");
            }
        } else { panic!("esperado Group"); }
    }
}
