//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/parse/parser/state.md
//! @prompt-hash 06b2c01c
//! @layer L1
//! @updated 2026-09-26

use super::*;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::SyntaxErrorKind;
use crate::entities::syntax_set::SyntaxSet;
use std::ops::{Index, IndexMut};

impl AtNewline {
    /// Whether to stop at a newline or continue based on the current context.
    fn stop_at(self, Newline { column, parbreak }: Newline, kind: SyntaxKind) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            AtNewline::Continue => false,
            AtNewline::Stop => true,
            AtNewline::ContextualContinue => match kind {
                SyntaxKind::Else | SyntaxKind::Dot => false,
                _ => true,
            },
            AtNewline::StopParBreak => parbreak,
            AtNewline::RequireColumn(min_col) => {
                // When the column is `None`, the newline doesn't start a
                // column, and we continue parsing. This may happen on the
                // boundary of syntax modes, since we only report a column in
                // Markup.
                column.is_some_and(|column| column <= min_col)
            }
        }
    }
}

// Index into the parser with markers.
impl Index<Marker> for Parser<'_> {
    type Output = SyntaxNode;

    fn index(&self, m: Marker) -> &Self::Output {
        &self.nodes[m.0]
    }
}

impl IndexMut<Marker> for Parser<'_> {
    fn index_mut(&mut self, m: Marker) -> &mut Self::Output {
        &mut self.nodes[m.0]
    }
}

/// Creating/Consuming the parser and getting info about the current token.
impl<'s> Parser<'s> {
    /// Create a new parser starting from the given text offset and syntax mode.
    pub(in crate::compiler::parse) fn new(
        text: &'s str,
        offset: usize,
        mode: SyntaxMode,
    ) -> Self {
        let mut lexer = Lexer::new(text, mode);
        lexer.jump(offset);
        let nl_mode = AtNewline::Continue;
        let mut nodes = vec![];
        let token = Self::lex(&mut nodes, &mut lexer, nl_mode);
        Self {
            text,
            lexer,
            nl_mode,
            token,
            balanced: true,
            nodes,
            memo: Default::default(),
            depth: 0,
        }
    }

    /// Consume the parser, yielding the full vector of parsed SyntaxNodes.
    pub(in crate::compiler::parse) fn finish(self) -> Vec<SyntaxNode> {
        self.nodes
    }

    /// Consume the parser, generating a single top-level node.
    pub(in crate::compiler::parse) fn finish_into(self, kind: SyntaxKind) -> SyntaxNode {
        assert!(self.at(SyntaxKind::End));
        SyntaxNode::inner(kind, self.finish())
    }

    /// Similar to a `peek()` function: returns the `kind` of the next token to
    /// be eaten.
    pub(in crate::compiler::parse) fn current(&self) -> SyntaxKind {
        self.token.kind
    }

    /// Whether the current token is a given [`SyntaxKind`].
    pub(in crate::compiler::parse) fn at(&self, kind: SyntaxKind) -> bool {
        self.token.kind == kind
    }

    /// Whether the current token is contained in a [`SyntaxSet`].
    pub(in crate::compiler::parse) fn at_set(&self, set: SyntaxSet) -> bool {
        set.contains(self.token.kind)
    }

    /// Whether we're at the end of the token stream.
    ///
    /// Note: This might be a fake end due to the newline mode.
    pub(in crate::compiler::parse) fn end(&self) -> bool {
        self.at(SyntaxKind::End)
    }

    /// If we're at the given `kind` with no preceding trivia tokens.
    pub(in crate::compiler::parse) fn directly_at(&self, kind: SyntaxKind) -> bool {
        self.token.kind == kind && !self.had_trivia()
    }

    /// Whether `token` had any preceding trivia.
    pub(in crate::compiler::parse) fn had_trivia(&self) -> bool {
        self.token.n_trivia > 0
    }

    /// Whether `token` had a newline among any of its preceding trivia.
    pub(in crate::compiler::parse) fn had_newline(&self) -> bool {
        self.token.newline.is_some()
    }

    /// The number of characters until the most recent newline from the start of
    /// the current token. Uses a cached value from the newline mode if present.
    pub(in crate::compiler::parse) fn current_column(&self) -> usize {
        self.token
            .newline
            .and_then(|newline| newline.column)
            .unwrap_or_else(|| self.lexer.column(self.token.start))
    }

