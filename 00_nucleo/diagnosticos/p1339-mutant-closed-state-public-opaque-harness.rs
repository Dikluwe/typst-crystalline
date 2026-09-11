// Batch-one mechanical public opaque adapter; NOT_EXECUTED_PRESEAL.
// Include as a child beside relation_harness inside the frozen API-harness wrapper.
use super::relation_harness::{ActualDiscriminant, ActualRelation};
use super::*;
use crate::entities::elements::test_callout::CalloutElem;

fn run_opaque_case(
    case: &Json,
    profile: &str,
    order: &str,
    actual: ActualRelation,
    identity: &Json,
) -> (Json, Vec<String>) {
    let mut source_spec = case.clone();
    assert!(case["value_bindings"].as_array().unwrap().is_empty());
    source_spec["value_bindings"] = Json::Array(
        case["binding_operations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|op| op["kind"] == "eval_binding")
            .map(|op| json!({"name":op["output_binding"],"expression":op["expression"]}))
            .collect(),
    );
    let world = FixtureWorld::new(&source_spec);
    let mut scopes = Scopes::new(None);
    let stdlib = make_stdlib_with_features(&world.inputs(), features(profile));
    scopes.define("std", Value::Module(Module::new("global", stdlib.clone())));
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    let mut construction = EvalContext::new();
    construction.features = features(profile);
    let mut construction_styles = StyleChain::default_chain();
    let mut construction_sink = Sink::new();
    eval_source(
        &world,
        "setup",
        &mut scopes,
        &mut construction,
        &mut construction_styles,
        &mut construction_sink,
    )
    .expect("fixture setup failed");
    let mut constructed = BTreeMap::<String, Value>::new();
    for binding in case["binding_operations"].as_array().unwrap() {
        let name = string(binding, "output_binding");
        let value = match string(binding, "kind") {
            "eval_binding" => eval_source(
                &world,
                &format!("binding/{name}"),
                &mut scopes,
                &mut construction,
                &mut construction_styles,
                &mut construction_sink,
            )
            .expect("fixture binding failed"),
            "dynamic_callout" => Value::Content(Content::dynamic(CalloutElem::new(
                require_content(constructed[string(binding, "body_binding")].clone()),
                string(binding, "title"),
                string(binding, "tone"),
            ))),
            other => panic!("untranslated opaque binding {other}"),
        };
        scopes.define(name, value.clone());
        assert!(constructed.insert(name.into(), value).is_none());
    }
    let pair: Json = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-ab-private-relation-fixtures-r1.json"
    )))
    .unwrap();
    let linked = pair["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == case["private_pair_link"]["case_id"])
        .expect("missing private pair");
    assert_eq!(
        linked["operations"], case["binding_operations"],
        "opaque pair construction drift"
    );
    let private_discriminant = match actual(&constructed["left"], &constructed["right"]) {
        ActualDiscriminant::Same => "Same",
        ActualDiscriminant::Different => "Different",
        ActualDiscriminant::Unproven => "Unproven",
    };
    let mut locator = Locator::new();
    let symbols: HashMap<_, _> = case["location_symbols"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| (s.as_str().unwrap().to_string(), locator.next()))
        .collect();
    let mut snapshots = BTreeMap::<String, InputSnapshot>::new();
    for spec in case["snapshots"].as_array().unwrap() {
        let id = string(spec, "id");
        let mut snap = match string(spec, "origin") {
            "empty" => InputSnapshot {
                intr: TagIntrospector::empty(),
                positions: HashMap::new(),
            },
            "clone_of" => snapshots
                .get(string(spec, "clone_of"))
                .expect("snapshot clone absent")
                .clone(),
            "content_expression" => {
                let content = require_content(
                    eval_source(
                        &world,
                        &format!("snapshot/{id}/content"),
                        &mut scopes,
                        &mut construction,
                        &mut construction_styles,
                        &mut construction_sink,
                    )
                    .expect("snapshot content failed"),
                );
                let intr = match string(spec, "introspection_entrypoint") {
                    "pure" => crate::compiler::introspect::introspect_with_introspector(
                        &content,
                    ),
                    "runtime" => with_engine(
                        &world,
                        world.main(),
                        &mut construction_styles,
                        &mut construction_sink,
                        |engine| {
                            crate::compiler::introspect::introspect_with_runtime(
                                &content,
                                engine,
                                &mut construction,
                            )
                        },
                    )
                    .expect("runtime snapshot failed"),
                    _ => panic!("unknown introspection entrypoint"),
                };
                InputSnapshot { intr, positions: HashMap::new() }
            }
            _ => panic!("unknown snapshot origin"),
        };
        for (i, overlay) in spec["overlays"].as_array().into_iter().flatten().enumerate()
        {
            let key = format!("snapshot/{id}/overlay/{i}");
            match string(overlay, "kind") {
                "state_init" | "state_update" => {
                    let value = eval_source(
                        &world,
                        &format!("{key}/value"),
                        &mut scopes,
                        &mut construction,
                        &mut construction_styles,
                        &mut construction_sink,
                    )
                    .expect("state input failed");
                    let loc = location(string(overlay, "location"), &symbols, &snap.intr);
                    if overlay["kind"] == "state_init" {
                        snap.intr.state.init(string(overlay, "key").into(), value, loc);
                    } else {
                        snap.intr.state.update(string(overlay, "key").into(), value, loc);
                    }
                }
                "label" => {
                    let loc = location(string(overlay, "location"), &symbols, &snap.intr);
                    snap.intr.labels.add(Label(string(overlay, "label").into()), loc);
                }
                "element" => {
                    let loc = location(string(overlay, "location"), &symbols, &snap.intr);
                    let content = require_content(
                        eval_source(
                            &world,
                            &format!("{key}/content"),
                            &mut scopes,
                            &mut construction,
                            &mut construction_styles,
                            &mut construction_sink,
                        )
                        .expect("element input failed"),
                    );
                    let fields = if overlay["fields"].is_null() {
                        None
                    } else {
                        let mut fields = indexmap::IndexMap::with_hasher(
                            rustc_hash::FxBuildHasher::default(),
                        );
                        for (j, field) in
                            overlay["fields"].as_array().unwrap().iter().enumerate()
                        {
                            let value = eval_source(
                                &world,
                                &format!("{key}/field/{j}"),
                                &mut scopes,
                                &mut construction,
                                &mut construction_styles,
                                &mut construction_sink,
                            )
                            .expect("field input failed");
                            fields.insert(
                                ecow::EcoString::from(field[0].as_str().unwrap()),
                                value,
                            );
                        }
                        Some(fields)
                    };
                    snap.intr
                        .elements
                        .insert(loc, IntrospectedContent::new(content, fields));
                }
                "position" => {
                    let loc = location(string(overlay, "location"), &symbols, &snap.intr);
                    snap.positions.insert(
                        loc,
                        Position {
                            page: NonZeroUsize::new(
                                overlay["page"].as_u64().unwrap() as usize
                            )
                            .unwrap(),
                            point: Point {
                                x: Pt(overlay["x_pt"].as_f64().unwrap()),
                                y: Pt(overlay["y_pt"].as_f64().unwrap()),
                            },
                        },
                    );
                    snap.intr.inject_positions(SealedPositions::from_runtime(
                        snap.positions.clone(),
                    ));
                }
                "pages" => snap.intr.inject_pages(PageStore::from_total_pages(
                    NonZeroUsize::new(overlay["total_pages"].as_u64().unwrap() as usize)
                        .unwrap(),
                )),
                _ => panic!("untranslated overlay"),
            }
        }
        assert!(snapshots.insert(id.into(), snap).is_none(), "duplicate snapshot id");
    }
    let mut ctx = EvalContext::new();
    ctx.features = features(profile);
    assert_eq!(string(case, "target"), "paged");
    ctx.target = EvalTarget::Paged;
    ctx.in_context = case["in_context"].as_bool().unwrap();
    ctx.introspector = snapshots[string(case, "body_snapshot")].intr.clone();
    ctx.current_location =
        Some(location(string(case, "current_location"), &symbols, &ctx.introspector));
    let mut body_styles = StyleChain::default_chain();
    let mut body_sink = Sink::new();
    let result = eval_source(
        &world,
        "body",
        &mut scopes,
        &mut ctx,
        &mut body_styles,
        &mut body_sink,
    );
    let selected = ctx.has_filtered_counter_reads();
    let mut validations = serde_json::Map::new();
    let mut validation_sinks = serde_json::Map::new();
    for id in case["validate_candidate_ids"].as_array().unwrap() {
        let id = id.as_str().unwrap();
        let mut styles = body_styles.clone();
        let mut sink = Sink::new();
        let value = with_engine(&world, world.main(), &mut styles, &mut sink, |engine| {
            ctx.context_reads_valid_for(&snapshots[id].intr, engine)
        });
        validations.insert(
            id.into(),
            match value {
                Ok(v) => json!({"result":"Ok","value":v}),
                Err(ds) => {
                    json!({"result":"Err","diagnostics":diagnostics_json(&world,&ds)})
                }
            },
        );
        validation_sinks
            .insert(id.into(), diagnostics_json(&world, &sink.into_diagnostics()));
    }
    let history: Vec<_> = case["history_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| snapshots[id.as_str().unwrap()].intr.clone())
        .collect();
    let mut diagnosis_styles = body_styles.clone();
    let mut diagnosis_sink = Sink::new();
    let diagnosis = with_engine(
        &world,
        world.main(),
        &mut diagnosis_styles,
        &mut diagnosis_sink,
        |engine| ctx.context_nonconvergence_diagnostics(&history, engine),
    );
    let diagnosis = match diagnosis {
        Ok(ds) => {
            json!({"result":"Ok","length":ds.len(),"diagnostics":diagnostics_json(&world,&ds)})
        }
        Err(ds) => json!({"result":"Err","diagnostics":diagnostics_json(&world,&ds)}),
    };
    let sources:Vec<_>=world.sources.iter().map(|(key,s)|json!({"key":key,"file_id":s.id().into_raw().get(),"text":s.text()})).collect();
    let mut observed = json!({"id":case["id"],"profile":profile,"order":order,"sources":sources,
        "body":body_json(&world,&result),"has_filtered_counter_reads":selected,"validation":validations,
        "nonconvergence_diagnostics":diagnosis,"construction_sink":diagnostics_json(&world,&construction_sink.into_diagnostics()),
        "body_sink":diagnostics_json(&world,&body_sink.into_diagnostics()),"validation_sinks":validation_sinks,
        "diagnosis_sink":diagnostics_json(&world,&diagnosis_sink.into_diagnostics())});
    observed["private_discriminant"] = json!(private_discriminant);
    observed["actual_operation_identity"] = identity.clone();
    observed["same_bindings_private_and_public"] = json!({"construction":"each operation evaluated once; same retained Values bound in scopes and passed by reference to actual relation", "operations":case["binding_operations"]});
    observed["body"]["value_type"] = json!(match &result {
        Ok(Value::Array(_)) => "Array",
        Ok(v) => v.type_name(),
        Err(_) => "Error",
    });
    observed["body"]["length"] = match &result {
        Ok(Value::Array(xs)) => json!(xs.len()),
        _ => Json::Null,
    };
    let mut failures = opaque_predicates(case, &observed);
    if observed["private_discriminant"] != linked["expected"]["discriminant"] {
        failures.push(format!(
            "private relation actual={} expected={}",
            observed["private_discriminant"], linked["expected"]["discriminant"]
        ));
    }
    (observed, failures)
}

fn opaque_predicates(case: &Json, observed: &Json) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = &case["predicates"];
    for (key, value) in expected["body"].as_object().unwrap() {
        assert!(
            matches!(key.as_str(), "result" | "value_type" | "length"),
            "untranslated body predicate"
        );
        if &observed["body"][key] != value {
            failures.push(format!(
                "body.{key}: actual={} expected={value}",
                observed["body"][key]
            ));
        }
    }
    if observed["has_filtered_counter_reads"] != expected["has_filtered_counter_reads"] {
        failures.push("selection mismatch".into());
    }
    for (id, rule) in expected["validation"].as_object().unwrap() {
        let row = &observed["validation"][id];
        let name = match (row["result"].as_str(), row["value"].as_bool()) {
            (Some("Ok"), Some(false)) => "Ok(false)",
            (Some("Ok"), Some(true)) => "Ok(true)",
            (Some("Err"), _)
                if row["diagnostics"].as_array().is_some_and(|ds| !ds.is_empty()) =>
            {
                "Err(SourceDiagnostic)"
            }
            _ => "Unknown",
        };
        if !rule["allowed_result_variants"]
            .as_array()
            .unwrap()
            .iter()
            .any(|x| x == name)
        {
            failures.push(format!("validation {id}: forbidden {name}"));
        }
    }
    let row = &observed["nonconvergence_diagnostics"];
    let name = match row["result"].as_str() {
        Some("Ok")
            if row["length"] == 0
                && row["diagnostics"].as_array().is_some_and(Vec::is_empty) =>
        {
            "Ok(empty)"
        }
        Some("Err") if row["diagnostics"].as_array().is_some_and(|ds| !ds.is_empty()) => {
            "Err(SourceDiagnostic)"
        }
        _ => "Unknown",
    };
    if !expected["nonconvergence_diagnostics"]["allowed_result_variants"]
        .as_array()
        .unwrap()
        .iter()
        .any(|x| x == name)
    {
        failures.push(format!("diagnosis forbidden {name}"));
    }
    failures
}

pub(super) fn run_public_opaque_matrix(actual: ActualRelation, identity: &Json) {
    let fixture: Json = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-ab-batch1-public-opaque-fixture.json"
    )))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    let mut failures = Vec::new();
    for order in ["normal", "repeat", "reverse"] {
        let mut indices: Vec<_> = (0..cases.len()).collect();
        if order == "reverse" {
            indices.reverse();
        }
        for profile in ["default", "html", "a11y", "html+a11y"] {
            for &i in &indices {
                let (row, errors) =
                    run_opaque_case(&cases[i], profile, order, actual, identity);
                println!("P1339_PUBLIC_OPAQUE {}", serde_json::to_string(&row).unwrap());
                failures.extend(
                    errors
                        .into_iter()
                        .map(|e| format!("{} {profile} {order}: {e}", cases[i]["id"])),
                );
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
