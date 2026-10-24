//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/figure.md
//! @prompt-hash 3a522b9b
//! @layer L1
//! @updated 2026-06-26

use crate::compiler::lang::figure_supplement::figure_supplement_for_lang;
use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::elements::figure::FigureElem;
use crate::entities::introspector::Introspector;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

#[derive(Clone)]
enum CaptionPrefix {
    Generated(Content),
    Prepared { supplement: Option<Content>, suffix: Content },
}

/// Layout de `figure(...)` (atomização ADR-0109 P378): calcula o prefixo de
/// numeração (gate na chain, número via Introspector) e delega o desenho a
/// `layout_figure`. Content-preserving — era inline no `layout_content`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &FigureElem,
) {
    let (body, caption) = (&e.body, &e.caption);
    let location = layouter.current_location;
    let enabled = !matches!(layouter.chain.custom("figure.numbering"), Some(Value::None));
    let pattern = match layouter.chain.custom("figure.numbering") {
        Some(Value::Str(pattern)) => Some(pattern.to_string()),
        None => Some("1".to_string()),
        Some(Value::None | Value::Func(_)) => None,
        Some(_) => None,
    };
    let caption_prefix = if let Some(kind) = &e.resolved_kind {
        if enabled && caption.is_some() {
            let key =
                crate::compiler::introspect::figure_counters::figure_counter_key(kind);
            let values = location.and_then(|location| {
                layouter
                    .introspector
                    .counter_values_at(&key, location)
                    .map(ToOwned::to_owned)
            });
            let Some(values) = values else {
                layouter.layout_errors.push(
                    crate::entities::source_result::SourceDiagnostic::error(
                        layouter.current_source_span,
                        "figure counter snapshot could not be verified",
                    ),
                );
                return;
            };
            // Function-numbering visual formatting remains the measured separate debt.
            pattern.map(|pattern| {
                let number = format_counter(&values, &pattern).unwrap_or_default();
                let supplement = e
                    .resolved_supplement
                    .as_ref()
                    .and_then(|value| value.as_ref())
                    .cloned();
                layouter
                    .runtime
                    .figure_page_numbers
                    .push(layouter.current_page_number());
                let suffix = Content::text(format!("{number}: "));
                CaptionPrefix::Prepared { supplement, suffix }
            })
        } else {
            None
        }
    } else if let Some(pattern) = pattern {
        // Explicit compatibility boundary for external synthetic, unprepared Figures.
        let kind_key = match e.kind.as_ref().and_then(|kind| kind.as_ref()) {
            Some(crate::entities::elements::figure::FigureKind::Name(name)) => {
                name.as_str()
            }
            Some(crate::entities::elements::figure::FigureKind::Element(_)) | None => {
                "image"
            }
        };
        let progress = layouter.figure_progress.entry(kind_key.to_string()).or_insert(0);
        let index = *progress;
        *progress += 1;
        let number = layouter
            .introspector
            .figure_number_at_index(kind_key, index)
            .unwrap_or(index + 1);
        layouter
            .runtime
            .figure_page_numbers
            .push(layouter.current_page_number());
        let formatted =
            format_counter(&[number], &pattern).unwrap_or_else(|| number.to_string());
        let supplement =
            figure_supplement_for_lang(kind_key, layouter.chain.lang().as_ref());
        Some(CaptionPrefix::Generated(Content::text(format!(
            "{supplement} {formatted}: "
        ))))
    } else {
        None
    };
    match caption_prefix {
        Some(prefix @ CaptionPrefix::Prepared { .. }) => {
            layout_figure_prefix(layouter, body, caption, Some(prefix))
        }
        Some(CaptionPrefix::Generated(prefix)) => {
            layout_figure(layouter, body, caption, Some(prefix))
        }
        None => layout_figure(layouter, body, caption, None),
    }
}

