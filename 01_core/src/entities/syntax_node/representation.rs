//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/syntax_node/representation.md
//! @prompt-hash 532721ec
//! @layer L1
//! @updated 2026-09-26

use std::fmt::{self, Debug, Formatter};
use std::ops::Range;
use std::sync::Arc;

use super::{NumberingResult, SyntaxError, Unnumberable};
use crate::entities::file_id::FileId;
use crate::entities::span::Span;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_text::SyntaxText;

#[path = "linked.rs"]
mod linked;
pub use linked::{LinkedChildren, LinkedNode, Side};

/// A node in the untyped syntax tree.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SyntaxNode(NodeKind);

/// The three internal representations.
#[derive(Clone, Eq, PartialEq, Hash)]
enum NodeKind {
    /// A leaf node.
    Leaf(LeafNode),
    /// A reference-counted inner node.
    Inner(Arc<InnerNode>),
    /// An error node.
    Error(Arc<ErrorNode>),
}

impl SyntaxNode {
    /// Create a new leaf node.
    pub fn leaf(kind: SyntaxKind, text: impl Into<SyntaxText>) -> Self {
        Self(NodeKind::Leaf(LeafNode::new(kind, text)))
    }

    /// Create a new inner node with children.
    pub fn inner(kind: SyntaxKind, children: Vec<SyntaxNode>) -> Self {
        Self(NodeKind::Inner(Arc::new(InnerNode::new(kind, children))))
    }

    /// Create a new error node.
    pub fn error(error: SyntaxError, text: impl Into<SyntaxText>) -> Self {
        Self(NodeKind::Error(Arc::new(ErrorNode::new(error, text))))
    }

    /// Create a dummy node of the given kind.
    ///
    /// Panics if `kind` is `SyntaxKind::Error`.
    #[track_caller]
    pub fn placeholder(kind: SyntaxKind) -> Self {
        assert!(!matches!(kind, SyntaxKind::Error), "cannot create error placeholder");
        Self(NodeKind::Leaf(LeafNode {
            kind,
            text: SyntaxText::new(),
            span: Span::detached(),
        }))
    }

    /// The type of the node.
    pub fn kind(&self) -> SyntaxKind {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.kind,
            NodeKind::Inner(inner) => inner.kind,
            NodeKind::Error(_) => SyntaxKind::Error,
        }
    }

    /// Return `true` if the length is 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The byte length of the node in the source text.
    pub fn len(&self) -> usize {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.len(),
            NodeKind::Inner(inner) => inner.len,
            NodeKind::Error(node) => node.len(),
        }
    }

    /// The span of the node.
    pub fn span(&self) -> Span {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.span,
            NodeKind::Inner(inner) => inner.span,
            NodeKind::Error(node) => node.error.span,
        }
    }

    /// The text of the node if it is a leaf or error node.
    ///
    /// Returns an empty `SyntaxText` if this is an inner node.
    /// Clone is O(1) — `SyntaxText` is backed by `Arc<str>`.
    pub fn text(&self) -> SyntaxText {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.text.clone(),
            NodeKind::Inner(_) => SyntaxText::new(),
            NodeKind::Error(node) => node.text.clone(),
        }
    }

    /// Extract the concatenated text from the node.
    ///
    /// For leaf and error nodes this is O(1).  For inner nodes it
    /// recursively concatenates all descendant leaf texts — O(n).
    pub fn into_text(self) -> SyntaxText {
        match self.0 {
            NodeKind::Leaf(leaf) => leaf.text,
            NodeKind::Inner(inner) => {
                let mut s = String::new();
                for child in inner.children.iter().cloned() {
                    let t = child.into_text();
                    s.push_str(t.as_str());
                }
                SyntaxText::from(s)
            }
            NodeKind::Error(node) => node.text.clone(),
        }
    }

    /// The text content as a string slice, borrowed from the node.
    ///
    /// Unlike `text()` which returns an owned `SyntaxText`, this borrows
    /// directly and preserves the lifetime `'a` — needed by AST methods
    /// that return `&'a str`.
    pub fn text_str(&self) -> &str {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.text.as_str(),
            NodeKind::Inner(_) => "",
            NodeKind::Error(node) => node.text.as_str(),
        }
    }

    /// The node's children.
    pub fn children(&self) -> std::slice::Iter<'_, SyntaxNode> {
        match &self.0 {
            NodeKind::Leaf(_) | NodeKind::Error(_) => [].iter(),
            NodeKind::Inner(inner) => inner.children.iter(),
        }
    }

    /// Whether the node or its children contain an error.
    pub fn erroneous(&self) -> bool {
        match &self.0 {
            NodeKind::Leaf(_) => false,
            NodeKind::Inner(inner) => inner.erroneous,
            NodeKind::Error(_) => true,
        }
    }

    /// The error messages for this node and its descendants.
    pub fn errors(&self) -> Vec<SyntaxError> {
        if !self.erroneous() {
            return vec![];
        }

        if let NodeKind::Error(node) = &self.0 {
            vec![node.error.clone()]
        } else {
            self.children()
                .filter(|node| node.erroneous())
                .flat_map(|node| node.errors())
                .collect()
        }
    }

    /// Add a user-presentable hint if this is an error node.
    pub fn hint(&mut self, hint: impl Into<SyntaxText>) {
        if let NodeKind::Error(node) = &mut self.0 {
            Arc::make_mut(node).hint(hint);
        }
    }

    /// Set a synthetic span for the node and all its descendants.
    pub fn synthesize(&mut self, span: Span) {
        match &mut self.0 {
            NodeKind::Leaf(leaf) => leaf.span = span,
            NodeKind::Inner(inner) => Arc::make_mut(inner).synthesize(span),
            NodeKind::Error(node) => Arc::make_mut(node).error.span = span,
        }
    }

    /// Whether the two syntax nodes are the same apart from spans.
    pub fn spanless_eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (NodeKind::Leaf(a), NodeKind::Leaf(b)) => a.spanless_eq(b),
            (NodeKind::Inner(a), NodeKind::Inner(b)) => a.spanless_eq(b),
            (NodeKind::Error(a), NodeKind::Error(b)) => a.spanless_eq(b),
            (NodeKind::Leaf(_), NodeKind::Inner(_))
            | (NodeKind::Leaf(_), NodeKind::Error(_))
            | (NodeKind::Inner(_), NodeKind::Leaf(_))
            | (NodeKind::Inner(_), NodeKind::Error(_))
            | (NodeKind::Error(_), NodeKind::Leaf(_))
            | (NodeKind::Error(_), NodeKind::Inner(_)) => false,
        }
    }
}

