from pathlib import Path
import json
out=Path(__file__).resolve().parents[1];rows=json.load(open(out/'value-fixtures.json'));exprs=[r['expression'].replace('state("x")','state("x",0)').replace('color.spot("x",fallback:red)','color.spot("x",red)') for r in rows];p=Path('01_core/src/compiler/eval/call_dispatch/call.rs');s=p.read_text();assert s.endswith('}\n');literal=',\n'.join('            '+json.dumps(x,ensure_ascii=False) for x in exprs)
s=s[:-2]+'''
    fn all_values() -> Vec<Value> {
        let world = ImportMockWorld::new("", &[]);
        let mut values = vec![];
        for expression in [
'''+literal+'''
        ] {
            let (value, _) = eval_expression(&world, expression);
            values.push(value.unwrap_or_else(|errors| panic!("{expression}: {errors:?}")));
        }
        let location = crate::entities::locator::Locator::new().next();
        values.push(Value::Location(location));
        values.push(Value::LocatedContent(
            crate::entities::value::IntrospectedContent::new(crate::entities::content::Content::Empty, None), location,
        ));
        let kinds: std::collections::HashSet<_> = values.iter().map(std::mem::discriminant).collect();
        assert_eq!(values.len(), 39);
        assert_eq!(kinds.len(), 39, "each current Value variant must be exercised");
        values
    }

    fn with_engine(run: impl FnOnce(&mut crate::compiler::scopes::Scopes<'_>, &mut crate::compiler::eval::EvalContext, &mut crate::entities::engine::Engine<'_>)) {
        use comemo::Track;
        let world = ImportMockWorld::new("", &[]);
        let library = Library::with_global(crate::compiler::stdlib::make_stdlib_with_features(
            &crate::contracts::world::SysInputs::default(), crate::entities::compiler_features::Features::empty(),
        ));
        let mut scopes = crate::compiler::scopes::Scopes::new(Some(&library));
        let mut ctx = crate::compiler::eval::EvalContext::new();
        let mut styles = crate::entities::style::StyleChain::default_chain();
        let mut show_rules = std::sync::Arc::from([]);
        let mut active_guards = Vec::new();
        let route = crate::entities::world_types::Route::root().with_id(world.main());
        let metrics = crate::compiler::layout::FixedMetrics;
        let mut sink = crate::entities::sink::Sink::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = crate::entities::engine::Engine {
            world: &world, font_metrics: &metrics, route: route.track(), styles: &mut styles,
            show_rules: &mut show_rules, active_guards: &mut active_guards,
            current_file: world.main(), sink: &mut tracked_sink,
        };
        run(&mut scopes, &mut ctx, &mut engine);
    }

    #[test]
    fn call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments() {
        let values = all_values();
        with_engine(|scopes, ctx, engine| {
            for value in values {
                let original = value.clone();
                let pointer = if let Value::Array(array) = &value { Some(array.as_ptr()) } else { None };
                let result = crate::compiler::stdlib::try_dispatch_collection_method_with(
                    value, "__unknown", |_, _, _| panic!("unknown operation evaluated arguments"), None,
                    scopes, ctx, engine,
                );
                let crate::compiler::stdlib::CollectionDispatch::Unhandled(value) = result else { panic!("unknown operation handled") };
                assert_eq!(std::mem::discriminant(&original), std::mem::discriminant(&value));
                assert_eq!(original, value);
                if let Value::Array(array) = &value { assert_eq!(pointer, Some(array.as_ptr())); }
            }
        });
    }

    #[test]
    fn call_once_callable_classification_covers_all_value_variants_before_arguments() {
        use crate::entities::ast::{AstNode, expr::Expr};
        let values = all_values();
        let source = Source::new_with_parser(FileId::from_raw(NonZeroU16::new(1).unwrap()),
            "placeholder(panic(\\\"argument executed\\\"))".into(), crate::compiler::parse::parse_code);
        let call = source.root().children().find_map(|child| {
            if let Some(Expr::FuncCall(call)) = Expr::from_untyped(child) { Some(call) } else { None }
        }).expect("call fixture");
        with_engine(|scopes, ctx, engine| {
            for value in values {
                let callable = matches!(value, Value::Func(_) | Value::Type(_));
                let expected = if callable { "panicked with: argument executed".to_owned() }
                    else { format!("expected function, found {}", crate::compiler::eval::operators::error_formatting::vanilla_type_name(&value)) };
                let errors = super::super::invoke::invoke_resolved_call(Some(value), None, call, scopes, ctx, engine).unwrap_err();
                assert_eq!(errors[0].message, expected);
                if !callable { assert_eq!(errors[0].span, call.callee().span()); }
            }
        });
    }
}
''';s=s.replace('crate::entities::style::StyleChain','crate::entities::style::StyleChain');p.write_text(s);print('two 39-variant matrices added')
