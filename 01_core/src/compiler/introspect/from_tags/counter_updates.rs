//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/from_tags/counter_updates.md
//! @prompt-hash 44f0eea7
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Resolve o log inteiro demandado. O prefixo é uma projeção posterior,
/// portanto um callback inválido depois do ponto consultado também falha.
pub(crate) fn resolve_filtered_counter(
    key: &crate::entities::counter::CounterKey,
    intr: &TagIntrospector,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    span: crate::entities::span::Span,
    observer: &EvalContext,
) -> SourceResult<Vec<(crate::entities::location::Location, Vec<usize>)>> {
    use crate::compiler::eval::operators::equality::counter_keys_eq;
    use crate::entities::counter::CounterKey;
    use crate::entities::value::Value;
    let automatic = match key {
        CounterKey::Selector(selector) => intr.query(selector),
        _ => Vec::new(),
    };
    let mut events: Vec<_> = intr
        .counters
        .actions()
        .iter()
        .filter(|event| match &event.key {
            Some(manual) => counter_keys_eq(manual, key),
            None => automatic.contains(&event.location),
        })
        .collect();
    events.sort_by_key(|event| event.location.as_u128());
    let mut state = vec![0usize];
    let mut history = Vec::new();
    for event in events {
        match &event.action {
            CounterUpdate::Step(level) => {
                let level = level.get();
                state.resize(level, 0);
                state[level - 1] += 1;
            }
            CounterUpdate::Set(values) => state = values.clone(),
            CounterUpdate::Func(function) => {
                let mut context = EvalContext::new();
                context.target = observer.target;
                context.features = observer.features.clone();
                // Context::none: lexicais são da closure; introspecção não é
                // herdada do corpo que demandou a sequência.
                let arguments = Args::from_parts(
                    state.iter().map(|n| Value::Int(*n as i64)).collect(),
                    indexmap::IndexMap::default(),
                    span,
                );
                let value = apply_func(
                    function.clone(),
                    arguments,
                    scopes,
                    &mut context,
                    engine,
                )?;
                state = match value {
                    Value::Int(n) if n >= 0 => vec![n as usize],
                    Value::Array(values) => values.into_iter().map(|value| match value {
                        Value::Int(n) if n >= 0 => Ok(n as usize),
                        other => Err(vec![crate::entities::source_result::SourceDiagnostic::error(span,
                            format!("counter update function returned {} instead of integer", other.type_name()))]),
                    }).collect::<SourceResult<Vec<_>>>()?,
                    other => return Err(vec![crate::entities::source_result::SourceDiagnostic::error(span,
                        format!("counter update function returned {} instead of integer or array", other.type_name()))]),
                };
            }
        }
        history.push((event.location, state.clone()));
    }
    Ok(history)
}

/// P1148 — aplica callbacks de `counter.update` na mesma fase pós-walk dos
/// callbacks de state. O argumento é o estado corrente como array; o retorno
/// deve ser inteiro não-negativo ou array de inteiros não-negativos.
pub fn apply_counter_funcs(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    let mut scopes = Scopes::new(None);
    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        let ElementPayload::CounterUpdate { key, action: CounterUpdate::Func(func) } =
            &info.payload
        else {
            continue;
        };

        if matches!(key, crate::entities::counter::CounterKey::Selector(selector)
            if crate::compiler::eval::operators::equality::selector_contains_element(selector))
        {
            continue;
        }
        apply_counter_func_at(key, func, *loc, intr, &mut scopes, engine, ctx)?;
    }
    Ok(())
}

/// No bundle, o walk puro já registrou Set/Step, mas Func só pode ser
/// avaliada com Engine. Reconstruir exclusivamente Page evita inserir
/// snapshots antigos após a fronteira do body seguinte.
pub(crate) fn apply_bundle_counter_funcs(
    tags: &[Tag],
    bodies: &[(crate::entities::location::Location, std::ops::Range<usize>)],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::entities::counter::CounterKey;

    intr.counters.clear_page_snapshots();
    let mut scopes = Scopes::new(None);
    for (boundary, range) in bodies {
        intr.counters
            .apply_at(CounterKey::Page, CounterUpdate::Set(vec![1]), *boundary);
        for tag in &tags[range.clone()] {
            let Tag::Start(loc, info) = tag else { continue };
            let ElementPayload::CounterUpdate { key, action } = &info.payload else {
                continue;
            };
            if *key == CounterKey::Page {
                match action {
                    CounterUpdate::Set(values) => intr.counters.apply_at(
                        CounterKey::Page,
                        CounterUpdate::Set(values.clone()),
                        *loc,
                    ),
                    CounterUpdate::Step(level) => intr.counters.apply_at(
                        CounterKey::Page,
                        CounterUpdate::Step(*level),
                        *loc,
                    ),
                    CounterUpdate::Func(func) => {
                        apply_counter_func_at(
                            key,
                            func,
                            *loc,
                            intr,
                            &mut scopes,
                            engine,
                            ctx,
                        )?;
                    }
                }
            } else if let CounterUpdate::Func(func) = action {
                if matches!(key, CounterKey::Selector(selector)
                    if crate::compiler::eval::operators::equality::selector_contains_element(selector))
                {
                    continue;
                }
                apply_counter_func_at(key, func, *loc, intr, &mut scopes, engine, ctx)?;
            }
        }
    }
    Ok(())
}

fn apply_counter_func_at(
    key: &crate::entities::counter::CounterKey,
    func: &crate::entities::func::Func,
    loc: crate::entities::location::Location,
    intr: &mut TagIntrospector,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::entities::value::Value;

    let current = intr.counters.value_at(key, loc).unwrap_or(&[0]);
    let inputs = current.iter().map(|value| Value::Int(*value as i64)).collect();
    let result = apply_func(func.clone(), Args::positional(inputs), scopes, ctx, engine)?;
    let state = match result {
        Value::Int(value) if value >= 0 => vec![value as usize],
        Value::Array(values) => values
            .into_iter()
            .map(|value| match value {
                Value::Int(value) if value >= 0 => Ok(value as usize),
                other => {
                    Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                        crate::entities::span::Span::detached(),
                        format!(
                            "counter update function returned {} instead of integer",
                            other.type_name()
                        ),
                    )])
                }
            })
            .collect::<SourceResult<Vec<_>>>()?,
        other => {
            return Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                format!(
                    "counter update function returned {} instead of integer or array",
                    other.type_name()
                ),
            )])
        }
    };
    intr.counters.apply_at(key.clone(), CounterUpdate::Set(state), loc);
    Ok(())
}
