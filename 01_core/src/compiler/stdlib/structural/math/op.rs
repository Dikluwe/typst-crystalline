//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/op.md
//! @prompt-hash 728b2bac
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `math.op` na camada stdlib.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `op(text, limits: false)` — emite `Content::MathOp { text, limits }`.
/// Text posicional obrigatório; `limits` named opcional (default `false`).
///
/// **P1030** — passou a `native_with_engine` para poder ler
/// `custom("math.op.limits")` da chain quando o argumento explícito está
/// ausente (`#set math.op(limits: true)`, `eval.md` §P1030). É o **consumidor**
/// a ler a chain — a alternativa (o dispatcher alimentar a native) está
/// rejeitada desde P365, que a removeu em nome de fonte única.
pub fn native_op(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    let text = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "op() text espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "op() exige text como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "limits" permitido.
    for k in args.named.keys() {
        if k.as_str() != "limits" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(): argumento nomeado inesperado '{}' (válido: limits)", k),
            )]);
        }
    }

    let limits = match args.named.get("limits") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(limits:) espera bool, recebeu {}", other.type_name()),
            )])
        }
        // **P1030** — sem argumento explícito, a chain decide; sem chain, o
        // default `false`. Precedência arg > chain > default.
        None => match engine.styles.custom("math.op.limits") {
            Some(Value::Bool(b)) => *b,
            _ => false,
        },
    };

    Ok(Value::Content(Content::math_op(text, limits)))
}

pub(super) fn op_value(text: &str, limits: bool) -> Value {
    Value::Content(Content::math_op(Content::text(text), limits))
}
