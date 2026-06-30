//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/state.md
//! @prompt-hash c9f3117b
//! @layer L1
//! @updated 2026-06-30
//!
//! `state(key, init)` como valor de primeira classe + métodos `.update()`,
//! `.get()` e `.display()`. P506 — runtime state via `context`.

use ecow::EcoString;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::introspector::Introspector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::state::State;
use crate::entities::state_update::StateUpdate;
use crate::entities::value::Value;
use crate::rules::eval::closures::apply_func;
use crate::rules::eval::EvalContext;
use crate::rules::scopes::Scopes;

use super::err;

/// `state(key, init)` → `Value::State`.
pub fn native_state(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), init] => Ok(Value::State(State {
            key: key.clone(),
            init: Box::new(init.clone()),
        })),
        [other, _] => err(format!(
            "state() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state() requer 2 argumentos (key, init), recebeu {}",
            args.items.len()
        )),
    }
}

/// Resolve `.update(value)` num `Content::StateUpdate`.
pub fn state_update(key: EcoString, value: Value) -> Value {
    Value::Content(Content::state_update(
        key.to_string(),
        StateUpdate::Set(Box::new(value)),
    ))
}

/// Resolve `.get()` dentro ou fora de context.
/// Fora de context: erro descritivo. Dentro: consulta o introspector.
pub fn state_get(state: &State, ctx: &EvalContext, span: Span) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.get() can only be used inside context".to_string(),
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.get() requer uma localização de contexto".to_string(),
        )]);
    };
    let value = ctx
        .introspector
        .state_value(state.key.as_str(), location)
        .unwrap_or(state.init.as_ref())
        .clone();
    Ok(value)
}

/// Resolve `.display([callback])` dentro ou fora de context.
pub fn state_display(
    state: &State,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.display() can only be used inside context".to_string(),
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.display() requer uma localização de contexto".to_string(),
        )]);
    };

    let value = ctx
        .introspector
        .state_value(state.key.as_str(), location)
        .unwrap_or(state.init.as_ref())
        .clone();

    match args.items.as_slice() {
        [] => Ok(Value::Content(value_to_content(&value))),
        [Value::Func(callback)] => {
            let result = apply_func(
                callback.clone(),
                Args::positional(vec![value]),
                scopes,
                ctx,
                engine,
            )?;
            Ok(Value::Content(value_to_content(&result)))
        }
        [other] => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "state.display() requer função como argumento, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "state.display() requer 0 ou 1 argumentos".to_string(),
        )]),
    }
}

/// Converte um `Value` resolvido em `Content` para display.
pub fn value_to_content(value: &Value) -> Content {
    match value {
        Value::Content(c) => c.clone(),
        Value::Str(s) => Content::text(s.clone()),
        Value::Int(i) => Content::text(i.to_string()),
        Value::Float(f) => Content::text(format_float(*f)),
        Value::Bool(b) => Content::text(b.to_string()),
        Value::Array(arr) => {
            let text = arr
                .iter()
                .map(|v| match v {
                    Value::Int(i) => i.to_string(),
                    Value::Str(s) => s.to_string(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join(".");
            Content::text(text)
        }
        _ => Content::Empty,
    }
}

fn format_float(f: f64) -> String {
    if f == f.trunc() {
        format!("{:.1}", f)
    } else {
        f.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU16;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};

    struct NullWorld { library: Library, book: FontBook }
    impl World for NullWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }
    fn null_world() -> NullWorld { NullWorld { library: Library::new(), book: FontBook::new() } }
    fn test_file_id() -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }

    #[test]
    fn native_state_cria_valor() {
        let s = native_state(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Str("x".into()), Value::Int(0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(s, Value::State(_)));
    }

    #[test]
    fn native_state_rejeita_key_nao_string() {
        let r = native_state(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Int(1), Value::Int(0)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
    }
}