    /// The current token's text.
    pub(in crate::compiler::parse) fn current_text(&self) -> &'s str {
        &self.text[self.token.start..self.current_end()]
    }

    /// The offset into `text` of the current token's start.
    pub(in crate::compiler::parse) fn current_start(&self) -> usize {
        self.token.start
    }

    /// The offset into `text` of the current token's end.
    fn current_end(&self) -> usize {
        self.lexer.cursor()
    }

    /// The offset into `text` of the previous token's end.
    #[allow(dead_code)] // usado por reparse_block — migração incremental futura
    pub(in crate::compiler::parse) fn prev_end(&self) -> usize {
        self.token.prev_end
    }
}

// The main parsing interface for generating tokens and eating/modifying nodes.
impl<'s> Parser<'s> {
    /// A marker that will point to the current token in the parser once it's
    /// been eaten.
    pub(in crate::compiler::parse) fn marker(&self) -> Marker {
        Marker(self.nodes.len())
    }

    /// A marker that will point to first trivia before this token in the
    /// parser (or the token itself if no trivia precede it).
    pub(in crate::compiler::parse) fn before_trivia(&self) -> Marker {
        Marker(self.nodes.len() - self.token.n_trivia)
    }

    /// Eat the current node and return a reference for in-place mutation.
    #[track_caller]
    pub(in crate::compiler::parse) fn eat_and_get(&mut self) -> &mut SyntaxNode {
        let offset = self.nodes.len();
        self.eat();
        &mut self.nodes[offset]
    }

    /// Eat the token if at `kind`. Returns `true` if eaten.
    ///
    /// Note: In Math and Code, this will ignore trivia in front of the
    /// `kind`, To forbid skipping trivia, consider using `eat_if_direct`.
    pub(in crate::compiler::parse) fn eat_if(&mut self, kind: SyntaxKind) -> bool {
        let at = self.at(kind);
        if at {
            self.eat();
        }
        at
    }

    /// Assert that we are at the given [`SyntaxKind`] and eat it. This should
    /// be used when moving between functions that expect to start with a
    /// specific token.
    #[track_caller]
    pub(in crate::compiler::parse) fn assert(&mut self, kind: SyntaxKind) {
        assert_eq!(self.token.kind, kind);
        self.eat();
    }

    /// Convert the current token's [`SyntaxKind`] and eat it.
    pub(in crate::compiler::parse) fn convert_and_eat(&mut self, kind: SyntaxKind) {
        // Only need to replace the node here.
        self.token.node.convert_to_kind(kind);
        self.eat();
    }

    /// Eat the current token by saving it to the `nodes` vector, then move
    /// the lexer forward to prepare a new token.
    pub(in crate::compiler::parse) fn eat(&mut self) {
        self.nodes.push(std::mem::take(&mut self.token.node));
        self.token = Self::lex(&mut self.nodes, &mut self.lexer, self.nl_mode);
    }

    /// Detach the parsed trivia nodes from this token (but not newline info) so
    /// that subsequent wrapping will include the trivia.
    pub(in crate::compiler::parse) fn flush_trivia(&mut self) {
        self.token.n_trivia = 0;
        self.token.prev_end = self.token.start;
    }

    /// Wrap the nodes from a marker up to (but excluding) the current token in
    /// a new [inner node](`SyntaxNode::inner`) of the given kind. This is an
    /// easy interface for creating nested syntax nodes _after_ having parsed
    /// their children.
    pub(in crate::compiler::parse) fn wrap(&mut self, from: Marker, kind: SyntaxKind) {
        let to = self.before_trivia().0;
        let from = from.0.min(to);
        let children = self.nodes.drain(from..to).collect();
        self.nodes.insert(from, SyntaxNode::inner(kind, children));
    }

