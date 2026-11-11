//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/cli/semantic_json_tests.md
//! @prompt-hash 3bc09b9b
//! @layer L2
//! @updated 2026-11-10

use super::*;
use typst_core::entities::content::Content;
use typst_core::entities::value::IntrospectedContent;

fn obj(fields: Vec<(&str, SemanticValue)>) -> SemanticValue {
    SemanticValue::Object(fields.into_iter().map(|(k, v)| (k.into(), v)).collect())
}

fn legacy_bytes(value: &SemanticValue, pretty: bool) -> Vec<u8> {
    let tree = semantic_to_json(value);
    let mut bytes = if pretty {
        serde_json::to_vec_pretty(&tree).unwrap()
    } else {
        serde_json::to_vec(&tree).unwrap()
    };
    bytes.push(b'\n');
    bytes
}

fn all_variants() -> SemanticValue {
    SemanticValue::Array(vec![
        SemanticValue::Null,
        SemanticValue::Bool(false),
        SemanticValue::Bool(true),
        SemanticValue::Int(i64::MIN),
        SemanticValue::Int(i64::MAX),
        SemanticValue::Int(9_007_199_254_740_993),
        SemanticValue::Float(-0.0),
        SemanticValue::Float(0.0),
        SemanticValue::Float(1.2345678901234567),
        SemanticValue::Float(f64::MIN_POSITIVE),
        SemanticValue::Float(f64::from_bits(1)),
        SemanticValue::Float(f64::MAX),
        SemanticValue::Float(f64::MIN),
        SemanticValue::Float(f64::NAN),
        SemanticValue::Float(f64::INFINITY),
        SemanticValue::Float(f64::NEG_INFINITY),
        SemanticValue::String("é🦀\"\\\n\t\r\u{0000}\u{0008}\u{000c}\u{001f}".into()),
        SemanticValue::String(String::new()),
        SemanticValue::Array(vec![]),
        SemanticValue::Object(vec![]),
        obj(vec![
            ("z", SemanticValue::Int(1)),
            ("a", SemanticValue::Int(2)),
            ("z", SemanticValue::Int(3)),
            (
                "m",
                obj(vec![
                    ("q", SemanticValue::Bool(false)),
                    ("p", SemanticValue::Null),
                    ("q", SemanticValue::Bool(true)),
                ]),
            ),
        ]),
    ])
}

#[test]
fn independent_compact_numeric_and_escaping_golden() {
    let v = SemanticValue::Array(vec![
        SemanticValue::Null,
        SemanticValue::Bool(true),
        SemanticValue::Bool(false),
        SemanticValue::Int(i64::MIN),
        SemanticValue::Int(i64::MAX),
        SemanticValue::Int(9_007_199_254_740_993),
        SemanticValue::Float(-0.0),
        SemanticValue::Float(1.2345678901234567),
        SemanticValue::Float(f64::NAN),
        SemanticValue::Float(f64::INFINITY),
        SemanticValue::Float(f64::NEG_INFINITY),
        SemanticValue::String("é🦀\"\\\n\t\r\0\u{0008}\u{000c}\u{001f}".into()),
        SemanticValue::Array(vec![]),
        SemanticValue::Object(vec![]),
    ]);
    assert_eq!(serialize_semantic(&v, true, false).unwrap(),
        "[null,true,false,-9223372036854775808,9223372036854775807,9007199254740993,-0.0,1.2345678901234567,null,null,null,\"é🦀\\\"\\\\\\n\\t\\r\\u0000\\b\\f\\u001f\",[],{}]\n".as_bytes());
}

#[test]
fn independent_nonadjacent_duplicate_order_and_pretty_golden() {
    let v = obj(vec![
        ("z", SemanticValue::Int(1)),
        ("a", SemanticValue::Int(2)),
        ("z", SemanticValue::Int(9)),
        (
            "nested",
            obj(vec![
                ("b", SemanticValue::Null),
                ("a", SemanticValue::Int(4)),
                ("b", SemanticValue::Bool(true)),
            ]),
        ),
    ]);
    assert_eq!(
        serialize_semantic(&v, true, false).unwrap(),
        b"{\"z\":9,\"a\":2,\"nested\":{\"b\":true,\"a\":4}}\n"
    );
    assert_eq!(serialize_semantic(&v, true, true).unwrap(), b"{\n  \"z\": 9,\n  \"a\": 2,\n  \"nested\": {\n    \"b\": true,\n    \"a\": 4\n  }\n}\n");
    // YAML continues to expose the original projection, including repeated members.
    assert_eq!(
        serialize_semantic(&v, false, true).unwrap(),
        b"z: 1\na: 2\nz: 9\nnested:\n  b: null\n  a: 4\n  b: true\n"
    );
}

