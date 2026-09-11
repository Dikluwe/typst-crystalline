// Independent frozen terminal-policy oracle. Include under cfg(test) in L3.
// run_suite requires a late binding to the REAL product session/compile path.
// No fallback runner, mock stabilization, or synthesized outcome is provided.
use std::sync::Arc;
use typst_core::entities::func::Func;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{Severity, SourceDiagnostic, SourceResult};
use typst_core::entities::span::Span;
use typst_core::entities::value::Value;

pub const TERMINAL: &str = "contextual stability could not be verified";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Case {
    OpaqueAtCeiling,
    MixedDifferentAndOpaque,
    QueryDifferentThenStable,
    OriginalErrorPrecedence,
    ProvenImpossibleTopologyTerminal,
    ClosedIncreasing,
}

pub struct Input {
    pub case: Case,
    pub source_text: &'static str,
    // Binding substitutes only the labeled <opaque> metadata input at the
    // real post-eval/pre-discovery session boundary, never a read result.
    pub opaque_input: Option<Value>,
    // Sentinel warnings are input data emitted into actual sinks at the
    // prescribed boundaries in the binding specification, not returned DTOs.
    pub warning_names: [&'static str; 5],
}

pub struct Observation {
    pub source: Source,
    pub result: SourceResult<()>, // lossless Ok(document)->Ok(()) projection
    pub warnings: Vec<SourceDiagnostic>,
    pub original_errors: Vec<SourceDiagnostic>, // actual pre-return body Err
    pub attempts: Vec<usize>, // actual AttemptStarted hooks
    pub export_calls: usize, // actual exporter dispatch hooks
    pub final_read_counter: Option<Vec<usize>>,
    pub metadata_integers: Vec<i64>,
    pub selected_body_ok: bool,
    pub topology_impossibility: bool,
}

pub type ActualRun = fn(&Input) -> Observation;

fn source(case: Case) -> &'static str {
    match case {
        Case::OpaqueAtCeiling => "#metadata(0)<opaque>\n#let c = counter(heading.where())\n#context { let _ = c.final(); let _ = query(<opaque>); [] }\n",
        Case::MixedDifferentAndOpaque => "#metadata(0)<opaque>\n#let c = counter(heading.where())\n#context { let _ = query(<opaque>); c.update(c.final().first() + 1) }\n",
        Case::QueryDifferentThenStable => "#let c = counter(heading.where())\n#context { let _ = c.final(); metadata(query(<made>).len()) }\n#context [#metadata(1)<made>]\n",
        Case::OriginalErrorPrecedence => "#metadata(0)<opaque>\n#let c = counter(heading.where())\n#context { query(<opaque>); c.final(); panic(\"P1340_ORIGINAL\") }\n",
        Case::ProvenImpossibleTopologyTerminal => "#let c = counter(heading.where())\n#context c.final()\n",
        Case::ClosedIncreasing => "#let c = counter(heading.where())\n#context c.update(c.final().first() + 1)\n#context metadata(c.final().first())\n",
    }
}

fn equal_diagnostics(left: &[SourceDiagnostic], right: &[SourceDiagnostic]) {
    assert_eq!(left.len(), right.len());
    for (a, b) in left.iter().zip(right) {
        assert_eq!(a.severity, b.severity);
        assert_eq!(a.span, b.span);
        assert_eq!(a.message, b.message);
        assert_eq!(a.hints, b.hints);
        assert_eq!(a.trace, b.trace);
    }
}

fn terminal_error(errors: &[SourceDiagnostic], root: Span) {
    assert_eq!(errors.len(), 1);
    let error = &errors[0];
    assert_eq!(error.severity, Severity::Error);
    assert_eq!(error.message, TERMINAL);
    assert_eq!(error.span, root);
    assert!(error.hints.is_empty());
    assert!(error.trace.is_empty());
}

// Compilation of this source alone grants no semantic execution credit.
// The independent verifier must accept the binding/callgraph before crediting
// these assertions, and run the ordinary public cases alongside this suite.
pub fn run_suite(actual: ActualRun) {
    run_suite_with_order(actual, false);
}