/// Renderiza uma figura com legenda opcional (Passo 62/75, DEBT-14/15).
///
/// `caption_prefix`: prefixo de numeração pré-calculado pelo layouter
/// (ex: "Figura 1: "), ou `None` se a figura não tiver numeração activa.
/// O cálculo acontece em `layout_content` usando `figure_progress` e
/// `counter_state.figure_numbers` — a introspecção pré-computou os números.
pub(super) fn layout_figure<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    caption: &Option<Content>,
    caption_prefix: Option<Content>,
) {
    layout_figure_prefix(
        layouter,
        body,
        caption,
        caption_prefix.map(CaptionPrefix::Generated),
    );
}

fn layout_figure_prefix<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    caption: &Option<Content>,
    caption_prefix: Option<CaptionPrefix>,
) {
    // Figure is a block even when it has no caption. Snapshot validation in
    // layout has already completed before this renderer changes paragraph flow.
    super::parbreak::layout(layouter);

    // 1. Desenhar o corpo da figura.
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::FigureBody,
        body,
    );

    // 2. Desenhar a legenda, se existir.
    if let Some(cap) = caption {
        layouter.layout_content(&Content::linebreak());

        if let Some(prefix) = caption_prefix {
            match prefix {
                CaptionPrefix::Generated(prefix) => layouter.layout_content(&prefix),
                CaptionPrefix::Prepared { supplement, suffix } => {
                    if let Some(supplement) = supplement {
                        if !supplement.is_empty() {
                            layouter.layout_planned_child(crate::compiler::introspect::page_running::PlanEdge::FigureSelectedSupplement, &supplement);
                            layouter.layout_content(&Content::text(" "));
                        }
                    }
                    layouter.layout_content(&suffix);
                }
            }
        }
        layouter.layout_planned_child(
            crate::compiler::introspect::page_running::PlanEdge::FigureCaption,
            cap,
        );
    }
    super::parbreak::layout(layouter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::{LocatedContent, SourceContent},
        elements::figure::FigureKind,
        func::Func,
        introspector::{FigureCounterSnapshot, TagIntrospector},
        location::Location,
        span::Span,
        style::Styles,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn prepared() -> FigureElem {
        FigureElem {
            body: Content::text("body"),
            caption: Some(Content::text("caption")),
            kind: None,
            resolved_kind: Some(FigureKind::Element(Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ))),
            supplement: None,
            resolved_supplement: Some(Some(Content::text("Figure"))),
        }
    }

    #[test]
    fn figure_layout_missing_snapshot_keeps_source_span_and_emits_no_fabricated_prefix() {
        let intr = TagIntrospector::empty();
        let span = Span::from_range(
            crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(4).unwrap(),
            ),
            3..10,
        );
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let content = Content::Source(Arc::new(SourceContent {
            span,
            body: Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(71),
                body: Content::Figure(Arc::new(prepared())),
            })),
        }));
        layouter.layout_content(&content);
        assert_eq!(layouter.layout_errors.len(), 1);
        assert_eq!(
            layouter.layout_errors[0].message,
            "figure counter snapshot could not be verified"
        );
        assert_eq!(layouter.layout_errors[0].span, span);
        assert!(layouter.figure_progress.is_empty());
        let document = layouter.finish();
        assert!(!document.plain_text().contains("body"));
        assert!(!document.plain_text().contains("Figure"));
    }

    #[test]
    fn figure_layout_prepared_location_never_consumes_synthetic_progress() {
        let mut intr = TagIntrospector::empty();
        let location = Location::from_raw(71);
        let mut figure = prepared();
        figure.resolved_supplement = Some(Some(Content::strong(Content::text("Figure"))));
        let key = crate::compiler::introspect::figure_counters::figure_counter_key(
            figure.resolved_kind.as_ref().unwrap(),
        );
        intr.counters.observe_location(location);
        intr.figure_counter_snapshots
            .push(FigureCounterSnapshot { key, history: vec![(location, vec![3])] });
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        layouter.figure_progress.insert("image".into(), 7);
        let content = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::Figure(Arc::new(figure)),
        }));
        let checkpoint = layouter.capture_flow_suffix_checkpoint();
        layouter.layout_content(&content);
        assert_eq!(layouter.figure_progress.get("image"), Some(&7));
        assert!(layouter.layout_errors.is_empty());
        layouter.restore_flow_suffix_checkpoint(checkpoint);
        assert_eq!(layouter.figure_progress.get("image"), Some(&7));
        layouter.layout_content(&content);
        let document = layouter.finish();
        assert!(document
            .plain_text()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .contains("Figure 3: caption"));
        struct BoldSupplement(bool);
        impl crate::entities::layout_types::FrameVisitor for BoldSupplement {
            fn visit_text(&mut self, item: &crate::entities::layout_types::FrameItem) {
                if let crate::entities::layout_types::FrameItem::Text {
                    text,
                    style,
                    ..
                } = item
                {
                    if text.as_str().contains("Figure") && style.bold {
                        self.0 = true;
                    }
                }
            }
        }
        use crate::entities::layout_types::FrameVisitor;
        let mut visitor = BoldSupplement(false);
        for page in &document.pages {
            visitor.visit_items(&page.items);
        }
        assert!(
            visitor.0,
            "resolved supplement retains Strong morphology in emitted text"
        );
    }

    #[test]
    fn figure_layout_returned_located_styled_occurrence_keeps_exact_location() {
        let mut intr = TagIntrospector::empty();
        let location = Location::from_raw(71);
        let figure = prepared();
        let key = crate::compiler::introspect::figure_counters::figure_counter_key(
            figure.resolved_kind.as_ref().unwrap(),
        );
        intr.counters.observe_location(location);
        intr.figure_counter_snapshots
            .push(FigureCounterSnapshot { key, history: vec![(location, vec![3])] });
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let content = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::styled(Content::Figure(Arc::new(figure)), Styles::new()),
        }));
        layouter.layout_content(&content);
        assert_eq!(
            layouter.current_location,
            Some(location),
            "transparent style cannot allocate another Figure identity"
        );
        assert!(layouter.layout_errors.is_empty());
    }
}

