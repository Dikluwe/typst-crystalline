    mod r3_t1_environment_refuters {
        use super::*;
        use crate::entities::content::ShowRecipeContent;
        use crate::entities::show::{NodeKind, Selector, Transformation};

        #[test]
        fn r3_t1_environment_restored_and_callback_once() {
            use comemo::Track;
            for generic in [false, true] {
                for fail in [false, true] {
                    let world = MockWorld::new("A");
                    let mut ctx = EvalContext::new();
                    ctx.defer_show_rules = true;
                    ctx.next_context_id = 3;
                    let route = Route::root().with_id(world.main());
                    let mut styles = StyleChain::default_chain();
                    let expected_styles = styles.collapse();
                    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
                    let expected_rules = show_rules.clone();
                    let mut guards = vec![900];
                    let metrics = FixedMetrics;
                    let mut sink = Sink::new();
                    let mut tracked_sink = sink.track_mut();
                    let mut engine = Engine {
                        world: &world,
                        font_metrics: &metrics,
                        route: route.track(),
                        styles: &mut styles,
                        show_rules: &mut show_rules,
                        active_guards: &mut guards,
                        current_file: world.main(),
                        sink: &mut tracked_sink,
                    };
                    let frame = rules::textual::enter_scope(&mut ctx, &engine);
                    let transform = if fail {
                        Func::native("r3-fail", |ctx, _, _, _| {
                            ctx.next_context_id += 1;
                            Err(vec![SourceDiagnostic::error(Span::detached(), "R3-FAIL")])
                        })
                    } else if generic {
                        Func::native("r3-alias", |ctx, args, _, _| {
                            ctx.next_context_id += 1;
                            Ok(args.items[0].clone())
                        })
                    } else {
                        Func::native("r3-text", |ctx, _, _, _| {
                            ctx.next_context_id += 1;
                            Ok(Value::Content(Content::text("B")))
                        })
                    };
                    let local: Arc<[ShowRule]> = Arc::from([ShowRule {
                        id: 7,
                        selector: if generic { Selector::NodeKind(NodeKind::Strong) } else { Selector::Text("A".into()) },
                        transform: Transformation::Func(transform),
                    }]);
                    let pending = Content::ShowRecipe(Arc::new(ShowRecipeContent {
                        body: if generic { Content::strong(Content::text("A")) } else { Content::text("A") },
                        textual: !generic,
                        context_id_floor: 3,
                        rules: local.clone(),
                        styles: rules::textual::capture_styles(engine.styles, &local, &ctx, world.main()),
                        guards: vec![],
                        file: world.main(),
                    }));
                    rules::textual::leave_scope(&mut ctx, frame);
                    let result = crate::compiler::realize::recipes::realize(&pending, &mut ctx, &mut engine);
                    assert_eq!(result.is_err(), fail, "generic={generic}: {result:?}");
                    if let Ok(product) = result {
                        assert_eq!(product.plain_text(), if generic { "A" } else { "B" });
                        let again = crate::compiler::realize::recipes::realize(&product, &mut ctx, &mut engine).unwrap();
                        assert_eq!(again.plain_text(), product.plain_text());
                    }
                    assert_eq!(ctx.next_context_id, 4, "callback runs once");
                    assert!(ctx.defer_show_rules);
                    assert!(!ctx.realizing_show_rules);
                    assert!(ctx.textual_frames.is_empty());
                    assert!(ctx.recipe_protected_locations.is_empty());
                    assert_eq!(ctx.recipe_math_depth, 0);
                    assert_eq!(ctx.recipe_text_edges, (false, false));
                    assert_eq!(ctx.occurrence_show_depth, 0);
                    assert_eq!(*engine.active_guards, [900]);
                    assert_eq!(engine.styles.collapse(), expected_styles);
                    assert!(Arc::ptr_eq(engine.show_rules, &expected_rules));
                    assert_eq!(engine.current_file, world.main());
                }
            }
        }
    }
