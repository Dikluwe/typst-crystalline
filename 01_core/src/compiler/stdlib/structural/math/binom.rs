//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/binom.md
//! @prompt-hash 009984e6
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.binom` na camada stdlib.

use super::casts::p1293_math_content;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Construtor puro canônico da fração sem barra de `binom`.
pub(crate) fn math_binom_content(upper: Content, lower: Vec<Content>) -> Content {
    debug_assert!(!lower.is_empty());
    let mut denominator =
        Vec::with_capacity(lower.len().saturating_mul(2).saturating_sub(1));
    for (index, item) in lower.into_iter().enumerate() {
        if index > 0 {
            denominator.push(Content::MathText(", ".into()));
        }
        denominator.push(item);
    }
    Content::math_delimited(
        '(',
        Content::math_frac_unlined(
            upper,
            Content::MathSequence(std::sync::Arc::from(denominator)),
        ),
        ')',
    )
}

/// Validação fechada e construção única de `binom` para os dois caminhos.
pub(crate) fn math_binom_from_args(args: &Args) -> SourceResult<Content> {
    let Some((upper, lower_values)) = args.items.split_first() else {
        if args.named.contains_key("upper") {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "the argument `upper` is positional".to_string(),
            )
            .with_hint("try removing `upper:`")]);
        }
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: upper".to_string(),
        )]);
    };
    if lower_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "missing argument: lower".to_string(),
        )]);
    }
    if let Some(name) = args.named.keys().next() {
        if name.as_str() == "upper" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "the argument `upper` is positional".to_string(),
            )
            .with_hint("try removing `upper:`")]);
        }
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let upper = p1293_math_content(upper, args.span)?;
    let lower = lower_values
        .iter()
        .map(|value| p1293_math_content(value, args.span))
        .collect::<SourceResult<Vec<_>>>()?;
    Ok(math_binom_content(upper, lower))
}

/// `math.binom(upper, ..lower)` com pelo menos um lower.
pub fn native_math_binom(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_binom_from_args(args).map(Value::Content)
}
