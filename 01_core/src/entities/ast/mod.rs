//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/ast/mod.md
//! @prompt-hash df5b9eee
//! @layer L1
//! @updated 2026-03-25

pub mod markup;
pub mod math;
pub mod code;
pub mod expr;

use crate::entities::span::Span;
use crate::entities::syntax_node::SyntaxNode;

/// A typed AST node.
///
/// Wrappers with lifetime `'a` over `&'a SyntaxNode`.
/// Zero I/O — purely domain-level view over the CST.
///
/// Note: `placeholder()` is omitted from L1 (requires static SyntaxNode which
/// needs const fn support not yet available). `cast_first`/`cast_last` panic
/// on malformed trees. This is acceptable in Passo 5; revisit in Passo 10.
pub trait AstNode<'a>: Sized {
    /// Convert a node into its typed variant.
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>;

    /// A reference to the underlying syntax node.
    fn to_untyped(self) -> &'a SyntaxNode;

    /// The source code location.
    fn span(self) -> Span {
        self.to_untyped().span()
    }
}

/// Implements `AstNode` for a struct whose name matches a `SyntaxKind` variant.
///
/// The struct becomes a wrapper `struct Name<'a>(&'a SyntaxNode)`.
/// `from_untyped` checks `node.kind() == SyntaxKind::Name`.
#[macro_export]
macro_rules! node {
    ($(#[$attr:meta])* struct $name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        $(#[$attr])*
        pub struct $name<'a>(pub(crate) &'a $crate::entities::syntax_node::SyntaxNode);

        impl<'a> $crate::entities::ast::AstNode<'a> for $name<'a> {
            #[inline]
            fn from_untyped(
                node: &'a $crate::entities::syntax_node::SyntaxNode,
            ) -> Option<Self> {
                if node.kind() == $crate::entities::syntax_kind::SyntaxKind::$name {
                    Some(Self(node))
                } else {
                    Option::None
                }
            }

            #[inline]
            fn to_untyped(self) -> &'a $crate::entities::syntax_node::SyntaxNode {
                self.0
            }
        }
    };
}


// Methods added to SyntaxNode from the AST module to avoid circular imports.
impl SyntaxNode {
    /// Whether the node can be cast to the given AST node.
    pub fn is<'a, T: AstNode<'a>>(&'a self) -> bool {
        self.cast::<T>().is_some()
    }

    /// Try to convert the node to a typed AST node.
    pub fn cast<'a, T: AstNode<'a>>(&'a self) -> Option<T> {
        T::from_untyped(self)
    }

    /// Find the first child that can cast to the AST type `T`.
    pub(crate) fn try_cast_first<'a, T: AstNode<'a>>(&'a self) -> Option<T> {
        self.children().find_map(Self::cast)
    }

    /// Find the last child that can cast to the AST type `T`.
    pub(crate) fn try_cast_last<'a, T: AstNode<'a>>(&'a self) -> Option<T> {
        self.children().rev().find_map(Self::cast)
    }

    /// Get the first child of AST type `T`, panicking if not found.
    ///
    /// Panics on malformed trees. Placeholder() removed in L1 — see prompt.
    pub(crate) fn cast_first<'a, T: AstNode<'a>>(&'a self) -> T {
        self.try_cast_first().expect("AST: expected child not found")
    }

    /// Get the last child of AST type `T`, panicking if not found.
    pub(crate) fn cast_last<'a, T: AstNode<'a>>(&'a self) -> T {
        self.try_cast_last().expect("AST: expected child not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        source::Source,
        syntax_kind::SyntaxKind,
    };
    use crate::entities::ast::markup::Markup;

    #[test]
    fn markup_from_markup_node() {
        let src = Source::detached("Hello *world*");
        let node = src.root();
        assert_eq!(node.kind(), SyntaxKind::Markup);
        let markup = Markup::from_untyped(node);
        assert!(markup.is_some());
    }

    #[test]
    fn markup_from_wrong_kind_returns_none() {
        let src = Source::detached("Hello *world*");
        let text_node = src.root()
            .children()
            .find(|n| n.kind() == SyntaxKind::Text);
        if let Some(node) = text_node {
            let markup = Markup::from_untyped(node);
            assert!(markup.is_none());
        }
    }

    #[test]
    fn markup_to_untyped_roundtrip() {
        let src = Source::detached("Hello");
        let node = src.root();
        let markup = Markup::from_untyped(node).unwrap();
        let back = markup.to_untyped();
        assert_eq!(back.kind(), SyntaxKind::Markup);
    }

    #[test]
    fn span_delegates_to_untyped() {
        let src = Source::detached("Hello");
        let node = src.root();
        let markup = Markup::from_untyped(node).unwrap();
        let _span = markup.span();
    }
}
