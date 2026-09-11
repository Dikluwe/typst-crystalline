// Additional independent F05 fixtures. Mechanical API adapter, batch one.
// NOT_EXECUTED_PRESEAL. Include as child of frozen API harness wrapper.
use super::*;
use crate::entities::numbering::Numbering;

pub(super) trait ActualProjectionHooks {
    fn begin(&mut self, ctx: &mut EvalContext);
    fn phase(&mut self, ctx: &mut EvalContext, phase: &str);
    fn finish(&mut self, ctx: &EvalContext) -> Json;
}

fn run_projection_case(
    case: &Json,
    profile: &str,
    order: &str,
    hooks: &mut dyn ActualProjectionHooks,
) -> (Json, Vec<String>) {
    let mut world = FixtureWorld::new(case);
    let mut next_id = u16::try_from(world.sources.len() + 1).unwrap();
    for spec in case["snapshots"].as_array().unwrap() {
        for (i, overlay) in spec["overlays"].as_array().into_iter().flatten().enumerate()
        {
            if overlay["kind"] == "pages_raw" {
                for (j, expr) in
                    overlay["supplements"].as_array().unwrap().iter().enumerate()
                {
                    let id = FileId::from_raw(NonZeroU16::new(next_id).unwrap());
                    next_id += 1;
                    let key = format!(
                        "snapshot/{}/overlay/{i}/supplement/{j}",
                        string(spec, "id")
                    );
                    assert!(
                        world
                            .sources
                            .insert(
                                key,
                                Source::new_with_parser(
                                    id,
                                    expr.as_str().unwrap().into(),
                                    crate::compiler::parse::parse_code
                                )
                            )
                            .is_none()
                    );
                }
            }
        }
    }
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
    for binding in case["value_bindings"].as_array().unwrap() {
        let name = string(binding, "name");
        let value = eval_source(
            &world,
            &format!("binding/{name}"),
            &mut scopes,
            &mut construction,
            &mut construction_styles,
            &mut construction_sink,
        )
        .expect("fixture binding failed");
        scopes.define(name, value);
    }
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
                "pages_raw" => {
                    let total = NonZeroUsize::new(
                        overlay["total_pages"].as_u64().unwrap() as usize,
                    )
                    .unwrap();
                    let numberings = overlay["numberings"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|n| {
                            if n.is_null() {
                                None
                            } else {
                                Some(match string(n, "kind") {
                                    "Pattern" => {
                                        Numbering::Pattern(string(n, "value").into())
                                    }
                                    "Func" => match scopes
                                        .get(string(n, "binding"))
                                        .expect("missing numbering binding")
                                    {
                                        Value::Func(f) => Numbering::Func(f.clone()),
                                        _ => panic!("numbering binding is not a Func"),
                                    },
                                    other => {
                                        panic!("untranslated numbering variant {other}")
                                    }
                                })
                            }
                        })
                        .collect::<Vec<_>>();
                    let supplements = overlay["supplements"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .map(|(j, _)| {
                            require_content(
                                eval_source(
                                    &world,
                                    &format!("{key}/supplement/{j}"),
                                    &mut scopes,
                                    &mut construction,
                                    &mut construction_styles,
                                    &mut construction_sink,
                                )
                                .expect("supplement construction failed"),
                            )
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(numberings.len(), total.get());
                    assert_eq!(supplements.len(), total.get());
                    snap.intr.inject_pages(PageStore::from_runtime(
                        total,
                        numberings,
                        supplements,
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
    hooks.begin(&mut ctx);
    hooks.phase(&mut ctx, "body");
    let result = eval_source(
        &world,
        "body",
        &mut scopes,
        &mut ctx,
        &mut body_styles,
        &mut body_sink,
    );
    let selected = ctx.has_filtered_counter_reads();
    hooks.phase(&mut ctx, "validation");
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
    hooks.phase(&mut ctx, "diagnostics");
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
    observed["body"]["value_type"] = json!(match &result {
        Ok(Value::None) => "None",
        Ok(Value::Int(_)) => "Int",
        Ok(Value::Location(_)) => "Location",
        Ok(Value::Func(_)) => "Func",
        Ok(other) => other.type_name(),
        Err(_) => "Error",
    });
    observed["actual_effects"] = hooks.finish(&ctx);
    let mut failures = projection_predicates(case, &observed);
    for (key, expected) in
        case["actual_effect_predicates"].as_object().into_iter().flatten()
    {
        if key == "same_binding_returned_from_body" {
            let name = expected.as_str().unwrap();
            let same = match (&result, scopes.get(name)) {
                (Ok(Value::Func(a)), Some(Value::Func(b))) => Arc::ptr_eq(&a.0, &b.0),
                _ => false,
            };
            observed["body_binding_identity_check"] =
                json!({"binding":name,"actual_retained_allocation_equal":same});
            if !same {
                failures.push(
                    "body did not return the declared existing function binding".into(),
                );
            }
        } else if observed["actual_effects"].get(key) != Some(expected) {
            failures.push(format!(
                "actual callback/request effect {key}: missing or mismatch"
            ));
        }
    }
    (observed, failures)
}

fn projection_predicates(case: &Json, observed: &Json) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = &case["predicates"];
    for (key, value) in expected["body"].as_object().unwrap() {
        let actual = match key.as_str() {
            "result" | "value_type" | "value" => observed["body"][key].clone(),
            "message" => observed["body"]["diagnostics"][0]["message"].clone(),
            "span_required" => {
                let span = &observed["body"]["diagnostics"][0]["span"];
                json!(span["detached"] == false && span["byte_range"].is_array())
            }
            unknown => panic!("untranslated body predicate {unknown}"),
        };
        if actual != *value {
            failures.push(format!("body.{key}: actual={actual} expected={value}"));
        }
    }
    if observed["has_filtered_counter_reads"] != expected["has_filtered_counter_reads"] {
        failures.push("selection mismatch".into());
    }
    for (id, rules) in expected["validation"].as_object().unwrap() {
        for (key, value) in rules.as_object().unwrap() {
            assert!(
                matches!(key.as_str(), "result" | "value"),
                "untranslated validation predicate"
            );
            if observed["validation"][id][key] != *value {
                failures.push(format!("validation {id}.{key} mismatch"));
            }
        }
    }
    for (key, value) in expected["nonconvergence_diagnostics"].as_object().unwrap() {
        assert!(
            matches!(key.as_str(), "result" | "length"),
            "untranslated diagnosis predicate"
        );
        if observed["nonconvergence_diagnostics"][key] != *value {
            failures.push(format!("diagnosis {key} mismatch"));
        }
    }
    failures
}

pub(super) fn run_projection_matrix(
    mut new_hooks: impl FnMut() -> Box<dyn ActualProjectionHooks>,
) {
    let fixture: Json = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-ab-batch1-projection-fixtures-r2.json"
    )))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 8);
    let mut failures = Vec::new();
    for order in ["normal", "repeat", "reverse"] {
        let mut indices: Vec<_> = (0..cases.len()).collect();
        if order == "reverse" {
            indices.reverse();
        }
        for profile in ["default", "html", "a11y", "html+a11y"] {
            for &i in &indices {
                let mut hooks = new_hooks();
                let (row, errors) =
                    run_projection_case(&cases[i], profile, order, &mut *hooks);
                println!(
                    "P1339_EXTRA_PROJECTION {}",
                    serde_json::to_string(&row).unwrap()
                );
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
