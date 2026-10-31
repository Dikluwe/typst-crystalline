//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/optical_margin.md
//! @prompt-hash e483aef9
//! @layer L1
//! @updated 2026-10-28

pub(super) const RATIOS: &[(char, f64)] = &[
    ('–', 0.2),
    ('—', 0.2),
    ('-', 0.55),
    ('\u{ad}', 0.55),
    ('.', 0.8),
    (',', 0.8),
    (';', 0.3),
    (':', 0.3),
    ('\u{60c}', 0.4),
    ('\u{6d4}', 0.4),
];

use super::FontMetrics;
use crate::entities::{
    dir::Dir,
    layout_types::{FrameItem, Pt, TextStyle},
};

fn certified(amount: Pt, glyphs: usize, exterior: bool) -> f64 {
    if amount.0.is_finite() && amount.0 >= 0.0 && glyphs > 0 && (glyphs > 1 || exterior) {
        amount.0
    } else {
        0.0
    }
}

pub(super) fn text_amount(
    metrics: &impl FontMetrics,
    text: &str,
    style: &TextStyle,
    exterior: bool,
) -> f64 {
    if style.dir.is_some_and(|dir| dir != Dir::LTR) || style.math {
        return 0.0;
    }
    metrics
        .text_trailing_overhang(text, style.size, style, RATIOS)
        .map_or(0.0, |(amount, glyphs)| certified(amount, glyphs, exterior))
}

fn plain_ltr(text: &str) -> bool {
    use unicode_script::{Script, UnicodeScript};
    !text.is_empty()
        && text.chars().all(|c| {
            c.is_ascii_graphic()
                || c == ' '
                || matches!(c.script(), Script::Latin | Script::Inherited)
        })
}

fn leaves<'a>(
    items: &'a [FrameItem],
    out: &mut Vec<&'a FrameItem>,
    retain_literal: bool,
) {
    for item in items {
        if super::html_frame::is_context_marker(item) {
            continue;
        }
        match item {
            FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => {
                leaves(items, out, retain_literal)
            }
            FrameItem::TextSeparator {
                origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                ..
            } => {
                if retain_literal {
                    out.push(item);
                }
            }
            FrameItem::TextSeparator {
                origin:
                    crate::entities::layout_types::TextSeparatorOrigin::Markup
                    | crate::entities::layout_types::TextSeparatorOrigin::Newline,
                ..
            } => {}
            FrameItem::Text { text, .. } => {
                if !text.trim().is_empty() {
                    out.push(item);
                }
            }
            FrameItem::TextShaped { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. } => out.push(item),
        }
    }
}

pub(super) fn line_amount(metrics: &impl FontMetrics, items: &[FrameItem]) -> f64 {
    aligned_line_amount(metrics, items, false)
}

