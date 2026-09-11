// Batch-one direct same-EvalContext/same-Engine input adapter; no pipeline loop.
// Descendant of the frozen API harness. Successor r3: associated-call correction for r2 E0599 only.
// Same logical Engine resources; short-lived reborrow views, no runtime credit.
use super::*;
use crate::entities::style_chain::StyleDelta;

pub(super) trait ActualStyleHooks {
    // Attach passive recording to the actual record/capture/replay operations.
    // No fixture JSON, expected chain, or desired request identity is supplied.
    fn begin(&mut self, ctx: &mut EvalContext, engine: &mut Engine<'_>);
    fn finish(&mut self, ctx: &EvalContext, engine: &Engine<'_>) -> Json;
}

fn eval_existing_engine(
    world: &FixtureWorld,
    key: &str,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let source = &world.sources[key];
    engine.current_file = source.id();
    if source.root().erroneous() {
        return Err(source
            .root()
            .errors()
            .into_iter()
            .map(|e| {
                let mut d = SourceDiagnostic::error(e.span, e.message.to_string());
                d.hints = e.hints.iter().map(|h| h.to_string()).collect();
                d
            })
            .collect());
    }
    let mut value = Value::None;
    for child in source.root().children() {
        if let Some(expr) = Expr::from_untyped(child) {
            value = eval_expr(expr, scopes, ctx, engine)?;
        }
    }
    Ok(value)
}

fn fixture_result(world: &FixtureWorld, result: &SourceResult<Value>) -> Json {
    match result {
        Ok(v) => json!({"kind":"Ok","value":data_json(v)}),
        Err(ds) => json!({"kind":"Err","diagnostics":diagnostics_json(world,ds)}),
    }
}

fn path<'a>(value: &'a Json, path: &str) -> &'a Json {
    path.split('.').fold(value, |v, k| {
        v.get(k).unwrap_or_else(|| panic!("missing DTO field {path}"))
    })
}

fn predicates(case: &Json, row: &Json) -> Vec<String> {
    let mut failures = Vec::new();
    for rule in case["typed_predicates"].as_array().unwrap() {
        let good = match string(rule, "op") {
            "equal" => path(row, string(rule, "path")) == &rule["expected"],
            "length_equal" => {
                json!(path(row, string(rule, "path")).as_array().unwrap().len())
                    == rule["expected"]
            }
            "project_equal" => {
                let fields: Vec<_> = path(row, string(rule, "path"))
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| path(v, string(rule, "field")).clone())
                    .collect();
                json!(fields) == rule["expected"]
            }
            "pointwise_identity_equal" => {
                let left = path(row, string(rule, "left")).as_array().unwrap();
                let right = path(row, string(rule, "right")).as_array().unwrap();
                left.len() == right.len()
                    && left.iter().zip(right).all(|(a, b)| {
                        rule["fields"].as_array().unwrap().iter().all(|key| {
                            let key = key.as_str().unwrap();
                            let av = path(a, key);
                            let bv = path(b, key);
                            av.as_str().is_some_and(|s| !s.is_empty()) && av == bv
                        })
                    })
            }
            "all_same_identity" => {
                let mut identities = Vec::new();
                for field in rule["paths"].as_array().unwrap() {
                    let (array, key) =
                        field.as_str().unwrap().split_once("[*].").unwrap();
                    for value in path(row, array).as_array().unwrap() {
                        identities.push(
                            path(value, key).as_str().expect("identity must be string"),
                        );
                    }
                }
                !identities.is_empty()
                    && !identities[0].is_empty()
                    && identities.iter().all(|x| *x == identities[0])
            }
            op => panic!("untranslated typed style predicate {op}"),
        };
        if !good {
            failures.push(format!("typed style predicate failed: {rule}"));
        }
    }
    failures
}

