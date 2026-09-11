// Baseline-only observation harness. Emits actual operation results, no verdict.
// Included as cfg(test) child of compiler::eval in an isolated pinned copy.
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../00_nucleo/diagnosticos/p1339-mutant-closed-state-api-harness.rs"));

#[test]
fn p1340_adversary_focal_actual_apis() {
    use crate::entities::elements::test_callout::CalloutElem;
    let inputs: Json = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../00_nucleo/diagnosticos/p1340-adversary-focal-inputs-r2.json"))).unwrap();
    for case in inputs["cases"].as_array().unwrap() {
        let spec = json!({
            "setup_code":inputs["construction"]["setup_code"], "body_code":case["body"],
            "value_bindings":[{"name":"held_closure","expression":inputs["construction"]["retained_closure_expression"]}],
            "snapshots":[{"id":"candidate","origin":"content_expression","content_expression":inputs["construction"]["candidate_content_expression"]}]
        });
        let world = FixtureWorld::new(&spec);
        let mut scopes = Scopes::new(None);
        let stdlib = make_stdlib_with_features(&world.inputs(), Features::default());
        for (name, binding) in stdlib.iter() { scopes.define(name, binding.value().clone()); }
        let mut construct = EvalContext::new();
        let mut styles = StyleChain::default_chain();
        let mut sink = Sink::new();
        eval_source(&world,"setup",&mut scopes,&mut construct,&mut styles,&mut sink).expect("setup is real valid code");
        let state_init = match case["state"].as_str().unwrap() {
            "opaque_callout" => Value::Content(Content::dynamic(CalloutElem::new(Content::text("opaque"), "title", "note"))),
            "retained_closure" => eval_source(&world,"binding/held_closure",&mut scopes,&mut construct,&mut styles,&mut sink).expect("real closure construction"),
            "closed_int" => Value::Int(7),
            _ => panic!("unfrozen input"),
        };
        scopes.define("s",Value::State(crate::entities::state::State {key:"p1340".into(),init:Box::new(state_init)}));
        let location = Locator::new().next();
        let mut original = TagIntrospector::empty();
        let mut positions = HashMap::new();
        positions.insert(location, Position {page:NonZeroUsize::new(1).unwrap(),point:Point {x:Pt(0.0),y:Pt(0.0)}});
        original.inject_positions(SealedPositions::from_runtime(positions.clone()));
        original.inject_pages(PageStore::from_total_pages(NonZeroUsize::new(1).unwrap()));
        let mut candidate = original.clone();
        match case["candidate"].as_str().unwrap() {
            "counter_update_1" => {
                let output = eval_source(&world,"snapshot/candidate/content",&mut scopes,&mut construct,&mut styles,&mut sink).expect("real update constructor");
                candidate = crate::compiler::introspect::introspect_with_introspector(&require_content(output));
                candidate.inject_positions(SealedPositions::from_runtime(positions.clone()));
                candidate.inject_pages(PageStore::from_total_pages(NonZeroUsize::new(1).unwrap()));
            },
            "position_x_10" => {
                positions.get_mut(&location).unwrap().point.x = Pt(10.0);
                candidate.inject_positions(SealedPositions::from_runtime(positions));
            },
            "same" => {},
            _ => panic!("unfrozen candidate"),
        }
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.current_location = Some(location);
        ctx.introspector = original.clone();
        let mut body_styles = StyleChain::default_chain();
        let mut body_sink = Sink::new();
        let body = eval_source(&world,"body",&mut scopes,&mut ctx,&mut body_styles,&mut body_sink);
        let mut validation_styles = StyleChain::default_chain();
        let mut validation_sink = Sink::new();
        let reads = ctx.context_reads.borrow().reads.clone();
        let observed = with_engine(&world,world.main(),&mut validation_styles,&mut validation_sink,|engine| {
            let self_valid = ctx.context_reads_valid_for(&original,engine);
            let candidate_valid = ctx.context_reads_valid_for(&candidate,engine);
            let mut relations = Vec::new();
            for read in &reads {
                let reflexive = ctx.replay_context_read(read,&original,engine);
                let next = ctx.replay_context_read(read,&candidate,engine);
                relations.push(json!({
                    "self_relation":format!("{:?}",read.result.observation_relation(&reflexive)),
                    "candidate_relation":format!("{:?}",read.result.observation_relation(&next))
                }));
            }
            let details = ctx.context_nonconvergence_diagnostics(&[original.clone(),candidate.clone()],engine);
            json!({"self_validation":self_valid.unwrap(),"candidate_validation":candidate_valid.unwrap(),
                "relations":relations,"details":diagnostics_json(&world,&details.unwrap())})
        });
        println!("P1340_ADVERSARY {}",serde_json::to_string(&json!({
            "id":case["id"],"selected":ctx.has_filtered_counter_reads(),"body":body_json(&world,&body),
            "observed":observed,"construction_sink":diagnostics_json(&world,&sink.into_diagnostics()),
            "body_sink":diagnostics_json(&world,&body_sink.into_diagnostics()),
            "validation_sink":diagnostics_json(&world,&validation_sink.into_diagnostics())
        })).unwrap());
    }
}
