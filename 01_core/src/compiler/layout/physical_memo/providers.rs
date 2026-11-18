//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/physical_memo/providers.md
//! @prompt-hash 0b8ac9eb
//! @layer L1
//! @updated 2026-11-13

use super::{frame_charge, style_charge, Journal};
use crate::compiler::layout::FontMetrics;
use crate::entities::{
    glyph_variants::{GlyphAssembly, GlyphVariants, MathGlyphKern},
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Pt, TextStyle},
    math_constants::MathConstants,
};
#[derive(Clone)]
pub(super) struct RecordingMetrics<M> {
    pub(super) immutable: bool,
    pub(super) inner: M,
    pub(super) journal: Journal,
}
fn point(p: Pt) -> u64 {
    p.0.to_bits()
}
fn pair(p: (Pt, Pt)) -> (u64, u64) {
    (point(p.0), point(p.1))
}
impl<M: FontMetrics> FontMetrics for RecordingMetrics<M> {
    fn physical_epoch(&self) -> Option<crate::compiler::layout::MetricEpoch> {
        self.inner.physical_epoch()
    }

    fn paragraph_session(
        &self,
    ) -> Option<crate::compiler::layout::paragraph_session::ParagraphSession> {
        self.inner.paragraph_session()
    }
    fn paragraph_identity(
        &self,
        text: &str,
        style: &TextStyle,
    ) -> Option<crate::compiler::layout::paragraph_session::MetricIdentity> {
        let output = self.inner.paragraph_identity(text, style);
        if self.immutable {
            return output;
        }
        let owner_charge = output
            .as_ref()
            .and_then(|id| id.font())
            .map_or(Some(0), |(owner, size)| self.journal.font_charge(owner, size));
        if text.len() <= 16384 && style_charge(style).is_some() && owner_charge.is_some()
        {
            let expected = output.clone();
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len()
                    + style_charge(&style).unwrap_or(0)
                    + owner_charge.unwrap_or(0),
                move |input| match (&expected, input.paragraph_identity(&text, &style)) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a.same(&b),
                    (None, Some(_)) | (Some(_), None) => false,
                },
            );
        } else {
            self.journal.decline("paragraph_identity");
        }
        output
    }
    fn text_trailing_overhang(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
        ratios: &[(char, f64)],
    ) -> Option<(Pt, usize)> {
        let output = self.inner.text_trailing_overhang(text, size, style, ratios);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() && ratios.len() <= 256 {
            let expected = output.map(|(p, n)| (point(p), n));
            let text = text.to_owned();
            let style = style.clone();
            let ratios = ratios.to_vec();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0) + ratios.len() * 16,
                move |input| {
                    input
                        .text_trailing_overhang(&text, size, &style, &ratios)
                        .map(|(p, n)| (point(p), n))
                        == expected
                },
            );
        } else {
            self.journal.decline("text_trailing_overhang");
        }
        output
    }
    fn glyph_overhang(
        &self,
        font: &crate::entities::world_types::Font,
        glyph_id: u16,
        advance: Pt,
        ratios: &[(char, f64)],
    ) -> Option<Pt> {
        let output = self.inner.glyph_overhang(font, glyph_id, advance, ratios);
        if matches!(font, crate::entities::world_types::Font::Vec(_))
            && font.as_slice().len() <= 16 * 1024 * 1024
            && ratios.len() <= 256
        {
            if self.immutable {
                return output;
            }
            let expected = output.map(point);
            let font = font.clone();
            let ratios = ratios.to_vec();
            self.journal.metric(
                128 + font.as_slice().len() + ratios.len() * 16,
                move |input| {
                    input.glyph_overhang(&font, glyph_id, advance, &ratios).map(point)
                        == expected
                },
            );
        } else {
            self.journal.decline("glyph_overhang");
        }
        output
    }
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        let output = self.inner.advance(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = point(output);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| point(input.advance(&text, size, &style)) == expected,
            );
        } else {
            self.journal.decline("advance");
        }
        output
    }
    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let output = self.inner.vertical_metrics(size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = pair(output);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    pair(input.vertical_metrics(size, &style)) == expected
                });
        } else {
            self.journal.decline("vertical_metrics");
        }
        output
    }
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
        let output = self.inner.cap_height(size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = point(output);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    point(input.cap_height(size, &style)) == expected
                });
        } else {
            self.journal.decline("cap_height");
        }
        output
    }
    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let output = self.inner.text_edges(size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = pair(output);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    pair(input.text_edges(size, &style)) == expected
                });
        } else {
            self.journal.decline("text_edges");
        }
        output
    }
    fn text_superscript_metrics(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> (Pt, Pt, Pt, bool) {
        let output = self.inner.text_superscript_metrics(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = (point(output.0), point(output.1), point(output.2), output.3);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| {
                    let p = input.text_superscript_metrics(&text, size, &style);
                    (point(p.0), point(p.1), point(p.2), p.3) == expected
                },
            );
        } else {
            self.journal.decline("text_superscript_metrics");
        }
        output
    }
    fn text_superscript_runs(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> Vec<(std::ops::Range<usize>, TextStyle, Pt)> {
        self.journal.decline("text_superscript_runs");
        self.inner.text_superscript_runs(text, size, style)
    }
    fn underline_metrics(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let output = self.inner.underline_metrics(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = pair(output);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| {
                    pair(input.underline_metrics(&text, size, &style)) == expected
                },
            );
        } else {
            self.journal.decline("underline_metrics");
        }
        output
    }
    fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let output = self.inner.text_ink_bounds(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = pair(output);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| pair(input.text_ink_bounds(&text, size, &style)) == expected,
            );
        } else {
            self.journal.decline("text_ink_bounds");
        }
        output
    }
    fn text_ink_bounds_signed(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> (Pt, Pt) {
        let output = self.inner.text_ink_bounds_signed(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = pair(output);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| {
                    pair(input.text_ink_bounds_signed(&text, size, &style)) == expected
                },
            );
        } else {
            self.journal.decline("text_ink_bounds_signed");
        }
        output
    }
    fn math_constants(&self, style: &TextStyle) -> MathConstants {
        self.journal.decline("math_constants");
        self.inner.math_constants(style)
    }
    fn vertical_glyph_variants(&self, c: char, style: &TextStyle) -> GlyphVariants {
        self.journal.decline("vertical_glyph_variants");
        self.inner.vertical_glyph_variants(c, style)
    }
    fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
        let output = self.inner.glyph_to_char(glyph_id);
        if self.immutable {
            return output;
        }
        if true {
            let expected = output;
            self.journal
                .metric(128, move |input| input.glyph_to_char(glyph_id) == expected);
        } else {
            self.journal.decline("glyph_to_char");
        }
        output
    }
    fn glyph_ink_bounds(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let output = self.inner.glyph_ink_bounds(glyph_id, size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = pair(output);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    pair(input.glyph_ink_bounds(glyph_id, size, &style)) == expected
                });
        } else {
            self.journal.decline("glyph_ink_bounds");
        }
        output
    }
    fn italics_correction(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> Pt {
        let output = self.inner.italics_correction(glyph_id, size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = point(output);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    point(input.italics_correction(glyph_id, size, &style)) == expected
                });
        } else {
            self.journal.decline("italics_correction");
        }
        output
    }
    fn char_italics_correction(&self, c: char, size: Pt, style: &TextStyle) -> Pt {
        let output = self.inner.char_italics_correction(c, size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = point(output);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    point(input.char_italics_correction(c, size, &style)) == expected
                });
        } else {
            self.journal.decline("char_italics_correction");
        }
        output
    }
    fn top_accent_attach(&self, c: char, size: Pt, style: &TextStyle) -> Option<Pt> {
        let output = self.inner.top_accent_attach(c, size, style);
        if self.immutable {
            return output;
        }
        if style_charge(style).is_some() {
            let expected = output.map(point);
            let style = style.clone();
            self.journal
                .metric(128 + style_charge(&style).unwrap_or(0), move |input| {
                    input.top_accent_attach(c, size, &style).map(point) == expected
                });
        } else {
            self.journal.decline("top_accent_attach");
        }
        output
    }
    fn vertical_glyph_assembly(&self, c: char, style: &TextStyle) -> GlyphAssembly {
        self.journal.decline("vertical_glyph_assembly");
        self.inner.vertical_glyph_assembly(c, style)
    }
    fn horizontal_glyph_variants(&self, c: char, style: &TextStyle) -> GlyphVariants {
        self.journal.decline("horizontal_glyph_variants");
        self.inner.horizontal_glyph_variants(c, style)
    }
    fn horizontal_glyph_assembly(&self, c: char, style: &TextStyle) -> GlyphAssembly {
        self.journal.decline("horizontal_glyph_assembly");
        self.inner.horizontal_glyph_assembly(c, style)
    }
    fn math_kern(&self, c: char, style: &TextStyle) -> MathGlyphKern {
        self.journal.decline("math_kern");
        self.inner.math_kern(c, style)
    }
    fn advance_shaped(&self, text: &str, size: Pt, style: &TextStyle) -> Option<Pt> {
        let output = self.inner.advance_shaped(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = output.map(point);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| {
                    input.advance_shaped(&text, size, &style).map(point) == expected
                },
            );
        } else {
            self.journal.decline("advance_shaped");
        }
        output
    }
    fn text_width(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        let output = self.inner.text_width(text, size, style);
        if self.immutable {
            return output;
        }
        if text.len() <= 16384 && style_charge(style).is_some() {
            let expected = point(output);
            let text = text.to_owned();
            let style = style.clone();
            self.journal.metric(
                128 + text.len() + style_charge(&style).unwrap_or(0),
                move |input| point(input.text_width(&text, size, &style)) == expected,
            );
        } else {
            self.journal.decline("text_width");
        }
        output
    }
    fn line_content_right(&self, items: &[&FrameItem]) -> f64 {
        let output = self.inner.line_content_right(items);
        match frame_charge(items) {
            Ok(item_charge) => {
                if self.immutable {
                    return output;
                }
                let expected = output.to_bits();
                let items: Vec<_> = items.iter().map(|i| (*i).clone()).collect();
                self.journal.metric(128 + item_charge, move |input| {
                    input.line_content_right(&items.iter().collect::<Vec<_>>()).to_bits()
                        == expected
                });
            }
            Err(reason) => self.journal.decline(reason),
        }
        output
    }
}
#[derive(Clone)]
pub(super) struct RecordingSizer<S> {
    pub(super) inner: S,
    pub(super) journal: Journal,
}
impl<S: ImageSizer> ImageSizer for RecordingSizer<S> {
    fn prepare_svg(
        &self,
        data: &[u8],
        source: &crate::entities::svg_image::SvgSource,
        families: Option<&crate::entities::font_list::FontList>,
        fallback: bool,
    ) -> Result<crate::entities::svg_image::PreparedSvg, String> {
        self.journal.decline("image prepare_svg");
        self.inner.prepare_svg(data, source, families, fallback)
    }
    fn size(&self, data: &[u8]) -> Option<(u32, u32)> {
        self.journal.decline("image size");
        self.inner.size(data)
    }
    fn dpi(&self, data: &[u8]) -> Option<f64> {
        self.journal.decline("image dpi");
        self.inner.dpi(data)
    }
    fn orientation(&self, data: &[u8]) -> Option<u32> {
        self.journal.decline("image orientation");
        self.inner.orientation(data)
    }
}

