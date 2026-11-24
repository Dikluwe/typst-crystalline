use super::{substitute_context_blocks, Arc, Content, HashMap, PRESERVE};
use typst_core::entities::{
    content::SourceContent, elements::context_block::ContextBlockElem, file_id::FileId,
    func::Func, span::Span, value::Value,
};

#[test]
fn substitution_preserves_only_unchanged_owners() {
    let span = Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 14..21);
    let stable = Content::strong(Content::Source(Arc::new(SourceContent {
        body: Content::text("stable"),
        span,
    })));
    let context = Arc::new(ContextBlockElem {
        id: 41,
        closure: Func::native("test", |_context, _args, _world, _file| Ok(Value::None)),
    });
    let changing = Content::strong(Content::ContextBlock(context.clone()));
    let Content::Strong(stable_owner) = &stable else { unreachable!() };
    let Content::Strong(changing_owner) = &changing else { unreachable!() };
    let stable_sequence: Arc<[Content]> = vec![stable.clone()].into();
    let mixed: Arc<[Content]> = vec![Content::Sequence(stable_sequence.clone()), changing.clone()].into();
    let resolved = HashMap::from([(41, Content::text("changed"))]);
    let result = substitute_context_blocks(Content::Sequence(mixed.clone()), &resolved);
    let Content::Sequence(parts) = result else { unreachable!() };
    assert!(!Arc::ptr_eq(&parts, &mixed));
    let [Content::Sequence(prefix), Content::Strong(mapped)] = &*parts else { unreachable!() };
    let [Content::Strong(unchanged)] = &**prefix else { unreachable!() };
    assert_eq!(Arc::ptr_eq(prefix, &stable_sequence), PRESERVE);
    assert_eq!(Arc::ptr_eq(unchanged, stable_owner), PRESERVE);
    let Content::Source(source) = &unchanged.body else { unreachable!() };
    assert_eq!(source.span, span);
    let Content::Source(before) = &stable_owner.body else { unreachable!() };
    assert_eq!(Arc::ptr_eq(source, before), PRESERVE);
    assert!(!Arc::ptr_eq(mapped, changing_owner));
    let Content::Sequence(body) = &mapped.body else { unreachable!() };
    let [Content::ContextBlock(marker), value] = &**body else { unreachable!() };
    assert!(Arc::ptr_eq(marker, &context));
    assert_eq!(value.plain_text(), "changed");
    assert!(matches!(&changing_owner.body, Content::ContextBlock(_)));
}
