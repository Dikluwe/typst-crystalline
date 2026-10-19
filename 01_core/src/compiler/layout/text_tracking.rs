//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text_tracking.md
//! @prompt-hash ffeaca5d
//! @layer L1
//! @updated 2026-09-29

use crate::entities::layout_types::{Pt, TextStyle};

/// Private lexical continuity, independent of resolved render-style equality.
#[derive(Clone, Default)]
pub(super) struct State {
    current: std::sync::Arc<()>,
    previous: Option<std::sync::Arc<()>>,
}

impl State {
    pub(super) fn enter(&mut self) -> std::sync::Arc<()> {
        std::mem::replace(&mut self.current, std::sync::Arc::new(()))
    }
    pub(super) fn leave(&mut self, previous: std::sync::Arc<()>) {
        self.current = previous;
    }
    pub(super) fn record(&mut self) {
        self.previous = Some(self.current.clone());
    }
    pub(super) fn interrupt(&mut self) {
        self.previous = Some(std::sync::Arc::new(()));
    }
    pub(super) fn boundary(
        &self,
        items: &[crate::entities::layout_types::FrameItem],
        style: &TextStyle,
    ) -> Pt {
        if self
            .previous
            .as_ref()
            .is_some_and(|previous| !std::sync::Arc::ptr_eq(previous, &self.current))
        {
            return Pt(0.0);
        }
        boundary(items, style)
    }
}

/// Interval entering the next unit, owned by the previous uniform textual unit.
pub(super) fn boundary(
    items: &[crate::entities::layout_types::FrameItem],
    style: &TextStyle,
) -> Pt {
    use crate::entities::layout_types::FrameItem;
    fn previous(item: &FrameItem) -> Option<&TextStyle> {
        match item {
            FrameItem::Text { text, style, .. }
            | FrameItem::TextShaped { text, style, .. }
                if !text.is_empty() =>
            {
                Some(style)
            }
            FrameItem::TextSeparator { style, .. } => Some(style),
            FrameItem::TextReplacement { items, .. } => items.last().and_then(previous),
            FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::Line { .. }
            | FrameItem::Semantic { .. } => None,
        }
    }
    if style.math {
        return Pt(0.0);
    }
    let last = items
        .iter()
        .rev()
        .find(|item| !super::html_frame::is_context_marker(item));
    let Some(old) = last.and_then(previous) else { return Pt(0.0) };
    if old != style {
        return Pt(0.0);
    }
    Pt(style.tracking.map(|t| t.resolve_pt(style.size.0)).unwrap_or(0.0))
}

/// Terminal interval kept by a non-justified RTL line that will continue.
pub(super) fn continued_rtl_end(
    items: &[crate::entities::layout_types::FrameItem],
) -> Pt {
    use crate::entities::{dir::Dir, layout_types::FrameItem};
    for item in items.iter().rev() {
        if super::html_frame::is_context_marker(item)
            || matches!(item, FrameItem::TextSeparator { .. })
        {
            continue;
        }
        return match item {
            FrameItem::Text { text, style, .. }
            | FrameItem::TextShaped { text, style, .. } => {
                if !text.is_empty() && !style.math && style.dir == Some(Dir::RTL) {
                    Pt(style.tracking.map(|t| t.resolve_pt(style.size.0)).unwrap_or(0.0))
                } else {
                    Pt(0.0)
                }
            }
            FrameItem::TextReplacement { items, .. } => continued_rtl_end(items),
            FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::Line { .. }
            | FrameItem::Semantic { .. } => Pt(0.0),
            FrameItem::TextSeparator { .. } => unreachable!("filtered before match"),
        };
    }
    Pt(0.0)
}

