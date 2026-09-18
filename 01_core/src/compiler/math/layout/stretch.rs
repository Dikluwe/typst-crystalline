//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/stretch.md
//! @prompt-hash ac316b8c
//! @layer L1

use crate::compiler::layout::FontMetrics;
use crate::entities::content::Content;
use crate::entities::elements::math_stretch::MathStretchElem;
use crate::entities::layout_types::{Length, TextStyle};
use crate::entities::rel::Rel;
use crate::entities::value::Value;

use super::{MathBox, MathLayouter};

fn leaf_char(content: &Content) -> Option<char> {
    let text = match content {
        Content::Text(text) | Content::MathText(text) | Content::MathIdent(text) => text,
        Content::MathClassOverride(elem) => return leaf_char(&elem.body),
        Content::MathLimitsOverride(elem) => return leaf_char(&elem.body),
        _ => return None,
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
        _ => None,
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
