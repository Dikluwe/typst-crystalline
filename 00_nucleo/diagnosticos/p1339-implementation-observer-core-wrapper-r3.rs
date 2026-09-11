// Include this file as cfg(test) child of compiler::eval. No productive API.
// One final test named p1339_frozen_core_bound_matrix must call run_bound_core.
// Its exact filter excludes the inherited standalone API test, avoiding duplicate rows.
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../00_nucleo/diagnosticos/p1339-mutant-closed-state-api-harness.rs"
));
mod relation_harness {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-mutant-closed-state-relation-harness.rs"
    ));
}
mod opaque_harness {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-mutant-closed-state-public-opaque-harness-r2.rs"
    ));
}
mod style_harness {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-mutant-closed-state-style-harness-r3.rs"
    ));
}
mod projection_harness {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-mutant-closed-state-projection-harness.rs"
    ));
}

pub(super) fn run_bound_core(
    actual: relation_harness::ActualRelation,
    operation_identity: &Json,
    style_factory: impl FnMut() -> Box<dyn style_harness::ActualStyleHooks>,
    projection_factory: impl FnMut() -> Box<dyn projection_harness::ActualProjectionHooks>,
) {
    // Existing independent API predicates are assertions, not printed labels.
    p1339_closed_apis_frozen_matrix();
    let pairs: Json = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-ab-private-relation-fixtures-r1.json"
    )))
    .unwrap();
    let cases = pairs["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 71);
    let ids: std::collections::BTreeSet<_> =
        cases.iter().map(|c| c["id"].as_str().unwrap()).collect();
    assert_eq!(ids.len(), 71);
    let report = relation_harness::run_relation_matrix(actual, operation_identity);
    assert_eq!(report.rows, 71 * 4 * 3, "complete pair/profile/order matrix");
    assert!(report.failures.is_empty(), "{}", report.failures.join("\n"));
    assert_eq!(
        report.outstanding_public_controls,
        vec!["relation-opaque-independent-callout"]
    );
    // This real control, plus the original independent Python all-channel
    // predicate in the external wrapper, discharges that separate public test.
    opaque_harness::run_public_opaque_matrix(actual, operation_identity);
    style_harness::run_style_matrix(style_factory);
    projection_harness::run_projection_matrix(projection_factory);
}
