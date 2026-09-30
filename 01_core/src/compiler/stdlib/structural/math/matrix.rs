//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/matrix.md
//! @prompt-hash 9d2a6246
//! @layer L1
//! @updated 2026-09-30
//!
//! Opções, leitura de linhas e construção semântica de `math.mat`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_matrix::{MathMatrixAugment, MathMatrixExplicit};
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{HAlign, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::cases::math_cases_delim;
use super::vanilla_type_name_class;

#[derive(Debug, Clone)]
pub(crate) struct MathMatrixOptions {
    pub delim: (Option<char>, Option<char>),
    pub align: HAlign,
    pub augment: Option<MathMatrixAugment>,
    pub row_gap: Rel<Length>,
    pub column_gap: Rel<Length>,
    pub explicit: MathMatrixExplicit,
}

impl Default for MathMatrixOptions {
    fn default() -> Self {
        Self {
            delim: (Some('('), Some(')')),
            align: HAlign::Center,
            augment: None,
            row_gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            column_gap: Rel { rel: 0.0, abs: Length::em(0.5) },
            explicit: MathMatrixExplicit::default(),
        }
    }
}

fn math_matrix_rel(value: &Value, span: Span) -> SourceResult<Rel<Length>> {
    match value {
        Value::Relative(value) => Ok(*value),
        Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
        Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected relative length, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

fn math_matrix_offsets(value: &Value, span: Span) -> SourceResult<Vec<i64>> {
    match value {
        Value::Int(value) => Ok(vec![*value]),
        Value::Array(values) => values
            .iter()
            .map(|value| match value {
                Value::Int(value) => Ok(*value),
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected integer, found {}", vanilla_type_name_class(other)),
                )]),
            })
            .collect(),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected integer or array of integers, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

pub(crate) fn math_matrix_augment(
    value: &Value,
    span: Span,
) -> SourceResult<Option<MathMatrixAugment>> {
    match value {
        Value::None => Ok(None),
        Value::Int(offset) => Ok(Some(MathMatrixAugment {
            hline: vec![],
            vline: vec![*offset],
            stroke: None,
        })),
        Value::Dict(dict) => {
            let hline = dict
                .get("hline")
                .map(|value| math_matrix_offsets(value, span))
                .transpose()?
                .unwrap_or_default();
            let vline = dict
                .get("vline")
                .map(|value| math_matrix_offsets(value, span))
                .transpose()?
                .unwrap_or_default();
            let stroke = match dict.get("stroke") {
                None | Some(Value::Auto) => None,
                Some(value) => Some(crate::compiler::stdlib::layout::extract_stroke(
                    value,
                    "math.mat",
                    "augment.stroke",
                )?),
            };
            Ok(Some(MathMatrixAugment { hline, vline, stroke }))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected integer, dictionary, or none, found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

pub(crate) fn apply_math_matrix_option(
    options: &mut MathMatrixOptions,
    name: &str,
    value: &Value,
    span: Span,
    explicit: bool,
) -> SourceResult<()> {
    match name {
        "delim" => {
            options.delim = math_cases_delim(value, span)?;
            options.explicit.delim |= explicit;
        }
        "align" => {
            let Value::Align(align) = value else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected alignment, found {}",
                        vanilla_type_name_class(value)
                    ),
                )]);
            };
            let Some(horizontal) = align.h.filter(|_| align.v.is_none()) else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected start, left, center, right, or end".to_string(),
                )]);
            };
            options.align = horizontal;
            options.explicit.align |= explicit;
        }
        "augment" => {
            options.augment = math_matrix_augment(value, span)?;
            options.explicit.augment |= explicit;
        }
        "gap" => {
            let gap = math_matrix_rel(value, span)?;
            options.row_gap = gap;
            options.column_gap = gap;
            options.explicit.row_gap |= explicit;
            options.explicit.column_gap |= explicit;
        }
        "row-gap" => {
            options.row_gap = math_matrix_rel(value, span)?;
            options.explicit.row_gap |= explicit;
        }
        "column-gap" => {
            options.column_gap = math_matrix_rel(value, span)?;
            options.explicit.column_gap |= explicit;
        }
        other => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )])
        }
    }
    Ok(())
}

fn math_matrix_cell(value: &Value) -> Content {
    match value {
        Value::Content(content) => content.clone(),
        Value::LocatedContent(content, _) => content.content().clone(),
        Value::None => Content::Empty,
        Value::Symbol(symbol) => Content::MathText(symbol.value.clone()),
        other => crate::compiler::stdlib::state::value_to_content(other),
    }
}