fn run_style_case(
    case: &Json,
    profile: &str,
    order: &str,
    hooks: &mut dyn ActualStyleHooks,
) -> (Json, Vec<String>) {
    let mut world = FixtureWorld::new(
        &json!({"setup_code":case["setup"]["setup_code"],"body_code":"","value_bindings":[],"snapshots":[]}),
    );
    let mut next_id = 3u16;
    for op in case["operations"].as_array().unwrap() {
        if op["op"] == "eval_expr" {
            let id = FileId::from_raw(NonZeroU16::new(next_id).unwrap());
            next_id += 1;
            assert!(
                world
                    .sources
                    .insert(
                        string(op, "source_id").into(),
                        Source::new_with_parser(
                            id,
                            string(op, "source").into(),
                            crate::compiler::parse::parse_code
                        )
                    )
                    .is_none()
            );
        }
    }
    let mut scopes = Scopes::new(None);
    let stdlib = make_stdlib_with_features(&world.inputs(), features(profile));
    scopes.define("std", Value::Module(Module::new("global", stdlib.clone())));
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    let mut ctx = EvalContext::new();
    ctx.in_context = true;
    ctx.target = EvalTarget::Paged;
    ctx.features = features(profile);
    let snapshot = TagIntrospector::empty();
    ctx.introspector = snapshot.clone();
    ctx.current_location = Some(Locator::new().next());
    let mut product_sink = Sink::new();
    let mut validation_sink = Sink::new();
    let mut styles = StyleChain::default_chain();
    let route = Route::root().with_id(world.main());
    let metrics = FixedMetrics;
    let mut rules: Arc<[ShowRule]> = Arc::from([]);
    let mut guards = Vec::new();
    let mut operation_rows = Vec::new();
    let mut operation_failures = Vec::new();
    let mut row;
    {
        let mut product_tracked = product_sink.track_mut();
        let mut validation_tracked = validation_sink.track_mut();
        // Engine is a short-lived borrow view; all logical resources and ctx
        // remain the same. Reborrow tracked sinks to bound each view's lifetime.
        let mut current_file = {
            let mut setup_tracked = comemo::TrackedMut::reborrow_mut(&mut product_tracked);
            let mut engine = Engine {
                world: &world,
                font_metrics: &metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut rules,
                active_guards: &mut guards,
                current_file: world.main(),
                sink: &mut setup_tracked,
            };
            eval_existing_engine(&world, "setup", &mut scopes, &mut ctx, &mut engine)
                .expect("real style setup failed");
            hooks.begin(&mut ctx, &mut engine);
            engine.current_file
        };
        let mut validation_phase = false;
        let mut selection = Json::Null;
        let mut validation = Json::Null;
        for op in case["operations"].as_array().unwrap() {
            if op["op"] == "call_actual_context_reads_valid_for" {
                validation_phase = true;
            }
            let mut operation_tracked = if validation_phase {
                comemo::TrackedMut::reborrow_mut(&mut validation_tracked)
            } else {
                comemo::TrackedMut::reborrow_mut(&mut product_tracked)
            };
            let mut engine = Engine {
                world: &world,
                font_metrics: &metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut rules,
                active_guards: &mut guards,
                current_file,
                sink: &mut operation_tracked,
            };
            match string(op, "op") {
                "set_engine_chain" => {
                    let value = string(op, "construction")
                        .strip_prefix("default_chain.push(StyleDelta { size: Some(")
                        .unwrap()
                        .strip_suffix("), ..StyleDelta::empty() })")
                        .unwrap()
                        .parse::<f64>()
                        .unwrap();
                    *engine.styles = StyleChain::default_chain()
                        .push(StyleDelta { size: Some(value), ..StyleDelta::empty() });
                    operation_rows.push(json!({"op":op["op"],"chain_id_input_label_only":op["chain_id"],"actual_size_bits":format!("{:016x}",engine.styles.size().to_bits())}));
                }
                "eval_expr" => {
                    assert_eq!(op["same_ctx"], true);
                    let result = eval_existing_engine(
                        &world,
                        string(op, "source_id"),
                        &mut scopes,
                        &mut ctx,
                        &mut engine,
                    );
                    let observed = fixture_result(&world, &result);
                    operation_rows.push(json!({"op":op["op"],"source_id":op["source_id"],"result":observed}));
                    if observed != op["expected_result"] {
                        operation_failures.push(format!(
                            "input evaluation result mismatch: {}",
                            op["source_id"]
                        ));
                    }
                }
                "call_actual_has_filtered_counter_reads" => {
                    selection = json!(ctx.has_filtered_counter_reads());
                    if selection != op["expected"] {
                        operation_failures.push("actual selection mismatch".into());
                    }
                }
                "call_actual_context_reads_valid_for" => {
                    assert_eq!(op["same_ctx"], true);
                    assert_eq!(op["candidate"], "clone of same actual empty snapshot");
                    validation = match ctx
                        .context_reads_valid_for(&snapshot.clone(), &mut engine)
                    {
                        Ok(v) => json!({"kind":"Ok","value":v}),
                        Err(ds) => {
                            json!({"kind":"Err","diagnostics":diagnostics_json(&world,&ds)})
                        }
                    };
                    if validation != op["expected_result"] {
                        operation_failures.push("actual validation mismatch".into());
                    }
                }
                unknown => panic!("untranslated operation {unknown}"),
            }
            current_file = engine.current_file;
        }
        let mut finish_tracked = if validation_phase {
            comemo::TrackedMut::reborrow_mut(&mut validation_tracked)
        } else {
            comemo::TrackedMut::reborrow_mut(&mut product_tracked)
        };
        let engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file,
            sink: &mut finish_tracked,
        };
        row = hooks.finish(&ctx, &engine);
        assert!(row.is_object(), "actual style hook DTO missing");
        row["selection"] = selection;
        row["validation"] = validation;
        let actual_final_bits = json!(format!("{:016x}", engine.styles.size().to_bits()));
        assert_eq!(
            row["engine_chain_at_validation"]["chain_size_pt_bits"], actual_final_bits,
            "hook must report actual engine chain"
        );
    }
    row["case_id"] = case["id"].clone();
    row["profile"] = json!(profile);
    row["order"] = json!(order);
    row["operation_results"] = json!(operation_rows);
    row["product_sink"] = diagnostics_json(&world, &product_sink.into_diagnostics());
    row["validation_sink"] =
        diagnostics_json(&world, &validation_sink.into_diagnostics());
    operation_failures.extend(predicates(case, &row));
    (row, operation_failures)
}

pub(super) fn run_style_matrix(mut new_hooks: impl FnMut() -> Box<dyn ActualStyleHooks>) {
    let fixture: Json = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../00_nucleo/diagnosticos/p1339-ab-batch1-same-context-style-fixture.json"
    )))
    .unwrap();
    let mut failures = Vec::new();
    for order in ["normal", "repeat", "reverse"] {
        for profile in ["default", "html", "a11y", "html+a11y"] {
            let mut hooks = new_hooks();
            let (row, errors) = run_style_case(&fixture, profile, order, &mut *hooks);
            println!("P1339_SAME_CONTEXT_STYLE {}", serde_json::to_string(&row).unwrap());
            failures
                .extend(errors.into_iter().map(|e| format!("{profile} {order}: {e}")));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
