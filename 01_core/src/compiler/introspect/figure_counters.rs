//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/figure_counters.md
//! @prompt-hash 10cc250e
//! @layer L1
//! @updated 2026-10-19

use super::figure_demands::{collect_figure_counter_demands, FigureCounterDemand};
use crate::compiler::eval::operators::equality::counter_keys_eq;
use crate::compiler::{
    eval::{EvalContext, EvalTarget},
    scopes::Scopes,
};
use crate::entities::{
    content::Content,
    counter::CounterKey,
    elements::figure::FigureKind,
    engine::Engine,
    func::Func,
    introspector::{FigureCounterSnapshot, Introspector, TagIntrospector},
    selector::Selector,
    source_result::{SourceDiagnostic, SourceResult},
    style_chain::StyleChain,
};

/// Typed Figure selector identity, independent of user string counters.
pub(crate) fn figure_counter_key(kind: &FigureKind) -> CounterKey {
    let value = match kind {
        FigureKind::Element(function) => {
            crate::entities::value::Value::Func(function.clone())
        }
        FigureKind::Name(name) => crate::entities::value::Value::Str(name.clone()),
    };
    CounterKey::Selector(Selector::Element {
        function: Func::native("figure", crate::compiler::stdlib::native_figure),
        fields: vec![("kind".into(), value)].into(),
    })
}

pub(super) fn prepare_figure_counters(
    content: &Content,
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()> {
    prepare_figure_counters_for_bodies(&[content], intr, engine, observer)
}

pub(super) fn prepare_figure_counters_for_bodies(
    bodies: &[&Content],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()> {
    if observer.target == EvalTarget::Html {
        intr.figure_counter_snapshots.clear();
        return Ok(());
    }
    let mut demands = Vec::new();
    for body in bodies {
        demands.extend(collect_figure_counter_demands(
            body,
            intr,
            &StyleChain::default_chain(),
        )?);
    }
    finish_demands(demands, intr, engine, observer)
}

pub(super) fn finish_demands(
    demands: Vec<FigureCounterDemand>,
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()> {
    let mut staged: Vec<FigureCounterSnapshot> = Vec::new();
    let mut scopes = Scopes::new(None);
    for demand in &demands {
        if staged
            .iter()
            .any(|snapshot| counter_keys_eq(&snapshot.key, &demand.key))
        {
            continue;
        }
        let history = super::from_tags::resolve_filtered_counter(
            &demand.key,
            intr,
            &mut scopes,
            engine,
            demand.span,
            observer,
        )?;
        staged.push(FigureCounterSnapshot { key: demand.key.clone(), history });
    }
    for FigureCounterDemand { key, location, span } in demands {
        let failure = || {
            vec![SourceDiagnostic::error(
                span,
                "figure counter snapshot could not be verified",
            )]
        };
        let CounterKey::Selector(selector) = &key else {
            return Err(failure());
        };
        let selected = intr.query(selector).contains(&location);
        let automatic = intr
            .counters
            .actions()
            .iter()
            .any(|event| event.location == location && event.key.is_none());
        let prefix = staged
            .iter()
            .find(|snapshot| counter_keys_eq(&snapshot.key, &key))
            .is_some_and(|snapshot| {
                snapshot.history.iter().any(|(at, _)| *at == location)
            });
        if intr.element_at(location).is_none() || !selected || !automatic || !prefix {
            return Err(failure());
        }
    }
    let mut projections = Vec::new();
    for (label, key) in &intr.label_to_counter_key {
        let Some(location) = intr.query_by_label(label) else {
            continue;
        };
        let Some(Content::Figure(figure)) = intr.element_at(location) else {
            continue;
        };
        if figure.resolved_kind.is_none() {
            continue;
        }
        let scalar = staged
            .iter()
            .find(|snapshot| counter_keys_eq(&snapshot.key, key))
            .and_then(|snapshot| {
                snapshot.history.iter().rev().find(|(at, _)| {
                    intr.counters.order_key(*at) <= intr.counters.order_key(location)
                })
            })
            .and_then(|(_, values)| values.last())
            .copied();
        projections.push((label.clone(), scalar));
    }
    for (label, scalar) in projections {
        intr.figure_label_numbers.remove(&label);
        if let Some(scalar) = scalar {
            intr.figure_label_numbers.insert(label, scalar);
        }
    }
    intr.figure_counter_snapshots = staged;
    Ok(())
}
