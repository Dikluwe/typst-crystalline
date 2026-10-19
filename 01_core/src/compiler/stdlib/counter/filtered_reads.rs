//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/counter/filtered_reads.md
//! @prompt-hash bcbb951c
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
    if !is_filtered_key(&counter.key) {
        return counter_final(counter, ctx, span);
    }
    ctx.mark_filtered_counter_read(&counter.key);
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "can only be used when context is known",
        )]);
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
    #[test]
    fn plain_key_is_not_filtered() {
        assert!(!is_filtered_key(&CounterKey::Str("x".into())));
    }
}