#[cfg(test)]
mod tracking_tests {
    use super::*;
    use crate::entities::layout_types::{FrameItem, Length, Point, TextSeparatorOrigin};
    #[test]
    fn textual_tracking_boundary_is_single_signed_and_style_scoped() {
        let mut style = TextStyle::regular(Pt(12.0));
        for t in [0.2, -0.2, 0.0] {
            style.tracking = Some(Length::pt(t));
            let text = FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: "a".into(),
                style: style.clone(),
                bidi: None,
            };
            let space = FrameItem::TextSeparator {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                advance: Pt(3.0),
                style: style.clone(),
                origin: TextSeparatorOrigin::Literal,
                bidi: None,
            };
            assert_eq!(boundary(&[], &style), Pt(0.0));
            assert_eq!(boundary(std::slice::from_ref(&text), &style), Pt(t));
            assert_eq!(boundary(&[text.clone(), space.clone(), space], &style), Pt(t));
            assert_eq!(
                boundary(
                    &[FrameItem::TextReplacement {
                        text: "logical".into(),
                        items: vec![text]
                    }],
                    &style
                ),
                Pt(t)
            );
        }
        style.tracking = Some(Length::pt(0.2));
        let text = FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            text: "a".into(),
            style: style.clone(),
            bidi: None,
        };
        style.size = Pt(20.0);
        assert_eq!(boundary(&[text], &style), Pt(0.0));
    }
    #[test]
    fn explicit_rtl_has_the_same_signed_boundary_as_ltr() {
        use crate::entities::dir::Dir;
        for t in [0.2, -0.2, 0.0] {
            let mut style = TextStyle::regular(Pt(12.0));
            style.dir = Some(Dir::RTL);
            style.tracking = Some(Length::pt(t));
            let text = FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: "abc".into(),
                style: style.clone(),
                bidi: None,
            };
            let separator = FrameItem::TextSeparator {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                advance: Pt(3.0),
                style: style.clone(),
                origin: TextSeparatorOrigin::Markup,
                bidi: None,
            };
            assert_eq!(boundary(&[], &style), Pt(0.0));
            assert_eq!(boundary(&[text], &style), Pt(t));
            assert_eq!(boundary(&[separator], &style), Pt(t));
        }
    }
}

#[cfg(test)]
mod lexical_scope_tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::Length,
        style::{Style, Styles},
        style_chain::StyleDelta,
    };
    use comemo::Track;

    #[test]
    fn lexical_scopes_stop_tracking_even_when_resolved_styles_match() {
        let intr = TagIntrospector::empty();
        for t in [0.2, -0.2] {
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                12.0,
                (&intr as &dyn Introspector).track(),
            );
            l.chain = l.chain.push(StyleDelta {
                tracking: Some(Length::pt(t)),
                ..StyleDelta::empty()
            });
            l.style.tracking = Some(Length::pt(t));
            let start = l.regions.current.cursor_x.0;
            let width = super::super::text_measure::word(&l.metrics, "abc", &l.style).0;
            l.layout_content(&Content::text("abc"));
            l.layout_content(&Content::styled(
                Content::text("def"),
                Styles::from_iter([Style::Tracking(Length::pt(t))]),
            ));
            l.layout_content(&Content::text("ghi"));
            assert!(
                (l.regions.current.cursor_x.0 - start - 3.0 * width).abs() < 1e-10,
                "redundant lexical override must interrupt both boundaries"
            );
        }
    }

    #[test]
    fn empty_lexical_scope_preserves_the_previous_textual_boundary() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push(StyleDelta {
            tracking: Some(Length::pt(0.2)),
            ..StyleDelta::empty()
        });
        l.style.tracking = Some(Length::pt(0.2));
        let start = l.regions.current.cursor_x.0;
        let width = super::super::text_measure::word(&l.metrics, "abc", &l.style).0;
        l.layout_content(&Content::text("abc"));
        l.layout_content(&Content::styled(
            Content::Empty,
            Styles::from_iter([Style::Tracking(Length::pt(0.2))]),
        ));
        l.layout_content(&Content::text("def"));
        assert!((l.regions.current.cursor_x.0 - start - 2.0 * width - 0.2).abs() < 1e-10);
    }
    #[test]
    fn checkpoint_restores_the_lexical_owner_of_the_pending_line() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push(StyleDelta {
            tracking: Some(Length::pt(0.2)),
            ..StyleDelta::empty()
        });
        l.style.tracking = Some(Length::pt(0.2));
        let start = l.regions.current.cursor_x.0;
        let width = super::super::text_measure::word(&l.metrics, "abc", &l.style).0;
        l.layout_content(&Content::text("abc"));
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.layout_content(&Content::styled(
            Content::text("def"),
            Styles::from_iter([Style::Tracking(Length::pt(0.2))]),
        ));
        l.restore_flow_suffix_checkpoint(checkpoint);
        l.layout_content(&Content::text("ghi"));
        assert!((l.regions.current.cursor_x.0 - start - 2.0 * width - 0.2).abs() < 1e-10);
    }
}
