//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text.md
//! @prompt-hash af3f28be
//! @layer L1
//! @updated 2026-10-24
//!
//! Atomização (ADR-0109, P381): o layout de `Text` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Folha de render —
//! decodifica o `#set text`/`#set par` da chain, faz merge top-wins com o
//! estilo tipado, e dispõe cada palavra via `layout_word`. **Não re-entra
//! `layout_content`** (não orquestra). Caminho quente — content-preserving.

use crate::entities::layout_types::{Pt, TextStyle};

mod style;
pub(super) fn effective_style<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> TextStyle {
    style::effective_style(layouter)
}

/// Resolve an inert lexical scope with the same policy used for real text.
pub(super) fn effective_style_in(
    chain: &crate::entities::style_chain::StyleChain,
    style: &TextStyle,
) -> TextStyle {
    self::style::from_parts(chain, style)
}

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn observe_content_boundary<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    content: &crate::entities::content::Content,
) {
    use crate::entities::content::Content;
    if !matches!(
        content,
        Content::Empty
            | Content::Sequence(..)
            | Content::Styled(..)
            | Content::Strong(..)
            | Content::Emph(..)
            | Content::Source(..)
            | Content::Located(..)
            | Content::Realized(..)
            | Content::ShowRecipe(..)
            | Content::Space
            | Content::SpaceFromNewline
    ) {
        layouter.literal_script_break_pending = false;
    }
}

/// Layout de um nó `Text`: resolve o estilo efectivo (chain tipada vence o
/// render `#set` decodificado da chain `custom`) e dispõe as palavras.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
) {
    layouter.literal_script_break_pending = false;
    let original = layouter.bidi_cursor.as_mut().and_then(|c| c.next());
    super::paragraph_indent::start(layouter);
    let previous_span = layouter.text_bidi.take();
    if text.is_empty() {
        let effective = effective_style(layouter);
        let prev_style = std::mem::replace(&mut layouter.style, effective);
        super::text_baseline::admit(layouter);
        super::bibliography::emit_pending_prefix(layouter);
        layouter.regions.current.current_line.push(
            crate::entities::layout_types::FrameItem::Text {
                pos: crate::entities::layout_types::Point {
                    x: layouter.regions.current.cursor_x,
                    y: layouter.baseline_y(),
                },
                text: "".into(),
                style: layouter.style.clone(),
                bidi: original,
            },
        );
        layouter.style = prev_style;
        layouter.text_bidi = previous_span;
        return;
    }
    if original.is_none()
        && !layouter.style.superscript
        && !layouter.paragraph_planning_suppressed
        && super::paragraph_plan::try_layout(
            layouter,
            &crate::entities::content::Content::text(text),
        )
    {
        return;
    }
    let mut request = style::script_request(layouter);
    let mut runs = if request.superscript && !request.subscript {
        if text.contains(['\n', '\r']) {
            request.superscript_native = Some(false);
        }
        layouter.metrics.text_superscript_runs(text, request.size, &request)
    } else {
        Vec::new()
    };
    if !complete_script_partition(text, &runs) {
        let (effective, horizontal) = style::for_text(layouter, text);
        runs = vec![(0..text.len(), effective, horizontal)];
    }
    for (range, effective, horizontal) in runs {
        if range.start > 0
            && !text[..range.start].ends_with(['\n', '\r'])
            && !text[range.start..].starts_with(['\n', '\r'])
        {
            layouter.regions.current.cursor_x += layouter
                .text_tracking_state
                .source_run_boundary(&layouter.regions.current.current_line, &effective);
        }
        let run_span = original.as_ref().and_then(|s| s.subspan(range.clone()));
        layout_run(layouter, &text[range], run_span, effective, horizontal);
    }
    layouter.text_bidi = previous_span;
}

