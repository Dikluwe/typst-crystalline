use typst_core::entities::content::Content;

use crate::fixtures;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Text,
    Math,
    Structure,
    Layout,
    External,
}

fn family(content: &Content) -> Family {
    match content {
        Content::Source(content) => family(&content.body),
        Content::ShowRecipe(content) => family(&content.body),
        Content::Realized(content) => family(&content.original),
        Content::Located(content) => family(&content.body),
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::Par { .. }
        | Content::Styled(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Linebreak(_)
        | Content::SmallCaps { .. } => Family::Text,

        Content::Equation(_)
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
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_) => Family::Math,

        Content::Heading(_)
        | Content::Title(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_) => Family::Structure,

        Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::SmartQuote(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::Table(_)
        | Content::TableCell(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Repeat(_)
        | Content::Columns(_) => Family::Layout,

        Content::Document { .. }
        | Content::Asset { .. }
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_) => Family::External,
    }
}

pub fn assert_catalog_families() {
    let catalog = fixtures::representative();
    let families =
        [Family::Text, Family::Math, Family::Structure, Family::Layout, Family::External];

    for expected in families {
        assert!(
            catalog.iter().any(|content| family(content) == expected),
            "Content benchmark catalog has no fixture for {expected:?}"
        );
    }
}
