//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 325595fd
//! @layer L3
//! @updated 2026-06-27
//!
//! **P482** — Post-processing shaping pass (Trilha 5 Fase 1).
//! Converte `FrameItem::Text` → `FrameItem::TextShaped` via rustybuzz.
//! Executado entre layout e export. ADR-0120 Opção A1.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use rustybuzz::UnicodeBuffer;
use typst_core::contracts::world::World;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::{FontList, FontNamePattern};
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, ShapedGlyph};

/// Converte todos os `FrameItem::Text` de um `PagedDocument` em
/// `FrameItem::TextShaped` via rustybuzz.
///
/// Itens sem fonte resolvida (`style.font == None` ou lookup falha)
/// são preservados como `FrameItem::Text` (fallback Helvetica).
pub fn shape_document(world: &dyn World, mut doc: PagedDocument) -> PagedDocument {
    for page in &mut doc.pages {
        shape_page(world, page);
    }
    doc
}

fn shape_page(world: &dyn World, page: &mut Page) {
    for item in page.items.iter_mut() {
        shape_item(world, item);
    }
}

fn shape_item(world: &dyn World, item: &mut FrameItem) {
    match item {
        FrameItem::Text { pos, text, style } if style.font.is_some() => {
            if let Some(shaped) = try_shape(world, pos, text, style) {
                *item = shaped;
            }
        }
        FrameItem::Group { items, .. } => {
            for child in items.iter_mut() {
                shape_item(world, child);
            }
        }
        FrameItem::Link { items, .. } => {
            for child in items.iter_mut() {
                shape_item(world, child);
            }
        }
        _ => {}
    }
}

fn try_shape(
    world: &dyn World,
    pos:   &Point,
    text:  &ecow::EcoString,
    style: &typst_core::entities::layout_types::TextStyle,
) -> Option<FrameItem> {
    let font_list = style.font.as_ref()?;

    let slot_idx = resolve_slot(world, font_list)?;
    let font = world.font(slot_idx)?;
    let font_data = font.as_slice();

    let rb_face = rustybuzz::Face::from_slice(font_data, 0)?;

    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text.as_str());
    buffer.guess_segment_properties();
    let output = rustybuzz::shape(&rb_face, &[], buffer);

    let infos     = output.glyph_infos();
    let positions = output.glyph_positions();

    let glyphs: Vec<ShapedGlyph> = infos.iter().zip(positions.iter())
        .map(|(info, pos_g)| {
            let char_code = byte_idx_to_char(text.as_str(), info.cluster as usize)
                .unwrap_or('\u{FFFD}');
            ShapedGlyph {
                glyph_id:  info.glyph_id as u16,
                x_advance: pos_g.x_advance,
                x_offset:  pos_g.x_offset,
                y_offset:  pos_g.y_offset,
                cluster:   info.cluster,
                char_code,
            }
        })
        .collect();

    Some(FrameItem::TextShaped {
        pos:   *pos,
        glyphs,
        style: style.clone(),
        text:  text.clone(),
    })
}

fn resolve_slot(world: &dyn World, font_list: &FontList) -> Option<usize> {
    let variant = FontVariant::default();
    let book = world.book();
    for family in font_list.as_slice() {
        if let Some(idx) = book.select_pattern(&family.name, &variant) {
            if world.font(idx).is_some() {
                return Some(idx);
            }
        }
    }
    None
}

fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char> {
    s.get(byte_idx..)?.chars().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecow::EcoString;
    use typst_core::entities::font_book::FontBook;
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle};
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::source::Source;
    use typst_core::entities::world_types::{Bytes, Datetime, FileResult, Font, Library};

    struct MockWorld {
        book: FontBook,
    }

    impl typst_core::contracts::world::World for MockWorld {
        fn library(&self) -> &Library { static L: std::sync::OnceLock<Library> = std::sync::OnceLock::new(); L.get_or_init(Library::new) }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { unimplemented!() }
        fn source(&self, _: FileId) -> FileResult<Source> { unimplemented!() }
        fn file(&self, _: FileId) -> FileResult<Bytes> { unimplemented!() }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn empty_world() -> MockWorld { MockWorld { book: FontBook::new() } }

    fn text_item(text: &str) -> FrameItem {
        FrameItem::Text {
            pos:   Point { x: Pt(72.0), y: Pt(72.0) },
            text:  EcoString::from(text),
            style: TextStyle::default(),
        }
    }

    fn doc_with(items: Vec<FrameItem>) -> PagedDocument {
        PagedDocument::new(vec![Page { width: 595.0, height: 842.0, items }])
    }

    #[test]
    fn p482_shape_document_preserves_text_sem_font() {
        let doc = doc_with(vec![text_item("Hello")]);
        let shaped = shape_document(&empty_world(), doc);
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P482: Text sem style.font deve permanecer Text");
    }

    #[test]
    fn p482_shape_document_preserves_text_content() {
        let doc = doc_with(vec![text_item("world")]);
        let shaped = shape_document(&empty_world(), doc);
        if let FrameItem::Text { text, .. } = &shaped.pages[0].items[0] {
            assert_eq!(text.as_str(), "world");
        } else {
            panic!("esperava FrameItem::Text");
        }
    }

    #[test]
    fn p482_byte_idx_to_char_ascii() {
        assert_eq!(byte_idx_to_char("hello", 0), Some('h'));
        assert_eq!(byte_idx_to_char("hello", 1), Some('e'));
        assert_eq!(byte_idx_to_char("hello", 5), None);
    }

    #[test]
    fn p482_byte_idx_to_char_utf8() {
        let s = "héllo";
        assert_eq!(byte_idx_to_char(s, 0), Some('h'));
        assert_eq!(byte_idx_to_char(s, 1), Some('é'));
        assert_eq!(byte_idx_to_char(s, 3), Some('l'));
    }

    #[test]
    fn p482_shaped_glyph_clone_eq() {
        let g = ShapedGlyph {
            glyph_id: 42, x_advance: 600, x_offset: 0, y_offset: 0,
            cluster: 0, char_code: 'A',
        };
        assert_eq!(g.clone(), g);
    }

    // ── P483 ────────────────────────────────────────────────────────────────

    #[test]
    fn p483_text_com_font_helvetica_tenta_shape_mas_sem_fontes_preserva_text() {
        // FrameItem::Text com style.font = Some(Helvetica) → shaper tenta;
        // MockWorld não tem fontes → resolve_slot retorna None → item preservado
        // como Text. O path de tentativa é exercitado (style.font.is_some() == true).
        use typst_core::entities::font_list::FontList;
        let mut style = TextStyle::default();
        style.font = Some(FontList::single(EcoString::from("Helvetica")));
        let item = FrameItem::Text {
            pos:  Point { x: Pt(72.0), y: Pt(72.0) },
            text: EcoString::from("Ola"),
            style,
        };
        let doc = doc_with(vec![item]);
        let shaped = shape_document(&empty_world(), doc);
        // MockWorld.font() retorna None → try_shape retorna None → Text preservado
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P483: sem fontes reais, Text com font=Some preservado como fallback");
    }

    #[test]
    fn p483_text_sem_font_nao_tenta_shape() {
        // style.font = None → shaper ignora (guarda is_some() false).
        let item = text_item("sem fonte"); // text_item usa TextStyle::default() → font=None
        let doc = doc_with(vec![item]);
        let shaped = shape_document(&empty_world(), doc);
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P483: Text sem style.font=None não é tentado pelo shaper");
    }

    #[test]
    fn p483_shaped_glyph_debug_display() {
        let g = ShapedGlyph {
            glyph_id: 1, x_advance: 500, x_offset: 0, y_offset: 0,
            cluster: 0, char_code: 'A',
        };
        let s = format!("{:?}", g);
        assert!(s.contains("glyph_id: 1"), "Debug deve incluir glyph_id");
    }

    #[test]
    fn p482_shape_document_group_children_passthrough() {
        use typst_core::entities::layout_types::TransformMatrix;
        let inner = text_item("inner");
        let group = FrameItem::Group {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            matrix: TransformMatrix::default(),
            clip_mask: None,
            inner_width: 100.0,
            inner_height: 100.0,
            items: vec![inner],
        };
        let doc = doc_with(vec![group]);
        let shaped = shape_document(&empty_world(), doc);
        if let FrameItem::Group { items, .. } = &shaped.pages[0].items[0] {
            // sem font → Text preservado dentro do Group
            assert!(matches!(&items[0], FrameItem::Text { .. }));
        } else {
            panic!("esperava Group");
        }
    }
}
