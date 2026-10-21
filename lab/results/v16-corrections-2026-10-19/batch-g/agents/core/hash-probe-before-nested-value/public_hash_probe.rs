//! Evidence-only executable using the actual public crystalline-core artifact.
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroU16;
use std::sync::Arc;
use typst_core::entities::content::{Content, LocatedContent, SourceContent};
use typst_core::entities::elements::context_block::ContextBlockElem;
use typst_core::entities::file_id::FileId;
use typst_core::entities::func::Func;
use typst_core::entities::html::{HtmlAttrs, HtmlBody, HtmlElem, HtmlFrameElem};
use typst_core::entities::locator::Locator;
use typst_core::entities::span::Span;

fn std_hash(content: &Content) -> u64 {
    let mut hash = DefaultHasher::new();
    content.hash(&mut hash);
    hash.finish()
}
fn strong(mut body: Content, depth: usize) -> Content {
    for _ in 0..depth { body = Content::strong(body); }
    body
}
fn context(name: &'static str) -> Content {
    Content::ContextBlock(Arc::new(ContextBlockElem {
        id: 7,
        closure: Func::native(name, |_, _, _, _| panic!("hash probe executed closure")),
    }))
}
fn leaf(family: &str, second: bool) -> Content {
    match family {
        "Context" => context(if second { "second" } else { "first" }),
        "Space" if second => Content::SpaceFromNewline,
        "Space" => Content::Space,
        _ => Content::text("A"),
    }
}
fn family(family: &str, body: Content, deep: bool) -> Content {
    let inner = if deep { strong(body, 6) } else { body };
    match family {
        "Strong" => Content::strong(inner),
        "Sequence" => Content::Sequence(vec![Content::text("prefix"),inner].into()),
        "Title" => Content::title(inner),
        "Frame" => Content::HtmlFrame(Arc::new(HtmlFrameElem { body: inner })),
        "Html" => Content::HtmlElem(Arc::new(HtmlElem::new(
            "div".into(), None, HtmlBody::Content(Box::new(inner)),
        ))),
        "Context" | "Space" if deep => Content::strong(inner),
        "Context" | "Space" => inner,
        _ => panic!("unknown evidence family"),
    }
}
fn transport(mut body: Content, carrier: &str, depth: usize, locator: &mut Locator) -> Content {
    for _ in 0..depth {
        body = match carrier {
            "Source" => Content::Source(Arc::new(SourceContent {
                body,
                span: Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()),2..3),
            })),
            "Located" => Content::Located(Arc::new(LocatedContent {
                body, location: locator.next(),
            })),
            _ => panic!("unknown transport"),
        };
    }
    body
}
fn record(family: &str, placement: &str, carrier: &str, depth: usize, a: &Content, b: &Content) {
    let ah=typst_core::entities::content_hash::hash_content(a);
    let bh=typst_core::entities::content_hash::hash_content(b);
    let ash=std_hash(a);let bsh=std_hash(b);
    println!("{{\"family\":{:?},\"placement\":{:?},\"carrier\":{:?},\"depth\":{},\"equal\":{},\"debug_a\":{:?},\"debug_b\":{:?},\"public_hash_a\":\"{}\",\"public_hash_b\":\"{}\",\"std_hash_a\":\"{}\",\"std_hash_b\":\"{}\",\"public_hash_equal\":{},\"std_hash_equal\":{}}}",
        family,placement,carrier,depth,a==b,format!("{a:?}"),format!("{b:?}"),ah,bh,ash,bsh,ah==bh,ash==bsh);
}
fn main() {
    let mut locator=Locator::new();
    for name in ["Strong","Sequence","Title","Frame","Html","Context","Space"] {
        for placement in ["root","leaf"] {
            for carrier in ["Source","Located"] {
                for depth in 0..=12 {
                    let a=family(name,leaf(name,false),placement=="leaf");
                    let b=if placement=="root" {
                        transport(family(name,leaf(name,true),false),carrier,depth,&mut locator)
                    } else {
                        family(name,transport(leaf(name,true),carrier,depth,&mut locator),true)
                    };
                    record(name,placement,carrier,depth,&a,&b);
                }
            }
        }
    }
    let attrs_a:HtmlAttrs=[("id".into(),"a".into()),("class".into(),"b".into())].into_iter().collect();
    let attrs_b:HtmlAttrs=[("class".into(),"b".into()),("id".into(),"a".into())].into_iter().collect();
    let make=|attrs|Content::HtmlElem(Arc::new(HtmlElem::new("div".into(),Some(attrs),HtmlBody::Content(Box::new(Content::text("A"))))));
    record("HtmlAttrsOrder","root","none",0,&make(attrs_a),&make(attrs_b));
}
