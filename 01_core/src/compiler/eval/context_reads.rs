//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/context_reads.md
//! @prompt-hash eb27bea9
//! @layer L1
//! @updated 2026-10-06

use super::*;

/// Closed vocabulary of causal reads. It is deliberately separate from the
/// language's equality and from the introspector's stores.
#[derive(Clone)]
pub(crate) enum ContextReadRequest {
    CounterFold {
        key: crate::entities::counter::CounterKey,
    },
    CounterAt {
        key: crate::entities::counter::CounterKey,
        location: crate::entities::location::Location,
    },
    CounterLabelAt {
        key: crate::entities::counter::CounterKey,
        label: crate::entities::label::Label,
    },
    CounterFinal {
        key: crate::entities::counter::CounterKey,
    },
    CounterTotal {
        key: crate::entities::counter::CounterKey,
    },
    CounterResolve {
        value: Value,
    },
    CounterLegacyAt {
        key: String,
        label: crate::entities::label::Label,
    },
    CounterLegacyFinal {
        key: String,
    },
    StateGet {
        state: crate::entities::state::State,
    },
    StateDisplay {
        state: crate::entities::state::State,
    },
    StateAt {
        state: crate::entities::state::State,
        location: crate::entities::location::Location,
    },
    StateFinal {
        state: crate::entities::state::State,
    },
    StateResolveLabel {
        label: crate::entities::label::Label,
    },
    StateLegacyAt {
        key: String,
        label: crate::entities::label::Label,
    },
    StateLegacyFinal {
        key: String,
    },
    Query {
        selector: crate::entities::selector::Selector,
    },
    Locate {
        selector: crate::entities::selector::Selector,
    },
    Here,
    LocationPage {
        location: crate::entities::location::Location,
    },
    LocationPosition {
        location: crate::entities::location::Location,
    },
    LocationPageNumbering {
        location: crate::entities::location::Location,
    },
    OutlineEntryPage {
        location: crate::entities::location::Location,
    },
    OutlineEntryIndented(Box<OutlineEntryIndentedRead>),
}

#[derive(Clone)]
pub(crate) struct OutlineEntryIndentedRead {
    entry: crate::entities::content::Content,
    prefix: Value,
    inner: Value,
    gap: crate::entities::layout_types::Length,
}

impl ContextReadRequest {
    pub(crate) fn outline_entry_indented(
        entry: crate::entities::content::Content,
        prefix: Value,
        inner: Value,
        gap: crate::entities::layout_types::Length,
    ) -> Self {
        Self::OutlineEntryIndented(Box::new(OutlineEntryIndentedRead {
            entry,
            prefix,
            inner,
            gap,
        }))
    }
}

#[derive(Clone)]
pub(super) struct ContextRead {
    request: ContextReadRequest,
    span: Span,
    current_file: Option<FileId>,
    location: Option<crate::entities::location::Location>,
    in_context: bool,
    target: EvalTarget,
    features: crate::entities::compiler_features::Features,
    styles: Arc<StyleChain>,
    show_rules: Arc<[ShowRule]>,
    active_guards: Vec<RuleId>,
    result: SourceResult<Value>,
}

pub(super) struct ContextReads {
    selected: bool,
    replaying: bool,
    styles: StyleChain,
    show_rules: Arc<[ShowRule]>,
    active_guards: Vec<RuleId>,
    current_file: Option<FileId>,
    pub(super) reads: Vec<Arc<ContextRead>>,
}

impl ContextReads {
    pub(super) fn new() -> Self {
        Self {
            selected: false,
            replaying: false,
            styles: StyleChain::default_chain(),
            show_rules: Arc::from([]),
            active_guards: Vec::new(),
            current_file: None,
            reads: Vec::new(),
        }
    }
}

impl EvalContext {
    pub(crate) fn replace_context_read_styles(&self, styles: StyleChain) -> StyleChain {
        std::mem::replace(&mut self.context_reads.borrow_mut().styles, styles)
    }

