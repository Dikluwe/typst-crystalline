//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/counter.md
//! @prompt-hash 2b2815c2
//! @layer L1
//! @updated 2026-08-13
//!
//! `counter(selector)` como valor de primeira classe + métodos `.update()`,
//! `.step()`, `.get()`, `.display()` e `.at()`. P506 — runtime state via
//! `context`.
//! **P1018** — `CounterKey` enum (`Page | Selector | Str`); conversão de
//! strings/selectors/funções nativas para a chave canónica.

use crate::compiler::eval::call_dispatch::apply_func;
use crate::compiler::eval::EvalContext;
use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::counter::Counter;
use crate::entities::counter::CounterKey;
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::introspector::Introspector;
use crate::entities::label::Label;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::err;

/// `counter(selector)` → `Value::Counter`.
pub fn native_counter(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::compiler::stdlib::{
        native_figure, native_footnote, native_heading, native_table,
    };
    use std::ptr::fn_addr_eq;

    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => Ok(Value::Counter(Counter { key: CounterKey::Str(key.clone()) })),
        [Value::Selector(selector)] => {
            let key = selector_to_key(selector)?;
            Ok(Value::Counter(Counter { key }))
        }
        [Value::Func(f)] => {
            let key = f.native_fn_addr().and_then(|addr| {
                if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                    Some(CounterKey::Selector(Selector::Kind(ElementKind::Heading)))
                } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                    Some(CounterKey::Selector(Selector::Kind(ElementKind::Figure)))
                } else if fn_addr_eq(addr, native_table as fn(_, _, _, _) -> _) {
                    Some(CounterKey::Selector(Selector::Kind(ElementKind::Table)))
                } else if fn_addr_eq(addr, native_footnote as fn(_, _, _, _) -> _) {
                    // **P1016** — `Content::Footnote` locatable; counter
                    // `Selector(Kind(Footnote))`. Paridade vanilla:
                    // `counter(footnote).get()` devolve o número.
                    Some(CounterKey::Selector(Selector::Kind(ElementKind::Footnote)))
                } else {
                    None
                }
            });
            match key {
                Some(k) => Ok(Value::Counter(Counter { key: k })),
                None => err(format!(
                    "counter() requer string, selector ou função de elemento, recebeu function"
                )),
            }
        }
        [other] => err(format!(
            "counter() requer string ou selector, recebeu {}",
            other.type_name()
        )),
        _ => err(format!("counter() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

fn selector_to_key(selector: &Selector) -> SourceResult<CounterKey> {
    match selector {
        Selector::Kind(kind) => Ok(CounterKey::Selector(Selector::Kind(*kind))),
        Selector::Label(_) | Selector::Location(_) | Selector::Regex(_) => {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "counter() não suporta este tipo de selector".to_string(),
            )])
        }
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

/// Resolve `.update(value)` num `Content::CounterUpdate`.
pub fn counter_update(key: CounterKey, value: Value) -> SourceResult<Value> {
    let n = match value {
        Value::Int(i) => i.max(0) as usize,
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("counter.update() requer inteiro, recebeu {}", other.type_name()),
            )])
        }
    };
    Ok(Value::Content(Content::counter_update(key, CounterAction::Update(n))))
}

/// Resolve `.step()` num `Content::CounterUpdate`.
pub fn counter_step(key: CounterKey) -> Value {
    Value::Content(Content::counter_update(key, CounterAction::Step))
}

/// Resolve `.get()` dentro ou fora de context.
pub fn counter_get(
    counter: &Counter,
    ctx: &EvalContext,
    span: Span,
) -> SourceResult<Value> {
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
        .counter_values_at(&counter.key, location)
        .unwrap_or(&[0]);
    Ok(Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect()))
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
        .counter_values_at(&counter.key, location)
        .unwrap_or(&[0]);

    match args.items.as_slice() {
        [] => {
            // **P844** (achado #52 de P831) — sem padrão explícito, usa
            // o numbering activo do contexto para o counter (medido no
            // vanilla 0.15.0: `#set heading(numbering: "1.")` →
            // `counter(heading).display()` renderiza "1."). O pattern
            // viaja na chain como custom `"{key}.numbering.pattern"`
            // (canal `rules.rs`, mesmo mecanismo do próprio heading).
            // Sem pattern na chain, mantém o join pré-P844.
            let pattern_key = match &counter.key {
                CounterKey::Str(s) => format!("{}.numbering.pattern", s),
                CounterKey::Selector(Selector::Kind(k)) => format!("{:?}.numbering.pattern", k).to_lowercase(),
                CounterKey::Page => "page.numbering.pattern".to_string(),
                _ => "counter.numbering.pattern".to_string(),
            };
            let pattern = engine
                .styles
                .custom(&pattern_key)
                .and_then(|v| match v {
                    Value::Str(s) => Some(s.to_string()),
                    _ => None,
                });
            match pattern {
                Some(p) => {
                    let numbers: Vec<u32> = values.iter().map(|&n| n as u32).collect();
                    let text = super::numbering::format_pattern(engine, span, &p, &numbers)?;
                    Ok(Value::Content(Content::text(text)))
                }
                None => {
                    let text =
                        values.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(".");
                    Ok(Value::Content(Content::text(text)))
                }
            }
        }
        [Value::Str(pattern)] => {
            // **P844** (achado #53 de P831) — partilha o algoritmo real
            // de numbering (`format_pattern`, P793 em
            // `stdlib/numbering.rs`; extraído de `structural.rs` no P847):
            // estilos romano/alfabético/
            // circled, descarte de tokens extra e repetição do último
            // token — paridade vanilla medida (`II B ii ② 2` para
            // counter=2). Substitui o stub "Pattern minimal".
            let numbers: Vec<u32> = values.iter().map(|&n| n as u32).collect();
            let text =
                super::numbering::format_pattern(engine, span, pattern.as_str(), &numbers)?;
            Ok(Value::Content(Content::text(text)))
        }
        [Value::Func(callback)] => {
            let arr =
                Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect());
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
        .and_then(|loc| ctx.introspector.counter_values_at(&counter.key, loc))
        .unwrap_or(&[]);

    Ok(Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect()))
}

