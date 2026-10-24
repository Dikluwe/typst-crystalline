//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/rust_hash.md
//! @prompt-hash a7632e40
//! @layer L1
//! @updated 2026-10-07

use super::*;

// P204B (M8): impl Hash via hash_content (existing Debug-based hash
// function from P162). Necessária para `#[comemo::track]` no trait
// `Introspector` per ADR-0073 — métodos como `headings_for_toc` que
// retornam `&[(Label, Option<String>, Content, usize)]` exigem `Content: Hash`.
// Estratégia: delega ao hash_content u128 que já existe, hashing-o
// como tuple no hasher genérico.
impl std::hash::Hash for Content {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Source(e) => e.body.hash(state),
            Self::ShowRecipe(e) => e.body.hash(state),
            Self::Located(e) => e.body.hash(state),
            Self::Realized(e) => e.visual.hash(state),
            // `Content` has a deliberately compact `Debug` implementation,
            // therefore the historical Debug-based fallback cannot observe
            // the causal fields of this carrier.
            Self::MathCases(elem) => {
                std::mem::discriminant(self).hash(state);
                elem.hash(state);
            }
            Self::MathMatrix(elem) => {
                std::mem::discriminant(self).hash(state);
                elem.hash(state);
            }
            Self::Empty
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
            | Self::Symbol(..)
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
            | Self::TableCell(..)
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
            | Self::Table(..)
            | Self::Repeat(..)
            | Self::Columns(..)
            | Self::Metadata(..)
            | Self::State(..)
            | Self::StateUpdate(..)
            | Self::StateDisplay(..)
            | Self::CounterDisplayCallback(..)
            | Self::ContextBlock(..)
            | Self::Dynamic(..) => {
                crate::entities::content_hash::hash_content(self).hash(state)
            }
        }
    }
}

#[cfg(test)]
mod rust_hash_tests {
    use super::Content;
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn equal_space_variants_have_equal_hashes() {
        fn hash(content: &Content) -> u64 {
            let mut state = DefaultHasher::new();
            content.hash(&mut state);
            state.finish()
        }
        assert_eq!(hash(&Content::Space), hash(&Content::SpaceFromNewline));
    }
}
