//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/classification.md
//! @prompt-hash 0347632d
//! @layer L1
//! @updated 2026-10-07

use super::*;

/// **P162 .E**: emite `Tag::Start`/`Tag::End` em paralelo para os 3 kinds
/// locatable (Heading/Figure/Cite). `label_from_parent` é `Some(label)`
/// quando este nó é descendente directo de um `Content::Labelled` wrapper;
/// `None` caso contrário.
///
/// **P191B (ADR-0071)**: walk fn ganha `intr: &mut TagIntrospector`.
/// **P856** — classifica o body de uma `Content::Label` não-auto que
/// não produziu Tag locatable, determinando a mensagem de erro vanilla.
/// Raw (bloco de código) é detetado mesmo quando embrulhado em Styled
/// ou Sequence; tudo o resto é tratado como Text.
pub(super) fn classify_unreferencable_body(content: &Content) -> UnreferencableKind {
    match content {
        Content::Source(e) => classify_unreferencable_body(&e.body),
        Content::ShowRecipe(e) => classify_unreferencable_body(&e.body),
        Content::Located(e) => classify_unreferencable_body(&e.body),
        Content::Realized(e) => classify_unreferencable_body(&e.original),
        Content::Raw(_) => UnreferencableKind::Raw,
        Content::Equation(_) => UnreferencableKind::EquationWithoutNumbering,
        Content::Styled(styled) => classify_unreferencable_body(&styled.body),
        Content::PageRun(e) => classify_unreferencable_body(&e.body),
        Content::HtmlFrame(e) => classify_unreferencable_body(&e.body),
        Content::Sequence(seq) => {
            for child in seq.iter() {
                let kind = classify_unreferencable_body(child);
                if kind == UnreferencableKind::Raw
                    || kind == UnreferencableKind::EquationWithoutNumbering
                {
                    return kind;
                }
            }
            UnreferencableKind::Text
        }
        // intencional: todo outro elemento não-Raw e não-Equation sem numeração é classificado como Text para mensagens de erro
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::Flush(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::HtmlElem(_)
        | Content::OutlineEntry(_) => UnreferencableKind::Text,
    }
}