impl SyntaxNode {
    /// Convert the child to another kind.
    ///
    /// Don't use this for converting to an error!
    #[track_caller]
    pub(crate) fn convert_to_kind(&mut self, kind: SyntaxKind) {
        debug_assert!(!kind.is_error());
        match &mut self.0 {
            NodeKind::Leaf(leaf) => leaf.kind = kind,
            NodeKind::Inner(inner) => Arc::make_mut(inner).kind = kind,
            NodeKind::Error(_) => panic!("cannot convert error"),
        }
    }

    /// Convert the child to an error, if it isn't already one.
    pub(crate) fn convert_to_error(&mut self, message: impl Into<SyntaxText>) {
        if !self.kind().is_error() {
            let text = std::mem::take(self).into_text();
            *self = SyntaxNode::error(SyntaxError::new(message), text);
        }
    }

    /// Convert the child to an error stating that the given thing was
    /// expected, but the current kind was found.
    pub(crate) fn expected(&mut self, expected: &str) {
        let kind = self.kind();
        self.convert_to_error(format!("expected {expected}, found {}", kind.name()));
        if kind.is_keyword() && matches!(expected, "identifier" | "pattern") {
            let text = self.text();
            self.hint(format!(
                "keyword `{text}` is not allowed as an identifier; try `{text}_` instead",
            ));
        }
    }

    /// Convert the child to an error stating it was unexpected.
    pub(crate) fn unexpected(&mut self) {
        self.convert_to_error(format!("unexpected {}", self.kind().name()));
    }

    /// Assign spans to each node.
    pub(crate) fn numberize(
        &mut self,
        id: FileId,
        within: Range<u64>,
    ) -> NumberingResult {
        if within.start >= within.end {
            return Err(Unnumberable);
        }

        let midpoint = within.start + (within.end - within.start) / 2;
        let mid = Span::from_number(id, midpoint).ok_or(Unnumberable)?;
        match &mut self.0 {
            NodeKind::Leaf(leaf) => leaf.span = mid,
            NodeKind::Inner(inner) => Arc::make_mut(inner).numberize(id, None, within)?,
            NodeKind::Error(node) => Arc::make_mut(node).error.span = mid,
        }

        Ok(())
    }

