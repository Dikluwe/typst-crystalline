//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/stretch.md
//! @prompt-hash a7144035
//! @layer L1

use crate::compiler::layout::FontMetrics;
use crate::entities::content::Content;
use crate::entities::elements::math_stretch::MathStretchElem;
use crate::entities::layout_types::{Length, TextStyle};
use crate::entities::rel::Rel;
use crate::entities::value::Value;

use super::{MathBox, MathLayouter};

fn leaf_char(content: &Content) -> Option<char> {
    let text = match content.source_body() {
        Content::Text(text) | Content::MathText(text) | Content::MathIdent(text) => text,
        Content::MathClassOverride(elem) => return leaf_char(&elem.body),
        Content::MathLimitsOverride(elem) => return leaf_char(&elem.body),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
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
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
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
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
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
        | Content::Dynamic(_) => return None,
    };
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

fn cascaded_size(value: &Value) -> Option<Rel<Length>> {
    match value {
        Value::Relative(size) => Some(*size),
        Value::Ratio(ratio) => Some(Rel { rel: ratio.get(), abs: Length::ZERO }),
        Value::Length(length) => Some(Rel { rel: 0.0, abs: *length }),
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None,
    }
}

pub(super) fn layout<M: FontMetrics>(
    layouter: &MathLayouter<'_, M>,
    elem: &MathStretchElem,
    style: &TextStyle,
) -> MathBox {
    let natural = layouter.layout_node(&elem.body, style);
    let Some(c) = leaf_char(&elem.body) else { return natural };

    let size = if elem.size_explicit {
        elem.size
    } else {
        layouter
            .lexical_styles
            .and_then(|styles| styles.custom("math.stretch.size"))
            .and_then(cascaded_size)
            .unwrap_or(elem.size)
    };

    let horizontal = !layouter.metrics.horizontal_glyph_variants(c, style).is_empty()
        || !layouter.metrics.horizontal_glyph_assembly(c, style).is_empty();
    let vertical = !layouter.metrics.vertical_glyph_variants(c, style).is_empty()
        || !layouter.metrics.vertical_glyph_assembly(c, style).is_empty();
    if horizontal == vertical {
        return natural;
    }

    let reference = if horizontal { natural.width } else { natural.height() };
    let target_pt = size.resolve(Length::pt(reference)).resolve_pt(style.size.val());
    let target_du = target_pt * layouter.constants.upem / style.size.val().max(0.001);
    if horizontal {
        layouter.layout_stretchy_glyph_horizontal(c, target_du, style, 0.0)
    } else {
        layouter.layout_stretchy_glyph_vertical(c, target_du, style)
    }
}
