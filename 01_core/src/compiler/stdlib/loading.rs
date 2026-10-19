//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading.md
//! @prompt-hash b241a6aa
//! @layer L1
//! @updated 2026-06-21
//!
//! Módulo `loading` — carregamento de dados (`read`/`csv`/`json`/`yaml`/`toml`/
//! `cbor`/`xml`). Achado da Lista B do Passo 386; materializado no Passo 387.
//!
//! Arquitetura (ADR-0029 + ADR-0111): o **decode é L1 puro** — `decode_X(bytes,
//! opts) -> SourceResult<Value>`, zero I/O, testável com bytes literais. A
//! **leitura** reusa o L3 já existente `World::read_bytes`. As `native_X`
//! compõem os dois estratos. Paridade é com o `Value` de saída (ADR-0107), não
//! com a mecânica do parser.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

mod cbor;
mod csv;
mod encoding;
mod json;
mod json_encode;
mod read;
mod toml;
mod toml_encode;
mod xml;
mod yaml;
mod yaml_encode;

type Dict = IndexMap<EcoString, Value, FxBuildHasher>;

#[cfg(test)]
pub use self::cbor::decode_cbor;
pub use self::cbor::{native_cbor, native_cbor_encode};
pub use self::csv::native_csv;
#[cfg(test)]
pub use self::csv::{decode_csv, RowType};
pub use self::json::decode_json;
pub use self::json_encode::native_json_encode;
pub use self::read::native_read;
use self::read::{reject_named, resolve_data};
pub use self::toml::decode_toml;
pub use self::toml_encode::native_toml_encode;
pub use self::xml::decode_xml;
pub use self::yaml::decode_yaml;
pub use self::yaml_encode::native_yaml_encode;

fn err(msg: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), msg.into())]
}

fn new_dict() -> Dict {
    IndexMap::with_hasher(FxBuildHasher)
}

macro_rules! native_loader {
    ($fn_name:ident, $name:literal, $decode:expr) => {
        pub fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            world: &dyn crate::contracts::world::World,
            current_file: FileId,
        ) -> SourceResult<Value> {
            reject_named(args, $name)?;
            let data = resolve_data(args, world, current_file, $name)?;
            $decode(&data[..])
        }
    };
}

native_loader!(native_json, "json", decode_json);
native_loader!(native_yaml, "yaml", decode_yaml);
native_loader!(native_toml, "toml", decode_toml);
native_loader!(native_xml, "xml", decode_xml);

#[cfg(test)]
mod tests {
    use super::*;

    fn p1321_occ(
        name: Option<&str>,
        value: Value,
        start: usize,
    ) -> crate::entities::args::ArgOccurrence {
        crate::entities::args::ArgOccurrence {
            name: name.map(Into::into),
            value,
            span: Span::from_range(tfid(), start..start + 10),
            value_span: Span::from_range(tfid(), start + 3..start + 8),
        }
    }

