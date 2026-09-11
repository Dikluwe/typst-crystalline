// Transport-only adapter. The canonical oracle below is included unchanged.
mod oracle {
    include!("/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/p1339-contract-array-owner-harness-r2.rs");
}
use crate::entities::{args::Args, file_id::FileId, source::Source, span::Span};
use serde_json::{json, Value as Json};

fn span_json(span: Span) -> Json {
    json!({"raw":span.into_raw().get(),"file_id":span.id().map(|id|id.into_raw().get()),
           "range":span.range().map(|r|vec![r.start,r.end]),"detached":span.is_detached()})
}
fn input_json(args: &Args) -> Json {
    json!({"span":span_json(args.span),"occurrences_present":args.occurrences.is_some(),
      "items":args.items.iter().map(|v|format!("{v:?}")).collect::<Vec<_>>(),
      "named":args.named.iter().map(|(k,v)|json!([k.as_str(),format!("{v:?}")])).collect::<Vec<_>>(),
      "occurrences":args.occurrence_sequence().iter().map(|o|json!({"name":o.name.as_deref(),
        "value":format!("{:?}",o.value),"span":span_json(o.span),"value_span":span_json(o.value_span)})).collect::<Vec<_>>()})
}
#[test]
fn p1339_array_owner_frozen() {
    let file = FileId::from_raw(std::num::NonZeroU16::new(49151).unwrap());
    let source = Source::new(file, " ".repeat(200));
    let callee = Span::from_range(file, 90..95);
    assert!(source.span_to_line_col(callee).is_some());
    let mut calls = 0;
    oracle::assert_array_owner(file, |args| {
        calls += 1;
        let input_before = input_json(args);
        let result = super::p1339_mutant_array::convert(args, callee);
        println!("P1339_ARRAY_OWNER_RAW {}", json!({
          "call_index":calls,"source_file_id":source.id().into_raw().get(),
          "source_text":source.text(),"callee_span":span_json(callee),
          "input_before":input_before,"input_after":input_json(args),
          "result_debug":format!("{result:?}")}));
        result
    });
    assert_eq!(calls, 10, "all canonical owner calls executed");
    println!("P1339_ARRAY_OWNER_COMPLETED {}", json!({"calls":calls}));
}
