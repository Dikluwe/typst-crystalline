// Late binding only: included by the real L3 session under test + observation cfg.
// Sealed terminal R5 input successor is included unchanged. No lifecycle DTO or verdict.
use super::*;
use std::cell::{Cell, RefCell};
use std::num::NonZeroU16;
use std::rc::Rc;
use typst_core::compiler::eval::{
    p1340_observation_diagnostics, p1340_observation_value,
};
use typst_core::entities::file_id::FileId;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};

mod terminal_contract {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1340-contract-terminal-harness-r5.rs"
    ));
}

struct Trace {
    opaque: Option<Value>,
    names: [&'static str; 5],
    first_selected: Option<usize>,
    attempts: Vec<usize>,
    exports: usize,
    original_errors: Option<Vec<SourceDiagnostic>>,
    result: Option<SourceResult<()>>,
    warnings: Option<Vec<SourceDiagnostic>>,
    selected_body_ok: bool,
    impossible: bool,
    metadata: Vec<Value>,
    final_counter: Option<Vec<usize>>,
    opaque_query_seen: bool,
    sink_inputs: [usize; 5],
}

std::thread_local! {
    static ACTIVE: RefCell<Option<Rc<RefCell<Trace>>>> = const { RefCell::new(None) };
    static SOURCE_ID: Cell<u16> = const { Cell::new(1) };
}

fn observe(f: impl FnOnce(&mut Trace)) {
    ACTIVE.with(|active| {
        let trace = active.borrow().clone();
        if let Some(trace) = trace {
            f(&mut trace.borrow_mut());
        }
    });
}

struct ActiveGuard;
impl Drop for ActiveGuard {
    fn drop(&mut self) {
        ACTIVE.with(|active| {
            active.borrow_mut().take();
        });
    }
}

fn sentinel(source: &Source, trace: &mut Trace, which: usize) -> SourceDiagnostic {
    trace.sink_inputs[which] += 1;
    SourceDiagnostic::warning(source.root().span(), trace.names[which])
}

// The only content mutation is the input transform authorized by binding R2.
fn opaque_input(content: &Content, value: &Value) -> Content {
    let mut replaced = 0;
    let transformed = content
        .map_content(&mut |node| {
            let Content::Label(label) = node else { return Ok(None) };
            if label.name != "opaque" {
                return Ok(None);
            }
            let mut label = label.as_ref().clone();
            label.body = label.body.map_content(&mut |body| {
                if let Content::Metadata(metadata) = body {
                    assert!(
                        matches!(&*metadata.value, Value::Int(0)),
                        "opaque input was not Int(0)"
                    );
                    let mut metadata = metadata.as_ref().clone();
                    metadata.value = Box::new(value.clone());
                    replaced += 1;
                    return Ok(Some(Content::Metadata(Arc::new(metadata))));
                }
                Ok(None)
            })?;
            Ok(Some(Content::Label(Arc::new(label))))
        })
        .expect("normed input transform must be total");
    assert_eq!(replaced, 1, "opaque input label must identify one real metadata");
    transformed
}

pub(in crate::pipeline) fn post_eval(
    content: &mut Content,
    intr_content: &mut Content,
    source: &Source,
    global_warnings: &mut Vec<SourceDiagnostic>,
) {
    observe(|trace| {
        global_warnings.push(sentinel(source, trace, 0));
        if let Some(value) = &trace.opaque {
            *content = opaque_input(content, value);
            *intr_content = opaque_input(intr_content, value);
        }
    });
}

pub(super) fn body_started(session: &Session<'_>, index: usize, sink: &mut TypstSink) {
    observe(|trace| {
        // R2 sources place their first demand in the first actual root. Verify
        // selection at discovery_completed; never decide selection in this hook.
        if session.roots.first() == Some(&index) {
            sink.warn(sentinel(session.source, trace, 1));
        }
    });
}

pub(super) fn discovery_completed(session: &mut Session<'_>, index: usize) {
    observe(|trace| {
        if session.nodes[index].selected && trace.first_selected.is_none() {
            assert_eq!(
                session.roots.first(),
                Some(&index),
                "terminal input binding needs its first root selected"
            );
            trace.first_selected = Some(index);
            session.nodes[index]
                .current
                .as_mut()
                .expect("real discovery execution")
                .sink
                .warn(sentinel(session.source, trace, 2));
        }
    });
}

pub(in crate::pipeline) fn attempt_started(attempt: usize) {
    observe(|trace| trace.attempts.push(attempt));
}

pub(super) fn execution_invalidated(source: &Source, execution: &mut Execution) {
    observe(|trace| execution.sink.warn(sentinel(source, trace, 3)));
}

pub(in crate::pipeline) fn replay_started(source: &Source, sink: &mut TypstSink) {
    // This real sink is discarded for reflection/validation/diagnostics alike.
    // It does not stand in for a validation decision or a lifecycle event.
    observe(|trace| sink.warn(sentinel(source, trace, 4)));
}

pub(in crate::pipeline) fn candidate_built(candidate: &TagIntrospector) {
    observe(|trace| trace.metadata = candidate.metadata.query().to_vec());
}

fn contains_carrier(value: &serde_json::Value, carrier: &serde_json::Value) -> bool {
    if value == carrier {
        return true;
    }
    match value {
        serde_json::Value::Array(values) => {
            values.iter().any(|v| contains_carrier(v, carrier))
        }
        serde_json::Value::Object(values) => {
            values.values().any(|v| contains_carrier(v, carrier))
        }
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => false,
    }
}

pub(super) fn before_decision(session: &Session<'_>) {
    observe(|trace| {
        trace.original_errors = Some(session.errors());
        trace.impossible = session.impossible;
        let selected: Vec<_> = session
            .order()
            .into_iter()
            .filter(|&i| session.nodes[i].selected)
            .filter_map(|i| session.nodes[i].current.as_ref())
            .collect();
        trace.selected_body_ok =
            !selected.is_empty() && selected.iter().all(|e| e.result.is_ok());
        trace.final_counter = None;
        for execution in &selected {
            let requests = execution.ctx.p1340_observation_requests();
            for row in requests.as_array().expect("L1 request array") {
                if row["operation"] == "CounterFinal" && row["result"]["kind"] == "Ok" {
                    trace.final_counter = Some(
                        row["result"]["value"]
                            .as_array()
                            .expect("CounterFinal array result")
                            .iter()
                            .map(|v| {
                                usize::try_from(
                                    v.as_u64().expect("CounterFinal nonnegative integer"),
                                )
                                .expect("CounterFinal usize")
                            })
                            .collect(),
                    );
                }
            }
        }
        if let Some(opaque) = &trace.opaque {
            let carrier = p1340_observation_value(opaque);
            for execution in session
                .nodes
                .iter()
                .filter_map(|n| n.current.as_ref())
                .chain(session.retired.iter())
            {
                // Exact retained input identity is only a carrier witness. It
                // is not a structural/stability comparison or a replay result.
                for rows in [
                    execution.ctx.p1340_observation_requests(),
                    execution.ctx.p1340_observation_replays(),
                ] {
                    trace.opaque_query_seen |=
                        rows.as_array().expect("L1 observation array").iter().any(|r| {
                            r["operation"] == "Query"
                                && r["result"]["kind"] == "Ok"
                                && contains_carrier(&r["result"]["value"], &carrier)
                        });
                }
            }
        }
    });
}

pub(in crate::pipeline) fn compilation_returned(
    source: &Source,
    result: &SourceResult<PagedDocument>,
    warnings: &[SourceDiagnostic],
) {
    observe(|trace| {
        assert!(trace.result.is_none(), "one real compilation return per invocation");
        trace.result = Some(result.as_ref().map(|_| ()).map_err(Clone::clone));
        trace.warnings = Some(warnings.to_vec());
        // Retain the actual returned root identity in diagnostics; do not remap spans.
        assert!(source.root().span().id().is_some());
    });
}

pub(in crate::pipeline) fn exporter_dispatched() {
    observe(|trace| trace.exports += 1);
}

struct MemoryWorld {
    library: Library,
    book: FontBook,
    source: Source,
}
impl World for MemoryWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound)
        }
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(
        &self,
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

fn actual(input: &terminal_contract::Input) -> terminal_contract::Observation {
    let id = SOURCE_ID.with(|next| {
        let current = next.get();
        next.set(current.checked_add(1).expect("test source id space"));
        FileId::from_raw(NonZeroU16::new(current).expect("nonzero source id"))
    });
    let world = MemoryWorld {
        library: Library::new(),
        book: FontBook::new(),
        source: Source::new(id, input.source_text.to_owned()),
    };
    let trace = Rc::new(RefCell::new(Trace {
        opaque: input.opaque_input.clone(),
        names: input.warning_names,
        first_selected: None,
        attempts: vec![],
        exports: 0,
        original_errors: None,
        result: None,
        warnings: None,
        selected_body_ok: false,
        impossible: false,
        metadata: vec![],
        final_counter: None,
        opaque_query_seen: false,
        sink_inputs: [0; 5],
    }));
    ACTIVE.with(|active| {
        assert!(active.borrow().is_none());
        *active.borrow_mut() = Some(trace.clone());
    });
    let _guard = ActiveGuard;
    let public_warnings;
    if input.case == terminal_contract::Case::ProvenImpossibleTopologyTerminal {
        // Normed unit input to the exact product terminal operation. This does
        // not claim to discover impossible topology from this valid source.
        let intr = introspect_with_introspector(&Content::Empty);
        let mut session = Session::new(
            Content::Empty,
            &intr,
            &world,
            &world.source,
            false,
            Features::default(),
        );
        session.impossible = true;
        let mut global = vec![];
        observe(|trace| global.push(sentinel(&world.source, trace, 0)));
        before_decision(&session);
        assert!(session.impossible);
        let result = terminal::<PagedDocument>(session.source, session.errors());
        compilation_returned(&world.source, &result, &global);
        public_warnings = global;
    } else {
        let (export_result, warnings) = super::super::compile_to_pdf_bytes_impl(
            &world,
            &world.source,
            false,
            None,
            StreamMode::Verbose,
            PdfTags::Disabled,
            Features::default(),
            false,
            &mut Timings::default(),
        );
        // Preserve document and exporter outcomes separately; an exporter error
        // must not be silently projected into a successful public compilation.
        if trace.borrow().result.as_ref().is_some_and(Result::is_ok) {
            assert!(export_result.is_ok(), "actual PDF export failed: {export_result:?}");
        }
        public_warnings = warnings;
    }
    let mut trace = trace.borrow_mut();
    assert!(
        trace.opaque.is_none() || trace.opaque_query_seen,
        "binding gap: no actual query result witnessed the retained opaque carrier"
    );
    let result = trace.result.take().expect("missing real compilation return hook");
    let original_errors = trace
        .original_errors
        .take()
        .expect("missing pre-decision body error capture");
    let hook_warnings = trace.warnings.take().expect("missing warning capture");
    assert_eq!(
        p1340_observation_diagnostics(&hook_warnings),
        p1340_observation_diagnostics(&public_warnings)
    );
    println!(
        "P1340_TERMINAL_OBSERVATION {}",
        serde_json::json!({
            "case":format!("{:?}",input.case),"source_text":world.source.text(),
            "attempts":trace.attempts,"export_calls":trace.exports,"sink_inputs":trace.sink_inputs,
            "result_errors":result.as_ref().err().map(|e|p1340_observation_diagnostics(e)),
            "original_errors":p1340_observation_diagnostics(&original_errors),
            "warnings":p1340_observation_diagnostics(&public_warnings),
            "metadata":trace.metadata.iter().map(p1340_observation_value).collect::<Vec<_>>(),
            "final_read_counter":trace.final_counter,"selected_body_ok":trace.selected_body_ok,
            "topology_impossibility":trace.impossible,"opaque_query_seen":trace.opaque_query_seen
        })
    );
    terminal_contract::Observation {
        source: world.source.clone(),
        result,
        warnings: public_warnings,
        original_errors,
        attempts: trace.attempts.clone(),
        export_calls: trace.exports,
        final_read_counter: trace.final_counter.clone(),
        metadata_integers: trace
            .metadata
            .iter()
            .filter_map(|v| if let Value::Int(i) = v { Some(*i) } else { None })
            .collect(),
        selected_body_ok: trace.selected_body_ok,
        topology_impossibility: trace.impossible,
    }
}

#[test]
fn p1340_terminal_r2_real_binding_default_repeat_reverse() {
    terminal_contract::run_suite(actual);
    terminal_contract::run_suite(actual);
    terminal_contract::run_suite_with_order(actual, true);
}
