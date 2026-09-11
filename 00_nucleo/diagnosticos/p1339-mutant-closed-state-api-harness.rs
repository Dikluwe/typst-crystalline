// Mechanical test adapter only. Include as a #[cfg(test)] child of compiler::eval.
// Fixtures: p1339-ab-closed-api-fixtures-r1.json
// SHA256 dda8618c9ede3640fab05d6cd4278f6c766c3410e4a646561f4bd32436f20dd5
// No relation/retention/lifecycle implementation or private-comparator claim.
use super::*;
use crate::entities::compiler_features::{Feature, Features};
use crate::entities::font_book::FontBook;
use crate::entities::introspector::TagIntrospector;
use crate::entities::label::Label;
use crate::entities::layout_types::{Point, Pt};
use crate::entities::location::Location;
use crate::entities::locator::Locator;
use crate::entities::page_store::PageStore;
use crate::entities::position::Position;
use crate::entities::sealed_positions::SealedPositions;
use crate::entities::source_result::{Severity, Tracepoint};
use crate::entities::value::IntrospectedContent;
use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font};
use comemo::Track;
use serde_json::{Value as Json, json};
use std::collections::{BTreeMap, HashMap};
use std::num::{NonZeroU16, NonZeroUsize};

const FIXTURES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../00_nucleo/diagnosticos/p1339-ab-closed-api-fixtures-r1.json"
));

fn string<'a>(v: &'a Json, key: &str) -> &'a str {
    v[key].as_str().unwrap_or_else(|| panic!("missing string {key}"))
}

struct FixtureWorld {
    library: Library,
    book: FontBook,
    sources: BTreeMap<String, Source>,
    main: FileId,
}

impl FixtureWorld {
    fn new(case: &Json) -> Self {
        let mut texts = BTreeMap::<String, String>::new();
        texts.insert("setup".into(), string(case, "setup_code").into());
        texts.insert("body".into(), string(case, "body_code").into());
        for binding in case["value_bindings"].as_array().unwrap() {
            texts.insert(
                format!("binding/{}", string(binding, "name")),
                string(binding, "expression").into(),
            );
        }
        for snapshot in case["snapshots"].as_array().unwrap() {
            let id = string(snapshot, "id");
            if snapshot["origin"] == "content_expression" {
                texts.insert(
                    format!("snapshot/{id}/content"),
                    string(snapshot, "content_expression").into(),
                );
            }
            for (i, overlay) in
                snapshot["overlays"].as_array().into_iter().flatten().enumerate()
            {
                let key = format!("snapshot/{id}/overlay/{i}");
                if let Some(expr) = overlay["expression"].as_str() {
                    texts.insert(format!("{key}/value"), expr.into());
                }
                if let Some(expr) = overlay["content_expression"].as_str() {
                    texts.insert(format!("{key}/content"), expr.into());
                }
                if let Some(fields) = overlay["fields"].as_array() {
                    for (j, field) in fields.iter().enumerate() {
                        texts.insert(
                            format!("{key}/field/{j}"),
                            field[1].as_str().unwrap().into(),
                        );
                    }
                }
            }
        }
        let sources: BTreeMap<_, _> = texts
            .into_iter()
            .enumerate()
            .map(|(i, (key, text))| {
                let id = FileId::from_raw(
                    NonZeroU16::new(u16::try_from(i + 1).unwrap()).unwrap(),
                );
                (
                    key,
                    Source::new_with_parser(id, text, crate::compiler::parse::parse_code),
                )
            })
            .collect();
        let main = sources["body"].id();
        Self {
            library: Library::new(),
            book: FontBook::new(),
            sources,
            main,
        }
    }
}

impl World for FixtureWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.main
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        self.sources
            .values()
            .find(|s| s.id() == id)
            .cloned()
            .ok_or(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> {
        None
    }
}

