// Evidence-only draft for the exclusive entities/content/identity.rs owner.
// Not compiled, installed or authorized for implementation by this artifact.
#[cfg(test)]
mod carrier_debug_depth_tests {
    use super::Content;
    use crate::entities::content::{LocatedContent,RealizedContent,ShowRecipeContent,SourceContent};
    use crate::entities::elements::context_block::ContextBlockElem;
    use crate::entities::file_id::FileId;
    use crate::entities::func::Func;
    use crate::entities::html::{HtmlAttrs,HtmlBody,HtmlElem,HtmlFrameElem};
    use crate::entities::locator::Locator;
    use crate::entities::show::{NodeKind,Selector,ShowRule,Transformation};
    use crate::entities::span::Span;
    use crate::entities::style_chain::StyleChain;
    use std::hash::{DefaultHasher,Hash,Hasher};
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn strong(mut body:Content, depth:usize)->Content {
        for _ in 0..depth { body=Content::strong(body); }
        body
    }
    fn source(body:Content)->Content {
        Content::Source(Arc::new(SourceContent {
            body,
            span:Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()),2..3),
        }))
    }
    fn context()->Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {
            id:91,
            closure:Func::native("must-not-execute",|_,_,_,_|panic!("Debug executed Context")),
        }))
    }
    fn carrier(body:Content, kind:usize, locator:&mut Locator)->Content {
        match kind {
            0=>source(body),
            1=>Content::Located(Arc::new(LocatedContent {body,location:locator.next()})),
            2=>Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,textual:true,context_id_floor:73,guards:vec![31],
                styles:StyleChain::default_chain(),
                rules:Arc::from([ShowRule {
                    id:31,selector:Selector::NodeKind(NodeKind::Heading),
                    transform:Transformation::Func(Func::native("never-run-recipe",|_,_,_,_|panic!("Debug executed recipe"))),
                }]),
                file:FileId::from_raw(NonZeroU16::new(1).unwrap()),
            })),
            3=>Content::Realized(Arc::new(RealizedContent {
                original:Content::sequence(vec![Content::text("ORIGINAL_MUST_NOT_APPEAR"),context()]),
                visual:body,
            })),
            _=>panic!("unknown test carrier"),
        }
    }
    fn family(name:usize, body:Content)->Content {
        // All leaves remain at the visible-depth boundary, never beyond it.
        match name {
            0=>strong(body,7),
            1=>Content::Sequence(vec![Content::text("prefix"),strong(body,6)].into()),
            2=>Content::title(strong(body,6)),
            3=>Content::HtmlFrame(Arc::new(HtmlFrameElem {body:strong(body,6)})),
            4=>Content::HtmlElem(Arc::new(HtmlElem::new("div".into(),None,HtmlBody::Content(Box::new(strong(body,6)))))),
            5=>strong(body,7), // Same-id Context mechanics, not its language equality.
            _=>panic!("unknown test family"),
        }
    }
    fn std_hash(content:&Content)->u64 {
        let mut state=DefaultHasher::new();content.hash(&mut state);state.finish()
    }

    #[test]
    fn carrier_debug_transparency_reaches_the_same_visible_depth_boundary() {
        let mut locator=Locator::new();
        for family_id in 0..=5 {
            let leaf=if family_id==5 {context()} else {Content::text("VISIBLE")};
            let ordinary=family(family_id,leaf.clone());
            let expected=format!("{ordinary:?}");
            assert!(!expected.contains("..."));
            for kind in 0..4 {
                let transported=family(family_id,carrier(leaf.clone(),kind,&mut locator));
                assert_eq!(ordinary,transported,"mechanical equality is unchanged by transport");
                assert_eq!(format!("{transported:?}"),expected,"carrier must not consume visible depth");
            }
        }
    }

    #[test]
    fn carrier_hash_compatibility_covers_selected_old_and_new_content_families() {
        let mut locator=Locator::new();
        for family_id in 0..=5 {
            let leaf=if family_id==5 {context()} else {Content::text("VISIBLE")};
            let ordinary=family(family_id,leaf.clone());
            for kind in 0..4 {
                let mut transported_leaf=leaf.clone();
                for _ in 0..12 {transported_leaf=carrier(transported_leaf,kind,&mut locator);}
                let transported=family(family_id,transported_leaf);
                assert_eq!(ordinary,transported);
                assert_eq!(crate::entities::content_hash::hash_content(&ordinary),crate::entities::content_hash::hash_content(&transported));
                assert_eq!(std_hash(&ordinary),std_hash(&transported));
            }
        }
        // Ordered attributes are identical here; no order-insensitive Hash promise.
        let attrs:HtmlAttrs=[("id".into(),"a".into()),("class".into(),"b".into())].into_iter().collect();
        let html=Content::HtmlElem(Arc::new(HtmlElem::new("div".into(),Some(attrs),HtmlBody::Content(Box::new(Content::text("A"))))));
        assert_eq!(html,html.clone());
        assert_eq!(crate::entities::content_hash::hash_content(&html),crate::entities::content_hash::hash_content(&html.clone()));
        assert_eq!(std_hash(&html),std_hash(&html.clone()));
        assert_eq!(Content::Space,Content::SpaceFromNewline);
        assert_eq!(std_hash(&Content::Space),std_hash(&Content::SpaceFromNewline));
    }

    #[test]
    fn debug_retains_visible_max_depth_and_iterates_carriers_without_consuming_originals() {
        let visible=strong(Content::text("VISIBLE"),7);
        let too_deep=strong(Content::text("VISIBLE"),8);
        assert!(format!("{visible:?}").contains("VISIBLE"));
        let truncated=format!("{too_deep:?}");
        assert!(truncated.contains("..."));
        assert!(!truncated.contains("VISIBLE"),"visible structures still obey MAX_DEPTH8");
        let realized=Arc::new(RealizedContent {
            original:Content::sequence(vec![Content::text("ORIGINAL_MUST_NOT_APPEAR"),context()]),
            visual:visible.clone(),
        });
        let prepared=Arc::new(LocatedContent {
            location:Locator::new().next(),body:Content::Realized(Arc::clone(&realized)),
        });
        let mut tree=Content::Located(Arc::clone(&prepared));
        let mut locator=Locator::new();
        for depth in 0..512 {tree=carrier(tree,depth%4,&mut locator);}
        let debug=format!("{tree:?}");
        assert_eq!(debug,format!("{visible:?}"));
        assert!(!debug.contains("ORIGINAL_MUST_NOT_APPEAR"));
        assert!(matches!(&prepared.body,Content::Realized(e) if Arc::ptr_eq(e,&realized)));
        assert_eq!(realized.visual,visible);
        assert!(realized.original.plain_text().contains("ORIGINAL_MUST_NOT_APPEAR"));
        // All stored Context/recipe closures panic if executed; Debug must read data only.
    }
}
