//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/syntax-node.md
//! @prompt-hash 90523fde
//! @layer L1
//! @updated 2026-04-23
//!
//! Excepção Regra 6 da ADR-0037: `SyntaxNode` é a árvore sintáctica
//! fundamental (inner enum + impls de navegação/edição/construção).
//! A estrutura é naturalmente densa — dividir `impl SyntaxNode` por
//! tema (leaves, inner, errors, construção) fragmentaria a sua API
//! sem ganho de invariante. ~1090 linhas aceitas como custo de
//! coesão estrutural.

use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use super::file_id::FileId;
use super::span::Span;
use super::syntax_kind::SyntaxKind;
use super::syntax_text::SyntaxText;

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
            _ => false,
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

        let mid = Span::from_number(id, (within.start + within.end) / 2).unwrap();
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
        let descendants = match &range {
            Some(range) if range.is_empty() => return Ok(()),
            Some(range) => self.children[range.clone()]
                .iter()
                .map(SyntaxNode::descendants)
                .sum::<usize>(),
            None => self.descendants,
        };

        let space = within.end - within.start;
        let mut stride = space / (2 * descendants as u64);
        if stride == 0 {
            stride = space / self.descendants as u64;
            if stride == 0 {
                return Err(Unnumberable);
            }
        }

        let mut start = within.start;
        if range.is_none() {
            let end = start + stride;
            self.span = Span::from_number(id, (start + end) / 2).unwrap();
            self.upper = within.end;
            start = end;
        }

        let len = self.children.len();
        for child in &mut self.children[range.unwrap_or(0..len)] {
            let end = start + child.descendants() as u64 * stride;
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

/// A syntactical error.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxError {
    /// The node's span.
    pub span: Span,
    /// The error message.
    pub message: SyntaxText,
    /// Additional hints to the user.
    pub hints: Vec<SyntaxText>,
}

impl SyntaxError {
    /// Create a new detached syntax error.
    pub fn new(message: impl Into<SyntaxText>) -> Self {
        Self {
            span: Span::detached(),
            message: message.into(),
            hints: vec![],
        }
    }

    fn spanless_eq(&self, other: &Self) -> bool {
        self.message == other.message && self.hints == other.hints
    }
}

/// A syntax node in a context.
///
/// Knows its exact offset in the file and provides access to its
/// children, parent and siblings.
///
/// **Note that all sibling and leaf accessors skip over trivia!**
#[derive(Clone)]
pub struct LinkedNode<'a> {
    node: &'a SyntaxNode,
    parent: Option<Rc<Self>>,
    index: usize,
    offset: usize,
}

impl<'a> LinkedNode<'a> {
    /// Start a new traversal at a root node.
    pub fn new(root: &'a SyntaxNode) -> Self {
        Self { node: root, parent: None, index: 0, offset: 0 }
    }

    /// Get the contained syntax node.
    pub fn get(&self) -> &'a SyntaxNode {
        self.node
    }

    /// The index of this node in its parent's children list.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The absolute byte offset of this node in the source file.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// The byte range of this node in the source file.
    pub fn range(&self) -> Range<usize> {
        self.offset..self.offset + self.node.len()
    }

    /// An iterator over this node's children.
    pub fn children(&self) -> LinkedChildren<'a> {
        LinkedChildren {
            parent: Rc::new(self.clone()),
            iter: self.node.children().enumerate(),
            front: self.offset,
            back: self.offset + self.len(),
        }
    }

    /// Find a descendant with the given span.
    pub fn find(&self, span: Span) -> Option<LinkedNode<'a>> {
        if self.span() == span {
            return Some(self.clone());
        }

        if let NodeKind::Inner(inner) = &self.node.0 {
            if span.number() < inner.span.number() {
                return None;
            }

            let mut children = self.children().peekable();
            while let Some(child) = children.next() {
                if children
                    .peek()
                    .is_none_or(|next| next.span().number() > span.number())
                {
                    if let Some(found) = child.find(span) {
                        return Some(found);
                    }
                }
            }
        }

        None
    }
}

impl std::ops::Deref for LinkedNode<'_> {
    type Target = SyntaxNode;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl Debug for LinkedNode<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.node.fmt(f)
    }
}

/// Access to parents and siblings.
impl LinkedNode<'_> {
    /// Get this node's parent.
    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    /// Get the first previous non-trivia sibling node.
    pub fn prev_sibling(&self) -> Option<Self> {
        let parent = self.parent.as_ref()?;
        let children = parent.node.children().as_slice();
        let mut offset = self.offset;
        for (index, node) in children[..self.index].iter().enumerate().rev() {
            offset -= node.len();
            if !node.kind().is_trivia() {
                let parent = Some(parent.clone());
                return Some(Self { node, parent, index, offset });
            }
        }
        None
    }

    /// Get the first previous sibling node, including potential trivia.
    pub fn prev_sibling_with_trivia(&self) -> Option<Self> {
        let parent = self.parent.as_ref()?;
        let children = parent.node.children().as_slice();
        let (index, node) = children[..self.index].iter().enumerate().next_back()?;
        let offset = self.offset - node.len();
        let parent = Some(parent.clone());
        Some(Self { node, parent, index, offset })
    }

    /// Get the next non-trivia sibling node.
    pub fn next_sibling(&self) -> Option<Self> {
        let parent = self.parent.as_ref()?;
        let children = parent.node.children();
        let mut offset = self.offset + self.len();
        for (index, node) in children.enumerate().skip(self.index + 1) {
            if !node.kind().is_trivia() {
                let parent = Some(parent.clone());
                return Some(Self { node, parent, index, offset });
            }
            offset += node.len();
        }
        None
    }

    /// Get the next sibling node, including potential trivia.
    pub fn next_sibling_with_trivia(&self) -> Option<Self> {
        let parent = self.parent.as_ref()?;
        let children = parent.node.children();
        let (index, node) = children.enumerate().nth(self.index + 1)?;
        let offset = self.offset + self.len();
        let parent = Some(parent.clone());
        Some(Self { node, parent, index, offset })
    }

    /// Get the kind of this node's parent.
    pub fn parent_kind(&self) -> Option<SyntaxKind> {
        Some(self.parent()?.node.kind())
    }

    /// Get the kind of this node's first previous non-trivia sibling.
    pub fn prev_sibling_kind(&self) -> Option<SyntaxKind> {
        Some(self.prev_sibling()?.node.kind())
    }

    /// Get the kind of this node's next non-trivia sibling.
    pub fn next_sibling_kind(&self) -> Option<SyntaxKind> {
        Some(self.next_sibling()?.node.kind())
    }
}

