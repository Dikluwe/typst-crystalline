//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/source.md
//! @prompt-hash dedea926
//! @layer L1
//! @updated 2026-10-07

use super::{Content, SourceContent};
use crate::entities::span::Span;
use std::sync::Arc;

impl Content {
    pub(crate) fn source_body(&self) -> &Content {
        match self {
            Self::Source(e) => e.body.source_body(),
            _ => self,
        }
    }
    pub(crate) fn source_span(&self) -> Span {
        match self {
            Self::Source(e) => e.span,
            Self::TableCell(e) => e.source_span,
            Self::Table(e) => e.source_span,
            Self::ShowRecipe(..)
            | Self::Realized(..)
            | Self::Located(..)
            | Self::Empty
            | Self::Text(..)
            | Self::Space
            | Self::SpaceFromNewline
            | Self::Parbreak
            | Self::Sequence(..)
            | Self::HtmlElem(..)
            | Self::HtmlFrame(..)
            | Self::Par { .. }
            | Self::Heading(..)
            | Self::Title(..)
            | Self::Strong(..)
            | Self::Emph(..)
            | Self::Raw(..)
            | Self::ListItem(..)
            | Self::EnumItem(..)
            | Self::Link(..)
            | Self::Equation(..)
            | Self::MathSequence(..)
            | Self::MathIdent(..)
            | Self::MathText(..)
            | Self::MathFrac(..)
            | Self::MathAttach(..)
            | Self::MathPrimes(..)
            | Self::MathRoot(..)
            | Self::MathStretch(..)
            | Self::MathDelimited(..)
            | Self::MathLr(..)
            | Self::MathMid(..)
            | Self::MathOverbrace(..)
            | Self::MathUnderbrace(..)
            | Self::MathUnderbracket(..)
            | Self::MathUnderparen(..)
            | Self::MathUndershell(..)
            | Self::MathOverbracket(..)
            | Self::MathOverparen(..)
            | Self::MathOvershell(..)
            | Self::MathAlignPoint(..)
            | Self::Linebreak(..)
            | Self::MathMatrix(..)
            | Self::MathCases(..)
            | Self::MathAccent(..)
            | Self::MathCancel(..)
            | Self::MathUnderline(..)
            | Self::MathOverline(..)
            | Self::MathVec(..)
            | Self::MathClassOverride(..)
            | Self::MathLimitsOverride(..)
            | Self::MathUnderover(..)
            | Self::MathOp(..)
            | Self::MathStyled(..)
            | Self::Label(..)
            | Self::Ref(..)
            | Self::CounterDisplay(..)
            | Self::CounterUpdate(..)
            | Self::Outline(..)
            | Self::OutlineEntry(..)
            | Self::Figure(..)
            | Self::FigureCaption(..)
            | Self::Image(..)
            | Self::Shape(..)
            | Self::Curve(..)
            | Self::Transform(..)
            | Self::Grid(..)
            | Self::GridHeader(..)
            | Self::GridFooter(..)
            | Self::GridCell(..)
            | Self::SetPage(..)
            | Self::PageRun(..)
            | Self::Align(..)
            | Self::Place(..)
            | Self::Flush(..)
            | Self::Styled(..)
            | Self::Divider(..)
            | Self::Terms(..)
            | Self::TermItem(..)
            | Self::Quote(..)
            | Self::Document { .. }
            | Self::Asset { .. }
            | Self::SmartQuote(..)
            | Self::PdfAttach(..)
            | Self::PdfArtifact(..)
            | Self::Underline(..)
            | Self::Strike(..)
            | Self::Overline(..)
            | Self::SmallCaps { .. }
            | Self::Pad(..)
            | Self::Hide(..)
            | Self::HSpace(..)
            | Self::VSpace(..)
            | Self::Pagebreak(..)
            | Self::Colbreak(..)
            | Self::Stack(..)
            | Self::Boxed(..)
            | Self::Block(..)
            | Self::Bibliography(..)
            | Self::Cite(..)
            | Self::Footnote(..)
            | Self::FootnoteEntry(..)
            | Self::TableHeader(..)
            | Self::TableFooter(..)
            | Self::GridHLine(..)
            | Self::GridVLine(..)
            | Self::TableHLine(..)
            | Self::TableVLine(..)
            | Self::Repeat(..)
            | Self::Columns(..)
            | Self::Metadata(..)
            | Self::State(..)
            | Self::StateUpdate(..)
            | Self::StateDisplay(..)
            | Self::CounterDisplayCallback(..)
            | Self::ContextBlock(..)
            | Self::Dynamic(..) => Span::detached(),
        }
    }
    pub(crate) fn with_source_span(self, span: Span) -> Self {
        if span.is_detached() || !self.source_span().is_detached() {
            self
        } else {
            Self::Source(Arc::new(SourceContent { body: self, span }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn span(offset: usize) -> Span {
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        Span::from_range(id, offset..offset + 1)
    }
    #[test]
    fn first_source_survives_reference_and_single_traversal() {
        let original = Content::text("a").with_source_span(span(2));
        let alias = original.clone().with_source_span(span(9));
        assert_eq!(alias.source_span(), span(2));
        let mut visits = 0;
        let mapped = alias
            .map_content(&mut |c| {
                visits += 1;
                assert_eq!(c.elem_name(), "text");
                Ok(Some(Content::text("b")))
            })
            .unwrap();
        assert_eq!(visits, 1);
        assert_eq!(mapped.source_span(), span(2));
        assert_eq!(mapped.plain_text(), "b");
    }
    #[test]
    fn origin_is_transparent_to_equality_and_hash() {
        use std::hash::{Hash, Hasher};
        let raw = Content::text("a");
        let sourced = raw.clone().with_source_span(span(2));
        assert_eq!(raw, sourced);
        let hash = |c: &Content| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            c.hash(&mut h);
            h.finish()
        };
        assert_eq!(hash(&raw), hash(&sourced));
        assert!(matches!(raw.with_source_span(Span::detached()), Content::Text(_)));
    }
}
