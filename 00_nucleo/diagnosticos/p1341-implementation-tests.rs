// P1341 independent RED test. Include this after
// p1340-implementation-pipeline-observer.rs inside
// context_stabilization::observation. This file owns no product hook.

const P1341_BINDING_MANIFEST: &str =
    "180d34a571365b225e515c873b794f8956c29065e559d72c4f5a2ca530b26b81";

fn p1341_keys(value: &serde_json::Value) -> std::collections::BTreeSet<&str> {
    value
        .as_object()
        .expect("P1341 closed object")
        .keys()
        .map(String::as_str)
        .collect()
}

fn p1341_expected_keys<'a>(names: &'a [&'a str]) -> std::collections::BTreeSet<&'a str> {
    names.iter().copied().collect()
}

fn p1341_nonempty_prefixed(value: &serde_json::Value, prefix: &str) -> String {
    let value = value.as_str().expect("P1341 identity must be a string");
    assert!(value.starts_with(prefix) && value.len() > prefix.len());
    value.to_owned()
}

fn p1341_assert_focal_ledger(ledger: &serde_json::Value, source: &str) {
    assert_eq!(
        p1341_keys(ledger),
        p1341_expected_keys(&[
            "cell_key",
            "binding_manifest_sha256",
            "objects",
            "events",
            "causal_edges",
            "payload_state",
        ]),
        "ledger schema must be closed",
    );
    assert!(ledger["cell_key"].as_str().is_some_and(|key| !key.is_empty()));
    assert_eq!(ledger["binding_manifest_sha256"], P1341_BINDING_MANIFEST);
    assert_eq!(ledger["payload_state"], serde_json::json!({"kind":"inspectable"}));

    let required_roles = [
        "execution",
        "context.default",
        "context.selected",
        "callback.with",
        "func.callback",
        "func.body",
        "counter.step.current",
        "content.walked",
        "dict.result",
    ];
    let mut by_role = std::collections::BTreeMap::new();
    let mut all_identities = std::collections::BTreeSet::new();
    for object in ledger["objects"].as_array().expect("objects array") {
        let role = object["stable_role"].as_str().expect("stable role");
        assert!(by_role.insert(role, object).is_none(), "duplicate role {role}");
        let expected = match role {
            "callback.with" => &["runtime_id", "stable_role", "kind", "carrier_id"][..],
            "func.callback" | "func.body" => {
                &["runtime_id", "stable_role", "kind", "func_id", "carrier_id"][..]
            }
            "dict.result" => &["runtime_id", "stable_role", "kind", "value_id"][..],
            _ => &["runtime_id", "stable_role", "kind"][..],
        };
        assert_eq!(p1341_keys(object), p1341_expected_keys(expected));
        let runtime = p1341_nonempty_prefixed(&object["runtime_id"], "r-");
        assert!(all_identities.insert(runtime));
        for (field, prefix) in
            [("func_id", "func:"), ("carrier_id", "carrier:"), ("value_id", "value:")]
        {
            if let Some(value) = object.get(field) {
                assert!(all_identities.insert(p1341_nonempty_prefixed(value, prefix)));
            }
        }
    }
    assert_eq!(
        by_role.keys().copied().collect::<std::collections::BTreeSet<_>>(),
        required_roles.into_iter().collect(),
        "all nine externally pinned roles must occur exactly once",
    );

    let runtime_role: std::collections::BTreeMap<_, _> = by_role
        .iter()
        .map(|(role, object)| (object["runtime_id"].as_str().unwrap(), *role))
        .collect();
    let events = ledger["events"].as_array().expect("events array");
    assert_eq!(events.len(), 9, "closed R5 event cardinality");
    for (seq, event) in events.iter().enumerate() {
        assert_eq!(event["seq"].as_u64(), Some(seq as u64));
        assert!(runtime_role.contains_key(event["subject"].as_str().unwrap_or("")));
        let expected = match event["kind"].as_str().expect("event kind") {
            "execution-start" | "execution-end" => &["seq", "kind", "subject"][..],
            "feature-origin" => {
                &["seq", "kind", "subject", "origin", "features", "witness_ordinal"][..]
            }
            "callback-dispatch" => {
                &["seq", "kind", "subject", "func_ref", "body_func_ref", "phase"][..]
            }
            "function-invoked" | "body-started" => {
                &["seq", "kind", "subject", "carrier_ref", "producer_role", "origin"][..]
            }
            "counter-occurrence" => &[
                "seq",
                "kind",
                "subject",
                "key",
                "action",
                "occurrence_ordinal",
                "span_role",
                "walked_content_ref",
                "location",
                "snapshot_id",
                "raw_span",
            ][..],
            "dict-value" => &["seq", "kind", "subject", "typed_value", "typed_path"][..],
            other => panic!("unknown P1341 event kind {other}"),
        };
        assert_eq!(p1341_keys(event), p1341_expected_keys(expected));
    }

    let one = |kind: &str| {
        let found: Vec<_> = events.iter().filter(|event| event["kind"] == kind).collect();
        assert_eq!(found.len(), 1, "one {kind}");
        found[0]
    };
    let role_of = |reference: &serde_json::Value| {
        runtime_role
            .get(reference.as_str().expect("runtime reference"))
            .copied()
            .expect("reference to an object in this cell")
    };
    assert_eq!(role_of(&events[0]["subject"]), "execution");
    assert_eq!(role_of(&events[8]["subject"]), "execution");

    let features: Vec<_> = events
        .iter()
        .filter(|event| event["kind"] == "feature-origin")
        .collect();
    assert_eq!(features.len(), 2);
    assert_eq!(
        (
            role_of(&features[0]["subject"]),
            &features[0]["origin"],
            &features[0]["features"],
            features[0]["witness_ordinal"].as_u64()
        ),
        (
            "context.default",
            &serde_json::json!("default"),
            &serde_json::json!([]),
            Some(0)
        )
    );
    assert_eq!(
        (
            role_of(&features[1]["subject"]),
            &features[1]["origin"],
            &features[1]["features"],
            features[1]["witness_ordinal"].as_u64()
        ),
        (
            "context.selected",
            &serde_json::json!("entrypoint-supplied"),
            &serde_json::json!(["html"]),
            Some(1)
        )
    );

    let dispatch = one("callback-dispatch");
    assert_eq!(
        (
            role_of(&dispatch["subject"]),
            role_of(&dispatch["func_ref"]),
            role_of(&dispatch["body_func_ref"]),
            dispatch["phase"].as_str()
        ),
        ("callback.with", "func.callback", "func.body", Some("eval"))
    );
    let invoked = one("function-invoked");
    assert_eq!(
        (
            role_of(&invoked["subject"]),
            role_of(&invoked["carrier_ref"]),
            invoked["producer_role"].as_str(),
            invoked["origin"].as_str()
        ),
        (
            "func.callback",
            "callback.with",
            Some("callback.with"),
            Some("callback-dispatch")
        )
    );
    let body = one("body-started");
    assert_eq!(
        (
            role_of(&body["subject"]),
            role_of(&body["carrier_ref"]),
            body["producer_role"].as_str(),
            body["origin"].as_str()
        ),
        ("func.body", "callback.with", Some("callback.with"), Some("body-entry"))
    );

    let counter = one("counter-occurrence");
    assert_eq!(
        (
            role_of(&counter["subject"]),
            counter["key"].as_str(),
            counter["action"].as_str(),
            counter["occurrence_ordinal"].as_u64(),
            counter["span_role"].as_str(),
            role_of(&counter["walked_content_ref"])
        ),
        (
            "counter.step.current",
            Some("chapter"),
            Some("step"),
            Some(0),
            Some("counter-consumer"),
            "content.walked"
        )
    );
    let location = p1341_nonempty_prefixed(&counter["location"], "location:");
    let snapshot = p1341_nonempty_prefixed(&counter["snapshot_id"], "snapshot:");
    assert_ne!(location, snapshot);
    assert!(all_identities.insert(location));
    assert!(all_identities.insert(snapshot));
    assert_eq!(
        p1341_keys(&counter["raw_span"]),
        p1341_expected_keys(&["source", "start", "end", "lexical_role"])
    );
    assert!(all_identities
        .insert(p1341_nonempty_prefixed(&counter["raw_span"]["source"], "source:")));
    assert_eq!(counter["raw_span"]["lexical_role"], "counter-consumer");
    let start = counter["raw_span"]["start"].as_u64().expect("span start") as usize;
    let end = counter["raw_span"]["end"].as_u64().expect("span end") as usize;
    assert!(
        start < end && source.get(start..end) == Some("step"),
        "span must bind the real lexical consumer"
    );

    let dict = one("dict-value");
    assert_eq!(role_of(&dict["subject"]), "dict.result");
    assert_eq!(dict["typed_path"], serde_json::json!([]));
    assert_eq!(
        dict["typed_value"],
        serde_json::json!({"tag":"dict","pairs":[
            ["outer",{"tag":"dict","pairs":[["a",{"tag":"int","value":0}],["b",{"tag":"str","value":"x"}]]}],
            ["done",{"tag":"bool","value":true}]
        ]})
    );

    let expected_edges = [
        ("execution", "context.default"),
        ("execution", "context.selected"),
        ("context.selected", "callback.with"),
        ("callback.with", "func.callback"),
        ("func.callback", "func.body"),
        ("func.body", "counter.step.current"),
        ("counter.step.current", "content.walked"),
        ("content.walked", "dict.result"),
    ];
    let edges: std::collections::BTreeSet<_> = ledger["causal_edges"]
        .as_array()
        .expect("edge array")
        .iter()
        .map(|edge| {
            assert_eq!(p1341_keys(edge), p1341_expected_keys(&["from", "to"]));
            (role_of(&edge["from"]), role_of(&edge["to"]))
        })
        .collect();
    assert_eq!(edges, expected_edges.into_iter().collect());

    let positions: std::collections::BTreeMap<_, _> = events
        .iter()
        .enumerate()
        .map(|(index, event)| (event["kind"].as_str().unwrap(), index))
        .collect();
    assert!(positions["execution-start"] < features[0]["seq"].as_u64().unwrap() as usize);
    assert!(
        (features[1]["seq"].as_u64().unwrap() as usize) < positions["callback-dispatch"]
    );
    assert!(
        positions["callback-dispatch"] < positions["function-invoked"]
            && positions["function-invoked"] < positions["body-started"]
            && positions["body-started"] < positions["counter-occurrence"]
            && positions["counter-occurrence"] < positions["dict-value"]
            && positions["dict-value"] < positions["execution-end"]
    );
}

#[test]
fn p1341_real_closed_ledger_normal_repeat_reverse() {
    let source_text = concat!(
        "#set page(height: 100cm)\n",
        "#counter(\"chapter\").step()\n",
        "#context metadata((outer: (a: counter(heading.where()).get().first(), b: \"x\"), done: true))\n",
    );
    // This one missing test-only facade is the intended initial RED. It must
    // execute the real pipeline and return its append-only runtime ledger; it
    // may not synthesize events from this assertion or from the external oracle.
    for _order in ["normal", "repeat", "reverse"] {
        let ledger = super::p1341_real_ledger_focal();
        p1341_assert_focal_ledger(&ledger, source_text);
    }
}
