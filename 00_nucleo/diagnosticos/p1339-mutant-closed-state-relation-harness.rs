// Batch-one mechanical preparation. NOT_EXECUTED_PRESEAL; no runtime credit.
// Mechanical child of the frozen API harness, which is itself an eval child.
// No private comparator implementation; the actual productive function is late-bound.
use super::*;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::elements::test_callout::CalloutElem;
use crate::entities::syntax_node::SyntaxNode;

const RELATION_FIXTURES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../00_nucleo/diagnosticos/p1339-ab-private-relation-fixtures-r1.json"
));

#[derive(Debug, Clone, Copy)]
pub(super) enum ActualDiscriminant {
    Same,
    Different,
    Unproven,
}

impl ActualDiscriminant {
    fn name(self) -> &'static str {
        match self {
            Self::Same => "Same",
            Self::Different => "Different",
            Self::Unproven => "Unproven",
        }
    }
}

// This port may only translate the discriminant returned by the real operation
// used in ordinary context validation. It cannot calculate another relation.
pub(super) type ActualRelation = fn(&Value, &Value) -> ActualDiscriminant;

fn variant(v: &Value) -> &'static str {
    match v {
        Value::None => "None",
        Value::Bool(_) => "Bool",
        Value::Int(_) => "Int",
        Value::Float(_) => "Float",
        Value::Str(_) => "Str",
        Value::Array(_) => "Array",
        Value::Dict(_) => "Dict",
        Value::Module(_) => "Module",
        Value::Datetime(_) => "Datetime",
        Value::Func(_) => "Func",
        Value::Content(_) => "Content",
        Value::LocatedContent(..) => "LocatedContent",
        Value::Auto => "Auto",
        Value::Length(_) => "Length",
        Value::Relative(_) => "Relative",
        Value::Ratio(_) => "Ratio",
        Value::Angle(_) => "Angle",
        Value::Color(_) => "Color",
        Value::Stroke(_) => "Stroke",
        Value::Fraction(_) => "Fraction",
        Value::Align(_) => "Align",
        Value::Location(_) => "Location",
        Value::Gradient(_) => "Gradient",
        Value::Regex(_) => "Regex",
        Value::Tiling(_) => "Tiling",
        Value::Bytes(_) => "Bytes",
        Value::Decimal(_) => "Decimal",
        Value::Duration(_) => "Duration",
        Value::Version(_) => "Version",
        Value::Selector(_) => "Selector",
        Value::Symbol(_) => "Symbol",
        Value::Args(_) => "Args",
        Value::State(_) => "State",
        Value::Counter(_) => "Counter",
        Value::Label(_) => "Label",
        Value::Dir(_) => "Dir",
        Value::Path(_) => "Path",
        Value::Type(_) => "Type",
    }
}

fn source_expressions(op: &Json, prefix: &str, texts: &mut BTreeMap<String, String>) {
    let mut put = |suffix: &str, text: &str| {
        assert!(texts.insert(format!("{prefix}/{suffix}"), text.into()).is_none());
    };
    match string(op, "kind") {
        "eval_binding" => put("expression", string(op, "expression")),
        "args_literal" => {
            for (i, expr) in op["items"].as_array().unwrap().iter().enumerate() {
                put(&format!("item/{i}"), expr.as_str().unwrap());
            }
            for (i, field) in op["named"].as_array().unwrap().iter().enumerate() {
                put(&format!("named/{i}"), field[1].as_str().unwrap());
            }
            for (i, occurrence) in
                op["occurrences"].as_array().into_iter().flatten().enumerate()
            {
                put(&format!("occurrence/{i}"), string(occurrence, "value"));
            }
        }
        "located_content" => {
            for (i, field) in op["fields"].as_array().into_iter().flatten().enumerate() {
                put(&format!("field/{i}"), field[1].as_str().unwrap());
            }
        }
        "source" | "clone_binding" | "float_bits" | "location_symbol"
        | "dynamic_callout" => {}
        kind => panic!("untranslated operation {kind}"),
    }
}

fn relation_world(case: &Json) -> FixtureWorld {
    let empty =
        json!({"setup_code":"", "body_code":"", "value_bindings":[], "snapshots":[]});
    let mut world = FixtureWorld::new(&empty);
    let mut texts = BTreeMap::new();
    for (i, op) in case["operations"].as_array().unwrap().iter().enumerate() {
        source_expressions(op, &format!("operation/{i}"), &mut texts);
        if op["kind"] == "source" {
            assert!(
                texts
                    .insert(
                        format!("origin/{}", string(op, "id")),
                        string(op, "text").into()
                    )
                    .is_none()
            );
        }
    }
    for (i, (key, text)) in texts.into_iter().enumerate() {
        let id =
            FileId::from_raw(NonZeroU16::new(u16::try_from(i + 3).unwrap()).unwrap());
        assert!(
            world
                .sources
                .insert(
                    key,
                    Source::new_with_parser(id, text, crate::compiler::parse::parse_code)
                )
                .is_none()
        );
    }
    world
}

