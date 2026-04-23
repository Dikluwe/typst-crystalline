//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/lexer/mod.md
//! @prompt-hash 91498f0f
//! @layer L1
//! @updated 2026-04-23
//!
//! Lexer do Typst — modo `Math`. Extraído de `lexer/mod.rs` no
//! Passo 96.9 conforme ADR-0037.


use unicode_segmentation::UnicodeSegmentation;

use crate::entities::math_class::default_math_class;
use crate::entities::math_class::MathClass;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::{SyntaxError, SyntaxNode};

use super::{is_id_continue, is_id_start,
    is_math_id_continue, is_math_id_start, Lexer};

/// Math.
impl Lexer<'_> {
    pub(super) fn math(&mut self, start: usize, c: char) -> (SyntaxKind, Option<SyntaxNode>) {
        let kind = match c {
            '\\' => self.backslash(),
            '"' => self.string(),

            '-' if self.s.eat_if(">>") => SyntaxKind::MathShorthand,
            '-' if self.s.eat_if('>') => SyntaxKind::MathShorthand,
            '-' if self.s.eat_if("->") => SyntaxKind::MathShorthand,
            ':' if self.s.eat_if('=') => SyntaxKind::MathShorthand,
            ':' if self.s.eat_if(":=") => SyntaxKind::MathShorthand,
            '!' if self.s.eat_if('=') => SyntaxKind::MathShorthand,
            '.' if self.s.eat_if("..") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("==>") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("-->") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("--") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("-<") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("->") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("<-") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("<<") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("=>") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("==") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if("~~") => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if('=') => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if('<') => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if('-') => SyntaxKind::MathShorthand,
            '<' if self.s.eat_if('~') => SyntaxKind::MathShorthand,
            '>' if self.s.eat_if("->") => SyntaxKind::MathShorthand,
            '>' if self.s.eat_if(">>") => SyntaxKind::MathShorthand,
            '=' if self.s.eat_if("=>") => SyntaxKind::MathShorthand,
            '=' if self.s.eat_if('>') => SyntaxKind::MathShorthand,
            '=' if self.s.eat_if(':') => SyntaxKind::MathShorthand,
            '>' if self.s.eat_if('=') => SyntaxKind::MathShorthand,
            '>' if self.s.eat_if('>') => SyntaxKind::MathShorthand,
            '|' if self.s.eat_if("->") => SyntaxKind::MathShorthand,
            '|' if self.s.eat_if("=>") => SyntaxKind::MathShorthand,
            '|' if self.s.eat_if('|') => SyntaxKind::MathShorthand,
            '~' if self.s.eat_if("~>") => SyntaxKind::MathShorthand,
            '~' if self.s.eat_if('>') => SyntaxKind::MathShorthand,
            '*' | '-' | '~' => SyntaxKind::MathShorthand,

            '.' => SyntaxKind::Dot,
            ',' => SyntaxKind::Comma,
            ';' => SyntaxKind::Semicolon,

            '#' => SyntaxKind::Hash,
            '_' => SyntaxKind::Underscore,
            '$' => SyntaxKind::Dollar,
            '/' => SyntaxKind::Slash,
            '^' => SyntaxKind::Hat,
            '&' => SyntaxKind::MathAlignPoint,
            '√' | '∛' | '∜' => SyntaxKind::Root,
            '!' => SyntaxKind::Bang,

            '\'' => {
                self.s.eat_while('\'');
                SyntaxKind::MathPrimes
            }

            // We lex delimiters as `{Left,Right}{Brace,Paren}` and convert back
            // to `MathText` or `MathShorthand` in the parser.
            '(' => SyntaxKind::LeftParen,
            ')' => SyntaxKind::RightParen,
            // TODO: We may instead want to add `MathOpening` and `MathClosing`
            // kinds for these.
            '[' if self.s.eat_if('|') => SyntaxKind::LeftBrace,
            '|' if self.s.eat_if(']') => SyntaxKind::RightBrace,
            c if default_math_class(c) == Some(MathClass::Opening) => {
                SyntaxKind::LeftBrace
            }
            c if default_math_class(c) == Some(MathClass::Closing) => {
                SyntaxKind::RightBrace
            }

            // Identifiers.
            c if is_math_id_start(c) && self.s.at(is_math_id_continue) => {
                self.s.eat_while(is_math_id_continue);
                let (last_index, _) =
                    self.s.from(start).grapheme_indices(true).next_back().unwrap();
                if last_index == 0 {
                    // If this was just a single grapheme.
                    SyntaxKind::MathText
                } else {
                    let (kind, node) = self.math_ident_or_field(start);
                    return (kind, Some(node));
                }
            }

            // Other math atoms.
            _ => self.math_text(start, c),
        };
        (kind, None)
    }

    /// Parse a single `MathIdent` or an entire `FieldAccess`.
    fn math_ident_or_field(&mut self, start: usize) -> (SyntaxKind, SyntaxNode) {
        let mut kind = SyntaxKind::MathIdent;
        let mut node = SyntaxNode::leaf(kind, self.s.from(start));
        while let Some(ident) = self.maybe_dot_ident() {
            kind = SyntaxKind::FieldAccess;
            let field_children = vec![
                node,
                SyntaxNode::leaf(SyntaxKind::Dot, "."),
                SyntaxNode::leaf(SyntaxKind::Ident, ident),
            ];
            node = SyntaxNode::inner(kind, field_children);
        }
        (kind, node)
    }

    /// If at a dot and a math identifier, eat and return the identifier.
    fn maybe_dot_ident(&mut self) -> Option<&str> {
        if self.s.scout(1).is_some_and(is_math_id_start) && self.s.eat_if('.') {
            let ident_start = self.s.cursor();
            self.s.eat();
            self.s.eat_while(is_math_id_continue);
            Some(self.s.from(ident_start))
        } else {
            None
        }
    }

    fn math_text(&mut self, start: usize, c: char) -> SyntaxKind {
        // Keep numbers and grapheme clusters together.
        if c.is_numeric() {
            self.s.eat_while(char::is_numeric);
            let mut s = self.s;
            if s.eat_if('.') && !s.eat_while(char::is_numeric).is_empty() {
                self.s = s;
            }
        } else {
            let len = self
                .s
                .get(start..self.s.string().len())
                .graphemes(true)
                .next()
                .map_or(0, str::len);
            self.s.jump(start + len);
        }
        SyntaxKind::MathText
    }

    /// Handle named arguments in math function call.
    pub fn maybe_math_named_arg(&mut self, start: usize) -> Option<SyntaxNode> {
        let cursor = self.s.cursor();
        self.s.jump(start);
        if self.s.eat_if(is_id_start) {
            self.s.eat_while(is_id_continue);
            // Check that a colon directly follows the identifier, and not the
            // `:=` or `::=` math shorthands.
            if self.s.at(':') && !self.s.at(":=") && !self.s.at("::=") {
                // Check that the identifier is not just `_`.
                let node = if self.s.from(start) != "_" {
                    SyntaxNode::leaf(SyntaxKind::Ident, self.s.from(start))
                } else {
                    let msg = SyntaxError::new("expected identifier, found underscore");
                    SyntaxNode::error(msg, self.s.from(start))
                };
                return Some(node);
            }
        }
        self.s.jump(cursor);
        None
    }

    /// Handle spread arguments in math function call.
    pub fn maybe_math_spread_arg(&mut self, start: usize) -> Option<SyntaxNode> {
        let cursor = self.s.cursor();
        self.s.jump(start);
        if self.s.eat_if("..") {
            // We only infer a spread operator if it is not followed by:
            // - a space/trivia/end
            // - a dot (this would clash with the `...` math shorthand)
            // - an end of arg character: `,`, `;`, ')', `$` (spreads nothing)
            if !self.space_or_end() && !self.s.at(['.', ',', ';', ')', '$']) {
                let node = SyntaxNode::leaf(SyntaxKind::Dots, self.s.from(start));
                return Some(node);
            }
        }
        self.s.jump(cursor);
        None
    }
}



#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