    pub(crate) fn replace_context_read_rules(
        &self,
        rules: Arc<[ShowRule]>,
        guards: Vec<RuleId>,
    ) -> (Arc<[ShowRule]>, Vec<RuleId>) {
        let mut log = self.context_reads.borrow_mut();
        (
            std::mem::replace(&mut log.show_rules, rules),
            std::mem::replace(&mut log.active_guards, guards),
        )
    }

    pub(crate) fn replace_context_read_file(
        &self,
        file: Option<FileId>,
    ) -> Option<FileId> {
        std::mem::replace(&mut self.context_reads.borrow_mut().current_file, file)
    }

    pub(crate) fn mark_filtered_counter_read(
        &self,
        key: &crate::entities::counter::CounterKey,
    ) {
        fn contains_element(selector: &crate::entities::selector::Selector) -> bool {
            use crate::entities::selector::Selector;
            match selector {
                Selector::NativeElement(_) | Selector::Element { .. } => true,
                Selector::And(items) | Selector::Or(items) => {
                    items.iter().any(contains_element)
                }
                Selector::Where { base, .. } => contains_element(base),
                Selector::Within { base, ancestor } => {
                    contains_element(base) || contains_element(ancestor)
                }
                Selector::Before { base, end, .. } => {
                    contains_element(base) || contains_element(end)
                }
                Selector::After { base, start, .. } => {
                    contains_element(base) || contains_element(start)
                }
                Selector::Kind(_)
                | Selector::Label(_)
                | Selector::Location(_)
                | Selector::Regex(_) => false,
            }
        }
        if let crate::entities::counter::CounterKey::Selector(selector) = key {
            if contains_element(selector) {
                self.context_reads.borrow_mut().selected = true;
            }
        }
    }

    pub(crate) fn observe_context_read(
        &self,
        request: ContextReadRequest,
        span: Span,
        result: SourceResult<Value>,
    ) -> SourceResult<Value> {
        let mut log = self.context_reads.borrow_mut();
        if !log.replaying {
            let styles = Arc::new(log.styles.clone());
            let current_file = log.current_file;
            let show_rules = log.show_rules.clone();
            let active_guards = log.active_guards.clone();
            log.reads.push(Arc::new(ContextRead {
                request,
                span,
                current_file,
                location: self.current_location,
                in_context: self.in_context,
                target: self.target,
                features: self.features,
                styles,
                show_rules,
                active_guards,
                result: result.clone(),
            }));
        }
        result
    }

    pub fn has_filtered_counter_reads(&self) -> bool {
        self.context_reads.borrow().selected
    }
    /// P1493 — indica demanda causal efetivamente observada por posição HTML.
    #[doc(hidden)]
    pub fn has_location_position_reads(&self) -> bool {
        self.context_reads.borrow().reads.iter().any(|read| {
            matches!(&read.request, ContextReadRequest::LocationPosition { .. })
        })
    }

