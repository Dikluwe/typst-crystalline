//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/counter/filtered_reads.md
//! @prompt-hash d842cbb1
//! @layer L1
//! @updated 2026-10-05

use super::*;

/// Resolve `.get()` dentro ou fora de context.
pub(crate) fn is_filtered_key(key: &CounterKey) -> bool {
    matches!(key, CounterKey::Selector(selector)
        if crate::compiler::eval::operators::equality::selector_contains_element(selector))
}

pub(super) fn state_value(values: &[usize]) -> Value {
    Value::Array(values.iter().map(|n| Value::Int(*n as i64)).collect())
}

pub(super) fn resolve_counter_location(
    value: &Value,
    ctx: &EvalContext,
    span: Span,
) -> SourceResult<Value> {
    let result = match value {
        Value::Auto => {
            Ok(ctx.current_location.map(Value::Location).unwrap_or(Value::None))
        }
        Value::Label(label) => Ok(ctx
            .introspector
            .query_by_label(label)
            .map(Value::Location)
            .unwrap_or(Value::None)),
        Value::Str(label) => Ok(ctx
            .introspector
            .query_by_label(&Label(label.to_string()))
            .map(Value::Location)
            .unwrap_or(Value::None)),
        Value::Location(location) => Ok(Value::Location(*location)),
        Value::Selector(selector) => match ctx.introspector.query(selector).as_slice() {
            [location] => Ok(Value::Location(*location)),
            [] => Err(vec![SourceDiagnostic::error(
                span,
                "selector does not match any element",
            )]),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                "selector matches multiple elements",
            )]),
        },
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected label, function, location, selector, or auto, found {}",
                other.type_name()
            ),
        )]),
    };
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterResolve {
            value: value.clone(),
        },
        span,
        result,
    )
}

pub(super) fn filtered_history(
    counter: &Counter,
    ctx: &EvalContext,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Vec<(crate::entities::location::Location, Vec<usize>)>> {
    let history = crate::compiler::introspect::from_tags::resolve_filtered_counter(
        &counter.key,
        &ctx.introspector,
        scopes,
        engine,
        span,
        ctx,
    );
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterFold {
            key: counter.key.clone(),
        },
        span,
        history.clone().map(|events| {
            Value::Array(
                events
                    .iter()
                    .map(|(location, values)| {
                        Value::Array(vec![
                            Value::Location(*location),
                            state_value(values),
                        ])
                    })
                    .collect(),
            )
        }),
    )?;
    history
}

pub(crate) fn counter_get_owned(
    counter: &Counter,
    ctx: &mut EvalContext,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    if !is_filtered_key(&counter.key) {
        return counter_get(counter, ctx, span);
    }
    ctx.mark_filtered_counter_read(&counter.key);
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "counter.get() can only be used inside context",
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "counter.get() requer uma localização de contexto",
        )]);
    };
    counter_at_location_owned(counter, location, ctx, scopes, engine, span)
}

pub(crate) fn counter_at_location_owned(
    counter: &Counter,
    location: crate::entities::location::Location,
    ctx: &mut EvalContext,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    if !is_filtered_key(&counter.key) {
        return counter_at_location_recorded(counter, location, ctx, span);
    }
    ctx.mark_filtered_counter_read(&counter.key);
    let result = filtered_history(counter, ctx, scopes, engine, span).map(|history| {
        state_value(
            history
                .iter()
                .rev()
                .find(|(at, _)| {
                    ctx.introspector.counters.order_key(*at)
                        <= ctx.introspector.counters.order_key(location)
                })
                .map(|(_, values)| values.as_slice())
                .unwrap_or(&[0]),
        )
    });
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterAt {
            key: counter.key.clone(),
            location,
        },
        span,
        result,
    )
}

pub(crate) fn counter_at_owned(
    counter: &Counter,
    label: Label,
    ctx: &mut EvalContext,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    if !is_filtered_key(&counter.key) {
        return counter_at(counter, label, ctx, span);
    }
    ctx.mark_filtered_counter_read(&counter.key);
    let resolved = resolve_counter_location(&Value::Label(label.clone()), ctx, span)?;
    let result = match resolved {
        Value::Location(location) => {
            counter_at_location_owned(counter, location, ctx, scopes, engine, span)
        }
        _ => Ok(Value::Array(Vec::new())),
    };
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterLabelAt {
            key: counter.key.clone(),
            label,
        },
        span,
        result,
    )
}