    /// Whether this is a leaf node.
    pub(crate) fn is_leaf(&self) -> bool {
        matches!(self.0, NodeKind::Leaf(_))
    }

    /// The number of descendants, including the node itself.
    pub(crate) fn descendants(&self) -> usize {
        match &self.0 {
            NodeKind::Leaf(_) | NodeKind::Error(_) => 1,
            NodeKind::Inner(inner) => inner.descendants,
        }
    }

    /// The node's children, mutably.
    pub(crate) fn children_mut(&mut self) -> &mut [SyntaxNode] {
        match &mut self.0 {
            NodeKind::Leaf(_) | NodeKind::Error(_) => &mut [],
            NodeKind::Inner(inner) => &mut Arc::make_mut(inner).children,
        }
    }

    /// Replaces a range of children with a replacement.
    ///
    /// May have mutated the children if it returns `Err(_)`.
    #[allow(dead_code)] // usado por reparse_block/reparse_markup — migração incremental
    pub(crate) fn replace_children(
        &mut self,
        range: Range<usize>,
        replacement: Vec<SyntaxNode>,
    ) -> NumberingResult {
        if let NodeKind::Inner(inner) = &mut self.0 {
            Arc::make_mut(inner).replace_children(range, replacement)?;
        }
        Ok(())
    }

    /// Update this node after changes were made to one of its children.
    #[allow(dead_code)] // usado por replace_children — migração incremental
    pub(crate) fn update_parent(
        &mut self,
        prev_len: usize,
        new_len: usize,
        prev_descendants: usize,
        new_descendants: usize,
    ) {
        if let NodeKind::Inner(inner) = &mut self.0 {
            Arc::make_mut(inner).update_parent(
                prev_len,
                new_len,
                prev_descendants,
                new_descendants,
            );
        }
    }

    /// The upper bound of assigned numbers in this subtree.
    #[allow(dead_code)] // usado por replace_children internamente
    pub(crate) fn upper(&self) -> u64 {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.span.number() + 1,
            NodeKind::Inner(inner) => inner.upper,
            NodeKind::Error(node) => node.error.span.number() + 1,
        }
    }
}

impl Debug for SyntaxNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.0 {
            NodeKind::Leaf(leaf) => leaf.fmt(f),
            NodeKind::Inner(inner) => inner.fmt(f),
            NodeKind::Error(node) => node.fmt(f),
        }
    }
}

impl Default for SyntaxNode {
    fn default() -> Self {
        Self::leaf(SyntaxKind::End, "")
    }
}

/// A leaf node in the untyped syntax tree.
#[derive(Clone, Eq, PartialEq, Hash)]
struct LeafNode {
    kind: SyntaxKind,
    text: SyntaxText,
    span: Span,
}

impl LeafNode {
    #[track_caller]
    fn new(kind: SyntaxKind, text: impl Into<SyntaxText>) -> Self {
        debug_assert!(!kind.is_error());
        Self { kind, text: text.into(), span: Span::detached() }
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn spanless_eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.text == other.text
    }
}

impl Debug for LeafNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.kind, self.text)
    }
}

/// An inner node in the untyped syntax tree.
#[derive(Clone, Eq, PartialEq, Hash)]
struct InnerNode {
    kind: SyntaxKind,
    len: usize,
    span: Span,
    descendants: usize,
    erroneous: bool,
    upper: u64,
    children: Vec<SyntaxNode>,
}

impl InnerNode {
    #[track_caller]
    fn new(kind: SyntaxKind, children: Vec<SyntaxNode>) -> Self {
        debug_assert!(!kind.is_error());

        let mut len = 0;
        let mut descendants = 1;
        let mut erroneous = false;

        for child in &children {
            len += child.len();
            descendants += child.descendants();
            erroneous |= child.erroneous();
        }

        Self {
            kind,
            len,
            span: Span::detached(),
            descendants,
            erroneous,
            upper: 0,
            children,
        }
    }

    fn synthesize(&mut self, span: Span) {
        self.span = span;
        self.upper = span.number();
        for child in &mut self.children {
            child.synthesize(span);
        }
    }

