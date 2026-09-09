    fn failures(features: Features) -> Vec<(Value, String)> {
        use crate::entities::content::Content;
        use crate::entities::value::IntrospectedContent;
        let mut values: Vec<_> = family_values(features).into_iter()
            .map(|(value, kind)| (value, rejection(kind))).collect();
        let content = Content::strong(Content::text("-1"));
        values.extend([
            (Value::Content(content.clone()), CONTENT_ERROR.into()),
            (Value::LocatedContent(IntrospectedContent::new(content, None), Locator::new().next()), CONTENT_ERROR.into()),
            (Value::Int(i64::MIN), "the result is too large".into()),
        ]);
        for abs in [-2.0, 2.0] {
            for em in [-3.0, 3.0] {
                values.push((Value::Length(Length { abs: crate::entities::layout_types::Abs::pt(abs), em }), "cannot take absolute value of this length".into()));
            }
        }
        values
    }

    #[test]
    fn p1333_native_first_failure_wins_all_families_and_origins() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for (value, message) in failures(features) {
                for count in [1, 2, 3] {
                    for named in [false, true] {
                        for value_span in [origin, Span::detached()] {
                            let mut items = vec![value.clone()];
                            items.extend(vec![Value::Int(-7); count - 1]);
                            let mut args = Args::from_parts(items, Default::default(), aggregate);
                            if named { args.named.insert("bad".into(), Value::Int(1)); }
                            args.occurrences = Some(vec![
                                ArgOccurrence { name: Some("bad".into()), value: Value::Int(1), span: aggregate, value_span: decoy },
                                // Deliberately inconsistent metadata must not replace Args.items.
                                ArgOccurrence { name: None, value: Value::Int(7), span: occurrence, value_span },
                                ArgOccurrence { name: None, value: Value::Bool(false), span: decoy, value_span: decoy },
                            ]);
                            bare_diagnostic(&native(&args, features).unwrap_err(), &message, value_span);
                            for origins in [None, Some(vec![]), Some(vec![ArgOccurrence { name: Some("bad".into()), value: value.clone(), span: occurrence, value_span: decoy }])] {
                                args.occurrences = origins;
                                bare_diagnostic(&native(&args, features).unwrap_err(), &message, Span::detached());
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn p1333_native_valid_first_never_substitutes_a_later_failure() {
        use crate::entities::layout_types::{Angle, Ratio};
        let source = Source::detached("aggregate origin");
        let aggregate = Span::from_range(source.id(), 0..16);
        for features in profiles() {
            for first in [
                Value::Int(-19), Value::Float(-1.25), Value::Float(f64::NAN), Value::Float(f64::NEG_INFINITY),
                Value::Decimal(Decimal::new(-125, 2)), Value::Length(Length::pt(-2.0)),
                Value::Length(Length { abs: crate::entities::layout_types::Abs::pt(0.0), em: -2.0 }),
                Value::Angle(Angle::deg(-810.0)), Value::Ratio(Ratio(-2.5)), Value::Fraction(-2.0),
            ] {
                for later in [Value::Bool(false), Value::Int(i64::MIN), Value::Length(Length { abs: crate::entities::layout_types::Abs::pt(2.0), em: 3.0 })] {
                    let mut args = Args::from_parts(vec![first.clone(), later], Default::default(), aggregate);
                    bare_diagnostic(&native(&args, features).unwrap_err(), "calc.abs() requer 1 argumento, recebeu 2", Span::detached());
                    args.named.insert("bad".into(), Value::Int(1));
                    bare_diagnostic(&native(&args, features).unwrap_err(), "argumento nomeado inesperado: 'bad'", Span::detached());
                    args.items.truncate(1);
                    bare_diagnostic(&native(&args, features).unwrap_err(), "argumento nomeado inesperado: 'bad'", Span::detached());
                }
            }
            let mut args = Args::from_parts(vec![], Default::default(), aggregate);
            args.occurrences = Some(vec![ArgOccurrence { name: None, value: Value::Bool(false), span: aggregate, value_span: aggregate }]);
            bare_diagnostic(&native(&args, features).unwrap_err(), "calc.abs() requer 1 argumento, recebeu 0", Span::detached());
            args.named.insert("bad".into(), Value::Int(1));
            bare_diagnostic(&native(&args, features).unwrap_err(), "argumento nomeado inesperado: 'bad'", Span::detached());
        }
    }

    #[test]
    fn p1333_public_errors_precede_named_and_extra_without_losing_origin() {
        for (value, message) in [
            ("false", rejection("boolean")), ("\"-19\"", rejection("string")),
            ("[x]", CONTENT_ERROR.into()), ("-9223372036854775807 - 1", "the result is too large".into()),
            ("2pt + 3em", "cannot take absolute value of this length".into()),
        ] {
            for text in [
                format!("#calc.abs({value}, 2)"),
                format!("#calc.abs({value}, bad: 1)"),
                format!("#calc.abs(bad: 1, {value}, 2)"),
                format!("#let a=calc.abs\n#a({value}, bad: 1, 2)"),
                format!("#import calc: abs\n#abs(bad: 1, {value}, 2)"),
            ] { public_error(&text, value, &message, None); }
            for text in [
                format!("#let a=calc.abs.with({value}, bad: 1)\n#a(2)"),
                format!("#import calc: abs\n#let a=abs.with(bad: 1).with({value})\n#a(2)"),
            ] { public_error(&text, value, &message, Some("a(2)")); }
            let text = format!("#let aa=arguments(bad: 1, {value}, 2)\n#calc.abs(..aa)");
            public_error(&text, value, &message, Some("calc.abs(..aa)"));
        }
        public_error("#calc.abs(..(false, 2), bad: 1)", "..(false, 2)", &rejection("boolean"), None);
        public_error("texto á\n#let café=[á]\n#calc.abs(bad: 1,\n café, 2)", "café", CONTENT_ERROR, None);
    }

    #[test]
    fn p1333_eager_argument_failure_still_precedes_native_entry() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expr in [
                "calc.abs(false, panic(\"p1333-eager\"))",
                "calc.abs(false, bad: panic(\"p1333-eager\"))",
                "calc.abs(bad: panic(\"p1333-eager\"), false)",
            ] {
                let (result, warnings) = eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty());
                let errors = result.unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message, "panicked with: p1333-eager");
                assert_eq!(errors[0].severity, Severity::Error);
                assert!(errors[0].hints.is_empty());
            }
        }
    }
}
