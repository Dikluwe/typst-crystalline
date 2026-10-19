    #[test]
    fn v16_c_update_hint_inspects_metadata_content_and_arrays() {
        for field in ["state(\"k\",0).update(1)", "(state(\"k\",0).update(1),)"] {
            let source = format!("#{{ metadata({field}); return \"x\" }}");
            let diagnostic = warning(&source);
            assert!(has_update_hint(&diagnostic), "{source}: {diagnostic:?}");
        }
    }
    #[test]
    fn v16_c_update_hint_preserves_metadata_dictionary_opacity() {
        let diagnostic = warning("#{ metadata((value:state(\"k\",0).update(1))); return \"x\" }");
        assert!(!has_update_hint(&diagnostic));
    }