    fn replay_context_read(
        &self,
        read: &Arc<ContextRead>,
        candidate: &crate::entities::introspector::TagIntrospector,
        engine: &mut Engine<'_>,
    ) -> SourceResult<Value> {
        let mut ctx = EvalContext::new();
        ctx.introspector = candidate.clone();
        ctx.current_location = read.location;
        ctx.in_context = read.in_context;
        ctx.target = read.target;
        ctx.features = read.features;
        ctx.full_error = self.full_error;
        ctx.max_loop_iterations = self.max_loop_iterations;
        ctx.context_reads.borrow_mut().replaying = true;
        ctx.replace_context_read_styles((*read.styles).clone());
        let mut styles = (*read.styles).clone();
        let mut rules = read.show_rules.clone();
        let mut guards = read.active_guards.clone();
        let mut sink = Sink::new();
        let mut sink_tracked = sink.track_mut();
        let mut transaction = Engine {
            world: engine.world,
            font_metrics: engine.font_metrics,
            route: engine.route,
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: read.current_file.unwrap_or(engine.current_file),
            sink: &mut sink_tracked,
        };
        use ContextReadRequest::*;
        let result = match &read.request {
            CounterFold { .. }
            | CounterAt { .. }
            | CounterLabelAt { .. }
            | CounterFinal { .. }
            | CounterTotal { .. }
            | CounterResolve { .. }
            | CounterLegacyAt { .. }
            | CounterLegacyFinal { .. } => {
                crate::compiler::stdlib::counter::replay_context_read(
                    &read.request,
                    &mut ctx,
                    &mut transaction,
                    read.span,
                )
            }
            StateGet { .. }
            | StateDisplay { .. }
            | StateAt { .. }
            | StateFinal { .. }
            | StateResolveLabel { .. }
            | StateLegacyAt { .. }
            | StateLegacyFinal { .. } => {
                crate::compiler::stdlib::state::replay_context_read(
                    &read.request,
                    &mut ctx,
                    &mut transaction,
                    read.span,
                )
            }
            Query { selector } | Locate { selector } => {
                let mut args = crate::entities::args::Args::positional(Vec::new());
                args.span = read.span;
                args.items.push(Value::Selector(selector.clone()));
                let call = if matches!(read.request, Query { .. }) {
                    crate::compiler::stdlib::native_query
                } else {
                    crate::compiler::stdlib::native_locate
                };
                call(&mut ctx, &args, transaction.world, transaction.current_file)
            }
            Here => {
                let mut args = crate::entities::args::Args::positional(Vec::new());
                args.span = read.span;
                crate::compiler::stdlib::native_here(
                    &mut ctx,
                    &args,
                    transaction.world,
                    transaction.current_file,
                )
            }
            LocationPage { location } => call_dispatch::eval_location_method(
                *location, "page", &mut ctx, read.span,
            ),
            LocationPosition { location } => call_dispatch::eval_location_method(
                *location, "position", &mut ctx, read.span,
            ),
            LocationPageNumbering { location } => call_dispatch::eval_location_method(
                *location,
                "page-numbering",
                &mut ctx,
                read.span,
            ),
            OutlineEntryPage { location } => {
                let mut scopes = crate::compiler::scopes::Scopes::new(None);
                crate::compiler::stdlib::realize_outline_entry_page(
                    *location,
                    read.span,
                    &mut scopes,
                    &mut ctx,
                    &mut transaction,
                )
            }
            OutlineEntryIndented(payload) => {
                let OutlineEntryIndentedRead { entry, prefix, inner, gap } =
                    payload.as_ref();
                let mut scopes = crate::compiler::scopes::Scopes::new(None);
                let mut named = indexmap::IndexMap::default();
                named.insert("gap".into(), Value::Length(*gap));
                let args = crate::entities::args::Args::from_parts(
                    vec![Value::Content(entry.clone()), prefix.clone(), inner.clone()],
                    named,
                    read.span,
                );
                crate::compiler::stdlib::native_outline_entry_indented(
                    &mut ctx,
                    &args,
                    transaction.world,
                    transaction.current_file,
                    &mut scopes,
                    &mut transaction,
                )
            }
        };
        result
    }

    pub fn context_reads_valid_for(
        &self,
        candidate: &crate::entities::introspector::TagIntrospector,
        engine: &mut Engine<'_>,
    ) -> SourceResult<bool> {
        let reads = self.context_reads.borrow().reads.clone();
        let mut valid = true;
        for read in &reads {
            let result = self.replay_context_read(read, candidate, engine);
            if read.result.observation_relation(&result) != ObservationRelation::Same {
                valid = false;
                break;
            }
        }
        Ok(valid)
    }

