
#[cfg(test)]
mod lab_substitution_preservation {
    use super::*;
    fn context(id:u64)->Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {id, closure: typst_core::entities::func::Func::native("dummy", |_ctx,_args,_world,_file|Ok(Value::None))}))
    }
    #[test]
    fn inert_sequence_retains_exact_owner() {
        let original=Content::Sequence(vec![Content::strong(Content::text("intact")),Content::Space,Content::text("tail")].into());
        let result=substitute_context_blocks(original.clone(), &HashMap::from([(999,Content::text("irrelevant"))]));
        let (Content::Sequence(a),Content::Sequence(b))=(&original,&result) else {panic!("sequence")};
        assert!(Arc::ptr_eq(a,b),"pure substitution reconstructed inert owner");
    }
    #[test]
    fn context_replacements_remain_fresh_and_original_unchanged() {
        let original=Content::strong(context(9));
        let a=substitute_context_blocks(original.clone(),&HashMap::from([(9,Content::text("FIRST"))]));
        let b=substitute_context_blocks(original.clone(),&HashMap::from([(9,Content::text("SECOND"))]));
        assert!(a.plain_text().contains("FIRST"));assert!(b.plain_text().contains("SECOND"));
        assert!(!b.plain_text().contains("FIRST"));assert!(!original.plain_text().contains("FIRST"));
    }
}