pub fn run_suite_with_order(actual: ActualRun, reverse: bool) {
    let mut cases = [Case::OpaqueAtCeiling, Case::MixedDifferentAndOpaque,
        Case::QueryDifferentThenStable, Case::OriginalErrorPrecedence,
        Case::ProvenImpossibleTopologyTerminal, Case::ClosedIncreasing];
    if reverse { cases.reverse(); }
    for case in cases {
        let opaque = matches!(case, Case::OpaqueAtCeiling
            | Case::MixedDifferentAndOpaque | Case::OriginalErrorPrecedence);
        let input = Input {
            case,
            source_text: source(case),
            opaque_input: opaque.then(|| Value::Func(Func::element(
                "p1340_opaque", Arc::new(|_| panic!("opaque constructor must never run"))))),
            warning_names: ["P1340_GLOBAL", "P1340_RETAINED", "P1340_DISCOVERY",
                "P1340_REPLACED", "P1340_VALIDATION"],
        };
        let observed = actual(&input);
        assert_eq!(observed.source.text(), input.source_text);
        assert_eq!(observed.attempts, (1..=observed.attempts.len()).collect::<Vec<_>>());
        assert!(observed.attempts.len() <= 5);
        let messages: Vec<_> = observed.warnings.iter().map(|d| d.message.as_str()).collect();
        assert_eq!(messages.iter().filter(|&&m| m == "P1340_GLOBAL").count(), 1);
        assert!(!messages.iter().any(|m| ["P1340_DISCOVERY", "P1340_REPLACED",
            "P1340_VALIDATION"].contains(m)));
        assert!(observed.warnings.iter().all(|d| d.severity == Severity::Warning));
        for warning in observed.warnings.iter().filter(|d| input.warning_names.contains(&d.message.as_str())) {
            assert_eq!(warning.span, observed.source.root().span());
            assert!(warning.hints.is_empty());
            assert!(warning.trace.is_empty());
        }
        match case {
            Case::OpaqueAtCeiling | Case::MixedDifferentAndOpaque => {
                assert!(observed.selected_body_ok);
                assert_eq!(observed.attempts, vec![1, 2, 3, 4, 5]);
                terminal_error(observed.result.as_ref().unwrap_err(), observed.source.root().span());
                assert_eq!(observed.export_calls, 0);
                assert_eq!(messages, vec!["P1340_GLOBAL", "P1340_RETAINED"]);
            }
            Case::OriginalErrorPrecedence => {
                let errors = observed.result.as_ref().unwrap_err();
                assert!(!observed.original_errors.is_empty());
                assert!(observed.original_errors.iter().any(|d| d.message == "panicked with: P1340_ORIGINAL"));
                equal_diagnostics(errors, &observed.original_errors);
                assert!(!errors.iter().any(|d| d.message == TERMINAL));
                assert_eq!(observed.export_calls, 0);
                assert_eq!(messages, vec!["P1340_GLOBAL", "P1340_RETAINED"]);
            }
            Case::ProvenImpossibleTopologyTerminal => {
                assert!(observed.topology_impossibility);
                terminal_error(observed.result.as_ref().unwrap_err(), observed.source.root().span());
                assert_eq!(observed.export_calls, 0);
                assert_eq!(messages, vec!["P1340_GLOBAL"]);
            }
            Case::QueryDifferentThenStable => {
                assert!(observed.result.is_ok());
                assert!(observed.attempts.len() >= 2);
                assert_eq!(observed.metadata_integers, vec![1, 1]);
                assert_eq!(messages, vec!["P1340_GLOBAL", "P1340_RETAINED"]);
            }
            Case::ClosedIncreasing => {
                assert!(observed.result.is_ok());
                assert_eq!(observed.attempts, vec![1, 2, 3, 4, 5]);
                assert_eq!(observed.final_read_counter, Some(vec![4]));
                assert_eq!(observed.metadata_integers, vec![4]);
                assert!(!messages.contains(&TERMINAL));
                // Exact ordinary nonconvergence warnings are additionally
                // governed by the unchanged historical public oracle.
                assert_eq!(messages.iter().filter(|&&m| m == "P1340_RETAINED").count(), 1);
            }
        }
    }
}