fn source_span(world: &FixtureWorld, input: &str) -> Span {
    if input == "detached" {
        return Span::detached();
    }
    let reference = input.strip_prefix("source:").expect("untranslated span");
    let (key, range) = reference.rsplit_once(':').unwrap();
    let (first, last) = range.split_once("..").unwrap();
    let range = first.parse::<usize>().unwrap()..last.parse::<usize>().unwrap();
    let source = &world.sources[&format!("origin/{key}")];
    fn visit(
        node: &SyntaxNode,
        source: &Source,
        range: &std::ops::Range<usize>,
        found: &mut Vec<Span>,
    ) {
        if source.span_byte_range(node.span()).as_ref() == Some(range) {
            found.push(node.span());
        }
        for child in node.children() {
            visit(child, source, range, found);
        }
    }
    let mut found = Vec::new();
    visit(source.root(), source, &range, &mut found);
    assert_eq!(found.len(), 1, "missing/ambiguous actual parsed source span: {input}");
    found[0]
}

fn observe_relation(
    case: &Json,
    profile: &str,
    actual: ActualRelation,
    identity: &Json,
) -> Json {
    let world = relation_world(case);
    let mut scopes = Scopes::new(None);
    let stdlib = make_stdlib_with_features(&world.inputs(), features(profile));
    scopes.define("std", Value::Module(Module::new("global", stdlib.clone())));
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    // Same additional global bindings as the baseline public expression entry.
    for (name, value) in crate::compiler::stdlib::predefined_color_bindings() {
        scopes.define(name.as_str(), value);
    }
    scopes.define(
        "text",
        Value::Func(Func::native("text", crate::compiler::stdlib::native_text)),
    );
    let mut ctx = EvalContext::new();
    ctx.features = features(profile);
    let mut styles = StyleChain::default_chain();
    let mut sink = Sink::new();
    let mut locator = Locator::new();
    let mut locations = BTreeMap::<String, Location>::new();
    let mut values = BTreeMap::<String, Value>::new();
    let mut events = Vec::<Json>::new();
    for (i, op) in case["operations"].as_array().unwrap().iter().enumerate() {
        let prefix = format!("operation/{i}");
        let mut evaluate = |suffix: &str| {
            eval_source(
                &world,
                &format!("{prefix}/{suffix}"),
                &mut scopes,
                &mut ctx,
                &mut styles,
                &mut sink,
            )
            .unwrap_or_else(|errors| {
                panic!(
                    "real input evaluation failed: {}",
                    diagnostics_json(&world, &errors)
                )
            })
        };
        let value = match string(op, "kind") {
            "source" => {
                let source = &world.sources[&format!("origin/{}", string(op, "id"))];
                events.push(json!({"operation":i,"kind":"source","file_id":format!("{:?}",source.id()),"text":source.text()}));
                continue;
            }
            "location_symbol" => {
                let token = locator.next();
                assert!(locations.insert(string(op, "symbol").into(), token).is_none());
                events.push(json!({"operation":i,"kind":"location_symbol","actual_location":token.as_u128().to_string()}));
                continue;
            }
            "eval_binding" => evaluate("expression"),
            "clone_binding" => values[string(op, "input_binding")].clone(),
            "float_bits" => Value::Float(f64::from_bits(
                u64::from_str_radix(string(op, "u64_hex"), 16).unwrap(),
            )),
            "args_literal" => {
                let items = op["items"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(j, _)| evaluate(&format!("item/{j}")))
                    .collect();
                let mut named =
                    indexmap::IndexMap::with_hasher(rustc_hash::FxBuildHasher);
                for (j, field) in op["named"].as_array().unwrap().iter().enumerate() {
                    assert!(
                        named
                            .insert(
                                field[0].as_str().unwrap().into(),
                                evaluate(&format!("named/{j}"))
                            )
                            .is_none()
                    );
                }
                let occurrences = op["occurrences"].as_array().map(|rows| {
                    rows.iter()
                        .enumerate()
                        .map(|(j, row)| ArgOccurrence {
                            name: row["name"].as_str().map(Into::into),
                            value: evaluate(&format!("occurrence/{j}")),
                            span: source_span(&world, string(row, "span")),
                            value_span: source_span(&world, string(row, "value_span")),
                        })
                        .collect()
                });
                Value::Args(Args {
                    items,
                    named,
                    span: source_span(&world, string(op, "span")),
                    occurrences,
                })
            }
            "located_content" => {
                let body = require_content(values[string(op, "content_binding")].clone());
                let fields = op["fields"].as_array().map(|rows| {
                    let mut fields =
                        indexmap::IndexMap::with_hasher(rustc_hash::FxBuildHasher);
                    for (j, field) in rows.iter().enumerate() {
                        assert!(
                            fields
                                .insert(
                                    field[0].as_str().unwrap().into(),
                                    evaluate(&format!("field/{j}"))
                                )
                                .is_none()
                        );
                    }
                    fields
                });
                Value::LocatedContent(
                    IntrospectedContent::new(body, fields),
                    locations[string(op, "location")],
                )
            }
            "dynamic_callout" => Value::Content(Content::dynamic(CalloutElem::new(
                require_content(values[string(op, "body_binding")].clone()),
                string(op, "title"),
                string(op, "tone"),
            ))),
            kind => panic!("untranslated operation {kind}"),
        };
        let output = string(op, "output_binding");
        let function_allocation = match &value {
            Value::Func(f) => Some(format!("{:p}", Arc::as_ptr(&f.0))),
            _ => None,
        };
        events.push(json!({"operation":i,"kind":op["kind"],"output_binding":output,"input_binding":op["input_binding"],
            "actual_variant":variant(&value),"function_allocation_observed_only":function_allocation,
            "identity_limit":"Construction event and optional real allocation observation; neither computes nor proves Same."}));
        scopes.define(output, value.clone());
        assert!(values.insert(output.into(), value).is_none());
    }
    let left = &values[string(case, "left_binding")];
    let right = &values[string(case, "right_binding")];
    let discriminant = actual(left, right);
    json!({"case":case["id"],"profile":profile,"actual_discriminant":discriminant.name(),
        "left_variant":variant(left),"right_variant":variant(right),"binding_provenance":events,
        "actual_operation_identity":identity,"construction_diagnostics":diagnostics_json(&world,&sink.into_diagnostics()),
        "public_noncertification":"PENDING independent concrete public fixture; not executed and no credit"})
}

