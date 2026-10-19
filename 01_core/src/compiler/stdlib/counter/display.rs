//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/counter/display.md
//! @prompt-hash a36cf120
//! @layer L1
//! @updated 2026-10-05

use super::filtered_reads::filtered_history;
use super::*;

/// Resolve `.display([pattern|callback])` dentro ou fora de context.
pub(super) fn render(
    counter: &Counter,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    ctx.mark_filtered_counter_read(&counter.key);
    let requested = args.named.get("at").unwrap_or(&Value::Auto);
    if matches!(requested, Value::Auto) {
        if !ctx.in_context {
            return Err(vec![SourceDiagnostic::error(
                span,
                "counter.display() can only be used inside context",
            )]);
        }
        if ctx.current_location.is_none() {
            return Err(vec![SourceDiagnostic::error(
                span,
                "counter.display() requer uma localização de contexto",
            )]);
        }
    }
    let location = match resolve_counter_location(requested, ctx, span)? {
        Value::Location(location) => location,
        _ => crate::entities::location::Location::from_raw(0),
    };

    if let Some((name, _)) = args
        .named
        .iter()
        .find(|(name, _)| name.as_str() != "at" && name.as_str() != "both")
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let both = match args.named.get("both") {
        None => false,
        Some(Value::Bool(value)) => *value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };

    let history = if is_filtered_key(&counter.key) {
        Some(filtered_history(counter, ctx, scopes, engine, span)?)
    } else {
        None
    };
    let page_values = page_counter_value(counter, location, ctx);
    let mut values = page_values
        .as_deref()
        .unwrap_or_else(|| match &history {
            Some(history) => history
                .iter()
                .rev()
                .find(|(at, _)| {
                    ctx.introspector.counters.order_key(*at)
                        <= ctx.introspector.counters.order_key(location)
                })
                .map(|(_, state)| state.as_slice())
                .unwrap_or(&[0]),
            None => ctx
                .introspector
                .counter_values_at(&counter.key, location)
                .unwrap_or(&[0]),
        })
        .to_vec();
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterAt {
            key: counter.key.clone(),
            location,
        },
        span,
        Ok(state_value(&values)),
    )?;
    if both {
        let total = match &history {
            Some(history) => history
                .last()
                .and_then(|(_, state)| state.first())
                .copied()
                .unwrap_or(0),
            None => page_counter_final_value(counter, ctx)
                .as_deref()
                .or_else(|| ctx.introspector.counter_final_values(&counter.key))
                .and_then(|state| state.first())
                .copied()
                .unwrap_or(0),
        };
        ctx.observe_context_read(
            crate::compiler::eval::ContextReadRequest::CounterTotal {
                key: counter.key.clone(),
            },
            span,
            Ok(Value::Int(total as i64)),
        )?;
        values.push(total);
    }

    match args.items.as_slice() {
        [] | [Value::Auto] => {
            // **P844** (achado #52 de P831) — sem padrão explícito, usa
            // o numbering activo do contexto para o counter (medido no
            // vanilla 0.15.0: `#set heading(numbering: "1.")` →
            // `counter(heading).display()` renderiza "1."). O pattern
            // viaja na chain como custom `"{key}.numbering.pattern"`
            // (canal `rules.rs`, mesmo mecanismo do próprio heading).
            // Sem pattern na chain, mantém o join pré-P844.
            let pattern_key = match &counter.key {
                CounterKey::Str(s) => format!("{}.numbering.pattern", s),
                CounterKey::Selector(selector) => match selector {
                    Selector::Kind(kind) => {
                        format!("{:?}.numbering.pattern", kind).to_lowercase()
                    }
                    Selector::NativeElement(..)
                    | Selector::Element { .. }
                    | Selector::Label(..)
                    | Selector::Location(..)
                    | Selector::And(..)
                    | Selector::Or(..)
                    | Selector::Regex(..)
                    | Selector::Where { .. }
                    | Selector::Within { .. }
                    | Selector::Before { .. }
                    | Selector::After { .. } => "counter.numbering.pattern".to_string(),
                },
                CounterKey::Page => "page.numbering.pattern".to_string(),
            };
            let pattern = engine.styles.custom(&pattern_key).and_then(|v| match v {
                Value::Str(s) => Some(s.to_string()),
                _ => None,
            });
            match pattern {
                Some(p) => {
                    let numbers: Vec<u32> = values.iter().map(|&n| n as u32).collect();
                    let text = super::super::numbering::format_pattern(
                        engine, span, &p, &numbers,
                    )?;
                    Ok(Value::Content(Content::text(text)))
                }
                None => {
                    let text = values
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(".");
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
            let text = super::super::numbering::format_pattern(
                engine,
                span,
                pattern.as_str(),
                &numbers,
            )?;
            Ok(Value::Content(Content::text(text)))
        }
        [Value::Func(callback)] => {
            let result = apply_func(
                callback.clone(),
                Args::positional(values.iter().map(|n| Value::Int(*n as i64)).collect()),
                scopes,
                ctx,
                engine,
            )?;
            Ok(Value::Content(super::super::state::value_to_content(&result)))
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_entrypoint_has_static_signature() {
        let _: fn(
            &Counter,
            &Args,
            &mut Scopes<'_>,
            &mut EvalContext,
            &mut Engine<'_>,
            Span,
        ) -> SourceResult<Value> = render;
    }
}

#[cfg(test)]
mod batch_f_tests {
    use super::*;

    #[derive(Default)]
    struct TestWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for TestWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            test_file()
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }
    fn test_file() -> crate::entities::file_id::FileId {
        crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    fn with_engine<R>(
        f: impl FnOnce(&mut crate::entities::engine::Engine<'_>) -> R,
    ) -> R {
        use comemo::Track;
        let world = TestWorld::default();
        let route = crate::entities::world_types::Route::root();
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let mut rules = std::sync::Arc::from([]);
        let mut guards = Vec::new();
        let mut sink_local = crate::entities::sink::Sink::new();
        let mut sink = sink_local.track_mut();
        let mut engine = crate::entities::engine::Engine {
            world: &world,
            font_metrics: &crate::compiler::layout::FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: test_file(),
            sink: &mut sink,
        };
        f(&mut engine)
    }

    #[test]
    fn batch_f_counter_numbering_keeps_kind_and_composite_keys_distinct() {
        use crate::entities::element_kind::ElementKind;
        use crate::entities::location::Location;
        let kind = Selector::Kind(ElementKind::Heading);
        let composite = Selector::Where {
            base: Box::new(kind.clone()),
            field: "level".into(),
            value: Box::new(Value::Int(2)),
        };
        for (key, expected) in [
            (CounterKey::Str("named".into()), "S2"),
            (CounterKey::Selector(kind), "H2"),
            (CounterKey::Selector(composite), "G2"),
        ] {
            with_engine(|engine| {
                *engine.styles = engine
                    .styles
                    .push_custom("named.numbering.pattern", Value::Str("S1".into()))
                    .push_custom("heading.numbering.pattern", Value::Str("H1".into()))
                    .push_custom("counter.numbering.pattern", Value::Str("G1".into()));
                let location = Location::from_raw(12);
                let mut ctx = EvalContext::new();
                ctx.in_context = true;
                ctx.current_location = Some(location);
                ctx.introspector.counters.apply_at(
                    key.clone(),
                    crate::entities::counter_update::CounterUpdate::Set(vec![2]),
                    location,
                );
                let counter = Counter { key: key.clone() };
                let result = render(
                    &counter,
                    &Args::positional(vec![]),
                    &mut Scopes::new(None),
                    &mut ctx,
                    engine,
                    Span::detached(),
                )
                .unwrap();
                let Value::Content(content) = result else {
                    panic!("expected content");
                };
                assert_eq!(content.plain_text(), expected);
                assert_eq!(counter.key, key);
            });
        }
    }
    #[test]
    fn batch_f_counter_context_error_precedes_named_validation() {
        with_engine(|engine| {
            let mut args = Args::positional(vec![]);
            args.named.insert("bad".into(), Value::Bool(true));
            let errors = render(
                &Counter { key: CounterKey::Str("x".into()) },
                &args,
                &mut Scopes::new(None),
                &mut EvalContext::new(),
                engine,
                Span::detached(),
            )
            .unwrap_err();
            assert_eq!(
                errors[0].message,
                "counter.display() can only be used inside context"
            );
        });
    }
}