    fn numberize(
        &mut self,
        id: FileId,
        range: Option<Range<usize>>,
        within: Range<u64>,
    ) -> NumberingResult {
        let selected = range.clone().unwrap_or(0..self.children.len());
        let children = self.children.get(selected.clone()).ok_or(Unnumberable)?;
        let descendants = match &range {
            Some(range) if range.is_empty() => return Ok(()),
            Some(_) => children.iter().try_fold(0usize, |total, child| {
                total.checked_add(child.descendants()).ok_or(Unnumberable)
            })?,
            None => self.descendants,
        };
        let descendants = u64::try_from(descendants).map_err(|_| Unnumberable)?;
        let space = within.end.checked_sub(within.start).ok_or(Unnumberable)?;
        let denominator = descendants.checked_mul(2).ok_or(Unnumberable)?;
        let mut stride = space.checked_div(denominator).ok_or(Unnumberable)?;
        if stride == 0 {
            let total = u64::try_from(self.descendants).map_err(|_| Unnumberable)?;
            stride = space.checked_div(total).ok_or(Unnumberable)?;
            if stride == 0 {
                return Err(Unnumberable);
            }
        }
        let mut start = within.start;
        if range.is_none() {
            let end = start.checked_add(stride).ok_or(Unnumberable)?;
            let midpoint = start + (end - start) / 2;
            self.span = Span::from_number(id, midpoint).ok_or(Unnumberable)?;
            self.upper = within.end;
            start = end;
        }
        for child in self.children.get_mut(selected).ok_or(Unnumberable)? {
            let count = u64::try_from(child.descendants()).map_err(|_| Unnumberable)?;
            let width = count.checked_mul(stride).ok_or(Unnumberable)?;
            let end = start.checked_add(width).ok_or(Unnumberable)?;
            if end > within.end {
                return Err(Unnumberable);
            }
            child.numberize(id, start..end)?;
            start = end;
        }

        Ok(())
    }

    fn spanless_eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.len == other.len
            && self.descendants == other.descendants
            && self.erroneous == other.erroneous
            && self.children.len() == other.children.len()
            && self
                .children
                .iter()
                .zip(&other.children)
                .all(|(a, b)| a.spanless_eq(b))
    }

    fn replace_children(
        &mut self,
        mut range: Range<usize>,
        replacement: Vec<SyntaxNode>,
    ) -> NumberingResult {
        let Some(id) = self.span.id() else { return Err(Unnumberable) };
        let mut replacement_range = 0..replacement.len();

        while range.start < range.end
            && replacement_range.start < replacement_range.end
            && self.children[range.start]
                .spanless_eq(&replacement[replacement_range.start])
        {
            range.start += 1;
            replacement_range.start += 1;
        }

        while range.start < range.end
            && replacement_range.start < replacement_range.end
            && self.children[range.end - 1]
                .spanless_eq(&replacement[replacement_range.end - 1])
        {
            range.end -= 1;
            replacement_range.end -= 1;
        }

        let mut replacement_vec = replacement;
        let replacement = &replacement_vec[replacement_range.clone()];
        let superseded = &self.children[range.clone()];

        self.len = self.len + replacement.iter().map(SyntaxNode::len).sum::<usize>()
            - superseded.iter().map(SyntaxNode::len).sum::<usize>();

        self.descendants = self.descendants
            + replacement.iter().map(SyntaxNode::descendants).sum::<usize>()
            - superseded.iter().map(SyntaxNode::descendants).sum::<usize>();

        self.erroneous = replacement.iter().any(SyntaxNode::erroneous)
            || (self.erroneous
                && (self.children[..range.start].iter().any(SyntaxNode::erroneous))
                || self.children[range.end..].iter().any(SyntaxNode::erroneous));

        self.children
            .splice(range.clone(), replacement_vec.drain(replacement_range.clone()));
        range.end = range.start + replacement_range.len();

        let mut left = 0;
        let mut right = 0;
        let max_left = range.start;
        let max_right = self.children.len() - range.end;
        loop {
            let renumber = range.start - left..range.end + right;

            let start_number = renumber
                .start
                .checked_sub(1)
                .and_then(|i| self.children.get(i))
                .map_or(self.span.number() + 1, |child| child.upper());

            let end_number = self
                .children
                .get(renumber.end)
                .map_or(self.upper, |next| next.span().number());

            let within = start_number..end_number;
            if self.numberize(id, Some(renumber), within).is_ok() {
                return Ok(());
            }

            if left == max_left && right == max_right {
                return Err(Unnumberable);
            }

            left = (left + 1).next_power_of_two().min(max_left);
            right = (right + 1).next_power_of_two().min(max_right);
        }
    }

    fn update_parent(
        &mut self,
        prev_len: usize,
        new_len: usize,
        prev_descendants: usize,
        new_descendants: usize,
    ) {
        self.len = self.len + new_len - prev_len;
        self.descendants = self.descendants + new_descendants - prev_descendants;
        self.erroneous = self.children.iter().any(SyntaxNode::erroneous);
    }
}

