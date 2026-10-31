//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/assert.md
//! @prompt-hash 3c9c2be3
//! @layer L1
//! @updated 2026-04-23
//!
//! Função nativa `assert` (+ namespace `assert.eq` / `assert.ne` desde P723).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── `assert()` — prova de fogo dos named args (Passo 66, DEBT-16) ───────────

/// `assert(condition, message: ...)` → sem output; erro se condição for falsa.
///
/// Primeira função com named arg documentado (não apenas tolerado).
/// Prova de que o mecanismo de named args (DEBT-16) funciona de ponta a ponta.
pub fn native_assert(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
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
        Some(other) => {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|items| items.iter().find(|item| item.name.is_none()))
                .map(|item| item.value_span)
                .filter(|span| !span.is_detached())
                .unwrap_or(args.span);
            return Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "assert() requer 1 argumento posicional (condição)".to_string(),
            )])
        }
    };

    // Argumento nomeado: message (opcional).
    // **P843 (F7)** — mensagens verbatim do vanilla (foundations/mod.rs:179-181,
    // medido em `temp/p843/f7_*.typ`): default `assertion failed`;
    // com `message:` → `assertion failed: {msg}` (sem prefixo `message:`,
    // concatenado directo). Antes: `Asserção falhou` / mensagem nua.
    let message = args.named.get("message").map(|v| match v {
        Value::Str(s) => s.to_string(),
        Value::Content(c) => c.plain_text(),
        other => other.type_name().to_string(),
    });

    if !condition {
        let msg = match message {
            Some(m) => format!("assertion failed: {}", m),
            None => "assertion failed".to_string(),
        };
        return Err(vec![SourceDiagnostic::error(args.span, msg)]);
    }

    Ok(Value::None)
}

// ── P723 — `assert.eq` / `assert.ne` (namespace de assert) ──────────────────
//
// Bloqueio real do cetz (shapes.typ:151, anchor.typ:123, boolean.typ:189) —
// a premissa do passo apontava o namespace de `curve`, refutada pela sonda
// (existe desde P513). Semântica medida no vanilla,
// `foundations/mod.rs:198-247`: a mensagem de erro é o observável.

/// Reutiliza a igualdade da linguagem, incluindo coerções e valores aninhados.
fn values_equal(a: &Value, b: &Value) -> bool {
    crate::compiler::eval::operators::equality::values_eq(a, b)
}

/// Validação partilhada de `assert.eq`/`assert.ne`: apenas o named arg
/// `message` é aceite e são obrigatórios 2 argumentos posicionais.
/// Devolve `(left, right, message)`.
fn assert_eq_ne_args<'a>(
    args: &'a Args,
    fname: &str,
) -> SourceResult<(&'a Value, &'a Value, Option<String>)> {
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

    let message = args.named.get("message").map(|v| match v {
        Value::Str(s) => s.to_string(),
        Value::Content(c) => c.plain_text(),
        other => other.type_name().to_string(),
    });

    Ok((&args.items[0], &args.items[1], message))
}

/// `assert.eq(left, right, message: ...)` → none; erro se `left != right`
/// (igualdade da linguagem, com coerção Int↔Float).
///
/// Mensagem default medida no vanilla (foundations/mod.rs:215-219):
/// `equality assertion failed: value {left} was not equal to {right}`
/// com o `repr` da linguagem; com `message:` → `equality assertion failed: {message}`.
pub fn native_assert_eq(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (left, right, message) = assert_eq_ne_args(args, "assert.eq")?;

    if !values_equal(left, right) {
        let msg = match message {
            Some(m) => format!("equality assertion failed: {}", m),
            None => format!(
                "equality assertion failed: value {} was not equal to {}",
                crate::compiler::eval::repr::repr_value(left),
                crate::compiler::eval::repr::repr_value(right),
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
pub fn native_assert_ne(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (left, right, message) = assert_eq_ne_args(args, "assert.ne")?;

    if values_equal(left, right) {
        let msg = match message {
            Some(m) => format!("inequality assertion failed: {}", m),
            None => format!(
                "inequality assertion failed: value {} was equal to {}",
                crate::compiler::eval::repr::repr_value(left),
                crate::compiler::eval::repr::repr_value(right),
            ),
        };
        return Err(vec![SourceDiagnostic::error(args.span, msg)]);
    }

    Ok(Value::None)
}

#[cfg(test)]
mod a1_equality_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

    struct ConditionWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl ConditionWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                )
                .expect("valid assert-condition source"),
            }
        }
    }

    impl World for ConditionWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    #[test]
    fn assert_condition_errors_keep_native_type_and_actual_argument() {
        for (source, token, message) in [
            ("#assert([BODY])", "[BODY]", "expected boolean, found content"),
            ("#assert(1)", "1", "expected boolean, found integer"),
            ("#assert(\"x\")", "\"x\"", "expected boolean, found string"),
        ] {
            let world = ConditionWorld::new(source);
            let errors = crate::compiler::eval::eval_for_test(&world, &world.source)
                .expect_err("invalid assert condition must reject");
            assert_eq!(errors[0].message, message);
            let start = source.find(token).unwrap();
            assert_eq!(
                world.source.span_byte_range(errors[0].span),
                Some(start..start + token.len())
            );
        }
    }

    #[test]
    fn assert_condition_alias_error_points_at_argument_use() {
        let source = "#let x = [BODY]\n#assert(x)";
        let world = ConditionWorld::new(source);
        let errors =
            crate::compiler::eval::eval_for_test(&world, &world.source).unwrap_err();
        assert_eq!(errors[0].message, "expected boolean, found content");
        let start = source.rfind('x').unwrap();
        assert_eq!(world.source.span_byte_range(errors[0].span), Some(start..start + 1));
    }

    #[test]
    fn assert_condition_true_and_false_messages_stay_intact() {
        let world = ConditionWorld::new("#assert(true)\nOK");
        let module = crate::compiler::eval::eval_for_test(&world, &world.source).unwrap();
        assert_eq!(module.content().unwrap().plain_text().trim(), "OK");
        let world = ConditionWorld::new("#assert(false)");
        let errors =
            crate::compiler::eval::eval_for_test(&world, &world.source).unwrap_err();
        assert_eq!(errors[0].message, "assertion failed");
    }

    #[test]
    fn a1_assert_uses_language_relative_and_nested_numeric_equality() {
        let absolute = Value::Length(crate::entities::layout_types::Length::pt(1.0));
        let relative = Value::Relative(crate::entities::rel::Rel {
            rel: 0.0,
            abs: crate::entities::layout_types::Length::pt(1.0),
        });
        assert!(values_equal(&absolute, &relative));
        assert!(values_equal(
            &Value::Array(vec![Value::Int(1)]),
            &Value::Array(vec![Value::Float(1.0)])
        ));
        assert!(!values_equal(&Value::Int(1), &Value::Int(2)));
    }
}