pub(super) struct PrivatePairReport {
    pub(super) rows: usize,
    pub(super) failures: Vec<String>,
    pub(super) outstanding_public_controls: Vec<String>,
}

// Frozen callable entrypoint, not a self-passing test. The final owner-local
// #[test] must supply the real operation and independently audited identity.
// Private pair success cannot close public opaque non-certification.
pub(super) fn run_relation_matrix(
    actual: ActualRelation,
    identity: &Json,
) -> PrivatePairReport {
    for key in ["source_sha256", "binary_sha256", "config_sha256"] {
        let value = string(identity, key);
        assert!(
            value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit()),
            "missing operation pin {key}"
        );
    }
    assert!(!string(identity, "source_path").is_empty());
    assert!(identity["source_line"].as_u64().unwrap() > 0);
    let fixture: Json =
        serde_json::from_str(RELATION_FIXTURES).expect("invalid frozen pair fixture");
    let cases = fixture["cases"].as_array().unwrap();
    let mut report = PrivatePairReport {
        rows: 0,
        failures: Vec::new(),
        outstanding_public_controls: Vec::new(),
    };
    for case in cases {
        if case["expected"]["public_noncertification_required"] == true {
            report.outstanding_public_controls.push(string(case, "id").into());
        }
    }
    for order in ["normal", "repeat", "reverse"] {
        let mut indices: Vec<_> = (0..cases.len()).collect();
        if order == "reverse" {
            indices.reverse();
        }
        for profile in ["default", "html", "a11y", "html+a11y"] {
            for &index in &indices {
                let case = &cases[index];
                let mut row = observe_relation(case, profile, actual, identity);
                row["order"] = json!(order);
                println!(
                    "P1339_PRIVATE_RELATION {}",
                    serde_json::to_string(&row).unwrap()
                );
                report.rows += 1;
                if row["actual_discriminant"] != case["expected"]["discriminant"] {
                    report.failures.push(format!(
                        "{} {profile} {order}: actual {} expected {}",
                        case["id"],
                        row["actual_discriminant"],
                        case["expected"]["discriminant"]
                    ));
                }
            }
        }
    }
    println!(
        "P1339_PRIVATE_RELATION_SUMMARY {}",
        json!({"scope":"private pairs only", "rows":report.rows,
        "failures":report.failures,"outstanding_public_controls":report.outstanding_public_controls,
        "closure":"NOT_A_COMPLETE_F06_VERDICT"})
    );
    report
}
