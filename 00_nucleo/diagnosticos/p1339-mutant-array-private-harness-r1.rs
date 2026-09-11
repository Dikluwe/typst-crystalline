// Frozen private owner vectors from p1339-contract-array-private-bridge-r1.md.
// Include under cfg(test), bind only the real control/helper or productive owner.
// No runtime credit until actual binding/configuration is independently audited.
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::source::Source;
use crate::entities::source_result::{Severity, SourceDiagnostic, SourceResult, Tracepoint};
use crate::entities::span::Span;
use crate::entities::value::Value;
use serde_json::{json, Value as Json};
use std::num::NonZeroU16;

fn span_json(span: Span) -> Json {
    json!({"raw":span.into_raw().get(),"file_id":span.id().map(|id|id.into_raw().get()),
           "range":span.range().map(|r|vec![r.start,r.end]),"detached":span.is_detached()})
}

fn diagnostics_json(diagnostics: &[SourceDiagnostic]) -> Json {
    json!(diagnostics.iter().map(|d| {
        let trace: Vec<_> = d.trace.iter().map(|point| {
            let (kind,payload) = match &point.v {
                Tracepoint::Call(name) => ("Call",name.clone()),
                Tracepoint::Show(name) => ("Show",Some(name.clone())),
                Tracepoint::Import(name) => ("Import",Some(name.clone())),
                Tracepoint::Include(name) => ("Include",Some(name.clone())),
            };
            json!({"kind":kind,"payload":payload,"span":span_json(point.span)})
        }).collect();
        json!({"severity":format!("{:?}",d.severity),"message":d.message,
               "span":span_json(d.span),"hints":d.hints,"trace":trace})
    }).collect::<Vec<_>>())
}

fn input_json(args: &Args) -> Json {
    json!({"span":span_json(args.span),"occurrences_present":args.occurrences.is_some(),
           "items":args.items.iter().map(|v|format!("{v:?}")).collect::<Vec<_>>(),
           "named":args.named.iter().map(|(k,v)|json!([k.as_str(),format!("{v:?}")])).collect::<Vec<_>>(),
           "occurrences":args.occurrence_sequence().iter().map(|o|json!({"name":o.name.as_deref(),
              "value":format!("{:?}",o.value),"span":span_json(o.span),"value_span":span_json(o.value_span)})).collect::<Vec<_>>()})
}

pub(super) fn run_array_owner_vectors(
    mut actual: impl FnMut(&Args, Span) -> SourceResult<Value>,
) -> usize {
    let file = FileId::from_raw(NonZeroU16::new(49151).unwrap());
    let source = Source::new(file, " ".repeat(200));
    let range = |start,end| {
        let span = Span::from_range(source.id(),start..end);
        assert!(end <= source.len_bytes());
        assert!(source.span_to_line_col(span).is_some());
        span
    };
    let aggregate = range(100,140);
    let callee = range(90,95);
    let vectors = [
        ("ARR-U01-detached-positional",None,Span::detached(),range(122,123),"unexpected argument",aggregate),
        ("ARR-U02-known-positional",None,range(120,125),range(122,123),"unexpected argument",range(120,125)),
        ("ARR-U03-detached-named",Some("other"),Span::detached(),range(122,123),"unexpected argument: other",aggregate),
        ("ARR-U04-known-named",Some("other"),range(118,130),range(127,128),"unexpected argument: other",range(118,130)),
    ];
    let mut failures = Vec::new();
    let mut rows = 0;
    for (id,name,occurrence_span,value_span,message,expected_span) in vectors {
        let args = Args::from_occurrences(aggregate,vec![
            ArgOccurrence {name:None,value:Value::Bytes(Bytes::new(vec![0,255])),span:range(101,110),value_span:range(103,109)},
            ArgOccurrence {name:name.map(Into::into),value:Value::Int(9),span:occurrence_span,value_span},
        ]);
        let before = input_json(&args);
        let result = actual(&args,callee);
        let good = match &result {
            Err(ds) if ds.len()==1 => ds[0].severity==Severity::Error
                && ds[0].message==message && ds[0].span==expected_span,
            _ => false,
        };
        let observed = match &result {
            Ok(value) => json!({"kind":"Ok","value_debug":format!("{value:?}")}),
            Err(ds) => json!({"kind":"Err","diagnostics":diagnostics_json(ds)}),
        };
        println!("P1339_ARRAY_OWNER {}",json!({"id":id,"source_file_id":source.id().into_raw().get(),
            "source_text":source.text(),"input_before":before,"input_after":input_json(&args),
            "callee_span":span_json(callee),"result":observed,"full_result_debug":format!("{result:?}")}));
        rows += 1;
        if !good { failures.push(id); }
    }
    // Positive owner-reachability control explicitly required by the oracle bridge.
    // From_parts is suitable here: neither input item claims a known origin.
    let args = Args::from_parts(vec![Value::Bytes(Bytes::new(vec![0,255]))],Default::default(),aggregate);
    let before = input_json(&args);
    let result = actual(&args,callee);
    let good = matches!(&result,Ok(Value::Array(values)) if values==&vec![Value::Int(0),Value::Int(255)]);
    println!("P1339_ARRAY_OWNER {}",json!({"id":"ARR-UP01-reachable-bytes","source_file_id":source.id().into_raw().get(),
        "source_text":source.text(),"input_before":before,"input_after":input_json(&args),
        "callee_span":span_json(callee),"full_result_debug":format!("{result:?}")}));
    rows += 1;
    if !good { failures.push("ARR-UP01-reachable-bytes"); }
    assert_eq!(rows,5,"complete frozen private vector inventory");
    assert!(failures.is_empty(),"actual owner vector mismatches: {failures:?}");
    rows
}
