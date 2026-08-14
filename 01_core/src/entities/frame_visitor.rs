//! Crystalline Lineage
//! @layer L1
//! @updated 2026-08-14

use crate::entities::layout_types::FrameItem;

/// Trait unificador para percurso recursivo de árvores de `FrameItem`.
///
/// Centraliza a travessia de `Group` e `Link` e define um ponto único de fallback
/// com `debug_assert!` para capturar variantes novas em tempo de debug/teste sem
/// custo em release.
pub trait FrameVisitor {
    /// Ponto de entrada padrão para visitar um `FrameItem`.
    fn visit_item(&mut self, item: &FrameItem) {
        match item {
            FrameItem::Text { .. } => self.visit_text(item),
            FrameItem::TextShaped { .. } => self.visit_text_shaped(item),
            FrameItem::Line { .. } => self.visit_line(item),
            FrameItem::Glyph { .. } => self.visit_glyph(item),
            FrameItem::Image { .. } => self.visit_image(item),
            FrameItem::Shape { .. } => self.visit_shape(item),
            FrameItem::Group { items, .. } => {
                self.visit_group(item);
                for child in items {
                    self.visit_item(child);
                }
            }
            FrameItem::Link { items, .. } => {
                self.visit_link(item);
                for child in items {
                    self.visit_item(child);
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                debug_assert!(false, "FrameItem não tratado: {:?}", item);
            }
        }
    }

    /// Visita uma fatia de `FrameItem` sequencialmente.
    fn visit_items(&mut self, items: &[FrameItem]) {
        for item in items {
            self.visit_item(item);
        }
    }

    fn visit_text(&mut self, _item: &FrameItem) {}
    fn visit_text_shaped(&mut self, _item: &FrameItem) {}
    fn visit_line(&mut self, _item: &FrameItem) {}
    fn visit_glyph(&mut self, _item: &FrameItem) {}
    fn visit_image(&mut self, _item: &FrameItem) {}
    fn visit_shape(&mut self, _item: &FrameItem) {}
    fn visit_group(&mut self, _item: &FrameItem) {}
    fn visit_link(&mut self, _item: &FrameItem) {}
}

/// Helper para invocar um `FrameVisitor` sobre uma lista de `FrameItem`.
pub fn walk_frame_items<V: FrameVisitor>(visitor: &mut V, items: &[FrameItem]) {
    visitor.visit_items(items);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::{FrameItem, Point, TextStyle, TransformMatrix};

    struct DummyVisitor {
        visited_text: bool,
    }
    impl FrameVisitor for DummyVisitor {
        fn visit_text(&mut self, _item: &FrameItem) {
            self.visited_text = true;
        }
    }

    #[test]
    #[allow(deprecated)]
    
    fn visitor_traverses_group_and_items() {
        let mut v = DummyVisitor { visited_text: false };
        let item = FrameItem::Group {
            pos: Point::ZERO,
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 100.0,
            inner_height: 100.0,
            items: vec![FrameItem::Text {
                pos: Point::ZERO,
                text: "hello".into(),
                style: TextStyle::default(),
            }],
        };
        v.visit_item(&item);
        assert!(v.visited_text);
    }
}