/// **P844** (achado #50 de P831) — Resolve `.at(location)` —
/// paridade vanilla `Counter::at` com `Location` directa
/// (`introspection/counter.rs:452`). Medido no vanilla 0.15.0
/// (`temp/p844/a4_counter_at_location.typ`): após 3 headings com
/// numbering, `counter(heading).at(here())` → `(3,)`; sem updates
/// prévios → `(0,)` (fallback `[0]`, como `counter_get`).
pub fn counter_at_location(
    counter: &Counter,
    location: crate::entities::location::Location,
    ctx: &EvalContext,
) -> SourceResult<Value> {
    let values = ctx
        .introspector
        .counter_values_at(&counter.key, location)
        .unwrap_or(&[0]);
    Ok(Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect()))
}

/// **P844** (achado #49 de P831) — Resolve `.final()` dentro de
/// context — paridade vanilla `Counter::final`: array de inteiros com
/// os valores no fim do documento; counter nunca tocado → `(0,)`
/// (medido no vanilla 0.15.0: `counter(heading).final()` num documento
/// sem headings com numbering → `(0,)`).
pub fn counter_final(counter: &Counter, ctx: &EvalContext, span: Span) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "can only be used when context is known".to_string(),
        )]);
    }
    let values = ctx
        .introspector
        .counter_final_values(&counter.key)
        .unwrap_or(&[0]);
    Ok(Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect()))
}

/// Aplica um pattern simples de numbering ao slice de counters.
///
/// **P844** — removido o stub "Pattern minimal" que aqui existia:
/// `counter.display(pattern)` usa agora
/// `super::numbering::format_pattern` (P793), partilhado com
/// `numbering()`.

// ── Nativas globais absorvidas de `foundations.rs` no Passo 1032 ────────────

/// `counter_display(key, [callback])` — render-mediated counter display. P241.
pub fn native_counter_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => Ok(Value::Content(
            crate::entities::content::Content::counter_display_callback(key.to_string(), None),
        )),
        [Value::Str(key), Value::Func(callback)] => Ok(Value::Content(
            crate::entities::content::Content::counter_display_callback(
                key.to_string(),
                Some(callback.clone()),
            ),
        )),
        [Value::Str(_), other] => err(format!(
            "counter_display() requer função como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        [other, ..] => err(format!(
            "counter_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_at(key_str, label_str)` — valor do counter `key` na
/// `Location` associada à `label_str`. P177.
pub fn native_counter_at(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;

    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label_str)] => {
            let label = Label(label_str.to_string());
            let counter_key = CounterKey::Str(key.clone());
            let formatted = ctx
                .introspector
                .query_by_label(&label)
                .and_then(|loc| ctx.introspector.formatted_counter_at(&counter_key, loc))
                .unwrap_or_default();
            Ok(Value::Str(formatted.into()))
        }
        [_, other] => err(format!(
            "counter_at() requer string como segundo argumento (label), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_final(key_str)` — consulta o valor final do counter `key`
/// no `Introspector` da iteração de fixpoint anterior. P176.
pub fn native_counter_final(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;

    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let counter_key = CounterKey::Str(key.clone());
            let formatted =
                ctx.introspector.formatted_counter(&counter_key).unwrap_or_default();
            Ok(Value::Str(formatted.into()))
        }
        [other] => err(format!(
            "counter_final() requer string como argumento, recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_final() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_step(key)` — emite `Content::CounterUpdate` com ação Step. P210B.
pub fn native_counter_step(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::content::Content;
    use crate::entities::counter_update::CounterUpdate as CounterAction;

    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let content = Content::counter_update(key.to_string(), CounterAction::Step);
            Ok(Value::Content(content))
        }
        [other] => err(format!(
            "counter_step() requer string como argumento (key), \
             recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_step() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
    }
    fn null_world() -> NullWorld {
        NullWorld { library: Library::new(), book: FontBook::new() }
    }
    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

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

    /// **P1016** — `counter(footnote)` resolve para `CounterKey::Selector(Kind(Footnote))`.
    /// Antes deste passo errava com *"requer string, selector ou função de
    /// elemento"*; o vanilla devolve o contador
    /// (`#context counter(footnote).get()` → `(2,)` com duas notas).
    #[test]
    fn p1016_native_counter_aceita_funcao_footnote() {
        use crate::compiler::stdlib::native_footnote;
        use crate::entities::func::Func;

        let c = native_counter(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Func(Func::native(
                "footnote",
                native_footnote,
            ))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        match c {
            Value::Counter(counter) => {
                assert_eq!(
                    counter.key,
                    CounterKey::Selector(Selector::Kind(ElementKind::Footnote))
                );
            }
            other => panic!("esperado Value::Counter, recebido {other:?}"),
        }
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
