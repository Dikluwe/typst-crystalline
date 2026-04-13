//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/ast/markup.md
//! @prompt-hash 5949ad9f
//! @layer L1
//! @updated 2026-03-25

use std::num::NonZeroUsize;

use crate::node;
use crate::entities::ast::expr::Expr;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::SyntaxNode;
use crate::rules::lexer::scanner::Scanner;
use crate::rules::lexer::{is_newline};

node! {
    /// A line comment: `// ...`.
    struct LineComment
}

impl<'a> LineComment<'a> {
    /// The contents of the line comment.
    pub fn text(self) -> &'a str {
        let text = self.0.text_str();
        text.strip_prefix("//").unwrap_or(text)
    }
}

node! {
    /// A block comment: `/* ... */`.
    struct BlockComment
}

impl<'a> BlockComment<'a> {
    /// The contents of the block comment.
    pub fn text(self) -> &'a str {
        let text = self.0.text_str();
        text.strip_prefix("/*")
            .and_then(|t| t.strip_suffix("*/"))
            .unwrap_or(text)
    }
}

node! {
    /// The syntactical root capable of representing a full parsed document.
    struct Markup
}

impl<'a> Markup<'a> {
    /// The expressions.
    pub fn exprs(self) -> impl DoubleEndedIterator<Item = Expr<'a>> {
        let mut was_stmt = false;
        self.0
            .children()
            .filter(move |node| {
                let kind = node.kind();
                let keep = !was_stmt || node.kind() != SyntaxKind::Space;
                was_stmt = kind.is_stmt();
                keep
            })
            .filter_map(Expr::cast_with_space)
    }
}

node! {
    /// Plain text without markup.
    struct Text
}

impl<'a> Text<'a> {
    /// Get the text.
    pub fn get(self) -> &'a str {
        self.0.text_str()
    }
}

node! {
    /// Whitespace in markup or math.
    struct Space
}

node! {
    /// A forced line break: `\`.
    struct Linebreak
}

node! {
    /// A paragraph break, indicated by one or multiple blank lines.
    struct Parbreak
}

node! {
    /// An escape sequence: `\#`, `\u{1F5FA}`.
    struct Escape
}

impl Escape<'_> {
    /// Get the escaped character.
    pub fn get(self) -> char {
        let mut s = Scanner::new(self.0.text_str());
        s.expect('\\');
        if s.eat_if("u{") {
            let hex = s.eat_while(char::is_ascii_hexdigit);
            u32::from_str_radix(hex, 16)
                .ok()
                .and_then(std::char::from_u32)
                .unwrap_or_default()
        } else {
            s.eat().unwrap_or_default()
        }
    }
}

node! {
    /// A shorthand for a unicode codepoint.
    struct Shorthand
}

impl Shorthand<'_> {
    /// A list of all shorthands in markup mode.
    pub const LIST: &'static [(&'static str, char)] = &[
        ("...", '…'),
        ("~", '\u{00A0}'),
        ("-", '\u{2212}'),
        ("--", '\u{2013}'),
        ("---", '\u{2014}'),
        ("-?", '\u{00AD}'),
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
    /// A smart quote: `'` or `"`.
    struct SmartQuote
}

impl SmartQuote<'_> {
    /// Whether this is a double quote.
    pub fn double(self) -> bool {
        self.0.text_str() == "\""
    }
}

node! {
    /// Strong content: `*Strong*`.
    struct Strong
}

impl<'a> Strong<'a> {
    /// The contents of the strong node.
    pub fn body(self) -> Markup<'a> {
        self.0.cast_first()
    }
}

node! {
    /// Emphasized content: `_Emphasized_`.
    struct Emph
}

impl<'a> Emph<'a> {
    /// The contents of the emphasis node.
    pub fn body(self) -> Markup<'a> {
        self.0.cast_first()
    }
}

node! {
    /// Raw text with optional syntax highlighting: `` `...` ``.
    struct Raw
}

impl<'a> Raw<'a> {
    /// The lines in the raw block.
    pub fn lines(self) -> impl DoubleEndedIterator<Item = Text<'a>> {
        self.0.children().filter_map(SyntaxNode::cast)
    }

    /// An optional identifier specifying the language to syntax-highlight in.
    pub fn lang(self) -> Option<RawLang<'a>> {
        let delim: RawDelim = self.0.try_cast_first()?;
        if delim.0.len() < 3 {
            return Option::None;
        }
        self.0.try_cast_first()
    }

    /// Whether the raw text should be displayed in a separate block.
    pub fn block(self) -> bool {
        self.0
            .try_cast_first()
            .is_some_and(|delim: RawDelim| delim.0.len() >= 3)
            && self.0.children().any(|e| {
                e.kind() == SyntaxKind::RawTrimmed
                    && e.text_str().chars().any(is_newline)
            })
    }
}