    /// Parse within the [`SyntaxMode`] for subsequent tokens (does not change the
    /// current token). This may re-lex the final token on exit.
    ///
    /// This function effectively repurposes the call stack as a stack of modes.
    pub(in crate::compiler::parse) fn enter_modes(
        &mut self,
        mode: SyntaxMode,
        stop: AtNewline,
        func: impl FnOnce(&mut Parser<'s>),
    ) {
        let previous = self.lexer.mode();
        self.lexer.set_mode(mode);
        self.with_nl_mode(stop, func);
        if mode != previous {
            // **P648/P649** — ao mudar de modo, o lexer re-lexa o token de
            // lookahead. Se esse token for um erro de lexer genuíno (número
            // hexadecimal inválido ou escape Unicode inválido), a mensagem
            // original seria perdida. Preservamo-lo selectivamente para que
            // o eval o possa propagar. Apenas estas classes são seguras:
            // outras mensagens de lexer (ex: `#` em bloco de código) marcam
            // construções válidas em contextos de markup e não podem ser
            // propagadas sem repetir as regressões de P634.
            let preserved_error = if self.token.kind == SyntaxKind::Error {
                let errors = self.token.node.errors();
                let text = self.token.node.text();
                errors.into_iter().next().and_then(|e| {
                    if matches!(
                        e.kind,
                        SyntaxErrorKind::InvalidHexNumber
                            | SyntaxErrorKind::InvalidUnicodeCodepoint
                    ) {
                        Some(SyntaxNode::error(e, text))
                    } else {
                        None
                    }
                })
            } else {
                None
            };

            self.lexer.set_mode(previous);
            self.lexer.jump(self.token.prev_end);
            self.nodes.truncate(self.nodes.len() - self.token.n_trivia);
            if let Some(node) = preserved_error {
                self.nodes.push(node);
            }
            self.token = Self::lex(&mut self.nodes, &mut self.lexer, self.nl_mode);
        }
    }

    /// Parse within the [`AtNewline`] mode for subsequent tokens (does not
    /// change the current token). This may re-lex the final token on exit.
    ///
    /// This function effectively repurposes the call stack as a stack of modes.
    pub(in crate::compiler::parse) fn with_nl_mode(
        &mut self,
        mode: AtNewline,
        func: impl FnOnce(&mut Parser<'s>),
    ) {
        let previous = self.nl_mode;
        self.nl_mode = mode;
        func(self);
        self.nl_mode = previous;
        if let Some(newline) = self.token.newline {
            if mode != previous {
                // Restore our actual token's kind or insert a fake end.
                let actual_kind = self.token.node.kind();
                if self.nl_mode.stop_at(newline, actual_kind) {
                    self.token.kind = SyntaxKind::End;
                } else {
                    self.token.kind = actual_kind;
                }
            }
        }
    }

    /// Move the lexer forward and prepare the current token. In Code, this
    /// might insert a temporary [`SyntaxKind::End`] based on our newline mode.
    ///
    /// This is not a method on `self` because we need a valid token before we
    /// can initialize the parser.
    fn lex(nodes: &mut Vec<SyntaxNode>, lexer: &mut Lexer, nl_mode: AtNewline) -> Token {
        let prev_end = lexer.cursor();
        let mut start = prev_end;
        let (mut kind, mut node) = lexer.next();
        let mut n_trivia = 0;
        let mut had_newline = false;
        let mut parbreak = false;

        while kind.is_trivia() {
            had_newline |= lexer.newline(); // Newlines are always trivia.
            parbreak |= kind == SyntaxKind::Parbreak;
            n_trivia += 1;
            nodes.push(node);
            start = lexer.cursor();
            (kind, node) = lexer.next();
        }

        let newline = if had_newline {
            let column =
                (lexer.mode() == SyntaxMode::Markup).then(|| lexer.column(start));
            let newline = Newline { column, parbreak };
            if nl_mode.stop_at(newline, kind) {
                // Insert a temporary `SyntaxKind::End` to halt the parser.
                // The actual kind will be restored from `node` later.
                kind = SyntaxKind::End;
            }
            Some(newline)
        } else {
            None
        };

        Token { kind, node, n_trivia, newline, start, prev_end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_new_smoke() {
        // Smoke test: criar um Parser não deve crashar em input vazio.
        let p = Parser::new("", 0, SyntaxMode::Markup);
        assert!(p.end());
    }

    #[test]
    fn marker_equality() {
        // Markers com o mesmo índice são iguais.
        assert_eq!(Marker(0), Marker(0));
        assert_ne!(Marker(0), Marker(1));
    }

    #[test]
    fn newline_stop_is_scoped() {
        let mut p = Parser::new("a\nb", 0, SyntaxMode::Code);
        p.with_nl_mode(AtNewline::Stop, |p| {
            p.eat();
            assert!(p.end());
        });
        assert!(!p.end());
        assert_eq!(p.current_text(), "b");
    }
}
