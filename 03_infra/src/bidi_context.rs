//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/bidi_context.md
//! @prompt-hash 7c078bce
//! @layer L3
use std::{ops::Range, sync::Arc};
use typst_core::entities::{
    bidi_context::{BidiSpan, ParagraphBidiInput, ParagraphBidiSegment},
    dir::Dir,
    layout_types::{FrameItem, PagedDocument},
};
use unicode_bidi::{BidiInfo, Level, ParagraphBidiInfo};
struct Entry {
    input: Arc<ParagraphBidiInput>,
    text: String,
    segments: Vec<Range<usize>>,
}
pub(crate) struct Registry {
    entries: Vec<Entry>,
}
pub(crate) struct Resolved<'a> {
    registry: &'a Registry,
    infos: Vec<BidiInfo<'a>>,
}
impl Registry {
    pub(crate) fn new(doc: &PagedDocument) -> Self {
        fn collect(items: &[FrameItem], inputs: &mut Vec<Arc<ParagraphBidiInput>>) {
            for item in items {
                if let Some(span) = span(item) {
                    if !inputs.iter().any(|p| Arc::ptr_eq(p, span.input())) {
                        inputs.push(span.input().clone());
                    }
                }
                match item {
                    FrameItem::Group { items, .. }
                    | FrameItem::Link { items, .. }
                    | FrameItem::Semantic { items, .. }
                    | FrameItem::TextReplacement { items, .. } => collect(items, inputs),
                    _ => {}
                }
            }
        }
        let mut inputs = Vec::new();
        for page in &doc.pages {
            collect(&page.background, &mut inputs);
            collect(&page.items, &mut inputs);
            collect(&page.foreground, &mut inputs);
        }
        Self::inputs(inputs)
    }
    fn inputs(inputs: Vec<Arc<ParagraphBidiInput>>) -> Self {
        let entries = inputs
            .into_iter()
            .map(|input| {
                let mut text = String::new();
                let mut segments = Vec::new();
                for segment in input.segments() {
                    match segment {
                        ParagraphBidiSegment::Text { text: original, dir } => {
                            let embedded = *dir != input.base_dir();
                            if embedded {
                                text.push(if *dir == Dir::RTL {
                                    '\u{202b}'
                                } else {
                                    '\u{202a}'
                                });
                            }
                            let start = text.len();
                            text.push_str(original);
                            segments.push(start..text.len());
                            if embedded {
                                text.push('\u{202c}');
                            }
                        }
                        ParagraphBidiSegment::Space { .. } => {
                            let start = text.len();
                            text.push(' ');
                            segments.push(start..text.len());
                        }
                        ParagraphBidiSegment::Linebreak => {
                            let start = text.len();
                            text.push('\n');
                            segments.push(start..text.len());
                        }
                    }
                }
                Entry { input, text, segments }
            })
            .collect();
        Self { entries }
    }
    pub(crate) fn resolve(&self) -> Resolved<'_> {
        Resolved {
            registry: self,
            infos: self
                .entries
                .iter()
                .map(|e| {
                    BidiInfo::new(
                        &e.text,
                        Some(if e.input.base_dir() == Dir::RTL {
                            Level::rtl()
                        } else {
                            Level::ltr()
                        }),
                    )
                })
                .collect(),
        }
    }
}
pub(crate) fn span(item: &FrameItem) -> Option<&BidiSpan> {
    match item {
        FrameItem::Text { bidi, .. }
        | FrameItem::TextSeparator { bidi, .. }
        | FrameItem::TextShaped { bidi, .. } => bidi.as_ref(),
        _ => None,
    }
}
impl Resolved<'_> {
    fn index(&self, span: &BidiSpan) -> Option<usize> {
        self.registry
            .entries
            .iter()
            .position(|e| Arc::ptr_eq(&e.input, span.input()))
    }
    pub(crate) fn mapped(&self, span: &BidiSpan) -> Option<Range<usize>> {
        let e = &self.registry.entries[self.index(span)?];
        let segment = e.segments.get(span.segment())?;
        let r = span.range();
        Some(segment.start + r.start..segment.start + r.end)
    }
    pub(crate) fn matches(&self, span: &BidiSpan, text: &str) -> bool {
        self.index(span)
            .and_then(|i| {
                self.mapped(span).and_then(|r| self.registry.entries[i].text.get(r))
            })
            .is_some_and(|s| s == text)
    }
    pub(crate) fn line_runs(
        &self,
        span: &BidiSpan,
        range: Range<usize>,
    ) -> Option<Vec<(Range<usize>, bool)>> {
        let info = &self.infos[self.index(span)?];
        let para = info
            .paragraphs
            .iter()
            .find(|p| p.range.start <= range.start && range.end <= p.range.end)?;
        // Keep the levels resolved from the complete input. The local view
        // performs line-end reordering without cloning unrelated paragraphs.
        let local = ParagraphBidiInfo {
            text: info.text.get(range.clone())?,
            original_classes: info.original_classes[range.clone()].to_vec(),
            levels: info.levels[range.clone()].to_vec(),
            paragraph_level: para.level,
            is_pure_ltr: false,
        };
        let (levels, runs) = local.visual_runs(0..local.text.len());
        Some(
            runs.into_iter()
                .map(|r| {
                    let rtl = levels[r.start].is_rtl();
                    (range.start + r.start..range.start + r.end, rtl)
                })
                .collect(),
        )
    }
    pub(crate) fn runs(
        &self,
        span: &BidiSpan,
        text: &str,
    ) -> Option<Vec<(Range<usize>, bool)>> {
        if !self.matches(span, text) {
            return None;
        }
        let mapped = self.mapped(span)?;
        Some(
            self.line_runs(span, mapped.clone())?
                .into_iter()
                .map(|(r, rtl)| (r.start - mapped.start..r.end - mapped.start, rtl))
                .collect(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ecow::EcoString;
    #[test]
    fn numeric_slice_uses_complete_paragraph_and_virtual_controls_stay_auxiliary() {
        let input = Arc::new(
            ParagraphBidiInput::new(
                Dir::RTL,
                vec![ParagraphBidiSegment::Text {
                    text: EcoString::from("ABC 123 456 789 مرحبا"),
                    dir: Dir::RTL,
                }],
            )
            .unwrap(),
        );
        let span = BidiSpan::new(input.clone(), 0, 8..15).unwrap();
        let registry = Registry::inputs(vec![input]);
        let resolved = registry.resolve();
        assert_eq!(resolved.runs(&span, "456 789").unwrap(), vec![(0..7, false)]);
        let embedded = Arc::new(
            ParagraphBidiInput::new(
                Dir::RTL,
                vec![
                    ParagraphBidiSegment::Text { text: "ABC 123".into(), dir: Dir::LTR },
                    ParagraphBidiSegment::Text {
                        text: "مرحبا".into(), dir: Dir::RTL
                    },
                ],
            )
            .unwrap(),
        );
        let span = BidiSpan::new(embedded.clone(), 0, 0..7).unwrap();
        let registry = Registry::inputs(vec![embedded]);
        let resolved = registry.resolve();
        assert!(resolved.matches(&span, "ABC 123"));
        assert_eq!(resolved.runs(&span, "ABC 123").unwrap(), vec![(0..7, false)]);
    }
    #[test]
    fn explicit_nonjustified_break_resets_unicode_paragraph_but_soft_wrap_does_not() {
        let input = Arc::new(
            ParagraphBidiInput::new(
                Dir::RTL,
                vec![
                    ParagraphBidiSegment::Text { text: "ABC 123".into(), dir: Dir::RTL },
                    ParagraphBidiSegment::Linebreak,
                    ParagraphBidiSegment::Text {
                        text: "456 789 مرحبا".into(),
                        dir: Dir::RTL,
                    },
                ],
            )
            .unwrap(),
        );
        let span = BidiSpan::new(input.clone(), 2, 0..7).unwrap();
        let registry = Registry::inputs(vec![input]);
        let resolved = registry.resolve();
        assert_eq!(
            resolved.runs(&span, "456 789").unwrap(),
            vec![(4..7, false), (3..4, true), (0..3, false)]
        );
    }
}