fn complete_script_partition(
    text: &str,
    runs: &[(std::ops::Range<usize>, TextStyle, Pt)],
) -> bool {
    let mut next = 0;
    if runs.is_empty() {
        return false;
    }
    for (range, style, x) in runs {
        if range.start != next
            || range.end < range.start
            || range.end > text.len()
            || !text.is_char_boundary(range.start)
            || !text.is_char_boundary(range.end)
            || (!text.is_empty() && range.start == range.end)
            || !style.size.0.is_finite()
            || !x.0.is_finite()
            || !style.baseline_offset.resolve_pt(style.size.0).is_finite()
        {
            return false;
        }
        next = range.end;
    }
    next == text.len() && (!text.is_empty() || runs.len() == 1)
}

fn layout_run<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
    original: Option<crate::entities::bidi_context::BidiSpan>,
    effective: TextStyle,
    horizontal: Pt,
) {
    let previous_horizontal =
        std::mem::replace(&mut layouter.text_horizontal_offset, horizontal);
    let prev_style = std::mem::replace(&mut layouter.style, effective.clone());
    let mut offset = 0usize;
    // Literal script breaks belong to layout, so following siblings use the final line.
    let split_breaks = effective.superscript && !effective.subscript;
    let lines = if split_breaks {
        text.split_inclusive('\n').collect::<Vec<_>>()
    } else {
        vec![text]
    };
    for raw_line in lines {
        let has_break = split_breaks && raw_line.ends_with('\n');
        let line = if has_break {
            raw_line
                .strip_suffix('\n')
                .unwrap()
                .strip_suffix('\r')
                .unwrap_or(raw_line.strip_suffix('\n').unwrap())
        } else {
            raw_line
        };
        // **P544/P547** — preservar espaços entre palavras dentro de um nó Text,
        // incluindo espaços iniciais/finais e nós Text compostos só por espaços.
        // `split_whitespace()` perdia esses separadores, juntando palavras e
        // removendo o espaço antes de pontuação. `split(' ')` mantém cada
        // posição de espaço como um segmento vazio; adicionamos `space_width()`
        // entre segmentos e para cada segmento vazio.
        let mut parts = line.split(' ').peekable();
        while let Some(part) = parts.next() {
            layouter.text_bidi =
                original.as_ref().and_then(|s| s.subspan(offset..offset + part.len()));
            if !part.is_empty() {
                layouter.literal_script_break_pending = false;
                if layouter.smallcaps {
                    const SCALE: f64 = 0.8;
                    let chars: Vec<char> = part.chars().collect();
                    let mut i = 0;
                    while i < chars.len() {
                        let is_lower = chars[i].is_lowercase();
                        let mut j = i;
                        while j < chars.len() && chars[j].is_lowercase() == is_lower {
                            j += 1;
                        }
                        let run: String = chars[i..j].iter().collect();
                        if is_lower {
                            let upper = run.to_uppercase();
                            let base_size = layouter.style.size;
                            layouter.style.size = Pt(base_size.0 * SCALE);
                            layouter.layout_chunk(&upper);
                            layouter.style.size = base_size;
                        } else {
                            layouter.layout_chunk(&run);
                        }
                        i = j;
                    }
                } else {
                    layouter.layout_word(part);
                }
            }
            offset += part.len();
            if parts.peek().is_some() {
                layouter.text_bidi =
                    original.as_ref().and_then(|s| s.subspan(offset..offset + 1));
                offset += 1;
                super::space::advance(
                    layouter,
                    crate::entities::layout_types::TextSeparatorOrigin::Literal,
                );
            }
        }

        if has_break {
            offset += raw_line.len() - line.len();
            layouter.style = prev_style.clone();
            let cursor = layouter.bidi_cursor.take();
            super::linebreak::layout(
                layouter,
                &crate::entities::elements::linebreak::LinebreakElem {
                    justify: false,
                    justify_explicit: false,
                },
            );
            layouter.bidi_cursor = cursor;
            layouter.literal_script_break_pending = true;
            layouter.style = effective.clone();
        }
    }
    layouter.style = prev_style;
    layouter.text_horizontal_offset = previous_horizontal;
}