#[cfg(test)]
mod paragraph_boundary_tests {
    use super::*;
    use crate::entities::{
        content::LocatedContent,
        elements::figure::FigureKind,
        func::Func,
        introspector::TagIntrospector,
        layout_types::{FrameItem, FrameVisitor},
        location::Location,
    };
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn figure_blocks_keep_adjacent_bodies_and_following_text_in_separate_paragraphs() {
        let intr = TagIntrospector::empty();
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let figure = |text: &str, number: u128| {
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(number),
                body: Content::Figure(Arc::new(FigureElem {
                    body: Content::text(text),
                    caption: None,
                    kind: None,
                    resolved_kind: Some(FigureKind::Element(Func::native(
                        "image",
                        crate::compiler::stdlib::native_image,
                    ))),
                    supplement: None,
                    resolved_supplement: Some(Some(Content::text("Figure"))),
                })),
            }))
        };
        layouter.layout_content(&Content::Sequence(
            vec![figure("ALPHA", 71), figure("BETA", 72), Content::text("TAIL")].into(),
        ));
        assert!(layouter.layout_errors.is_empty());
        let document = layouter.finish();
        struct Lines(Vec<(String, f64)>);
        impl FrameVisitor for Lines {
            fn visit_text(&mut self, item: &FrameItem) {
                if let FrameItem::Text { text, pos, .. } = item {
                    self.0.push((text.to_string(), pos.y.val()));
                }
            }
        }
        let mut lines = Lines(Vec::new());
        for page in &document.pages {
            lines.visit_items(&page.items);
        }
        let y =
            |word: &str| lines.0.iter().find(|(text, _)| text.contains(word)).unwrap().1;
        assert!(
            y("ALPHA") < y("BETA") && y("BETA") < y("TAIL"),
            "each Figure is a block; actual emitted lines: {:?}",
            lines.0
        );
        assert_eq!(
            document.plain_text().split_whitespace().collect::<Vec<_>>(),
            vec!["ALPHA", "BETA", "TAIL"]
        );
    }
}