fn with_engine<R>(
    world: &FixtureWorld,
    file: FileId,
    styles: &mut StyleChain,
    sink: &mut Sink,
    call: impl FnOnce(&mut Engine<'_>) -> R,
) -> R {
    let route = Route::root().with_id(file);
    let fixed = FixedMetrics;
    let mut rules: Arc<[ShowRule]> = Arc::from([]);
    let mut guards = Vec::new();
    let mut tracked = sink.track_mut();
    let mut engine = Engine {
        world,
        font_metrics: &fixed,
        route: route.track(),
        styles,
        show_rules: &mut rules,
        active_guards: &mut guards,
        current_file: file,
        sink: &mut tracked,
    };
    call(&mut engine)
}

fn eval_source(
    world: &FixtureWorld,
    key: &str,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    styles: &mut StyleChain,
    sink: &mut Sink,
) -> SourceResult<Value> {
    let source = world
        .sources
        .get(key)
        .unwrap_or_else(|| panic!("source absent {key}"));
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
    with_engine(world, source.id(), styles, sink, |engine| {
        let mut last = Value::None;
        for child in source.root().children() {
            if let Some(expr) = Expr::from_untyped(child) {
                last = eval_expr(expr, scopes, ctx, engine)?;
            }
        }
        Ok(last)
    })
}

fn require_content(v: Value) -> Content {
    match v {
        Value::Content(c) => c,
        other => panic!("fixture expected Content, got {}", other.type_name()),
    }
}

#[derive(Clone)]
struct InputSnapshot {
    intr: TagIntrospector,
    // Input construction only: preserves all explicit position overlays while cloning.
    positions: HashMap<Location, Position>,
}

fn location(
    key: &str,
    symbols: &HashMap<String, Location>,
    intr: &TagIntrospector,
) -> Location {
    if let Some(label) = key.strip_prefix("label:") {
        intr.labels
            .lookup(&Label(label.into()))
            .unwrap_or_else(|| panic!("label absent {label}"))
    } else {
        *symbols.get(key).unwrap_or_else(|| panic!("location absent {key}"))
    }
}

fn features(profile: &str) -> Features {
    let mut out = Features::default();
    match profile {
        "default" => {}
        "html" => out.enable(Feature::Html),
        "a11y" => out.enable(Feature::A11yExtras),
        "html+a11y" => {
            out.enable(Feature::Html);
            out.enable(Feature::A11yExtras);
        }
        _ => panic!("unknown profile"),
    }
    out
}

fn span_json(world: &FixtureWorld, span: Span) -> Json {
    let source = span
        .id()
        .and_then(|id| world.sources.iter().find(|(_, s)| s.id() == id));
    let range = source.and_then(|(_, s)| s.span_byte_range(span));
    json!({"raw":span.into_raw().get().to_string(),"detached":span.is_detached(),
        "source_key":source.map(|(k,_)|k),"file_id":span.id().map(|id|id.into_raw().get()),
        "byte_range":range.map(|r|vec![r.start,r.end]),
        "line_col":source.and_then(|(_,s)|s.span_to_line_col(span))})
}

fn diagnostics_json(world: &FixtureWorld, ds: &[SourceDiagnostic]) -> Json {
    Json::Array(ds.iter().map(|d| {
        let trace:Vec<_>=d.trace.iter().map(|t| {
            let point=match &t.v { Tracepoint::Call(v)=>json!({"Call":v}),
                Tracepoint::Show(v)=>json!({"Show":v}),Tracepoint::Import(v)=>json!({"Import":v}),
                Tracepoint::Include(v)=>json!({"Include":v}) };
            json!({"point":point,"span":span_json(world,t.span)})
        }).collect();
        json!({"severity":match d.severity {Severity::Error=>"Error",Severity::Warning=>"Warning"},
            "message":d.message,"span":span_json(world,d.span),"hints":d.hints,"trace":trace})
    }).collect())
}

fn data_json(v: &Value) -> Json {
    match v {
        Value::None => Json::Null,
        Value::Bool(x) => json!(x),
        Value::Int(x) => json!(x),
        Value::Float(x) => json!({"float_bits":format!("{:016x}",x.to_bits())}),
        Value::Str(x) => json!(x.as_str()),
        Value::Array(xs) => Json::Array(xs.iter().map(data_json).collect()),
        Value::Dict(xs) => {
            json!({"ordered_dict":xs.iter().map(|(k,v)|json!([k.as_str(),data_json(v)])).collect::<Vec<_>>()})
        }
        // Public result rendering only; this is never the observation comparator.
        other => {
            json!({"type":other.type_name(),"repr":repr_value_for_serialization(other)})
        }
    }
}

fn body_json(world: &FixtureWorld, result: &SourceResult<Value>) -> Json {
    match result {
        Ok(v) => {
            json!({"result":"Ok","type":v.type_name(),"value":data_json(v),"repr":repr_value_for_serialization(v)})
        }
        Err(ds) => json!({"result":"Err","diagnostics":diagnostics_json(world,ds)}),
    }
}

fn assert_predicates(case: &Json, observed: &Json) -> Vec<String> {
    let mut failures = Vec::new();
    let expected = &case["predicates"];
    let mut eq = |path: &str, a: &Json, b: &Json| {
        if a != b {
            failures.push(format!("{path}: actual={a} expected={b}"));
        }
    };
    eq("body.result", &observed["body"]["result"], &expected["body"]["result"]);
    for (key, value) in expected["body"].as_object().unwrap() {
        match key.as_str() {
            "result" => {}
            "type" | "value" => eq(key, &observed["body"][key], value),
            "diagnostic_message" => {
                eq(key, &observed["body"]["diagnostics"][0]["message"], value)
            }
            "span_must_be_present" => {
                let s = &observed["body"]["diagnostics"][0]["span"];
                eq(
                    key,
                    &json!(s["detached"] == false && s["byte_range"].is_array()),
                    value,
                );
            }
            _ => panic!("untranslated body predicate {key}"),
        }
    }
    eq(
        "has_filtered_counter_reads",
        &observed["has_filtered_counter_reads"],
        &expected["has_filtered_counter_reads"],
    );
    for (id, predicate) in expected["validation"].as_object().unwrap() {
        for (key, value) in predicate.as_object().unwrap() {
            assert!(
                matches!(key.as_str(), "result" | "value"),
                "untranslated validation predicate"
            );
            eq(
                &format!("validation.{id}.{key}"),
                &observed["validation"][id][key],
                value,
            );
        }
    }
    for (key, value) in expected["nonconvergence_diagnostics"].as_object().unwrap() {
        assert!(
            matches!(key.as_str(), "result" | "length"),
            "untranslated diagnosis predicate"
        );
        eq(
            &format!("nonconvergence_diagnostics.{key}"),
            &observed["nonconvergence_diagnostics"][key],
            value,
        );
    }
    failures
}

fn run_case(case: &Json, profile: &str, order: &str) -> (Json, Vec<String>) {
    let world = FixtureWorld::new(case);
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
    let observed = json!({"id":case["id"],"profile":profile,"order":order,"sources":sources,
        "body":body_json(&world,&result),"has_filtered_counter_reads":selected,"validation":validations,
        "nonconvergence_diagnostics":diagnosis,"construction_sink":diagnostics_json(&world,&construction_sink.into_diagnostics()),
        "body_sink":diagnostics_json(&world,&body_sink.into_diagnostics()),"validation_sinks":validation_sinks,
        "diagnosis_sink":diagnostics_json(&world,&diagnosis_sink.into_diagnostics())});
    let failures = assert_predicates(case, &observed);
    (observed, failures)
}

#[test]
fn p1339_closed_apis_frozen_matrix() {
    let fixture: Json = serde_json::from_str(FIXTURES).expect("invalid frozen JSON");
    let cases = fixture["cases"].as_array().unwrap();
    let mut failures = Vec::new();
    for order in ["normal", "repeat", "reverse"] {
        let mut indices: Vec<_> = (0..cases.len()).collect();
        if order == "reverse" {
            indices.reverse();
        }
        for profile in ["default", "html", "a11y", "html+a11y"] {
            for index in indices.iter().copied() {
                let case = &cases[index];
                assert!(
                    case["profiles"].as_array().unwrap().iter().any(|p| p == profile)
                );
                assert!(case["orders"].as_array().unwrap().iter().any(|p| p == order));
                let (observation, errors) = run_case(case, profile, order);
                println!(
                    "P1339_CLOSED_API {}",
                    serde_json::to_string(&observation).unwrap()
                );
                failures.extend(
                    errors
                        .into_iter()
                        .map(|e| format!("{} {profile} {order}: {e}", case["id"])),
                );
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
