// Public query serializer controls. The owning L0 specifies insertion order,
// JSON/YAML escaping and newline, field filtering, and cardinality before field
// selection. No alternate serializer or performance assertion is introduced.
use typst_core::entities::bytes::Bytes;
use typst_core::entities::value::Value;
use typst_shell::cli::{serialize_query_with_format, QueryFormat};

fn dict(entries: &[(&str, Value)]) -> Value {
    let mut result = Value::Dict(Default::default());
    let Value::Dict(ref mut map) = result else { unreachable!() };
    for (key, value) in entries {
        map.insert((*key).into(), value.clone());
    }
    result
}

fn output(
    elements: &[Value],
    field: Option<&str>,
    one: bool,
    pretty: bool,
    format: QueryFormat,
) -> String {
    let bytes =
        serialize_query_with_format(elements, field, one, pretty, format).unwrap();
    assert!(bytes.ends_with(b"\n"), "structured output must end with a newline");
    String::from_utf8(bytes).unwrap()
}

fn ordered_nested() -> Value {
    dict(&[
        ("zeta", Value::Str("ação α".into())),
        ("alpha", Value::Str("quote\" slash\\ newline\n tab\t".into())),
        (
            "list",
            Value::Array(vec![Value::None, Value::Bool(true), Value::Int(-7), dict(&[])]),
        ),
    ])
}

#[test]
fn compact_json_preserves_unicode_escaping_nested_values_and_insertion_order() {
    assert_eq!(output(&[ordered_nested()], None, false, false, QueryFormat::Json),
        "[{\"zeta\":\"ação α\",\"alpha\":\"quote\\\" slash\\\\ newline\\n tab\\t\",\"list\":[null,true,-7,{}]}]\n");
}

#[test]
fn pretty_json_has_two_space_indentation_and_one_trailing_newline() {
    let value = dict(&[
        ("z", Value::Int(2)),
        ("a", Value::Array(vec![Value::Bool(false), Value::None])),
    ]);
    assert_eq!(
        output(&[value], None, false, true, QueryFormat::Json),
        "[\n  {\n    \"z\": 2,\n    \"a\": [\n      false,\n      null\n    ]\n  }\n]\n"
    );
}

#[test]
fn yaml_preserves_order_unicode_escaping_and_two_space_nested_indentation() {
    let expected = "-\n  zeta: \"ação α\"\n  alpha: \"quote\\\" slash\\\\ newline\\n tab\\t\"\n  list:\n    - null\n    - true\n    - -7\n    - {}\n";
    assert_eq!(
        output(&[ordered_nested()], None, false, false, QueryFormat::Yaml),
        expected
    );
    assert_eq!(
        output(&[ordered_nested()], None, false, true, QueryFormat::Yaml),
        expected,
        "pretty does not change YAML output"
    );
}

#[test]
fn yaml_quotes_scalar_like_strings_and_preserves_empty_containers() {
    let value = Value::Array(vec![
        Value::Str("true".into()),
        Value::Str("false".into()),
        Value::Str("null".into()),
        Value::Str("123".into()),
        Value::Str("-word".into()),
        Value::Str("a_b".into()),
        Value::Str("".into()),
        Value::Array(vec![]),
        dict(&[]),
    ]);
    assert_eq!(output(&[value], None, true, false, QueryFormat::Yaml),
        "- \"true\"\n- \"false\"\n- \"null\"\n- \"123\"\n- \"-word\"\n- a_b\n- \"\"\n- []\n- {}\n");
}

#[test]
fn quoted_dictionary_keys_are_escaped_without_sorting() {
    let value = dict(&[
        ("z:key", Value::Int(1)),
        ("á\"key", Value::Int(2)),
        ("a", Value::Int(3)),
    ]);
    assert_eq!(
        output(&[value.clone()], None, true, false, QueryFormat::Json),
        "{\"z:key\":1,\"á\\\"key\":2,\"a\":3}\n"
    );
    assert_eq!(
        output(&[value], None, true, false, QueryFormat::Yaml),
        "\"z:key\": 1\n\"á\\\"key\": 2\na: 3\n"
    );
}

#[test]
fn missing_fields_filter_out_but_present_null_remains_in_original_order() {
    let values = vec![
        dict(&[("pick", Value::Int(1))]),
        Value::Int(9),
        dict(&[("other", Value::None)]),
        dict(&[("pick", Value::None)]),
        dict(&[("pick", Value::Str("último".into()))]),
    ];
    assert_eq!(
        output(&values, Some("pick"), false, false, QueryFormat::Json),
        "[1,null,\"último\"]\n"
    );
    assert_eq!(
        output(&values, Some("pick"), false, false, QueryFormat::Yaml),
        "- 1\n- null\n- \"último\"\n"
    );
}

#[test]
fn one_selects_single_value_before_field_and_reports_verbatim_errors() {
    for format in [QueryFormat::Json, QueryFormat::Yaml] {
        for (elements, expected) in [
            (vec![], "expected exactly one element, found 0"),
            (
                vec![dict(&[("pick", Value::Int(7))]), dict(&[("other", Value::None)])],
                "expected exactly one element, found 2",
            ),
        ] {
            assert_eq!(
                serialize_query_with_format(&elements, Some("pick"), true, false, format)
                    .unwrap_err(),
                expected
            );
        }
        for value in [dict(&[("other", Value::Int(3))]), Value::Int(9)] {
            assert_eq!(
                serialize_query_with_format(&[value], Some("pick"), true, false, format)
                    .unwrap_err(),
                "no such field found for element"
            );
        }
        assert_eq!(
            output(
                &[dict(&[("pick", Value::Int(7))])],
                Some("pick"),
                true,
                false,
                format
            ),
            "7\n"
        );
        assert_eq!(
            output(&[dict(&[("pick", Value::None)])], Some("pick"), true, false, format),
            "null\n"
        );
    }
}

#[test]
fn no_results_and_all_missing_fields_emit_empty_array_with_newline() {
    for format in [QueryFormat::Json, QueryFormat::Yaml] {
        assert_eq!(output(&[], None, false, false, format), "[]\n");
        assert_eq!(
            output(&[dict(&[]), Value::None], Some("missing"), false, true, format),
            "[]\n"
        );
        assert_eq!(output(&[dict(&[])], None, true, false, format), "{}\n");
    }
}

#[test]
fn nonfinite_floats_and_bytes_follow_public_projection() {
    let values = vec![
        Value::Float(f64::NAN),
        Value::Float(f64::INFINITY),
        Value::Float(f64::NEG_INFINITY),
        Value::Float(1.25),
        Value::Bytes(Bytes::new(vec![0, 1, 255])),
    ];
    assert_eq!(
        output(&values, None, false, false, QueryFormat::Json),
        "[null,null,null,1.25,\"bytes(3)\"]\n"
    );
    assert_eq!(
        output(&values, None, false, false, QueryFormat::Yaml),
        "- .nan\n- .inf\n- -.inf\n- 1.25\n- \"bytes(3)\"\n"
    );
}