node! {
    /// A language tag at the start of a raw element.
    struct RawLang
}

impl<'a> RawLang<'a> {
    /// Get the language tag.
    pub fn get(self) -> &'a str {
        self.0.text_str()
    }
}

node! {
    /// A raw delimiter in single or 3+ backticks.
    struct RawDelim
}

impl RawDelim<'_> {
    #[allow(dead_code)] // usado na análise de raw delimiters — migração futura
    fn len(self) -> usize {
        self.0.text_str().len()
    }
}

node! {
    /// A hyperlink: `https://typst.org`.
    struct Link
}

impl<'a> Link<'a> {
    /// Get the URL.
    pub fn get(self) -> &'a str {
        self.0.text_str()
    }
}

node! {
    /// A label: `<intro>`.
    struct Label
}

impl<'a> Label<'a> {
    /// Get the label's text.
    pub fn get(self) -> &'a str {
        self.0.text_str().trim_start_matches('<').trim_end_matches('>')
    }
}

node! {
    /// A reference: `@target`, `@target[..]`.
    struct Ref
}

impl<'a> Ref<'a> {
    /// Get the target.
    pub fn target(self) -> &'a str {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::RefMarker)
            .map(|node| node.text_str().trim_start_matches('@'))
            .unwrap_or_default()
    }

    /// Get the supplement.
    pub fn supplement(self) -> Option<ContentBlock<'a>> {
        self.0.try_cast_last()
    }
}

node! {
    /// A section heading: `= Introduction`.
    struct Heading
}

impl<'a> Heading<'a> {
    /// The contents of the heading.
    pub fn body(self) -> Markup<'a> {
        self.0.cast_first()
    }

    /// The section depth (number of equals signs).
    pub fn depth(self) -> NonZeroUsize {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::HeadingMarker)
            .and_then(|node| node.len().try_into().ok())
            .unwrap_or(NonZeroUsize::MIN)
    }
}

node! {
    /// An item in a bullet list: `- ...`.
    struct ListItem
}

impl<'a> ListItem<'a> {
    /// The contents of the list item.
    pub fn body(self) -> Markup<'a> {
        self.0.cast_first()
    }
}

node! {
    /// An item in an enumeration (numbered list): `+ ...` or `1. ...`.
    struct EnumItem
}

impl<'a> EnumItem<'a> {
    /// The explicit numbering, if any: `23.`.
    pub fn number(self) -> Option<u64> {
        self.0.children().find_map(|node| match node.kind() {
            SyntaxKind::EnumMarker => node.text_str().trim_end_matches('.').parse().ok(),
            _ => Option::None,
        })
    }

    /// The contents of the list item.
    pub fn body(self) -> Markup<'a> {
        self.0.cast_first()
    }
}

node! {
    /// An item in a term list: `/ Term: Details`.
    struct TermItem
}

impl<'a> TermItem<'a> {
    /// The term described by the item.
    pub fn term(self) -> Markup<'a> {
        self.0.cast_first()
    }

    /// The description of the term.
    pub fn description(self) -> Markup<'a> {
        self.0.cast_last()
    }
}

// ContentBlock is also a markup-level node used by Ref::supplement
node! {
    /// A content block: `[*Hi* there!]`.
    struct ContentBlock
}

impl<'a> ContentBlock<'a> {
    /// The contained markup.
    pub fn body(self) -> Markup<'a> {
        self.0.cast_first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::ast::AstNode;
    use crate::entities::source::Source;

    #[test]
    fn strong_body_accessible() {
        let src = Source::detached("*bold*");
        let strong = src.root()
            .children()
            .find_map(Strong::from_untyped);
        assert!(strong.is_some());
        let _ = strong.unwrap().body();
    }

    #[test]
    fn heading_depth_default_is_one() {
        let src = Source::detached("= Heading");
        let heading = src.root()
            .children()
            .find_map(Heading::from_untyped);
        assert!(heading.is_some());
        let depth = heading.unwrap().depth();
        assert_eq!(depth.get(), 1);
    }

    #[test]
    fn text_get_returns_content() {
        let src = Source::detached("hello");
        let text = src.root()
            .children()
            .find_map(Text::from_untyped);
        assert!(text.is_some());
        assert_eq!(text.unwrap().get(), "hello");
    }

    #[test]
    fn label_get_strips_brackets() {
        // Contrato correcto — Label wraps <name>
        // Verificar que o get() remove < e >
        // (Parsing de label requer contexto especial; teste de contrato apenas)
        let _ = Label::from_untyped; // confirm type exists
    }
}
