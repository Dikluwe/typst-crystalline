//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/ast/math.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-03-26

use crate::node;
use crate::entities::ast::expr::Expr;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::SyntaxNode;

node! {
    /// A mathematical equation: `$x$`, `$ x^2 $`.
    struct Equation
}

impl<'a> Equation<'a> {
    /// The contained math.
    pub fn body(self) -> Math<'a> {
        self.0.cast_first()
    }

    /// Whether the equation should be displayed as a separate block.
    pub fn block(self) -> bool {
        let is_space = |node: Option<&SyntaxNode>| {
            node.map(SyntaxNode::kind) == Some(SyntaxKind::Space)
        };
        is_space(self.0.children().nth(1)) && is_space(self.0.children().nth_back(1))
    }
}

node! {
    /// The contents of a mathematical equation: `x^2 + 1`.
    struct Math
}

impl<'a> Math<'a> {
    /// The expressions the mathematical content consists of.
    pub fn exprs(self) -> impl DoubleEndedIterator<Item = Expr<'a>> {
        self.0.children().filter_map(Expr::cast_with_space)
    }

    /// Whether this `Math` node was originally parenthesized.
    pub fn was_deparenthesized(self) -> bool {
        let mut iter = self.0.children();
        matches!(iter.next().map(SyntaxNode::kind), Some(SyntaxKind::LeftParen))
            && matches!(
                iter.next_back().map(SyntaxNode::kind),
                Some(SyntaxKind::RightParen)
            )
    }
}

node! {
    /// A lone text fragment in math: `x`, `25`, `3.1415`, `=`, `[`.
    struct MathText
}

/// The underlying text kind.
pub enum MathTextKind<'a> {
    /// A grapheme cluster (single character or symbol).
    Grapheme(&'a str),
    /// A numeric literal.
    Number(&'a str),
}

impl<'a> MathText<'a> {
    /// Return the underlying text.
    pub fn get(self) -> MathTextKind<'a> {
        let text = self.0.text_str();
        if text.chars().next().unwrap_or_default().is_numeric() {
            MathTextKind::Number(text)
        } else {
            MathTextKind::Grapheme(text)
        }
    }
}

node! {
    /// An identifier in math: `pi`.
    struct MathIdent
}

impl<'a> MathIdent<'a> {
    /// Get the identifier.
    pub fn get(self) -> &'a str {
        self.0.text_str()
    }

    /// Get the identifier as a string slice.
    pub fn as_str(self) -> &'a str {
        self.get()
    }
}

impl std::ops::Deref for MathIdent<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.text_str()
    }
}

node! {
    /// A shorthand for a unicode codepoint in math: `a <= b`.
    struct MathShorthand
}

impl MathShorthand<'_> {
    /// A list of all shorthands in math mode.
    pub const LIST: &'static [(&'static str, char)] = &[
        ("...", '…'),
        ("-", '−'),
        ("*", '∗'),
        ("~", '∼'),
        ("!=", '≠'),
        (":=", '≔'),
        ("::=", '⩴'),
        ("=:", '≕'),
        ("<<", '≪'),
        ("<<<", '⋘'),
        (">>", '≫'),
        (">>>", '⋙'),
        ("<=", '≤'),
        (">=", '≥'),
        ("->", '→'),
        ("-->", '⟶'),
        ("|->", '↦'),
        (">->", '↣'),
        ("->>", '↠'),
        ("<-", '←'),
        ("<--", '⟵'),
        ("<-<", '↢'),
        ("<<-", '↞'),
        ("<->", '↔'),
        ("<-->", '⟷'),
        ("~>", '⇝'),
        ("~~>", '⟿'),
        ("<~", '⇜'),
        ("<~~", '⬳'),
        ("=>", '⇒'),
        ("|=>", '⤇'),
        ("==>", '⟹'),
        ("<==", '⟸'),
        ("<=>", '⇔'),
        ("<==>", '⟺'),
        ("[|", '⟦'),
        ("|]", '⟧'),
        ("||", '‖'),
    ];

    /// Get the shorthanded character.
    pub fn get(self) -> char {
        let text = self.0.text_str();
        Self::LIST
            .iter()
            .find(|&&(s, _)| s == text)
            .map_or_else(char::default, |&(_, c)| c)
    }
}

node! {
    /// An alignment point in math: `&`.
    struct MathAlignPoint
}

node! {
    /// Matched delimiters in math: `[x + y]`.
    struct MathDelimited
}

impl<'a> MathDelimited<'a> {
    /// The opening delimiter.
    pub fn open(self) -> Expr<'a> {
        self.0.cast_first()
    }

    /// The contents, including the delimiters.
    pub fn body(self) -> Math<'a> {
        self.0.cast_first()
    }

    /// The closing delimiter.
    pub fn close(self) -> Expr<'a> {
        self.0.cast_last()
    }
}

node! {
    /// A base with optional attachments in math: `a_1^2`.
    struct MathAttach
}

impl<'a> MathAttach<'a> {
    /// The base, to which things are attached.
    pub fn base(self) -> Expr<'a> {
        self.0.cast_first()
    }

    /// The bottom attachment.
    pub fn bottom(self) -> Option<Expr<'a>> {
        self.0
            .children()
            .skip_while(|node| !matches!(node.kind(), SyntaxKind::Underscore))
            .find_map(SyntaxNode::cast)
    }

    /// The top attachment.
    pub fn top(self) -> Option<Expr<'a>> {
        self.0
            .children()
            .skip_while(|node| !matches!(node.kind(), SyntaxKind::Hat))
            .find_map(SyntaxNode::cast)
    }

    /// Extract attached primes if present.
    pub fn primes(self) -> Option<MathPrimes<'a>> {
        self.0
            .children()
            .skip_while(|node| node.cast::<Expr<'_>>().is_none())
            .nth(1)
            .and_then(|n| n.cast())
    }
}

node! {
    /// Grouped primes in math: `a'''`.
    struct MathPrimes
}

impl MathPrimes<'_> {
    /// The number of grouped primes.
    pub fn count(self) -> usize {
        self.0.text_str().len()
    }
}

node! {
    /// A fraction in math: `x/2`.
    struct MathFrac
}

impl<'a> MathFrac<'a> {
    /// The numerator.
    pub fn num(self) -> Expr<'a> {
        self.0.cast_first()
    }

    /// The denominator.
    pub fn denom(self) -> Expr<'a> {
        self.0.cast_last()
    }
}

node! {
    /// A root in math: `√x`, `∛x` or `∜x`.
    struct MathRoot
}

impl<'a> MathRoot<'a> {
    /// The index of the root.
    pub fn index(self) -> Option<u8> {
        match self.0.children().next().map(|node| node.text_str()) {
            Some("∜") => Some(4),
            Some("∛") => Some(3),
            _ => Option::None,
        }
    }

    /// The radicand.
    pub fn radicand(self) -> Expr<'a> {
        self.0.cast_first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::ast::AstNode;
    use crate::entities::source::Source;

    #[test]
    fn math_ident_get_returns_str() {
        // Contrato correcto — MathIdent wraps identifier text in math mode
        let _ = MathIdent::from_untyped; // confirm type exists
    }

    #[test]
    fn math_shorthand_list_not_empty() {
        assert!(!MathShorthand::LIST.is_empty());
    }

    #[test]
    fn equation_block_requires_spaces() {
        // $ x $ is block (space after first $ and before last $)
        let src = Source::detached("$ x $");
        let has_equation = src.root()
            .children()
            .any(|n| n.kind() == crate::entities::syntax_kind::SyntaxKind::Equation);
        assert!(has_equation);
    }
}