    pub fn context_nonconvergence_diagnostics(
        &self,
        history: &[crate::entities::introspector::TagIntrospector],
        engine: &mut Engine<'_>,
    ) -> SourceResult<Vec<SourceDiagnostic>> {
        if history.len() < 2 {
            return Ok(Vec::new());
        }
        let reads = self.context_reads.borrow().reads.clone();
        let mut diagnostics = Vec::new();
        for read in &reads {
            // Fullfold and lookup-resolution are dependencies of projections,
            // not additional user-facing value queries.
            let (name, counter) = match &read.request {
                ContextReadRequest::CounterAt { key, .. }
                | ContextReadRequest::CounterLabelAt { key, .. }
                | ContextReadRequest::CounterFinal { key }
                | ContextReadRequest::CounterTotal { key } => {
                    let name = match key {
                        crate::entities::counter::CounterKey::Page => {
                            "the page counter".to_owned()
                        }
                        _ => format!(
                            "`{}`",
                            repr::repr_value(&Value::Counter(
                                crate::entities::counter::Counter { key: key.clone() }
                            ))
                        ),
                    };
                    (name, true)
                }
                ContextReadRequest::StateGet { state }
                | ContextReadRequest::StateDisplay { state }
                | ContextReadRequest::StateAt { state, .. }
                | ContextReadRequest::StateFinal { state } => (
                    format!(
                        "`state({})`",
                        repr::repr_value(&Value::Str(state.key.clone()))
                    ),
                    false,
                ),
                ContextReadRequest::CounterFold { .. }
                | ContextReadRequest::CounterResolve { .. }
                | ContextReadRequest::CounterLegacyAt { .. }
                | ContextReadRequest::CounterLegacyFinal { .. }
                | ContextReadRequest::StateResolveLabel { .. }
                | ContextReadRequest::StateLegacyAt { .. }
                | ContextReadRequest::StateLegacyFinal { .. }
                | ContextReadRequest::Query { .. }
                | ContextReadRequest::Locate { .. }
                | ContextReadRequest::Here
                | ContextReadRequest::LocationPage { .. }
                | ContextReadRequest::LocationPosition { .. }
                | ContextReadRequest::LocationPageNumbering { .. }
                | ContextReadRequest::OutlineEntryPage { .. } => continue,
                ContextReadRequest::OutlineEntryIndented(_) => continue,
            };
            let projections: Vec<_> = history
                .iter()
                .map(|snapshot| self.replay_context_read(read, snapshot, engine))
                .collect();
            let last = projections.len() - 1;
            if projections[last - 1].observation_relation(&projections[last])
                != ObservationRelation::Different
            {
                continue;
            }
            let mut hint = String::from("the following values were observed:");
            for (index, projection) in projections.iter().enumerate() {
                let iteration = if index == last {
                    "final".to_owned()
                } else {
                    format!("run {}", index + 1)
                };
                let rendered = match projection {
                    Ok(Value::Array(values)) if counter => {
                        values.iter().map(repr::repr_value).collect::<Vec<_>>().join(", ")
                    }
                    Ok(value) if counter => repr::repr_value(value),
                    Ok(value) => format!("`{}`", repr::repr_value(value)),
                    Err(_) => "(errored)".to_owned(),
                };
                hint.push_str(&format!("\n- {iteration}: {rendered}"));
            }
            let mut diagnostic = SourceDiagnostic::warning(
                read.span,
                format!("value of {name} did not converge"),
            )
            .with_hint(hint);
            if !counter {
                diagnostic = diagnostic
                    .with_hint("see https://typst.app/help/state-convergence for help");
            }
            diagnostics.push(diagnostic);
        }
        Ok(diagnostics)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}

#[cfg(test)]
mod storage_budget_tests {
    use super::*;

    #[test]
    fn context_read_request_fits_common_value_budget() {
        let request_bytes = std::mem::size_of::<ContextReadRequest>();
        let record_bytes = std::mem::size_of::<ContextRead>();
        let budget =
            std::mem::size_of::<Value>() + std::mem::align_of::<ContextReadRequest>();
        eprintln!("context-read-size request={request_bytes} record={record_bytes} budget={budget}");
        assert!(
            request_bytes <= budget,
            "request storage {request_bytes} exceeds common-value budget {budget}; record={record_bytes}",
        );
    }
}
