    #[test]
    fn visual_only_and_source_styles_preserve_owners() {
        use typst_core::entities::content::{RealizedContent,SourceContent};
        use typst_core::entities::span::Span;
        let visual=Content::strong(Content::text("visible"));
        let original=Content::Realized(Arc::new(RealizedContent{original:context(8),visual}));
        let result=substitute_context_blocks(original.clone(),&HashMap::from([(8,Content::text("hidden"))]));
        let (Content::Realized(a),Content::Realized(b))=(&original,&result) else {panic!("realized")};
        assert!(Arc::ptr_eq(a,b));assert_eq!(result.plain_text(),"visible");
        let original=Content::Source(Arc::new(SourceContent{body:Content::styled(Content::text("styled"),typst_core::entities::style::Styles::default()),span:Span::detached()}));
        let result=substitute_context_blocks(original.clone(),&HashMap::new());
        let (Content::Source(a),Content::Source(b))=(&original,&result) else {panic!("source")};
        assert!(Arc::ptr_eq(a,b));
    }
    #[test]
    fn bounded_proof_refuses_context_and_large_inputs() {
        assert!(!lab_substitution_inert(&context(8)));
        assert!(!lab_substitution_inert(&Content::text("x".repeat(17000))));
        let mut deep=Content::text("x");for _ in 0..34 {deep=Content::strong(deep);}
        assert!(!lab_substitution_inert(&deep));
        let wide=Content::Sequence(vec![Content::Space;513].into());
        assert!(!lab_substitution_inert(&wide));
        let a=Content::strong(Content::text("same"));let b=Content::strong(Content::text("same"));
        let a=substitute_context_blocks(a,&HashMap::new());let b=substitute_context_blocks(b,&HashMap::new());
        let (Content::Strong(a),Content::Strong(b))=(a,b) else {panic!("strong")};
        assert!(!Arc::ptr_eq(&a,&b));
    }
