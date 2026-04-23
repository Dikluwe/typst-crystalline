//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/lexer/mod.md
//! @prompt-hash 91498f0f
//! @layer L1
//! @updated 2026-03-23


use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::{SyntaxError, SyntaxNode};
use crate::entities::syntax_text::SyntaxText;
use crate::rules::lexer::scanner::Scanner;
use unicode_ident::{is_xid_continue, is_xid_start};

pub mod scanner;

// Lexers por modo extraídos (Passo 96.9, ADR-0037).
mod markup;
mod math;
mod code;

/// An iterator over a source code string which returns tokens.
#[derive(Clone)]
// Visibilidade `pub(super)` nos campos (Passo 96.9): os submódulos
// `markup.rs`, `math.rs`, `code.rs` contêm impls do `Lexer<'_>` que
// acedem directamente a `s` (Scanner), `mode`, `newline` e `error` —
// todos os métodos do lexer são pequenas transições de estado sobre
// esses 4 campos. Métodos accessor/setter não agregam invariante.
// ADR-0037 Regra 3 autoriza `pub(super)` nestes casos.
pub(super) struct Lexer<'s> {
    /// The scanner: contains the underlying string and location as a "cursor".
    pub(super) s: Scanner<'s>,
    /// The mode the lexer is in. This determines which kinds of tokens it
    /// produces.
    pub(super) mode: SyntaxMode,
    /// Whether the last token contained a newline.
    pub(super) newline: bool,
    /// An error for the last token.
    pub(super) error: Option<SyntaxError>,
}

impl<'s> Lexer<'s> {
    /// Create a new lexer with the given mode and a prefix to offset column
    /// calculations.
    pub fn new(text: &'s str, mode: SyntaxMode) -> Self {
        Self {
            s: Scanner::new(text),
            mode,
            newline: false,
            error: None,
        }
    }

    /// Get the current lexing mode.
    pub fn mode(&self) -> SyntaxMode {
        self.mode
    }

    /// Change the lexing mode.
    pub fn set_mode(&mut self, mode: SyntaxMode) {
        self.mode = mode;
    }

    /// The index in the string at which the last token ends and next token
    /// will start.
    pub fn cursor(&self) -> usize {
        self.s.cursor()
    }

    /// Jump to the given index in the string.
    pub fn jump(&mut self, index: usize) {
        self.s.jump(index);
    }

    /// Whether the last token contained a newline.
    pub fn newline(&self) -> bool {
        self.newline
    }

    /// The number of characters until the most recent newline from an index.
    pub fn column(&self, index: usize) -> usize {
        let mut s = self.s; // Make a new temporary scanner (cheap).
        s.jump(index);
        s.before().chars().rev().take_while(|&c| !is_newline(c)).count()
    }
}

impl Lexer<'_> {
    /// Construct a full-positioned syntax error.
    pub(super) fn error(&mut self, message: impl Into<SyntaxText>) -> SyntaxKind {
        self.error = Some(SyntaxError::new(message));
        SyntaxKind::Error
    }

    /// If the current node is an error, adds a hint.
    pub(super) fn hint(&mut self, message: impl Into<SyntaxText>) {
        if let Some(error) = &mut self.error {
            error.hints.push(message.into());
        }
    }
}

/// Shared methods with all [`SyntaxMode`].
impl Lexer<'_> {
    /// Return the next token in our text. Returns both the [`SyntaxNode`]
    /// and the raw [`SyntaxKind`] to make it more ergonomic to check the kind
    pub fn next(&mut self) -> (SyntaxKind, SyntaxNode) {
        debug_assert!(self.error.is_none());
        let start = self.s.cursor();

        self.newline = false;
        let kind = match self.s.eat() {
            Some(c) if is_space(c, self.mode) => self.whitespace(start, c),
            Some('#') if start == 0 && self.s.eat_if('!') => self.shebang(),
            Some('/') if self.s.eat_if('/') => self.line_comment(),
            Some('/') if self.s.eat_if('*') => self.block_comment(),
            Some('*') if self.s.eat_if('/') => {
                let error = self.error("unexpected end of block comment");
                self.hint(
                    "consider escaping the `*` with a backslash or \
                     opening the block comment with `/*`",
                );
                error
            }
            Some('`') if self.mode != SyntaxMode::Math => return self.raw(),
            Some(c) => match self.mode {
                SyntaxMode::Markup => self.markup(start, c),
                SyntaxMode::Math => match self.math(start, c) {
                    (kind, None) => kind,
                    (kind, Some(node)) => return (kind, node),
                },
                SyntaxMode::Code => self.code(start, c),
            },

            None => SyntaxKind::End,
        };

        let text = self.s.from(start);
        let node = match self.error.take() {
            Some(error) => SyntaxNode::error(error, text),
            None => SyntaxNode::leaf(kind, text),
        };
        (kind, node)
    }

    /// Eat whitespace characters greedily.
    fn whitespace(&mut self, start: usize, c: char) -> SyntaxKind {
        let more = self.s.eat_while(|c| is_space(c, self.mode));
        let newlines = match c {
            // Optimize eating a single space.
            ' ' if more.is_empty() => 0,
            _ => count_newlines(self.s.from(start)),
        };

        self.newline = newlines > 0;
        if self.mode == SyntaxMode::Markup && newlines >= 2 {
            SyntaxKind::Parbreak
        } else {
            SyntaxKind::Space
        }
    }

    fn shebang(&mut self) -> SyntaxKind {
        self.s.eat_until(is_newline);
        SyntaxKind::Shebang
    }

    fn line_comment(&mut self) -> SyntaxKind {
        self.s.eat_until(is_newline);
        SyntaxKind::LineComment
    }

    fn block_comment(&mut self) -> SyntaxKind {
        let mut state = '_';
        let mut depth = 1;

        // Find the first `*/` that does not correspond to a nested `/*`.
        while let Some(c) = self.s.eat() {
            state = match (state, c) {
                ('*', '/') => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    '_'
                }
                ('/', '*') => {
                    depth += 1;
                    '_'
                }
                _ => c,
            }
        }

        SyntaxKind::BlockComment
    }
}