#[cfg(test)]
mod empty_text_geometry_tests {
    use super::super::{FixedMetrics, FontMetrics};
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMargins, Pt},
    };
    use comemo::Track;

    #[test]
    fn explicit_empty_text_establishes_typographic_height_without_ink() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = f64::INFINITY;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.regions.current.width = f64::INFINITY;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        let style = effective_style(&l);
        let top = l.metrics.text_edges(style.size, &style).0 .0;
        l.layout_content(&Content::text(""));
        let page = &l.finish().pages[0];
        assert!((page.height - top).abs() < 1e-9, "height={} top={}", page.height, top);
        assert!(page.plain_text().is_empty());
    }
}

#[cfg(test)]
mod r8_script_geometry_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::FrameItem,
    };
    use comemo::Track;
    #[test]
    fn r8_actual_superscript_uses_injected_metrics_once_and_restores_style() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::superscript(Content::text("1")));
        let item = l
            .regions
            .current
            .current_line
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, style, text, .. } if text.as_str() == "1" => {
                    Some((pos, style))
                }
                _ => None,
            })
            .expect("actual generated marker");
        assert!((item.1.size.0 - 6.6).abs() < 1e-8);
        assert!((item.0.y.0 - l.regions.current.cursor_y.0 + 5.5).abs() < 1e-8);
        assert_eq!(l.style.size, Pt(11.0));
    }
}

#[cfg(test)]
mod r8_horizontal_offset_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::FrameItem,
    };
    use comemo::Track;
    struct OffsetMetrics;
    impl FontMetrics for OffsetMetrics {
        fn advance(&self, t: &str, s: Pt, style: &TextStyle) -> Pt {
            super::super::FixedMetrics.advance(t, s, style)
        }
        fn vertical_metrics(&self, s: Pt, style: &TextStyle) -> (Pt, Pt) {
            super::super::FixedMetrics.vertical_metrics(s, style)
        }
        fn cap_height(&self, s: Pt, style: &TextStyle) -> Pt {
            super::super::FixedMetrics.cap_height(s, style)
        }
        fn text_edges(&self, s: Pt, style: &TextStyle) -> (Pt, Pt) {
            super::super::FixedMetrics.text_edges(s, style)
        }
        fn text_superscript_metrics(
            &self,
            _: &str,
            s: Pt,
            _: &TextStyle,
        ) -> (Pt, Pt, Pt, bool) {
            (s * 0.6, Pt(3.0), s * -0.5, false)
        }
    }
    #[test]
    fn r8_script_horizontal_offset_does_not_advance_following_text() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            OffsetMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::text("BASE"));
        let start = l.regions.current.cursor_x.0;
        l.layout_content(&Content::superscript(Content::text("1")));
        l.layout_content(&Content::text("NEXT"));
        let find = |needle: &str| {
            l.regions
                .current
                .current_line
                .iter()
                .find_map(|i| match i {
                    FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                        Some(pos.x.0)
                    }
                    _ => None,
                })
                .unwrap()
        };
        assert!((find("1") - (start + 3.0)).abs() < 1e-8);
        assert!((find("NEXT") - (start + 6.6 * 0.6)).abs() < 1e-8);
    }
}

