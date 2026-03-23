//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/syntax-set.md
//! @prompt-hash c67f481f
//! @layer L1
//! @updated 2026-03-22

// Acknowledgement:
// Based on rust-analyzer's `TokenSet`.
// https://github.com/rust-lang/rust-analyzer/blob/master/crates/parser/src/token_set.rs

use super::syntax_kind::SyntaxKind;

/// A set of syntax kinds, represented as a `u128` bitset.
///
/// Each bit corresponds to the `u8` discriminant of a `SyntaxKind` variant.
/// Only kinds with discriminant < 128 can be stored.
#[derive(Default, Copy, Clone)]
pub struct SyntaxSet(u128);

impl SyntaxSet {
    /// Create a new empty set.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Insert a syntax kind into the set.
    ///
    /// Only kinds with discriminant < 128 are supported.
    pub const fn add(self, kind: SyntaxKind) -> Self {
        assert!((kind as u8) < BITS);
        Self(self.0 | bit(kind))
    }

    /// Remove a syntax kind from the set. Does nothing if not present.
    ///
    /// Only kinds with discriminant < 128 are supported.
    pub const fn remove(self, kind: SyntaxKind) -> Self {
        assert!((kind as u8) < BITS);
        Self(self.0 & !bit(kind))
    }

    /// Combine two syntax sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether the set contains the given syntax kind.
    pub const fn contains(&self, kind: SyntaxKind) -> bool {
        (kind as u8) < BITS && (self.0 & bit(kind)) != 0
    }
}

const BITS: u8 = 128;

const fn bit(kind: SyntaxKind) -> u128 {
    1 << (kind as usize)
}

/// Generate a compile-time constant `SyntaxSet` of the given kinds.
#[macro_export]
macro_rules! syntax_set {
    ($($kind:ident),* $(,)?) => {{
        const SET: $crate::entities::syntax_set::SyntaxSet =
            $crate::entities::syntax_set::SyntaxSet::new()
            $(.add($crate::entities::syntax_kind::SyntaxKind:: $kind))*;
        SET
    }}
}

/// Syntax kinds that can start a statement.
pub const STMT: SyntaxSet = syntax_set!(Let, Set, Show, Import, Include, Return);

/// Syntax kinds that can start a math expression.
pub const MATH_EXPR: SyntaxSet = syntax_set!(
    Hash,
    MathIdent,
    FieldAccess,
    Dot,
    Comma,
    Semicolon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    MathText,
    MathShorthand,
    Linebreak,
    MathAlignPoint,
    MathPrimes,
    Escape,
    Str,
    Root,
    Bang,
);

/// Syntax kinds that can start a code expression.
pub const CODE_EXPR: SyntaxSet = CODE_PRIMARY.union(UNARY_OP);

/// Syntax kinds that can start an atomic code expression.
pub const ATOMIC_CODE_EXPR: SyntaxSet = ATOMIC_CODE_PRIMARY;

/// Syntax kinds that can start a code primary.
pub const CODE_PRIMARY: SyntaxSet = ATOMIC_CODE_PRIMARY.add(SyntaxKind::Underscore);

/// Syntax kinds that can start an atomic code primary.
pub const ATOMIC_CODE_PRIMARY: SyntaxSet = syntax_set!(
    Ident,
    LeftBrace,
    LeftBracket,
    LeftParen,
    Dollar,
    Let,
    Set,
    Show,
    Context,
    If,
    While,
    For,
    Import,
    Include,
    Break,
    Continue,
    Return,
    None,
    Auto,
    Int,
    Float,
    Bool,
    Numeric,
    Str,
    Label,
    Raw,
);

/// Syntax kinds that are unary operators.
pub const UNARY_OP: SyntaxSet = syntax_set!(Plus, Minus, Not);

/// Syntax kinds that are binary operators.
pub const BINARY_OP: SyntaxSet = syntax_set!(
    Plus, Minus, Star, Slash, And, Or, EqEq, ExclEq, Lt, LtEq, Gt, GtEq, Eq, In, PlusEq,
    HyphEq, StarEq, SlashEq,
);

/// Syntax kinds that can start an item in an array or dictionary.
pub const ARRAY_OR_DICT_ITEM: SyntaxSet = CODE_EXPR.add(SyntaxKind::Dots);

/// Syntax kinds that can start an argument in a function call.
pub const ARG: SyntaxSet = CODE_EXPR.add(SyntaxKind::Dots);

/// Syntax kinds that can start a parameter in a parameter list.
pub const PARAM: SyntaxSet = PATTERN.add(SyntaxKind::Dots);

/// Syntax kinds that can start a destructuring item.
pub const DESTRUCTURING_ITEM: SyntaxSet = PATTERN.add(SyntaxKind::Dots);

/// Syntax kinds that can start a pattern.
pub const PATTERN: SyntaxSet =
    PATTERN_LEAF.add(SyntaxKind::LeftParen).add(SyntaxKind::Underscore);

/// Syntax kinds that can start a pattern leaf.
pub const PATTERN_LEAF: SyntaxSet = ATOMIC_CODE_EXPR;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_contains_nothing() {
        let set = SyntaxSet::new();
        assert!(!set.contains(SyntaxKind::Text));
        assert!(!set.contains(SyntaxKind::Let));
        assert!(!set.contains(SyntaxKind::Error));
    }

    #[test]
    fn add_and_contains() {
        let set = SyntaxSet::new().add(SyntaxKind::And).add(SyntaxKind::Or);
        assert!(set.contains(SyntaxKind::And));
        assert!(set.contains(SyntaxKind::Or));
        assert!(!set.contains(SyntaxKind::Not));
    }

    #[test]
    fn remove() {
        let set = SyntaxSet::new()
            .add(SyntaxKind::And)
            .add(SyntaxKind::Or)
            .remove(SyntaxKind::And);
        assert!(!set.contains(SyntaxKind::And));
        assert!(set.contains(SyntaxKind::Or));
    }

    #[test]
    fn union() {
        let a = SyntaxSet::new().add(SyntaxKind::Plus);
        let b = SyntaxSet::new().add(SyntaxKind::Minus);
        let c = a.union(b);
        assert!(c.contains(SyntaxKind::Plus));
        assert!(c.contains(SyntaxKind::Minus));
        assert!(!c.contains(SyntaxKind::Star));
    }

    #[test]
    fn stmt_constants() {
        assert!(STMT.contains(SyntaxKind::Let));
        assert!(STMT.contains(SyntaxKind::Set));
        assert!(STMT.contains(SyntaxKind::Import));
        assert!(!STMT.contains(SyntaxKind::Text));
        assert!(!STMT.contains(SyntaxKind::Ident));
    }

    #[test]
    fn unary_op_constants() {
        assert!(UNARY_OP.contains(SyntaxKind::Not));
        assert!(UNARY_OP.contains(SyntaxKind::Plus));
        assert!(UNARY_OP.contains(SyntaxKind::Minus));
        assert!(!UNARY_OP.contains(SyntaxKind::Star));
    }

    #[test]
    fn code_expr_includes_ident() {
        assert!(CODE_EXPR.contains(SyntaxKind::Ident));
        assert!(CODE_EXPR.contains(SyntaxKind::Not));
        assert!(CODE_EXPR.contains(SyntaxKind::Int));
    }

    #[test]
    fn is_copy() {
        let a = SyntaxSet::new().add(SyntaxKind::Text);
        let b = a;
        assert!(b.contains(SyntaxKind::Text));
        assert!(a.contains(SyntaxKind::Text));
    }
}
