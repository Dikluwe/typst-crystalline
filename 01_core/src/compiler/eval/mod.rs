//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval.md
//! @prompt-hash a49a7e39
//! @layer L1
//! @updated 2026-09-01
//!
//! Dispatcher central do eval: `EvalContext` struct + impl, `pub fn eval`
//! entry point, `eval_markup` iterator, `eval_expr` dispatcher delegando
//! cada armo para o submódulo do respectivo domínio (ADR-0037 Regra 4,
//! completada no Passo 96.2).
//!
//! Armos triviais (literais `Int`/`Float`/`Bool`/etc., `Ref`/`Label`/
//! `Parenthesized`) permanecem inline; armos que contêm scoping
//! cross-cutting (`CodeBlock`, `ContentBlock`) também, por não
//! pertencerem a nenhum cluster em particular.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use comemo::{Track, Tracked, TrackedMut};
use ecow::EcoString;
use hayagriva::citationberg::IndependentStyle;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::layout::FixedMetrics;
use crate::compiler::scopes::Scopes;
use crate::contracts::world::{SysInputs, World};
use crate::entities::ast::AstNode;
#[cfg(test)]
use crate::entities::ast::expr::UnOp;
use crate::entities::ast::expr::{ArrayItem, BinOp, Expr};
use crate::entities::ast::markup::Label as AstLabel;
use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::document_info::DocumentInfo;
use crate::entities::elements::bibliography::BibliographyElem;
use crate::entities::elements::context_block::ContextBlockElem;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::func::{ClosureRepr, Func};
use crate::entities::module::Module;
use crate::entities::package_spec::PackageSpec;
use crate::entities::path::RootedPath;
use crate::entities::scope::Scope;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleChain;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::{SyntaxErrorKind, SyntaxNode};
use crate::entities::value::{Type, Value};
use crate::entities::world_types::{
    Bytes, Datetime, FileResult, Font, Library, Route, Routines, Sink, Traced,
};

// Submódulos por domínio (Passo 96.1, ADR-0037).
pub(crate) mod cast;
pub(crate) mod font_dict;
mod math;
pub(crate) mod operators;
pub use cast::{CastError, cast_length};
pub(crate) mod flow;
pub use flow::FlowEvent;
pub(crate) mod call_dispatch;
pub(crate) mod closures;
mod control_flow;
pub use call_dispatch::apply_func;
pub(crate) mod bibliography;
pub mod bibtex;
pub(crate) mod bindings;
mod markup;
mod modules;
pub(crate) mod repr;
pub(crate) mod table;

/// Representação morfológica pública e estreita de `Stroke` para consumidores
/// que possuem uma branch nominal do tipo. Não é fallback genérico de `Value`.
pub fn repr_stroke_value(stroke: &crate::entities::geometry::Stroke) -> String {
    repr::repr_stroke(stroke)
}

/// Representação morfológica pública usada por serializers estruturados.
///
/// A classificação entre valores estruturais e fallback textual pertence ao
/// formatter consumidor; esta fachada apenas expõe o `repr` total de L1.
pub fn repr_value_for_serialization(value: &crate::entities::value::Value) -> String {
    repr::repr_value(value)
}
pub(crate) mod rules;
pub(crate) mod selector_matching;
pub(crate) mod show_rule_termination;

#[cfg(test)]
mod p1339_observer_local_tests {
    use super::*;

    struct MemoryWorld {
        library: Library,
        book: FontBook,
    }
    impl World for MemoryWorld {
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
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(crate::entities::world_types::FileError::NotFound)
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

    #[test]
    fn p1339_observation_real_state_replay_uses_captured_context() {
        let world = MemoryWorld { library: Library::new(), book: FontBook::new() };
        let metrics = FixedMetrics;
        let route = Route::root().with_id(world.main());
        let mut styles = StyleChain::default_chain();
        let mut rules: Arc<[ShowRule]> = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        let mut ctx = EvalContext::new();
        let location = crate::entities::locator::Locator::new().next();
        ctx.in_context = true;
        ctx.current_location = Some(location);
        ctx.replace_context_read_file(Some(world.main()));
        let state = crate::entities::state::State {
            key: "read".into(),
            init: Box::new(Value::Int(1)),
        };
        let body =
            crate::compiler::stdlib::state::state_get(&state, &ctx, Span::detached());
        assert_eq!(
            body.observation_relation(&Ok(Value::Int(1))),
            ObservationRelation::Same
        );
        let mut candidate = crate::entities::introspector::TagIntrospector::empty();
        // A captured request must not be replayed under subsequently changed
        // ambient location/context flags.
        ctx.in_context = false;
        ctx.current_location = None;
        assert_eq!(ctx.context_reads_valid_for(&candidate, &mut engine).unwrap(), true);
        candidate.state.init("read".into(), Value::Int(2), location);
        assert_eq!(ctx.context_reads_valid_for(&candidate, &mut engine).unwrap(), false);
        assert_eq!(ctx.context_reads.borrow().reads.len(), 1);
        assert_eq!(
            body.observation_relation(&Ok(Value::Int(1))),
            ObservationRelation::Same
        );
        assert!(comemo::internal::to_parts_mut_ref(engine.sink).0.is_empty());
    }

    #[test]
    fn p1339_observation_ieee_and_recursive_fields() {
        let nan = Value::Float(f64::from_bits(0x7ff8000000001234));
        assert_eq!(nan.observation_relation(&nan.clone()), ObservationRelation::Same);
        assert_eq!(
            Value::Float(0.0).observation_relation(&Value::Float(-0.0)),
            ObservationRelation::Different
        );
        let a = Value::Content(Content::metadata(nan.clone()));
        let b = Value::Content(Content::metadata(Value::Float(f64::from_bits(
            0x7ff8000000001235,
        ))));
        assert_eq!(a.observation_relation(&a.clone()), ObservationRelation::Same);
        assert_eq!(a.observation_relation(&b), ObservationRelation::Different);
    }

    #[cfg(p1339_observation)]
    #[test]
    fn p1340_binding_actual_relation_prefix_and_lossless_payloads() {
        let world = MemoryWorld { library: Library::new(), book: FontBook::new() };
        let metrics = FixedMetrics;
        let route = Route::root().with_id(world.main());
        let mut styles = StyleChain::default_chain();
        let mut rules: Arc<[ShowRule]> = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world: &world, font_metrics: &metrics, route: route.track(),
            styles: &mut styles, show_rules: &mut rules, active_guards: &mut guards,
            current_file: world.main(), sink: &mut tracked,
        };
        let mut ctx = EvalContext::new();
        let location = crate::entities::locator::Locator::new().next();
        ctx.in_context = true;
        ctx.current_location = Some(location);
        ctx.replace_context_read_styles(engine.styles.clone());
        let state = crate::entities::state::State { key: "observed".into(), init: Box::new(Value::Int(1)) };
        for _ in 0..2 {
            crate::compiler::stdlib::state::state_get(&state, &ctx, Span::detached()).unwrap();
        }
        let mut candidate = crate::entities::introspector::TagIntrospector::empty();
        assert!(ctx.context_reads_valid_for(&candidate, &mut engine).unwrap());
        let first = ctx.p1340_observation_replays();
        assert_eq!(first.as_array().unwrap().len(), 2);
        assert!(first.as_array().unwrap().iter().all(|row| row["relation"] == "Same" && row["phase"] == "validation"));
        candidate.state.init("observed".into(), Value::Int(2), location);
        assert!(!ctx.context_reads_valid_for(&candidate, &mut engine).unwrap());
        let second = ctx.p1340_observation_replays();
        assert_eq!(second.as_array().unwrap().len(), 3);
        assert_eq!(second[2]["relation"], "Different");
        assert_eq!(second[2]["result"], serde_json::json!({"kind":"Ok","value":2}));
        let recorded = ctx.p1340_observation_requests();
        assert_eq!(recorded.as_array().unwrap().len(), 2);
        assert_eq!(recorded[0]["request_id"], second[2]["request_id"]);
        assert_eq!(recorded[0]["chain_id"], second[2]["chain_id"]);
        assert_eq!(recorded[0]["result"], serde_json::json!({"kind":"Ok","value":1}));
        assert!(comemo::internal::to_parts_mut_ref(engine.sink).0.is_empty());

        let high = crate::entities::location::Location::from_raw(u128::MAX);
        let fold = Value::Array(vec![Value::Array(vec![Value::Location(high), Value::Array(vec![Value::Int(9)])])]);
        assert_eq!(p1340_observation_value(&fold), serde_json::json!([[{"Location":u128::MAX.to_string()},[9]]]));
        let span = Span::from_raw(std::num::NonZeroU64::new(u64::MAX).unwrap());
        assert_eq!(p1340_observation_span(span)["raw_span"], u64::MAX.to_string());
        let mut diagnostic = SourceDiagnostic::error(span, "original").with_hint("ordered hint");
        diagnostic.trace.push(crate::entities::span::Spanned::new(crate::entities::source_result::Tracepoint::Call(None), span));
        let projected = p1340_observation_diagnostics(&[diagnostic]);
        assert_eq!(projected[0]["trace"][0]["span"]["raw_span"], u64::MAX.to_string());
        assert!(projected[0]["trace"][0]["payload"].is_null());
    }

    #[test]
    fn p1339_observation_records_before_selection_and_preserves_error() {
        let ctx = EvalContext::new();
        let result = Err(vec![SourceDiagnostic::error(Span::detached(), "read failed")]);
        let returned = ctx.observe_context_read(
            ContextReadRequest::Here,
            Span::detached(),
            result.clone(),
        );
        assert_eq!(returned.observation_relation(&result), ObservationRelation::Same);
        assert!(!ctx.has_filtered_counter_reads());
        assert_eq!(ctx.context_reads.borrow().reads.len(), 1);
    }
}