pub(crate) fn counter_final_owned(
    counter: &Counter,
    ctx: &mut EvalContext,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    let filtered = is_filtered_key(&counter.key);
    if filtered {
        ctx.mark_filtered_counter_read(&counter.key);
    }
    if !ctx.in_context || ctx.current_location.is_none() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "can only be used when context is known",
        )
        .with_hint("try wrapping this in a `context` expression")
        .with_hint(
            "the `context` expression should wrap everything that depends on this function",
        )]);
    }
    if !filtered {
        return counter_final(counter, ctx, span);
    }
    let result = filtered_history(counter, ctx, scopes, engine, span).map(|history| {
        state_value(history.last().map(|(_, values)| values.as_slice()).unwrap_or(&[0]))
    });
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::CounterFinal {
            key: counter.key.clone(),
        },
        span,
        result,
    )
}

pub(crate) fn replay_context_read(
    request: &crate::compiler::eval::ContextReadRequest,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    use crate::compiler::eval::ContextReadRequest as Read;
    let mut scopes = Scopes::new(None);
    match request {
        Read::CounterFold { key } => filtered_history(
            &Counter { key: key.clone() },
            ctx,
            &mut scopes,
            engine,
            span,
        )
        .map(|events| {
            Value::Array(
                events
                    .iter()
                    .map(|(location, values)| {
                        Value::Array(vec![
                            Value::Location(*location),
                            state_value(values),
                        ])
                    })
                    .collect(),
            )
        }),
        Read::CounterAt { key, location } => counter_at_location_owned(
            &Counter { key: key.clone() },
            *location,
            ctx,
            &mut scopes,
            engine,
            span,
        ),
        Read::CounterLabelAt { key, label } => counter_at_owned(
            &Counter { key: key.clone() },
            label.clone(),
            ctx,
            &mut scopes,
            engine,
            span,
        ),
        Read::CounterFinal { key } => counter_final_owned(
            &Counter { key: key.clone() },
            ctx,
            &mut scopes,
            engine,
            span,
        ),
        Read::CounterTotal { key } => {
            if is_filtered_key(key) {
                filtered_history(
                    &Counter { key: key.clone() },
                    ctx,
                    &mut scopes,
                    engine,
                    span,
                )
                .map(|history| {
                    Value::Int(
                        history
                            .last()
                            .and_then(|(_, values)| values.first())
                            .copied()
                            .unwrap_or(0) as i64,
                    )
                })
            } else {
                Ok(Value::Int(
                    page_counter_final_value(&Counter { key: key.clone() }, ctx)
                        .as_deref()
                        .or_else(|| ctx.introspector.counter_final_values(key))
                        .and_then(|values| values.first())
                        .copied()
                        .unwrap_or(0) as i64,
                ))
            }
        }
        Read::CounterResolve { value } => resolve_counter_location(value, ctx, span),
        Read::CounterLegacyAt { key, label } => native_counter_at(
            ctx,
            &Args::from_parts(
                vec![Value::Str(key.clone().into()), Value::Str(label.0.clone().into())],
                indexmap::IndexMap::default(),
                span,
            ),
            engine.world,
            engine.current_file,
        ),
        Read::CounterLegacyFinal { key } => native_counter_final(
            ctx,
            &Args::from_parts(
                vec![Value::Str(key.clone().into())],
                indexmap::IndexMap::default(),
                span,
            ),
            engine.world,
            engine.current_file,
        ),
        _ => unreachable!("counter replay called for another owner"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::location::Location;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Sink,
    };
    use comemo::Track;

    #[derive(Default)]
    struct TestWorld {
        library: Library,
        book: FontBook,
    }

    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn native_element_key() -> CounterKey {
        CounterKey::Selector(Selector::Element {
            function: Func::native("heading", crate::compiler::stdlib::native_heading),
            fields: Vec::new().into(),
        })
    }

    fn seeded_context(key: &CounterKey) -> EvalContext {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let location = Location::from_raw(41);
        ctx.introspector.counters.observe_location(location);
        ctx.introspector
            .counters
            .apply(key.clone(), CounterAction::Set(vec![7]));
        ctx.introspector.counters.record_action(
            location,
            Some(key.clone()),
            CounterAction::Set(vec![7]),
        );
        ctx
    }

    fn forbidden_fold_effect(
        _ctx: &mut EvalContext,
        args: &Args,
        _world: &dyn World,
        _file: FileId,
        _scopes: &mut Scopes<'_>,
        engine: &mut Engine<'_>,
    ) -> SourceResult<Value> {
        engine.sink.warn_diagnostic(SourceDiagnostic::warning(
            args.span,
            "FORBIDDEN_FOLD_EFFECT",
        ));
        Err(vec![SourceDiagnostic::error(args.span, "FORBIDDEN_FOLD_EXECUTED")])
    }

    fn owned_final(
        key: &CounterKey,
        ctx: &mut EvalContext,
        span: Span,
    ) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
        let world = TestWorld::default();
        let route = Route::root().with_id(world.main());
        let mut styles = StyleChain::default_chain();
        let mut rules = std::sync::Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut scopes = Scopes::new(None);
        let result = {
            let mut tracked = sink.track_mut();
            let mut engine = Engine {
                world: &world,
                font_metrics: &FixedMetrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut rules,
                active_guards: &mut guards,
                current_file: world.main(),
                sink: &mut tracked,
            };
            counter_final_owned(
                &Counter { key: key.clone() },
                ctx,
                &mut scopes,
                &mut engine,
                span,
            )
        };
        (result, sink.into_diagnostics())
    }

    fn assert_unavailable_context(key: CounterKey, known: bool, located: bool) {
        let world = TestWorld::default();
        let source = Source::new(world.main(), "counter(\"x\").final()".into())
            .expect("valid Source test fixture");
        let span = Span::from_range(world.main(), 0..source.text().len());
        let mut ctx = seeded_context(&key);
        ctx.in_context = known;
        ctx.current_location = located.then(|| Location::from_raw(42));
        ctx.introspector.counters.record_action(
            Location::from_raw(43),
            Some(key.clone()),
            CounterAction::Func(Func::native_with_engine(
                "forbidden-fold",
                forbidden_fold_effect,
            )),
        );
        let (result, diagnostics) = owned_final(&key, &mut ctx, span);
        assert!(
            diagnostics.is_empty(),
            "rejected read executed a Func effect: {diagnostics:?}"
        );
        assert_eq!(
            ctx.has_filtered_counter_reads(),
            is_filtered_key(&key),
            "demand must be marked before rejection only for native Element keys"
        );
        let errors =
            result.expect_err("materialized data does not grant document context");
        assert_eq!(errors.len(), 1);
        let error = &errors[0];
        assert_eq!(error.severity, Severity::Error);
        assert_eq!(error.message, "can only be used when context is known");
        assert_eq!(error.span, span);
        assert_eq!(source.span_byte_range(error.span), Some(0..source.text().len()));
        assert_eq!(error.hints, vec![
            "try wrapping this in a `context` expression",
            "the `context` expression should wrap everything that depends on this function",
        ]);
        assert!(error.trace.is_empty());
    }

    #[test]
    fn counter_final_owned_rejects_style_only_string_with_materialized_seven() {
        assert_unavailable_context(CounterKey::Str("x".into()), true, false);
    }

    #[test]
    fn counter_final_owned_rejects_style_only_page_with_materialized_seven() {
        assert_unavailable_context(CounterKey::Page, true, false);
    }

    #[test]
    fn counter_final_owned_rejects_style_only_element_before_func_effect() {
        assert_unavailable_context(native_element_key(), true, false);
    }

    #[test]
    fn counter_final_owned_rejects_unknown_context_even_with_location() {
        for key in [CounterKey::Str("x".into()), CounterKey::Page, native_element_key()] {
            assert_unavailable_context(key, false, true);
        }
    }

    #[test]
    fn counter_final_owned_located_context_returns_real_final_seven() {
        for key in [CounterKey::Str("x".into()), CounterKey::Page, native_element_key()] {
            let mut ctx = seeded_context(&key);
            ctx.current_location = Some(Location::from_raw(42));
            let (result, diagnostics) = owned_final(&key, &mut ctx, Span::detached());
            assert_eq!(result.unwrap(), Value::Array(vec![Value::Int(7)]));
            assert!(diagnostics.is_empty());
        }
    }

    #[test]
    fn counter_final_owned_keeps_public_materialized_helper_without_location() {
        for key in [CounterKey::Str("x".into()), CounterKey::Page, native_element_key()] {
            let ctx = seeded_context(&key);
            assert!(ctx.current_location.is_none());
            assert_eq!(
                counter_final(&Counter { key }, &ctx, Span::detached()).unwrap(),
                Value::Array(vec![Value::Int(7)])
            );
        }
    }

    #[test]
    fn plain_key_is_not_filtered() {
        assert!(!is_filtered_key(&CounterKey::Str("x".into())));
    }
}