/// Try to parse an identifier into a keyword.
pub(super) fn keyword(ident: &str) -> Option<SyntaxKind> {
    Some(match ident {
        "none" => SyntaxKind::None,
        "auto" => SyntaxKind::Auto,
        "true" => SyntaxKind::Bool,
        "false" => SyntaxKind::Bool,
        "not" => SyntaxKind::Not,
        "and" => SyntaxKind::And,
        "or" => SyntaxKind::Or,
        "let" => SyntaxKind::Let,
        "set" => SyntaxKind::Set,
        "show" => SyntaxKind::Show,
        "context" => SyntaxKind::Context,
        "if" => SyntaxKind::If,
        "else" => SyntaxKind::Else,
        "for" => SyntaxKind::For,
        "in" => SyntaxKind::In,
        "while" => SyntaxKind::While,
        "break" => SyntaxKind::Break,
        "continue" => SyntaxKind::Continue,
        "return" => SyntaxKind::Return,
        "import" => SyntaxKind::Import,
        "include" => SyntaxKind::Include,
        "as" => SyntaxKind::As,
        _ => return None,
    })
}

pub(super) trait ScannerExt {
    fn advance(&mut self, by: usize);
    fn eat_newline(&mut self) -> bool;
}

impl ScannerExt for Scanner<'_> {
    fn advance(&mut self, by: usize) {
        self.jump(self.cursor() + by);
    }

    fn eat_newline(&mut self) -> bool {
        let ate = self.eat_if(is_newline);
        if ate && self.before().ends_with('\r') {
            self.eat_if('\n');
        }
        ate
    }
}

/// Whether a character will become a [`SyntaxKind::Space`] token.
#[inline]
pub(super) fn is_space(character: char, mode: SyntaxMode) -> bool {
    match mode {
        SyntaxMode::Markup => matches!(character, ' ' | '\t') || is_newline(character),
        _ => character.is_whitespace(),
    }
}

