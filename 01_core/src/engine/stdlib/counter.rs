//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/counter.md
//! @prompt-hash 0f49c05c
//! @layer L1
//! @updated 2026-06-30
//!
//! `counter(selector)` como valor de primeira classe + métodos `.update()`,
//! `.step()`, `.get()`, `.display()` e `.at()`. P506 — runtime state via
//! `context`.

use ecow::EcoString;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::counter::Counter;
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::introspector::Introspector;
use crate::entities::label::Label;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::engine::eval::closures::apply_func;
use crate::engine::eval::EvalContext;
use crate::engine::scopes::Scopes;

use super::err;

/// `counter(selector)` → `Value::Counter`.
pub fn native_counter(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::engine::stdlib::{native_figure, native_heading, native_table};
    use std::ptr::fn_addr_eq;

    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => Ok(Value::Counter(Counter { key: key.clone() })),
        [Value::Selector(selector)] => {
            let key = selector_to_key(selector)?;
            Ok(Value::Counter(Counter { key: key.into() }))
        }
        [Value::Func(f)] => {
            let key = f.native_fn_addr().and_then(|addr| {
                if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                    Some("heading")
                } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                    Some("figure")
                } else if fn_addr_eq(addr, native_table as fn(_, _, _, _) -> _) {
                    Some("table")
                } else {
                    None
                }
            });
            match key {
                Some(k) => Ok(Value::Counter(Counter { key: k.into() })),
                None => err(format!(
                    "counter() requer string, selector ou função de elemento, recebeu function"
                )),
            }
        }
        [other] => err(format!(
            "counter() requer string ou selector, recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter() requer 1 argumento, recebeu {}",
            args.items.len()
        )),
    }
}

fn selector_to_key(selector: &Selector) -> SourceResult<String> {
    match selector {
        Selector::Kind(kind) => Ok(kind_to_string(kind)),
        Selector::Label(_) | Selector::Location(_) | Selector::Regex(_) => Err(vec![
            SourceDiagnostic::error(
                Span::detached(),
                "counter() não suporta este tipo de selector".to_string(),
            ),
        ]),
        Selector::And(sels) | Selector::Or(sels) => {
            // Fallback: tenta extrair kind do primeiro selector.
            selector_to_key(sels.first().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    "counter() requer selector não vazio".to_string(),
                )]
            })?)
        }
        Selector::Within { base, .. } => selector_to_key(base),
        Selector::Where { base, .. } => selector_to_key(base),
    }
}

fn kind_to_string(kind: &crate::entities::element_kind::ElementKind) -> String {
    use crate::entities::element_kind::ElementKind;
    match kind {
        ElementKind::Heading => "heading".to_string(),
        ElementKind::Figure => "figure".to_string(),
        ElementKind::Equation => "equation".to_string(),
        ElementKind::Table => "table".to_string(),
        ElementKind::Quote => "quote".to_string(),
        ElementKind::ContextBlock => "context".to_string(),
        _ => format!("{:?}", kind).to_lowercase(),
    }
}

/// Resolve `.update(value)` num `Content::CounterUpdate`.
pub fn counter_update(key: EcoString, value: Value) -> SourceResult<Value> {
    let n = match value {
        Value::Int(i) => i.max(0) as usize,
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "counter.update() requer inteiro, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    Ok(Value::Content(Content::counter_update(
        key.to_string(),
        CounterAction::Update(n),
    )))
}

/// Resolve `.step()` num `Content::CounterUpdate`.
pub fn counter_step(key: EcoString) -> Value {
    Value::Content(Content::counter_update(
        key.to_string(),
        CounterAction::Step,
    ))
}

/// Resolve `.get()` dentro ou fora de context.
pub fn counter_get(counter: &Counter, ctx: &EvalContext, span: Span) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "counter.get() can only be used inside context".to_string(),
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "counter.get() requer uma localização de contexto".to_string(),
        )]);
    };
    let values = ctx
        .introspector
        .counter_values_at(counter.key.as_str(), location)
        .unwrap_or(&[0]);
    Ok(Value::Array(
        values.iter().map(|n| Value::Int(*n as i64)).collect(),
    ))
}

/// Resolve `.display([pattern|callback])` dentro ou fora de context.
pub fn counter_display(
    counter: &Counter,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "counter.display() can only be used inside context".to_string(),
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "counter.display() requer uma localização de contexto".to_string(),
        )]);
    };

    let values = ctx
        .introspector
        .counter_values_at(counter.key.as_str(), location)
        .unwrap_or(&[0]);

    match args.items.as_slice() {
        [] => {
            let text = values
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(".");
            Ok(Value::Content(Content::text(text)))
        }
        [Value::Str(pattern)] => {
            let text = apply_numbering_pattern(values, pattern.as_str());
            Ok(Value::Content(Content::text(text)))
        }
        [Value::Func(callback)] => {
            let arr = Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect());
            let result = apply_func(
                callback.clone(),
                Args::positional(vec![arr]),
                scopes,
                ctx,
                engine,
            )?;
            Ok(Value::Content(super::state::value_to_content(&result)))
        }
        [other] => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "counter.display() requer string ou função como argumento, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "counter.display() requer 0 ou 1 argumentos".to_string(),
        )]),
    }
}

/// Resolve `.at(label)` — pode ser usado dentro ou fora de context.
pub fn counter_at(
    counter: &Counter,
    label: Label,
    ctx: &EvalContext,
    _span: Span,
) -> SourceResult<Value> {
    let values = ctx
        .introspector
        .query_by_label(&label)
        .and_then(|loc| ctx.introspector.counter_values_at(counter.key.as_str(), loc))
        .unwrap_or(&[]);

    Ok(Value::Array(
        values.iter().map(|n| Value::Int(*n as i64)).collect(),
    ))
}

/// Aplica um pattern simples de numbering ao slice de counters.
fn apply_numbering_pattern(values: &[usize], pattern: &str) -> String {
    if values.is_empty() {
        return String::new();
    }
    // Pattern minimal: "1." → prefixa o primeiro valor e acrescenta '.'.
    // Suporta apenas padrões terminados em separador ou literais fixos.
    if pattern.ends_with('.') || pattern.ends_with(')') {
        format!("{}{}", values[0], pattern.chars().last().unwrap_or('.'))
    } else {
        // Substitui o primeiro '1' do pattern pelo valor real.
        pattern.replacen('1', &values[0].to_string(), 1)
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
    fn native_counter_aceita_string() {
        let c = native_counter(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Str("heading".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(c, Value::Counter(_)));
    }

    #[test]
    fn native_counter_rejeita_inteiro() {
        let r = native_counter(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Int(1)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
    }
}
