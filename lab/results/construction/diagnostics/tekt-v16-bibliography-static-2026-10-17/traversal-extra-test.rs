
    #[test]
    fn bibliography_static_visit_agrees_with_existing_container_fields() {
        for input in cases() {
            let mut expected = Vec::new();
            input.map_content(&mut |c| {
                if let Content::Text(t) = c { expected.push(t.to_string()); }
                Ok(None)
            }).unwrap();
            let mut actual = Vec::new();
            input.visit_content_static(&mut |c| {
                if let Content::Text(t) = c { actual.push(t.to_string()); }
                true
            });
            assert_eq!(actual, expected, "{}", input.elem_name());
        }
        let input = Content::Sequence(vec![
            Content::text("first"),
            Content::Par { body: Box::new(Content::text("pruned")) },
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("hidden"), visual: Content::text("visible"),
            })),
            Content::text("last"),
        ].into());
        let mut seen = Vec::new();
        input.visit_content_static(&mut |c| {
            if let Content::Text(t) = c { seen.push(t.to_string()); }
            !matches!(c, Content::Par { .. })
        });
        assert_eq!(seen, ["first", "visible", "last"]);
    }