#[cfg(test)]
mod physical_retention_provider_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::introspector::TagIntrospector;
    use std::sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    };
    #[derive(Clone)]
    struct Custom(Arc<AtomicU64>);
    impl FontMetrics for Custom {
        fn advance(&self, t: &str, s: Pt, st: &TextStyle) -> Pt {
            FixedMetrics.advance(t, s, st)
        }
        fn vertical_metrics(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(s, st)
        }
        fn cap_height(&self, s: Pt, st: &TextStyle) -> Pt {
            FixedMetrics.cap_height(s, st)
        }
        fn text_edges(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(s, st)
        }
        fn line_content_right(&self, items: &[&FrameItem]) -> f64 {
            f64::from_bits(self.0.load(Ordering::Relaxed))
                + items
                    .iter()
                    .map(|item| match item {
                        FrameItem::Shape { width, .. } => *width,
                        _ => 0.0,
                    })
                    .sum::<f64>()
        }
    }
    #[test]
    fn physical_retention_preserves_and_revalidates_custom_line_override() {
        let original = Custom(Arc::new(AtomicU64::new(42.0f64.to_bits())));
        let journal = Journal::default();
        let recorded = RecordingMetrics {
            inner: original.clone(),
            journal: journal.clone(),
            immutable: false,
        };
        assert_eq!(recorded.line_content_right(&[]), 42.0);
        assert!(journal.validates(&original, &TagIntrospector::empty()));
        original.0.store(43.0f64.to_bits(), Ordering::Relaxed);
        assert!(!journal.validates(&original, &TagIntrospector::empty()));
    }
    #[test]
    fn physical_retention_complex_metrics_decline_without_changing_the_answer() {
        let journal = Journal::default();
        let recorded = RecordingMetrics {
            inner: FixedMetrics,
            journal: journal.clone(),
            immutable: false,
        };
        let style = TextStyle::default();
        let answer = recorded.math_constants(&style);
        assert_eq!(
            format!("{:?}", answer),
            format!("{:?}", FixedMetrics.math_constants(&style))
        );
        assert!(!journal.validates(&FixedMetrics, &TagIntrospector::empty()));
    }
    #[test]
    fn physical_retention_image_queries_decline_and_preserve_provider_answer() {
        struct Images;
        impl ImageSizer for Images {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                Some((9, 7))
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                Some(96.0)
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                Some(3)
            }
        }
        let journal = Journal::default();
        let s = RecordingSizer { inner: Images, journal: journal.clone() };
        assert_eq!(s.size(&[1, 2]), Some((9, 7)));
        assert_eq!(s.dpi(&[1, 2]), Some(96.0));
        assert_eq!(s.orientation(&[1, 2]), Some(3));
        assert!(!journal.validates(&FixedMetrics, &TagIntrospector::empty()));
    }
    fn shape(kind: crate::entities::geometry::ShapeKind<Pt>) -> FrameItem {
        FrameItem::Shape {
            pos: crate::entities::layout_types::Point { x: Pt(3.0), y: Pt(7.0) },
            kind,
            width: 12.0,
            height: 6.0,
            fill: Some(crate::entities::frame_paint::FramePaint::Solid(
                crate::entities::color::Color::rgb(5, 10, 20),
            )),
            stroke: Some(crate::entities::frame_paint::FrameStroke::default()),
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        }
    }
    #[test]
    fn physical_retention_geometric_line_request_is_owned_and_revalidated() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        for kind in [
            ShapeKind::Rect,
            ShapeKind::Ellipse,
            ShapeKind::Line { dx: 3.0, dy: 7.0 },
            ShapeKind::Path(vec![PathItem::ClosePath]),
        ] {
            let original = Custom(Arc::new(AtomicU64::new(42.0f64.to_bits())));
            let journal = Journal::default();
            let recorded = RecordingMetrics {
                inner: original.clone(),
                journal: journal.clone(),
                immutable: false,
            };
            let mut item = shape(kind);
            assert_eq!(recorded.line_content_right(&[&item]), 54.0);
            if let FrameItem::Shape { width, .. } = &mut item {
                *width = 100.0;
            }
            assert_eq!(original.line_content_right(&[&item]), 142.0);
            assert!(
                journal.validates(&original, &TagIntrospector::empty()),
                "owned scalar geometry should be certifiable"
            );
            original.0.store(43.0f64.to_bits(), Ordering::Relaxed);
            assert!(!journal.validates(&original, &TagIntrospector::empty()));
        }
    }
    #[test]
    fn physical_retention_large_geometry_and_complex_paint_stay_uncached() {
        use crate::entities::{
            frame_paint::FramePaint,
            geometry::{PathItem, ShapeKind},
            gradient::Gradient,
            layout_types::Angle,
        };
        let mut complex = shape(ShapeKind::Rect);
        if let FrameItem::Shape { fill, .. } = &mut complex {
            *fill =
                Some(FramePaint::Gradient(Gradient::linear(vec![], Angle::deg(30.0))));
        }
        for item in [shape(ShapeKind::Path(vec![PathItem::ClosePath; 8193])), complex] {
            let original = Custom(Arc::new(AtomicU64::new(42.0f64.to_bits())));
            let journal = Journal::default();
            let recorded = RecordingMetrics {
                inner: original.clone(),
                journal: journal.clone(),
                immutable: false,
            };
            assert_eq!(recorded.line_content_right(&[&item]), 54.0);
            assert!(!journal.validates(&original, &TagIntrospector::empty()));
        }
    }
}