pub(super) fn aligned_line_amount(
    metrics: &impl FontMetrics,
    items: &[FrameItem],
    retain_literal: bool,
) -> f64 {
    let mut flat = Vec::new();
    leaves(items, &mut flat, retain_literal);
    let Some(last) = flat.last() else {
        return 0.0;
    };
    match last {
        FrameItem::Text { text, style, .. } => {
            text_amount(metrics, text, style, flat.len() > 1)
        }
        FrameItem::TextShaped {
            glyphs,
            font: Some(font),
            units_per_em,
            style,
            text,
            ..
        } => {
            if *units_per_em == 0
                || style.dir.is_some_and(|dir| dir != Dir::LTR)
                || style.math
                || !style.size.0.is_finite()
                || style.size.0 <= 0.0
                || !plain_ltr(text)
            {
                return 0.0;
            }
            let Some(glyph) = glyphs.last() else {
                return 0.0;
            };
            let advance =
                Pt(f64::from(glyph.x_advance) * style.size.0 / f64::from(*units_per_em));
            metrics
                .glyph_overhang(font, glyph.glyph_id, advance, RATIOS)
                .map_or(0.0, |amount| certified(amount, glyphs.len(), flat.len() > 1))
        }
        FrameItem::TextShaped { font: None, .. }
        | FrameItem::Line { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Group { .. }
        | FrameItem::Link { .. }
        | FrameItem::TextSeparator { .. }
        | FrameItem::TextReplacement { .. }
        | FrameItem::Semantic { .. } => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, FontMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PageMarginSpec, PageMargins, Pt, TextStyle},
        value::Value,
        world_types::Font,
    };
    use comemo::Track;

    #[derive(Clone, Copy)]
    struct Supplied;
    impl FontMetrics for Supplied {
        fn advance(&self, text: &str, _: Pt, _: &TextStyle) -> Pt {
            Pt(text.chars().count() as f64 * 5.0)
        }
        fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(size, style)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.cap_height(size, style)
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(size, style)
        }
        fn text_trailing_overhang(
            &self,
            _: &str,
            _: Pt,
            _: &TextStyle,
            ratios: &[(char, f64)],
        ) -> Option<(Pt, usize)> {
            assert!(ratios.contains(&('.', 0.8)));
            Some((Pt(3.0), 3))
        }
        fn glyph_overhang(
            &self,
            _: &Font,
            gid: u16,
            advance: Pt,
            _: &[(char, f64)],
        ) -> Option<Pt> {
            Some(Pt(f64::from(gid) + advance.0))
        }
    }

    #[test]
    fn readiness_optical_defaults_and_borrowed_override_dispatch() {
        fn text<M: FontMetrics>(m: &M) -> Option<(Pt, usize)> {
            m.text_trailing_overhang(
                "A.",
                Pt(11.0),
                &TextStyle::regular(Pt(11.0)),
                RATIOS,
            )
        }
        fn glyph<M: FontMetrics>(m: &M) -> Option<Pt> {
            // Opaque fake bytes isolate forwarding; real-font validation is L3.
            m.glyph_overhang(&Font::from_data(vec![]), 4, Pt(2.0), RATIOS)
        }
        assert_eq!(text(&FixedMetrics), None);
        assert_eq!(glyph(&FixedMetrics), None);
        assert_eq!(text(&Supplied), Some((Pt(3.0), 3)));
        assert_eq!(glyph(&Supplied), Some(Pt(6.0)));
        let dynamic: &dyn FontMetrics = &Supplied;
        assert_eq!(text(&dynamic), Some((Pt(3.0), 3)));
        assert_eq!(glyph(&dynamic), Some(Pt(6.0)));
    }

    fn accepted_edge(legacy: bool, enabled: bool) -> f64 {
        edge_for(legacy, enabled, false)
    }
    fn edge_for(legacy: bool, enabled: bool, scoped: bool) -> f64 {
        edge_options(legacy, enabled, scoped, false)
    }
    fn edge_options(legacy: bool, enabled: bool, scoped: bool, last: bool) -> f64 {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            Supplied,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = if last { 54.0 } else { 55.0 };
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = l.page_config.width;
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_x = Pt(10.0);
        l.chain = l
            .chain
            .push_custom("par.justify", Value::Bool(true))
            .push_custom("text.hyphenate", Value::Bool(false))
            .push_custom("text.overhang", Value::Bool(enabled));
        l.paragraph_planning_suppressed = legacy;
        let content = if last {
            Content::text("aa. bb.")
        } else if scoped {
            Content::sequence(vec![
                Content::text("aa. "),
                Content::styled(
                    Content::text("bb."),
                    crate::entities::style::Styles::new()
                        .push_custom("text.overhang", Value::Bool(false)),
                ),
                Content::text(" cc."),
            ])
        } else {
            Content::text("aa. bb. cc.")
        };
        l.layout_content(&content);
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        let mut found = Vec::new();
        for item in &doc.pages[0].items {
            if let FrameItem::Text { text, pos, .. } = item {
                if text == "bb." {
                    found.push(pos.x.0 + 15.0);
                }
            }
        }
        assert_eq!(
            found.len(),
            1,
            "actual accepted bb. occurrence: {:?}",
            doc.pages[0].items
        );
        found[0]
    }

    #[test]
    fn readiness_optical_prepared_emitter_consumes_injected_amount() {
        assert_eq!(accepted_edge(false, true), 48.0);
    }
    #[test]
    fn readiness_optical_legacy_emitter_consumes_injected_amount() {
        assert_eq!(accepted_edge(true, true), 48.0);
    }
    #[test]
    fn readiness_optical_false_preserves_both_emitters() {
        for legacy in [false, true] {
            assert_eq!(accepted_edge(legacy, false), 45.0);
        }
    }
    #[test]
    fn readiness_optical_final_false_survives_outer_style_restoration() {
        for legacy in [false, true] {
            assert_eq!(edge_for(legacy, true, true), 45.0);
        }
    }
    #[test]
    fn readiness_optical_uncertified_and_single_glyph_guards() {
        assert_eq!(certified(Pt(3.0), 1, false), 0.0);
        assert_eq!(certified(Pt(3.0), 1, true), 3.0);
        for n in [0, 1, 2] {
            assert_eq!(certified(Pt(f64::NAN), n, true), 0.0);
            assert_eq!(certified(Pt(-1.0), n, true), 0.0);
        }
        assert_eq!(certified(Pt(3.0), 0, true), 0.0);
        let mut style = TextStyle::regular(Pt(12.0));
        for dir in [
            crate::entities::dir::Dir::RTL,
            crate::entities::dir::Dir::TTB,
            crate::entities::dir::Dir::BTT,
        ] {
            style.dir = Some(dir);
            assert_eq!(text_amount(&Supplied, "aa.", &style, true), 0.0);
        }
    }
    #[test]
    fn readiness_optical_shaped_uses_supplied_gid_advance_without_rewriting() {
        use crate::entities::layout_types::{Point, ShapedGlyph};
        let glyph = ShapedGlyph {
            glyph_id: 4,
            x_advance: 1,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: '.',
        };
        let mut items = vec![FrameItem::TextShaped {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            glyphs: vec![glyph.clone(), glyph],
            style: TextStyle::regular(Pt(12.0)),
            text: "A.".into(),
            units_per_em: 6,
            font: Some(Font::from_data(vec![])),
            bidi: None,
        }];
        let original = format!("{items:?}");
        assert_eq!(line_amount(&Supplied, &items), 6.0);
        assert_eq!(format!("{items:?}"), original);
        if let FrameItem::TextShaped { font, .. } = &mut items[0] {
            *font = None;
        }
        assert_eq!(line_amount(&Supplied, &items), 0.0);
    }
    #[test]
    fn readiness_optical_parent_line_survives_isolated_subframe() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            Supplied,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 55.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 55.0;
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_x = Pt(10.0);
        l.chain = l
            .chain
            .push_custom("par.justify", Value::Bool(true))
            .push_custom("text.hyphenate", Value::Bool(false));
        l.paragraph_planning_suppressed = true;
        l.layout_content(&Content::text("aa. bb."));
        let child = Content::styled(
            Content::text("child"),
            crate::entities::style::Styles::new()
                .push_custom("text.overhang", Value::Bool(false)),
        );
        let _ = l.layout_sub_frame(
            &child,
            super::super::SubLayoutRegion {
                origin_x: 0.0,
                width: 100.0,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );
        l.layout_content(&Content::text(" cc."));
        let doc = l.finish();
        let bb: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|page| &page.items)
            .filter_map(|item| {
                let FrameItem::Text { text, pos, .. } = item else {
                    return None;
                };
                (text == "bb.").then_some(pos.x.0 + 15.0)
            })
            .collect();
        assert_eq!(bb, vec![48.0]);
    }
    #[test]
    fn readiness_optical_final_compression_uses_accepted_remainder() {
        assert_eq!(edge_options(false, false, false, true), 44.0);
        // Natural width is 35pt, available 34pt. Optical 3pt avoids compression;
        // the final left-aligned line preserves its natural right edge at45pt.
        assert_eq!(edge_options(false, true, false, true), 45.0);
    }
    fn aligned_start(
        alignment: crate::entities::layout_types::HAlign,
        enabled: bool,
    ) -> f64 {
        aligned_text_start(alignment, enabled, "aa.")
    }

    fn aligned_text_start(
        alignment: crate::entities::layout_types::HAlign,
        enabled: bool,
        text: &str,
    ) -> f64 {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            Supplied,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 100.0;
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_x = Pt(10.0);
        l.chain = l.chain.push_custom("text.overhang", Value::Bool(enabled));
        l.explicit_line_alignment = Some(alignment);
        l.paragraph_planning_suppressed = true;
        l.layout_content(&Content::text(text));
        l.flush_line();
        assert!(l.layout_errors.is_empty());
        l.regions
            .current
            .current_items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { text, pos, .. } if text.as_str() == "aa." => {
                    Some(pos.x.0)
                }
                _ => None,
            })
            .expect("one actual accepted aa. line")
    }

    #[test]
    fn readiness_optical_centered_line_uses_half_of_certified_terminal_amount() {
        use crate::entities::layout_types::HAlign;
        assert!((aligned_start(HAlign::Center, false) - 42.5).abs() < 1e-8);
        assert!((aligned_start(HAlign::Center, true) - 44.0).abs() < 1e-8);
    }

    #[test]
    fn readiness_optical_right_line_uses_certified_terminal_amount() {
        use crate::entities::layout_types::HAlign;
        assert!((aligned_start(HAlign::Right, false) - 75.0).abs() < 1e-8);
        assert!((aligned_start(HAlign::Right, true) - 78.0).abs() < 1e-8);
    }
    #[test]
    fn readiness_optical_center_retains_literal_terminal_space_without_punctuation_overhang(
    ) {
        use crate::entities::layout_types::HAlign;
        for enabled in [false, true] {
            let x = aligned_text_start(HAlign::Center, enabled, "aa. ");
            assert!((x - 40.0).abs() < 1e-8, "enabled={enabled}, x={x}");
        }
    }

    #[test]
    fn readiness_optical_right_retains_literal_terminal_space_without_punctuation_overhang(
    ) {
        use crate::entities::layout_types::HAlign;
        for enabled in [false, true] {
            let x = aligned_text_start(HAlign::Right, enabled, "aa. ");
            assert!((x - 70.0).abs() < 1e-8, "enabled={enabled}, x={x}");
        }
    }
}
