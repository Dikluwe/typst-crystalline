    // P1327: independently frozen against L0 and pinned vanilla; no runtime input.
    mod p1327_oracles {
        use super::*;
        use crate::entities::source_result::{Severity, SourceDiagnostic};

        fn world() -> ImportMockWorld {
            ImportMockWorld::new(
                "",
                &[
                    ("ordinary.typ", "#let x = 17\n"),
                    ("math.typ", "#let x = 23\n"),
                    ("warning.typ", "#import std\n#let x = std.calc.abs(-19)\n"),
                ],
            )
        }

        fn assert_on_source(
            source: &Source,
            side: &[SourceDiagnostic],
            ranges: &[std::ops::Range<usize>],
        ) {
            assert_eq!(side.len(), ranges.len(), "{}: {side:?}", source.text());
            for (diagnostic, range) in side.iter().zip(ranges) {
                assert_eq!(diagnostic.message, "this import has no effect");
                assert_eq!(diagnostic.severity, Severity::Warning);
                assert!(diagnostic.hints.is_empty(), "{diagnostic:?}");
                assert!(diagnostic.trace.is_empty(), "{diagnostic:?}");
                assert_eq!(source.span_byte_range(diagnostic.span), Some(range.clone()));
            }
        }

        pub(super) fn assert_warnings(
            expression: &str,
            side: &[SourceDiagnostic],
            ranges: &[std::ops::Range<usize>],
        ) {
            let source = Source::new_with_parser(
                world().main(),
                expression.to_string(),
                crate::compiler::parse::parse_code,
            );
            assert_on_source(&source, side, ranges);
        }

        pub(super) fn observe(
            expression: &str,
            features: Features,
            ranges: &[std::ops::Range<usize>],
        ) -> Value {
            let (result, side) =
                eval_expression_with_features(&world(), expression, features);
            assert_warnings(expression, &side, ranges);
            result.unwrap_or_else(|errors| panic!("{expression}: {errors:?}"))
        }

        #[test]
        fn p1327_bare_module_warning_values_profiles_and_orders() {
            let cases = [
                ("{ import std; std.calc.abs(-7) }", "std", 7),
                ("{ let renamed = std; import renamed; renamed.calc.abs(-8) }", "renamed", 8),
                ("{ import calc; calc.abs(-9) }", "calc", 9),
                ("{ import \"ordinary.typ\" as original; import original; original.x }", "original", 17),
                ("{ import \"math.typ\" as math; import math; math.x }", "math", 23),
                ("{ import \"ordinary.typ\" as original; let std = original; import std; std.x }", "std", 17),
                ("{ let f() = { import std; std.calc.abs(-11) }; f() }", "std", 11),
                ("{\n let prefix = \"á🦀\";\n let módulo = std;\n import módulo; módulo.calc.abs(-13)\n}", "módulo", 13),
            ];
            for reverse in [false, false, true] {
                for (profile, features) in p1300_profiles() {
                    for index in 0..cases.len() {
                        let index = if reverse { cases.len() - 1 - index } else { index };
                        let (expression, ident, expected) = cases[index];
                        let anchor = format!("import {ident};");
                        let start = expression.find(&anchor).unwrap() + "import ".len();
                        assert_eq!(
                            observe(expression, features, &[start..start + ident.len()]),
                            Value::Int(expected),
                            "{profile}/{expression}",
                        );
                    }
                }
            }
        }

        #[test]
        fn p1327_distinct_locations_and_same_source_execution_dedup() {
            for (_, features) in p1300_profiles() {
                let expression = "{ import std; import std; std.calc.abs(-12) }";
                assert_eq!(
                    observe(expression, features, &[9..12, 21..24]),
                    Value::Int(12)
                );
                let expression =
                    "{ let f() = { import std; std.calc.abs(-11) }; (f(), f()) }";
                assert_eq!(
                    observe(expression, features, &[21..24]),
                    Value::Array(vec![Value::Int(11), Value::Int(11)]),
                );
            }
        }

        #[test]
        fn p1327_successful_import_warning_survives_later_error() {
            for (_, features) in p1300_profiles() {
                for (expression, warning, error) in [
                    ("{ import std; global }", 9..12, 14..20),
                    ("{ let renamed = std; import renamed; global }", 28..35, 37..43),
                ] {
                    let source = Source::new_with_parser(
                        world().main(),
                        expression.to_string(),
                        crate::compiler::parse::parse_code,
                    );
                    let (result, side) =
                        eval_expression_with_features(&world(), expression, features);
                    assert_warnings(expression, &side, &[warning]);
                    let errors = result.expect_err(expression);
                    assert_eq!(errors.len(), 1);
                    assert_eq!(errors[0].message, "unknown variable `global`");
                    assert_eq!(errors[0].severity, Severity::Error);
                    assert!(errors[0].hints.is_empty());
                    assert!(errors[0].trace.is_empty());
                    assert_eq!(source.span_byte_range(errors[0].span), Some(error));
                }
            }
        }

        #[test]
        fn p1327_no_warning_on_other_import_forms() {
            for (_, features) in p1300_profiles() {
                for (expression, expected) in [
                    ("{ let box = (saved: std); import box.saved; saved.calc.abs(-14) }", 14),
                    ("{ import std as chosen; chosen.calc.abs(-15) }", 15),
                    ("{ import std as std; std.calc.abs(-16) }", 16),
                    ("{ import std: calc; calc.abs(-17) }", 17),
                    ("{ import calc: *; abs(-18) }", 18),
                    ("{ import \"ordinary.typ\"; ordinary.x }", 17),
                    ("{ import (std) as chosen; chosen.calc.abs(-20) }", 20),
                    ("{ import (std): calc; calc.abs(-21) }", 21),
                    ("{ import (calc): *; abs(-22) }", 22),
                ] {
                    assert_eq!(observe(expression, features, &[]), Value::Int(expected));
                }
            }
        }

        #[test]
        fn p1327_errors_before_binding_keep_exact_diagnostics_and_no_warning() {
            for (_, features) in p1300_profiles() {
                for (expression, message, range, hints) in [
                    ("{ import (std); none }", "dynamic import requires an explicit name", 9..14, &["you can name the import with `as`"][..]),
                    ("{ import { std }; none }", "dynamic import requires an explicit name", 9..16, &["you can name the import with `as`"][..]),
                    ("{ let f() = std; import f(); none }", "dynamic import requires an explicit name", 24..27, &["you can name the import with `as`"][..]),
                    ("{ import absent; none }", "unknown variable `absent`", 9..15, &[][..]),
                    ("{ let specimen = 42; import specimen; none }", "import: a fonte tem de ser um caminho string ou um módulo, recebeu int", 28..36, &[][..]),
                    ("{ global; import std; none }", "unknown variable `global`", 2..8, &[][..]),
                ] {
                    let source = Source::new_with_parser(
                        world().main(), expression.to_string(), crate::compiler::parse::parse_code,
                    );
                    let (result, side) = eval_expression_with_features(&world(), expression, features);
                    assert_warnings(expression, &side, &[]);
                    let errors = result.expect_err(expression);
                    assert_eq!(errors.len(), 1);
                    assert_eq!(errors[0].message, message);
                    assert_eq!(errors[0].severity, Severity::Error);
                    assert_eq!(errors[0].hints.iter().map(|h| h.as_str()).collect::<Vec<_>>(), hints);
                    assert!(errors[0].trace.is_empty());
                    assert_eq!(source.span_byte_range(errors[0].span), Some(range));
                }
            }
        }

        #[test]
        fn p1327_imported_route_preserves_real_source_anchor() {
            for (_, features) in p1300_profiles() {
                let world = world();
                let expression = "{ import \"warning.typ\" as warned; warned.x }";
                let (result, side) =
                    eval_expression_with_features(&world, expression, features);
                assert_eq!(result.expect(expression), Value::Int(19));
                assert_on_source(
                    world.files.get("warning.typ").unwrap(),
                    &side,
                    &[8..11],
                );
            }
        }
    }