#[cfg(all(test, p1339_observation))]
mod p1339_frozen_observation_binding {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-implementation-observer-core-wrapper-r3.rs"
    ));

    fn actual_relation(a: &Value, b: &Value) -> relation_harness::ActualDiscriminant {
        match a.observation_relation(b) {
            ObservationRelation::Same => relation_harness::ActualDiscriminant::Same,
            ObservationRelation::Different => {
                relation_harness::ActualDiscriminant::Different
            }
            ObservationRelation::Unproven => {
                relation_harness::ActualDiscriminant::Unproven
            }
        }
    }

    struct StyleHooks {
        sources: BTreeMap<u16, String>,
    }
    impl StyleHooks {
        fn new() -> Self {
            // Lossless rendering of the input provenance established by the
            // frozen style harness: source IDs start at three, in operation
            // order. This mapping reads only construction inputs, never
            // expected results, styles, predicates, or a case classification.
            let input: Json = serde_json::from_slice(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../00_nucleo/diagnosticos/p1339-ab-batch1-same-context-style-fixture.json"))).unwrap();
            let sources = input["operations"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|operation| operation["op"] == "eval_expr")
                .enumerate()
                .map(|(index, operation)| {
                    (
                        u16::try_from(index + 3).unwrap(),
                        operation["source_id"].as_str().unwrap().to_owned(),
                    )
                })
                .collect();
            Self { sources }
        }

        fn request(
            &self,
            ctx: &EvalContext,
            read: &Arc<ContextRead>,
            actual_styles: &StyleChain,
            result: &SourceResult<Value>,
        ) -> Json {
            let file = read
                .span
                .id()
                .expect("actual request span lacks source")
                .into_raw()
                .get();
            json!({"actual_request_id":format!("{:p}", Arc::as_ptr(read)),
                "actual_ctx_id":format!("{:p}", &ctx.context_reads),
                "source_id":self.sources.get(&file).expect("actual request file is not in construction provenance"),
                "chain_identity":match actual_styles.p1339_observation_identity() {Some(identity)=>format!("{identity:#x}"),None=>"empty".to_owned()},
                "chain_size_pt_bits":format!("{:016x}",actual_styles.size().to_bits()),
                "operation":"CounterFinal",
                "result":match result {Ok(value)=>json!({"kind":"Ok","value":data_json(value)}),Err(ds)=>json!({"kind":"Err","diagnostic_count":ds.len()})}})
        }
    }
    impl style_harness::ActualStyleHooks for StyleHooks {
        fn begin(&mut self, ctx: &mut EvalContext, _: &mut Engine<'_>) {
            ctx.context_reads.borrow_mut().observation =
                ContextObservationTrace::default();
        }
        fn finish(&mut self, ctx: &EvalContext, engine: &Engine<'_>) -> Json {
            let log = ctx.context_reads.borrow();
            let recorded: Vec<_> = log
                .reads
                .iter()
                .filter(|r| matches!(r.request, ContextReadRequest::CounterFinal { .. }))
                .map(|r| self.request(ctx, r, &r.styles, &r.result))
                .collect();
            let replayed: Vec<_> = log
                .observation
                .replays
                .iter()
                .filter(|replay| {
                    matches!(replay.read.request, ContextReadRequest::CounterFinal { .. })
                })
                .map(|replay| {
                    self.request(ctx, &replay.read, &replay.actual_styles, &replay.result)
                })
                .collect();
            json!({"recorded_requests":recorded,"replayed_requests":replayed,
                "engine_chain_at_validation":{"chain_identity":match engine.styles.p1339_observation_identity() {Some(identity)=>format!("{identity:#x}"),None=>"empty".to_owned()},"chain_size_pt_bits":format!("{:016x}",engine.styles.size().to_bits())},
                // Read the real caller's validation sink through comemo's
                // read-only test surface; never synthesize an empty count.
                "validation_sink_publications":comemo::internal::to_parts_mut_ref(engine.sink).0.clone().into_diagnostics().len()})
        }
    }

    struct ProjectionHooks;
    impl projection_harness::ActualProjectionHooks for ProjectionHooks {
        fn begin(&mut self, ctx: &mut EvalContext) {
            ctx.context_reads.borrow_mut().observation =
                ContextObservationTrace::default();
        }
        fn phase(&mut self, ctx: &mut EvalContext, phase: &str) {
            ctx.context_reads.borrow_mut().observation.phase = match phase {
                "body" => ContextObservationPhase::Body,
                "validation" => ContextObservationPhase::Validation,
                "diagnostics" => ContextObservationPhase::Diagnostics,
                _ => panic!("unknown test phase"),
            };
        }
        fn finish(&mut self, ctx: &EvalContext) -> Json {
            let log = ctx.context_reads.borrow();
            let count = |phase, kind| {
                log.observation
                    .callbacks
                    .iter()
                    .filter(|entry| **entry == (phase, kind))
                    .count()
            };
            let operations: Vec<_> = log
                .reads
                .iter()
                .filter_map(|read| match read.request {
                    ContextReadRequest::StateDisplay { .. } => {
                        Some("state_display_value")
                    }
                    ContextReadRequest::CounterFinal { .. } => Some("counter_final"),
                    _ => None,
                })
                .collect();
            json!({"display_callback_invocations_during_body":count(ContextObservationPhase::Body,ContextCallbackKind::StateDisplay),
                "display_callback_invocations_during_validation":count(ContextObservationPhase::Validation,ContextCallbackKind::StateDisplay),
                "display_callback_invocations_during_diagnostics":count(ContextObservationPhase::Diagnostics,ContextCallbackKind::StateDisplay),
                "numbering_callback_invocations_body":count(ContextObservationPhase::Body,ContextCallbackKind::PageNumbering),
                "numbering_callback_invocations_validation":count(ContextObservationPhase::Validation,ContextCallbackKind::PageNumbering),
                "numbering_callback_invocations_diagnostics":count(ContextObservationPhase::Diagnostics,ContextCallbackKind::PageNumbering),
                "recorded_operations_in_order":operations,"counter_request_selector_contains_element":log.selected,
                "retained_body_error_not_replaced_by_validation_success":matches!(log.observation.body_result,Some(Err(_)))})
        }
    }

    #[test]
    fn p1339_frozen_style_bound_matrix() {
        style_harness::run_style_matrix(|| Box::new(StyleHooks::new()));
    }

    #[test]
    fn p1339_frozen_core_bound_matrix() {
        // Only the test harness consumes driver provenance. No environment
        // access, executable identity or oracle reaches the productive relation.
        let audit: Json = serde_json::from_str(
            &std::env::var("P1339_BINDING_AUDIT_JSON")
                .expect("independent binding audit required"),
        )
        .unwrap();
        let source = audit["port_source_pins"]
            .as_array()
            .unwrap()
            .iter()
            .find(|pin| {
                pin["path"]
                    .as_str()
                    .unwrap()
                    .ends_with("01_core/src/compiler/eval/mod.rs")
            })
            .expect("observer source pin missing");
        let source_line = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/compiler/eval/mod.rs"
        ))
        .lines()
        .position(|line| {
            line.starts_with("impl ObservationEq for crate::entities::value::Value")
        })
        .expect("actual productive Value relation missing")
            + 1;
        let identity = json!({"source_path":source["path"],"source_sha256":source["sha256"],
            "source_line":source_line,"operation":"ObservationEq::observation_relation for Value",
            "binary_sha256":std::env::var("P1339_TEST_BINARY_SHA256").unwrap(),
            "config_sha256":std::env::var("P1339_COMPILED_CONFIGURATION_SHA256").unwrap()});
        run_bound_core(
            actual_relation,
            &identity,
            || Box::new(StyleHooks::new()),
            || Box::new(ProjectionHooks),
        );
    }
}
// Mechanical declaration expansion: exhaustive matches and all-field destructuring.
impl ObservationEq for crate::entities::content::Content {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Empty => match other {
                Self::Empty => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Text(a0) => match other {
                Self::Text(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Space => match other {
                Self::Space => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parbreak => match other {
                Self::Parbreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Sequence(a0) => match other {
                Self::Sequence(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::HtmlElem(a0) => match other {
                Self::HtmlElem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Par { body: a0 } => match other {
                Self::Par { body: b0 } => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Heading(a0) => match other {
                Self::Heading(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Title(a0) => match other {
                Self::Title(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Strong(a0) => match other {
                Self::Strong(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Emph(a0) => match other {
                Self::Emph(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Raw(a0) => match other {
                Self::Raw(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::ListItem(a0) => match other {
                Self::ListItem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::EnumItem(a0) => match other {
                Self::EnumItem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Link(a0) => match other {
                Self::Link(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Equation(a0) => match other {
                Self::Equation(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathSequence(a0) => match other {
                Self::MathSequence(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathIdent(a0) => match other {
                Self::MathIdent(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathText(a0) => match other {
                Self::MathText(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathFrac(a0) => match other {
                Self::MathFrac(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathAttach(a0) => match other {
                Self::MathAttach(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathRoot(a0) => match other {
                Self::MathRoot(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathDelimited(a0) => match other {
                Self::MathDelimited(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathAlignPoint(a0) => match other {
                Self::MathAlignPoint(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Linebreak(a0) => match other {
                Self::Linebreak(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathMatrix(a0) => match other {
                Self::MathMatrix(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathCases(a0) => match other {
                Self::MathCases(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathAccent(a0) => match other {
                Self::MathAccent(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathCancel(a0) => match other {
                Self::MathCancel(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderline(a0) => match other {
                Self::MathUnderline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathVec(a0) => match other {
                Self::MathVec(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathClassOverride(a0) => match other {
                Self::MathClassOverride(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathLimitsOverride(a0) => match other {
                Self::MathLimitsOverride(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderover(a0) => match other {
                Self::MathUnderover(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOp(a0) => match other {
                Self::MathOp(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathStyled(a0) => match other {
                Self::MathStyled(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Ref(a0) => match other {
                Self::Ref(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CounterDisplay(a0) => match other {
                Self::CounterDisplay(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CounterUpdate(a0) => match other {
                Self::CounterUpdate(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Outline(a0) => match other {
                Self::Outline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Figure(a0) => match other {
                Self::Figure(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Image(a0) => match other {
                Self::Image(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Shape(a0) => match other {
                Self::Shape(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Curve(a0) => match other {
                Self::Curve(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Transform(a0) => match other {
                Self::Transform(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Grid(a0) => match other {
                Self::Grid(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridHeader(a0) => match other {
                Self::GridHeader(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridFooter(a0) => match other {
                Self::GridFooter(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridCell(a0) => match other {
                Self::GridCell(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::SetPage {
                paper: a0,
                flipped: a1,
                binding: a2,
                width: a3,
                height: a4,
                margin: a5,
                numbering: a6,
                number_align: a7,
                header: a8,
                header_ascent: a9,
                footer: a10,
                footer_descent: a11,
                supplement: a12,
                columns: a13,
                bleed: a14,
                fill: a15,
                background: a16,
                foreground: a17,
            } => match other {
                Self::SetPage {
                    paper: b0,
                    flipped: b1,
                    binding: b2,
                    width: b3,
                    height: b4,
                    margin: b5,
                    numbering: b6,
                    number_align: b7,
                    header: b8,
                    header_ascent: b9,
                    footer: b10,
                    footer_descent: b11,
                    supplement: b12,
                    columns: b13,
                    bleed: b14,
                    fill: b15,
                    background: b16,
                    foreground: b17,
                } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3))
                    .and(a4.observation_relation(b4))
                    .and(a5.observation_relation(b5))
                    .and(a6.observation_relation(b6))
                    .and(a7.observation_relation(b7))
                    .and(a8.observation_relation(b8))
                    .and(a9.observation_relation(b9))
                    .and(a10.observation_relation(b10))
                    .and(a11.observation_relation(b11))
                    .and(a12.observation_relation(b12))
                    .and(a13.observation_relation(b13))
                    .and(a14.observation_relation(b14))
                    .and(a15.observation_relation(b15))
                    .and(a16.observation_relation(b16))
                    .and(a17.observation_relation(b17)),
                _ => ObservationRelation::Different,
            },
            Self::PageRun(a0) => match other {
                Self::PageRun(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Align(a0) => match other {
                Self::Align(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Place(a0) => match other {
                Self::Place(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Flush(a0) => match other {
                Self::Flush(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Styled(a0, a1) => match other {
                Self::Styled(b0, b1) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Divider(a0) => match other {
                Self::Divider(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Terms(a0) => match other {
                Self::Terms(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TermItem(a0) => match other {
                Self::TermItem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Quote(a0) => match other {
                Self::Quote(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Document { title: a0, author: a1, date: a2, keywords: a3 } => {
                match other {
                    Self::Document { title: b0, author: b1, date: b2, keywords: b3 } => {
                        ObservationRelation::Same
                            .and(a0.observation_relation(b0))
                            .and(a1.observation_relation(b1))
                            .and(a2.observation_relation(b2))
                            .and(a3.observation_relation(b3))
                    }
                    _ => ObservationRelation::Different,
                }
            }
            Self::Asset { path: a0, kind: a1 } => match other {
                Self::Asset { path: b0, kind: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::SmartQuote(a0) => match other {
                Self::SmartQuote(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::PdfAttach(a0) => match other {
                Self::PdfAttach(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::PdfArtifact(a0) => match other {
                Self::PdfArtifact(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Underline(a0) => match other {
                Self::Underline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Strike(a0) => match other {
                Self::Strike(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Overline(a0) => match other {
                Self::Overline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::SmallCaps { body: a0 } => match other {
                Self::SmallCaps { body: b0 } => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Pad(a0) => match other {
                Self::Pad(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Hide(a0) => match other {
                Self::Hide(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::HSpace(a0) => match other {
                Self::HSpace(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::VSpace(a0) => match other {
                Self::VSpace(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Pagebreak(a0) => match other {
                Self::Pagebreak(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Colbreak(a0) => match other {
                Self::Colbreak(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Stack(a0) => match other {
                Self::Stack(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Boxed(a0) => match other {
                Self::Boxed(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Block(a0) => match other {
                Self::Block(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableCell(a0) => match other {
                Self::TableCell(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Bibliography(a0) => match other {
                Self::Bibliography(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Cite(a0) => match other {
                Self::Cite(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Footnote(a0) => match other {
                Self::Footnote(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableHeader(a0) => match other {
                Self::TableHeader(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableFooter(a0) => match other {
                Self::TableFooter(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridHLine(a0) => match other {
                Self::GridHLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridVLine(a0) => match other {
                Self::GridVLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableHLine(a0) => match other {
                Self::TableHLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableVLine(a0) => match other {
                Self::TableVLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Table(a0) => match other {
                Self::Table(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Repeat(a0) => match other {
                Self::Repeat(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Columns(a0) => match other {
                Self::Columns(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Metadata(a0) => match other {
                Self::Metadata(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::State(a0) => match other {
                Self::State(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::StateUpdate(a0) => match other {
                Self::StateUpdate(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::StateDisplay(a0) => match other {
                Self::StateDisplay(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CounterDisplayCallback(a0) => match other {
                Self::CounterDisplayCallback(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::ContextBlock(a0) => match other {
                Self::ContextBlock(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Dynamic(a0) => match other {
                Self::Dynamic(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::align::AlignElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { alignment: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.alignment.observation_relation(&other.alignment))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::layout_types::Align2D {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { h: _, v: _ } = self;
        ObservationRelation::Same
            .and(self.h.observation_relation(&other.h))
            .and(self.v.observation_relation(&other.v))
    }
}
impl ObservationEq for crate::entities::layout_types::HAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Left => match other {
                Self::Left => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Center => match other {
                Self::Center => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Right => match other {
                Self::Right => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Start => match other {
                Self::Start => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::End => match other {
                Self::End => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::VAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Top => match other {
                Self::Top => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Horizon => match other {
                Self::Horizon => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bottom => match other {
                Self::Bottom => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_styled::MathStyledElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { kind: _, bold: _, italic: _, body: _, cramped: _ } = self;
        ObservationRelation::Same
            .and(self.kind.observation_relation(&other.kind))
            .and(self.bold.observation_relation(&other.bold))
            .and(self.italic.observation_relation(&other.italic))
            .and(self.body.observation_relation(&other.body))
            .and(self.cramped.observation_relation(&other.cramped))
    }
}
impl ObservationEq for crate::entities::math_style::MathStyleKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Plain => match other {
                Self::Plain => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SansSerif => match other {
                Self::SansSerif => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Chancery => match other {
                Self::Chancery => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Roundhand => match other {
                Self::Roundhand => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fraktur => match other {
                Self::Fraktur => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Monospace => match other {
                Self::Monospace => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::DoubleStruck => match other {
                Self::DoubleStruck => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Script => match other {
                Self::Script => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SScript => match other {
                Self::SScript => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Display => match other {
                Self::Display => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Inline => match other {
                Self::Inline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::style_chain::StyleDelta {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            bold: _,
            bold_from_strong: _,
            italic: _,
            italic_from_emph: _,
            size: _,
            fill: _,
            heading_level: _,
            weight: _,
            tracking: _,
            leading: _,
            top_edge: _,
            bottom_edge: _,
            lang: _,
            font: _,
            subscript: _,
            superscript: _,
            highlight: _,
            highlight_radius: _,
            highlight_extent: _,
            subscript_size: _,
            superscript_size: _,
            custom: _,
        } = self;
        ObservationRelation::Same
            .and(self.bold.observation_relation(&other.bold))
            .and(self.bold_from_strong.observation_relation(&other.bold_from_strong))
            .and(self.italic.observation_relation(&other.italic))
            .and(self.italic_from_emph.observation_relation(&other.italic_from_emph))
            .and(self.size.observation_relation(&other.size))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.heading_level.observation_relation(&other.heading_level))
            .and(self.weight.observation_relation(&other.weight))
            .and(self.tracking.observation_relation(&other.tracking))
            .and(self.leading.observation_relation(&other.leading))
            .and(self.top_edge.observation_relation(&other.top_edge))
            .and(self.bottom_edge.observation_relation(&other.bottom_edge))
            .and(self.lang.observation_relation(&other.lang))
            .and(self.font.observation_relation(&other.font))
            .and(self.subscript.observation_relation(&other.subscript))
            .and(self.superscript.observation_relation(&other.superscript))
            .and(self.highlight.observation_relation(&other.highlight))
            .and(self.highlight_radius.observation_relation(&other.highlight_radius))
            .and(self.highlight_extent.observation_relation(&other.highlight_extent))
            .and(self.subscript_size.observation_relation(&other.subscript_size))
            .and(self.superscript_size.observation_relation(&other.superscript_size))
            .and(self.custom.observation_relation(&other.custom))
    }
}
impl ObservationEq for crate::entities::value::Value {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool(a0) => match other {
                Self::Bool(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Int(a0) => match other {
                Self::Int(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Float(a0) => match other {
                Self::Float(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Str(a0) => match other {
                Self::Str(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Array(a0) => match other {
                Self::Array(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Dict(a0) => match other {
                Self::Dict(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Module(a0) => match other {
                Self::Module(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Datetime(a0) => match other {
                Self::Datetime(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::LocatedContent(a0, a1) => match other {
                Self::LocatedContent(b0, b1) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Relative(a0) => match other {
                Self::Relative(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Ratio(a0) => match other {
                Self::Ratio(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Angle(a0) => match other {
                Self::Angle(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Color(a0) => match other {
                Self::Color(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Stroke(a0) => match other {
                Self::Stroke(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Fraction(a0) => match other {
                Self::Fraction(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Align(a0) => match other {
                Self::Align(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Location(a0) => match other {
                Self::Location(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Gradient(a0) => match other {
                Self::Gradient(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Regex(a0) => match other {
                Self::Regex(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Tiling(a0) => match other {
                Self::Tiling(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Bytes(a0) => match other {
                Self::Bytes(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Decimal(a0) => match other {
                Self::Decimal(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Duration(a0) => match other {
                Self::Duration(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Version(a0) => match other {
                Self::Version(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Selector(a0) => match other {
                Self::Selector(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Symbol(a0) => match other {
                Self::Symbol(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Args(a0) => match other {
                Self::Args(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::State(a0) => match other {
                Self::State(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Counter(a0) => match other {
                Self::Counter(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Dir(a0) => match other {
                Self::Dir(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Path(a0) => match other {
                Self::Path(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Type(a0) => match other {
                Self::Type(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::path::VirtualRoot {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Project => match other {
                Self::Project => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Package(a0) => match other {
                Self::Package(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::package_spec::PackageSpec {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { namespace: _, name: _, version: _ } = self;
        ObservationRelation::Same
            .and(self.namespace.observation_relation(&other.namespace))
            .and(self.name.observation_relation(&other.name))
            .and(self.version.observation_relation(&other.version))
    }
}
impl ObservationEq for crate::entities::package_spec::PackageVersion {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { major: _, minor: _, patch: _ } = self;
        ObservationRelation::Same
            .and(self.major.observation_relation(&other.major))
            .and(self.minor.observation_relation(&other.minor))
            .and(self.patch.observation_relation(&other.patch))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::rel::Rel<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { rel: _, abs: _ } = self;
        ObservationRelation::Same
            .and(self.rel.observation_relation(&other.rel))
            .and(self.abs.observation_relation(&other.abs))
    }
}
impl ObservationEq for crate::entities::layout_types::Length {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { abs: _, em: _ } = self;
        ObservationRelation::Same
            .and(self.abs.observation_relation(&other.abs))
            .and(self.em.observation_relation(&other.em))
    }
}
impl ObservationEq for crate::entities::layout_types::Abs {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl ObservationEq for crate::entities::layout_types::TextEdge {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Metric(a0) => match other {
                Self::Metric(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::color::Color {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Srgb { r: a0, g: a1, b: a2, a: a3 } => match other {
                Self::Srgb { r: b0, g: b1, b: b2, a: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Luma { l: a0, a: a1 } => match other {
                Self::Luma { l: b0, a: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::LinearRgb { r: a0, g: a1, b: a2, a: a3 } => match other {
                Self::LinearRgb { r: b0, g: b1, b: b2, a: b3 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                        .and(a3.observation_relation(b3))
                }
                _ => ObservationRelation::Different,
            },
            Self::Oklab { l: a0, a: a1, b: a2, alpha: a3 } => match other {
                Self::Oklab { l: b0, a: b1, b: b2, alpha: b3 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                        .and(a3.observation_relation(b3))
                }
                _ => ObservationRelation::Different,
            },
            Self::Oklch { l: a0, c: a1, h: a2, alpha: a3 } => match other {
                Self::Oklch { l: b0, c: b1, h: b2, alpha: b3 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                        .and(a3.observation_relation(b3))
                }
                _ => ObservationRelation::Different,
            },
            Self::Hsl { h: a0, s: a1, l: a2, a: a3 } => match other {
                Self::Hsl { h: b0, s: b1, l: b2, a: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Hsv { h: a0, s: a1, v: a2, a: a3 } => match other {
                Self::Hsv { h: b0, s: b1, v: b2, a: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Cmyk { c: a0, m: a1, y: a2, k: a3 } => match other {
                Self::Cmyk { c: b0, m: b1, y: b2, k: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::font_list::FontFamily {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            name: _,
            variants: _,
            variant: _,
            weight: _,
            style: _,
            covers: _,
        } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.variants.observation_relation(&other.variants))
            .and(self.variant.observation_relation(&other.variant))
            .and(self.weight.observation_relation(&other.weight))
            .and(self.style.observation_relation(&other.style))
            .and(self.covers.observation_relation(&other.covers))
    }
}
impl ObservationEq for crate::entities::font_list::FontNamePattern {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Literal(a0) => match other {
                Self::Literal(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Regex(a0) => match other {
                Self::Regex(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::font_list::Covers {
    fn observation_relation(&self, _other: &Self) -> ObservationRelation {
        match *self {}
    }
}
impl ObservationEq for crate::entities::elements::underline::UnderlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, stroke: _, offset: _, extent: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.extent.observation_relation(&other.extent))
    }
}
impl ObservationEq for crate::entities::elements::v_space::VSpaceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { amount: _, weak: _, weak_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.amount.observation_relation(&other.amount))
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
    }
}
impl ObservationEq for crate::entities::elements::enum_item::EnumItemElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            number: _,
            body: _,
            numbering: _,
            indent: _,
            body_indent: _,
            tight: _,
        } = self;
        ObservationRelation::Same
            .and(self.number.observation_relation(&other.number))
            .and(self.body.observation_relation(&other.body))
            .and(self.numbering.observation_relation(&other.numbering))
            .and(self.indent.observation_relation(&other.indent))
            .and(self.body_indent.observation_relation(&other.body_indent))
            .and(self.tight.observation_relation(&other.tight))
    }
}
impl ObservationEq for crate::entities::enum_numbering::EnumNumbering {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Decimal => match other {
                Self::Decimal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LowerAlpha => match other {
                Self::LowerAlpha => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::UpperAlpha => match other {
                Self::UpperAlpha => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LowerRoman => match other {
                Self::LowerRoman => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Custom(a0) => match other {
                Self::Custom(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::table_footer::TableFooterElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::footnote::FootnoteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, numbering: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.numbering.observation_relation(&other.numbering))
    }
}
impl ObservationEq for crate::entities::elements::shape::ShapeElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            kind: _,
            width: _,
            height: _,
            fill: _,
            stroke: _,
            fill_rule: _,
        } = self;
        ObservationRelation::Same
            .and(self.kind.observation_relation(&other.kind))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill_rule.observation_relation(&other.fill_rule))
    }
}
impl ObservationEq for crate::entities::paint::Paint {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Solid(a0) => match other {
                Self::Solid(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Gradient(a0) => match other {
                Self::Gradient(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Tiling(a0) => match other {
                Self::Tiling(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::Stroke {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            paint: _,
            thickness: _,
            cap: _,
            join: _,
            dash: _,
            miter_limit: _,
            specified: _,
            overhang: _,
        } = self;
        ObservationRelation::Same
            .and(self.paint.observation_relation(&other.paint))
            .and(self.thickness.observation_relation(&other.thickness))
            .and(self.cap.observation_relation(&other.cap))
            .and(self.join.observation_relation(&other.join))
            .and(self.dash.observation_relation(&other.dash))
            .and(self.miter_limit.observation_relation(&other.miter_limit))
            .and(self.specified.observation_relation(&other.specified))
            .and(self.overhang.observation_relation(&other.overhang))
    }
}
impl ObservationEq for crate::entities::geometry::DashPattern {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { array: _, phase: _ } = self;
        ObservationRelation::Same
            .and(self.array.observation_relation(&other.array))
            .and(self.phase.observation_relation(&other.phase))
    }
}
impl ObservationEq for crate::entities::geometry::DashLength {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::LineWidth => match other {
                Self::LineWidth => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::LineJoin {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Miter => match other {
                Self::Miter => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Round => match other {
                Self::Round => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bevel => match other {
                Self::Bevel => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::StrokeFields {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            paint: _,
            thickness: _,
            cap: _,
            join: _,
            dash: _,
            miter_limit: _,
        } = self;
        ObservationRelation::Same
            .and(self.paint.observation_relation(&other.paint))
            .and(self.thickness.observation_relation(&other.thickness))
            .and(self.cap.observation_relation(&other.cap))
            .and(self.join.observation_relation(&other.join))
            .and(self.dash.observation_relation(&other.dash))
            .and(self.miter_limit.observation_relation(&other.miter_limit))
    }
}
impl ObservationEq for crate::entities::geometry::LineCap {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Butt => match other {
                Self::Butt => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Round => match other {
                Self::Round => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Square => match other {
                Self::Square => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::FillRule {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::NonZero => match other {
                Self::NonZero => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EvenOdd => match other {
                Self::EvenOdd => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::geometry::ShapeKind<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Rect => match other {
                Self::Rect => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RoundedRect { radii: a0 } => match other {
                Self::RoundedRect { radii: b0 } => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Ellipse => match other {
                Self::Ellipse => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Line { dx: a0, dy: a1 } => match other {
                Self::Line { dx: b0, dy: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Path(a0) => match other {
                Self::Path(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::PathItem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::MoveTo(a0) => match other {
                Self::MoveTo(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::LineTo(a0) => match other {
                Self::LineTo(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CubicTo(a0, a1, a2) => match other {
                Self::CubicTo(b0, b1, b2) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2)),
                _ => ObservationRelation::Different,
            },
            Self::ClosePath => match other {
                Self::ClosePath => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::Point {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { x: _, y: _ } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
    }
}
impl ObservationEq for crate::entities::layout_types::Pt {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::corners::Corners<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            top_left: _,
            top_right: _,
            bottom_right: _,
            bottom_left: _,
        } = self;
        ObservationRelation::Same
            .and(self.top_left.observation_relation(&other.top_left))
            .and(self.top_right.observation_relation(&other.top_right))
            .and(self.bottom_right.observation_relation(&other.bottom_right))
            .and(self.bottom_left.observation_relation(&other.bottom_left))
    }
}
impl ObservationEq for crate::entities::elements::math_matrix::MathMatrixElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            rows: _,
            delim: _,
            row_gap: _,
            column_gap: _,
            gap: _,
            augment: _,
        } = self;
        ObservationRelation::Same
            .and(self.rows.observation_relation(&other.rows))
            .and(self.delim.observation_relation(&other.delim))
            .and(self.row_gap.observation_relation(&other.row_gap))
            .and(self.column_gap.observation_relation(&other.column_gap))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.augment.observation_relation(&other.augment))
    }
}
impl ObservationEq for crate::entities::elements::grid::GridElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            columns: _,
            rows: _,
            cells: _,
            hlines: _,
            vlines: _,
            gutter: _,
            align: _,
            inset: _,
            header: _,
            footer: _,
            stroke: _,
            fill: _,
        } = self;
        ObservationRelation::Same
            .and(self.columns.observation_relation(&other.columns))
            .and(self.rows.observation_relation(&other.rows))
            .and(self.cells.observation_relation(&other.cells))
            .and(self.hlines.observation_relation(&other.hlines))
            .and(self.vlines.observation_relation(&other.vlines))
            .and(self.gutter.observation_relation(&other.gutter))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.header.observation_relation(&other.header))
            .and(self.footer.observation_relation(&other.footer))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::sides::Sides<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { left: _, top: _, right: _, bottom: _ } = self;
        ObservationRelation::Same
            .and(self.left.observation_relation(&other.left))
            .and(self.top.observation_relation(&other.top))
            .and(self.right.observation_relation(&other.right))
            .and(self.bottom.observation_relation(&other.bottom))
    }
}
impl ObservationEq for crate::entities::elements::grid_hline::GridHLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, row: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.row.observation_relation(&other.row))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::layout_types::TrackSizing {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Fixed(a0) => match other {
                Self::Fixed(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fraction(a0) => match other {
                Self::Fraction(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::grid_vline::GridVLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, col: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.col.observation_relation(&other.col))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::counter_update::CounterUpdateElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, action: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.action.observation_relation(&other.action))
    }
}
impl ObservationEq for crate::entities::counter::CounterKey {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Page => match other {
                Self::Page => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Selector(a0) => match other {
                Self::Selector(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Str(a0) => match other {
                Self::Str(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::label::LabelElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, body: _, auto: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.body.observation_relation(&other.body))
            .and(self.auto.observation_relation(&other.auto))
    }
}
impl ObservationEq for crate::entities::elements::pdf_artifact::PdfArtifactElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { kind: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.kind.observation_relation(&other.kind))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::pdf_artifact::ArtifactKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Header => match other {
                Self::Header => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Footer => match other {
                Self::Footer => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Watermark => match other {
                Self::Watermark => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::PageNumber => match other {
                Self::PageNumber => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LineNumber => match other {
                Self::LineNumber => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Redaction => match other {
                Self::Redaction => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bates => match other {
                Self::Bates => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Page => match other {
                Self::Page => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::PaginationOther => match other {
                Self::PaginationOther => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Layout => match other {
                Self::Layout => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Background => match other {
                Self::Background => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Other => match other {
                Self::Other => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { double: _, alternative: _, quotes: _ } = self;
        ObservationRelation::Same
            .and(self.double.observation_relation(&other.double))
            .and(self.alternative.observation_relation(&other.alternative))
            .and(self.quotes.observation_relation(&other.quotes))
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteQuotes {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Custom(a0) => match other {
                Self::Custom(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteOverrides {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { single: _, double: _ } = self;
        ObservationRelation::Same
            .and(self.single.observation_relation(&other.single))
            .and(self.double.observation_relation(&other.double))
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuotePair {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { open: _, close: _ } = self;
        ObservationRelation::Same
            .and(self.open.observation_relation(&other.open))
            .and(self.close.observation_relation(&other.close))
    }
}
impl ObservationEq
    for crate::entities::elements::math_limits_override::MathLimitsOverrideElem
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, limits: _, inline: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.limits.observation_relation(&other.limits))
            .and(self.inline.observation_relation(&other.inline))
    }
}
impl ObservationEq for crate::entities::elements::r#ref::RefElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, supplement: _, form: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.supplement.observation_relation(&other.supplement))
            .and(self.form.observation_relation(&other.form))
    }
}
impl ObservationEq for crate::entities::elements::r#ref::RefForm {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Normal => match other {
                Self::Normal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Page => match other {
                Self::Page => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::page_run::PageRunElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            paper: _,
            flipped: _,
            binding: _,
            width: _,
            height: _,
            margin: _,
            numbering: _,
            number_align: _,
            header: _,
            header_ascent: _,
            footer: _,
            footer_descent: _,
            supplement: _,
            columns: _,
            bleed: _,
            fill: _,
            background: _,
            foreground: _,
            body: _,
        } = self;
        ObservationRelation::Same
            .and(self.paper.observation_relation(&other.paper))
            .and(self.flipped.observation_relation(&other.flipped))
            .and(self.binding.observation_relation(&other.binding))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.margin.observation_relation(&other.margin))
            .and(self.numbering.observation_relation(&other.numbering))
            .and(self.number_align.observation_relation(&other.number_align))
            .and(self.header.observation_relation(&other.header))
            .and(self.header_ascent.observation_relation(&other.header_ascent))
            .and(self.footer.observation_relation(&other.footer))
            .and(self.footer_descent.observation_relation(&other.footer_descent))
            .and(self.supplement.observation_relation(&other.supplement))
            .and(self.columns.observation_relation(&other.columns))
            .and(self.bleed.observation_relation(&other.bleed))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.background.observation_relation(&other.background))
            .and(self.foreground.observation_relation(&other.foreground))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::page_canvas::PageBleedSpec {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { left: _, right: _, top: _, bottom: _, two_sided: _ } = self;
        ObservationRelation::Same
            .and(self.left.observation_relation(&other.left))
            .and(self.right.observation_relation(&other.right))
            .and(self.top.observation_relation(&other.top))
            .and(self.bottom.observation_relation(&other.bottom))
            .and(self.two_sided.observation_relation(&other.two_sided))
    }
}
impl ObservationEq for crate::entities::page_running::PageNumberAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { horizontal: _, vertical: _ } = self;
        ObservationRelation::Same
            .and(self.horizontal.observation_relation(&other.horizontal))
            .and(self.vertical.observation_relation(&other.vertical))
    }
}
impl ObservationEq for crate::entities::page_running::PageNumberVAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Top => match other {
                Self::Top => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bottom => match other {
                Self::Bottom => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::numbering::Numbering {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Pattern(a0) => match other {
                Self::Pattern(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_geometry::PageBinding {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Left => match other {
                Self::Left => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Right => match other {
                Self::Right => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_supplement::PageSupplement {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::PageMarginSpec {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { left: _, right: _, top: _, bottom: _, two_sided: _ } = self;
        ObservationRelation::Same
            .and(self.left.observation_relation(&other.left))
            .and(self.right.observation_relation(&other.right))
            .and(self.top.observation_relation(&other.top))
            .and(self.bottom.observation_relation(&other.bottom))
            .and(self.two_sided.observation_relation(&other.two_sided))
    }
}
impl ObservationEq for crate::entities::layout_types::PageDimension {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_running::PageMarginal {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_canvas::PageFill {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Paint(a0) => match other {
                Self::Paint(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::columns::ColumnsElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { count: _, gutter: _, body: _, page_columns: _ } = self;
        ObservationRelation::Same
            .and(self.count.observation_relation(&other.count))
            .and(self.gutter.observation_relation(&other.gutter))
            .and(self.body.observation_relation(&other.body))
            .and(self.page_columns.observation_relation(&other.page_columns))
    }
}
impl ObservationEq for crate::entities::elements::strong::StrongElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::math_delimited::MathDelimitedElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { open: _, body: _, close: _ } = self;
        ObservationRelation::Same
            .and(self.open.observation_relation(&other.open))
            .and(self.body.observation_relation(&other.body))
            .and(self.close.observation_relation(&other.close))
    }
}
impl ObservationEq for crate::entities::elements::raw::RawElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { text: _, lang: _, block: _ } = self;
        ObservationRelation::Same
            .and(self.text.observation_relation(&other.text))
            .and(self.lang.observation_relation(&other.lang))
            .and(self.block.observation_relation(&other.block))
    }
}
impl ObservationEq for crate::entities::elements::bibliography::BibliographyElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { entries: _, path: _, title: _, style: _, locale: _ } = self;
        ObservationRelation::Same
            .and(self.entries.observation_relation(&other.entries))
            .and(self.path.observation_relation(&other.path))
            .and(self.title.observation_relation(&other.title))
            .and(self.style.observation_relation(&other.style))
            .and(self.locale.observation_relation(&other.locale))
    }
}
impl ObservationEq for crate::entities::bib_entry::BibEntry {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            key: _,
            author: _,
            title: _,
            year: _,
            volume: _,
            pages: _,
            journal: _,
            publisher: _,
            url: _,
            doi: _,
            editor: _,
            series: _,
            note: _,
            isbn: _,
            location: _,
            organization: _,
        } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.author.observation_relation(&other.author))
            .and(self.title.observation_relation(&other.title))
            .and(self.year.observation_relation(&other.year))
            .and(self.volume.observation_relation(&other.volume))
            .and(self.pages.observation_relation(&other.pages))
            .and(self.journal.observation_relation(&other.journal))
            .and(self.publisher.observation_relation(&other.publisher))
            .and(self.url.observation_relation(&other.url))
            .and(self.doi.observation_relation(&other.doi))
            .and(self.editor.observation_relation(&other.editor))
            .and(self.series.observation_relation(&other.series))
            .and(self.note.observation_relation(&other.note))
            .and(self.isbn.observation_relation(&other.isbn))
            .and(self.location.observation_relation(&other.location))
            .and(self.organization.observation_relation(&other.organization))
    }
}
impl ObservationEq for crate::entities::elements::grid_header::GridHeaderElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::pdf_attach::PdfAttachElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            path: _,
            data: _,
            relationship: _,
            mime_type: _,
            description: _,
        } = self;
        ObservationRelation::Same
            .and(self.path.observation_relation(&other.path))
            .and(self.data.observation_relation(&other.data))
            .and(self.relationship.observation_relation(&other.relationship))
            .and(self.mime_type.observation_relation(&other.mime_type))
            .and(self.description.observation_relation(&other.description))
    }
}
impl ObservationEq for crate::entities::elements::pdf_attach::AttachedFileRelationship {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Source => match other {
                Self::Source => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Data => match other {
                Self::Data => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alternative => match other {
                Self::Alternative => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Supplement => match other {
                Self::Supplement => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_op::MathOpElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { text: _, limits: _ } = self;
        ObservationRelation::Same
            .and(self.text.observation_relation(&other.text))
            .and(self.limits.observation_relation(&other.limits))
    }
}
impl ObservationEq for crate::entities::elements::equation::EquationElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, block: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.block.observation_relation(&other.block))
    }
}
impl ObservationEq for crate::entities::elements::table::TableElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            columns: _,
            rows: _,
            children: _,
            hlines: _,
            vlines: _,
            header: _,
            footer: _,
            stroke: _,
            fill: _,
            caption: _,
            summary: _,
            inset: _,
            align: _,
        } = self;
        ObservationRelation::Same
            .and(self.columns.observation_relation(&other.columns))
            .and(self.rows.observation_relation(&other.rows))
            .and(self.children.observation_relation(&other.children))
            .and(self.hlines.observation_relation(&other.hlines))
            .and(self.vlines.observation_relation(&other.vlines))
            .and(self.header.observation_relation(&other.header))
            .and(self.footer.observation_relation(&other.footer))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.caption.observation_relation(&other.caption))
            .and(self.summary.observation_relation(&other.summary))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.align.observation_relation(&other.align))
    }
}
impl ObservationEq for crate::entities::elements::table_vline::TableVLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, col: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.col.observation_relation(&other.col))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::table_hline::TableHLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, row: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.row.observation_relation(&other.row))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::math_frac::MathFracElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { num: _, den: _, line: _ } = self;
        ObservationRelation::Same
            .and(self.num.observation_relation(&other.num))
            .and(self.den.observation_relation(&other.den))
            .and(self.line.observation_relation(&other.line))
    }
}
impl ObservationEq for crate::entities::elements::h_space::HSpaceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { amount: _, weak: _, weak_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.amount.observation_relation(&other.amount))
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
    }
}
impl ObservationEq for crate::entities::elements::h_space::Spacing {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Absolute(a0) => match other {
                Self::Absolute(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Fractional(a0) => match other {
                Self::Fractional(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_vec::MathVecElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            children: _,
            delim: _,
            align: _,
            gap: _,
            explicit: _,
        } = self;
        ObservationRelation::Same
            .and(self.children.observation_relation(&other.children))
            .and(self.delim.observation_relation(&other.delim))
            .and(self.align.observation_relation(&other.align))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.explicit.observation_relation(&other.explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_vec::MathVecExplicit {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { delim: _, align: _, gap: _ } = self;
        ObservationRelation::Same
            .and(self.delim.observation_relation(&other.delim))
            .and(self.align.observation_relation(&other.align))
            .and(self.gap.observation_relation(&other.gap))
    }
}
impl ObservationEq for crate::entities::elements::math_underline::MathUnderlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::context_block::ContextBlockElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { id: _, closure: _ } = self;
        ObservationRelation::Same
            .and(self.id.observation_relation(&other.id))
            .and(self.closure.observation_relation(&other.closure))
    }
}
impl ObservationEq for crate::entities::source_result::SourceDiagnostic {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            severity: _,
            span: _,
            message: _,
            hints: _,
            trace: _,
        } = self;
        ObservationRelation::Same
            .and(self.severity.observation_relation(&other.severity))
            .and(self.span.observation_relation(&other.span))
            .and(self.message.observation_relation(&other.message))
            .and(self.hints.observation_relation(&other.hints))
            .and(self.trace.observation_relation(&other.trace))
    }
}
impl ObservationEq for crate::entities::source_result::Severity {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Error => match other {
                Self::Error => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Warning => match other {
                Self::Warning => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::source_result::Tracepoint {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Call(a0) => match other {
                Self::Call(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Show(a0) => match other {
                Self::Show(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Import(a0) => match other {
                Self::Import(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Include(a0) => match other {
                Self::Include(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::span::Spanned<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { v: _, span: _ } = self;
        ObservationRelation::Same
            .and(self.v.observation_relation(&other.v))
            .and(self.span.observation_relation(&other.span))
    }
}
impl ObservationEq for crate::entities::show::ShowRule {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { id: _, selector: _, transform: _ } = self;
        ObservationRelation::Same
            .and(self.id.observation_relation(&other.id))
            .and(self.selector.observation_relation(&other.selector))
            .and(self.transform.observation_relation(&other.transform))
    }
}
impl ObservationEq for crate::entities::selector::Selector {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Kind(a0) => match other {
                Self::Kind(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Location(a0) => match other {
                Self::Location(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::And(a0) => match other {
                Self::And(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Or(a0) => match other {
                Self::Or(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Regex(a0) => match other {
                Self::Regex(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Where { base: a0, field: a1, value: a2 } => match other {
                Self::Where { base: b0, field: b1, value: b2 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                }
                _ => ObservationRelation::Different,
            },
            Self::Within { base: a0, ancestor: a1 } => match other {
                Self::Within { base: b0, ancestor: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Element { function: a0, fields: a1 } => match other {
                Self::Element { function: b0, fields: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::show::Selector {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::NativeElement(a) => match other {
                Self::NativeElement(b) => a.observation_relation(b),
                _ => ObservationRelation::Different,
            },
            Self::Text(a0) => match other {
                Self::Text(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::NodeKind(a0) => match other {
                Self::NodeKind(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::DynKind(a0) => match other {
                Self::DynKind(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Regex(a0) => match other {
                Self::Regex(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Where { base: a0, field: a1, value: a2 } => match other {
                Self::Where { base: b0, field: b1, value: b2 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                }
                _ => ObservationRelation::Different,
            },
            Self::And(a0) => match other {
                Self::And(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Or(a0) => match other {
                Self::Or(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::element_kind::ElementKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Heading => match other {
                Self::Heading => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Figure => match other {
                Self::Figure => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Citation => match other {
                Self::Citation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Metadata => match other {
                Self::Metadata => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::State => match other {
                Self::State => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::StateUpdate => match other {
                Self::StateUpdate => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Outline => match other {
                Self::Outline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bibliography => match other {
                Self::Bibliography => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Equation => match other {
                Self::Equation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::CounterUpdate => match other {
                Self::CounterUpdate => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Table => match other {
                Self::Table => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::List => match other {
                Self::List => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Enum => match other {
                Self::Enum => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Par => match other {
                Self::Par => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Link => match other {
                Self::Link => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Raw => match other {
                Self::Raw => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Quote => match other {
                Self::Quote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Footnote => match other {
                Self::Footnote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::StateDisplay => match other {
                Self::StateDisplay => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::CounterDisplay => match other {
                Self::CounterDisplay => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ContextBlock => match other {
                Self::ContextBlock => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::show::Transformation {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Str(a0) => match other {
                Self::Str(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Style(a0) => match other {
                Self::Style(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::args::Args {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { items: _, named: _, span: _, occurrences: _ } = self;
        ObservationRelation::Same
            .and(self.items.observation_relation(&other.items))
            .and(self.named.observation_relation(&other.named))
            .and(self.span.observation_relation(&other.span))
            .and(self.occurrences.observation_relation(&other.occurrences))
    }
}
impl ObservationEq for crate::entities::args::ArgOccurrence {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, value: _, span: _, value_span: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.value.observation_relation(&other.value))
            .and(self.span.observation_relation(&other.span))
            .and(self.value_span.observation_relation(&other.value_span))
    }
}
impl ObservationEq for crate::entities::show::NodeKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Heading => match other {
                Self::Heading => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Figure => match other {
                Self::Figure => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Strong => match other {
                Self::Strong => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Emph => match other {
                Self::Emph => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Raw => match other {
                Self::Raw => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Equation => match other {
                Self::Equation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ListItem => match other {
                Self::ListItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Underline => match other {
                Self::Underline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Strike => match other {
                Self::Strike => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Overline => match other {
                Self::Overline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Smallcaps => match other {
                Self::Smallcaps => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Subscript => match other {
                Self::Subscript => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Superscript => match other {
                Self::Superscript => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Highlight => match other {
                Self::Highlight => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Link => match other {
                Self::Link => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Quote => match other {
                Self::Quote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Footnote => match other {
                Self::Footnote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::List => match other {
                Self::List => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Enum => match other {
                Self::Enum => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Par => match other {
                Self::Par => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::syntax_kind::SyntaxKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::End => match other {
                Self::End => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Error => match other {
                Self::Error => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Shebang => match other {
                Self::Shebang => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LineComment => match other {
                Self::LineComment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::BlockComment => match other {
                Self::BlockComment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Markup => match other {
                Self::Markup => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Text => match other {
                Self::Text => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Space => match other {
                Self::Space => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Linebreak => match other {
                Self::Linebreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parbreak => match other {
                Self::Parbreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Escape => match other {
                Self::Escape => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Shorthand => match other {
                Self::Shorthand => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SmartQuote => match other {
                Self::SmartQuote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Strong => match other {
                Self::Strong => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Emph => match other {
                Self::Emph => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Raw => match other {
                Self::Raw => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RawLang => match other {
                Self::RawLang => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RawDelim => match other {
                Self::RawDelim => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RawTrimmed => match other {
                Self::RawTrimmed => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Link => match other {
                Self::Link => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Label => match other {
                Self::Label => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Ref => match other {
                Self::Ref => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RefMarker => match other {
                Self::RefMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Heading => match other {
                Self::Heading => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::HeadingMarker => match other {
                Self::HeadingMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ListItem => match other {
                Self::ListItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ListMarker => match other {
                Self::ListMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EnumItem => match other {
                Self::EnumItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EnumMarker => match other {
                Self::EnumMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::TermItem => match other {
                Self::TermItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::TermMarker => match other {
                Self::TermMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Equation => match other {
                Self::Equation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Math => match other {
                Self::Math => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathText => match other {
                Self::MathText => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathIdent => match other {
                Self::MathIdent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathShorthand => match other {
                Self::MathShorthand => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathAlignPoint => match other {
                Self::MathAlignPoint => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathDelimited => match other {
                Self::MathDelimited => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathAttach => match other {
                Self::MathAttach => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathPrimes => match other {
                Self::MathPrimes => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathFrac => match other {
                Self::MathFrac => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathRoot => match other {
                Self::MathRoot => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hash => match other {
                Self::Hash => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LeftBrace => match other {
                Self::LeftBrace => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RightBrace => match other {
                Self::RightBrace => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LeftBracket => match other {
                Self::LeftBracket => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RightBracket => match other {
                Self::RightBracket => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LeftParen => match other {
                Self::LeftParen => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RightParen => match other {
                Self::RightParen => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Comma => match other {
                Self::Comma => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Semicolon => match other {
                Self::Semicolon => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Colon => match other {
                Self::Colon => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Star => match other {
                Self::Star => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Underscore => match other {
                Self::Underscore => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dollar => match other {
                Self::Dollar => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Plus => match other {
                Self::Plus => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Minus => match other {
                Self::Minus => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Slash => match other {
                Self::Slash => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hat => match other {
                Self::Hat => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dot => match other {
                Self::Dot => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Eq => match other {
                Self::Eq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EqEq => match other {
                Self::EqEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ExclEq => match other {
                Self::ExclEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Lt => match other {
                Self::Lt => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LtEq => match other {
                Self::LtEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Gt => match other {
                Self::Gt => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::GtEq => match other {
                Self::GtEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::PlusEq => match other {
                Self::PlusEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::HyphEq => match other {
                Self::HyphEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::StarEq => match other {
                Self::StarEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SlashEq => match other {
                Self::SlashEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dots => match other {
                Self::Dots => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Arrow => match other {
                Self::Arrow => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Root => match other {
                Self::Root => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bang => match other {
                Self::Bang => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Not => match other {
                Self::Not => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::And => match other {
                Self::And => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Or => match other {
                Self::Or => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Let => match other {
                Self::Let => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Set => match other {
                Self::Set => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Show => match other {
                Self::Show => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Context => match other {
                Self::Context => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::If => match other {
                Self::If => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Else => match other {
                Self::Else => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::For => match other {
                Self::For => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::In => match other {
                Self::In => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::While => match other {
                Self::While => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Break => match other {
                Self::Break => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Continue => match other {
                Self::Continue => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Return => match other {
                Self::Return => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Import => match other {
                Self::Import => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Include => match other {
                Self::Include => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::As => match other {
                Self::As => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Code => match other {
                Self::Code => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Ident => match other {
                Self::Ident => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool => match other {
                Self::Bool => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Int => match other {
                Self::Int => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Float => match other {
                Self::Float => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Numeric => match other {
                Self::Numeric => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Str => match other {
                Self::Str => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::CodeBlock => match other {
                Self::CodeBlock => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ContentBlock => match other {
                Self::ContentBlock => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parenthesized => match other {
                Self::Parenthesized => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Array => match other {
                Self::Array => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dict => match other {
                Self::Dict => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Named => match other {
                Self::Named => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Keyed => match other {
                Self::Keyed => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Unary => match other {
                Self::Unary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Binary => match other {
                Self::Binary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::FieldAccess => match other {
                Self::FieldAccess => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::FuncCall => match other {
                Self::FuncCall => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Args => match other {
                Self::Args => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Spread => match other {
                Self::Spread => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Closure => match other {
                Self::Closure => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Params => match other {
                Self::Params => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LetBinding => match other {
                Self::LetBinding => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SetRule => match other {
                Self::SetRule => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ShowRule => match other {
                Self::ShowRule => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Contextual => match other {
                Self::Contextual => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Conditional => match other {
                Self::Conditional => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::WhileLoop => match other {
                Self::WhileLoop => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ForLoop => match other {
                Self::ForLoop => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ModuleImport => match other {
                Self::ModuleImport => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ImportItems => match other {
                Self::ImportItems => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ImportItemPath => match other {
                Self::ImportItemPath => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RenamedImportItem => match other {
                Self::RenamedImportItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ModuleInclude => match other {
                Self::ModuleInclude => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LoopBreak => match other {
                Self::LoopBreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LoopContinue => match other {
                Self::LoopContinue => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::FuncReturn => match other {
                Self::FuncReturn => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Destructuring => match other {
                Self::Destructuring => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::DestructAssignment => match other {
                Self::DestructAssignment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::scope::Capturer {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Function => match other {
                Self::Function => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Context => match other {
                Self::Context => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::image::ImageElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { path: _, data: _, width: _, height: _, fit: _ } = self;
        ObservationRelation::Same
            .and(self.path.observation_relation(&other.path))
            .and(self.data.observation_relation(&other.data))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.fit.observation_relation(&other.fit))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::ptr_eq_arc::PtrEqArc<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl ObservationEq for crate::entities::elements::math_cases::MathCasesElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { rows: _, delim: _, reverse: _, gap: _ } = self;
        ObservationRelation::Same
            .and(self.rows.observation_relation(&other.rows))
            .and(self.delim.observation_relation(&other.delim))
            .and(self.reverse.observation_relation(&other.reverse))
            .and(self.gap.observation_relation(&other.gap))
    }
}
impl ObservationEq
    for crate::entities::elements::math_class_override::MathClassOverrideElem
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { class: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.class.observation_relation(&other.class))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::math_class::MathClass {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Normal => match other {
                Self::Normal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alphabetic => match other {
                Self::Alphabetic => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Binary => match other {
                Self::Binary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Closing => match other {
                Self::Closing => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Diacritic => match other {
                Self::Diacritic => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fence => match other {
                Self::Fence => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::GlyphPart => match other {
                Self::GlyphPart => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Large => match other {
                Self::Large => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Opening => match other {
                Self::Opening => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Punctuation => match other {
                Self::Punctuation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Relation => match other {
                Self::Relation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Space => match other {
                Self::Space => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Unary => match other {
                Self::Unary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Vary => match other {
                Self::Vary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Special => match other {
                Self::Special => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::block::BlockElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            width: _,
            height: _,
            inset: _,
            breakable: _,
            outset: _,
            radius: _,
            clip: _,
            fill: _,
            stroke: _,
            spacing: _,
            above: _,
            below: _,
            sticky: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
            .and(self.outset.observation_relation(&other.outset))
            .and(self.radius.observation_relation(&other.radius))
            .and(self.clip.observation_relation(&other.clip))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.spacing.observation_relation(&other.spacing))
            .and(self.above.observation_relation(&other.above))
            .and(self.below.observation_relation(&other.below))
            .and(self.sticky.observation_relation(&other.sticky))
    }
}
impl ObservationEq for crate::entities::elements::repeat::RepeatElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, gap: _, justify: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.justify.observation_relation(&other.justify))
    }
}
impl ObservationEq for crate::entities::elements::state_update::StateUpdateElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, update: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.update.observation_relation(&other.update))
    }
}
impl ObservationEq for crate::entities::state_update::StateUpdate {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Set(a0) => match other {
                Self::Set(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::link::LinkElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { url: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.url.observation_relation(&other.url))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::place::PlaceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            alignment: _,
            dx: _,
            dy: _,
            scope: _,
            float: _,
            clearance: _,
            body: _,
        } = self;
        ObservationRelation::Same
            .and(self.alignment.observation_relation(&other.alignment))
            .and(self.dx.observation_relation(&other.dx))
            .and(self.dy.observation_relation(&other.dy))
            .and(self.scope.observation_relation(&other.scope))
            .and(self.float.observation_relation(&other.float))
            .and(self.clearance.observation_relation(&other.clearance))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::layout_types::PlaceScope {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Column => match other {
                Self::Column => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parent => match other {
                Self::Parent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq
    for crate::entities::elements::counter_display_callback::CounterDisplayCallbackElem
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, callback: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.callback.observation_relation(&other.callback))
    }
}
impl ObservationEq for crate::entities::elements::table_header::TableHeaderElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::title::TitleElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::emph::EmphElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::quote::QuoteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, attribution: _, block: _, quotes: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.attribution.observation_relation(&other.attribution))
            .and(self.block.observation_relation(&other.block))
            .and(self.quotes.observation_relation(&other.quotes))
    }
}
impl ObservationEq for crate::entities::elements::flush::FlushElem {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        let Self = self;
        ObservationRelation::Same
    }
}
impl ObservationEq for crate::entities::elements::stack::StackElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { children: _, dir: _, spacing: _ } = self;
        ObservationRelation::Same
            .and(self.children.observation_relation(&other.children))
            .and(self.dir.observation_relation(&other.dir))
            .and(self.spacing.observation_relation(&other.spacing))
    }
}
impl ObservationEq for crate::entities::dir::Dir {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::LTR => match other {
                Self::LTR => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RTL => match other {
                Self::RTL => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::TTB => match other {
                Self::TTB => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::BTT => match other {
                Self::BTT => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::outline::OutlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { title: _, depth: _, indent: _, target: _ } = self;
        ObservationRelation::Same
            .and(self.title.observation_relation(&other.title))
            .and(self.depth.observation_relation(&other.depth))
            .and(self.indent.observation_relation(&other.indent))
            .and(self.target.observation_relation(&other.target))
    }
}
impl ObservationEq for crate::entities::elements::outline::OutlineTarget {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Headings => match other {
                Self::Headings => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Figures => match other {
                Self::Figures => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Tables => match other {
                Self::Tables => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::outline::OutlineIndent {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool(a0) => match other {
                Self::Bool(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Function(a0) => match other {
                Self::Function(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::transform::TransformElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { matrix: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.matrix.observation_relation(&other.matrix))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::layout_types::TransformMatrix {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { a: _, b: _, c: _, d: _, tx: _, ty: _ } = self;
        ObservationRelation::Same
            .and(self.a.observation_relation(&other.a))
            .and(self.b.observation_relation(&other.b))
            .and(self.c.observation_relation(&other.c))
            .and(self.d.observation_relation(&other.d))
            .and(self.tx.observation_relation(&other.tx))
            .and(self.ty.observation_relation(&other.ty))
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathUnderoverElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { base: _, under: _, over: _ } = self;
        ObservationRelation::Same
            .and(self.base.observation_relation(&other.base))
            .and(self.under.observation_relation(&other.under))
            .and(self.over.observation_relation(&other.over))
    }
}
impl ObservationEq for crate::entities::elements::list_item::ListItemElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            marker: _,
            marker_align: _,
            indent: _,
            body_indent: _,
            tight: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.marker.observation_relation(&other.marker))
            .and(self.marker_align.observation_relation(&other.marker_align))
            .and(self.indent.observation_relation(&other.indent))
            .and(self.body_indent.observation_relation(&other.body_indent))
            .and(self.tight.observation_relation(&other.tight))
    }
}
impl ObservationEq for crate::entities::list_marker::ListMarker {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Default => match other {
                Self::Default => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Custom(a0) => match other {
                Self::Custom(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Array(a0) => match other {
                Self::Array(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::boxed::BoxedElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            width: _,
            height: _,
            inset: _,
            baseline: _,
            outset: _,
            radius: _,
            clip: _,
            fill: _,
            stroke: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.baseline.observation_relation(&other.baseline))
            .and(self.outset.observation_relation(&other.outset))
            .and(self.radius.observation_relation(&other.radius))
            .and(self.clip.observation_relation(&other.clip))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
    }
}
impl ObservationEq for crate::entities::elements::cite::CiteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, supplement: _, form: _, style: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.supplement.observation_relation(&other.supplement))
            .and(self.form.observation_relation(&other.form))
            .and(self.style.observation_relation(&other.style))
    }
}
impl ObservationEq for crate::entities::citation_style::CitationStyle {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::AuthorDate => match other {
                Self::AuthorDate => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Numeric => match other {
                Self::Numeric => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alphabetic => match other {
                Self::Alphabetic => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::citation_form::CitationForm {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Normal => match other {
                Self::Normal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Prose => match other {
                Self::Prose => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Author => match other {
                Self::Author => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Year => match other {
                Self::Year => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::hide::HideElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::grid_cell::GridCellElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            stroke: _,
            fill: _,
            align: _,
            inset: _,
            breakable: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
    }
}
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            length: _,
            inverted: _,
            cross: _,
            angle: _,
            stroke: _,
            background: _,
            span: _,
            explicit: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.length.observation_relation(&other.length))
            .and(self.inverted.observation_relation(&other.inverted))
            .and(self.cross.observation_relation(&other.cross))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.background.observation_relation(&other.background))
            .and(self.span.observation_relation(&other.span))
            .and(self.explicit.observation_relation(&other.explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelExplicit {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            length: _,
            inverted: _,
            cross: _,
            angle: _,
            stroke: _,
            background: _,
        } = self;
        ObservationRelation::Same
            .and(self.length.observation_relation(&other.length))
            .and(self.inverted.observation_relation(&other.inverted))
            .and(self.cross.observation_relation(&other.cross))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.background.observation_relation(&other.background))
    }
}
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelAngle {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Angle(a0) => match other {
                Self::Angle(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::grid_footer::GridFooterElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::math_align_point::MathAlignPointElem {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        let Self = self;
        ObservationRelation::Same
    }
}
impl ObservationEq for crate::entities::elements::metadata::MetadataElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { value: _ } = self;
        ObservationRelation::Same.and(self.value.observation_relation(&other.value))
    }
}
impl ObservationEq for crate::entities::elements::terms::TermsElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { items: _ } = self;
        ObservationRelation::Same.and(self.items.observation_relation(&other.items))
    }
}
impl ObservationEq for crate::entities::elements::state_display::StateDisplayElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, callback: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.callback.observation_relation(&other.callback))
    }
}
impl ObservationEq for crate::entities::elements::state::StateElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, init: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.init.observation_relation(&other.init))
    }
}
impl ObservationEq for crate::entities::elements::pagebreak::PagebreakElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { weak: _, weak_explicit: _, to: _ } = self;
        ObservationRelation::Same
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
            .and(self.to.observation_relation(&other.to))
    }
}
impl ObservationEq for crate::entities::parity::Parity {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Even => match other {
                Self::Even => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Odd => match other {
                Self::Odd => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::figure::FigureElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, caption: _, kind: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.caption.observation_relation(&other.caption))
            .and(self.kind.observation_relation(&other.kind))
    }
}
impl ObservationEq for crate::entities::elements::curve::CurveElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { segments: _ } = self;
        ObservationRelation::Same.and(self.segments.observation_relation(&other.segments))
    }
}
impl ObservationEq for crate::entities::elements::curve::CurveSegment {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Move(a0) => match other {
                Self::Move(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Line(a0) => match other {
                Self::Line(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Cubic(a0, a1, a2) => match other {
                Self::Cubic(b0, b1, b2) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2)),
                _ => ObservationRelation::Different,
            },
            Self::Quad(a0, a1) => match other {
                Self::Quad(b0, b1) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Close(a0) => match other {
                Self::Close(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::curve::CurvePoint {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { x: _, y: _ } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
    }
}
impl ObservationEq for crate::entities::elements::curve::CloseMode {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Smooth => match other {
                Self::Smooth => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Straight => match other {
                Self::Straight => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_root::MathRootElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { index: _, radicand: _ } = self;
        ObservationRelation::Same
            .and(self.index.observation_relation(&other.index))
            .and(self.radicand.observation_relation(&other.radicand))
    }
}
impl ObservationEq for crate::entities::elements::math_accent::MathAccentElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { base: _, accent: _ } = self;
        ObservationRelation::Same
            .and(self.base.observation_relation(&other.base))
            .and(self.accent.observation_relation(&other.accent))
    }
}
impl ObservationEq for crate::entities::elements::math_attach::MathAttachElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { base: _, t: _, b: _, tl: _, bl: _, tr: _, br: _ } = self;
        ObservationRelation::Same
            .and(self.base.observation_relation(&other.base))
            .and(self.t.observation_relation(&other.t))
            .and(self.b.observation_relation(&other.b))
            .and(self.tl.observation_relation(&other.tl))
            .and(self.bl.observation_relation(&other.bl))
            .and(self.tr.observation_relation(&other.tr))
            .and(self.br.observation_relation(&other.br))
    }
}
impl ObservationEq for crate::entities::elements::math_attach::MathAttachSlot {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Omitted => match other {
                Self::Omitted => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ExplicitNone => match other {
                Self::ExplicitNone => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Present(a0) => match other {
                Self::Present(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::strike::StrikeElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, stroke: _, offset: _, extent: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.extent.observation_relation(&other.extent))
    }
}
impl ObservationEq for crate::entities::elements::divider::DividerElem {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        let Self = self;
        ObservationRelation::Same
    }
}
impl ObservationEq for crate::entities::elements::linebreak::LinebreakElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { justify: _, justify_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.justify.observation_relation(&other.justify))
            .and(self.justify_explicit.observation_relation(&other.justify_explicit))
    }
}
impl ObservationEq for crate::entities::elements::pad::PadElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, sides: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.sides.observation_relation(&other.sides))
    }
}
impl ObservationEq for crate::entities::elements::heading::HeadingElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            level: _,
            body: _,
            outlined: _,
            bookmarked: _,
            set_fields: _,
        } = self;
        ObservationRelation::Same
            .and(self.level.observation_relation(&other.level))
            .and(self.body.observation_relation(&other.body))
            .and(self.outlined.observation_relation(&other.outlined))
            .and(self.bookmarked.observation_relation(&other.bookmarked))
            .and(self.set_fields.observation_relation(&other.set_fields))
    }
}
impl ObservationEq for crate::entities::elements::counter_display::CounterDisplayElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { kind: _ } = self;
        ObservationRelation::Same.and(self.kind.observation_relation(&other.kind))
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableCellElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            stroke: _,
            fill: _,
            align: _,
            inset: _,
            breakable: _,
            kind: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
            .and(self.kind.observation_relation(&other.kind))
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableCellKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Header { level: a0, scope: a1 } => match other {
                Self::Header { level: b0, scope: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Data => match other {
                Self::Data => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableHeaderScope {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Both => match other {
                Self::Both => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Column => match other {
                Self::Column => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Row => match other {
                Self::Row => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::overline::OverlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, stroke: _, offset: _, extent: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.extent.observation_relation(&other.extent))
    }
}
impl ObservationEq for crate::entities::elements::term_item::TermItemElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { term: _, description: _ } = self;
        ObservationRelation::Same
            .and(self.term.observation_relation(&other.term))
            .and(self.description.observation_relation(&other.description))
    }
}
impl ObservationEq for crate::entities::elements::colbreak::ColbreakElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { weak: _, weak_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
    }
}
impl ObservationEq for crate::entities::document_info::DocumentInfo {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { title: _, author: _, keywords: _ } = self;
        ObservationRelation::Same
            .and(self.title.observation_relation(&other.title))
            .and(self.author.observation_relation(&other.author))
            .and(self.keywords.observation_relation(&other.keywords))
    }
}
impl ObservationEq for crate::entities::counter::Counter {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _ } = self;
        ObservationRelation::Same.and(self.key.observation_relation(&other.key))
    }
}
impl ObservationEq for crate::entities::state::State {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, init: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.init.observation_relation(&other.init))
    }
}
impl ObservationEq for crate::entities::value::Type {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool => match other {
                Self::Bool => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Int => match other {
                Self::Int => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Float => match other {
                Self::Float => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Str => match other {
                Self::Str => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Array => match other {
                Self::Array => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dictionary => match other {
                Self::Dictionary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Module => match other {
                Self::Module => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Datetime => match other {
                Self::Datetime => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Function => match other {
                Self::Function => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content => match other {
                Self::Content => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Length => match other {
                Self::Length => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Ratio => match other {
                Self::Ratio => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Relative => match other {
                Self::Relative => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Angle => match other {
                Self::Angle => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Color => match other {
                Self::Color => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Stroke => match other {
                Self::Stroke => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fraction => match other {
                Self::Fraction => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alignment => match other {
                Self::Alignment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Location => match other {
                Self::Location => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Gradient => match other {
                Self::Gradient => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Regex => match other {
                Self::Regex => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Tiling => match other {
                Self::Tiling => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bytes => match other {
                Self::Bytes => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Decimal => match other {
                Self::Decimal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Duration => match other {
                Self::Duration => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Version => match other {
                Self::Version => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Selector => match other {
                Self::Selector => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Symbol => match other {
                Self::Symbol => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Arguments => match other {
                Self::Arguments => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::State => match other {
                Self::State => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Counter => match other {
                Self::Counter => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Label => match other {
                Self::Label => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Direction => match other {
                Self::Direction => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Path => match other {
                Self::Path => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Type => match other {
                Self::Type => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::gradient::GradientStop {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { color: _, offset: _ } = self;
        ObservationRelation::Same
            .and(self.color.observation_relation(&other.color))
            .and(self.offset.observation_relation(&other.offset))
    }
}
impl ObservationEq for crate::entities::gradient::RelativeTo {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Self_ => match other {
                Self::Self_ => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parent => match other {
                Self::Parent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::gradient::Linear {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            stops: _,
            angle: _,
            space: _,
            relative: _,
            anti_alias: _,
        } = self;
        ObservationRelation::Same
            .and(self.stops.observation_relation(&other.stops))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.space.observation_relation(&other.space))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.anti_alias.observation_relation(&other.anti_alias))
    }
}
impl ObservationEq for crate::entities::gradient::Radial {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            stops: _,
            center: _,
            radius: _,
            focal_center: _,
            focal_radius: _,
            space: _,
            relative: _,
            anti_alias: _,
        } = self;
        ObservationRelation::Same
            .and(self.stops.observation_relation(&other.stops))
            .and(self.center.observation_relation(&other.center))
            .and(self.radius.observation_relation(&other.radius))
            .and(self.focal_center.observation_relation(&other.focal_center))
            .and(self.focal_radius.observation_relation(&other.focal_radius))
            .and(self.space.observation_relation(&other.space))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.anti_alias.observation_relation(&other.anti_alias))
    }
}
impl ObservationEq for crate::entities::gradient::Conic {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            stops: _,
            center: _,
            angle: _,
            space: _,
            relative: _,
            anti_alias: _,
        } = self;
        ObservationRelation::Same
            .and(self.stops.observation_relation(&other.stops))
            .and(self.center.observation_relation(&other.center))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.space.observation_relation(&other.space))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.anti_alias.observation_relation(&other.anti_alias))
    }
}
impl ObservationEq for crate::entities::gradient::Gradient {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Linear(a0) => match other {
                Self::Linear(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Radial(a0) => match other {
                Self::Radial(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Conic(a0) => match other {
                Self::Conic(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::tiling::Tiling {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            size: _,
            relative: _,
            spacing: _,
            offset: _,
            angle: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.size.observation_relation(&other.size))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.spacing.observation_relation(&other.spacing))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.angle.observation_relation(&other.angle))
    }
}
impl ObservationEq for crate::entities::tiling::TilingBody {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Image(a0) => match other {
                Self::Image(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Gradient(a0) => match other {
                Self::Gradient(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Color(a0) => match other {
                Self::Color(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::tiling::TilingRelative {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Itself => match other {
                Self::Itself => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parent => match other {
                Self::Parent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::counter_update::CounterUpdate {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Set(a0) => match other {
                Self::Set(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Step(a0) => match other {
                Self::Step(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::duration::Duration {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { nanos: _ } = self;
        ObservationRelation::Same.and(self.nanos.observation_relation(&other.nanos))
    }
}
impl ObservationEq for crate::entities::symbol::Symbol {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { value: _, name: _, variants: _, applied: _ } = self;
        ObservationRelation::Same
            .and(self.value.observation_relation(&other.value))
            .and(self.name.observation_relation(&other.name))
            .and(self.variants.observation_relation(&other.variants))
            .and(self.applied.observation_relation(&other.applied))
    }
}
impl ObservationEq for crate::entities::layout_types::Ratio {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl ObservationEq for crate::entities::color::ColorSpace {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Oklab => match other {
                Self::Oklab => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Oklch => match other {
                Self::Oklch => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Srgb => match other {
                Self::Srgb => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Luma => match other {
                Self::Luma => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LinearRgb => match other {
                Self::LinearRgb => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hsl => match other {
                Self::Hsl => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hsv => match other {
                Self::Hsv => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Cmyk => match other {
                Self::Cmyk => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::axes::Axes<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { x: _, y: _ } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
    }
}
impl ObservationEq for crate::entities::layout_types::Size {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { width: _, height: _ } = self;
        ObservationRelation::Same
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
    }
}

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
}

#[derive(Clone)]
struct ContextRead {
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

struct ContextReads {
    selected: bool,
    replaying: bool,
    styles: StyleChain,
    show_rules: Arc<[ShowRule]>,
    active_guards: Vec<RuleId>,
    current_file: Option<FileId>,
    reads: Vec<Arc<ContextRead>>,
    #[cfg(p1339_observation)]
    observation: ContextObservationTrace,
}

impl ContextReads {
    fn new() -> Self {
        Self {
            selected: false,
            replaying: false,
            styles: StyleChain::default_chain(),
            show_rules: Arc::from([]),
            active_guards: Vec::new(),
            current_file: None,
            reads: Vec::new(),
            #[cfg(p1339_observation)]
            observation: ContextObservationTrace::default(),
        }
    }
}

#[cfg(p1339_observation)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContextCallbackKind {
    StateDisplay,
    PageNumbering,
    CounterFold,
}

#[cfg(p1339_observation)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ContextObservationPhase {
    #[default]
    Body,
    Validation,
    Diagnostics,
}

#[cfg(p1339_observation)]
struct ContextReplayObservation {
    read: Arc<ContextRead>,
    // Clone retains the actual dispatch resource, including its Arc backing,
    // so subsequent formatting cannot report the requested but unused chain
    // or confuse a reused allocation address with the observed resource.
    actual_styles: StyleChain,
    result: SourceResult<Value>,
    phase: ContextObservationPhase,
    relation: Option<ObservationRelation>,
    actual_context: serde_json::Value,
}

#[cfg(p1339_observation)]
#[derive(Default)]
struct ContextObservationTrace {
    phase: ContextObservationPhase,
    callbacks: Vec<(ContextObservationPhase, ContextCallbackKind)>,
    replays: Vec<ContextReplayObservation>,
    expression_depth: usize,
    body_result: Option<SourceResult<Value>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObservationRelation {
    Same,
    Different,
    Unproven,
}

impl ObservationRelation {
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Different, _) | (_, Self::Different) => Self::Different,
            (Self::Unproven, _) | (_, Self::Unproven) => Self::Unproven,
            (Self::Same, Self::Same) => Self::Same,
        }
    }
    fn exact(same: bool) -> Self {
        if same { Self::Same } else { Self::Different }
    }
}

trait ObservationEq {
    fn observation_relation(&self, other: &Self) -> ObservationRelation;
}

impl ObservationEq for crate::entities::html::HtmlElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { tag: _, attrs: _, body: _ } = self;
        self.tag
            .observation_relation(&other.tag)
            .and(self.attrs.observation_relation(&other.attrs))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::html::HtmlBody {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Unset => match other {
                Self::Unset => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content(a) => match other {
                Self::Content(b) => a.observation_relation(b),
                _ => ObservationRelation::Different,
            },
        }
    }
}

macro_rules! observation_scalar {
    ($($ty:ty),* $(,)?) => { $(impl ObservationEq for $ty {
        fn observation_relation(&self, other: &Self) -> ObservationRelation {
            ObservationRelation::exact(self == other)
        }
    })* };
}
// Audited leaves: no hidden Value, callback or floating-point payload.
observation_scalar!(
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    str,
    String,
    EcoString,
    crate::entities::span::Span,
    crate::entities::file_id::FileId,
    std::num::NonZeroUsize,
    crate::entities::label::Label,
    crate::entities::world_types::Datetime
);
impl ObservationEq for f64 {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.to_bits().observation_relation(&other.to_bits())
    }
}
impl ObservationEq for f32 {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.to_bits().observation_relation(&other.to_bits())
    }
}
impl<T: ObservationEq + ?Sized> ObservationEq for &T {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        (**self).observation_relation(*other)
    }
}
impl<T: ObservationEq + ?Sized> ObservationEq for Box<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        (**self).observation_relation(&**other)
    }
}
impl<T: ObservationEq + ?Sized> ObservationEq for Arc<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        (**self).observation_relation(&**other)
    }
}
impl<T: ObservationEq> ObservationEq for [T] {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        if self.len() != other.len() {
            return ObservationRelation::Different;
        }
        self.iter()
            .zip(other)
            .fold(ObservationRelation::Same, |r, (a, b)| r.and(a.observation_relation(b)))
    }
}
impl<T: ObservationEq, const N: usize> ObservationEq for [T; N] {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl<T: ObservationEq> ObservationEq for Vec<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl<T: ObservationEq + Clone> ObservationEq for ecow::EcoVec<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl<T: ObservationEq> ObservationEq for Option<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match (self, other) {
            (None, None) => ObservationRelation::Same,
            (Some(a), Some(b)) => a.observation_relation(b),
            _ => ObservationRelation::Different,
        }
    }
}
impl<T: ObservationEq, E: ObservationEq> ObservationEq for Result<T, E> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match (self, other) {
            (Ok(a), Ok(b)) => a.observation_relation(b),
            (Err(a), Err(b)) => a.observation_relation(b),
            _ => ObservationRelation::Different,
        }
    }
}
impl<A: ObservationEq, B: ObservationEq> ObservationEq for (A, B) {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.0
            .observation_relation(&other.0)
            .and(self.1.observation_relation(&other.1))
    }
}
impl<K: ObservationEq, V: ObservationEq, S> ObservationEq
    for indexmap::IndexMap<K, V, S>
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        if self.len() != other.len() {
            return ObservationRelation::Different;
        }
        self.iter().zip(other.iter()).fold(
            ObservationRelation::Same,
            |r, ((ak, av), (bk, bv))| {
                r.and(ak.observation_relation(bk)).and(av.observation_relation(bv))
            },
        )
    }
}
impl ObservationEq for crate::entities::value::IntrospectedContent {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.content()
            .observation_relation(other.content())
            .and(self.fields().observation_relation(&other.fields()))
    }
}
impl ObservationEq for crate::entities::style::Styles {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.delta().observation_relation(other.delta())
    }
}
impl ObservationEq for crate::entities::font_list::FontList {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl ObservationEq for crate::entities::lang::Lang {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_str().observation_relation(other.as_str())
    }
}
impl ObservationEq for crate::entities::location::Location {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_u128().observation_relation(&other.as_u128())
    }
}
impl ObservationEq for crate::entities::path::RootedPath {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.root()
            .observation_relation(other.root())
            .and(self.vpath().observation_relation(other.vpath()))
    }
}
impl ObservationEq for crate::entities::path::VirtualPath {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.get_with_slash().observation_relation(other.get_with_slash())
    }
}
impl ObservationEq for crate::entities::layout_types::Angle {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.to_rad().observation_relation(&other.to_rad())
    }
}
impl ObservationEq for crate::entities::bytes::Bytes {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl ObservationEq for crate::entities::world_types::Bytes {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl ObservationEq for crate::entities::regex::Regex {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.pattern().observation_relation(other.pattern())
    }
}
impl ObservationEq for crate::entities::decimal::Decimal {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.0.serialize().observation_relation(&other.0.serialize())
    }
}
impl ObservationEq for crate::entities::version::Version {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { components: _ } = self;
        self.components.observation_relation(&other.components)
    }
}
impl ObservationEq for crate::entities::page_geometry::Paper {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.name()
            .observation_relation(other.name())
            .and(self.width_pt().observation_relation(&other.width_pt()))
            .and(self.height_pt().observation_relation(&other.height_pt()))
    }
}
impl ObservationEq for Scope {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        if self.len() != other.len() {
            return ObservationRelation::Different;
        }
        self.iter().zip(other.iter()).fold(
            ObservationRelation::Same,
            |r, ((ak, av), (bk, bv))| {
                r.and(ak.observation_relation(bk))
                    .and(av.value().observation_relation(bv.value()))
            },
        )
    }
}
impl ObservationEq for Module {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        // Module::eq is audited Arc identity, not structural or language equality.
        // Shared ModuleInner cannot be mutated (setters require Arc::get_mut).
        ObservationRelation::exact(self == other)
    }
}
impl ObservationEq for Func {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        use crate::entities::func::FuncRepr;
        match (&*self.0, &*other.0) {
            (FuncRepr::Closure(_), FuncRepr::Closure(_)) => {
                ObservationRelation::exact(Arc::ptr_eq(&self.0, &other.0))
            }
            (FuncRepr::Native(a), FuncRepr::Native(b)) => {
                let crate::entities::func::NativeFunc { name: _, call: _, namespace: _ } =
                    a;
                ObservationRelation::exact(std::ptr::fn_addr_eq(a.call, b.call))
                    .and(a.name.observation_relation(b.name))
                    .and(a.namespace.observation_relation(&b.namespace))
            }
            (FuncRepr::NativeWithEngine(a), FuncRepr::NativeWithEngine(b)) => {
                let crate::entities::func::NativeFuncWithEngine {
                    name: _,
                    call: _,
                    namespace: _,
                } = a;
                ObservationRelation::exact(std::ptr::fn_addr_eq(a.call, b.call))
                    .and(a.name.observation_relation(b.name))
                    .and(a.namespace.observation_relation(&b.namespace))
            }
            (FuncRepr::With(a), FuncRepr::With(b)) => a.observation_relation(b),
            (FuncRepr::Element(_), FuncRepr::Element(_))
            | (FuncRepr::Plugin(_), FuncRepr::Plugin(_)) => ObservationRelation::Unproven,
            (
                FuncRepr::Closure(_)
                | FuncRepr::Native(_)
                | FuncRepr::NativeWithEngine(_)
                | FuncRepr::With(_)
                | FuncRepr::Element(_)
                | FuncRepr::Plugin(_),
                _,
            ) => ObservationRelation::Different,
        }
    }
}
impl ObservationEq for dyn crate::entities::elements::dynamic::DynElement {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        // The extension trait does not guarantee absence of interior mutation.
        // Neither shared allocation, dyn_eq nor a Debug string proves stability.
        ObservationRelation::Unproven
    }
}

#[cfg(p1339_observation)]
use serde_json::Value as ObservationJson;

/// Raw numbered spans stay decimal strings; the L3 binding resolves them against
/// its actual retained Source, not a reconstructed source or expected location.
#[cfg(p1339_observation)]
fn p1340_observation_span(span: Span) -> ObservationJson {
    serde_json::json!({"raw_span": span.into_raw().get().to_string(),
        "file_id": span.id().map(|id| id.into_raw().get().to_string())})
}

#[cfg(p1339_observation)]
pub fn p1340_observation_diagnostics(diagnostics: &[SourceDiagnostic]) -> ObservationJson {
    use crate::entities::source_result::{Severity, Tracepoint};
    ObservationJson::Array(diagnostics.iter().map(|diagnostic| {
        let trace: Vec<_> = diagnostic.trace.iter().map(|point| {
            let (kind, payload) = match &point.v {
                Tracepoint::Call(name) => ("Call", name.clone()),
                Tracepoint::Show(name) => ("Show", Some(name.clone())),
                Tracepoint::Import(name) => ("Import", Some(name.clone())),
                Tracepoint::Include(name) => ("Include", Some(name.clone())),
            };
            serde_json::json!({"kind":kind,"payload":payload,"span":p1340_observation_span(point.span)})
        }).collect();
        serde_json::json!({"severity": match diagnostic.severity {Severity::Error=>"Error",Severity::Warning=>"Warning"},
            "span":p1340_observation_span(diagnostic.span),"message":diagnostic.message,
            "hints":diagnostic.hints,"trace":trace})
    }).collect())
}

/// The caller must retain the actual Func/producer until its transcript ends.
/// Identity reports existing allocations; it never executes or compares a Func.
#[cfg(p1339_observation)]
pub fn p1340_observation_func(func: &Func) -> ObservationJson {
    use crate::entities::func::FuncRepr;
    let (kind, capture, body) = match &*func.0 {
        FuncRepr::Closure(closure) => ("Closure", Some(format!("{:p}",Arc::as_ptr(&closure.captured))), Some(closure.body.span())),
        FuncRepr::Native(_) => ("Native", None, None),
        FuncRepr::NativeWithEngine(_) => ("NativeWithEngine", None, None),
        FuncRepr::Element(_) => ("Element", None, None),
        FuncRepr::Plugin(_) => ("Plugin", None, None),
        FuncRepr::With(_) => ("With", None, None),
    };
    let mut value = serde_json::json!({"func_id":format!("{:p}",Arc::as_ptr(&func.0)),"kind":kind,
        "capture_id":capture,"diagnostic_span_raw":func.diagnostic_span().into_raw().get().to_string(),
        "body_span_raw":body.map(|span|span.into_raw().get().to_string())});
    if let FuncRepr::With(partial) = &*func.0 {
        value["target"] = p1340_observation_func(&partial.0);
        value["arguments"] = p1340_observation_args(&partial.1);
    }
    value
}

#[cfg(p1339_observation)]
fn p1340_observation_args(args: &crate::entities::args::Args) -> ObservationJson {
    let named: Vec<_> = args.named.iter().map(|(name,value)|serde_json::json!([name.as_str(),p1340_observation_value(value)])).collect();
    let occurrences = args.occurrences.as_ref().map(|items|items.iter().map(|item|serde_json::json!({
        "name":item.name.as_ref().map(|name|name.as_str()),"value":p1340_observation_value(&item.value),
        "span":p1340_observation_span(item.span),"value_span":p1340_observation_span(item.value_span)
    })).collect::<Vec<_>>());
    serde_json::json!({"items":args.items.iter().map(p1340_observation_value).collect::<Vec<_>>(),
        "named":named,"span":p1340_observation_span(args.span),"occurrences":occurrences})
}

#[cfg(p1339_observation)]
fn p1340_observation_selector(selector: &crate::entities::selector::Selector) -> ObservationJson {
    use crate::entities::selector::Selector;
    match selector {
        Selector::Kind(kind) => serde_json::json!({"Kind":kind.as_str()}),
        Selector::Element{function,fields} => serde_json::json!({"Element":{
            "function":p1340_observation_func(function),
            "fields":fields.iter().map(|(name,value)|serde_json::json!([name.as_str(),p1340_observation_value(value)])).collect::<Vec<_>>()}}),
        Selector::Label(label) => serde_json::json!({"Label":label.0}),
        Selector::Location(location) => serde_json::json!({"Location":location.as_u128().to_string()}),
        Selector::And(items) => serde_json::json!({"And":items.iter().map(p1340_observation_selector).collect::<Vec<_>>()}),
        Selector::Or(items) => serde_json::json!({"Or":items.iter().map(p1340_observation_selector).collect::<Vec<_>>()}),
        Selector::Regex(regex) => serde_json::json!({"Regex":regex.pattern()}),
        Selector::Where{base,field,value} => serde_json::json!({"Where":{"base":p1340_observation_selector(base),"field":field.as_str(),"value":p1340_observation_value(value)}}),
        Selector::Within{base,ancestor} => serde_json::json!({"Within":{"base":p1340_observation_selector(base),"ancestor":p1340_observation_selector(ancestor)}}),
    }
}

#[cfg(p1339_observation)]
fn p1340_observation_counter_key(key: &crate::entities::counter::CounterKey) -> ObservationJson {
    use crate::entities::counter::CounterKey;
    match key {
        CounterKey::Page => serde_json::json!({"Page":null}),
        CounterKey::Str(text) => serde_json::json!({"Str":text.as_str()}),
        CounterKey::Selector(selector) => serde_json::json!({"Selector":p1340_observation_selector(selector)}),
    }
}

#[cfg(p1339_observation)]
fn p1340_observation_content(content: &Content) -> ObservationJson {
    match content {
        Content::Empty => serde_json::json!({"Empty":null}),
        Content::Text(text) => serde_json::json!({"Text":text.as_str()}),
        Content::Space => serde_json::json!({"Space":null}),
        Content::Parbreak => serde_json::json!({"Parbreak":null}),
        Content::Sequence(items) => serde_json::json!({"Sequence":items.iter().map(p1340_observation_content).collect::<Vec<_>>()}),
        Content::Metadata(metadata) => serde_json::json!({"Metadata":p1340_observation_value(&metadata.value)}),
        Content::Label(label) => serde_json::json!({"Label":{"name":label.name.as_str(),"body":p1340_observation_content(&label.body),"auto":label.auto}}),
        _ => serde_json::json!({"binding_gap":{"type":"Content","variant":content.elem_name()}}),
    }
}

/// Ordered actions already recorded in the actual snapshot. Source spans and
/// producer generations are owned by the L3 tag provenance, not this registry.
#[cfg(p1339_observation)]
pub fn p1340_observation_counter_events(registry: &crate::entities::counter_registry::CounterRegistry) -> ObservationJson {
    use crate::entities::counter_update::CounterUpdate;
    ObservationJson::Array(registry.actions().iter().map(|event| {
        let action = match &event.action {
            CounterUpdate::Set(values) => serde_json::json!({"kind":"Set","values":values}),
            CounterUpdate::Step(level) => serde_json::json!({"kind":"Step","level":level.get()}),
            CounterUpdate::Func(func) => serde_json::json!({"kind":"Func","func_id":format!("{:p}",Arc::as_ptr(&func.0))}),
        };
        serde_json::json!({"key":event.key.as_ref().map(p1340_observation_counter_key),
            "location":event.location.as_u128().to_string(),"action":action})
    }).collect())
}

/// Direct typed projection, not equality, repr, or an alternative evaluator.
/// Unintegrated variants are explicit binding gaps rather than invented data.
#[cfg(p1339_observation)]
pub fn p1340_observation_value(value: &Value) -> ObservationJson {
    match value {
        Value::None => ObservationJson::Null,
        Value::Bool(value) => serde_json::json!(value),
        Value::Int(value) => serde_json::json!(value),
        Value::Float(value) => serde_json::json!({"Float":{"bits":format!("{:016x}",value.to_bits())}}),
        Value::Str(value) => serde_json::json!(value.as_str()),
        Value::Array(items) => ObservationJson::Array(items.iter().map(p1340_observation_value).collect()),
        Value::Dict(items) => ObservationJson::Object(items.iter().map(|(key,value)|
            (key.to_string(),p1340_observation_value(value))).collect()),
        Value::Func(func) => serde_json::json!({"Func":format!("{:p}",Arc::as_ptr(&func.0))}),
        Value::Length(length) => serde_json::json!({"Length":{"abs_pt_bits":format!("{:016x}",length.abs.to_pt().to_bits()),"em_bits":format!("{:016x}",length.em.to_bits())}}),
        Value::Location(location) => serde_json::json!({"Location":location.as_u128().to_string()}),
        Value::Label(label) => serde_json::json!({"Label":label.0}),
        Value::Auto => serde_json::json!({"Auto":null}),
        Value::Selector(selector) => serde_json::json!({"Selector":p1340_observation_selector(selector)}),
        Value::Counter(counter) => serde_json::json!({"Counter":p1340_observation_counter_key(&counter.key)}),
        Value::State(state) => serde_json::json!({"State":{"key":state.key.as_str(),"init":p1340_observation_value(&state.init)}}),
        Value::Args(args) => serde_json::json!({"Args":p1340_observation_args(args)}),
        Value::Content(content) => serde_json::json!({"Content":p1340_observation_content(content)}),
        Value::LocatedContent(content,location) => serde_json::json!({"LocatedContent":{
            "location":location.as_u128().to_string(),"content":p1340_observation_content(content.content()),
            "fields":content.fields().map(|fields|fields.iter().map(|(key,value)|serde_json::json!([key.as_str(),p1340_observation_value(value)])).collect::<Vec<_>>())}}),
        _ => serde_json::json!({"binding_gap":{"type":value.type_name()}}),
    }
}

#[cfg(p1339_observation)]
fn p1340_observation_result(result: &SourceResult<Value>) -> ObservationJson {
    match result {
        Ok(value) => serde_json::json!({"kind":"Ok","value":p1340_observation_value(value)}),
        Err(diagnostics) => serde_json::json!({"kind":"Err","diagnostics":p1340_observation_diagnostics(diagnostics)}),
    }
}

#[cfg(p1339_observation)]
fn p1340_observation_context(
    location: Option<crate::entities::location::Location>, in_context: bool,
    target: EvalTarget, features: crate::entities::compiler_features::Features,
    current_file: Option<FileId>,
) -> ObservationJson {
    use crate::entities::compiler_features::Feature;
    let enabled: Vec<_> = [(Feature::Html,"html"),(Feature::A11yExtras,"a11y-extras")]
        .into_iter().filter_map(|(feature,name)|features.contains(feature).then_some(name)).collect();
    serde_json::json!({"context_location":location.map(|location|location.as_u128().to_string()),
        "in_context":in_context,"target":match target{EvalTarget::Paged=>"paged",EvalTarget::Html=>"html"},
        "features":enabled,"current_file":current_file.map(|file|file.into_raw().get().to_string())})
}

#[cfg(p1339_observation)]
fn p1340_observation_request(request: &ContextReadRequest) -> (&'static str, ObservationJson) {
    use ContextReadRequest::*;
    match request {
        CounterFold{key} => ("CounterFold",serde_json::json!({"key":p1340_observation_counter_key(key)})),
        CounterAt{key,location} => ("CounterAt",serde_json::json!({"key":p1340_observation_counter_key(key),"location":location.as_u128().to_string()})),
        CounterLabelAt{key,label} => ("CounterLabelAt",serde_json::json!({"key":p1340_observation_counter_key(key),"label":label.0})),
        CounterFinal{key} => ("CounterFinal",serde_json::json!({"key":p1340_observation_counter_key(key)})),
        CounterTotal{key} => ("CounterTotal",serde_json::json!({"key":p1340_observation_counter_key(key)})),
        CounterResolve{value} => ("CounterResolve",serde_json::json!({"value":p1340_observation_value(value)})),
        CounterLegacyAt{key,label} => ("CounterLegacyAt",serde_json::json!({"key":key,"label":label.0})),
        CounterLegacyFinal{key} => ("CounterLegacyFinal",serde_json::json!({"key":key})),
        StateGet{state} => ("StateGet",serde_json::json!({"key":state.key.as_str(),"init":p1340_observation_value(&state.init)})),
        StateDisplay{state} => ("StateDisplay",serde_json::json!({"key":state.key.as_str(),"init":p1340_observation_value(&state.init)})),
        StateAt{state,location} => ("StateAt",serde_json::json!({"key":state.key.as_str(),"init":p1340_observation_value(&state.init),"location":location.as_u128().to_string()})),
        StateFinal{state} => ("StateFinal",serde_json::json!({"key":state.key.as_str(),"init":p1340_observation_value(&state.init)})),
        StateResolveLabel{label} => ("StateResolveLabel",serde_json::json!({"label":label.0})),
        StateLegacyAt{key,label} => ("StateLegacyAt",serde_json::json!({"key":key,"label":label.0})),
        StateLegacyFinal{key} => ("StateLegacyFinal",serde_json::json!({"key":key})),
        Query{selector} => ("Query",serde_json::json!({"selector":p1340_observation_selector(selector)})),
        Locate{selector} => ("Locate",serde_json::json!({"selector":p1340_observation_selector(selector)})),
        Here => ("Here",serde_json::json!({})),
        LocationPage{location} => ("LocationPage",serde_json::json!({"location":location.as_u128().to_string()})),
        LocationPosition{location} => ("LocationPosition",serde_json::json!({"location":location.as_u128().to_string()})),
        LocationPageNumbering{location} => ("LocationPageNumbering",serde_json::json!({"location":location.as_u128().to_string()})),
    }
}

#[cfg(p1339_observation)]
fn p1340_observation_row(read: &Arc<ContextRead>, styles: &StyleChain, context: ObservationJson, result: &SourceResult<Value>) -> ObservationJson {
    let (operation, arguments) = p1340_observation_request(&read.request);
    let mut row = context;
    let fields = row.as_object_mut().expect("actual context projection is an object");
    fields.insert("request_id".into(),serde_json::json!(format!("{:p}",Arc::as_ptr(read))));
    fields.insert("operation".into(),serde_json::json!(operation));
    fields.insert("typed_arguments".into(),arguments);
    fields.insert("span".into(),p1340_observation_span(read.span));
    fields.insert("chain_id".into(),serde_json::json!(match styles.p1339_observation_identity(){Some(identity)=>format!("{identity:#x}"),None=>"empty".into()}));
    fields.insert("chain_size_pt_bits".into(),serde_json::json!(format!("{:016x}",styles.size().to_bits())));
    fields.insert("result".into(),p1340_observation_result(result));
    row
}

#[cfg(p1339_observation)]
impl EvalContext {
    pub fn p1340_observation_requests(&self) -> ObservationJson {
        ObservationJson::Array(self.context_reads.borrow().reads.iter().map(|read| {
            p1340_observation_row(read,&read.styles,p1340_observation_context(
                read.location,read.in_context,read.target,read.features,read.current_file),&read.result)
        }).collect())
    }

    /// Append-only replay transcript. Callers select actual phase and the suffix
    /// since their boundary; this port never drops an early fail-fast witness.
    pub fn p1340_observation_replays(&self) -> ObservationJson {
        ObservationJson::Array(self.context_reads.borrow().observation.replays.iter().map(|replay| {
            let mut row=p1340_observation_row(&replay.read,&replay.actual_styles,replay.actual_context.clone(),&replay.result);
            row["phase"]=serde_json::json!(match replay.phase{ContextObservationPhase::Body=>"body",ContextObservationPhase::Validation=>"validation",ContextObservationPhase::Diagnostics=>"diagnostics"});
            row["relation"]=serde_json::json!(replay.relation.map(|relation|match relation{ObservationRelation::Same=>"Same",ObservationRelation::Different=>"Different",ObservationRelation::Unproven=>"Unproven"}));
            row
        }).collect())
    }
}

impl EvalContext {
    #[cfg(p1339_observation)]
    pub(crate) fn p1339_observe_callback(&self, kind: ContextCallbackKind) {
        let mut log = self.context_reads.borrow_mut();
        let phase = log.observation.phase;
        log.observation.callbacks.push((phase, kind));
    }

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
                Selector::Element { .. } => true,
                Selector::And(items) | Selector::Or(items) => {
                    items.iter().any(contains_element)
                }
                Selector::Where { base, .. } => contains_element(base),
                Selector::Within { base, ancestor } => {
                    contains_element(base) || contains_element(ancestor)
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
        #[cfg(p1339_observation)]
        {
            ctx.context_reads.borrow_mut().observation.phase =
                self.context_reads.borrow().observation.phase;
        }
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
        // Passive snapshot of the resource actually supplied to the read owner.
        // Deliberately not read.styles: substituting transaction.styles must
        // change the telemetry even when the request's result stays unchanged.
        #[cfg(p1339_observation)]
        let actual_dispatch_styles = transaction.styles.clone();
        #[cfg(p1339_observation)]
        let actual_dispatch_context = p1340_observation_context(
            ctx.current_location, ctx.in_context, ctx.target, ctx.features,
            Some(transaction.current_file),
        );
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
        };
        #[cfg(p1339_observation)]
        {
            let mut log = self.context_reads.borrow_mut();
            log.observation
                .callbacks
                .extend(ctx.context_reads.borrow().observation.callbacks.iter().copied());
            log.observation.replays.push(ContextReplayObservation {
                read: read.clone(),
                actual_styles: actual_dispatch_styles,
                result: result.clone(),
                phase: ctx.context_reads.borrow().observation.phase,
                relation: None,
                actual_context: actual_dispatch_context,
            });
        }
        result
    }

    pub fn context_reads_valid_for(
        &self,
        candidate: &crate::entities::introspector::TagIntrospector,
        engine: &mut Engine<'_>,
    ) -> SourceResult<bool> {
        #[cfg(p1339_observation)]
        let previous_phase = std::mem::replace(
            &mut self.context_reads.borrow_mut().observation.phase,
            ContextObservationPhase::Validation,
        );
        let reads = self.context_reads.borrow().reads.clone();
        let mut valid = true;
        for read in &reads {
            let result = self.replay_context_read(read, candidate, engine);
            if {
                #[cfg(not(p1339_observation))]
                { read.result.observation_relation(&result) != ObservationRelation::Same }
                #[cfg(p1339_observation)]
                {
                    let relation = read.result.observation_relation(&result);
                    let mut log = self.context_reads.borrow_mut();
                    let observed = log.observation.replays.last_mut()
                        .expect("actual replay must precede validation comparison");
                    assert!(Arc::ptr_eq(&observed.read, read));
                    observed.relation = Some(relation);
                    relation != ObservationRelation::Same
                }
            } {
                valid = false;
                break;
            }
        }
        #[cfg(p1339_observation)]
        {
            self.context_reads.borrow_mut().observation.phase = previous_phase;
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
        #[cfg(p1339_observation)]
        let previous_phase = std::mem::replace(
            &mut self.context_reads.borrow_mut().observation.phase,
            ContextObservationPhase::Diagnostics,
        );
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
                | ContextReadRequest::LocationPageNumbering { .. } => continue,
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
        #[cfg(p1339_observation)]
        {
            self.context_reads.borrow_mut().observation.phase = previous_phase;
        }
        Ok(diagnostics)
    }
}

/// Contexto de execução partilhado durante eval().
///
/// Limite de segurança para prevenir loops infinitos:
/// - `max_loop_iterations`: limite total global de iterações. Um contador
///   local por loop permite "loop-bombing" (milhares de loops pequenos que
///   colectivamente travam o motor). Counter global impede isso: 1.000.000
///   iterações falham em segundos independentemente da distribuição.
///
/// A profundidade de chamadas **não** é verificada aqui — é verificada pelo
/// `Route<'a>` através de `route.check_call_depth()` em `apply_closure`
/// (Passo 93, ADR-0033 paridade com vanilla, `MAX_CALL_DEPTH = 80`). O
/// campo antigo `depth`/`max_call_depth`/`enter_call`/`leave_call` foi
/// removido (DEBT-45 parcialmente pago).
///
/// A rota de compilação (`Route<'a>`) **não** é campo do contexto — é
/// passada como parâmetro `route: Tracked<'r, Route<'r>>` às funções
/// `eval_*` que participam na recursão. Paridade estrutural com o vanilla
/// e primeira aplicação concreta da ADR-0036 (atomização progressiva,
/// Passo 92). O campo `route: Vec<FileId>` + API `with_route_id` do Passo
/// 90 foram eliminados no Passo 92 (DEBT-44 fechado).
///
/// A cadeia de estilos (`StyleChain`) **também não** é campo do contexto
/// desde o Passo 94 — propaga-se como `&mut StyleChain` nas funções
/// `eval_*`. Cada bloco de scoping (`CodeBlock`, `ContentBlock`,
/// `Strong`/`Emph`/`Heading`, corpo de closure) cria uma cópia local
/// (`let mut local_styles = styles.clone()` ou `styles.push(delta)`),
/// eliminando o antigo par save/restore sobre um campo partilhado.
/// Segunda aplicação concreta da ADR-0036.
pub struct EvalContext {
    context_reads: RefCell<ContextReads>,
    pub target: EvalTarget,
    pub features: crate::entities::compiler_features::Features,
    // ADR-0036 Regra 4: contador monotónico global — limite de segurança
    // anti-loop-bombing, independente do fluxo de controlo.
    pub loop_iterations: usize,
    // ADR-0036 Regra 4: limite estático — configuração da execução, sem
    // semântica de fluxo.
    pub max_loop_iterations: usize,
    // ADR-0036 Regra 4: alocador monotónico para IDs de ShowRule (Passo 70)
    // — gera valores únicos durante a sessão de eval, não depende de fluxo.
    //
    // Ficaram fora do contexto em passos anteriores (todos como parâmetros
    // explícitos das funções `eval_*`, agora agregados em `Engine<'a>` no
    // Passo 109, ADR-0044):
    // - `world` (Passo 109) — `&'a dyn World` em `Engine`.
    // - `route` (Passo 92) — `Tracked<'r, Route<'r>>`.
    // - `styles` (Passo 94) — `&mut StyleChain`.
    // - `show_rules` + `active_guards` (Passo 95).
    // - `current_file` + `figure_numbering` (Passo 98).
    // - `sink` (Passo 107).
    pub next_rule_id: crate::entities::show::RuleId,

    /// Snapshot do `TagIntrospector` da iteração de fixpoint anterior
    /// (P174 / M7 sub-passo 1). Default: `TagIntrospector::empty()` —
    /// primeira iteração não vê resultados anteriores. `run_fixpoint`
    /// actualiza este field entre iterações; features stdlib P175+
    /// (`query`, `here`, `counter.at`) leem daqui.
    ///
    /// **Read-only no eval**: stdlib lê, nunca escreve. Mutação
    /// exclusiva por `run_fixpoint`.
    pub introspector: crate::entities::introspector::TagIntrospector,

    /// **P208B (M9c)** — `Location` "actual" disponível durante eval,
    /// para suportar stdlib `here()` (P208B) e `locate()` (P208C).
    ///
    /// **Infra minimal**: `Default = None`. Cristalino single-pass +
    /// fixpoint não avança `current_location` automaticamente no eval
    /// walk (divergência arquitectónica vs vanilla `Tracked<Context>`
    /// per P205A.div-1). Caller que conhece a Location actual (ex.:
    /// futuro show-rule para `Content::Context` block análogo a
    /// vanilla `ContextElem`, ou tests sintéticos) escreve este field
    /// directamente via `with_current_location` antes de invocar
    /// eval/stdlib. `here()` lê este field e devolve
    /// `Value::Location(loc)`; se `None`, erro contextual coerente.
    ///
    /// Mecanismo de captura no eval walk (sub-mecanismo i avançado)
    /// fica deferred — emerge naturalmente quando `Content::Context`
    /// block for materializado (sub-passo dedicado pós-P208).
    pub current_location: Option<crate::entities::location::Location>,

    /// **P350c — flag de "erro completo" (capacidade interna)**. Quando ligada,
    /// o erro de recursão de `#show` (teto, `apply_show_rules`) ganha um **3º hint**
    /// classificando **cíclico** (uma morfologia do caminho repetiu — fato medido
    /// pelo `==` do P345) ou **não-convergente** (teto sem repetição). A **mensagem
    /// base** e os 2 hints do vanilla **não mudam**; o 3º hint só aparece com a flag.
    /// **Caminho quente intacto**: o histórico de morfologias só é alocado quando
    /// esta flag está ligada (atrás do `if`). **L1 não lê env** — recebe o booleano
    /// já resolvido (via `eval_with_full_error`; origem em `RunIntent`, fio L4→L1
    /// interno + parsing CLI = débito P350c). Default `false` (= comportamento
    /// byte-idêntico ao vanilla).
    pub full_error: bool,

    /// **P498 — separação entre conteúdo original (para introspecção) e output
    /// de show-rules (para layout/render)**. Quando `false`, `intercept_content`
    /// não aplica show-rules, produzindo a árvore original de elementos locatable.
    /// O entrypoint `eval` corre duas passagens: uma com `false` (captura original)
    /// e outra com `true` (render real). Default `true`.
    pub apply_show_rules: bool,

    /// **P506 — indica que o eval está a correr dentro da expansão de um
    /// `context { ... }`. Quando `true`, métodos `.get()` e `.display()` de
    /// `state`/`counter` podem consultar o introspector e a localização actual.
    pub in_context: bool,

    /// **P506 — contador monotónico para IDs de `ContextBlock`. Garante
    /// identificadores estáveis entre a criação em eval e a expansão
    /// pós-introspecção.
    pub next_context_id: u64,

    /// **P429 (DEBT-63)** — styles CSL resolvidos em eval time, indexados pela
    /// chave determinística do `BibliographyElem` correspondente. Transporta-se
    /// para o `Module` no fim do eval e depois para o `BibStore` do
    /// `TagIntrospector` no pipeline (L3), evitando que o elemento guarde cache
    /// de estado computado.
    pub bibliography_styles: HashMap<u64, Arc<IndependentStyle>>,

    /// **P536** — metadados do documento definidos por `#set document(...)`.
    /// Transporta-se para o `Module` no fim do eval e depois para o
    /// exportador PDF (`/Info`).
    pub document_info: DocumentInfo,

    /// **P635 — evento de controlo de fluxo activo**. Equivalente a `vm.flow`
    /// do vanilla (`typst-eval/src/vm.rs:20`). Propagado de `eval_expr` para
    /// ciclos, funções e o entrypoint.
    pub flow: Option<FlowEvent>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            context_reads: RefCell::new(ContextReads::new()),
            target: EvalTarget::Paged,
            features: crate::entities::compiler_features::Features::default(),
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
            next_rule_id: 0,
            introspector: crate::entities::introspector::TagIntrospector::empty(),
            current_location: None,
            full_error: false,
            apply_show_rules: true,
            in_context: false,
            next_context_id: 0,
            bibliography_styles: HashMap::new(),
            document_info: DocumentInfo::empty(),
            flow: None,
        }
    }

    /// **P506** — gera um ID estável para `ContextBlockElem`.
    pub fn next_context_id(&mut self) -> u64 {
        let id = self.next_context_id;
        self.next_context_id += 1;
        id
    }

    /// **P429 (DEBT-63)** — regista o style CSL resolvido para o
    /// `BibliographyElem` indicado, usando a sua chave determinística.
    pub fn register_bibliography_style(
        &mut self,
        elem: &BibliographyElem,
        style: Arc<IndependentStyle>,
    ) {
        self.bibliography_styles.insert(elem.style_key(), style);
    }

    /// **P208B (M9c)** — Setter conveniente para `current_location`.
    /// Usado em tests sintéticos e por consumers futuros que conhecem
    /// a Location actual antes de invocar eval/stdlib `here()`.
    pub fn with_current_location(
        mut self,
        location: crate::entities::location::Location,
    ) -> Self {
        self.current_location = Some(location);
        self
    }

    /// Incrementa o contador de iterações e retorna Err se o limite foi atingido.
    pub fn tick_loop(&mut self, span: Span) -> SourceResult<()> {
        self.loop_iterations += 1;
        if self.loop_iterations > self.max_loop_iterations {
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "limite de iterações de loop atingido ({}) — \
                     possível loop infinito",
                    self.max_loop_iterations
                ),
            )])
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalTarget {
    Paged,
    Html,
}

/// Avalia um ficheiro Typst e retorna o módulo resultante (flag de erro completo
/// **desligada** — comportamento byte-idêntico ao vanilla). **Delegado** de
/// `eval_with_full_error` (P350c): mantém a assinatura estável para os ~165 callers
/// (produção L3 + testes) — a capacidade da flag entra pela sibling, não por aqui.
pub fn eval(
    routines: &Routines,
    world: &dyn World,
    traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    route: Tracked<Route>,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
) -> SourceResult<Module> {
    eval_with_full_error(routines, world, traced, sink, route, source, registry, false)
}

/// Avalia uma expressão Typst isolada em modo code e scope fresco.
pub fn eval_expression(
    world: &dyn World,
    expression: &str,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    eval_expression_with_features(
        world,
        expression,
        crate::entities::compiler_features::Features::default(),
    )
}

/// Disponibiliza a Source já analisada, sem modificar o World chamador.
struct ExpressionWorld<'a> {
    inner: &'a dyn World,
    source: &'a Source,
}

impl World for ExpressionWorld<'_> {
    fn library(&self) -> &Library {
        self.inner.library()
    }
    fn book(&self) -> &FontBook {
        self.inner.book()
    }
    fn main(&self) -> FileId {
        self.inner.main()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            self.inner.source(id)
        }
    }
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.inner.file(id)
    }
    fn resolve_path(&self, file: FileId, path: &str) -> Result<RootedPath, String> {
        self.inner.resolve_path(file, path)
    }
    fn read_path(&self, path: &RootedPath) -> Result<Arc<Vec<u8>>, String> {
        self.inner.read_path(path)
    }
    fn include_path(&self, path: &RootedPath) -> Result<Source, String> {
        self.inner.include_path(path)
    }
    fn read_bytes(&self, file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String> {
        self.inner.read_bytes(file, path)
    }
    fn include_source(&self, file: FileId, path: &str) -> Result<Source, String> {
        self.inner.include_source(file, path)
    }
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String> {
        self.inner.resolve_package(spec)
    }
    fn inputs(&self) -> SysInputs {
        self.inner.inputs()
    }
    fn plugin_host(&self) -> Option<Arc<dyn crate::contracts::plugin_host::PluginHost>> {
        self.inner.plugin_host()
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.inner.font(index)
    }
    fn candidates_for_char(&self, c: char) -> Vec<usize> {
        self.inner.candidates_for_char(c)
    }
    fn today(
        &self,
        offset: Option<crate::entities::duration::Duration>,
    ) -> Option<Datetime> {
        self.inner.today(offset)
    }
}

pub fn eval_expression_with_features(
    world: &dyn World,
    expression: &str,
    features: crate::entities::compiler_features::Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>) {
    let inputs = world.inputs();
    let mut global = Scope::new();
    let stdlib = make_stdlib_with_features(&inputs, features);
    global.define("std", Value::Module(Module::new("global", stdlib.clone())));
    for (name, binding) in stdlib.iter() {
        global.define(name, binding.value().clone());
    }
    for (name, value) in crate::compiler::stdlib::predefined_color_bindings() {
        global.define(name.as_str(), value);
    }
    global.define(
        "text",
        Value::Func(Func::native("text", crate::compiler::stdlib::native_text)),
    );
    let library = Library::with_global(global);
    let mut scopes = Scopes::new(Some(&library));
    let mut ctx = EvalContext::new();
    ctx.features = features;
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let current_file = world.main();
    let source = Source::new_with_parser(
        current_file,
        expression.to_string(),
        crate::compiler::parse::parse_code,
    );
    let expression_world = ExpressionWorld { inner: world, source: &source };
    let route = Route::root().with_id(current_file);
    let fixed_metrics = FixedMetrics;
    let mut sink = Sink::new();
    let result = {
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &expression_world,
            font_metrics: &fixed_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file,
            sink: &mut tracked_sink,
        };
        if source.root().erroneous() {
            Err(source
                .root()
                .errors()
                .into_iter()
                .map(|error| {
                    let mut diagnostic =
                        SourceDiagnostic::error(error.span, error.message.to_string());
                    diagnostic.hints =
                        error.hints.iter().map(|hint| hint.to_string()).collect();
                    diagnostic
                })
                .collect())
        } else {
            (|| -> SourceResult<Value> {
                let mut last = Value::None;
                for child in source.root().children() {
                    if let Some(expr) = Expr::from_untyped(child) {
                        last = eval_expr(expr, &mut scopes, &mut ctx, &mut engine)?;
                    }
                }
                Ok(last)
            })()
        }
    };
    (result, sink.into_diagnostics())
}

/// Como [`eval`], mas com a flag de **erro completo** (P350c) explícita. Quando
/// `full_error` está ligada, o erro de recursão de `#show` ganha um 3º hint
/// classificando cíclico/não-convergente (ver [`EvalContext::full_error`]). A
/// origem do booleano é `RunIntent` (L2), via o caminho **interno** de L3 — a
/// assinatura **pública** de L3 (`compile_to_pdf_bytes`) **não** muda; o parsing
/// da CLI + o fio `RunIntent`→L3-interno são **débito** (P350c). L1 recebe o
/// booleano já resolvido (não lê env).
///
/// Travessia AST parcial: literais, Ident, Let, CodeBlock, Binary, Unary,
/// Conditional, WhileLoop, ForLoop, Closure, FuncCall. **Invariante**: não importa
/// nada de `03_infra`; acesso ao world sempre via `World` (L1).
pub fn eval_with_full_error(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    // Lote F-3 inc-2: o threading registry→escopo que o F-1 deferiu. Elementos
    // de utilizador registados entram no escopo como funções (`#name(args)`).
    // Em produção é vazio até pacotes registarem elementos; testes injetam.
    registry: &crate::entities::element_registry::ElementRegistry,
    // P350c: flag de erro completo, já resolvida (origem `RunIntent`; L1 não lê env).
    full_error: bool,
) -> SourceResult<Module> {
    eval_with_full_error_and_target(
        _routines,
        world,
        _traced,
        sink,
        _route,
        source,
        registry,
        full_error,
        EvalTarget::Paged,
    )
}

pub fn eval_with_full_error_and_target(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
    full_error: bool,
    target: EvalTarget,
) -> SourceResult<Module> {
    eval_with_full_error_target_and_features(
        _routines,
        world,
        _traced,
        sink,
        _route,
        source,
        registry,
        full_error,
        target,
        crate::entities::compiler_features::Features::default(),
    )
}

pub fn eval_with_full_error_target_and_features(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    mut sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
    full_error: bool,
    target: EvalTarget,
    features: crate::entities::compiler_features::Features,
) -> SourceResult<Module> {
    let root = source.root();

    // **P786a** — propagação integral de erros de sintaxe (revoga a
    // filtragem selectiva de P648/P649, que só propagava
    // `InvalidHexNumber`/`InvalidUnicodeCodepoint`). O survey empírico
    // deste passo (20 construções válidas, teste
    // `p786a_valid_constructs_without_error_nodes`) mostrou zero falsos
    // positivos no parser actual — `#` em código é erro genuíno (o
    // vanilla 0.15.0 também o rejeita, exit 1 + 2 hints). Warnings de
    // markup (`NoTextWithin*`, produzidos em `parse/markup.rs`) seguem
    // para o sink como warning e **não abortam** (T1 — paridade vanilla:
    // `warning: no text within stars` + hint, exit 0). Hints dos
    // `SyntaxError` são propagados em ambos os caminhos.
    let mut syntax_errors: Vec<SourceDiagnostic> = Vec::new();
    for e in root.errors() {
        let is_warning = matches!(
            e.kind,
            SyntaxErrorKind::NoTextWithinStars | SyntaxErrorKind::NoTextWithinUnderscores
        );
        if is_warning {
            sink.warn_note(
                e.span,
                e.message.as_str(),
                e.hints.first().map_or("", |h| h.as_str()),
            );
        } else {
            let mut diag = SourceDiagnostic::error(e.span, e.message.to_string());
            for h in &e.hints {
                diag = diag.with_hint(h.as_str());
            }
            syntax_errors.push(diag);
        }
    }
    if !syntax_errors.is_empty() {
        return Err(syntax_errors);
    }

    // Passo 106 (ADR-0043): canal de warnings activo. Pilot: emitir nota
    // quando o ficheiro fonte está vazio. Prova de vida do canal — o
    // caller lê `sink.into_diagnostics()` após este retorno.
    if source.text().is_empty() {
        sink.warn_note(
            crate::entities::span::Span::detached(),
            "ficheiro vazio: sem conteúdo",
            "",
        );
    }

    // P498 — passagem dupla do eval:
    // 1. `apply_show_rules = false`: produz o conteúdo original (pré-show-rules)
    //    para alimentar a introspecção. Espelha o modelo vanilla, onde o
    //    Introspector vê os elementos antes da realização das show-rules.
    // 2. `apply_show_rules = true`: produz o output renderizado para layout/PDF.
    //
    // O scope, as definições de stdlib e as show-rules registadas são idênticos
    // nas duas passagens; só a aplicação das show-rules difere.
    //
    // P694 — `sys.inputs` vem do ambiente via `World::inputs()` (default vazio);
    // lido uma vez e partilhado pelas duas passagens.
    let inputs = world.inputs();
    let mut run_pass = |apply_show_rules: bool,
                        pass_sink: &mut TrackedMut<Sink>|
     -> SourceResult<(
        Value,
        Scope,
        HashMap<u64, Arc<IndependentStyle>>,
        DocumentInfo,
    )> {
        let mut ctx = EvalContext::new();
        ctx.target = target;
        ctx.features = features;
        ctx.full_error = full_error; // P350c: flag resolvida (default false via `eval`)
        ctx.apply_show_rules = apply_show_rules; // P498

        // Route raiz com o FileId do ficheiro principal — primeira aplicação da
        // ADR-0036 (Passo 92), agora campo do Engine (ADR-0044, Passo 109).
        let route = Route::root().with_id(source.id());
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards: Vec<RuleId> = Vec::new();
        let current_file = source.id();

        // P772n — stdlib/cores/`std`/`text`/elementos de utilizador deixam de
        // ser achatados em `scopes.top` (o que os tornava mutáveis sem erro,
        // P772l §2.3). Constrói-se um `Scope` próprio, embrulhado em
        // `Library::with_global`, e passado como `base` — só alcançável por
        // leitura (`Scopes::get`), nunca por `Scopes::get_mut`. Ver
        // `rules/eval.md` §P772n.
        let mut global = Scope::new();
        let stdlib = make_stdlib_with_features(&inputs, features);
        // P709 — `std`: clone independente da stdlib, tirado ANTES de ser
        // espalhada em `global`, dá acesso à versão não-sombreada mesmo que
        // o documento redefina `length`/`calc`/etc. (paridade vanilla,
        // `Library::std = Binding::detached(global.clone())`). Sombreável
        // como qualquer outro nome (medido: `#let std = "oops"` funciona
        // no vanilla) — por isso um binding normal, sem mecanismo especial.
        global.define("std", Value::Module(Module::new("global", stdlib.clone())));
        for (name, binding) in stdlib.iter() {
            global.define(name, binding.value().clone());
        }
        // P492 — cores predefinidas (red, blue, green, ...) como atalhos globais.
        for (name, value) in crate::compiler::stdlib::predefined_color_bindings() {
            global.define(name.as_str(), value);
        }
        // P492 — constructor `text(...)` no scope global (usado em show-rules, etc.).
        global.define(
            "text",
            Value::Func(crate::entities::func::Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            )),
        );
        // Lote F-3 inc-2: elementos de utilizador registados entram no escopo como
        // funções (`#name(args)` → `Content::Dynamic` via o construtor do registry).
        // Mesmo escopo base que os nativos (document-wide); `#set`/`#show` léxicos
        // por cima seguem o padrão `local_styles` (F-2).
        for name in registry.names() {
            if let Some(ctor) = registry.ctor(name) {
                global.define(
                    name.as_str(),
                    Value::Func(Func::element(name.as_str(), ctor)),
                );
            }
        }
        let library = Library::with_global(global);
        let mut scopes = Scopes::new(Some(&library));

        // ADR-0044 (Passo 109): agregar os 8 campos num `Engine<'_>` e passar
        // `&mut engine` às funções internas em vez de 8 parâmetros individuais.
        // Reborrow do `sink` encurta o lifetime inner do `TrackedMut` ao da
        // stack frame local, permitindo que `Engine<'a>` tenha um único `'a`.
        let fixed_metrics = FixedMetrics;
        let mut local_sink = TrackedMut::reborrow_mut(&mut *pass_sink);
        let mut engine = Engine {
            world,
            font_metrics: &fixed_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file,
            sink: &mut local_sink,
        };

        let content_val = eval_markup(root, &mut scopes, &mut ctx, &mut engine)?;
        if let Some(flow) = ctx.flow {
            return Err(vec![flow.forbidden()]);
        }
        let module_scope = scopes.exit();
        Ok((content_val, module_scope, ctx.bibliography_styles, ctx.document_info))
    };

    // Passo 1: captura do conteúdo original (pré-show-rules).
    // Usa o mesmo sink principal para que warnings emitidos antes de um erro
    // cheguem ao caller (dedup por (span, message) previne duplicação com passo 2).
    let (original_val, _, _, _) = run_pass(false, &mut sink)?;
    let original_content = match original_val {
        Value::Content(c) => Some(c),
        _ => None,
    };

    // Passo 2: eval normal (com show-rules) — este é o resultado oficial.
    let (rendered_val, module_scope, bibliography_styles, document_info) =
        run_pass(true, &mut sink)?;
    let rendered_content = match rendered_val {
        // **P537b** — ligar `#set page(columns: N)` ao consumer `Content::Columns`.
        Value::Content(c) => Some(c.wrap_page_columns()),
        _ => None,
    };

    let mut module = Module::new(source.id().into_raw().get().to_string(), module_scope);
    module.set_content(rendered_content);
    module.set_introspection_content(original_content);
    // P429 (DEBT-63): transportar styles resolvidos do eval para o Module,
    // de onde o pipeline os injectará no BibStore do TagIntrospector.
    module.set_bibliography_styles(bibliography_styles);
    // P536: transportar metadados do documento para o Module.
    module.set_document_info(document_info);
    Ok(module)
}

pub(crate) fn eval_markup(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut parts: Vec<Content> = Vec::new();
    // Passo 445: smart quotes context-aware em markup.
    // O lexer marca `"` e `'` como `SyntaxKind::SmartQuote`; o eval decide
    // open/close/apostrophe com base nos caracteres adjacentes e no `text.lang`.
    // Aspas duplas e simples são resolvidas independentemente.
    let src = node.clone().into_text();
    let src_str = src.as_str();
    let mut byte_offset = 0_usize;

    fn is_opening_context(c: Option<char>) -> bool {
        c.is_none()
            || c.unwrap().is_whitespace()
            || matches!(c.unwrap(), '(' | '[' | '{' | '<')
    }
    fn is_word_char(c: Option<char>) -> bool {
        c.is_some_and(|c| c.is_alphanumeric())
    }

    // β1 fatia 1 (P339, §3a.8) + F-item3 (P368, §3a.11): snapshot do **delta
    // completo** à entrada deste corpo, para detectar um `#set` local (numbering,
    // prop de elemento de usuário, ou estilo tipado como `text.bold`) e embrulhar
    // o resto do escopo léxico num `Content::Styled` (transporte aditivo
    // `StyledElem`-scoped). Generaliza o antigo snapshot só-de-custom.
    // P373 (§3a.14): correção do transporte — **wraps aninhados por escopo léxico**
    // (espelho do `styled_with_map` por-`#set` do vanilla, recursivo). Rastreia cada
    // **fronteira de `#set`** (mudança do delta vs o estado corrente) com o delta
    // QUE ESTE `#set` introduziu; no fim, fold de dentro para fora produz o
    // aninhamento. Conserta o bug single-wrap-final-collapse (P372): `#set`
    // sequenciais da mesma chave deixam de colapsar para o valor final.
    // P431 (DEBT-50): o delta agora inclui origem (`from_strong`/`from_emph`),
    // propagada pelo `StyleDelta` e embrulhada via `diff_styles`.
    let snap_delta = engine.styles.collapse();
    let mut running_delta = snap_delta.clone();
    let mut boundaries: Vec<(usize, crate::entities::style::Styles)> = Vec::new();

    for child in node.children() {
        match child.kind() {
            SyntaxKind::Text => {
                // F-5b fatia 2 (P373): o render não é mais assado no node — viaja
                // na chain (custom), resolvido no layout.
                let text_node = Content::Text(child.text().as_str().into());
                // Intercepção eager para Selector::Text (Passo 68).
                parts.push(rules::intercept_content(text_node, ctx, engine)?);
            }
            // Passo 445 — SmartQuote: emite glyph localizado consoante
            // open/close (contexto adjacente) e `text.lang`/`smartquote` activo.
            SyntaxKind::SmartQuote => {
                let raw = child.text();
                let is_double = raw.as_str() == "\"";
                let lang = engine.styles.lang();

                let enabled = match engine.styles.custom("smartquote.enabled") {
                    Some(Value::Bool(b)) => *b,
                    _ => true,
                };

                let quote_node = if !enabled {
                    Content::Text(raw.as_str().into())
                } else {
                    let off = byte_offset;
                    let prev = src_str[..off].chars().last();
                    let next = src_str[off + child.len()..].chars().next();
                    let alternative = match engine.styles.custom("smartquote.alternative")
                    {
                        Some(Value::Bool(value)) => *value,
                        _ => false,
                    };
                    let configured = engine
                        .styles
                        .custom("smartquote.quotes")
                        .map(|value| {
                            crate::compiler::lang::quotes::parse_smartquote_quotes(
                                value,
                                child.span(),
                            )
                        })
                        .transpose()?;
                    let pair = crate::compiler::lang::quotes::resolve_smartquote_pair(
                        lang.as_ref(),
                        alternative,
                        configured.as_ref(),
                        is_double,
                    );
                    let glyph = if !is_double && is_word_char(prev) && is_word_char(next)
                    {
                        pair.close
                    } else if is_opening_context(prev) {
                        pair.open
                    } else {
                        pair.close
                    };
                    Content::Text(glyph)
                };
                parts.push(rules::intercept_content(quote_node, ctx, engine)?);
            }
            SyntaxKind::Space => parts.push(Content::Space),
            // P622: quebra de parágrafo semântica — distinta de Space.
            SyntaxKind::Parbreak => parts.push(Content::Parbreak),
            k if k.is_trivia() => continue,
            // Passo 56 — associação retroactiva: <label> envolve o nó precedente.
            // O parser expõe <label> como nó irmão (não filho) do nó anterior.
            // Entre o nó alvo e a label pode haver Space — salta-os para encontrar
            // o elemento real, re-insere-os a seguir ao Labelled.
            SyntaxKind::Label => {
                if let Some(label_ast) = child.cast::<AstLabel<'_>>() {
                    let name = label_ast.get().to_string();
                    // Recolher espaços finais para re-inserir após o Labelled.
                    let mut trailing: Vec<Content> = Vec::new();
                    while matches!(
                        parts.last(),
                        Some(Content::Space) | Some(Content::Empty)
                    ) {
                        trailing.push(parts.pop().unwrap());
                    }
                    if let Some(last) = parts.pop() {
                        // P791 — o wrapper Label também viaja pela maquinaria
                        // de show rules (apenas regras `Selector::Label` —
                        // o corpo já foi interceptado no seu próprio ponto).
                        let labelled = Content::label_auto(name, last);
                        parts.push(rules::intercept_labelled(labelled, ctx, engine)?);
                        trailing.reverse();
                        parts.extend(trailing);
                    } else {
                        // P802 — label órfã (sem elemento anterior anexável):
                        // warning, paridade vanilla `typst-eval/markup.rs`
                        // ("label `<x>` is not attached to anything"). A label
                        // é descartada (como no vanilla); os espaços recolhidos
                        // são re-inseridos para não se perderem.
                        engine.sink.warn_note(
                            child.span(),
                            &format!("label `<{name}>` is not attached to anything"),
                            "",
                        );
                        trailing.reverse();
                        parts.extend(trailing);
                    }
                }
            }
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    let value = eval_expr(expr, scopes, ctx, engine)?;
                    if let Some(content) = value_to_display_content(value) {
                        parts.push(content);
                    }
                }
            }
        }

        byte_offset += child.len();

        // P373/P431: fronteira de `#set` — o delta completo mudou vs o estado
        // corrente. Regista `(parts.len(), Styles do delta introduzido)` e actualiza
        // o corrente. (Só o `#set` muta `engine.styles` neste loop; o `#set` não
        // produz `part`, logo `parts.len()` é o início da cauda que este `#set`
        // escopa.)
        let cur = engine.styles.collapse();
        if cur != running_delta {
            let styles = cur.diff_styles(&running_delta);
            boundaries.push((parts.len(), styles));
            running_delta = cur;
        }
    }

    // P373: fold de dentro para fora — cada fronteira embrulha a sua cauda
    // (`parts[idx..]`) no delta do seu `#set`, produzindo o aninhamento
    // `Styled(A, [X, Styled(B, [Y])])` (escopo léxico, fiel ao vanilla). `#set`
    // único → 1 fronteira → 1 wrap = comportamento de antes (content-preserving).
    for (idx, styles) in boundaries.into_iter().rev() {
        let tail = parts.split_off(idx);
        parts.push(Content::Styled(Box::new(Content::sequence(tail)), styles));
    }

    // P863: as show rules de elemento (NodeKind) são interceptadas nos pontos
    // de construção do conteúdo, mas os parágrafos são sintetizados a partir de
    // vários nós de texto/`Space`/`Parbreak`. Aplica-se apenas as regras
    // `NodeKind::Par` ao fluxo montado, para que `#show par: ...` possa realizar
    // e transformar os parágrafos sem re-disparar regras de texto já aplicadas
    // eager em cada nó de texto.
    Ok(Value::Content(rules::intercept_paragraphs(
        Content::sequence(parts),
        ctx,
        engine,
    )?))
}

/// **P545** (extraído em **P780**) — conversão genérica `Value` → `Content`
/// para exibição (interpolação `#{expr}` em markup, e resolução de
/// identificador de variável em modo math — `compiler/eval/math.rs`). `None`
/// = nada a mostrar (`Value::None`, `Value::Counter`, texto vazio) —
/// distinto de `Some(Content::Empty)`, que só ocorre se o próprio
/// `Value::Content` embrulhado já for `Content::Empty` (preserva-se, não
/// se filtra).
///
/// Paridade conceptual com `Value::display()` do vanilla
/// (`typst-eval/src/math.rs::ExprExt::eval_display` chama `.display()`
/// genericamente) — cristalino não tem esse método unificado; esta função
/// é o equivalente inlined.
pub(crate) fn value_to_display_content(value: Value) -> Option<Content> {
    match value {
        Value::Content(c) => Some(c),
        Value::Str(s) => Some(Content::Text(s)),
        // P471 — símbolo Unicode → char como Content::Text.
        Value::Symbol(s) => Some(Content::Text(EcoString::from(s.value))),
        // P506 — state(key, init) → Content::State locatável.
        Value::State(s) => {
            Some(Content::state(s.key.to_string(), s.init.as_ref().clone()))
        }
        // P506 — counter(selector) → nada visível directamente (só via
        // .update()/.step()/.display()).
        Value::Counter(_) => None,
        Value::None => None,
        // P796 — Version usa o `Display` (`to_string()`), não o `repr()`:
        // `#sys.version` → "0.15.0", não "version(0, 15, 0)" (paridade
        // vanilla `Value::display`, `entities/version.md` §8b).
        Value::Version(v) => {
            let text = v.to_string();
            if text.is_empty() { None } else { Some(Content::Text(text.into())) }
        }
        // Valores primitivos convertem-se para texto. Int, Float, Bool,
        // Array, Dict, Length, Datetime, etc. usam repr_value.
        other => {
            // **P739C** — display de Float: paridade vanilla (Display de
            // f64 — inteiros exactos sem `.0`: `#(1.0)` → "1", `#(4/2)` →
            // "2", medido). `repr` mantém "1.0" (repr_value inalterado).
            // **P817-D** — display de Decimal: paridade vanilla
            // (`#decimal("1.50")` → "1.50", não `decimal("1.50")` — o repr
            // constructor-form é só para `repr()`, medido nos dois binários).
            let text = match &other {
                Value::Float(f) => format!("{f}"),
                Value::Decimal(d) => d.to_string(),
                _ => crate::compiler::eval::repr::repr_value(&other),
            };
            if text.is_empty() { None } else { Some(Content::Text(text.into())) }
        }
    }
}

pub(crate) fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = expr.span();
    #[cfg(p1339_observation)]
    {
        ctx.context_reads.borrow_mut().observation.expression_depth += 1;
    }
    let previous = ctx.replace_context_read_styles(engine.styles.clone());
    let previous_file = ctx.replace_context_read_file(Some(engine.current_file));
    let previous_rules = ctx.replace_context_read_rules(
        engine.show_rules.clone(),
        engine.active_guards.clone(),
    );
    let result = eval_expr_inner(expr, scopes, ctx, engine).map(|value| match value {
        Value::Func(func) => Value::Func(func.with_diagnostic_span(span)),
        other => other,
    });
    ctx.replace_context_read_styles(previous);
    ctx.replace_context_read_file(previous_file);
    ctx.replace_context_read_rules(previous_rules.0, previous_rules.1);
    #[cfg(p1339_observation)]
    {
        let mut log = ctx.context_reads.borrow_mut();
        log.observation.expression_depth -= 1;
        if log.observation.expression_depth == 0
            && log.observation.phase == ContextObservationPhase::Body
        {
            log.observation.body_result = Some(result.clone());
        }
    }
    result
}

fn eval_expr_inner(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node) => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node) => Ok(Value::Str(EcoString::from(node.get()?))),
        Expr::Bool(node) => Ok(Value::Bool(node.get())),
        Expr::None(_) => Ok(Value::None),
        Expr::Auto(_) => Ok(Value::Auto),

        Expr::Ident(ident) => {
            let name = ident.as_str();
            if name == "html"
                && !ctx
                    .features
                    .contains(crate::entities::compiler_features::Feature::Html)
            {
                return Err(vec![SourceDiagnostic::error(
                    ident.span(),
                    "cannot access variable `html` because the `html` feature is not enabled",
                )
                .with_hint("try enabling the `html` feature")
                .with_hint("see https://typst.app/help/compiler-features for more details")]);
            }
            // P772r — hint de subtracção quando o nome contém hífen,
            // paridade vanilla `foundations/scope.rs::unknown_variable`.
            scopes
                .get(name)
                .cloned()
                .ok_or_else(|| vec![bindings::unknown_variable(ident.span(), name)])
        }

        Expr::LetBinding(binding) => bindings::eval_let(binding, scopes, ctx, engine),

        Expr::CodeBlock(code_block) => {
            // Bloco de código — styles e show_rules locais (atomização
            // Passos 94 e 95). `#set`/`#show` dentro do bloco mutam as
            // cópias locais mas não afectam o chamador. Engine
            // reconstruído localmente (ADR-0044, Passo 109).
            let mut local_styles = engine.styles.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            let mut output = Value::None;
            // **P772l** — paridade vanilla `ast::CodeBlock::eval`
            // (`typst-eval/src/code.rs:317-323`): o bloco introduz um
            // âmbito léxico próprio via `scopes.enter()`/`exit()`. Sem isto,
            // um `let` interno mutava o âmbito do chamador directamente —
            // medido: `#let x = 1; #{ let x = 2; x }; #x` devolvia `2, 2`
            // em vez de `2, 1` (shadowing vazava para fora do bloco).
            scopes.enter();
            {
                let mut local_engine = Engine {
                    world: engine.world,
                    font_metrics: engine.font_metrics,
                    route: engine.route,
                    styles: &mut local_styles,
                    show_rules: &mut local_show_rules,
                    active_guards: &mut *engine.active_guards,
                    current_file: engine.current_file,
                    sink: &mut local_sink,
                };
                for expr in code_block.body().exprs() {
                    // **P728** — o valor do bloco é o `join` sequencial dos
                    // valores das expressões (paridade vanilla
                    // `typst-eval/src/code.rs:57` + `ops::join`), não só o
                    // da última. Medido: `{ (1,); (2,) }` → `(1, 2)`.
                    let span = expr.span();
                    let value = eval_expr(expr, scopes, ctx, &mut local_engine)?;
                    output = operators::join(output, value)
                        .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])?;
                    if ctx.flow.is_some() {
                        break;
                    }
                }
                // **P740A** — paridade vanilla `warn_for_discarded_content`
                // (`typst-eval/src/code.rs:413-430`): `return` incondicional
                // (flag `conditional == false`) com conteúdo acumulado antes
                // dele emite warning + hint; um segundo hint é adicionado
                // quando o conteúdo descartado contém updates de
                // state/counter (seletor `State|Counter` no vanilla, aqui
                // travessia directa das variants equivalentes). A emissão
                // acontece aqui — antes do chamador tratar o flow —, pelo
                // que o warning coexiste com o erro "cannot return outside
                // of function" (medido no vanilla).
                if let Some(FlowEvent::Return(span, Some(_), false)) = &ctx.flow {
                    if let Value::Content(c) = &output {
                        let hint2 = if content_has_state_or_counter(c) {
                            "state/counter updates are content that must end up in the document to have an effect"
                        } else {
                            ""
                        };
                        local_engine.sink.warn_note2(
                            *span,
                            "this return unconditionally discards the content before it",
                            "try omitting the `return` to automatically join all values",
                            hint2,
                        );
                    }
                }
            }
            scopes.exit();
            Ok(output)
        }

        // **P715** — `Assign`/`AddAssign`/`SubAssign`/`MulAssign`/`DivAssign`
        // não podem avaliar `lhs` como valor (precisam do nome/local para
        // mutar) — intercepta antes do dispatch genérico de operadores.
        // Ver `bindings::eval_assign`.
        Expr::Binary(binary)
            if matches!(
                binary.op(),
                BinOp::Assign
                    | BinOp::AddAssign
                    | BinOp::SubAssign
                    | BinOp::MulAssign
                    | BinOp::DivAssign
            ) =>
        {
            bindings::eval_assign(binary, scopes, ctx, engine)
        }

        // **P728** — short-circuit de `and`/`or` (paridade vanilla
        // `apply_binary` em `typst-eval/src/ops.rs:52-66`): o segundo
        // operando só é avaliado se o primeiro não decidir o resultado.
        // Medido: `false and (1/0 == 0)` → `false` (sem erro de divisão).
        Expr::Binary(binary) if matches!(binary.op(), BinOp::And | BinOp::Or) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx, engine)?;
            let decided = matches!(
                (binary.op(), &lhs),
                (BinOp::And, Value::Bool(false)) | (BinOp::Or, Value::Bool(true))
            );
            if decided {
                Ok(lhs)
            } else {
                let rhs = eval_expr(binary.rhs(), scopes, ctx, engine)?;
                operators::eval_binary_op(binary.op(), lhs, rhs)
                    .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
            }
        }

        Expr::Binary(binary) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx, engine)?;
            let rhs = eval_expr(binary.rhs(), scopes, ctx, engine)?;
            operators::eval_binary_op(binary.op(), lhs, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
        }

        Expr::Unary(unary) => {
            let operand = eval_expr(unary.expr(), scopes, ctx, engine)?;
            operators::eval_unary_op(unary.op(), operand)
                .map_err(|msg| vec![SourceDiagnostic::error(unary.span(), msg)])
        }

        Expr::Conditional(cond) => {
            control_flow::eval_conditional(cond, scopes, ctx, engine)
        }
        Expr::WhileLoop(loop_expr) => {
            control_flow::eval_while(loop_expr, scopes, ctx, engine)
        }
        Expr::ForLoop(loop_expr) => {
            control_flow::eval_for(loop_expr, scopes, ctx, engine)
        }

        Expr::Closure(c) => closures::eval_closure_expr(c, scopes, ctx, engine),
        Expr::FuncCall(c) => call_dispatch::eval_func_call(c, scopes, ctx, engine),

        Expr::Strong(s) => markup::eval_strong(s, scopes, ctx, engine),
        Expr::Emph(e) => markup::eval_emph(e, scopes, ctx, engine),
        Expr::Heading(h) => markup::eval_heading(h, scopes, ctx, engine),
        Expr::Raw(r) => markup::eval_raw(r),
        Expr::Link(l) => markup::eval_link(l, &*engine.styles),
        Expr::ListItem(i) => markup::eval_list_item(i, scopes, ctx, engine),
        Expr::EnumItem(i) => markup::eval_enum_item(i, scopes, ctx, engine),

        Expr::FieldAccess(a) => bindings::eval_field_access(a, scopes, ctx, engine),

        Expr::SetRule(s) => rules::eval_set_rule(s, scopes, ctx, engine),

        Expr::ContentBlock(content_block) => {
            // Content block [ ] — styles locais ao bloco. Engine
            // reconstruído localmente (ADR-0044, Passo 109).
            //
            // **Caso 4 / f3s3 (P340, F-realização fatia 2):** `show_rules` também
            // local (clone O(1) do Arc), espelhando o `CodeBlock` (`{}`). Antes,
            // o `[]` partilhava `&mut *engine.show_rules` → um `#show` dentro do
            // bloco VAZAVA para fora (mutava a chain do chamador da declaração em
            // diante). Agora confina ao escopo do bloco — paridade com o vanilla,
            // que confina via `StyledElem` (`content/mod.rs:744-752`). É o
            // confinamento estrutural que faltava ao modelo eager no `[]`.
            let mut local_styles = engine.styles.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            // **P772l** — paridade vanilla `ast::ContentBlock::eval`
            // (`typst-eval/src/code.rs:326-332`): mesmo âmbito léxico
            // próprio do `CodeBlock`. Sem isto, `#let` dentro de `[ ]`
            // vazava para o âmbito do chamador (mesmo bug do CodeBlock).
            scopes.enter();
            let mut local_engine = Engine {
                world: engine.world,
                font_metrics: engine.font_metrics,
                route: engine.route,
                styles: &mut local_styles,
                show_rules: &mut local_show_rules,
                active_guards: &mut *engine.active_guards,
                current_file: engine.current_file,
                sink: &mut local_sink,
            };
            let result = eval_markup(
                content_block.body().to_untyped(),
                scopes,
                ctx,
                &mut local_engine,
            );
            scopes.exit();
            result
        }

        Expr::Equation(eq) => {
            let block = eq.block();
            let body = math::eval_math_content(scopes, ctx, engine, eq.body())?;
            // F-5a de-bake (P364, `f_fronteira_e1.md` §3a.9): a equação **não
            // baka** mais o gate. O `#set math.equation(numbering:)` vive **só na
            // chain** (`custom("equation.numbering")` no `Content::Styled` da
            // fatia-1). O consumidor lê o gate da chain e mantém `block &&
            // numbering` (só equações de bloco numeram, paridade vanilla). Fonte
            // única.
            let content = crate::compiler::stdlib::equation_content(body, block);
            Ok(Value::Content(content))
        }

        Expr::Math(math) => {
            // Math node isolado (fora de Equation) — produzir como sequence.
            let content = math::eval_math_content(scopes, ctx, engine, math)?;
            Ok(Value::Content(content))
        }

        Expr::ModuleImport(i) => modules::eval_module_import(i, scopes, ctx, engine),
        Expr::ModuleInclude(i) => modules::eval_module_include(i, scopes, ctx, engine),

        // Passo 56 — referência cruzada: @nome → Content::Ref placeholder.
        // **P788** — suplemento explícito `@nome[sup]` avaliado como
        // `ContentBlock` normal (antes: descartado — `@sec1[Cap]`
        // renderizava o suplemento default; vanilla: "Cap 1").
        Expr::Ref(ref_node) => {
            let name = ref_node.target().to_string();
            let supplement = match ref_node.supplement() {
                Some(block) => {
                    match eval_expr(Expr::ContentBlock(block), scopes, ctx, engine)? {
                        Value::Content(c) => Some(c),
                        _ => None,
                    }
                }
                None => None,
            };
            Ok(Value::Content(Content::reference_with_supplement(name, supplement)))
        }

        // Passo 56 — label em contexto de código; associação retroactiva em markup
        // acontece via SyntaxKind::Label. Em código, <label> é um valor de primeira
        // classe (P509) para query/locate.
        Expr::Label(label_node) => {
            let name = label_node.get().to_string();
            Ok(Value::Label(crate::entities::label::Label(name)))
        }

        Expr::ShowRule(s) => rules::eval_show_rule(s, scopes, ctx, engine),

        // Passo 81 — array literal `(1fr, 1fr)` / `(10pt, auto, 1fr)`.
        // Necessário para o argumento `columns` de `grid()`.
        // **P718** — mirror de `ast::Array::eval` (vanilla `code.rs:224-271`):
        // `..spread` de `none` é ignorado, de `array` estende, de `dict`
        // erra (com hint se TODOS os itens forem spreads de dict — ver
        // `remaining_are_dict_spreads`), outro tipo erra sem hint.
        Expr::Array(arr) => {
            let items_vec: Vec<ArrayItem> = arr.items().collect();
            let mut result: Vec<Value> = Vec::with_capacity(items_vec.len());
            let mut all_dict_spreads = true;
            for (i, item) in items_vec.iter().enumerate() {
                match *item {
                    ArrayItem::Pos(expr) => {
                        all_dict_spreads = false;
                        result.push(eval_expr(expr, scopes, ctx, engine)?);
                    }
                    ArrayItem::Spread(spread) => {
                        let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                        match value {
                            Value::None => {}
                            Value::Array(a) => {
                                all_dict_spreads = false;
                                result.extend(a);
                            }
                            Value::Dict(dict) => {
                                let all_remaining_are_dict_spreads = all_dict_spreads
                                    && remaining_are_dict_spreads(
                                        &items_vec[i + 1..],
                                        scopes,
                                        ctx,
                                        engine,
                                    );
                                if !all_remaining_are_dict_spreads {
                                    return Err(vec![SourceDiagnostic::error(
                                        spread.span(),
                                        format!(
                                            "cannot spread {} into array",
                                            Value::Dict(dict).type_name()
                                        ),
                                    )]);
                                }
                                let full_text =
                                    arr.to_untyped().clone().into_text().to_string();
                                let fixed = full_text.replacen('(', "(: ", 1);
                                return Err(vec![SourceDiagnostic::error(
                                    spread.span(),
                                    "cannot spread dictionary into array".to_string(),
                                )
                                .with_hint(format!(
                                    "add a colon to create a dictionary instead: `{fixed}`"
                                ))]);
                            }
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    spread.span(),
                                    format!(
                                        "cannot spread {} into array",
                                        vanilla_type_name(&other)
                                    ),
                                )]);
                            }
                        }
                    }
                }
            }
            Ok(Value::Array(result))
        }

        // **P466** — dict literal `(a: 1, b: 2)` e `("a": 1)`.
        // Dicts com chaves keyed não-string (ex.: regex) são deixados como
        // `Value::None` para que callers especializados (ex.: `#set text(font:)`)
        // possam inspeccionar o AST directamente.
        // **P718** — `..spread` mirror de `ast::Dict::eval` (vanilla
        // `code.rs:273-306`): `none` ignorado, `dict` estende, outro erra.
        Expr::Dict(dict) => {
            let mut map = indexmap::IndexMap::default();
            for item in dict.items() {
                match item {
                    crate::entities::ast::expr::DictItem::Named(named) => {
                        let key = named.name().as_str();
                        let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                        map.insert(key.into(), value);
                    }
                    crate::entities::ast::expr::DictItem::Keyed(keyed) => {
                        let key_expr = keyed.key();
                        let key = match key_expr {
                            crate::entities::ast::expr::Expr::Str(node) => {
                                EcoString::from(node.get()?)
                            }
                            _ => return Ok(Value::None),
                        };
                        let value = eval_expr(keyed.expr(), scopes, ctx, engine)?;
                        map.insert(key, value);
                    }
                    crate::entities::ast::expr::DictItem::Spread(spread) => {
                        let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                        match value {
                            Value::None => {}
                            Value::Dict(d) => map.extend(d),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    spread.span(),
                                    format!(
                                        "cannot spread {} into dictionary",
                                        vanilla_type_name(&other)
                                    ),
                                )]);
                            }
                        }
                    }
                }
            }
            Ok(Value::Dict(map))
        }

        // `(expr)` — parêntese de agrupamento. Expressão única dentro de
        // parênteses avalia para o valor da expressão. Passo 83.
        // (Um tuplo com um elemento requer a vírgula trailing: `(x,)`.)
        Expr::Parenthesized(paren) => eval_expr(paren.expr(), scopes, ctx, engine),

        // Passo 76 — literais numéricos com unidade (ex: 100pt, 1.5em).
        Expr::Numeric(num) => {
            use crate::entities::ast::expr::Unit;
            use crate::entities::layout_types::{Abs, Angle, Length};
            let (value, unit) = num.get();
            match unit {
                Unit::Pt => Ok(Value::Length(Length { abs: Abs(value), em: 0.0 })),
                Unit::Mm => Ok(Value::Length(Length {
                    abs: Abs(value * Length::PT_PER_MM),
                    em: 0.0,
                })),
                Unit::Cm => Ok(Value::Length(Length {
                    abs: Abs(value * Length::PT_PER_CM),
                    em: 0.0,
                })),
                Unit::In => Ok(Value::Length(Length { abs: Abs(value * 72.0), em: 0.0 })),
                Unit::Em => Ok(Value::Length(Length { abs: Abs(0.0), em: value })),
                Unit::Deg => Ok(Value::Angle(Angle::deg(value))),
                Unit::Rad => Ok(Value::Angle(Angle::rad(value))),
                // P842 (#32) — percentual puro materializa `Ratio` (paridade
                // vanilla medida: `type(50%)` → ratio). Pré-P842 era
                // `Value::Relative` com abs zero (P469), o que fundia os
                // tipos ratio/relative. `Ratio + Length` → `Relative` nos
                // operadores, como no vanilla.
                Unit::Percent => Ok(Value::Ratio(
                    crate::entities::layout_types::Ratio::from_percent(value),
                )),
                Unit::Fr => Ok(Value::Fraction(value)),
            }
        }

        // **P506** — `context { body }` cria um bloco de delayed evaluation.
        // O parser expõe `Contextual` como sugar; construímos uma closure
        // sem argumentos que captura o scope actual e devolvemos
        // `Content::ContextBlock`.
        Expr::Contextual(node) => {
            let body_span = node.body().span();
            let body = node.body().to_untyped().clone();
            let captured = std::sync::Arc::new(scopes.snapshot());
            let closure = Func::closure(ClosureRepr {
                name: None,
                params: Vec::new(),
                sink_name: None,
                body,
                captured,
                // P772q — bloco `context { }`, paridade vanilla
                // `CapturesVisitor::new(scopes, Capturer::Context)`.
                capturer: crate::entities::scope::Capturer::Context,
            })
            .with_diagnostic_span(body_span);
            let id = ctx.next_context_id();
            Ok(Value::Content(Content::ContextBlock(Arc::new(ContextBlockElem {
                id,
                closure,
            }))))
        }

        Expr::Escape(v) => Ok(Value::Str(ecow::EcoString::from(v.get()))),
        Expr::Shorthand(v) => Ok(Value::Str(ecow::EcoString::from(v.get()))),
        Expr::Linebreak(_) => Ok(Value::Content(Content::linebreak())),

        // P635 — controlo de fluxo: definir `FlowEvent` em `ctx.flow` e
        // devolver `Value::None`. O consumo (e a detecção de "fora de
        // contexto") é feito pelos ciclos, por `apply_closure` e pelo
        // entrypoint `eval_with_full_error`.
        Expr::LoopBreak(node) => {
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Break(node.span()));
            }
            Ok(Value::None)
        }
        Expr::LoopContinue(node) => {
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Continue(node.span()));
            }
            Ok(Value::None)
        }
        Expr::FuncReturn(node) => {
            let value = node
                .body()
                .map(|body| eval_expr(body, scopes, ctx, engine))
                .transpose()?;
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Return(node.span(), value, false));
            }
            Ok(Value::None)
        }

        // Fronteira deliberada — variantes estritamente estruturais que não
        // entram no dispatcher normal (markup/math) ou ainda não migradas.
        Expr::Text(_)
        | Expr::Space(_)
        | Expr::Parbreak(_)
        | Expr::SmartQuote(_)
        | Expr::TermItem(_)
        | Expr::MathText(_)
        | Expr::MathIdent(_)
        | Expr::MathShorthand(_)
        | Expr::MathAlignPoint(_)
        | Expr::MathDelimited(_)
        | Expr::MathAttach(_)
        | Expr::MathPrimes(_)
        | Expr::MathFrac(_)
        | Expr::MathRoot(_) => Ok(Value::None),

        // **P715** — `(a, b) = expr`. Ver `bindings::eval_destruct_assignment`.
        Expr::DestructAssignment(node) => {
            bindings::eval_destruct_assignment(node, scopes, ctx, engine)
        }
    }
}

/// **P740A** — deteta updates de state/counter no conteúdo descartado
/// por um `return` (paridade do seletor `State::select_any() |
/// Counter::select_any()` do vanilla em `warn_for_discarded_content`).
/// Travessia directa: as variants da família (state, state update,
/// counter update e os dois displays de counter) disparam; desce em
/// `Sequence` e `Styled` (as formas que o `join` produz em markup).
fn content_has_state_or_counter(c: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match c {
        Content::State(_)
        | Content::StateUpdate(_)
        | Content::CounterUpdate(_)
        | Content::CounterDisplay(_)
        | Content::CounterDisplayCallback(_) => true,
        Content::Sequence(items) => items.iter().any(content_has_state_or_counter),
        Content::Styled(body, _) => content_has_state_or_counter(body),
        _ => false,
    }
}

/// **P718** — lookahead para o hint de `Expr::Array` (mirror do
/// `items.all(...)` de `ast::Array::eval`, vanilla `code.rs:253-265`):
/// devolve `true` sse todos os itens restantes forem `Spread` cuja
/// expressão avalia a `Value::Dict`. Qualquer `Pos`, spread de outro tipo,
/// ou erro de avaliação → `false` (short-circuit, sem consumir o resto).
fn remaining_are_dict_spreads(
    items: &[ArrayItem],
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> bool {
    for item in items {
        match item {
            ArrayItem::Spread(s) => match eval_expr(s.expr(), scopes, ctx, engine) {
                Ok(Value::Dict(_)) => continue,
                _ => return false,
            },
            ArrayItem::Pos(_) => return false,
        }
    }
    true
}

/// Avalia o corpo de um nó de markup como Content.
fn eval_markup_body(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match eval_markup(node, scopes, ctx, engine)? {
        Value::Content(c) => Ok(c),
        _ => Ok(Content::Empty),
    }
}

// eval_args, apply_func, apply_closure extraídos para eval/closures.rs (Passo 96.1).

// Math eval extraído para eval/math.rs no Passo 96.1 (ADR-0037).

// eval_let extraído para eval/bindings.rs (Passo 96.1).

// apply_show_rules e intercept_content extraídos para eval/rules.rs (Passo 96.1).

/// Constrói a stdlib: `type`, `len`, `range`, `rgb`, `luma`, `str`, `int`, `float`, `figure`, `assert`, `upper`, `lower`, `replace`, `calc`.
///
/// Passo 64 (DEBT-16): `native_figure` migrada do interceptador em eval.rs para cá.
/// O avaliador deixa de conhecer o nome "figure" — desacoplamento total.
fn make_stdlib(inputs: &SysInputs) -> Scope {
    make_stdlib_with_features(
        inputs,
        crate::entities::compiler_features::Features::default(),
    )
}

fn make_stdlib_with_features(
    inputs: &SysInputs,
    features: crate::entities::compiler_features::Features,
) -> Scope {
    use crate::compiler::stdlib::{
        // P735 — módulos emoji e pdf.
        build_emoji_module,
        // P471 — módulo sym. **P731** — `build_sym_module` (era `build_sym_dict`).
        build_sym_module,
        make_calc_module,
        make_html_module,
        make_math_module,
        make_pdf_module,
        make_sys_module,
        native_accent,
        native_align,
        native_assert,
        native_assert_eq,
        native_assert_ne,
        native_asset,
        // P311b.3 + P765b — math style funcs.
        native_bb,
        native_bibliography,
        native_block,
        native_bold,
        native_box,
        native_cal,
        native_cancel,
        // P387 (ADR-0111) — data import.
        native_cbor,
        // P701 — cbor.encode (Value → CBOR), acedido via namespace de `cbor`.
        native_cbor_encode,
        native_circle,
        native_cite,
        native_cmyk,
        native_colbreak,
        native_columns,
        native_context,
        native_counter_at,
        native_counter_display,
        native_counter_final,
        native_counter_step,
        native_csv,
        native_curve,
        native_curve_close,
        native_curve_cubic,
        native_curve_line,
        native_curve_move,
        native_curve_quad,
        native_display,
        native_divider,
        native_document,
        native_ellipse,
        native_emph,
        native_enum,
        native_eval,
        native_figure,
        native_flush,
        native_footnote,
        native_frak,
        native_grid,
        native_grid_cell,
        native_grid_footer,
        native_grid_header,
        native_grid_hline,
        native_grid_vline,
        native_h,
        native_heading,
        native_here,
        native_hide,
        native_highlight,
        native_image,
        native_inline,
        native_json,
        native_json_encode,
        native_layout,
        native_line,
        native_linebreak,
        native_link,
        // P470 — list/enum com marcadores configuráveis.
        native_list,
        native_locate,
        // P472 — lof/lot.
        native_lof,
        native_lorem,
        native_lot,
        native_lower,
        native_luma,
        native_math_italic,
        native_measure,
        native_metadata,
        native_mono,
        native_move,
        // P793 — numbering.
        native_numbering,
        native_oklab,
        native_oklch,
        native_op,
        native_outline,
        native_overline,
        native_pad,
        native_page,
        native_pagebreak,
        native_panic,
        native_par,
        native_parbreak,
        native_place,
        // P697 — builtin plugin (nível 2 de P696); P819 — transition.
        native_plugin,
        native_plugin_transition,
        native_polygon,
        native_query,
        native_quote,
        native_range,
        native_raw,
        native_read,
        native_rect,
        native_ref,
        native_repeat,
        native_replace,
        native_repr,
        native_rgb,
        native_rotate,
        native_sans,
        native_scale,
        native_scr,
        native_script,
        native_serif,
        native_skew,
        native_smallcaps,
        native_smartquote,
        native_square,
        native_sscript,
        native_stack,
        native_state_at,
        native_state_display,
        native_state_final,
        native_state_update,
        native_state_update_with,
        native_strike,
        native_strong,
        native_subscript,
        native_superscript,
        native_table,
        native_table_cell,
        native_table_footer,
        native_table_header,
        native_table_hline,
        native_table_vline,
        native_target,
        native_terms,
        native_title,
        native_toml,
        native_toml_encode,
        native_underline,
        native_underover,
        native_upper,
        native_upright,
        native_v,
        native_xml,
        native_yaml,
        native_yaml_encode,
    };
    let mut scope = Scope::new();
    scope.define("html", Value::Module(make_html_module()));
    // P685 — `type` é um valor-tipo chamável (invoca native_type via eval_func_call).
    scope.define("type", Value::Type(Type::Type));
    scope.define("repr", Value::Func(Func::native("repr", native_repr)));
    scope.define("range", Value::Func(Func::native("range", native_range)));
    // P1300 — cinco constructors globais ratificados: `rgb`, `luma`, `cmyk`,
    // `oklab` e `oklch`; `linear-rgb`, `hsl` e `hsv` são somente `color.*`.
    scope.define("rgb", Value::Func(Func::native("rgb", native_rgb)));
    scope.define("luma", Value::Func(Func::native("luma", native_luma)));
    scope.define("oklab", Value::Func(Func::native("oklab", native_oklab)));
    scope.define("oklch", Value::Func(Func::native("oklch", native_oklch)));
    scope.define("cmyk", Value::Func(Func::native("cmyk", native_cmyk)));
    // P685 — `str`, `int`, `float` são valores-tipo chamáveis. Os campos
    // `str.from-unicode` e `int.min`/`int.max` são agora resolvidos por field
    // access em `Value::Type` (ver `eval_field_access` em bindings.rs).
    scope.define("str", Value::Type(Type::Str));
    scope.define("int", Value::Type(Type::Int));
    scope.define("float", Value::Type(Type::Float));
    scope.define("path", Value::Type(Type::Path));

    // P685 — nomes de tipo como valores de primeira classe (sem colisão com
    // nomes já registados como função/módulo). Nenhum é chamável. Permite
    // `type(x) == length`, `type(x) == ratio`, etc. (paridade vanilla).
    scope.define("bool", Value::Type(Type::Bool));
    scope.define("length", Value::Type(Type::Length));
    scope.define("ratio", Value::Type(Type::Ratio));
    // P842 (#32) — binding global `relative` (paridade vanilla medida:
    // `type(30% + 1em) == relative` → true).
    scope.define("relative", Value::Type(Type::Relative));
    scope.define("angle", Value::Type(Type::Angle));
    scope.define("fraction", Value::Type(Type::Fraction));
    scope.define("array", Value::Type(Type::Array));
    scope.define("dictionary", Value::Type(Type::Dictionary));
    scope.define("function", Value::Type(Type::Function));
    scope.define("content", Value::Type(Type::Content));
    scope.define("arguments", Value::Type(Type::Arguments));
    scope.define("module", Value::Type(Type::Module));
    scope.define("datetime", Value::Type(Type::Datetime));
    scope.define("bytes", Value::Type(Type::Bytes));
    scope.define("symbol", Value::Type(Type::Symbol));
    scope.define("alignment", Value::Type(Type::Alignment));
    scope.define("direction", Value::Type(Type::Direction));
    scope.define("location", Value::Type(Type::Location));
    // P1140.1-B — tipos chamáveis; call_dispatch delega aos construtores.
    scope.define("decimal", Value::Type(Type::Decimal));
    scope.define("duration", Value::Type(Type::Duration));
    scope.define("version", Value::Type(Type::Version));
    scope.define("heading", Value::Func(Func::native("heading", native_heading)));
    scope.define("title", Value::Func(Func::native("title", native_title)));
    scope.define("outline", Value::Func(Func::native("outline", native_outline)));
    // P806 — `par` invocável (achado #12 de P798; ver `stdlib/structural.md`
    // §`native_par` para o scope do body-devolvido-directamente).
    scope.define("par", Value::Func(Func::native("par", native_par)));
    // **P472** — lof() e lot() como aliases de outline(target: "figures"/"tables").
    scope.define("lof", Value::Func(Func::native("lof", native_lof)));
    scope.define("lot", Value::Func(Func::native("lot", native_lot)));
    scope.define("strong", Value::Func(Func::native("strong", native_strong)));
    scope.define("emph", Value::Func(Func::native("emph", native_emph)));
    scope.define("raw", Value::Func(Func::native("raw", native_raw)));
    // P284 (ADR-0054 graded): text decoration — underline / strike /
    // overline. Cosméticos `stroke`/`offset`/`extent` opcionais; `evade`
    // e `background` scope-out per diagnóstico §A.1.
    scope.define("underline", Value::Func(Func::native("underline", native_underline)));
    scope.define("strike", Value::Func(Func::native("strike", native_strike)));
    scope.define("overline", Value::Func(Func::native("overline", native_overline)));
    // P408: smallcaps — variant + stdlib materializados; consumer em layout é
    // stub transparente (small caps real requer shaping OpenType, DEBT-53).
    scope.define("smallcaps", Value::Func(Func::native("smallcaps", native_smallcaps)));
    // P448: subscript e superscript via Content::Styled + Style.
    scope.define("sub", Value::Func(Func::native("sub", native_subscript)));
    scope.define("super", Value::Func(Func::native("super", native_superscript)));
    // P449: highlight via Content::Styled + Style::Highlight(fill).
    scope.define("highlight", Value::Func(Func::native("highlight", native_highlight)));
    // P287 (frente `P-smartquote`): função stdlib paralela ao markup `"..."`
    // P155. `alternative`/`quotes` scope-out per diagnóstico §A.2; `enabled:
    // false` emite glyph ASCII literal (paridade vanilla).
    scope
        .define("smartquote", Value::Func(Func::native("smartquote", native_smartquote)));
    scope.define("lorem", Value::Func(Func::native("lorem", native_lorem)));
    scope.define("regex", Value::Type(Type::Regex));
    scope.define("figure", Value::Func(Func::native("figure", native_figure)));
    scope.define("image", Value::Func(Func::native("image", native_image)));
    // P396 — constructor `tiling(...)` (pattern fill).
    scope.define("tiling", Value::Type(Type::Tiling));
    // P387 (ADR-0111) — data import: read + 6 parsers. Decode L1 puro compõe
    // com L3 World::read_bytes. Paridade do Value de saída (ADR-0107).
    scope.define("read", Value::Func(Func::native("read", native_read)));
    scope.define("csv", Value::Func(Func::native("csv", native_csv)));
    type Codec = fn(
        &mut EvalContext,
        &crate::entities::args::Args,
        &dyn World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value>;
    let codecs: [(&'static str, Codec, Codec); 3] = [
        ("json", native_json, native_json_encode),
        ("yaml", native_yaml, native_yaml_encode),
        ("toml", native_toml, native_toml_encode),
    ];
    for (name, decode, encode) in codecs {
        let mut namespace = Scope::new();
        namespace.define("encode", Value::Func(Func::native("encode", encode)));
        scope.define(
            name,
            Value::Func(Func::native_with_namespace(name, decode, Arc::new(namespace))),
        );
    }
    // P701 — `cbor` ganha namespace com `encode` (mesmo padrão de curve/grid/table).
    {
        let mut cbor_namespace = Scope::new();
        cbor_namespace.define(
            "encode",
            Value::Func(Func::native("cbor.encode", native_cbor_encode)),
        );
        scope.define(
            "cbor",
            Value::Func(Func::native_with_namespace(
                "cbor",
                native_cbor,
                Arc::new(cbor_namespace),
            )),
        );
    }
    scope.define("xml", Value::Func(Func::native("xml", native_xml)));
    // P697 — builtin `plugin()` (nível 2 de P696: sintaxe + leitura; runtime em P698).
    // P819 — `plugin` ganha namespace com `transition` (mesmo padrão de cbor/curve/grid).
    {
        let mut plugin_namespace = Scope::new();
        plugin_namespace.define(
            "transition",
            Value::Func(Func::native("plugin.transition", native_plugin_transition)),
        );
        scope.define(
            "plugin",
            Value::Func(Func::native_with_namespace(
                "plugin",
                native_plugin,
                Arc::new(plugin_namespace),
            )),
        );
    }
    scope.define("rect", Value::Func(Func::native("rect", native_rect)));
    scope.define("square", Value::Func(Func::native("square", native_square)));
    scope.define("ellipse", Value::Func(Func::native("ellipse", native_ellipse)));
    scope.define("circle", Value::Func(Func::native("circle", native_circle)));
    scope.define("line", Value::Func(Func::native("line", native_line)));
    scope.define("polygon", Value::Func(Func::native("polygon", native_polygon)));
    // P293 (frente `P-curve-geometry`): activação posterior de
    // `PathItem::CubicTo` via stdlib novo. Reaplicação ADR-0099 para
    // `PathItem` (paralelo P285-P292 para `Style`). Hash `export.rs`
    // preservado pelo 10º passo consecutivo — emit já existe.
    // P513 — `curve` ganha namespace com move/line/cubic/quad/close.
    {
        let mut curve_namespace = Scope::new();
        curve_namespace
            .define("move", Value::Func(Func::native("curve.move", native_curve_move)));
        curve_namespace
            .define("line", Value::Func(Func::native("curve.line", native_curve_line)));
        curve_namespace.define(
            "cubic",
            Value::Func(Func::native("curve.cubic", native_curve_cubic)),
        );
        curve_namespace
            .define("quad", Value::Func(Func::native("curve.quad", native_curve_quad)));
        curve_namespace.define(
            "close",
            Value::Func(Func::native("curve.close", native_curve_close)),
        );
        scope.define(
            "curve",
            Value::Func(Func::native_with_namespace(
                "curve",
                native_curve,
                Arc::new(curve_namespace),
            )),
        );
    }
    // P512 — `grid` com namespace para cell/header/footer/hline/vline.
    {
        let mut grid_namespace = Scope::new();
        grid_namespace
            .define("cell", Value::Func(Func::native("cell", native_grid_cell)));
        grid_namespace
            .define("header", Value::Func(Func::native("header", native_grid_header)));
        grid_namespace
            .define("footer", Value::Func(Func::native("footer", native_grid_footer)));
        grid_namespace
            .define("hline", Value::Func(Func::native("hline", native_grid_hline)));
        grid_namespace
            .define("vline", Value::Func(Func::native("vline", native_grid_vline)));
        scope.define(
            "grid",
            Value::Func(Func::native_with_namespace(
                "grid",
                native_grid,
                Arc::new(grid_namespace),
            )),
        );
    }
    // Lote F-2 S4/D4 (P335): `page(...)` função-forma legacy removida.
    scope.define("move", Value::Func(Func::native("move", native_move)));
    scope.define("rotate", Value::Func(Func::native("rotate", native_rotate)));
    scope.define("scale", Value::Func(Func::native("scale", native_scale)));
    // Passo 156F (ADR-0061 Fase 1, sub-passo 4): skew via matriz unificada.
    scope.define("skew", Value::Func(Func::native("skew", native_skew)));
    scope.define("align", Value::Func(Func::native("align", native_align)));
    {
        let mut place_namespace = Scope::new();
        place_namespace.define("flush", Value::Func(Func::native("flush", native_flush)));
        scope.define(
            "place",
            Value::Func(Func::native_with_namespace(
                "place",
                native_place,
                Arc::new(place_namespace),
            )),
        );
    }
    // P723 — `assert` ganha namespace com eq/ne (bloqueio real do cetz;
    // a premissa do passo apontava `curve`, refutada pela sonda — o
    // namespace de curve existe desde P513).
    {
        let mut assert_namespace = Scope::new();
        assert_namespace
            .define("eq", Value::Func(Func::native("assert.eq", native_assert_eq)));
        assert_namespace
            .define("ne", Value::Func(Func::native("assert.ne", native_assert_ne)));
        scope.define(
            "assert",
            Value::Func(Func::native_with_namespace(
                "assert",
                native_assert,
                Arc::new(assert_namespace),
            )),
        );
    }
    scope.define("panic", Value::Func(Func::native("panic", native_panic)));
    scope.define(
        "numbering",
        Value::Func(Func::native_with_engine("numbering", native_numbering)),
    );
    // P394: eval(source) — re-avalia string como markup Typst no contexto actual.
    scope.define("eval", Value::Func(Func::native_with_engine("eval", native_eval)));
    // P169 (M9 sub-passo 1): metadata(value) — feature Introspection vanilla.
    scope.define("metadata", Value::Func(Func::native("metadata", native_metadata)));
    // P506: state(key, init) como valor de primeira classe.
    // **P737** — `state` é `Value::Type` (paridade vanilla — medido:
    // `type(state)` → `type`); chamabilidade via despacho P685 em
    // `eval/closures.rs` (`Type::State` → `native_state`).
    scope.define("state", Value::Type(Type::State));
    // P506: counter(selector) como valor de primeira classe.
    // **P737** — idem `state`: `Value::Type(Type::Counter)`.
    scope.define("counter", Value::Type(Type::Counter));
    // P506: context { expr } — delayed evaluation block.
    scope.define("context", Value::Func(Func::native("context", native_context)));
    // P171 (M9 sub-passo 3): state_update(key, value) — mantido como compatibilidade.
    scope.define(
        "state_update",
        Value::Func(Func::native("state_update", native_state_update)),
    );
    // P236 (Fase 5 Layout candidata Categoria D 1/?, refino aditivo
    // pós-P236.div-1): state_final(key) — valor final do state pós-walk.
    // Paralelo a counter_final P176. Reusa Introspector::state_final_value
    // P171. Retorna Value (init se ausente; última update caso contrário).
    scope.define(
        "state_final",
        Value::Func(Func::native("state_final", native_state_final)),
    );
    // P237 (Fase 5 Layout candidata Categoria D 1/?, refino estendido):
    // state_at(key, label) — valor do state na Location associada ao label.
    // Paralelo absoluto a counter_at P177. Reusa query_by_label P139+P140
    // + state_value P171. Retorna Value::None se key/label inexistentes
    // ou state nunca actualizado antes da Location.
    scope.define("state_at", Value::Func(Func::native("state_at", native_state_at)));
    // P172 (M9 sub-passo 4): state_update_with(key, fn) — callback variant.
    // **Stub**: from_tags ignora Func variant até pipeline restructuring.
    scope.define(
        "state_update_with",
        Value::Func(Func::native("state_update_with", native_state_update_with)),
    );
    // P240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ):
    // state_display(key, [callback]) — render-mediated state display real
    // walk-time. Walk emite `Content::StateDisplay` tag; `apply_state_displays`
    // pós-fixpoint pre-renderiza Content via apply_func(callback, [value],
    // ctx, engine). Layouter consome via Introspector::state_display_value
    // (Layouter permanece puro — Opção γ vs α/β/δ P239 audit).
    scope.define(
        "state_display",
        Value::Func(Func::native("state_display", native_state_display)),
    );
    // P241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial M7+2 paralelo P240):
    // counter_display(key, [callback]) — render-mediated counter display
    // real walk-time. Walk emite `Content::CounterDisplayCallback` tag;
    // `apply_counter_displays` pós-fixpoint converte counter slice para
    // Value::Array e aplica callback. Distinto de Content::CounterDisplay
    // { kind } legacy single-pass.
    scope.define(
        "counter_display",
        Value::Func(Func::native("counter_display", native_counter_display)),
    );
    // P175 (M9 sub-passo 5): query(kind_str) — consulta ctx.introspector
    // da iter de fixpoint anterior. Retorna Value::Int(count) — forma
    // minimal sem Value::Location.
    scope.define("query", Value::Func(Func::native("query", native_query)));
    // P504: selector(kind|func) — constrói selector como valor de primeira classe.
    scope.define("selector", Value::Type(Type::Selector));
    // P208B (M9c Bloco IV): here() — retorna Value::Location(loc) onde
    // loc = ctx.current_location. Erro contextual se current_location
    // é None (P208B infra minimal; captura automática deferred).
    scope.define("here", Value::Func(Func::native("here", native_here)));
    // P772w: target() — devolve sempre "paged" (cristalino só exporta PDF
    // via layout paginado; sem pipeline HTML/Bundle). Achado: função
    // ausente do scope global até este passo.
    scope.define("target", Value::Func(Func::native("target", native_target)));
    // P208C (M9c Bloco IV): locate(kind) — retorna primeira Location
    // do kind indicado, ou Value::None se sem matches. Reusa pattern
    // de native_query + Selector::Kind (P175 minimal); locate(<label>)
    // requer P209 (Selector::Label).
    scope.define("locate", Value::Func(Func::native("locate", native_locate)));
    // P210B (M9c Bloco V): counter_step(key) — emite
    // Content::CounterUpdate { key, action: Step } que aplica em
    // layout time. Q1=β subset minimal (counter.display + state.get
    // deferred até walk advance per P210A C3).
    scope.define(
        "counter_step",
        Value::Func(Func::native("counter_step", native_counter_step)),
    );
    // P176 (M9 sub-passo 6): counter_final(key) — formato hierárquico
    // do counter na iter de fixpoint anterior. Reusa
    // Introspector::formatted_counter (P170). Retorna Value::Str.
    scope.define(
        "counter_final",
        Value::Func(Func::native("counter_final", native_counter_final)),
    );
    // P177 (M9 sub-passo 7): counter_at(key, label) — valor do counter
    // na Location associada ao label. Reusa query_by_label +
    // formatted_counter_at. Retorna Value::Str.
    scope
        .define("counter_at", Value::Func(Func::native("counter_at", native_counter_at)));
    scope.define("upper", Value::Func(Func::native("upper", native_upper)));
    scope.define("lower", Value::Func(Func::native("lower", native_lower)));
    scope.define("replace", Value::Func(Func::native("replace", native_replace)));
    // Passo 154B (ADR-0060 Fase 1): terms + divider.
    scope.define("terms", Value::Func(Func::native("terms", native_terms)));
    scope.define("divider", Value::Func(Func::native("divider", native_divider)));
    // Passo 155 (ADR-0060 Fase 1, sub-passo 2): quote.
    scope.define("quote", Value::Func(Func::native("quote", native_quote)));
    // P397 — document metadata wrapper + asset placeholder.
    scope.define("document", Value::Func(Func::native("document", native_document)));
    scope.define("asset", Value::Func(Func::native("asset", native_asset)));
    // Passo 295 — footnote Fase 1 (marker only).
    scope.define("footnote", Value::Func(Func::native("footnote", native_footnote)));
    // Passo 296 — math accent + cancel (HIV + (a) minimal).
    scope.define("accent", Value::Func(Func::native("accent", native_accent)));
    scope.define("cancel", Value::Func(Func::native("cancel", native_cancel)));
    // Passo 297 — math underover (HV'.a + (b) Option fields).
    scope.define("underover", Value::Func(Func::native("underover", native_underover)));
    // Passo 298 — math op (HV'' adaptado; cross-variant interaction).
    scope.define("op", Value::Func(Func::native_with_engine("op", native_op)));
    // P311b.3 + P765b — 14 funções math style.
    scope.define("bb", Value::Func(Func::native("bb", native_bb)));
    scope.define("bold", Value::Func(Func::native("bold", native_bold)));
    scope.define("cal", Value::Func(Func::native("cal", native_cal)));
    scope.define("display", Value::Func(Func::native("display", native_display)));
    scope.define("frak", Value::Func(Func::native("frak", native_frak)));
    scope.define("inline", Value::Func(Func::native("inline", native_inline)));
    scope.define("italic", Value::Func(Func::native("italic", native_math_italic)));
    scope.define("mono", Value::Func(Func::native("mono", native_mono)));
    scope.define("sans", Value::Func(Func::native("sans", native_sans)));
    scope.define("scr", Value::Func(Func::native("scr", native_scr)));
    scope.define("script", Value::Func(Func::native("script", native_script)));
    scope.define("serif", Value::Func(Func::native("serif", native_serif)));
    scope.define("sscript", Value::Func(Func::native("sscript", native_sscript)));
    scope.define("upright", Value::Func(Func::native("upright", native_upright)));
    // Passo 156C (ADR-0061 Fase 1, sub-passo 1): pad + hide.
    scope.define("pad", Value::Func(Func::native("pad", native_pad)));
    scope.define("hide", Value::Func(Func::native("hide", native_hide)));
    // Passo 156D (ADR-0061 Fase 1, sub-passo 2): h + v spacing.
    scope.define("h", Value::Func(Func::native("h", native_h)));
    scope.define("v", Value::Func(Func::native("v", native_v)));
    // P1140.10 — constructor público; efeito visual de `justify` em P1140.11.
    scope.define("linebreak", Value::Func(Func::native("linebreak", native_linebreak)));
    // P1140.17 — função pública equivalente ao marker de linha vazia.
    scope.define("parbreak", Value::Func(Func::native("parbreak", native_parbreak)));
    // Passo 156E (ADR-0061 Fase 1, sub-passo 3): pagebreak manual.
    scope.define("pagebreak", Value::Func(Func::native("pagebreak", native_pagebreak)));
    // P1140.26 — o scope é também clonado para `std`, logo esta única
    // definição materializa `page` e `std.page` com o mesmo Func.
    scope.define("page", Value::Func(Func::native_with_engine("page", native_page)));
    // Passo 156G (ADR-0061 Fase 2 sub-passo 1): block container.
    scope.define("block", Value::Func(Func::native("block", native_block)));
    // Passo 156H (ADR-0061 Fase 2 sub-passo 2): box inline container.
    scope.define("box", Value::Func(Func::native("box", native_box)));
    // Passo 156I (ADR-0061 Fase 2 sub-passo 3): stack compositivo.
    // **Último sub-passo Fase 2; atinge target 72% Layout.**
    scope.define("stack", Value::Func(Func::native("stack", native_stack)));
    // Passo 156J (ADR-0061 Fase 3 sub-passo 1): repeat (paridade
    // estrutural; algoritmo dinâmico diferido per ADR-0054 graded).
    // **Primeira aplicação Fase 3.**
    scope.define("repeat", Value::Func(Func::native("repeat", native_repeat)));
    // P218 (DEBT-56 sub-fase b — Layout Fase 3): columns(count, body,
    // gutter: ?). Variant Content::Columns materializado em P217;
    // arm Layouter é stub transparente (consumer real P219).
    scope.define("columns", Value::Func(Func::native("columns", native_columns)));
    // P220 (DEBT-56 sub-fase b 4/4 — Layout Fase 3): colbreak(weak: ?).
    // Variant Content::Colbreak agregado (variant + arm + stdlib);
    // arm Layouter Opção β graded — downgrade a pagebreak literal.
    // **Fecha sub-fase (b) DEBT-56 estructuralmente.**
    scope.define("colbreak", Value::Func(Func::native("colbreak", native_colbreak)));
    // P222 (Fase 4 Layout candidata sub-passo 1; ADR-0066 §"Plano
    // promoção" Bloco C primeira materialização parcial): measure(body)
    // → Dict { width: Length, height: Length }. Helper privado
    // `measure_content` promovido a `pub(crate)`; semantic graded
    // (single-pass; runtime queries genuínas diferidas; width override
    // scope-out Opção β).
    scope.define("measure", Value::Func(Func::native("measure", native_measure)));
    // P792 — layout(func): fornece as dimensões do container via callback.
    // A lógica real vive na intercepção em eval/closures.rs (mesmo padrão
    // de measure/P712). Paridade vanilla: `layout/layout.rs:66` #[func].
    scope.define("layout", Value::Func(Func::native("layout", native_layout)));
    // P227 (ADR-0079 PROPOSTO Fase 5 Categoria A.1 sub-passo 1):
    // stroke(paint: ?, thickness: ?) constructor para Value::Stroke;
    // parametriza borders Grid/Table via Stroke shorthand parsing.
    // Valida ADR-0080 PROPOSTO N=7 → 8 (L0 não tocado em P227).
    scope.define("stroke", Value::Type(Type::Stroke));
    // Passo 157A (ADR-0060 Fase 2 sub-passo 1): table minimal
    // (subset 3 fields; reusa layout_grid; TableCell/Header/Footer
    // diferidos para P157B/C). **Primeiro sub-passo Model Fase 2.**
    // P493b — table com namespace anexado para table.header/footer/cell.
    // P512 — adiciona table.hline / table.vline.
    let mut table_namespace = Scope::new();
    table_namespace
        .define("header", Value::Func(Func::native("header", native_table_header)));
    table_namespace
        .define("footer", Value::Func(Func::native("footer", native_table_footer)));
    table_namespace.define("cell", Value::Func(Func::native("cell", native_table_cell)));
    table_namespace
        .define("hline", Value::Func(Func::native("hline", native_table_hline)));
    table_namespace
        .define("vline", Value::Func(Func::native("vline", native_table_vline)));
    scope.define(
        "table",
        Value::Func(Func::native_with_namespace(
            "table",
            native_table,
            Arc::new(table_namespace),
        )),
    );
    // Passo 157B (ADR-0060 Fase 2 sub-passo 2): table cell
    // (subset 5 fields; ADR-0064 Caso A para x/y, Caso C para
    // colspan/rowspan; placement diferido em DEBT-34e).
    // Mantém bindings flat como fallback não-regressão.
    scope
        .define("table_cell", Value::Func(Func::native("table_cell", native_table_cell)));
    // Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha "table foundations"):
    // par simétrico TableHeader/TableFooter. ADR-0064 Caso D para
    // `repeat: bool` default true (primeira aplicação Caso D em
    // Model). Algoritmo de repetição em page breaks diferido em
    // DEBT-56 (refactor multi-region). Mantém bindings flat como fallback.
    scope.define(
        "table_header",
        Value::Func(Func::native("table_header", native_table_header)),
    );
    scope.define(
        "table_footer",
        Value::Func(Func::native("table_footer", native_table_footer)),
    );
    // P224 (ADR-0061 Fase 4 Layout candidata sub-passo 3 — fecha série α
    // "terminar Layout"): grid_cell + grid_header + grid_footer paridade
    // P157B/C literal; grid_cell resolve placement real via P224.C
    // grid_placement.rs (fecha DEBT-34e).
    scope.define("grid_cell", Value::Func(Func::native("grid_cell", native_grid_cell)));
    scope.define(
        "grid_header",
        Value::Func(Func::native("grid_header", native_grid_header)),
    );
    scope.define(
        "grid_footer",
        Value::Func(Func::native("grid_footer", native_grid_footer)),
    );
    // Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado):
    // subset minimal sem hayagriva (input cristalino literal
    // Vec<BibEntry>). Naming flat per padrão P157B; placeholder
    // render per ADR-0033 + ADR-0054 graded; sem validação
    // cross-reference (ADR-0017 adiada). Refinos futuros (CSL,
    // form, hayagriva) NÃO reservados per política P158.
    scope.define(
        "bibliography",
        Value::Func(Func::native("bibliography", native_bibliography)),
    );
    scope.define("cite", Value::Func(Func::native("cite", native_cite)));
    scope.define("link", Value::Func(Func::native("link", native_link)));
    // P1140.2 — label é o valor-tipo chamável; o construtor é despachado
    // estaticamente por call_dispatch.
    scope.define("label", Value::Type(Type::Label));
    scope.define("ref", Value::Func(Func::native("ref", native_ref)));
    // P470 — list/enum com marcadores configuráveis.
    scope.define("list", Value::Func(Func::native("list", native_list)));
    scope.define("enum", Value::Func(Func::native("enum", native_enum)));
    // P471 — módulo sym de símbolos Unicode. **P731** — `Value::Module`
    // (era `Value::Dict`; paridade vanilla `type(sym)` → `module`).
    scope.define("sym", build_sym_module());
    scope.define("calc", make_calc_module());
    // P735 — módulos `emoji` (tabela codex, 1 codepoint + face) e `pdf`
    // (attach = scope-out com erro; artifact = passthrough do body).
    scope.define("emoji", build_emoji_module());
    scope.define("pdf", make_pdf_module(features));
    // P476 — operadores de cor. **P736** — `color` é `Value::Type`
    // (paridade vanilla — medido: `type(color)` → `type`); fields via
    // `color_type_field` em field access.
    scope.define("color", Value::Type(Type::Color));
    // P262 — `gradient.linear(...)` via field access no tipo (P736; era
    // module dict, ADR-0087). Paridade vanilla: `type(gradient)` → `type`.
    scope.define("gradient", Value::Type(Type::Gradient));
    // P299 — `math.sin`/`math.lim`/etc. (P298.X; 42 operadores
    // pré-definidos paridade vanilla via SSoT MathOp).
    scope.define("math", make_math_module());
    // P694 — módulo `sys` (sys.version / sys.inputs); `inputs` vem de
    // `World::inputs()` (vazio por omissão).
    scope.define("sys", make_sys_module(inputs));

    // Constantes de alinhamento (Passo 84.5, encerra DEBT-36).
    // Sintaxe preferida: `align(center, ...)`, `align(center + bottom, ...)`.
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    scope.define("left", Value::Align(Align2D { h: Some(HAlign::Left), v: None }));
    scope.define("center", Value::Align(Align2D { h: Some(HAlign::Center), v: None }));
    scope.define("right", Value::Align(Align2D { h: Some(HAlign::Right), v: None }));
    scope.define("start", Value::Align(Align2D { h: Some(HAlign::Start), v: None }));
    scope.define("end", Value::Align(Align2D { h: Some(HAlign::End), v: None }));
    scope.define("top", Value::Align(Align2D { h: None, v: Some(VAlign::Top) }));
    scope.define("horizon", Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }));
    scope.define("bottom", Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }));

    // Constantes de direcção (Passo 576).
    use crate::entities::dir::Dir;
    scope.define("ltr", Value::Dir(Dir::LTR));
    scope.define("rtl", Value::Dir(Dir::RTL));
    scope.define("ttb", Value::Dir(Dir::TTB));
    scope.define("btt", Value::Dir(Dir::BTT));

    scope
}

// ── Auxiliares para intercepção de counter(...).method() ──────────────────

/// Extrai o nome do contador de uma expressão `counter(key)`.
// extract_counter_key e eval_counter_method extraídos para eval/bindings.rs (Passo 96.1).

#[cfg(test)]
mod tests;
#[cfg(test)]
mod p1292_d_tests {
    use super::*;
    use crate::entities::args::Args;

    #[test]
    fn p1292_d_place_and_with_share_the_flush_namespace() {
        let stdlib = make_stdlib(&SysInputs::default());
        let Value::Func(place) = stdlib.get("place").unwrap() else {
            panic!("place must remain callable");
        };
        assert_eq!(place.name(), Some("place"));

        let namespace = place.namespace().expect("place namespace");
        let Value::Func(flush) = namespace.get("flush").unwrap() else {
            panic!("place.flush must be callable");
        };
        assert_eq!(flush.name(), Some("flush"));

        let with = place.clone().with(Args::positional(Vec::new()));
        assert!(with.namespace().unwrap().get("flush").is_some());
    }
}
#[cfg(test)]
mod p1293_d_tests {
    use super::*;
    use crate::entities::args::Args;

    fn assert_short_namespace_names(namespace_name: &str) {
        let stdlib = make_stdlib(&SysInputs::default());
        let Value::Func(container) = stdlib.get(namespace_name).unwrap() else {
            panic!("{namespace_name} must remain callable");
        };
        let namespace = container.namespace().expect("namespace must exist");
        let with_container = container.clone().with(Args::positional(Vec::new()));
        let with_namespace =
            with_container.namespace().expect("with must preserve namespace");

        for expected in ["cell", "header", "footer", "hline", "vline"] {
            let Value::Func(member) = namespace.get(expected).unwrap() else {
                panic!("{namespace_name}.{expected} must remain callable");
            };
            assert_eq!(member.name(), Some(expected));
            assert_eq!(
                member.clone().with(Args::positional(Vec::new())).name(),
                Some(expected),
            );

            let Value::Func(with_member) = with_namespace.get(expected).unwrap() else {
                panic!("{namespace_name}.with(...).{expected} must remain callable");
            };
            assert_eq!(with_member.name(), Some(expected));
        }
    }

    #[test]
    fn p1293_d_grid_namespace_uses_short_names_direct_and_with() {
        assert_short_namespace_names("grid");
    }

    #[test]
    fn p1293_d_table_namespace_uses_short_names_direct_and_with() {
        assert_short_namespace_names("table");
    }

    #[test]
    fn p1293_d_flat_aliases_keep_historical_names_and_lines_stay_namespaced() {
        let stdlib = make_stdlib(&SysInputs::default());
        for alias in [
            "grid_cell",
            "grid_header",
            "grid_footer",
            "table_cell",
            "table_header",
            "table_footer",
        ] {
            let Value::Func(function) = stdlib.get(alias).unwrap() else {
                panic!("{alias} must remain callable");
            };
            assert_eq!(function.name(), Some(alias));
        }

        for absent in ["grid_hline", "grid_vline", "table_hline", "table_vline"] {
            assert!(stdlib.get(absent).is_none(), "{absent} must stay namespaced");
        }
    }
}
#[cfg(test)]
pub(crate) use crate::compiler::eval::tests::eval_for_test;
// Re-export para o módulo de tests (que usa `use super::*;`).
#[cfg(test)]
pub(crate) use crate::compiler::eval::operators::{eval_binary_op, eval_unary_op};
