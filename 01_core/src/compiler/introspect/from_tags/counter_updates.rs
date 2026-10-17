//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/from_tags/counter_updates.md
//! @prompt-hash 5b1aa86e
//! @layer L1
//! @updated 2026-10-05

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
    events.sort_by_key(|event| intr.counters.order_key(event.location));
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
                context.features = observer.features;
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

        if *key == crate::entities::counter::CounterKey::Page {
            continue;
        }
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
                        let _ = func; // Page callbacks are evaluated in the realized fold.
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

/// Seal Page current/final together, against the candidate physical generation.
pub fn realize_page_counter_snapshot(
    _tags: &[Tag],
    documents: &[(
        Vec<crate::entities::location::Location>,
        Option<std::num::NonZeroUsize>,
    )],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()> {
    use crate::entities::{
        counter::CounterKey, counter_registry::CounterRegistry, location::Location,
    };
    use crate::entities::{page_store::PageStore, span::Span, value::Value};
    use std::collections::{HashMap, HashSet};
    use std::num::NonZeroUsize;
    let failure = || {
        vec![crate::entities::source_result::SourceDiagnostic::error(
            Span::detached(),
            "page counter snapshot could not be verified",
        )]
    };
    let actions: HashMap<_, _> = intr
        .counters
        .actions()
        .iter()
        .filter(|e| e.key.as_ref() == Some(&CounterKey::Page))
        .map(|e| (e.location, e.action.clone()))
        .collect();
    let mut all: HashSet<Location> = intr
        .elements
        .keys()
        .copied()
        .chain(intr.kind_index.values().flatten().copied())
        .chain(intr.context_block_locations.values().copied())
        .chain(actions.keys().copied())
        .collect();
    for (locations, _) in documents {
        all.extend(locations);
    }
    let mut current = HashMap::new();
    let mut finals = HashMap::new();
    let mut numbers = Vec::new();
    let mut offset = 0usize;
    let mut scopes = Scopes::new(None);
    let mut assigned = HashSet::new();
    for (index, (supplied, total)) in documents.iter().enumerate() {
        let mut locations = if documents.len() == 1 {
            all.iter().copied().collect::<Vec<_>>()
        } else {
            intr.counters.document_locations(index).unwrap_or(supplied).to_vec()
        };
        locations.sort_by_key(|&l| intr.counters.order_key(l));
        locations.dedup();
        if locations.iter().any(|&l| !assigned.insert(l)) {
            return Err(failure());
        }
        let mut state = CounterRegistry::empty();
        state.apply(CounterKey::Page, CounterUpdate::Set(vec![1]));
        let mut event_page = offset + 1;
        let mut page_numbers = vec![1; total.map_or(0, NonZeroUsize::get)];
        for &location in &locations {
            let physical = match total {
                Some(total) => match intr.position_of(location) {
                    Some(p)
                        if (offset + 1..=offset + total.get())
                            .contains(&p.page.get()) =>
                    {
                        p.page.get()
                    }
                    _ if actions.contains_key(&location) => return Err(failure()),
                    _ => continue,
                },
                None => event_page,
            };
            let delta = physical.checked_sub(event_page).ok_or_else(failure)?;
            let mut observed = state.clone();
            for _ in 0..delta {
                observed.apply(CounterKey::Page, CounterUpdate::step());
            }
            if let Some(action) = actions.get(&location) {
                if total.is_some() {
                    for page in event_page..physical {
                        page_numbers[page - offset - 1] = state
                            .value(&CounterKey::Page)
                            .and_then(|v| v.first())
                            .copied()
                            .unwrap_or(0)
                            + page
                            - event_page;
                    }
                }
                state = observed;
                match action {
                    CounterUpdate::Func(function) => {
                        let mut ctx = EvalContext::new();
                        ctx.target = if total.is_some() {
                            crate::compiler::eval::EvalTarget::Paged
                        } else {
                            crate::compiler::eval::EvalTarget::Html
                        };
                        ctx.features = observer.features;
                        let args = Args::positional(
                            state
                                .value(&CounterKey::Page)
                                .unwrap_or(&[])
                                .iter()
                                .map(|&n| Value::Int(n as i64))
                                .collect(),
                        );
                        let value = apply_func(
                            function.clone(),
                            args,
                            &mut scopes,
                            &mut ctx,
                            engine,
                        )?;
                        let invalid = |value: &Value, expected: &str| {
                            vec![crate::entities::source_result::SourceDiagnostic::error(
                                Span::detached(),
                                format!("counter update function returned {} instead of {expected}", value.type_name()),
                            )]
                        };
                        let values = match value {
                            Value::Int(n) if n >= 0 => vec![n as usize],
                            Value::Array(values) => values
                                .into_iter()
                                .map(|value| match value {
                                    Value::Int(n) if n >= 0 => Ok(n as usize),
                                    other => Err(invalid(&other, "integer")),
                                })
                                .collect::<SourceResult<Vec<_>>>()?,
                            other => return Err(invalid(&other, "integer or array")),
                        };
                        state.apply(CounterKey::Page, CounterUpdate::Set(values));
                    }
                    other => state.apply(CounterKey::Page, other.clone()),
                }
                event_page = physical;
                observed = state.clone();
            }
            current.insert(
                location,
                vec![observed
                    .value(&CounterKey::Page)
                    .and_then(|v| v.first())
                    .copied()
                    .unwrap_or(0)],
            );
        }
        if let Some(total) = total {
            let end = offset + total.get();
            for page in event_page..=end {
                page_numbers[page - offset - 1] = state
                    .value(&CounterKey::Page)
                    .and_then(|v| v.first())
                    .copied()
                    .unwrap_or(0);
                if page < end {
                    state.apply(CounterKey::Page, CounterUpdate::step());
                }
            }
            numbers.extend(page_numbers);
            offset = end;
        }
        let final_state = vec![state
            .value(&CounterKey::Page)
            .and_then(|v| v.first())
            .copied()
            .unwrap_or(0)];
        for location in locations {
            finals.insert(location, final_state.clone());
        }
    }
    if actions.keys().any(|l| !assigned.contains(l)) {
        return Err(failure());
    }
    let store = if let Some(total) = NonZeroUsize::new(offset) {
        let pages = (1..=offset)
            .map(|p| NonZeroUsize::new(p).unwrap())
            .collect::<Vec<_>>();
        PageStore::from_realized(
            total,
            pages
                .iter()
                .map(|&p| intr.page_store.numbering_for_page(p).cloned())
                .collect(),
            numbers,
            pages
                .iter()
                .map(|&p| {
                    intr.page_store.numbering_span_for_page(p).unwrap_or(Span::detached())
                })
                .collect(),
            pages
                .iter()
                .map(|&p| intr.page_store.visible_numbering_for_page(p).cloned())
                .collect(),
            pages
                .iter()
                .map(|&p| intr.page_store.reference_numbering_for_page(p).cloned())
                .collect(),
            pages
                .iter()
                .map(|&p| {
                    intr.page_store
                        .supplement_for_page(p)
                        .cloned()
                        .unwrap_or(crate::entities::content::Content::Empty)
                })
                .collect(),
        )
    } else {
        PageStore::empty()
    };
    intr.page_counter_values = current;
    intr.page_counter_final_values = finals;
    intr.inject_pages(store);
    Ok(())
}