/// Indicates whether the cursor is before the related byte index, or after.
#[derive(Debug, Clone)]
pub enum Side {
    Before,
    After,
}

/// Access to leaves.
impl LinkedNode<'_> {
    /// Get the rightmost non-trivia leaf before this node.
    pub fn prev_leaf(&self) -> Option<Self> {
        let mut node = self.clone();
        while let Some(prev) = node.prev_sibling() {
            if let Some(leaf) = prev.rightmost_leaf() {
                return Some(leaf);
            }
            node = prev;
        }
        self.parent()?.prev_leaf()
    }

    /// Find the leftmost contained non-trivia leaf.
    pub fn leftmost_leaf(&self) -> Option<Self> {
        if self.is_leaf() && !self.kind().is_trivia() && !self.kind().is_error() {
            return Some(self.clone());
        }

        for child in self.children() {
            if let Some(leaf) = child.leftmost_leaf() {
                return Some(leaf);
            }
        }

        None
    }

    /// Get the leaf immediately before the specified byte offset.
    fn leaf_before(&self, cursor: usize) -> Option<Self> {
        if self.node.children().len() == 0 && cursor <= self.offset + self.len() {
            return Some(self.clone());
        }

        let mut offset = self.offset;
        let count = self.node.children().len();
        for (i, child) in self.children().enumerate() {
            let len = child.len();
            if (offset < cursor && cursor <= offset + len)
                || (offset == cursor && i + 1 == count)
            {
                return child.leaf_before(cursor);
            }
            offset += len;
        }

        None
    }

    /// Get the leaf after the specified byte offset.
    fn leaf_after(&self, cursor: usize) -> Option<Self> {
        if self.node.children().len() == 0 && cursor < self.offset + self.len() {
            return Some(self.clone());
        }

        let mut offset = self.offset;
        for child in self.children() {
            let len = child.len();
            if offset <= cursor && cursor < offset + len {
                return child.leaf_after(cursor);
            }
            offset += len;
        }

        None
    }

    /// Get the leaf at the specified byte offset.
    pub fn leaf_at(&self, cursor: usize, side: Side) -> Option<Self> {
        match side {
            Side::Before => self.leaf_before(cursor),
            Side::After => self.leaf_after(cursor),
        }
    }

    /// Find the rightmost contained non-trivia leaf.
    pub fn rightmost_leaf(&self) -> Option<Self> {
        if self.is_leaf() && !self.kind().is_trivia() {
            return Some(self.clone());
        }

        for child in self.children().rev() {
            if let Some(leaf) = child.rightmost_leaf() {
                return Some(leaf);
            }
        }

        None
    }

    /// Get the leftmost non-trivia leaf after this node.
    pub fn next_leaf(&self) -> Option<Self> {
        let mut node = self.clone();
        while let Some(next) = node.next_sibling() {
            if let Some(leaf) = next.leftmost_leaf() {
                return Some(leaf);
            }
            node = next;
        }
        self.parent()?.next_leaf()
    }
}

/// An iterator over the children of a linked node.
pub struct LinkedChildren<'a> {
    parent: Rc<LinkedNode<'a>>,
    iter: std::iter::Enumerate<std::slice::Iter<'a, SyntaxNode>>,
    front: usize,
    back: usize,
}

impl<'a> Iterator for LinkedChildren<'a> {
    type Item = LinkedNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, node) = self.iter.next()?;
        let offset = self.front;
        self.front += node.len();
        Some(LinkedNode {
            node,
            parent: Some(self.parent.clone()),
            index,
            offset,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for LinkedChildren<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (index, node) = self.iter.next_back()?;
        self.back -= node.len();
        Some(LinkedNode {
            node,
            parent: Some(self.parent.clone()),
            index,
            offset: self.back,
        })
    }
}

impl ExactSizeIterator for LinkedChildren<'_> {}

/// Result of numbering a node within an interval.
pub(crate) type NumberingResult = Result<(), Unnumberable>;

/// Indicates that a node cannot be numbered within a given interval.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct Unnumberable;

impl Display for Unnumberable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("cannot number within this interval")
    }
}

impl std::error::Error for Unnumberable {}

#[cfg(test)]
mod tests {
    use super::*;

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
}
