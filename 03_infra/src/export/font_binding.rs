//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/font_binding.md
//! @prompt-hash 49007449
//! @layer L3
//! @updated 2026-10-08

use super::render::FontKey;
use crate::export::tiling::walk_resources as walk_frame_items;
use crate::font_variant::{
    axis_variations_for_font_variant, instantiate_variable_font,
    merge_explicit_variations, text_style_to_font_variant,
};
use std::borrow::Cow;
use typst_core::entities::frame_visitor::FrameVisitor;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, TextStyle};
use typst_core::entities::world_types::Font;

pub(crate) fn index(
    fonts: &[FontKey],
    style: &TextStyle,
    font: Option<&Font>,
) -> Option<usize> {
    let variant = text_style_to_font_variant(style);
    let vars = style.variations.clone().unwrap_or_default();
    if let Some(font) = font {
        fonts.iter().position(|((_, v, axes), data)| {
            *v == variant && *axes == vars && data.as_slice() == font.as_slice()
        })
    } else {
        Some(super::stream::font_index_for_style(fonts, style))
    }
}

/// Local positives, validated once against this export's append-only table.
#[derive(Default)]
pub(crate) struct LookupCache(
    std::cell::RefCell<
        Vec<(
            Font,
            typst_core::entities::font_book::FontVariant,
            typst_core::entities::font_variations::FontVariations,
            usize,
        )>,
    >,
);
impl LookupCache {
    pub(crate) fn index(
        &self,
        fonts: &[FontKey],
        style: &TextStyle,
        font: Option<&Font>,
    ) -> Option<usize> {
        let Some(font) = font else { return index(fonts, style, None) };
        let variant = text_style_to_font_variant(style);
        let vars = style.variations.clone().unwrap_or_default();
        if let Some((_, _, _, i)) = self.0.borrow().iter().find(|(old, v, axes, _)| {
            std::ptr::eq(old.as_slice(), font.as_slice())
                && *v == variant
                && *axes == vars
        }) {
            return Some(*i);
        }
        let i = index(fonts, style, Some(font))?;
        self.0.borrow_mut().push((font.clone(), variant, vars, i));
        Some(i)
    }
}
pub(crate) fn lookup(
    cache: Option<&LookupCache>,
    fonts: &[FontKey],
    style: &TextStyle,
    font: Option<&Font>,
) -> Option<usize> {
    cache.map_or_else(|| index(fonts, style, font), |c| c.index(fonts, style, font))
}
pub(crate) fn same_font(a: &Option<Font>, b: &Option<Font>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => std::ptr::eq(a.as_slice(), b.as_slice()) || a == b,
        (None, None) => true,
        _ => false,
    }
}

fn bind_items(items: &[FrameItem], fonts: &mut Vec<FontKey>) {
    let mut table = Cow::Owned(std::mem::take(fonts));
    bind_items_with_seed(items, &mut table, &mut <[FontKey]>::to_vec);
    // An owned table remains owned; this wrapper preserves the caller's buffers.
    *fonts = table.into_owned();
}

fn bind_items_with_seed(
    items: &[FrameItem],
    fonts: &mut Cow<'_, [FontKey]>,
    seed: &mut impl FnMut(&[FontKey]) -> Vec<FontKey>,
) {
    struct Binder<'a, 'b, F>(&'a mut Cow<'b, [FontKey]>, LookupCache, &'a mut F);
    impl<F: FnMut(&[FontKey]) -> Vec<FontKey>> FrameVisitor for Binder<'_, '_, F> {
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { font: Some(font), style, .. } = item {
                if self.1.index(self.0, style, Some(font)).is_some() {
                    return;
                }
                if let Some(family) = style.font.clone().or_else(|| {
                    crate::fonts::font_info_from_bytes(font.as_slice(), 0).map(|info| {
                        typst_core::entities::font_list::FontList::single(
                            info.family.into(),
                        )
                    })
                }) {
                    if matches!(self.0, Cow::Borrowed(_)) {
                        *self.0 = Cow::Owned((self.2)(self.0.as_ref()));
                    }
                    self.0.to_mut().push((
                        (
                            family,
                            text_style_to_font_variant(style),
                            style.variations.clone().unwrap_or_default(),
                        ),
                        font.as_slice().to_vec(),
                    ));
                }
            }
        }
    }
    walk_frame_items(&mut Binder(fonts, LookupCache::default(), seed), items);
}

pub(crate) fn bind_document<'a>(
    doc: &PagedDocument,
    supplied: &'a [FontKey],
) -> Cow<'a, [FontKey]> {
    bind_document_with_seed(doc, supplied, <[FontKey]>::to_vec)
}

