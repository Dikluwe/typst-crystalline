//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/syntax-node/linked.md
//! @prompt-hash 27524de5
//! @layer L1
//! @updated 2026-09-26

use std::fmt::{self, Debug, Formatter};
use std::ops::Range;
use std::rc::Rc;

use super::{NodeKind, SyntaxNode};
use crate::entities::span::Span;
use crate::entities::syntax_kind::SyntaxKind;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_navigation_crosses_nested_parents_and_skips_trivia() {
        let leaf = |text| SyntaxNode::leaf(SyntaxKind::Text, text);
        let root = SyntaxNode::inner(
            SyntaxKind::Markup,
            vec![
                leaf("a"),
                SyntaxNode::inner(
                    SyntaxKind::Markup,
                    vec![leaf("β"), SyntaxNode::leaf(SyntaxKind::Space, " "), leaf("c")],
                ),
                leaf("d"),
            ],
        );
        let linked = LinkedNode::new(&root);
        let beta = linked.children().nth(1).unwrap().leftmost_leaf().unwrap();
        assert_eq!(beta.range(), 1..3);
        assert_eq!(beta.prev_leaf().unwrap().text_str(), "a");
        let c = beta.next_leaf().unwrap();
        assert_eq!(c.range(), 4..5);
        assert_eq!(c.text_str(), "c");
        let d = c.next_leaf().unwrap();
        assert_eq!(d.range(), 5..6);
        assert_eq!(d.parent().unwrap().get(), &root);
        assert!(d.next_leaf().is_none());
    }
}