/// Whether a character is interpreted as a newline by Typst.
#[inline]
pub fn is_newline(character: char) -> bool {
    matches!(
        character,
        // Line Feed, Vertical Tab, Form Feed, Carriage Return.
        '\n' | '\x0B' | '\x0C' | '\r' |
        // Next Line, Line Separator, Paragraph Separator.
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

/// Extracts a prefix of the text that is a link and also returns whether the
/// parentheses and brackets in the link were balanced.
pub fn link_prefix(text: &str) -> (&str, bool) {
    let mut s = crate::rules::lexer::scanner::Scanner::new(text);
    let mut brackets = Vec::new();

    #[rustfmt::skip]
    s.eat_while(|c: char| {
        match c {
            | '0' ..= '9'
            | 'a' ..= 'z'
            | 'A' ..= 'Z'
            | '!' | '#' | '$' | '%' | '&' | '*' | '+'
            | ',' | '-' | '.' | '/' | ':' | ';' | '='
            | '?' | '@' | '_' | '~' | '\'' => true,
            '[' => {
                brackets.push(b'[');
                true
            }
            '(' => {
                brackets.push(b'(');
                true
            }
            ']' => brackets.pop() == Some(b'['),
            ')' => brackets.pop() == Some(b'('),
            _ => false,
        }
    });

    // Don't include the trailing characters likely to be part of text.
    while matches!(s.scout(-1), Some('!' | ',' | '.' | ':' | ';' | '?' | '\'')) {
        s.uneat();
    }

    (s.before(), brackets.is_empty())
}

/// Split text at newlines. These newline characters are not kept.
pub fn split_newlines(text: &str) -> Vec<&str> {
    let mut s = Scanner::new(text);
    let mut lines = Vec::new();
    let mut start = 0;
    let mut end = 0;

    while let Some(c) = s.eat() {
        if is_newline(c) {
            if c == '\r' {
                s.eat_if('\n');
            }

            lines.push(&text[start..end]);
            start = s.cursor();
        }
        end = s.cursor();
    }

    lines.push(&text[start..]);
    lines
}

/// Count the number of newlines in text.
pub(super) fn count_newlines(text: &str) -> usize {
    let mut newlines = 0;
    let mut s = Scanner::new(text);
    while let Some(c) = s.eat() {
        if is_newline(c) {
            if c == '\r' {
                s.eat_if('\n');
            }
            newlines += 1;
        }
    }
    newlines
}

/// Whether a string is a valid Typst identifier.
///
/// In addition to what is specified in the [Unicode Standard][uax31], we allow:
/// - `_` as a starting character,
/// - `_` and `-` as continuing characters.
///
/// [uax31]: http://www.unicode.org/reports/tr31/
#[inline]
pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    chars
        .next()
        .is_some_and(|c| is_id_start(c) && chars.all(is_id_continue))
}

/// Whether a character can start an identifier.
#[inline]
pub fn is_id_start(c: char) -> bool {
    is_xid_start(c) || c == '_'
}

/// Whether a character can continue an identifier.
#[inline]
pub fn is_id_continue(c: char) -> bool {
    is_xid_continue(c) || c == '_' || c == '-'
}

/// Whether a character can start an identifier in math.
#[inline]
pub(super) fn is_math_id_start(c: char) -> bool {
    is_xid_start(c)
}

/// Whether a character can continue an identifier in math.
#[inline]
pub(super) fn is_math_id_continue(c: char) -> bool {
    is_xid_continue(c) && c != '_'
}

/// Whether a character can be part of a label literal's name.
#[inline]
pub(super) fn is_valid_in_label_literal(c: char) -> bool {
    is_id_continue(c) || matches!(c, ':' | '.')
}

/// Returns true if this string is valid in a label literal.
pub fn is_valid_label_literal_id(id: &str) -> bool {
    !id.is_empty() && id.chars().all(is_valid_in_label_literal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::syntax_kind::SyntaxKind;
    use crate::entities::syntax_mode::SyntaxMode;

    fn lex_all(text: &str, mode: SyntaxMode) -> Vec<SyntaxKind> {
        let mut lexer = Lexer::new(text, mode);
        let mut kinds = Vec::new();
        loop {
            let (kind, _node) = lexer.next();
            kinds.push(kind);
            if kind == SyntaxKind::End { break; }
        }
        kinds
    }

    #[test]
    fn is_newline_lf() {
        assert!(is_newline('\n'));
        assert!(is_newline('\r'));
        assert!(!is_newline(' '));
        assert!(!is_newline('a'));
    }

    #[test]
    fn is_id_start_basic() {
        assert!(is_id_start('a'));
        assert!(is_id_start('_'));
        assert!(!is_id_start('1'));
        assert!(!is_id_start('-'));
    }

    #[test]
    fn is_id_continue_basic() {
        assert!(is_id_continue('a'));
        assert!(is_id_continue('_'));
        assert!(is_id_continue('1'));
        assert!(is_id_continue('-'));
        assert!(!is_id_continue(' '));
    }

    #[test]
    fn is_valid_label_literal_id_basic() {
        assert!(is_valid_label_literal_id("foo"));
        assert!(is_valid_label_literal_id("foo:bar"));
        assert!(is_valid_label_literal_id("foo.bar"));
        assert!(!is_valid_label_literal_id(""));
        assert!(!is_valid_label_literal_id("foo bar"));
    }

    #[test]
    fn lex_markup_text() {
        let kinds = lex_all("hello", SyntaxMode::Markup);
        assert_eq!(kinds[0], SyntaxKind::Text);
        assert_eq!(*kinds.last().unwrap(), SyntaxKind::End);
    }

    #[test]
    fn lex_empty_produces_end() {
        let kinds = lex_all("", SyntaxMode::Markup);
        assert_eq!(kinds, vec![SyntaxKind::End]);
    }

    #[test]
    fn link_prefix_simple() {
        let (prefix, balanced) = link_prefix("https://example.com rest");
        assert!(!prefix.is_empty());
        assert!(balanced);
    }
}
