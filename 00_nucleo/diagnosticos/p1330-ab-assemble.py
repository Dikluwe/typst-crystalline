#!/usr/bin/env python3
"""One-time test-author assembly from allowed historical oracle helpers only."""
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"

def add(name, body):
    path = D / name
    assert not path.exists(), path
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n"
    patch += "".join("+" + line + "\n" for line in body.splitlines())
    patch += "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch.encode(), check=True)

old = (D / "p1329-ab-p1328-successor.rs").read_text()
successor = old.replace("                (Value::Int(i64::MIN), Value::Int(i64::MAX)),\n", "")
needle = "            for (input, expected) in [\n"
successor = successor.replace(needle, '''            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features)
                    .unwrap_err(),
                "the result is too large",
                Span::detached(),
            );
''' + needle, 1)
successor = successor.replace('                ("calc.abs(-9223372036854775807 - 1)", Value::Int(i64::MAX)),\n', '')
needle = '            for (expr, message, name) in [\n'
successor = successor.replace(needle, '''            let expr = "calc.abs(-9223372036854775807 - 1)";
            let (result, warnings) = eval_expression_with_features(&world, expr, features);
            assert!(warnings.is_empty(), "{expr}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, "the result is too large");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            let source = Source::new_with_parser(
                world.main(),
                expr.into(),
                crate::compiler::parse::parse_code,
            );
            assert_eq!(source.span_byte_range(d.span), Some(9..expr.len() - 1));
''' + needle, 1)
assert successor != old
add("p1330-ab-p1328-successor.rs", successor)

history = (D / "p1329-ab-tests-r1.rs").read_text()
prefix = history[:history.index('    const MIXED_ERROR:')].replace('mod p1329_tests', 'mod p1330_tests')
prefix = prefix.replace('    use crate::entities::layout_types::{Abs, Angle, Length, Ratio};\n', '')
public = history[history.index('    fn public_error('):history.index('    #[test]\n    fn p1329_public_mixed_origins')]
tests = r'''
    const OVERFLOW: &str = "the result is too large";

    #[test]
    fn p1330_native_integer_limits_exact_species_and_magnitude() {
        for features in profiles() {
            // Explicit expected values avoid reusing the implementation formula.
            for (input, expected) in [
                (i64::MIN + 1, i64::MAX),
                (i64::MIN + 2, i64::MAX - 1),
                (-9007199254740993, 9007199254740993),
                (-2147483648, 2147483648),
                (-1, 1), (0, 0), (1, 1),
                (2147483648, 2147483648),
                (9007199254740993, 9007199254740993),
                (i64::MAX - 1, i64::MAX - 1),
                (i64::MAX, i64::MAX),
            ] {
                match native(&Args::positional(vec![Value::Int(input)]), features).unwrap() {
                    Value::Int(actual) => assert_eq!(actual, expected, "input {input}"),
                    other => panic!("integer abs changed species: {other:?}"),
                }
            }
            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features).unwrap_err(),
                OVERFLOW, Span::detached(),
            );
        }
    }

    #[test]
    fn p1330_native_overflow_origin_first_positional_and_no_fabrication() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for value_span in [origin, Span::detached()] {
                let mut args = Args::from_parts(vec![Value::Int(i64::MIN)], Default::default(), aggregate);
                // Metadata is intentionally inconsistent with the named view to
                // distinguish occurrence filtering from the named-argument guard.
                args.occurrences = Some(vec![
                    ArgOccurrence { name: Some("metadata".into()), value: Value::Int(1), span: aggregate, value_span: decoy },
                    ArgOccurrence { name: None, value: Value::Int(i64::MIN), span: occurrence, value_span },
                    ArgOccurrence { name: None, value: Value::Int(i64::MIN), span: decoy, value_span: decoy },
                ]);
                bare_diagnostic(&native(&args, features).unwrap_err(), OVERFLOW, value_span);
            }
            for occurrences in [None, Some(vec![])] {
                let mut args = Args::from_parts(vec![Value::Int(i64::MIN)], Default::default(), aggregate);
                args.occurrences = occurrences;
                bare_diagnostic(&native(&args, features).unwrap_err(), OVERFLOW, Span::detached());
            }
        }
    }

    #[test]
    fn p1330_native_named_and_arity_guards_precede_overflow() {
        for features in profiles() {
            for count in [0, 1, 2] {
                let mut args = Args::positional(vec![Value::Int(i64::MIN); count]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(&native(&args, features).unwrap_err(), "argumento nomeado inesperado: 'bad'", Span::detached());
                if count != 1 {
                    args.named.clear();
                    bare_diagnostic(&native(&args, features).unwrap_err(), &format!("calc.abs() requer 1 argumento, recebeu {count}"), Span::detached());
                }
            }
        }
    }

    #[test]
    fn p1330_public_integer_boundaries_and_positive_routes() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-9223372036854775807)", i64::MAX),
                ("calc.abs(-9223372036854775807 + 1)", i64::MAX - 1),
                ("calc.abs(9223372036854775807)", i64::MAX),
                ("calc.abs(-9007199254740993)", 9007199254740993),
                ("calc.abs(-0)", 0),
                ("{let ab=calc.abs; ab(-1)}", 1),
                ("{let ab=calc.abs.with(-19).with(); ab()}", 19),
                ("{let aa=arguments(-19); calc.abs(..aa)}", 19),
            ] {
                let (result, warnings) = eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                match result.unwrap() {
                    Value::Int(actual) => assert_eq!(actual, expected, "{expr}"),
                    other => panic!("{expr}: wrong species {other:?}"),
                }
            }
        }
    }

    #[test]
    fn p1330_public_overflow_real_origins_alias_with_arguments_utf8() {
        for (text, anchor, trace) in [
            ("#calc.abs(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with()\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1)\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1).with()\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#calc.abs(..(-9223372036854775807 - 1,))", "..(-9223372036854775807 - 1,)", None),
            ("#let aa = arguments(-9223372036854775807 - 1)\n#calc.abs(..aa)", "-9223372036854775807 - 1", Some("calc.abs(..aa)")),
            ("#let x = -9223372036854775807 - 1\n#let y = -9223372036854775807 - 1\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = -9223372036854775807 - 1\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, OVERFLOW, trace);
        }
    }

    #[test]
    fn p1330_math_spelling_remains_content_without_integer_coercion() {
        for text in [
            "$std.calc.abs(-9223372036854775807 - 1)$",
            "#let ab = calc.abs\n$ab(-9223372036854775807 - 1)$",
        ] {
            public_error(text, "-9223372036854775807 - 1", CONTENT_ERROR, None);
        }
    }

    #[test]
    fn p1330_warning_survives_overflow_and_boundary_success() {
        for features in profiles() {
            for (text, fails) in [
                ("#import std\n#calc.abs(-9223372036854775807)", false),
                ("#import std\n#calc.abs(-9223372036854775807 - 1)", true),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if fails {
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    let d = &errors[0];
                    assert_eq!(d.message, OVERFLOW);
                    assert_eq!(d.severity, Severity::Error);
                    assert!(d.hints.is_empty());
                    assert!(d.trace.is_empty());
                    let start = text.find("-9223372036854775807 - 1").unwrap();
                    assert_eq!(source.span_byte_range(d.span), Some(start..start + 24));
                } else {
                    assert!(result.is_ok());
                }
            }
        }
    }
}
'''
# The ASCII expression has 24? Count independently instead of baking a miscount.
tests = tests.replace('Some(start..start + 24)', 'Some(start..start + "-9223372036854775807 - 1".len())')
add("p1330-ab-tests.rs", prefix + public + tests)
