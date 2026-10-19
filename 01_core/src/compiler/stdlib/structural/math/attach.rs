//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/attach.md
//! @prompt-hash 772edf90
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.attach` na camada stdlib.

use super::casts::{p1293_math_content, vanilla_type_name_class};
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_attach::MathAttachSlot;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn p1293_math_attach_slot(value: &Value, span: Span) -> SourceResult<MathAttachSlot> {
    match value {
        Value::None => Ok(MathAttachSlot::ExplicitNone),
        Value::Content(content) => Ok(MathAttachSlot::Present(content.clone())),
        Value::LocatedContent(content, _) => {
            Ok(MathAttachSlot::Present(content.content().clone()))
        }
        Value::Str(text) => Ok(MathAttachSlot::Present(Content::MathText(text.clone()))),
        Value::Symbol(symbol) => {
            Ok(MathAttachSlot::Present(Content::MathText(symbol.value.clone())))
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content or none, found {}", vanilla_type_name_class(other)),
        )]),
    }
}

/// Construtor puro canônico de `attach`, compartilhado por sintaxe e função.
pub(crate) fn math_attach_content(
    base: Content,
    t: MathAttachSlot,
    b: MathAttachSlot,
    tl: MathAttachSlot,
    bl: MathAttachSlot,
    tr: MathAttachSlot,
    br: MathAttachSlot,
) -> Content {
    Content::math_attach(base, t, b, tl, bl, tr, br)
}

/// Validação fechada e construção única de `attach` para os dois caminhos.
pub(crate) fn math_attach_from_args(args: &Args) -> SourceResult<Content> {
    let base = match args.items.as_slice() {
        [base] => p1293_math_content(base, args.span)?,
        [] if args.named.contains_key("base") => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "the argument `base` is positional".to_string(),
            )
            .with_hint("try removing `base:`")])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: base".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    let mut t = MathAttachSlot::Omitted;
    let mut b = MathAttachSlot::Omitted;
    let mut tl = MathAttachSlot::Omitted;
    let mut bl = MathAttachSlot::Omitted;
    let mut tr = MathAttachSlot::Omitted;
    let mut br = MathAttachSlot::Omitted;
    for (name, value) in &args.named {
        match name.as_str() {
            "t" => t = p1293_math_attach_slot(value, args.span)?,
            "b" => b = p1293_math_attach_slot(value, args.span)?,
            "tl" => tl = p1293_math_attach_slot(value, args.span)?,
            "bl" => bl = p1293_math_attach_slot(value, args.span)?,
            "tr" => tr = p1293_math_attach_slot(value, args.span)?,
            "br" => br = p1293_math_attach_slot(value, args.span)?,
            "base" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "the argument `base` is positional".to_string(),
                )
                .with_hint("try removing `base:`")])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    Ok(math_attach_content(base, t, b, tl, bl, tr, br))
}

/// `math.attach(base, t:, b:, tl:, bl:, tr:, br:)`.
pub fn native_math_attach(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_attach_from_args(args).map(Value::Content)
}