#[cfg(test)]
mod r10_source_script_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        elements::linebreak::LinebreakElem,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PagedDocument},
    };
    use comemo::Track;
    use std::sync::Arc;
    fn render(body: Content) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::text("BASE"));
        l.layout_content(&Content::superscript(body));
        l.layout_content(&Content::text("END"));
        l.finish()
    }
    fn positions(doc: &PagedDocument) -> Vec<(String, f64, f64)> {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| {
                if let FrameItem::Text { text, pos, .. } = item {
                    if !text.is_empty() {
                        return Some((text.to_string(), pos.x.0, pos.y.0));
                    }
                }
                None
            })
            .collect()
    }
    fn explicit(parts: &[&str]) -> Content {
        let mut body = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                body.push(Content::Linebreak(Arc::new(LinebreakElem {
                    justify: false,
                    justify_explicit: false,
                })));
            }
            if !part.is_empty() {
                body.push(Content::text(*part));
            }
        }
        Content::sequence(body)
    }
    #[test]
    fn r10_literal_script_newline_participates_in_original_flow() {
        let literal = render(Content::text("123\nZ"));
        let authored = render(explicit(&["123", "Z"]));
        assert_eq!(
            positions(&literal),
            positions(&authored),
            "literal script break must move END and reset Z to the original line origin"
        );
    }
    #[test]
    fn r10_script_consecutive_breaks_preserve_empty_line_causes() {
        for parts in [vec!["123", "", "Z"], vec!["", "123", "Z"], vec!["123", "Z", ""]] {
            assert_eq!(
                positions(&render(Content::text(parts.join("\n")))),
                positions(&render(explicit(&parts))),
                "{parts:?}"
            );
        }
    }
    #[test]
    fn r10_terminal_literal_script_break_preserves_following_markup_space() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::text("BASE"));
        l.layout_content(&Content::superscript(Content::text("123\n")));
        let origin = l.regions.current.line_start_x.0;
        l.layout_content(&Content::Space);
        assert!(
            (l.regions.current.cursor_x.0 - origin - 6.6).abs() < 1e-10,
            "the original outer separator remains after the literal break"
        );
    }
    struct PartitionMetrics {
        broken: bool,
    }
    impl FontMetrics for PartitionMetrics {
        fn advance(&self, t: &str, s: Pt, style: &TextStyle) -> Pt {
            super::super::FixedMetrics.advance(t, s, style)
        }
        fn vertical_metrics(&self, s: Pt, style: &TextStyle) -> (Pt, Pt) {
            super::super::FixedMetrics.vertical_metrics(s, style)
        }
        fn cap_height(&self, s: Pt, style: &TextStyle) -> Pt {
            super::super::FixedMetrics.cap_height(s, style)
        }
        fn text_edges(&self, s: Pt, style: &TextStyle) -> (Pt, Pt) {
            super::super::FixedMetrics.text_edges(s, style)
        }
        fn text_superscript_runs(
            &self,
            t: &str,
            _: Pt,
            style: &TextStyle,
        ) -> Vec<(std::ops::Range<usize>, TextStyle, Pt)> {
            if self.broken {
                return vec![
                    (0..2, style.clone(), Pt(0.0)),
                    (2..t.len() + 1, style.clone(), Pt(0.0)),
                ];
            }
            vec![
                (
                    0..1,
                    TextStyle {
                        size: Pt(6.6),
                        tracking: Some(crate::entities::layout_types::Length::pt(0.12)),
                        ..style.clone()
                    },
                    Pt(0.0),
                ),
                (
                    1..2,
                    TextStyle {
                        size: Pt(7.7),
                        tracking: Some(crate::entities::layout_types::Length::pt(0.14)),
                        ..style.clone()
                    },
                    Pt(0.0),
                ),
            ]
        }
    }
    #[test]
    fn r10_generated_face_boundary_retains_previous_scaled_tracking() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            PartitionMetrics { broken: false },
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::superscript(Content::text("AB")));
        let doc = l.finish();
        let items = positions(&doc);
        let a = items.iter().find(|p| p.0 == "A").unwrap();
        let b = items.iter().find(|p| p.0 == "B").unwrap();
        assert!((b.1-a.1-6.6*0.6-0.12).abs()<1e-10,"a generated face boundary inside one source producer retains exactly its previous glyph interval");
    }
    #[test]
    fn r10_invalid_complete_partition_cannot_partially_admit_source() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            PartitionMetrics { broken: true },
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::text("BASE"));
        l.layout_content(&Content::superscript(Content::text("éZ")));
        l.layout_content(&Content::text("END"));
        assert_eq!(positions(&l.finish()), positions(&render(Content::text("éZ"))));
    }
}
