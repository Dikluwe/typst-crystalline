//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/syntax_node.md
//! @prompt-hash df97d042
//! @layer L1
//! @updated 2026-09-26

mod errors;
mod representation;

pub(crate) use errors::{NumberingResult, Unnumberable};
pub use errors::{SyntaxError, SyntaxErrorKind};
pub use representation::{LinkedChildren, LinkedNode, Side, SyntaxNode};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{file_id::FileId, span::Span, syntax_kind::SyntaxKind};

    fn text_leaf(s: &str) -> SyntaxNode {
        SyntaxNode::leaf(SyntaxKind::Text, s)
    }

    #[test]
    fn leaf_kind_and_text() {
        let node = text_leaf("hello");
        assert_eq!(node.kind(), SyntaxKind::Text);
        assert_eq!(node.text().as_str(), "hello");
        assert_eq!(node.len(), 5);
        assert!(!node.erroneous());
        assert!(node.errors().is_empty());
    }

    #[test]
    fn leaf_is_empty() {
        let node = SyntaxNode::leaf(SyntaxKind::End, "");
        assert!(node.is_empty());
    }

    #[test]
    fn inner_node_children() {
        let a = text_leaf("a");
        let b = text_leaf("b");
        let parent = SyntaxNode::inner(SyntaxKind::Markup, vec![a, b]);
        assert_eq!(parent.kind(), SyntaxKind::Markup);
        assert_eq!(parent.len(), 2);
        assert_eq!(parent.children().count(), 2);
        assert_eq!(parent.text().as_str(), ""); // inner nodes return empty
    }

    #[test]
    fn inner_node_erroneous_propagates() {
        let good = text_leaf("ok");
        let bad = SyntaxNode::error(SyntaxError::new("oops"), "x");
        let parent = SyntaxNode::inner(SyntaxKind::Markup, vec![good, bad]);
        assert!(parent.erroneous());
        assert_eq!(parent.errors().len(), 1);
        assert_eq!(parent.errors()[0].message.as_str(), "oops");
    }

    #[test]
    fn error_node() {
        let err = SyntaxNode::error(SyntaxError::new("bad token"), "??");
        assert_eq!(err.kind(), SyntaxKind::Error);
        assert_eq!(err.text().as_str(), "??");
        assert_eq!(err.len(), 2);
        assert!(err.erroneous());
        let errors = err.errors();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message.as_str(), "bad token");
    }

    #[test]
    fn syntax_error_new() {
        let e = SyntaxError::new("msg");
        assert_eq!(e.message.as_str(), "msg");
        assert!(e.hints.is_empty());
        assert!(e.span.is_detached());
    }

    #[test]
    fn hint_on_error_node() {
        let mut node = SyntaxNode::error(SyntaxError::new("err"), "x");
        node.hint("try this instead");
        assert_eq!(node.errors()[0].hints.len(), 1);
        assert_eq!(node.errors()[0].hints[0].as_str(), "try this instead");
    }

    #[test]
    fn placeholder() {
        let p = SyntaxNode::placeholder(SyntaxKind::Text);
        assert_eq!(p.kind(), SyntaxKind::Text);
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn into_text_leaf() {
        let node = text_leaf("world");
        assert_eq!(node.into_text().as_str(), "world");
    }

    #[test]
    fn into_text_inner() {
        let node = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![text_leaf("foo"), text_leaf("bar")],
        );
        assert_eq!(node.into_text().as_str(), "foobar");
    }

    #[test]
    fn spanless_eq() {
        let a = text_leaf("hi");
        let b = text_leaf("hi");
        let c = text_leaf("bye");
        assert!(a.spanless_eq(&b));
        assert!(!a.spanless_eq(&c));
    }

    #[test]
    fn linked_node_basics() {
        let leaf = text_leaf("x");
        let linked = LinkedNode::new(&leaf);
        assert_eq!(linked.kind(), SyntaxKind::Text);
        assert_eq!(linked.offset(), 0);
        assert_eq!(linked.range(), 0..1);
        assert!(linked.parent().is_none());
    }

    #[test]
    fn linked_node_children_iteration() {
        let a = text_leaf("ab");
        let b = text_leaf("cd");
        let root = SyntaxNode::inner(SyntaxKind::Markup, vec![a, b]);
        let linked = LinkedNode::new(&root);
        let kids: Vec<_> = linked.children().collect();
        assert_eq!(kids.len(), 2);
        assert_eq!(kids[0].offset(), 0);
        assert_eq!(kids[1].offset(), 2);
    }

    #[test]
    fn default_node() {
        let d = SyntaxNode::default();
        assert_eq!(d.kind(), SyntaxKind::End);
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn facade_contains_only_reexports_and_tests() {
        let source = include_str!("syntax_node.rs");
        assert!(source.lines().any(|line| line == "mod representation;"));
        assert!(source.lines().any(|line| line == "mod errors;"));
        assert!(!source.lines().any(|line| line.starts_with("pub struct SyntaxNode")));
    }

    #[test]
    fn error_category_and_clone_hints_stay_independent() {
        let mut original = SyntaxNode::error(
            SyntaxError::with_kind(
                "bad escape",
                SyntaxErrorKind::InvalidUnicodeCodepoint,
            ),
            "é",
        );
        let mut clone = original.clone();
        clone.hint("new hint");
        assert!(original.errors()[0].hints.is_empty());
        assert_eq!(clone.errors()[0].kind, SyntaxErrorKind::InvalidUnicodeCodepoint);
        assert_eq!(clone.errors()[0].hints[0].as_str(), "new hint");
        assert!(!original.spanless_eq(&clone));
        original.hint("new hint");
        assert!(original.spanless_eq(&clone));
    }

    fn numbered_tree() -> SyntaxNode {
        let mut root = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![
                text_leaf("é"),
                SyntaxNode::leaf(SyntaxKind::Space, " "),
                text_leaf("x"),
                SyntaxNode::inner(SyntaxKind::Markup, vec![text_leaf("yz")]),
            ],
        );
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        root.numberize(id, 1..1000).unwrap();
        root
    }

    #[test]
    fn linked_utf8_offsets_siblings_and_leaf_boundaries() {
        let root = numbered_tree();
        let linked = LinkedNode::new(&root);
        let kids: Vec<_> = linked.children().collect();
        assert_eq!(
            kids.iter().map(LinkedNode::offset).collect::<Vec<_>>(),
            vec![0, 2, 3, 4]
        );
        assert_eq!(kids[2].prev_sibling().unwrap().text_str(), "é");
        assert_eq!(kids[2].prev_sibling_with_trivia().unwrap().kind(), SyntaxKind::Space);
        assert_eq!(kids[0].next_sibling().unwrap().text_str(), "x");
        assert_eq!(kids[0].next_sibling_with_trivia().unwrap().offset(), 2);
        assert_eq!(linked.leaf_at(2, Side::Before).unwrap().text_str(), "é");
        assert_eq!(linked.leaf_at(2, Side::After).unwrap().kind(), SyntaxKind::Space);
        assert_eq!(linked.leaf_at(4, Side::Before).unwrap().text_str(), "x");
        assert_eq!(linked.leaf_at(4, Side::After).unwrap().text_str(), "yz");
        assert_eq!(kids[2].next_leaf().unwrap().text_str(), "yz");
        assert_eq!(kids[3].leftmost_leaf().unwrap().prev_leaf().unwrap().text_str(), "x");
        assert_eq!(linked.rightmost_leaf().unwrap().range(), 4..6);
        assert!(linked.leaf_at(6, Side::After).is_none());
    }

    #[test]
    fn linked_double_ended_iterator_preserves_parent_and_indices() {
        let root = numbered_tree();
        let linked = LinkedNode::new(&root);
        let mut children: LinkedChildren<'_> = linked.children();
        assert_eq!(children.len(), 4);
        let back = children.next_back().unwrap();
        assert_eq!((back.index(), back.offset()), (3, 4));
        let front = children.next().unwrap();
        assert_eq!((front.index(), front.offset()), (0, 0));
        assert_eq!(front.parent().unwrap().get(), &root);
        assert_eq!(children.next_back().unwrap().range(), 3..4);
        assert_eq!(children.next().unwrap().range(), 2..3);
        assert_eq!(children.size_hint(), (0, Some(0)));
        assert!(children.next().is_none());
        assert!(children.next_back().is_none());
    }

    #[test]
    fn linked_find_retains_numbering_and_full_span_identity() {
        let root = numbered_tree();
        let linked = LinkedNode::new(&root);
        let nested = root.children().nth(3).unwrap().children().next().unwrap();
        assert_eq!(linked.find(nested.span()).unwrap().range(), 4..6);
        assert_eq!(linked.find(root.span()).unwrap().get(), &root);
        let other_id = FileId::from_raw(std::num::NonZeroU16::new(2).unwrap());
        let other_span = Span::from_number(other_id, nested.span().number()).unwrap();
        assert!(linked.find(other_span).is_none());
    }

    #[test]
    fn representation_clone_and_spanless_equality_are_preserved() {
        let original = numbered_tree();
        let mut clone = original.clone();
        clone.synthesize(Span::detached());
        assert!(!original.span().is_detached());
        assert!(clone.children().all(|child| child.span().is_detached()));
        assert!(original.spanless_eq(&clone));
        assert_ne!(original, clone);
        assert_eq!(clone.into_text().as_str(), "é xyz");
    }

    #[test]
    fn numbering_failure_type_remains_available_through_facade() {
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let result: NumberingResult = text_leaf("x").numberize(id, 1..1);
        assert_eq!(result, Err(Unnumberable));
        assert_eq!(Unnumberable.to_string(), "cannot number within this interval");
    }
}
