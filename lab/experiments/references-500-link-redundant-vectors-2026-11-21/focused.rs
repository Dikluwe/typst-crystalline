use typst_core::entities::{
    elements::pdf_artifact::ArtifactKind,
    geometry::{FillRule, ShapeKind},
    layout_types::{FrameItem, LinkTarget, Point, Pt, SemanticKind, SemanticPlacement, Size},
};

fn rect(x: f64) -> FrameItem {
    FrameItem::Shape {
        pos: Point { x: Pt(x), y: Pt(1.0) }, kind: ShapeKind::Rect, width: 3.0,
        height: 4.0, fill: None, stroke: None, fill_rule: FillRule::NonZero,
        parent_bbox_at_emit: None,
    }
}

fn link(target: &LinkTarget, items: Vec<FrameItem>) -> FrameItem {
    FrameItem::Link {
        target: target.clone(), items, pos: Point { x: Pt(0.0), y: Pt(0.0) },
        size: Size { width: Pt(1.0), height: Pt(1.0) },
    }
}

fn envelope(items: Vec<FrameItem>) -> FrameItem {
    FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        alt: Some("lexical".into()), placement: SemanticPlacement::Inline, items,
    }
}

#[test]
fn retains_parent_and_unchanged_child_buffers() {
    let outer = LinkTarget::Url("https://example.test/one".into());
    let inner = LinkTarget::Url("https://example.test/two".into());
    let mut sibling = Vec::with_capacity(17);
    sibling.push(rect(1.0));
    let sibling_before = sibling.as_ptr();
    let mut parent = Vec::with_capacity(23);
    parent.push(envelope(sibling));
    parent.push(envelope(vec![link(&outer, vec![rect(2.0), rect(3.0)]), link(&inner, vec![rect(4.0)])]));
    let parent_before = parent.as_ptr();
    let actual = remove_redundant(parent, &outer);
    assert_eq!(actual.as_ptr(), parent_before, "parent with unchanged direct membership");
    assert_eq!(actual.capacity(), 23);
    let [FrameItem::Semantic { items: sibling, .. }, FrameItem::Semantic { items: changed, .. }] = &*actual else {
        panic!("wrappers and order must survive");
    };
    assert_eq!(sibling.as_ptr(), sibling_before);
    assert_eq!(sibling.capacity(), 17);
    let expected = vec![rect(2.0), rect(3.0), link(&inner, vec![rect(4.0)])];
    assert_eq!(format!("{changed:?}"), format!("{expected:?}"));
}

#[test]
fn keeps_order_and_never_opens_a_distinct_link() {
    let outer = LinkTarget::Url("https://example.test/one".into());
    let inner = LinkTarget::Url("https://example.test/two".into());
    let distinct = link(&inner, vec![link(&outer, vec![rect(9.0)])]);
    let input = vec![rect(0.0), link(&outer, vec![rect(1.0), link(&outer, vec![rect(2.0)])]),
        distinct.clone(), link(&outer, vec![]), rect(3.0)];
    let expected = vec![rect(0.0), rect(1.0), rect(2.0), distinct, rect(3.0)];
    let actual = remove_redundant(input, &outer);
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}
