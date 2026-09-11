// Batch-one passive actual-pipeline collector; include under cfg(test) in L3 pipeline.
// Cargo success is transport only. The frozen external wrapper must run the
// independent predicate-v2 over all144 raw observations and require success.
use serde_json::{Value as Json, json};

pub(super) struct ActualPipelineInput<'a> {
    pub(super) source_text: &'a str,
    pub(super) source_name: &'a str,
    pub(super) profile: &'a str,
    pub(super) target: &'a str,
    pub(super) context_coordinates: &'a Json,
    pub(super) projection_requests: &'a Json,
    pub(super) resource_construction: &'a Json,
}

// Late binding may construct the real immutable memory World and attach passive
// hooks, then invoke the actual compile_to_paged_document_full_error_and_features
// (or its actual public facade). It cannot implement a cycle, decide retention,
// replace FallbackFontMetrics, receive predicates, or manufacture DTO events.
pub(super) type ActualPaginatedRun = fn(&ActualPipelineInput<'_>) -> Json;

pub(super) fn collect_lifecycle_matrix(actual: ActualPaginatedRun) {
    let fixtures: Json = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-ab-batch1-retention-lifecycle-typed-r2.json"
    )))
    .unwrap();
    let cases = fixtures["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 12);
    let ids: std::collections::BTreeSet<_> =
        cases.iter().map(|c| c["id"].as_str().unwrap()).collect();
    assert_eq!(ids.len(), cases.len(), "frozen case identity uniqueness");
    let mut count = 0usize;
    let mut failures = Vec::new();
    for order in ["normal", "repeat", "reverse"] {
        let mut indices: Vec<_> = (0..cases.len()).collect();
        if order == "reverse" {
            indices.reverse();
        }
        for profile in ["default", "html", "a11y", "html+a11y"] {
            for &index in &indices {
                let case = &cases[index];
                let execution = &case["execution"];
                assert!(
                    execution["profiles"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|p| p == profile)
                );
                assert!(
                    execution["orders"].as_array().unwrap().iter().any(|p| p == order)
                );
                let input = ActualPipelineInput {
                    source_text: case["source"].as_str().unwrap(),
                    source_name: execution["named_source_id"].as_str().unwrap(),
                    profile,
                    target: execution["target"].as_str().unwrap(),
                    context_coordinates: &case["context_coordinates"],
                    projection_requests: &case["explicit_projections"],
                    resource_construction: execution,
                };
                let mut observation = actual(&input);
                assert!(
                    observation.is_object(),
                    "missing actual pipeline observation object"
                );
                // These are only input coordinates. No execution or identity
                // field is filled from a fixture or an expected output.
                observation["case_id"] = case["id"].clone();
                observation["source_sha256"] = case["source_sha256"].clone();
                observation["profile"] = json!(profile);
                observation["order"] = json!(order);
                println!(
                    "P1339_LIFECYCLE {}",
                    serde_json::to_string(&observation).unwrap()
                );
                count += 1;
                if observation["execution"] != "Observed" {
                    failures.push(format!(
                        "{} {profile} {order}: actual operation not Observed",
                        case["id"]
                    ));
                }
                if !observation["events"].as_array().is_some_and(|xs| !xs.is_empty()) {
                    failures.push(format!("{}: actual transcript absent", case["id"]));
                }
            }
        }
    }
    println!(
        "P1339_LIFECYCLE_COLLECTOR {}",
        json!({"rows":count,"status":"COLLECTION_ONLY_NOT_PASS","independent_checker_required":true})
    );
    assert_eq!(count, 144);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