#[test]
fn complete_legacy_oracle_precision_and_nested_shapes() {
    let mut cases = vec![all_variants()];
    for order in 0..3 {
        let mut fields = Vec::new();
        for j in 0..769 {
            let i = match order {
                0 => j,
                1 => 768 - j,
                _ => (j * 197) % 769,
            };
            fields.push((format!("key-{i:04}"), SemanticValue::Int(i as i64)));
        }
        fields.push(("key-0017".into(), SemanticValue::String("last".into())));
        fields.push(("key-0000".into(), SemanticValue::Array(vec![all_variants()])));
        cases.push(SemanticValue::Object(fields));
    }
    let mut deep = SemanticValue::String("leaf".into());
    for i in 0..40 {
        deep = if i % 2 == 0 {
            SemanticValue::Array(vec![deep])
        } else {
            obj(vec![
                ("b", SemanticValue::Null),
                ("a", deep),
                ("b", SemanticValue::Int(i)),
            ])
        };
    }
    cases.push(deep);
    for value in &cases {
        for pretty in [false, true] {
            assert_eq!(
                serialize_semantic(value, true, pretty).unwrap(),
                legacy_bytes(value, pretty)
            );
        }
    }
}

#[test]
fn public_snapshots_field_one_yaml_and_raw_contracts() {
    use typst_core::entities::locator::Locator;
    let make = |fields| {
        Value::LocatedContent(
            IntrospectedContent::new(Content::raw("fallback", None, false), fields),
            Locator::new().next(),
        )
    };
    let labeled = Value::Content(Content::label_auto("outside", Content::Empty));
    assert_eq!(
        serialize_eval(&labeled, EvalFormat::Json, false).unwrap(),
        b"{\"func\":\"sequence\",\"children\":[],\"label\":\"<outside>\"}\n"
    );
    let absent = make(None);
    let empty = make(Some(Default::default()));
    let known = make(Some(
        [
            ("text".into(), Value::Str("snapshot\n\"é".into())),
            ("payload".into(), Value::Array(vec![Value::None, Value::Int(i64::MAX)])),
        ]
        .into_iter()
        .collect(),
    ));
    assert_eq!(
        serialize_eval(&absent, EvalFormat::Json, false).unwrap(),
        b"{\"func\":\"raw\",\"text\":\"fallback\",\"block\":false}\n"
    );
    assert_eq!(
        serialize_eval(&empty, EvalFormat::Json, false).unwrap(),
        b"{\"func\":\"raw\"}\n"
    );
    assert_eq!(serialize_eval(&known, EvalFormat::Json, false).unwrap(), "{\"func\":\"raw\",\"text\":\"snapshot\\n\\\"é\",\"payload\":[null,9223372036854775807]}\n".as_bytes());
    assert_eq!(serialize_eval(&empty, EvalFormat::Yaml, true).unwrap(), b"func: raw\n");
    assert_eq!(
        serialize_query(std::slice::from_ref(&known), Some("text"), true, false).unwrap(),
        "\"snapshot\\n\\\"é\"\n".as_bytes()
    );
    assert_eq!(
        serialize_query(&[empty.clone(), known.clone()], Some("text"), false, false)
            .unwrap(),
        "[\"snapshot\\n\\\"é\"]\n".as_bytes()
    );
    assert_eq!(
        serialize_query(&[empty.clone(), known.clone()], Some("missing"), true, false)
            .unwrap_err(),
        "expected exactly one element, found 2"
    );
    assert_eq!(
        serialize_query(&[], Some("missing"), true, false).unwrap_err(),
        "expected exactly one element, found 0"
    );
    assert_eq!(
        serialize_query(&[empty], Some("text"), true, false).unwrap_err(),
        "no such field found for element"
    );
    assert_eq!(
        serialize_query_with_format(
            &[known],
            Some("text"),
            true,
            true,
            QueryFormat::Yaml
        )
        .unwrap(),
        "\"snapshot\\n\\\"é\"\n".as_bytes()
    );
    assert_eq!(
        serialize_eval(&Value::Str("raw\0\né".into()), EvalFormat::Raw, true).unwrap(),
        "raw\0\né".as_bytes()
    );
    let bytes = Value::Bytes(typst_core::entities::bytes::Bytes::new(vec![0, 255, 10]));
    assert_eq!(serialize_eval(&bytes, EvalFormat::Raw, false).unwrap(), [0, 255, 10]);
    assert_eq!(serialize_eval(&Value::Int(1), EvalFormat::Raw, false).unwrap_err(), "cannot print int in raw format\nhint: `--format=raw` only supports strings and bytes");
}

