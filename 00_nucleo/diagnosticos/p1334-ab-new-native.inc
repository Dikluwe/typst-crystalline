    fn occurrence(name: Option<&str>, value: Value, span: Span, value_span: Span) -> ArgOccurrence {
        ArgOccurrence { name: name.map(Into::into), value, span, value_span }
    }
    fn diagnostic(args: &Args, message: &str, span: Span, hint: bool) {
        for features in profiles() {
            let before = args.occurrence_sequence();
            let errors = native(args, features).unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, message);
            assert_eq!(d.severity, Severity::Error);
            assert_eq!(d.span, span);
            assert!(d.trace.is_empty());
            let hints: Vec<&str> = d.hints.iter().map(|s| s.as_str()).collect();
            assert_eq!(hints, if hint { vec!["try removing `value:`"] } else { vec![] });
            let after = args.occurrence_sequence();
            assert_eq!(before.len(), after.len());
            for (a,b) in before.iter().zip(&after) {
                assert_eq!(a.name,b.name); assert_eq!(a.span,b.span);
                assert_eq!(a.value_span,b.value_span);
            }
        }
    }
    #[test]
    fn p1334_missing_uses_received_aggregate_even_with_unrelated_named() {
        let source = Source::detached("aggregate named value");
        for aggregate in [Span::from_range(source.id(),0..20), Span::detached()] {
            for args in [
                Args::from_occurrences(aggregate,vec![]),
                Args::from_occurrences(aggregate,vec![occurrence(Some("bad"), Value::Bool(false), Span::from_range(source.id(),10..20), Span::from_range(source.id(),16..20))]),
                Args::from_parts(vec![],Default::default(),aggregate),
            ] { diagnostic(&args,"missing argument: value",aggregate,false); }
        }
    }
    #[test]
    fn p1334_named_value_selects_first_full_occurrence_and_hint_without_cast() {
        use std::num::NonZeroU16;
        let file_a=FileId::from_raw(NonZeroU16::new(121).unwrap());
        let file_b=FileId::from_raw(NonZeroU16::new(122).unwrap());
        let aggregate=Span::from_range(file_b,0..80);
        let first=Span::from_range(file_a,10..30);
        for argspan in [first, Span::detached()] {
            let args=Args::from_occurrences(aggregate,vec![
                occurrence(Some("bad"),Value::Int(1),Span::from_range(file_b,2..8),Span::from_range(file_b,7..8)),
                occurrence(Some("value"),Value::Bool(false),argspan,Span::from_range(file_a,18..23)),
                occurrence(Some("value"),Value::Int(-19),Span::from_range(file_b,40..50),Span::from_range(file_b,47..50)),
            ]);
            diagnostic(&args,"the argument `value` is positional",argspan,true);
        }
        let mut synthetic=Args::from_parts(vec![],Default::default(),aggregate);
        synthetic.named.insert("bad".into(),Value::Int(1));
        synthetic.named.insert("value".into(),Value::Bool(false));
        diagnostic(&synthetic,"the argument `value` is positional",Span::detached(),true);
    }
    #[test]
    fn p1334_surplus_joint_order_full_span_and_value_name_has_no_hint() {
        let source=Source::detached("aggregate first extra named");
        let aggregate=Span::from_range(source.id(),0..27);
        let first=occurrence(None,Value::Int(-19),Span::from_range(source.id(),10..15),Span::from_range(source.id(),11..15));
        let extra=occurrence(None,Value::Bool(false),Span::from_range(source.id(),16..21),Span::from_range(source.id(),17..21));
        let named=occurrence(Some("value"),Value::Int(7),Span::from_range(source.id(),22..27),Span::from_range(source.id(),24..27));
        for (sequence,message,span) in [
            (vec![first.clone(),extra.clone(),named.clone()],"unexpected argument",extra.span),
            (vec![named.clone(),first.clone(),extra.clone()],"unexpected argument: value",named.span),
            (vec![first.clone(),named.clone(),extra.clone()],"unexpected argument: value",named.span),
        ] { diagnostic(&Args::from_occurrences(aggregate,sequence),message,span,false); }
        let mut synthetic=Args::from_parts(vec![Value::Int(-1),Value::Bool(false)],Default::default(),aggregate);
        synthetic.named.insert("value".into(),Value::Int(1));
        diagnostic(&synthetic,"unexpected argument",Span::detached(),false);
        synthetic.items.truncate(1);
        diagnostic(&synthetic,"unexpected argument: value",Span::detached(),false);
    }
    #[test]
    fn p1334_coherent_first_invalid_remains_so_before_any_surplus() {
        use crate::entities::content::Content;
        use crate::entities::layout_types::Abs;
        let source=Source::detached("aggregate first extra named");
        let aggregate=Span::from_range(source.id(),0..27);
        for (value,message) in [
            (Value::Bool(false),"expected integer, float, length, angle, ratio, fraction, or decimal, found boolean"),
            (Value::Content(Content::empty()),CONTENT_ERROR),
            (Value::Int(i64::MIN),"the result is too large"),
            (Value::Length(Length { abs: Abs::pt(2.0), em: 3.0 }),"cannot take absolute value of this length"),
        ] {
            for origin in [Span::from_range(source.id(),11..15),Span::detached()] {
                let args=Args::from_occurrences(aggregate,vec![
                    occurrence(Some("value"),Value::Int(1),aggregate,aggregate),
                    occurrence(None,value.clone(),Span::from_range(source.id(),10..15),origin),
                    occurrence(None,Value::Int(7),aggregate,aggregate),
                ]);
                diagnostic(&args,message,origin,false);
            }
        }
    }
    #[test]
    fn p1334_all_valid_species_report_the_surplus_without_converting_it() {
        use crate::entities::layout_types::{Abs,Angle,Ratio};
        let source=Source::detached("first surplus");
        let extra=Span::from_range(source.id(),6..13);
        for first in [Value::Int(-19),Value::Float(-1.25),Value::Decimal(Decimal::new(-125,2)),Value::Length(Length::pt(-2.0)),Value::Length(Length{abs:Abs::pt(0.0),em:-2.0}),Value::Angle(Angle::deg(-810.0)),Value::Ratio(Ratio(-2.5)),Value::Fraction(-2.0),Value::Float(f64::NAN),Value::Float(f64::NEG_INFINITY)] {
            let args=Args::from_occurrences(Span::detached(),vec![occurrence(None,first,Span::detached(),Span::detached()),occurrence(None,Value::Int(i64::MIN),extra,Span::detached())]);
            diagnostic(&args,"unexpected argument",extra,false);
        }
    }
    fn public_error(text: &str, anchor: &str, message: &str, hint: bool, trace: Option<&str>) {
        for features in profiles() {
            let (source,result,warnings)=full(text,features);
            assert!(warnings.is_empty(),"{text}: {warnings:?}");
            let errors=result.unwrap_err(); assert_eq!(errors.len(),1);
            let d=&errors[0]; assert_eq!(d.message,message,"{text}");
            assert_eq!(d.severity,Severity::Error);
            let start=text.rfind(anchor).unwrap();
            assert_eq!(source.span_byte_range(d.span),Some(start..start+anchor.len()),"{text}");
            let hints:Vec<&str>=d.hints.iter().map(|x|x.as_str()).collect();
            assert_eq!(hints,if hint {vec!["try removing `value:`"]} else {vec![]});
            assert_eq!(d.trace.len(),usize::from(trace.is_some()),"{text}: {d:?}");
            if let Some(call)=trace { let start=text.rfind(call).unwrap(); assert_eq!(source.span_byte_range(d.trace[0].span),Some(start..start+call.len())); assert_eq!(d.trace[0].v,Tracepoint::Call(Some("abs".into()))); }
        }
    }
    #[test]
    fn p1334_public_missing_alias_import_with_spread_and_math() {
        for (text,anchor) in [
            ("#calc.abs()","calc.abs()"),
            ("#let café=calc.abs\n#café()","café()"),
            ("#import calc: abs\n#abs()","abs()"),
            ("#let a=calc.abs.with().with()\n#a()","a()"),
            ("#calc.abs(..none)","calc.abs(..none)"),
            ("#calc.abs(..())","calc.abs(..())"),
            ("#let aa=arguments()\n#calc.abs(..aa)","calc.abs(..aa)"),
            ("$std.calc.abs()$","std.calc.abs()"),
        ] { public_error(text,anchor,"missing argument: value",false,None); }
    }
    #[test]
    fn p1334_public_named_and_surplus_preserve_external_origins() {
        for (text,anchor,message,hint,trace) in [
            ("#calc.abs(bad: 1, value: false)","value: false","the argument `value` is positional",true,None),
            ("#let a=calc.abs.with(bad: 1).with(value: false).with(value: -1)\n#a()","value: false","the argument `value` is positional",true,Some("a()")),
            ("#let aa=arguments(bad: 1, value: false)\n#calc.abs(..aa)","value: false","the argument `value` is positional",true,Some("calc.abs(..aa)")),
            ("#calc.abs(..(value: false))","..(value: false)","the argument `value` is positional",true,None),
            ("#calc.abs(-1, 2, bad: 1)","2","unexpected argument",false,None),
            ("#calc.abs(value: 2, -1)","value: 2","unexpected argument: value",false,None),
            ("#let a=calc.abs.with(-1, bad: false).with(2)\n#a()","bad: false","unexpected argument: bad",false,Some("a()")),
            ("#let aa=arguments(-1, 2, bad: 1)\n#calc.abs(..aa)","2","unexpected argument",false,Some("calc.abs(..aa)")),
            ("#calc.abs(..(-1,2))","..(-1,2)","unexpected argument",false,None),
        ] { public_error(text,anchor,message,hint,trace); }
    }