fn bind_document_with_seed<'a>(
    doc: &PagedDocument,
    supplied: &'a [FontKey],
    mut seed: impl FnMut(&[FontKey]) -> Vec<FontKey>,
) -> Cow<'a, [FontKey]> {
    let mut fonts = Cow::Borrowed(supplied);
    for page in &doc.pages {
        bind_items_with_seed(&page.background, &mut fonts, &mut seed);
        bind_items_with_seed(&page.items, &mut fonts, &mut seed);
        bind_items_with_seed(&page.foreground, &mut fonts, &mut seed);
    }
    fonts
}

pub(super) struct PreparedFonts {
    original: Vec<FontKey>,
    prepared: Vec<FontKey>,
    cache: LookupCache,
}
impl PreparedFonts {
    pub(super) fn new(page: &Page, supplied: &[FontKey]) -> Self {
        let mut original = supplied.to_vec();
        bind_items(&page.background, &mut original);
        bind_items(&page.items, &mut original);
        bind_items(&page.foreground, &mut original);
        let prepared = original
            .iter()
            .map(|(key, bytes)| {
                let axes = merge_explicit_variations(
                    axis_variations_for_font_variant(&key.1),
                    &key.2,
                );
                let tuples: Vec<_> = axes.iter().map(|v| (v.tag, v.value)).collect();
                (
                    key.clone(),
                    instantiate_variable_font(bytes, &tuples)
                        .unwrap_or_else(|| bytes.clone()),
                )
            })
            .collect();
        Self { original, prepared, cache: LookupCache::default() }
    }
    pub(super) fn fonts(&self) -> &[FontKey] {
        &self.prepared
    }
    pub(super) fn get(
        &self,
        style: &TextStyle,
        font: Option<&Font>,
    ) -> Option<(usize, &[u8])> {
        let i = if font.is_some() {
            self.cache.index(&self.original, style, font)?
        } else {
            let family = style.font.as_ref()?;
            let variant = text_style_to_font_variant(style);
            let vars = style.variations.clone().unwrap_or_default();
            self.original.iter().position(|((fl, v, axes), _)| {
                fl == family && *v == variant && *axes == vars
            })?
        };
        self.prepared.get(i).map(|(_, data)| (i, data.as_slice()))
    }
}