#[test]
fn borrowed_adapter_matches_complete_legacy_oracle() {
    let v = all_variants();
    for pretty in [false, true] {
        let mut expected = legacy_bytes(&v, pretty);
        expected.pop();
        let actual = if pretty {
            serde_json::to_vec_pretty(&SemanticJson(&v)).unwrap()
        } else {
            serde_json::to_vec(&SemanticJson(&v)).unwrap()
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn recursive_writer_errors_preserve_prefix_and_do_not_retry() {
    use std::io::{self, Write};
    struct Failing {
        limit: usize,
        used: usize,
        failed: bool,
        retries: usize,
        bytes: [u8; 4096],
    }
    impl Write for Failing {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            if self.failed {
                self.retries += 1;
                return Err(io::Error::other("retry"));
            }
            if self.used == self.limit {
                self.failed = true;
                return Err(io::Error::other("nested writer sentinel"));
            }
            let n = b.len().min(self.limit - self.used);
            self.bytes[self.used..self.used + n].copy_from_slice(&b[..n]);
            self.used += n;
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let value = obj(vec![(
        "outer",
        SemanticValue::Array(vec![
            all_variants(),
            obj(vec![("k", SemanticValue::String("long escaped \n payload".into()))]),
        ]),
    )]);
    for pretty in [false, true] {
        let mut encoded = legacy_bytes(&value, pretty);
        encoded.pop();
        assert!(encoded.len() < 4096);
        for limit in 0..encoded.len() {
            let mut writer = Failing {
                limit,
                used: 0,
                failed: false,
                retries: 0,
                bytes: [0; 4096],
            };
            let error = if pretty {
                serde_json::to_writer_pretty(&mut writer, &SemanticJson(&value))
            } else {
                serde_json::to_writer(&mut writer, &SemanticJson(&value))
            }
            .unwrap_err();
            assert!(error.is_io(), "pretty={pretty}, limit={limit}: {error}");
            assert_eq!(error.io_error_kind(), Some(io::ErrorKind::Other));
            assert_eq!(error.to_string(), "nested writer sentinel");
            assert!(writer.failed);
            assert_eq!(writer.used, limit, "pretty={pretty}");
            assert_eq!(
                &writer.bytes[..writer.used],
                &encoded[..limit],
                "pretty={pretty}"
            );
            assert_eq!(writer.retries, 0, "pretty={pretty}, limit={limit}");
        }
    }
}

#[test]
fn emitted_bytes_remain_owned_after_input_destruction() {
    for pretty in [false, true] {
        let (emitted, expected, direct, evaluated, queried) = {
            let value = obj(vec![
                ("z", SemanticValue::String("owned é\n".into())),
                ("a", SemanticValue::Array(vec![SemanticValue::Int(i64::MAX)])),
                ("z", SemanticValue::String("last 🦀".into())),
            ]);
            let projected = Value::Str("caller-owned é\n".into());
            let direct = if pretty {
                serde_json::to_vec_pretty(&SemanticJson(&value)).unwrap()
            } else {
                serde_json::to_vec(&SemanticJson(&value)).unwrap()
            };
            (
                serialize_semantic(&value, true, pretty).unwrap(),
                legacy_bytes(&value, pretty),
                direct,
                serialize_eval(&projected, EvalFormat::Json, pretty).unwrap(),
                serialize_query(&[projected], None, true, pretty).unwrap(),
            )
        };
        assert_eq!(emitted, expected);
        assert_eq!(direct, expected[..expected.len() - 1]);
        assert_eq!(evaluated, "\"caller-owned é\\n\"\n".as_bytes());
        assert_eq!(queried, evaluated);
    }
}
