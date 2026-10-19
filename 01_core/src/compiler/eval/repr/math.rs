//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/repr/math.md
//! @prompt-hash 0e285a06
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn repr_math_attach(
    attach: &crate::entities::elements::math_attach::MathAttachElem,
) -> String {
    let mut fields = vec![format!("base: {}", repr_p1293_math_field(&attach.base))];
    for (name, slot) in [
        ("t", &attach.t),
        ("b", &attach.b),
        ("tl", &attach.tl),
        ("bl", &attach.bl),
        ("tr", &attach.tr),
        ("br", &attach.br),
    ] {
        match slot {
            MathAttachSlot::Omitted => {}
            MathAttachSlot::ExplicitNone => fields.push(format!("{name}: none")),
            MathAttachSlot::Present(content) => {
                fields.push(format!("{name}: {}", repr_p1293_math_field(content)))
            }
        }
    }
    format!("attach{}", pretty_array_like(&fields, false))
}

/// Projeção textual canônica de uma folha matemática direta nos campos
/// morfológicos fechados por P1292/P1293. O payload permanece matemático;
/// apenas a representação pública usa a forma de conteúdo `[texto]`.
pub(super) fn repr_p1293_math_field(content: &Content) -> String {
    match content {
        Content::MathIdent(text) | Content::MathText(text) => {
            repr_content(&Content::Text(text.clone()))
        }
        other => repr_content(other),
    }
}

pub(super) fn repr_frac_field(content: &Content) -> String {
    repr_p1293_math_field(content)
}

/// Projeção do body público de `limits`/`scripts`. A sintaxe matemática
/// pode envolver uma folha textual num `MathSequence`; morfologicamente ela
/// continua sendo conteúdo, como a chamada qualificada que recebe `[x]`.
pub(super) fn repr_math_limits_body(content: &Content) -> String {
    match content {
        Content::MathSequence(nodes)
            if nodes.iter().all(|node| {
                matches!(node, Content::MathIdent(_) | Content::MathText(_))
            }) =>
        {
            repr_content(&Content::text(content.plain_text()))
        }
        other => repr_p1293_math_field(other),
    }
}

pub(super) fn repr_math_lr_body(content: &Content) -> String {
    match content {
        Content::MathSequence(nodes) => format!(
            "sequence({})",
            nodes.iter().map(repr_p1293_math_field).collect::<Vec<_>>().join(", ")
        ),
        other => repr_p1293_math_field(other),
    }
}

pub(super) fn repr_canonical_binom(
    delimited: &crate::entities::elements::math_delimited::MathDelimitedElem,
) -> Option<String> {
    if (delimited.open, delimited.close) != ('(', ')') {
        return None;
    }
    let Content::MathFrac(frac) = &delimited.body else {
        return None;
    };
    if frac.line {
        return None;
    }
    let lower = match &frac.den {
        Content::MathSequence(denominator) => {
            if denominator.is_empty() || denominator.len() % 2 == 0 {
                return None;
            }
            let mut lower = Vec::with_capacity(denominator.len().div_ceil(2));
            for (index, content) in denominator.iter().enumerate() {
                if index % 2 == 0 {
                    lower.push(repr_p1293_math_field(content));
                } else if !matches!(content, Content::MathText(text) if text.as_str() == ", ")
                {
                    return None;
                }
            }
            lower
        }
        denominator => vec![repr_p1293_math_field(denominator)],
    };
    let fields = [
        format!("upper: {}", repr_p1293_math_field(&frac.num)),
        format!("lower: {}", pretty_array_like(&lower, lower.len() == 1)),
    ];
    Some(format!("binom{}", pretty_array_like(&fields, false)))
}

pub(super) fn repr_math_matrix(
    matrix: &crate::entities::elements::math_matrix::MathMatrixElem,
) -> String {
    fn delimiter(value: Option<char>) -> String {
        value.map_or_else(|| "none".to_string(), |value| format!("\"{value}\""))
    }
    fn offsets(values: &[i64]) -> String {
        let fields = values.iter().map(ToString::to_string).collect::<Vec<_>>();
        pretty_array_like(&fields, values.len() == 1)
    }
    fn relative(value: &Rel<Length>) -> String {
        format!(
            "{} + {}",
            format_float_with_unit(value.rel * 100.0, "%"),
            repr_length(&value.abs)
        )
    }
    fn sequence_cell(content: &Content) -> String {
        match content {
            Content::MathIdent(text) | Content::MathText(text) | Content::Text(text) => {
                text.to_string()
            }
            Content::MathAlignPoint(_) => " & ".to_string(),
            Content::MathSequence(nodes) => nodes.iter().map(sequence_cell).collect(),
            other => repr_content(other),
        }
    }
    fn cell(content: &Content) -> String {
        match content {
            Content::MathSequence(nodes) => nodes.iter().map(sequence_cell).collect(),
            Content::MathIdent(text) | Content::MathText(text) => text.to_string(),
            other => repr_content(other),
        }
    }

    let mut fields = Vec::new();
    if matrix.explicit.delim {
        fields.push(format!(
            "delim: ({}, {})",
            delimiter(matrix.delim.0),
            delimiter(matrix.delim.1)
        ));
    }
    if matrix.explicit.align {
        let align = match matrix.align {
            HAlign::Start => "start",
            HAlign::Left => "left",
            HAlign::Center => "center",
            HAlign::Right => "right",
            HAlign::End => "end",
        };
        fields.push(format!("align: {align}"));
    }
    if matrix.explicit.augment {
        let value = match &matrix.augment {
            None => "none".to_string(),
            Some(augment) => {
                if augment.hline.is_empty()
                    && augment.vline.len() == 1
                    && augment.stroke.is_none()
                {
                    augment.vline[0].to_string()
                } else {
                    format!(
                        "(hline: {}, vline: {}, stroke: {})",
                        offsets(&augment.hline),
                        offsets(&augment.vline),
                        augment
                            .stroke
                            .as_ref()
                            .map_or_else(|| "auto".to_string(), repr_stroke)
                    )
                }
            }
        };
        fields.push(format!("augment: {value}"));
    }
    if matrix.explicit.row_gap {
        fields.push(format!("row-gap: {}", relative(&matrix.row_gap)));
    }
    if matrix.explicit.column_gap {
        fields.push(format!("column-gap: {}", relative(&matrix.column_gap)));
    }
    let rows = matrix
        .rows
        .iter()
        .map(|row| {
            let cells = row.iter().map(cell).collect::<Vec<_>>();
            pretty_array_like(&cells, cells.len() == 1)
        })
        .collect::<Vec<_>>();
    fields.push(format!("rows: {}", pretty_array_like(&rows, rows.len() == 1)));
    format!("mat({})", fields.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_text_field_uses_content_morphology() {
        assert_eq!(repr_p1293_math_field(&Content::MathText("x".into())), "[x]");
    }
}