/// Flat, private collector views. They never replace rendered document geometry.
pub(super) fn partition(doc: &PagedDocument, fonts: &[FontKey]) -> Vec<PagedDocument> {
    let Some(first) = doc.pages.first() else {
        return vec![PagedDocument::new(vec![]); fonts.len()];
    };
    let mut page = first.clone();
    page.items.clear();
    page.background.clear();
    page.foreground.clear();
    let mut views = vec![PagedDocument::new(vec![page]); fonts.len()];
    struct Distributor<'a> {
        views: &'a mut [PagedDocument],
        fonts: &'a [FontKey],
        cache: LookupCache,
    }
    impl Distributor<'_> {
        fn put(&mut self, item: &FrameItem, style: &TextStyle, font: Option<&Font>) {
            if let Some(i) = self.cache.index(self.fonts, style, font) {
                if let Some(view) = self.views.get_mut(i) {
                    view.pages[0].items.push(item.clone());
                }
            }
        }
    }
    impl FrameVisitor for Distributor<'_> {
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { style, font, .. } = item {
                self.put(item, style, font.as_ref());
            }
        }
        fn visit_text(&mut self, item: &FrameItem) {
            match item {
                FrameItem::Text { style, .. }
                | FrameItem::TextSeparator { style, .. } => self.put(item, style, None),
                _ => {}
            }
        }
        fn visit_glyph(&mut self, item: &FrameItem) {
            if matches!(item, FrameItem::Glyph { .. }) {
                // Direct math Glyph has its existing reverse-map selection contract.
                for view in self.views.iter_mut() {
                    view.pages[0].items.push(item.clone());
                }
            }
        }
    }
    let mut visitor = Distributor {
        views: &mut views,
        fonts,
        cache: LookupCache::default(),
    };
    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.background);
        walk_frame_items(&mut visitor, &page.items);
        walk_frame_items(&mut visitor, &page.foreground);
    }
    views
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::{font_list::FontList, layout_types::Pt};
    #[test]
    fn empty_none_and_unidentifiable_faces_keep_borrowed_prefix() {
        let style = TextStyle::default();
        let supplied: Vec<FontKey> = vec![];
        for doc in [
            PagedDocument::new(vec![]),
            PagedDocument::new(vec![page_with(None, style.clone(), vec![], 1000)]),
            PagedDocument::new(vec![page_with(
                Some(Font::from_data(vec![1])),
                style.clone(),
                vec![],
                1000,
            )]),
        ] {
            let bound = bind_document_with_seed(&doc, &supplied, |_| {
                panic!("no appendable face")
            });
            assert!(matches!(bound, Cow::Borrowed(_)));
            assert!(bound.is_empty());
        }
    }
    #[test]
    fn complete_prefix_never_copies_font_buffers() {
        let style = TextStyle {
            font: Some(FontList::single("test".into())),
            ..Default::default()
        };
        let font = Font::from_data(vec![1, 2, 3]);
        let supplied = vec![(
            (
                style.font.clone().unwrap(),
                text_style_to_font_variant(&style),
                Default::default(),
            ),
            font.as_slice().to_vec(),
        )];
        let page = page_with(Some(font), style, vec![], 1000);
        let doc = PagedDocument::new(vec![page]);
        let mut copies = 0;
        let bound = bind_document_with_seed(&doc, &supplied, |prefix| {
            copies += 1;
            prefix.to_vec()
        });
        assert!(matches!(bound, Cow::Borrowed(_)));
        assert_eq!(bound.as_ref(), supplied.as_slice());
        assert_eq!(copies, 0, "complete prefix must not copy font buffers");
    }
    #[test]
    fn missing_faces_copy_prefix_once_and_preserve_order() {
        let style = TextStyle {
            font: Some(FontList::single("test".into())),
            ..Default::default()
        };
        let key = (
            style.font.clone().unwrap(),
            text_style_to_font_variant(&style),
            Default::default(),
        );
        let supplied = vec![(key.clone(), vec![0])];
        let mut page =
            page_with(Some(Font::from_data(vec![2])), style.clone(), vec![], 1000);
        page.background =
            page_with(Some(Font::from_data(vec![1])), style.clone(), vec![], 1000).items;
        page.foreground =
            page_with(Some(Font::from_data(vec![3])), style, vec![], 1000).items;
        page.items.push(page.background[0].clone());
        let doc = PagedDocument::new(vec![page]);
        let mut copies = 0;
        let bound = bind_document_with_seed(&doc, &supplied, |prefix| {
            copies += 1;
            prefix.to_vec()
        });
        assert!(matches!(bound, Cow::Owned(_)));
        assert_eq!(
            bound.as_ref(),
            &[
                (key.clone(), vec![0]),
                (key.clone(), vec![1]),
                (key.clone(), vec![2]),
                (key, vec![3])
            ]
        );
        assert_eq!(supplied[0].1, vec![0]);
        assert_eq!(copies, 1);
    }
    #[test]
    fn identical_metadata_does_not_alias_distinct_faces_or_axes() {
        let style = TextStyle {
            font: Some(FontList::single("same family".into())),
            size: Pt(11.),
            ..Default::default()
        };
        let key = (
            style.font.clone().unwrap(),
            text_style_to_font_variant(&style),
            Default::default(),
        );
        let fonts = vec![(key.clone(), vec![1, 2]), (key, vec![3, 4])];
        assert_eq!(index(&fonts, &style, Some(&Font::from_data(vec![3, 4]))), Some(1));
        assert_eq!(index(&fonts, &style, Some(&Font::from_data(vec![9]))), None);
        assert_eq!(index(&fonts, &style, None), Some(0));
        let cache = LookupCache::default();
        let first = Font::from_data(vec![3, 4]);
        assert_eq!(cache.index(&fonts, &style, Some(&first)), Some(1));
        assert_eq!(cache.index(&fonts, &style, Some(&first.clone())), Some(1));
        // A second allocation must validate bytes rather than alias by family.
        assert_eq!(
            cache.index(&fonts, &style, Some(&Font::from_data(vec![3, 4]))),
            Some(1)
        );
        let changed = TextStyle { weight: Some(700), ..style };
        assert_eq!(index(&fonts, &changed, Some(&Font::from_data(vec![3, 4]))), None);
    }
    fn page_with(
        font: Option<Font>,
        style: TextStyle,
        glyphs: Vec<typst_core::entities::layout_types::ShapedGlyph>,
        upem: u16,
    ) -> Page {
        use typst_core::entities::{content::Content, layout_types::Point};
        Page {
            width: 150.,
            height: 80.,
            logical_number: 1,
            numbering: None,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::TextShaped {
                pos: Point { x: Pt(20.), y: Pt(40.) },
                glyphs,
                style,
                text: "مرحبا".into(),
                units_per_em: upem,
                font,
                bidi: None,
            }],
        }
    }
    fn shaped(
        font: &Font,
    ) -> (Vec<typst_core::entities::layout_types::ShapedGlyph>, u16) {
        let face = rustybuzz::Face::from_slice(font.as_slice(), 0).unwrap();
        let mut buf = rustybuzz::UnicodeBuffer::new();
        buf.push_str("مرحبا");
        buf.set_direction(rustybuzz::Direction::RightToLeft);
        let run = rustybuzz::shape(&face, &[], buf);
        let glyphs = run
            .glyph_infos()
            .iter()
            .zip(run.glyph_positions())
            .map(|(g, p)| typst_core::entities::layout_types::ShapedGlyph {
                glyph_id: g.glyph_id as u16,
                x_advance: p.x_advance,
                x_offset: p.x_offset,
                y_offset: p.y_offset,
                cluster: g.cluster,
                char_code: 'م',
            })
            .collect();
        (glyphs, face.units_per_em() as u16)
    }
    #[test]
    fn actual_face_draws_same_ink_with_conflicting_legacy_resources() {
        let regular = Font::from_data(
            std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").unwrap(),
        );
        let oblique =
            std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf")
                .unwrap();
        let style = TextStyle {
            font: Some(FontList::single("DejaVu Sans".into())),
            italic: true,
            semantic_emph: true,
            size: Pt(15.),
            ..Default::default()
        };
        let key = (
            style.font.clone().unwrap(),
            text_style_to_font_variant(&style),
            Default::default(),
        );
        let (glyphs, upem) = shaped(&regular);
        let actual =
            page_with(Some(regular.clone()), style.clone(), glyphs.clone(), upem);
        let expected = page_with(None, style, glyphs, upem);
        let wrong = vec![(key.clone(), oblique)];
        let correct = vec![(key, regular.as_slice().to_vec())];
        let opts = super::super::render::RenderOptions::default();
        assert_eq!(
            super::super::render::render_page_to_png_with_fonts(&actual, &opts, &wrong),
            super::super::render::render_page_to_png_with_fonts(
                &expected, &opts, &correct
            )
        );
        let svg_opts = super::super::svg::SvgOptions::default();
        let paths = |xml: String| -> Vec<String> {
            xml.split(" d=\"")
                .skip(1)
                .map(|x| x.split('"').next().unwrap().to_owned())
                .collect()
        };
        let a =
            paths(super::super::svg::export_svg_with_fonts(&actual, &svg_opts, &wrong));
        let b = paths(super::super::svg::export_svg_with_fonts(
            &expected, &svg_opts, &correct,
        ));
        assert!(!a.is_empty());
        assert_eq!(a, b);
        let doc = PagedDocument::new(vec![actual]);
        let bound = bind_document(&doc, &wrong);
        assert_eq!(bound.len(), 2);
        let views = partition(&doc, &bound);
        assert!(super::super::fonts::collect_glyph_ids(&views[0]).is_empty());
        assert!(!super::super::fonts::collect_glyph_ids(&views[1]).is_empty());
    }
    #[test]
    fn variable_original_identity_and_explicit_axes_survive_instancing() {
        let data = include_bytes!("../../fixtures/fonts/UbuntuSans-Variable.ttf");
        let font = Font::from_data(data.to_vec());
        let style = TextStyle {
            font: Some(FontList::single("Ubuntu Sans".into())),
            size: Pt(11.),
            weight: Some(700),
            variations: Some(typst_core::entities::font_variations::FontVariations(
                vec![(*b"wght", 250.)],
            )),
            ..Default::default()
        };
        let page = page_with(Some(font.clone()), style.clone(), vec![], 1000);
        let bindings = PreparedFonts::new(&page, &[]);
        let (_, bytes) = bindings.get(&style, Some(&font)).unwrap();
        assert_eq!(bindings.original[0].1, data);
        assert!(ttf_parser::Face::parse(bytes, 0).unwrap().tables().fvar.is_none());
        let axes = vec![(ttf_parser::Tag::from_bytes(b"wght"), 250.)];
        let expected = instantiate_variable_font(data, &axes).unwrap();
        let gid = ttf_parser::Face::parse(data, 0).unwrap().glyph_index('m').unwrap();
        assert_eq!(
            ttf_parser::Face::parse(bytes, 0).unwrap().glyph_hor_advance(gid),
            ttf_parser::Face::parse(&expected, 0).unwrap().glyph_hor_advance(gid)
        );
        let other = TextStyle {
            variations: Some(typst_core::entities::font_variations::FontVariations(
                vec![(*b"wght", 900.)],
            )),
            ..style
        };
        assert!(bindings.get(&other, Some(&font)).is_none());
    }
}