impl Debug for InnerNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.len)?;
        if !self.children.is_empty() {
            f.write_str(" ")?;
            f.debug_list().entries(&self.children).finish()?;
        }
        Ok(())
    }
}

/// An error node in the untyped syntax tree.
#[derive(Clone, Eq, PartialEq, Hash)]
struct ErrorNode {
    text: SyntaxText,
    error: SyntaxError,
}

impl ErrorNode {
    fn new(error: SyntaxError, text: impl Into<SyntaxText>) -> Self {
        Self { text: text.into(), error }
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn hint(&mut self, hint: impl Into<SyntaxText>) {
        self.error.hints.push(hint.into());
    }

    fn spanless_eq(&self, other: &Self) -> bool {
        self.text == other.text && self.error.spanless_eq(&other.error)
    }
}

impl Debug for ErrorNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {:?} ({})", self.text, self.error.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spanless_equality_covers_all_representation_pairs_with_identical_text() {
        let nodes = [
            SyntaxNode::leaf(SyntaxKind::Text, "x"),
            SyntaxNode::inner(
                SyntaxKind::Markup,
                vec![SyntaxNode::leaf(SyntaxKind::Text, "x")],
            ),
            SyntaxNode::error(SyntaxError::new("invalid token"), "x"),
        ];
        let expected = [[true, false, false], [false, true, false], [false, false, true]];
        for (i, left) in nodes.iter().enumerate() {
            assert_eq!(left.clone().into_text().as_str(), "x");
            for (j, right) in nodes.iter().enumerate() {
                assert_eq!(left.spanless_eq(right), expected[i][j], "pair {i}/{j}");
            }
        }
        let empty_leaf = SyntaxNode::leaf(SyntaxKind::Text, "");
        let empty_inner = SyntaxNode::inner(SyntaxKind::Markup, vec![]);
        assert!(!empty_leaf.spanless_eq(&empty_inner));
        assert!(!empty_inner.spanless_eq(&empty_leaf));
    }

    #[test]
    fn spanless_equality_ignores_synthesized_spans_for_each_representation() {
        let nodes = [
            SyntaxNode::leaf(SyntaxKind::Text, "x"),
            SyntaxNode::inner(
                SyntaxKind::Markup,
                vec![SyntaxNode::leaf(SyntaxKind::Text, "x")],
            ),
            SyntaxNode::error(SyntaxError::new("invalid token"), "x"),
        ];
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let first_span = Span::from_number(id, 7).unwrap();
        let second_span = Span::from_number(id, 19).unwrap();
        for original in nodes {
            let mut first = original.clone();
            let mut second = original.clone();
            first.synthesize(first_span);
            second.synthesize(second_span);
            assert_eq!(first.span(), first_span);
            assert_eq!(second.span(), second_span);
            assert!(original.span().is_detached());
            assert_ne!(first, second);
            assert!(first.spanless_eq(&second));
            assert!(second.spanless_eq(&first));
            assert!(original.spanless_eq(&first));
        }
    }

    #[test]
    fn spanless_equality_retains_leaf_kind_text_and_inner_order_structure() {
        let text = SyntaxNode::leaf(SyntaxKind::Text, "ab");
        assert!(!text.spanless_eq(&SyntaxNode::leaf(SyntaxKind::Ident, "ab")));
        assert!(!text.spanless_eq(&SyntaxNode::leaf(SyntaxKind::Text, "ac")));
        let tree = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![
                SyntaxNode::leaf(SyntaxKind::Text, "a"),
                SyntaxNode::leaf(SyntaxKind::Text, "b"),
            ],
        );
        let reversed = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![
                SyntaxNode::leaf(SyntaxKind::Text, "b"),
                SyntaxNode::leaf(SyntaxKind::Text, "a"),
            ],
        );
        let merged = SyntaxNode::inner(SyntaxKind::Markup, vec![text]);
        let nested = SyntaxNode::inner(SyntaxKind::Markup, vec![merged.clone()]);
        let different_kind =
            SyntaxNode::inner(SyntaxKind::CodeBlock, tree.children().cloned().collect());
        assert!(!tree.spanless_eq(&reversed));
        assert!(!tree.spanless_eq(&merged));
        assert!(!merged.spanless_eq(&nested));
        assert!(!tree.spanless_eq(&different_kind));
        assert_eq!(tree.clone().into_text(), merged.clone().into_text());
        assert_eq!(merged.into_text(), nested.into_text());
    }

    #[test]
    fn spanless_equality_retains_error_category_message_hints_and_text() {
        use crate::entities::syntax_node::SyntaxErrorKind;

        let error = SyntaxError::new("invalid token");
        let original = SyntaxNode::error(error.clone(), "x");
        let mut category = error.clone();
        category.kind = SyntaxErrorKind::InvalidHexNumber;
        let mut message = error.clone();
        message.message = "different message".into();
        let mut hinted = error.clone();
        hinted.hints.push("try a number".into());
        for changed in [
            SyntaxNode::error(category, "x"),
            SyntaxNode::error(message, "x"),
            SyntaxNode::error(hinted, "x"),
            SyntaxNode::error(error, "y"),
        ] {
            assert!(!original.spanless_eq(&changed));
            assert!(!changed.spanless_eq(&original));
        }
        assert!(original.spanless_eq(&original.clone()));
    }

    #[test]
    fn editing_clone_updates_its_caches_without_mutating_original() {
        let original = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![SyntaxNode::leaf(SyntaxKind::Text, "a")],
        );
        let mut clone = original.clone();
        clone.children_mut()[0].convert_to_error("invalid token");
        clone.update_parent(1, 1, 1, 1);
        let NodeKind::Inner(inner) = &clone.0 else { panic!("expected inner") };
        assert_eq!(inner.len, 1);
        assert_eq!(inner.descendants, 2);
        assert!(inner.erroneous);
        assert!(!original.erroneous());
        assert_eq!(original.children().next().unwrap().kind(), SyntaxKind::Text);
    }

    #[test]
    fn replacement_retains_spans_of_unchanged_edges() {
        let leaf = |text| SyntaxNode::leaf(SyntaxKind::Text, text);
        let mut root = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![leaf("left"), leaf("old"), leaf("right")],
        );
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        root.numberize(id, 1..1_000_000).unwrap();
        let before: Vec<_> = root.children().map(SyntaxNode::span).collect();
        root.replace_children(0..3, vec![leaf("left"), leaf("new!"), leaf("right")])
            .unwrap();
        assert_eq!(root.children().next().unwrap().span(), before[0]);
        assert_eq!(root.children().next_back().unwrap().span(), before[2]);
        assert_eq!(root.len(), 13);
        assert_eq!(root.descendants(), 4);
        assert!(!root.erroneous());
        assert_eq!(root.into_text().as_str(), "leftnew!right");
    }

    #[test]
    fn no_panic_source_numbering_rejects_reserved_and_out_of_range_midpoints() {
        let id = FileId::from_raw(std::num::NonZeroU16::MIN);
        for within in [0..1, Span::FULL.end..Span::FULL.end + 1, u64::MAX - 1..u64::MAX] {
            let mut node = SyntaxNode::leaf(SyntaxKind::Text, "x");
            assert_eq!(node.numberize(id, within), Err(Unnumberable));
        }
    }

    #[test]
    fn no_panic_source_inner_numbering_rejects_invalid_child_intervals() {
        let id = FileId::from_raw(std::num::NonZeroU16::MIN);
        let mut inner = InnerNode::new(
            SyntaxKind::Markup,
            vec![SyntaxNode::leaf(SyntaxKind::Text, "x")],
        );
        assert_eq!(inner.numberize(id, Some(0..2), Span::FULL), Err(Unnumberable));
        assert_eq!(
            inner.numberize(id, None, Range { start: 7, end: 3 }),
            Err(Unnumberable)
        );
    }

    #[test]
    fn no_panic_source_inner_numbering_rejects_zero_or_overflowing_counts() {
        let id = FileId::from_raw(std::num::NonZeroU16::MIN);
        for descendants in [0, usize::MAX] {
            let mut inner = InnerNode::new(SyntaxKind::Markup, vec![]);
            inner.descendants = descendants;
            assert_eq!(inner.numberize(id, None, Span::FULL), Err(Unnumberable));
        }
    }
}