fn math_matrix_delimited_row(value: &Value) -> Option<Vec<Content>> {
    let Value::Content(Content::MathDelimited(delimited)) = value else {
        return None;
    };
    if delimited.open != '(' || delimited.close != ')' {
        return None;
    }
    let nodes: Vec<Content> = match &delimited.body {
        Content::MathSequence(nodes) => nodes.iter().cloned().collect(),
        Content::Empty => Vec::new(),
        other => vec![other.clone()],
    };
    let mut cells = Vec::new();
    let mut current = Vec::new();
    for node in nodes {
        let comma = matches!(
            &node,
            Content::MathIdent(text) | Content::MathText(text) if text.as_str() == ","
        );
        if comma {
            cells.push(math_matrix_row_cell(std::mem::take(&mut current)));
        } else {
            current.push(node);
        }
    }
    cells.push(math_matrix_row_cell(current));
    Some(cells)
}

fn math_matrix_row_cell(mut nodes: Vec<Content>) -> Content {
    match nodes.len() {
        0 => Content::Empty,
        1 => nodes.pop().unwrap(),
        _ => Content::MathSequence(nodes.into()),
    }
}

pub(crate) fn math_matrix_content(args: &Args) -> SourceResult<Content> {
    const NAMES: &[&str] = &["delim", "align", "augment", "gap", "row-gap", "column-gap"];
    for name in NAMES {
        let count = args
            .occurrence_sequence()
            .iter()
            .filter(|occurrence| occurrence.name.as_deref() == Some(*name))
            .count();
        if count > 1 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("duplicate argument: {name}"),
            )]);
        }
    }
    for name in args.named.keys() {
        if !NAMES.contains(&name.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }

    let mut options = MathMatrixOptions::default();
    if let Some(value) = args.named.get("gap") {
        apply_math_matrix_option(&mut options, "gap", value, args.span, true)?;
    }
    for name in ["delim", "align", "augment", "row-gap", "column-gap"] {
        if let Some(value) = args.named.get(name) {
            apply_math_matrix_option(&mut options, name, value, args.span, true)?;
        }
    }

    let array_mode = args.items.iter().any(|value| matches!(value, Value::Array(_)));
    let delimited_mode = args
        .items
        .iter()
        .any(|value| math_matrix_delimited_row(value).is_some());
    let mut rows = if array_mode || delimited_mode {
        args.items
            .iter()
            .map(|value| match value {
                Value::Array(values) if array_mode && !delimited_mode => {
                    Ok(values.iter().map(math_matrix_cell).collect::<Vec<_>>())
                }
                _ if delimited_mode && !array_mode => math_matrix_delimited_row(value)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected array, found {}",
                                vanilla_type_name_class(value)
                            ),
                        )]
                    }),
                _ => Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected array, found {}", vanilla_type_name_class(value)),
                )]),
            })
            .collect::<SourceResult<Vec<_>>>()?
    } else {
        vec![args.items.iter().map(math_matrix_cell).collect()]
    };
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    for row in &mut rows {
        row.resize(columns, Content::Empty);
    }

    let validate = |offsets: &[i64], dimension: usize, kind: &str| -> SourceResult<()> {
        if let Some(offset) = offsets
            .iter()
            .find(|offset| offset.unsigned_abs() as usize > dimension)
        {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{kind} offset {offset} exceeds dimension {dimension}"),
            )]);
        }
        Ok(())
    };
    if (array_mode || delimited_mode)
        && !args.span.is_detached()
        && !rows.is_empty()
        && columns > 0
    {
        if let Some(augment) = &options.augment {
            validate(&augment.hline, rows.len(), "horizontal line")?;
            validate(&augment.vline, columns, "vertical line")?;
        }
    }

    Ok(Content::math_matrix_complete(
        rows,
        options.delim,
        options.align,
        options.augment,
        options.row_gap,
        options.column_gap,
        options.explicit,
    ))
}

pub fn native_math_mat(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_matrix_content(args).map(Value::Content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_row_is_padded_and_specific_gap_overrides_shorthand() {
        let mut args = Args::positional(vec![
            Value::Array(vec![Value::Int(1), Value::Int(2)]),
            Value::Array(vec![Value::Int(3)]),
        ]);
        args.named.insert("gap".into(), Value::Length(Length::em(0.3)));
        args.named.insert("row-gap".into(), Value::Length(Length::em(0.7)));

        let Content::MathMatrix(matrix) = math_matrix_content(&args).unwrap() else {
            panic!("math.mat deve construir MathMatrix");
        };
        assert_eq!(matrix.rows.len(), 2);
        assert_eq!(matrix.rows[0].len(), 2);
        assert!(matches!(matrix.rows[1][1], Content::Empty));
        assert_eq!(matrix.row_gap.abs, Length::em(0.7));
        assert_eq!(matrix.column_gap.abs, Length::em(0.3));
        assert!(matrix.explicit.row_gap && matrix.explicit.column_gap);
    }
}
