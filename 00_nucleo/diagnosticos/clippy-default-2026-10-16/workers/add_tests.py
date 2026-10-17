from pathlib import Path
p=Path('01_core/src/compiler/eval/tests.rs')
s=p.read_text();anchor='pub(crate) fn eval_for_test<W: World>('
test='''#[test]
fn default_contract_eval_context_preserves_settings_budget_and_isolation() {
    fn assert_fresh(ctx: &EvalContext) {
        assert_eq!(ctx.target, EvalTarget::Paged);
        assert_eq!(ctx.features, crate::entities::compiler_features::Features::empty());
        assert_eq!(ctx.loop_iterations, 0);
        assert_eq!(ctx.max_loop_iterations, 1_000_000);
        assert_eq!(ctx.next_rule_id, 0);
        assert_eq!(ctx.next_context_id, 0);
        assert!(!ctx.full_error);
        assert!(ctx.apply_show_rules);
        assert!(!ctx.in_context);
        assert!(ctx.current_location.is_none());
        assert!(ctx.flow.is_none());
        assert!(ctx.bibliography_styles.is_empty());
        assert!(ctx.document_info.is_empty());
    }
    let mut changed = <EvalContext as Default>::default();
    assert_fresh(&changed);
    changed.max_loop_iterations = 1;
    assert!(changed.tick_loop(Span::detached()).is_ok());
    assert!(changed.tick_loop(Span::detached()).is_err());
    changed.target = EvalTarget::Html;
    changed.features.enable(crate::entities::compiler_features::Feature::Html);
    changed.full_error = true;
    changed.apply_show_rules = false;
    changed.in_context = true;
    changed.next_rule_id = 9;
    assert_eq!(changed.next_context_id(), 0);
    assert_eq!(changed.next_context_id(), 1);
    let mut fresh = <EvalContext as Default>::default();
    assert_fresh(&fresh);
    assert_fresh(&EvalContext::new());
    assert_eq!(fresh.next_context_id(), 0);
    assert!(fresh.tick_loop(Span::detached()).is_ok());
    assert_eq!(fresh.loop_iterations, 1);
    assert_eq!(changed.loop_iterations, 2);
}

'''
assert s.count(anchor)==1;p.write_text(s.replace(anchor,test+anchor,1))
p=Path('03_infra/src/world/font_session.rs');s=p.read_text();anchor='    #[test]\n    fn watch_session_reuses_and_renews_generation()'
test='''    #[test]
    fn default_contract_font_session_is_lazy_reusable_and_independent() {
        let session = <FontCatalogSession as Default>::default();
        let untouched = <FontCatalogSession as Default>::default();
        let control = FontCatalogSession::new();
        assert!(session.cached.lock().unwrap().is_none());
        assert!(untouched.cached.lock().unwrap().is_none());
        assert!(control.cached.lock().unwrap().is_none());
        let fixture = Fixture::new();
        let paths = fixture.paths();
        let catalog = session.catalog_with(|| fixture_prepare(&paths), build, empty);
        assert_eq!(catalog.0.len(), 1);
        assert_eq!(catalog.0.len(), catalog.1.len());
        let reused = session.catalog_with(
            || fixture_prepare(&paths),
            |_| panic!("unchanged Default session must reuse its catalog"),
            empty,
        );
        assert!(Arc::ptr_eq(&catalog, &reused));
        assert!(untouched.cached.lock().unwrap().is_none());
        assert!(control.cached.lock().unwrap().is_none());
    }
'''
assert s.count(anchor)==1;p.write_text(s.replace(anchor,test+anchor,1))
p=Path('03_infra/src/world/source_session.rs');s=p.read_text();anchor='    #[test]\n    fn unchanged_main_and_import_share_cst_but_worlds_stay_fresh()'
test='''    #[test]
    fn default_contract_source_session_reuses_validated_text_in_fresh_worlds() {
        let session = Arc::new(<SourceSession as Default>::default());
        let independent = Arc::new(<SourceSession as Default>::default());
        assert!(session.entries.lock().unwrap().is_empty());
        assert!(independent.entries.lock().unwrap().is_empty());
        assert!(SourceSession::new().entries.lock().unwrap().is_empty());
        let fixture = Fixture::new();
        let path = fixture.file("main.typ", "Alpha");
        let first_world = fixture.world("main.typ", &session);
        let first = first_world.source(first_world.main()).unwrap();
        let next_world = fixture.world("main.typ", &session);
        let reused = next_world.source(next_world.main()).unwrap();
        assert!(same(&first, &reused));
        assert!(next_world.read_cache.lock().unwrap().is_empty());
        assert!(independent.entries.lock().unwrap().is_empty());
        fs::write(path, "Bravo").unwrap();
        let independent_world = fixture.world("main.typ", &independent);
        assert_eq!(independent_world.source(independent_world.main()).unwrap().text(), "Bravo");
        assert_eq!(first.text(), "Alpha");
        assert_eq!(session.entries.lock().unwrap()[0].source.text(), "Alpha");
        let updated_world = fixture.world("main.typ", &session);
        let updated = updated_world.source(updated_world.main()).unwrap();
        assert_eq!(updated.text(), "Bravo");
        assert!(!same(&first, &updated));
    }
    #[test]
    fn default_contract_source_session_preserves_retention_bounds() {
        let fixture = Fixture::new();
        let session = <SourceSession as Default>::default();
        for n in 0..=MAX_ENTRIES {
            session.get_or_parse(fixture.0.join(format!("{n}.typ")), id(1), "a".into());
        }
        assert_eq!(session.entries.lock().unwrap().len(), MAX_ENTRIES);
        assert!(!session.entries.lock().unwrap().iter().any(|entry| entry.path == fixture.0.join("0.typ")));
        let oversized = "a".repeat(MAX_ENTRY_BYTES + 1);
        let source = session.get_or_parse(fixture.0.join("large.typ"), id(1), oversized.clone());
        assert_eq!(source.text(), oversized);
        assert_eq!(session.entries.lock().unwrap().len(), MAX_ENTRIES);
        assert!(!session.entries.lock().unwrap().iter().any(|entry| entry.path == fixture.0.join("large.typ")));
        let aggregate = <SourceSession as Default>::default();
        for n in 0..3 {
            aggregate.get_or_parse(fixture.0.join(format!("aggregate{n}.typ")), id(1), "a".repeat(MAX_ENTRY_BYTES));
        }
        let retained = aggregate.entries.lock().unwrap();
        assert_eq!(retained.len(), 2);
        assert_eq!(retained.iter().map(|entry| entry.source.len_bytes()).sum::<usize>(), MAX_BYTES);
    }
'''
assert s.count(anchor)==1;p.write_text(s.replace(anchor,test+anchor,1))
print('Quatro controles Default acrescentados; implementação ainda ausente.')
