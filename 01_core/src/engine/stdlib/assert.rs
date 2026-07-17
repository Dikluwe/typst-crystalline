//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/assert.md
//! @prompt-hash e4321396
//! @layer L1
//! @updated 2026-04-23
//!
//! Função nativa `assert` (+ namespace `assert.eq` / `assert.ne` desde P723).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::engine::eval::EvalContext;

// ── `assert()` — prova de fogo dos named args (Passo 66, DEBT-16) ───────────

/// `assert(condition, message: ...)` → sem output; erro se condição for falsa.
///
/// Primeira função com named arg documentado (não apenas tolerado).
/// Prova de que o mecanismo de named args (DEBT-16) funciona de ponta a ponta.
pub fn native_assert(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    // Validar named args: apenas "message" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "message" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado: '{}'", key),
            )]);
        }
    }

    // Argumento posicional: condição (obrigatório).
    let condition = match args.items.first() {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("assert() requer condição booleana, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            args.span,
            "assert() requer 1 argumento posicional (condição)".to_string(),
        )]),
    };

    // Argumento nomeado: message (opcional).
    let message = args.named.get("message")
        .map(|v| match v {
            Value::Str(s)     => s.to_string(),
            Value::Content(c) => c.plain_text(),
            other             => other.type_name().to_string(),
        })
        .unwrap_or_else(|| "Asserção falhou".to_string());

    if !condition {
        return Err(vec![SourceDiagnostic::error(args.span, message)]);
    }

    Ok(Value::None)
}

// ── P723 — `assert.eq` / `assert.ne` (namespace de assert) ──────────────────
//
// Bloqueio real do cetz (shapes.typ:151, anchor.typ:123, boolean.typ:189) —
// a premissa do passo apontava o namespace de `curve`, refutada pela sonda
// (existe desde P513). Semântica medida no vanilla,
// `foundations/mod.rs:198-247`: a mensagem de erro é o observável.

/// Igualdade de valor com a coerção Int↔Float do `==` da linguagem
/// (duplicado de `operators.rs::value_eq` — mesmo padrão já usado em
/// `color.rs`; o helper de operators é privado ao seu módulo).
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
        (a, b) => a == b,
    }
}

/// Validação partilhada de `assert.eq`/`assert.ne`: apenas o named arg
/// `message` é aceite e são obrigatórios 2 argumentos posicionais.
/// Devolve `(left, right, message)`.
fn assert_eq_ne_args<'a>(args: &'a Args, fname: &str) -> SourceResult<(&'a Value, &'a Value, Option<String>)> {
    for key in args.named.keys() {
        if key.as_str() != "message" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{}'", key),
            )]);
        }
    }

    if args.items.len() != 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}() requer 2 argumentos posicionais", fname),
        )]);
    }

    let message = args.named.get("message")
        .map(|v| match v {
            Value::Str(s)     => s.to_string(),
            Value::Content(c) => c.plain_text(),
            other             => other.type_name().to_string(),
        });

    Ok((&args.items[0], &args.items[1], message))
}

/// `assert.eq(left, right, message: ...)` → none; erro se `left != right`
/// (igualdade da linguagem, com coerção Int↔Float).
///
/// Mensagem default medida no vanilla (foundations/mod.rs:215-219):
/// `equality assertion failed: value {left} was not equal to {right}`
/// com o `repr` da linguagem; com `message:` → `equality assertion failed: {message}`.
pub fn native_assert_eq(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    let (left, right, message) = assert_eq_ne_args(args, "assert.eq")?;

    if !values_equal(left, right) {
        let msg = match message {
            Some(m) => format!("equality assertion failed: {}", m),
            None => format!(
                "equality assertion failed: value {} was not equal to {}",
                crate::engine::eval::repr::repr_value(left),
                crate::engine::eval::repr::repr_value(right),
            ),
        };
        return Err(vec![SourceDiagnostic::error(args.span, msg)]);
    }

    Ok(Value::None)
}

/// `assert.ne(left, right, message: ...)` → none; erro se `left == right`
/// (igualdade da linguagem, com coerção Int↔Float).
///
/// Mensagem default medida no vanilla (foundations/mod.rs:232-236):
/// `inequality assertion failed: value {left} was equal to {right}`;
/// com `message:` → `inequality assertion failed: {message}`.
pub fn native_assert_ne(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId) -> SourceResult<Value> {
    let (left, right, message) = assert_eq_ne_args(args, "assert.ne")?;

    if values_equal(left, right) {
        let msg = match message {
            Some(m) => format!("inequality assertion failed: {}", m),
            None => format!(
                "inequality assertion failed: value {} was equal to {}",
                crate::engine::eval::repr::repr_value(left),
                crate::engine::eval::repr::repr_value(right),
            ),
        };
        return Err(vec![SourceDiagnostic::error(args.span, msg)]);
    }

    Ok(Value::None)
}