    #[test]
    fn p1321_csv_missing_and_named_source() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let call = Span::from_range(tfid(), 0..100);
        for args in [
            Args::from_occurrences(call, vec![]),
            Args::from_parts(vec![], IndexMap::default(), call),
        ] {
            let e =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(e.len(), 1);
            assert_eq!(e[0].message, "missing argument: source");
            assert_eq!(e[0].span, call);
            assert!(e[0].hints.is_empty());
        }
        for detached in [false, true] {
            let mut source = p1321_occ(Some("source"), Value::Int(42), 30);
            if detached {
                source.span = Span::detached();
            }
            let expected = source.span;
            let args = Args::from_occurrences(
                call,
                vec![
                    p1321_occ(Some("unknown"), Value::None, 10),
                    source,
                    p1321_occ(Some("source"), Value::None, 50),
                ],
            );
            let e =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(e[0].message, "the argument `source` is positional");
            assert_eq!(e[0].span, expected);
            assert_eq!(e[0].hints, vec![String::from("try removing `source:`")]);
        }
    }

    #[test]
    fn p1321_csv_first_remnant_before_io_or_parsing() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for source in [
            Value::Str("never-read.csv".into()),
            Value::Bytes(Bytes::new(b"a,b\n1".to_vec())),
        ] {
            for named_first in [false, true] {
                for name in ["unknown", "source"] {
                    let mut tails = vec![
                        p1321_occ(None, Value::Int(42), 30),
                        p1321_occ(Some(name), Value::None, 60),
                    ];
                    if named_first {
                        tails.reverse();
                    }
                    let expected_span = tails[0].span;
                    let mut occurrences = vec![
                        p1321_occ(Some("delimiter"), Value::Str(",".into()), 10),
                        p1321_occ(None, source.clone(), 20),
                    ];
                    occurrences.extend(tails);
                    let args = Args::from_occurrences(
                        Span::from_range(tfid(), 0..100),
                        occurrences,
                    );
                    let e = native_csv(&mut EvalContext::new(), &args, &world, tfid())
                        .unwrap_err();
                    assert_eq!(e.len(), 1);
                    assert_eq!(
                        e[0].message,
                        if named_first {
                            format!("unexpected argument: {name}")
                        } else {
                            "unexpected argument".into()
                        }
                    );
                    assert_eq!(e[0].span, expected_span);
                    assert!(e[0].hints.is_empty());
                    assert!(e[0].trace.is_empty());
                }
            }
        }
        let mut named = IndexMap::default();
        named.insert("zeta".into(), Value::None);
        named.insert("alpha".into(), Value::None);
        for extra in [false, true] {
            let mut items = vec![Value::Bytes(Bytes::new(b"a,b".to_vec()))];
            if extra {
                items.push(Value::Int(42));
            }
            let args =
                Args::from_parts(items, named.clone(), Span::from_range(tfid(), 0..100));
            let e =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(
                e[0].message,
                if extra { "unexpected argument" } else { "unexpected argument: zeta" }
            );
            assert!(e[0].span.is_detached());
        }
    }

    #[test]
    fn p1321_csv_parameter_order_precedes_remaining_order() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for (source, delimiter, expected, start) in [
            (
                Value::Int(42),
                Value::Str("xx".into()),
                "expected path, string, or bytes, found integer",
                20,
            ),
            (
                Value::Str("never-read.csv".into()),
                Value::Str("xx".into()),
                "expected exactly one character",
                60,
            ),
            (
                Value::Str("never-read.csv".into()),
                Value::Str(",".into()),
                "expected type, found integer",
                40,
            ),
        ] {
            let args = Args::from_occurrences(
                Span::from_range(tfid(), 0..100),
                vec![
                    p1321_occ(Some("unknown"), Value::None, 10),
                    p1321_occ(None, source, 20),
                    p1321_occ(None, Value::Int(99), 30),
                    p1321_occ(Some("row-type"), Value::Int(1), 40),
                    p1321_occ(Some("delimiter"), delimiter, 60),
                ],
            );
            let e =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(e[0].message, expected);
            assert_eq!(e[0].span, Span::from_range(tfid(), start + 3..start + 8));
        }
    }

    #[test]
    fn p1319_csv_binary_files_keep_cause_path_and_argument_origin() {
        use crate::entities::args::ArgOccurrence;
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        for (data, cause, position) in [
            (b"\xff".as_slice(), "file is not valid UTF-8", "1:1"),
            (b"a,b\n1,\xff".as_slice(), "file is not valid UTF-8", "2:1"),
            (b"a,b\r\n1,\xff".as_slice(), "file is not valid UTF-8", "1:1"),
            (b"a,b\r1,\xff".as_slice(), "file is not valid UTF-8", "1:1"),
            (b"a,b\n\xff".as_slice(), "found 1 instead of 2 fields in line 2", "2:1"),
            (
                b"a,b\r\n1\n\xff,2".as_slice(),
                "found 1 instead of 2 fields in line 2",
                "1:1",
            ),
            (
                b"a,b\n1\n\xff,2".as_slice(),
                "found 1 instead of 2 fields in line 2",
                "2:1",
            ),
            ("a,b\né,😀\r1,\u{fffd}".as_bytes(), "valid control", "unused"),
        ] {
            for (root, expected_path) in [
                (VirtualRoot::Project, "dir/δ.csv"),
                (
                    VirtualRoot::Package("@preview/csv-test:1.2.3".parse().unwrap()),
                    "@preview/csv-test:1.2.3/dir/δ.csv",
                ),
            ] {
                for ty in [
                    crate::entities::value::Type::Array,
                    crate::entities::value::Type::Dictionary,
                ] {
                    for origin in [Span::from_range(tfid(), 30..48), Span::detached()] {
                        let mut world =
                            MockWorld { forbid_sources: true, ..MockWorld::default() };
                        world.files.insert("dir/δ.csv".into(), Arc::new(data.to_vec()));
                        let args = Args::from_occurrences(
                            Span::from_range(tfid(), 0..100),
                            vec![
                                ArgOccurrence {
                                    name: Some("row-type".into()),
                                    value: Value::Type(ty),
                                    span: Span::from_range(tfid(), 1..25),
                                    value_span: Span::from_range(tfid(), 11..25),
                                },
                                ArgOccurrence {
                                    name: None,
                                    value: Value::Path(RootedPath::new(
                                        root.clone(),
                                        VirtualPath::new("dir/./x/../δ.csv").unwrap(),
                                    )),
                                    span: origin,
                                    value_span: origin,
                                },
                            ],
                        );
                        let result =
                            native_csv(&mut EvalContext::new(), &args, &world, tfid());
                        if cause == "valid control" {
                            assert!(result.is_ok());
                        } else {
                            let errors = result.unwrap_err();
                            assert_eq!(errors.len(), 1);
                            assert_eq!(errors[0].message, format!("failed to parse CSV ({cause} in {expected_path}:{position})"));
                            assert_eq!(errors[0].span, origin);
                            assert_eq!(
                                errors[0].severity,
                                crate::entities::source_result::Severity::Error
                            );
                            assert!(errors[0].hints.is_empty());
                            assert!(errors[0].trace.is_empty());
                        }
                        assert!(
                            world.resolved.lock().unwrap().is_empty(),
                            "RootedPath must not be resolved again"
                        );
                        let reads = world.read_paths.lock().unwrap();
                        assert_eq!(reads.len(), 1);
                        assert_eq!(reads[0].root(), &root);
                        assert_eq!(reads[0].vpath().get_with_slash(), "/dir/δ.csv");
                        // P1321: parsing control retained; excess separately rejects before I/O.
                        let mut occurrences = args.occurrences.unwrap();
                        occurrences.push(ArgOccurrence {
                            name: None,
                            value: Value::Int(42),
                            span: Span::from_range(tfid(), 60..62),
                            value_span: Span::from_range(tfid(), 61..62),
                        });
                        let excess = Args::from_occurrences(args.span, occurrences);
                        let forbidden =
                            MockWorld { forbid_io: true, ..MockWorld::default() };
                        let e = native_csv(
                            &mut EvalContext::new(),
                            &excess,
                            &forbidden,
                            tfid(),
                        )
                        .unwrap_err();
                        assert_eq!(e[0].message, "unexpected argument");
                        assert_eq!(e[0].span, Span::from_range(tfid(), 60..62));
                    }
                }
            }
        }
    }

    #[test]
    fn p1319_csv_str_resolves_once_and_invalid_options_do_not_read() {
        let mut world = MockWorld { forbid_sources: true, ..MockWorld::default() };
        world
            .files
            .insert("dir/data.csv".into(), Arc::new(b"a,b\n1,\xff".to_vec()));
        let args = Args::positional(vec![Value::Str("dir/./x/../data.csv".into())]);
        let errors =
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
        assert_eq!(
            errors[0].message,
            "failed to parse CSV (file is not valid UTF-8 in dir/data.csv:2:1)"
        );
        assert!(errors[0].span.is_detached());
        assert_eq!(
            *world.resolved.lock().unwrap(),
            vec![(tfid(), "dir/./x/../data.csv".to_string())]
        );
        assert_eq!(world.read_paths.lock().unwrap().len(), 1);
        let mut args = args;
        args.named.insert("delimiter".into(), Value::Str("xx".into()));
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let errors =
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
        assert_eq!(errors[0].message, "expected exactly one character");
        assert!(world.resolved.lock().unwrap().is_empty());
        assert!(world.read_paths.lock().unwrap().is_empty());
    }

    #[test]
    fn p1318_csv_bytes_positions_follow_buffer_and_parser_offset() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for ty in [
            crate::entities::value::Type::Array,
            crate::entities::value::Type::Dictionary,
        ] {
            for (data, cause, position) in [
                (b"a,b\n1".as_slice(), "found 1 instead of 2 fields in line 2", "2:1"),
                (b"a,b\r\n1".as_slice(), "found 1 instead of 2 fields in line 2", "1:5"),
                (b"a,b\r1".as_slice(), "found 1 instead of 2 fields in line 2", "2:1"),
                (
                    b"\n\na,b\n1".as_slice(),
                    "found 1 instead of 2 fields in line 2",
                    "4:1",
                ),
                (
                    b"\"a\nb\",c\n1".as_slice(),
                    "found 1 instead of 2 fields in line 2",
                    "3:1",
                ),
                (
                    b"\xef\xbb\xbfa,b\n1".as_slice(),
                    "found 1 instead of 2 fields in line 2",
                    "2:1",
                ),
                (b"\xff".as_slice(), "file is not valid UTF-8", "1:1"),
                (b"a,b\n1,\xff".as_slice(), "file is not valid UTF-8", "2:1"),
                (b"a,b\r\n1,\xff".as_slice(), "file is not valid UTF-8", "1:1"),
                (b"a,b\r1,\xff".as_slice(), "file is not valid UTF-8", "1:1"),
                (b"a,b\n\xff".as_slice(), "found 1 instead of 2 fields in line 2", "2:1"),
                (
                    b"a,b\r1\r\xff,2".as_slice(),
                    "found 1 instead of 2 fields in line 2",
                    "1:1",
                ),
                (
                    "\"é\u{2028}😀\",b\r1".as_bytes(),
                    "found 1 instead of 2 fields in line 2",
                    "3:1",
                ),
                (
                    "\"a\u{b}\u{c}\u{85}b\",c\n1".as_bytes(),
                    "found 1 instead of 2 fields in line 2",
                    "5:1",
                ),
                (
                    "a,b\né,😀\r\n1".as_bytes(),
                    "found 1 instead of 2 fields in line 3",
                    "2:5",
                ),
                (
                    b"a,b\n\xc3\xa9,\xf0\x9f\x98\x80\r1,\xff".as_slice(),
                    "file is not valid UTF-8",
                    "2:5",
                ),
            ] {
                let mut args =
                    Args::positional(vec![Value::Bytes(Bytes::new(data.to_vec()))]);
                args.named.insert("row-type".into(), Value::Type(ty));
                let errors = native_csv(&mut EvalContext::new(), &args, &world, tfid())
                    .unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    errors[0].message,
                    format!("failed to parse CSV ({cause} at {position})"),
                    "data {data:?}"
                );
                assert!(errors[0].span.is_detached());
                assert!(errors[0].hints.is_empty());
                assert!(errors[0].trace.is_empty());
            }
        }
    }

    #[test]
    fn p1318_csv_text_position_is_not_argument_origin() {
        let origin = Span::from_range(tfid(), 35..55);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for value_span in [origin, Span::detached()] {
            let args = p1316_bytes_args(
                b"a,b\r\n1",
                value_span,
                crate::entities::value::Type::Dictionary,
            );
            let errors =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(
                errors[0].message,
                "failed to parse CSV (found 1 instead of 2 fields in line 2 at 1:5)"
            );
            assert_eq!(errors[0].span, value_span);
        }
    }

    #[test]
    fn p1318_csv_pure_and_path_do_not_acquire_bytes_suffix() {
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        for data in [b"a,b\r\n1".as_slice(), b"a,b\r\n1,\xff".as_slice()] {
            let mut world = MockWorld::default();
            world.files.insert("data.csv".into(), Arc::new(data.to_vec()));
            for (mode, ty) in [
                (RowType::Array, crate::entities::value::Type::Array),
                (RowType::Dictionary, crate::entities::value::Type::Dictionary),
            ] {
                let pure = decode_csv(data, b',', mode).unwrap_err();
                let cause = if data.ends_with(&[255]) {
                    "file is not valid UTF-8"
                } else {
                    "found 1 instead of 2 fields in line 2"
                };
                assert_eq!(pure[0].message, format!("failed to parse CSV ({cause})"));
                assert!(pure[0].span.is_detached());
                for source in [
                    Value::Str("data.csv".into()),
                    Value::Path(RootedPath::new(
                        VirtualRoot::Project,
                        VirtualPath::new("data.csv").unwrap(),
                    )),
                ] {
                    let mut args = Args::positional(vec![source]);
                    args.named.insert("row-type".into(), Value::Type(ty));
                    let errors =
                        native_csv(&mut EvalContext::new(), &args, &world, tfid())
                            .unwrap_err();
                    // P1319: invalid file gains its own suffix, never Bytes `at`.
                    let expected = if data.ends_with(&[255]) {
                        "failed to parse CSV (file is not valid UTF-8 in data.csv:1:1)"
                    } else {
                        pure[0].message.as_str()
                    };
                    assert_eq!(errors[0].message, expected);
                    assert!(errors[0].span.is_detached());
                }
            }
        }
    }

    #[test]
    fn p1317_csv_utf8_header_and_data_message() {
        for mode in [RowType::Array, RowType::Dictionary] {
            for (data, delimiter) in [
                (b"\xff".as_slice(), b','),
                (b"a,\xff\n1,2".as_slice(), b','),
                (b"a,b\n1,\xff".as_slice(), b','),
                (b"a,b\n1,2\n\"x\ny\",\xff".as_slice(), b','),
                (b"\n\na;b\r\n1;\xc3".as_slice(), b';'),
            ] {
                let errors = decode_csv(data, delimiter, mode).unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    errors[0].message,
                    "failed to parse CSV (file is not valid UTF-8)"
                );
                assert!(errors[0].span.is_detached());
                assert!(errors[0].hints.is_empty());
                assert!(errors[0].trace.is_empty());
            }
        }
    }

    #[test]
    fn p1317_csv_utf8_native_origin_and_synthetic() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let origin = Span::from_range(tfid(), 35..55);
        for ty in [
            crate::entities::value::Type::Array,
            crate::entities::value::Type::Dictionary,
        ] {
            for (args, expected_span, position) in [
                (p1316_bytes_args(b"a,b\n1,\xff", origin, ty), origin, "2:1"),
                (
                    p1316_bytes_args(b"a,b\n1,\xff", Span::detached(), ty),
                    Span::detached(),
                    "2:1",
                ),
                (
                    Args::positional(vec![Value::Bytes(Bytes::new(vec![255]))]),
                    Span::detached(),
                    "1:1",
                ),
            ] {
                let errors = native_csv(&mut EvalContext::new(), &args, &world, tfid())
                    .unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    errors[0].message,
                    format!(
                        "failed to parse CSV (file is not valid UTF-8 at {position})"
                    )
                );
                assert_eq!(errors[0].span, expected_span);
                assert!(errors[0].hints.is_empty());
                assert!(errors[0].trace.is_empty());
            }
        }
    }

    #[test]
    fn p1317_csv_utf8_path_text_without_origin_change() {
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        let mut world = MockWorld::default();
        world
            .files
            .insert("data.csv".into(), Arc::new(b"a,b\n1,\xff".to_vec()));
        for source in [
            Value::Str("data.csv".into()),
            Value::Path(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("data.csv").unwrap(),
            )),
        ] {
            for ty in [
                crate::entities::value::Type::Array,
                crate::entities::value::Type::Dictionary,
            ] {
                let mut args = Args::positional(vec![source.clone()]);
                args.named.insert("row-type".into(), Value::Type(ty));
                let errors = native_csv(&mut EvalContext::new(), &args, &world, tfid())
                    .unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    errors[0].message,
                    "failed to parse CSV (file is not valid UTF-8 in data.csv:2:1)"
                );
                assert!(errors[0].span.is_detached());
            }
        }
    }

    #[test]
    fn p1317_csv_utf8_does_not_preempt_fields_or_reject_unicode() {
        for mode in [RowType::Array, RowType::Dictionary] {
            for data in [b"a,b\n\xff".as_slice(), b"a,b\n1,2,\xff".as_slice()] {
                let errors = decode_csv(data, b',', mode).unwrap_err();
                let found = if data == b"a,b\n\xff" { 1 } else { 3 };
                assert_eq!(errors[0].message, format!("failed to parse CSV (found {found} instead of 2 fields in line 2)"));
            }
        }
        let data = "café,字\né,λ".as_bytes();
        assert_eq!(
            decode_csv(data, b',', RowType::Array).unwrap(),
            Value::Array(vec![
                Value::Array(vec![Value::Str("café".into()), Value::Str("字".into())]),
                Value::Array(vec![Value::Str("é".into()), Value::Str("λ".into())]),
            ])
        );
        assert_eq!(
            decode_csv(data, b',', RowType::Dictionary).unwrap(),
            Value::Array(vec![dict_of(vec![
                ("café", Value::Str("é".into())),
                ("字", Value::Str("λ".into()))
            ]),])
        );
    }

    fn p1316_bytes_args(
        data: &[u8],
        origin: Span,
        row_type: crate::entities::value::Type,
    ) -> Args {
        use crate::entities::args::ArgOccurrence;
        Args::from_occurrences(
            Span::from_range(tfid(), 0..100),
            vec![
                ArgOccurrence {
                    name: Some("row-type".into()),
                    value: Value::Type(row_type),
                    span: Span::from_range(tfid(), 1..20),
                    value_span: Span::from_range(tfid(), 11..20),
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Bytes(Bytes::new(data.to_vec())),
                    span: Span::from_range(tfid(), 30..60),
                    value_span: origin,
                },
            ],
        )
    }

    #[test]
    fn p1316_csv_bytes_parse_origin_not_named_or_call() {
        let origin = Span::from_range(tfid(), 35..55);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for (mode, ty) in [
            (RowType::Array, crate::entities::value::Type::Array),
            (RowType::Dictionary, crate::entities::value::Type::Dictionary),
        ] {
            for (data, position) in [
                (b"\"a\nb\",c\n1".as_slice(), "3:1"),
                (&[255][..], "1:1"),
                (b"a,b\n1,\xff".as_slice(), "2:1"),
            ] {
                let args = p1316_bytes_args(data, origin, ty);
                let before = decode_csv(data, b',', mode).unwrap_err();
                let actual = native_csv(&mut EvalContext::new(), &args, &world, tfid())
                    .unwrap_err();
                assert_eq!(actual.len(), before.len());
                assert_eq!(
                    actual[0].message,
                    format!(
                        "{} at {position})",
                        before[0].message.strip_suffix(')').unwrap()
                    )
                );
                assert_eq!(actual[0].span, origin);
                assert!(before[0].span.is_detached());
            }
        }
    }

    #[test]
    fn p1316_csv_parse_with_excess_uses_first_source_origin() {
        use crate::entities::args::ArgOccurrence;
        let origin = Span::from_range(tfid(), 35..55);
        let mut occurrences =
            p1316_bytes_args(b"a,b\n1", origin, crate::entities::value::Type::Array)
                .occurrences
                .unwrap();
        occurrences.push(ArgOccurrence {
            name: None,
            value: Value::Bytes(Bytes::new(b"valid".to_vec())),
            span: Span::from_range(tfid(), 70..90),
            value_span: Span::from_range(tfid(), 75..85),
        });
        let args = Args::from_occurrences(Span::from_range(tfid(), 0..100), occurrences);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let actual =
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
        // P1321 supersedes the historical parsing-with-excess behavior.
        assert_eq!(actual[0].message, "unexpected argument");
        assert_eq!(actual[0].span, Span::from_range(tfid(), 70..90));
    }

    #[test]
    fn p1316_csv_synthetic_and_explicit_detached_preserved() {
        let data = b"a,b\n1";
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for args in [
            Args::positional(vec![Value::Bytes(Bytes::new(data.to_vec()))]),
            p1316_bytes_args(data, Span::detached(), crate::entities::value::Type::Array),
        ] {
            let actual =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert!(actual[0].span.is_detached());
            assert_eq!(
                actual[0].message,
                "failed to parse CSV (found 1 instead of 2 fields in line 2 at 2:1)"
            );
        }
    }

    #[test]
    fn p1316_csv_path_parse_stays_detached() {
        use crate::entities::args::ArgOccurrence;
        let mut world = MockWorld::default();
        world.files.insert("data.csv".into(), Arc::new(b"a,b\n1".to_vec()));
        let args = Args::from_occurrences(
            Span::from_range(tfid(), 0..40),
            vec![ArgOccurrence {
                name: None,
                value: Value::Str("data.csv".into()),
                span: Span::from_range(tfid(), 4..20),
                value_span: Span::from_range(tfid(), 4..20),
            }],
        );
        let actual =
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
        assert!(actual[0].span.is_detached());
        assert_eq!(
            actual[0].message,
            "failed to parse CSV (found 1 instead of 2 fields in line 2)"
        );
    }

    #[test]
    fn p1315_csv_unequal_lengths_use_record_ordinal() {
        for mode in [RowType::Array, RowType::Dictionary] {
            for (data, delimiter, ordinal, found) in [
                ("\"a\nb\",c\n1", b',', 2, 1),
                ("a,b\n\"1\n2\",3\n4", b',', 3, 1),
                ("\n\na,b\n1", b',', 2, 1),
                ("a,b\n\n1,2\n\n3", b',', 3, 1),
                ("\"a\nb\";c\n1;2;3", b';', 2, 3),
                ("a,b\r\n\"1\r\n2\",3\r\n4", b',', 3, 1),
                ("a,b\n1,2\n3,4\n5", b',', 4, 1),
            ] {
                let errors = decode_csv(data.as_bytes(), delimiter, mode).unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message, format!(
                    "failed to parse CSV (found {found} instead of 2 fields in line {ordinal})"
                ), "input: {data:?}");
                assert!(errors[0].span.is_detached());
            }
        }
    }

    #[test]
    fn p1315_csv_native_bytes_ordinal_without_io() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for mode in [
            crate::entities::value::Type::Array,
            crate::entities::value::Type::Dictionary,
        ] {
            let mut args = Args::positional(vec![Value::Bytes(Bytes::new(
                b"\"a\nb\",c\n1".to_vec(),
            ))]);
            args.named.insert("row-type".into(), Value::Type(mode));
            let errors =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(
                errors[0].message,
                "failed to parse CSV (found 1 instead of 2 fields in line 2 at 3:1)"
            );
            assert!(errors[0].span.is_detached());
        }
    }

    #[test]
    fn p1315_csv_multiline_values_preserved() {
        let data = b"\"a\nb\",c\n1,\"2\n3\"";
        assert_eq!(
            decode_csv(data, b',', RowType::Array).unwrap(),
            Value::Array(vec![
                Value::Array(vec![Value::Str("a\nb".into()), Value::Str("c".into())]),
                Value::Array(vec![Value::Str("1".into()), Value::Str("2\n3".into())]),
            ])
        );
        assert_eq!(
            decode_csv(data, b',', RowType::Dictionary).unwrap(),
            Value::Array(vec![dict_of(vec![
                ("a\nb", Value::Str("1".into())),
                ("c", Value::Str("2\n3".into()))
            ]),])
        );
        for mode in [RowType::Array, RowType::Dictionary] {
            assert_eq!(decode_csv(b"\n\n", b',', mode).unwrap(), Value::Array(vec![]));
        }
    }

    fn p1314_csv_args(options: Vec<(&str, Value, Span)>) -> Args {
        use crate::entities::args::ArgOccurrence;
        let mut occurrences = vec![ArgOccurrence {
            name: None,
            value: Value::Bytes(Bytes::new(b"a;b\n1;2".to_vec())),
            span: Span::from_range(tfid(), 0..20),
            value_span: Span::from_range(tfid(), 4..19),
        }];
        occurrences.extend(options.into_iter().map(|(name, value, value_span)| {
            ArgOccurrence {
                name: Some(name.into()),
                value,
                span: Span::from_range(tfid(), 20..90),
                value_span,
            }
        }));
        Args::from_occurrences(Span::from_range(tfid(), 0..100), occurrences)
    }

    #[test]
    fn p1314_csv_option_errors_use_failing_value_origin() {
        let origin = Span::from_range(tfid(), 40..45);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for (name, value, message) in [
            ("delimiter", Value::Str("".into()), "expected exactly one character"),
            ("delimiter", Value::Str("ab".into()), "expected exactly one character"),
            ("delimiter", Value::Str("α".into()), "delimiter must be an ASCII character"),
            ("delimiter", Value::Bool(false), "expected string, found boolean"),
            ("delimiter", Value::Int(1), "expected string, found integer"),
            ("delimiter", Value::None, "expected string, found none"),
            ("row-type", Value::Str("array".into()), "expected type, found string"),
            ("row-type", Value::Bool(true), "expected type, found boolean"),
            (
                "row-type",
                Value::Type(crate::entities::value::Type::Str),
                "expected `array` or `dictionary`",
            ),
        ] {
            let args = p1314_csv_args(vec![(name, value, origin)]);
            let errors =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, message);
            assert_eq!(errors[0].span, origin);
            assert!(errors[0].hints.is_empty());
            assert!(errors[0].trace.is_empty());
        }
    }

    #[test]
    fn p1314_csv_invalid_occurrence_cannot_be_overwritten() {
        let first = Span::from_range(tfid(), 30..34);
        let last = Span::from_range(tfid(), 60..64);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for (name, bad, good, message) in [
            (
                "delimiter",
                Value::Str("ab".into()),
                Value::Str(";".into()),
                "expected exactly one character",
            ),
            (
                "row-type",
                Value::Str("array".into()),
                Value::Type(crate::entities::value::Type::Array),
                "expected type, found string",
            ),
        ] {
            for (options, origin) in [
                (vec![(name, bad.clone(), first), (name, good.clone(), last)], first),
                (vec![(name, good, first), (name, bad, last)], last),
            ] {
                let args = p1314_csv_args(options);
                let errors = native_csv(&mut EvalContext::new(), &args, &world, tfid())
                    .unwrap_err();
                assert_eq!(errors[0].message, message);
                assert_eq!(errors[0].span, origin);
            }
        }
    }

    #[test]
    fn p1314_csv_delimiter_group_precedes_row_type_group() {
        let first = Span::from_range(tfid(), 25..29);
        let origin = Span::from_range(tfid(), 40..44);
        let last = Span::from_range(tfid(), 70..73);
        let args = p1314_csv_args(vec![
            ("row-type", Value::Str("array".into()), first),
            ("delimiter", Value::Str("ab".into()), origin),
            ("delimiter", Value::Str(";".into()), last),
        ]);
        let errors =
            native_csv(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap_err();
        assert_eq!(errors[0].message, "expected exactly one character");
        assert_eq!(errors[0].span, origin);
    }

    #[test]
    fn p1314_csv_last_valid_options_preserve_values() {
        let args = p1314_csv_args(vec![
            ("delimiter", Value::Str(",".into()), Span::detached()),
            ("delimiter", Value::Str(";".into()), Span::detached()),
            (
                "row-type",
                Value::Type(crate::entities::value::Type::Array),
                Span::detached(),
            ),
            (
                "row-type",
                Value::Type(crate::entities::value::Type::Dictionary),
                Span::detached(),
            ),
        ]);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let got = native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap();
        assert_eq!(
            got,
            Value::Array(vec![dict_of(vec![
                ("a", Value::Str("1".into())),
                ("b", Value::Str("2".into())),
            ])])
        );
    }

    #[test]
    fn p1314_csv_synthetic_and_detached_options_stay_detached() {
        let explicit = p1314_csv_args(vec![(
            "delimiter",
            Value::Str("ab".into()),
            Span::detached(),
        )]);
        let mut named = IndexMap::default();
        named.insert("delimiter".into(), Value::Str("ab".into()));
        let synthetic = Args::from_parts(
            vec![Value::Bytes(Bytes::new(vec![]))],
            named,
            Span::from_range(tfid(), 0..100),
        );
        for args in [explicit, synthetic] {
            let errors =
                native_csv(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                    .unwrap_err();
            assert_eq!(errors[0].message, "expected exactly one character");
            assert!(errors[0].span.is_detached());
        }
    }

    #[test]
    fn p1314_csv_invalid_earlier_option_prevents_path_io() {
        use crate::entities::args::ArgOccurrence;
        let origin = Span::from_range(tfid(), 30..34);
        let args = p1314_csv_args(vec![
            ("delimiter", Value::Str("ab".into()), origin),
            ("delimiter", Value::Str(";".into()), Span::from_range(tfid(), 60..64)),
        ]);
        let mut occurrences = args.occurrences.unwrap();
        occurrences[0] = ArgOccurrence {
            name: None,
            value: Value::Str("never-read.csv".into()),
            span: Span::from_range(tfid(), 0..20),
            value_span: Span::from_range(tfid(), 4..19),
        };
        let args = Args::from_occurrences(Span::from_range(tfid(), 0..100), occurrences);
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let errors =
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
        assert_eq!(errors[0].message, "expected exactly one character");
        assert_eq!(errors[0].span, origin);
    }

    #[test]
    fn p1313_csv_bytes_values_without_world_access() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for (input, expected) in [
            ("", Value::Array(vec![])),
            (
                "a,b\n1,2",
                Value::Array(vec![
                    Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                    Value::Array(vec![Value::Str("1".into()), Value::Str("2".into())]),
                ]),
            ),
            (
                "\"olá,mundo\",\"a\"\"b\"",
                Value::Array(vec![Value::Array(vec![
                    Value::Str("olá,mundo".into()),
                    Value::Str("a\"b".into()),
                ])]),
            ),
        ] {
            let args = Args::positional(vec![Value::Bytes(Bytes::new(
                input.as_bytes().to_vec(),
            ))]);
            assert_eq!(
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap(),
                expected
            );
        }
        let mut args =
            Args::positional(vec![Value::Bytes(Bytes::new(b"a;b\n1;2".to_vec()))]);
        args.named.insert("delimiter".into(), Value::Str(";".into()));
        args.named.insert(
            "row-type".into(),
            Value::Type(crate::entities::value::Type::Dictionary),
        );
        assert_eq!(
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap(),
            Value::Array(vec![dict_of(vec![
                ("a", Value::Str("1".into())),
                ("b", Value::Str("2".into()))
            ])])
        );
    }

    #[test]
    fn p1313_csv_cast_public_type_names() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        for (value, name) in [
            (Value::Int(42), "integer"),
            (Value::Float(1.5), "float"),
            (Value::Bool(true), "boolean"),
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Array(vec![]), "array"),
            (Value::Dict(new_dict()), "dictionary"),
        ] {
            let errors = native_csv(
                &mut EvalContext::new(),
                &Args::positional(vec![value]),
                &world,
                tfid(),
            )
            .unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(
                errors[0].message,
                format!("expected path, string, or bytes, found {name}")
            );
            assert!(errors[0].span.is_detached());
            assert!(errors[0].hints.is_empty());
            assert!(errors[0].trace.is_empty());
        }
    }

    #[test]
    fn p1313_csv_cast_positional_value_origin() {
        use crate::entities::args::ArgOccurrence;
        let origin = Span::from_range(tfid(), 30..32);
        let args = Args::from_occurrences(
            Span::from_range(tfid(), 0..80),
            vec![
                ArgOccurrence {
                    name: Some("delimiter".into()),
                    value: Value::Str(";".into()),
                    span: Span::from_range(tfid(), 3..20),
                    value_span: Span::from_range(tfid(), 16..19),
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Int(42),
                    span: Span::from_range(tfid(), 25..35),
                    value_span: origin,
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Bool(true),
                    span: Span::from_range(tfid(), 40..44),
                    value_span: Span::from_range(tfid(), 40..44),
                },
            ],
        );
        let errors =
            native_csv(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap_err();
        assert_eq!(errors[0].span, origin);
        assert_eq!(errors[0].message, "expected path, string, or bytes, found integer");
    }

    #[test]
    fn p1313_csv_cast_detached_origin_is_not_invented() {
        use crate::entities::args::ArgOccurrence;
        let aggregate = Span::from_range(tfid(), 0..80);
        for args in [
            Args::from_parts(vec![Value::Int(42)], IndexMap::default(), aggregate),
            Args::from_occurrences(
                aggregate,
                vec![ArgOccurrence {
                    name: None,
                    value: Value::Int(42),
                    span: Span::from_range(tfid(), 10..20),
                    value_span: Span::detached(),
                }],
            ),
        ] {
            let errors =
                native_csv(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                    .unwrap_err();
            assert!(errors[0].span.is_detached());
            assert_eq!(
                errors[0].message,
                "expected path, string, or bytes, found integer"
            );
        }
    }

    #[test]
    fn p1313_csv_validation_precedence_without_io() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        let mut unknown = Args::positional(vec![Value::Int(42)]);
        unknown.named.insert("unknown".into(), Value::None);
        let mut cast = Args::positional(vec![Value::Int(42)]);
        cast.named.insert("delimiter".into(), Value::Str("ab".into()));
        let mut delimiter = Args::positional(vec![Value::Bytes(Bytes::new(vec![255]))]);
        delimiter.named.insert("delimiter".into(), Value::Str("ab".into()));
        let mut path_option = Args::positional(vec![Value::Str("never-read.csv".into())]);
        path_option.named.insert("row-type".into(), Value::Bool(true));
        for (args, expected) in [
            (unknown, "expected path, string, or bytes, found integer"),
            (cast, "expected path, string, or bytes, found integer"),
            (delimiter, "expected exactly one character"),
            (path_option, "expected type, found boolean"),
            (Args::positional(vec![]), "missing argument: source"),
        ] {
            let errors =
                native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
            assert_eq!(errors[0].message, expected);
            assert!(errors[0].span.is_detached());
        }
    }

    #[test]
    fn p1313_csv_bytes_preserve_legacy_parsing() {
        let world = MockWorld { forbid_io: true, ..MockWorld::default() };
        // Decoder vigente é a referência desta preservação, não paridade vanilla.
        let invalid_utf8 = [255];
        let legacy = decode_csv(&invalid_utf8, b',', RowType::Array).unwrap_err();
        let utf8_args =
            Args::positional(vec![Value::Bytes(Bytes::new(invalid_utf8.to_vec()))]);
        let actual =
            native_csv(&mut EvalContext::new(), &utf8_args, &world, tfid()).unwrap_err();
        assert_eq!(actual.len(), legacy.len());
        assert_eq!(legacy[0].message, "failed to parse CSV (file is not valid UTF-8)");
        assert_eq!(
            actual[0].message,
            "failed to parse CSV (file is not valid UTF-8 at 1:1)"
        );
        assert_eq!(actual[0].span, legacy[0].span);
        assert!(actual[0].span.is_detached());
        let args = Args::positional(vec![Value::Bytes(Bytes::new(b"a,b\n1".to_vec()))]);
        let errors =
            native_csv(&mut EvalContext::new(), &args, &world, tfid()).unwrap_err();
        assert_eq!(
            errors[0].message,
            "failed to parse CSV (found 1 instead of 2 fields in line 2 at 2:1)"
        );
        assert!(errors[0].span.is_detached());
    }

    #[test]
    fn p1312_read_path_cast_public_type_names() {
        for (value, name) in [
            (Value::Int(42), "integer"),
            (Value::Float(1.5), "float"),
            (Value::Bool(true), "boolean"),
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Array(vec![]), "array"),
            (Value::Dict(new_dict()), "dictionary"),
            (Value::Bytes(Bytes::new(vec![65])), "bytes"),
        ] {
            let errors = native_read(
                &mut EvalContext::new(),
                &Args::positional(vec![value]),
                &MockWorld::default(),
                tfid(),
            )
            .unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(
                errors[0].message,
                format!("expected path or string, found {name}")
            );
            assert!(errors[0].span.is_detached());
            assert!(errors[0].hints.is_empty());
            assert!(errors[0].trace.is_empty());
        }
    }

    #[test]
    fn p1312_read_path_cast_positional_value_origin() {
        use crate::entities::args::ArgOccurrence;
        let value_span = Span::from_range(tfid(), 30..32);
        let args = Args::from_occurrences(
            Span::from_range(tfid(), 0..80),
            vec![
                ArgOccurrence {
                    name: Some("encoding".into()),
                    value: Value::None,
                    span: Span::from_range(tfid(), 5..19),
                    value_span: Span::from_range(tfid(), 15..19),
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Int(42),
                    span: Span::from_range(tfid(), 25..35),
                    value_span,
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Bool(true),
                    span: Span::from_range(tfid(), 40..44),
                    value_span: Span::from_range(tfid(), 40..44),
                },
            ],
        );
        let errors =
            native_read(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap_err();
        assert_eq!(errors[0].span, value_span);
        assert_eq!(errors[0].message, "expected path or string, found integer");
    }

    #[test]
    fn p1312_read_path_cast_detached_origin_is_not_invented() {
        use crate::entities::args::ArgOccurrence;
        let aggregate = Span::from_range(tfid(), 0..80);
        for args in [
            Args::from_parts(vec![Value::Int(42)], IndexMap::default(), aggregate),
            Args::from_occurrences(
                aggregate,
                vec![ArgOccurrence {
                    name: None,
                    value: Value::Int(42),
                    span: Span::from_range(tfid(), 10..20),
                    value_span: Span::detached(),
                }],
            ),
        ] {
            let errors = native_read(
                &mut EvalContext::new(),
                &args,
                &MockWorld::default(),
                tfid(),
            )
            .unwrap_err();
            assert!(errors[0].span.is_detached());
            assert_eq!(errors[0].message, "expected path or string, found integer");
        }
    }

    #[test]
    fn p1312_read_valid_paths_and_missing_preserved() {
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        let mut world = MockWorld::default();
        world.files.insert("data.txt".into(), Arc::new(b"payload".to_vec()));
        for value in [
            Value::Str("data.txt".into()),
            Value::Path(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("data.txt").unwrap(),
            )),
        ] {
            let got = native_read(
                &mut EvalContext::new(),
                &Args::positional(vec![value]),
                &world,
                tfid(),
            )
            .unwrap();
            assert_eq!(got, Value::Str("payload".into()));
        }
        let missing = native_read(
            &mut EvalContext::new(),
            &Args::positional(vec![]),
            &world,
            tfid(),
        )
        .unwrap_err();
        assert_eq!(missing[0].message, "read() requer 1 argumento posicional (caminho)");
        assert!(missing[0].span.is_detached());
        // A antiga rejeição CSV Bytes/cast foi substituída pelo contrato P1313.
    }

    #[test]
    fn p1310_data_source_cast_public_type_names() {
        let cases = [
            (Value::Int(42), "integer"),
            (Value::Float(1.5), "float"),
            (Value::Bool(true), "boolean"),
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Array(vec![]), "array"),
            (Value::Dict(new_dict()), "dictionary"),
        ];
        let loaders = [native_json, native_yaml, native_toml, native_xml, native_cbor];
        for loader in loaders {
            for (value, name) in &cases {
                let args = Args::positional(vec![value.clone()]);
                let errors =
                    loader(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                        .unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    errors[0].message,
                    format!("expected path, string, or bytes, found {name}")
                );
                assert!(errors[0].span.is_detached());
                assert!(errors[0].hints.is_empty());
            }
        }
    }

    #[test]
    fn p1310_data_source_cast_uses_positional_value_origin() {
        use crate::entities::args::ArgOccurrence;
        let aggregate = Span::from_range(tfid(), 0..80);
        let value_span = Span::from_range(tfid(), 30..32);
        let args = Args::from_occurrences(
            aggregate,
            vec![
                ArgOccurrence {
                    name: Some("ignored-by-helper".into()),
                    value: Value::Bool(true),
                    span: Span::from_range(tfid(), 4..20),
                    value_span: Span::from_range(tfid(), 16..20),
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Int(42),
                    span: Span::from_range(tfid(), 25..35),
                    value_span,
                },
                ArgOccurrence {
                    name: None,
                    value: Value::Bool(false),
                    span: Span::from_range(tfid(), 40..45),
                    value_span: Span::from_range(tfid(), 40..45),
                },
            ],
        );
        for name in ["json", "yaml", "toml", "xml", "cbor"] {
            let errors =
                resolve_data(&args, &MockWorld::default(), tfid(), name).unwrap_err();
            assert_eq!(errors[0].span, value_span);
            assert_eq!(
                errors[0].message,
                "expected path, string, or bytes, found integer"
            );
        }
    }

    #[test]
    fn p1310_data_source_cast_does_not_invent_detached_origin() {
        use crate::entities::args::ArgOccurrence;
        let aggregate = Span::from_range(tfid(), 0..80);
        let synthetic =
            Args::from_parts(vec![Value::Int(42)], IndexMap::default(), aggregate);
        let transformed = Args::from_occurrences(
            aggregate,
            vec![ArgOccurrence {
                name: None,
                value: Value::Int(42),
                span: Span::from_range(tfid(), 10..20),
                value_span: Span::detached(),
            }],
        );
        for args in [synthetic, transformed] {
            let errors =
                resolve_data(&args, &MockWorld::default(), tfid(), "json").unwrap_err();
            assert!(errors[0].span.is_detached());
            assert_eq!(
                errors[0].message,
                "expected path, string, or bytes, found integer"
            );
        }
    }

    #[test]
    fn p1310_data_source_valid_inputs_and_missing_are_preserved() {
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        let mut world = MockWorld::default();
        let data = Arc::new(b"payload".to_vec());
        world.files.insert("data.bin".into(), data.clone());
        let path =
            RootedPath::new(VirtualRoot::Project, VirtualPath::new("data.bin").unwrap());
        for value in [
            Value::Str("data.bin".into()),
            Value::Path(path),
            Value::Bytes(Bytes::new(data.as_ref().clone())),
        ] {
            let got =
                resolve_data(&Args::positional(vec![value]), &world, tfid(), "json")
                    .unwrap();
            assert_eq!(got.as_slice(), data.as_slice());
        }
        let errors =
            resolve_data(&Args::positional(vec![]), &world, tfid(), "json").unwrap_err();
        assert_eq!(
            errors[0].message,
            "json() requer 1 argumento posicional (caminho ou bytes)"
        );
        assert!(errors[0].span.is_detached());
    }

    fn dict_of(pairs: Vec<(&str, Value)>) -> Value {
        let mut d = new_dict();
        for (k, v) in pairs {
            d.insert(k.into(), v);
        }
        Value::Dict(d)
    }

    // ── JSON ────────────────────────────────────────────────────────────────
    #[test]
    fn json_objeto_array_primitivos() {
        let v = decode_json(br#"{"a":1,"b":[true,null,"x"],"c":3.5}"#).unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("a", Value::Int(1)),
                (
                    "b",
                    Value::Array(vec![
                        Value::Bool(true),
                        Value::None,
                        Value::Str("x".into())
                    ])
                ),
                ("c", Value::Float(3.5)),
            ])
        );
    }

    #[test]
    fn json_escalar_e_erro() {
        assert_eq!(decode_json(b"3.5").unwrap(), Value::Float(3.5));
        assert_eq!(decode_json(br#""x""#).unwrap(), Value::Str("x".into()));
        assert!(decode_json(b"{bad").is_err());
    }

    #[test]
    fn json_dict_preserva_ordem_de_insercao() {
        let v = decode_json(br#"{"z":1,"a":2,"m":3}"#).unwrap();
        if let Value::Dict(d) = v {
            let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
            assert_eq!(keys, vec!["z", "a", "m"], "ordem de inserção, não alfabética");
        } else {
            panic!("esperava Dict");
        }
    }

    // ── YAML ────────────────────────────────────────────────────────────────
    #[test]
    fn yaml_mapping_e_sequence() {
        let v = decode_yaml(b"a: 1\nb: hello\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![("a", Value::Int(1)), ("b", Value::Str("hello".into()))])
        );
        let v2 = decode_yaml(b"- 1\n- 2\n").unwrap();
        assert_eq!(v2, Value::Array(vec![Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn yaml_tipos_e_null() {
        let v = decode_yaml(b"x: true\ny: 3.5\nz: null\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("x", Value::Bool(true)),
                ("y", Value::Float(3.5)),
                ("z", Value::None),
            ])
        );
    }

    // ── TOML ────────────────────────────────────────────────────────────────
    #[test]
    fn toml_table_tipos() {
        let v = decode_toml(b"x = 1\ny = \"s\"\nf = 1.5\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("x", Value::Int(1)),
                ("y", Value::Str("s".into())),
                ("f", Value::Float(1.5)),
            ])
        );
    }

    #[test]
    fn toml_datetime_graded_str() {
        // graded (ADR-0054 / DEBT-62 contexto): datetime → Str RFC 3339.
        let v = decode_toml(b"d = 1979-05-27\n").unwrap();
        if let Value::Dict(d) = v {
            assert_eq!(d.get("d"), Some(&Value::Str("1979-05-27".into())));
        } else {
            panic!("esperava Dict");
        }
    }

    // ── CBOR ────────────────────────────────────────────────────────────────
    fn cbor_bytes(v: &ciborium::value::Value) -> Vec<u8> {
        let mut out = Vec::new();
        ciborium::into_writer(v, &mut out).unwrap();
        out
    }

    #[test]
    fn cbor_tree() {
        use ciborium::value::Value as C;
        let doc = C::Map(vec![(C::Text("a".into()), C::Integer(1.into()))]);
        let v = decode_cbor(&cbor_bytes(&doc)).unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn cbor_byte_string_returns_bytes() {
        use ciborium::value::Value as C;
        let doc = C::Bytes(vec![1, 2, 3]);
        let v = decode_cbor(&cbor_bytes(&doc)).unwrap();
        assert_eq!(v, Value::Bytes(Bytes::new(vec![1, 2, 3])));
    }

    // ── CSV ─────────────────────────────────────────────────────────────────
    #[test]
    fn csv_array_default() {
        let v = decode_csv(b"a,b\n1,2", b',', RowType::Array).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![
                Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                Value::Array(vec![Value::Str("1".into()), Value::Str("2".into())]),
            ])
        );
    }

    #[test]
    fn csv_dictionary_e_delimiter() {
        let v = decode_csv(b"a;b\n1;2", b';', RowType::Dictionary).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![dict_of(vec![
                ("a", Value::Str("1".into())),
                ("b", Value::Str("2".into())),
            ])])
        );
    }

    // ── P787 — rigor de parsing (mensagens medidas no vanilla 0.15.0) ──────
    #[test]
    fn p787_csv_linha_malformada_rejeitada() {
        // Antes: `flexible(true)` aceitava em silêncio (P786 B1). Vanilla:
        // `failed to parse CSV (found 3 instead of 2 fields in line 2)`.
        let e = decode_csv(b"a,b\n1,2,3\n", b',', RowType::Array).unwrap_err();
        let msg = e.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("failed to parse CSV (found 3 instead of 2 fields in line 2)"),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p787_csv_linha_malformada_dictionary_rejeitada() {
        // Modo dictionary: a validação é contra o nº de chaves do cabeçalho.
        let e = decode_csv(b"a;b\n1;2;3\n", b';', RowType::Dictionary).unwrap_err();
        let msg = e.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("failed to parse CSV (found 3 instead of 2 fields in line 2)"),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p787_csv_valido_nao_regressao() {
        // CSV bem-formado continua a parsear (flexible desligado não quebra o válido).
        let v = decode_csv(b"a,b\n1,2\n3,4\n", b',', RowType::Array).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![
                Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                Value::Array(vec![Value::Str("1".into()), Value::Str("2".into())]),
                Value::Array(vec![Value::Str("3".into()), Value::Str("4".into())]),
            ])
        );
    }

    // ── XML ─────────────────────────────────────────────────────────────────
    #[test]
    fn xml_nodes() {
        let v = decode_xml(b"<r><c>t</c></r>").unwrap();
        let inner_c = dict_of(vec![
            ("tag", Value::Str("c".into())),
            ("attrs", Value::Dict(new_dict())),
            ("children", Value::Array(vec![Value::Str("t".into())])),
        ]);
        let root = dict_of(vec![
            ("tag", Value::Str("r".into())),
            ("attrs", Value::Dict(new_dict())),
            ("children", Value::Array(vec![inner_c])),
        ]);
        assert_eq!(v, Value::Array(vec![root]));
    }

    #[test]
    fn xml_atributos() {
        let v = decode_xml(br#"<a href="x"/>"#).unwrap();
        if let Value::Array(nodes) = v {
            if let Value::Dict(d) = &nodes[0] {
                assert_eq!(
                    d.get("attrs"),
                    Some(&dict_of(vec![("href", Value::Str("x".into()))]))
                );
            } else {
                panic!("esperava Dict");
            }
        } else {
            panic!("esperava Array");
        }
    }

    // ── read() texto vs binário — P398 ───────────────────────────────────────

    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct MockWorld {
        forbid_io: bool,
        forbid_sources: bool,
        resolved: std::sync::Mutex<Vec<(FileId, String)>>,
        read_paths: std::sync::Mutex<Vec<crate::entities::path::RootedPath>>,
        files: std::collections::HashMap<String, Arc<Vec<u8>>>,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl Default for MockWorld {
        fn default() -> Self {
            Self {
                forbid_io: false,
                forbid_sources: false,
                resolved: std::sync::Mutex::new(Vec::new()),
                read_paths: std::sync::Mutex::new(Vec::new()),
                files: std::collections::HashMap::new(),
                library: crate::entities::world_types::Library::default(),
                book: crate::entities::font_book::FontBook::default(),
            }
        }
    }
    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            assert!(!self.forbid_io && !self.forbid_sources, "unexpected World::source");
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            assert!(!self.forbid_io, "unexpected World::file");
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Arc<Vec<u8>>, String> {
            assert!(!self.forbid_io, "unexpected World::read_bytes");
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
        fn resolve_path(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<crate::entities::path::RootedPath, String> {
            assert!(!self.forbid_io, "unexpected World::resolve_path");
            self.resolved.lock().unwrap().push((current_file, path.to_string()));
            let vpath = crate::entities::path::VirtualPath::new(path)
                .map_err(|e| format!("path inválido: {e:?}"))?;
            Ok(crate::entities::path::RootedPath::new(
                crate::entities::path::VirtualRoot::Project,
                vpath,
            ))
        }
        fn read_path(
            &self,
            path: &crate::entities::path::RootedPath,
        ) -> Result<Arc<Vec<u8>>, String> {
            assert!(!self.forbid_io, "unexpected World::read_path");
            self.read_paths.lock().unwrap().push(path.clone());
            let key = path.vpath().get_with_slash().trim_start_matches('/');
            self.files
                .get(key)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {key}"))
        }
    }

    fn mock_args(path: &str) -> Args {
        Args::positional(vec![Value::Str(path.into())])
    }

    #[test]
    fn read_texto_utf8() {
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args("texto.txt"),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn read_binario_nao_utf8() {
        // P824 — ficheiro não-UTF8 SEM `encoding:` é ERRO (paridade vanilla
        // medida: `failed to convert to string (file is not valid UTF-8 in
        // {ficheiro}:{linha}:{col})`); o fallback silencioso para Bytes foi
        // removido (a "heurística vanilla" do comentário antigo estava
        // refutada por medição — P810 §11).
        let mut world = MockWorld::default();
        world.files.insert(
            "logo.png".into(),
            Arc::new(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
        );
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args("logo.png"),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /logo.png:1:1)"
        );
    }

    // ── P824 — read(): named `encoding:` (paridade vanilla read.rs:24-47) ───

    fn mock_args_encoding(path: &str, encoding: Value) -> Args {
        let mut args = mock_args(path);
        args.named.insert("encoding".into(), encoding);
        args
    }

    #[test]
    fn p824_read_encoding_utf8_aceite() {
        // `encoding: "utf8"` é língua válida (antes: rejeitado como named
        // inesperado). Equivale ao default.
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Str("utf8".into())),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn p824_read_encoding_none_devolve_bytes() {
        // `encoding: none` → bytes crus, mesmo em ficheiro UTF-8 válido.
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::None),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Bytes(Bytes::new(b"hello".to_vec())));
    }

    #[test]
    fn p824_read_encoding_fora_do_dominio_erro_vanilla() {
        // Lista medida no vanilla: só "utf8" ou none. String fora do domínio
        // → `expected "utf8" or none` (sem sufixo "found"); outro tipo →
        // `expected "utf8" or none, found {tipo}`.
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let fid = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Str("latin1".into())),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(e[0].message.to_string(), "expected \"utf8\" or none");
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Int(5)),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(e[0].message.to_string(), "expected \"utf8\" or none, found integer");
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Bool(true)),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(e[0].message.to_string(), "expected \"utf8\" or none, found boolean");
    }

    #[test]
    fn p824_read_nao_utf8_linha_col_vanilla() {
        // Posição `line:col` do byte problemático no formato do vanilla
        // (`LineCol::try_from_byte_pos`, diag.rs:1027-1039): com '\n' antes,
        // col = chars desde o último '\n' ("ab\ncd" + 0xE1 → 2:3).
        let mut world = MockWorld::default();
        world
            .files
            .insert("latin1.txt".into(), Arc::new(b"ol\xe1 mundo\n".to_vec()));
        world
            .files
            .insert("multilinha.txt".into(), Arc::new(b"ab\ncd\xe1".to_vec()));
        let fid = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let e =
            native_read(&mut EvalContext::new(), &mock_args("latin1.txt"), &world, fid)
                .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /latin1.txt:1:1)"
        );
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args("multilinha.txt"),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /multilinha.txt:2:3)"
        );
        // Com `encoding: "utf8"` explícito o erro é o mesmo.
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("latin1.txt", Value::Str("utf8".into())),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /latin1.txt:1:1)"
        );
    }

    #[test]
    fn read_vazio() {
        let mut world = MockWorld::default();
        world.files.insert("empty".into(), Arc::new(Vec::new()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args("empty"),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Str("".into()));
    }

    // ── P823 — erro CBOR no formato do vanilla (format_cbor_error) ──────────

    #[test]
    fn p823_cbor_malformado_mensagem_vanilla() {
        // 0xFF = break byte. Vanilla (`loading/cbor.rs:88-98`):
        // `failed to parse CBOR (invalid type: break, expected non-break)`.
        let e = decode_cbor(&[0xff]).unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to parse CBOR (invalid type: break, expected non-break)"
        );
    }

    #[test]
    fn p823_native_cbor_path_acrescenta_ficheiro() {
        // Fonte path: vanilla acrescenta ` in {ficheiro}` dentro dos
        // parênteses (`diag.rs` load_err_in_binary); fonte bytes fica sem
        // sufixo (ambos medidos em P823).
        let mut world = MockWorld::default();
        world.files.insert("invalid.cbor".into(), Arc::new(vec![0xff]));
        let e = native_cbor(
            &mut EvalContext::new(),
            &mock_args("invalid.cbor"),
            &world,
            tfid(),
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to parse CBOR (invalid type: break, expected non-break in invalid.cbor)"
        );
    }

    // ── P701 — cbor.encode (Value → CBOR) e path|bytes nos 5 loaders ─────────

    fn tfid() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn cbor_encode_tipos_basicos_ida_e_volta() {
        let v = dict_of(vec![
            ("a", Value::Int(1)),
            ("b", Value::Str("texto".into())),
            ("c", Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])),
            ("d", Value::None),
            ("e", Value::Bool(true)),
            ("f", Value::Float(1.5)),
        ]);
        let args = Args::positional(vec![v.clone()]);
        let encoded = native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid(),
        )
        .unwrap();
        let bytes = match &encoded {
            Value::Bytes(b) => b.as_slice().to_vec(),
            _ => panic!("esperava Bytes"),
        };
        assert_eq!(
            decode_cbor(&bytes).unwrap(),
            v,
            "ida e volta cbor.encode -> decode_cbor deve preservar o valor"
        );
    }

    #[test]
    fn cbor_encode_bytes_como_byte_string_nao_texto() {
        // Bytes -> CBOR byte-string (não a forma texto de repr()) — paridade
        // vanilla `bytes.rs:370` (`is_human_readable() == false` em ciborium).
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![1, 2, 3]))]);
        let encoded = native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid(),
        )
        .unwrap();
        let bytes = match &encoded {
            Value::Bytes(b) => b.as_slice().to_vec(),
            _ => panic!("esperava Bytes"),
        };
        assert_eq!(decode_cbor(&bytes).unwrap(), Value::Bytes(Bytes::new(vec![1, 2, 3])));
    }

    #[test]
    fn cbor_encode_tipo_opaco_cai_em_repr() {
        // Length não está na tabela dedicada (§3.4) -> fallback Text(repr(v)),
        // igual ao "other" do vanilla (value.rs:363).
        let args = Args::positional(vec![Value::Length(
            crate::entities::layout_types::Length::pt(12.0),
        )]);
        let encoded = native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid(),
        )
        .unwrap();
        let bytes = match &encoded {
            Value::Bytes(b) => b.as_slice().to_vec(),
            _ => panic!("esperava Bytes"),
        };
        assert_eq!(
            decode_cbor(&bytes).unwrap(),
            Value::Str(
                crate::compiler::eval::repr::repr_value(&Value::Length(
                    crate::entities::layout_types::Length::pt(12.0)
                ))
                .into()
            )
        );
    }

    #[test]
    fn cbor_encode_requer_exactamente_1_arg() {
        let args = Args::positional(vec![]);
        assert!(native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid()
        )
        .is_err());
    }

    #[test]
    fn native_json_aceita_bytes_alem_de_path() {
        // P701 — resolve_data: Bytes usado directamente, sem tocar em World::read_bytes.
        let args =
            Args::positional(vec![Value::Bytes(Bytes::new(br#"{"a":1}"#.to_vec()))]);
        let v =
            native_json(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn native_cbor_aceita_bytes_alem_de_path() {
        use ciborium::value::Value as C;
        let doc = C::Map(vec![(C::Text("a".into()), C::Integer(1.into()))]);
        let args = Args::positional(vec![Value::Bytes(Bytes::new(cbor_bytes(&doc)))]);
        let v =
            native_cbor(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn native_json_path_continua_a_funcionar() {
        // Regressão: resolve_data não deve quebrar o caminho já suportado.
        let mut world = MockWorld::default();
        world.files.insert("d.json".into(), Arc::new(br#"{"x":2}"#.to_vec()));
        let v =
            native_json(&mut EvalContext::new(), &mock_args("d.json"), &world, tfid())
                .unwrap();
        assert_eq!(v, dict_of(vec![("x", Value::Int(2))]));
    }

    #[test]
    fn native_json_tipo_invalido_erro() {
        let args = Args::positional(vec![Value::Int(1)]);
        let e =
            native_json(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap_err();
        assert!(
            e[0].message.contains("caminho") || e[0].message.contains("bytes"),
            "msg: {}",
            e[0].message
        );
    }
}
