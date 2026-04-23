//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-04-23
//!
//! `Parser` struct e tipos de apoio (Token, Newline, AtNewline, Marker,
//! MemoArena, Checkpoint, PartialState). Extraído de `parse.rs` no
//! Passo 96.4 conforme ADR-0037 (coesão por domínio).

use std::ops::{DerefMut, Index, IndexMut, Range};

use rustc_hash::FxHashMap;

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::{SyntaxError, SyntaxNode};
use crate::entities::syntax_set::SyntaxSet;
use crate::rules::lexer::Lexer;
use crate::syntax_set;
use crate::utils::defer;

// Picked by gut feeling.
pub(super) const MAX_DEPTH: u32 = 256;

/// Manages parsing a stream of tokens into a tree of [`SyntaxNode`]s.
///
/// The implementation presents an interface that investigates a current `token`
/// with a [`SyntaxKind`] and can take one of the following actions:
///
/// 1. Eat a token: push `token` onto the `nodes` vector as a [leaf
///    node](`SyntaxNode::leaf`) and prepare a new `token` by calling into the
///    lexer.
/// 2. Wrap nodes from a marker to the end of `nodes` (excluding `token` and any
///    attached trivia) into an [inner node](`SyntaxNode::inner`) of a specific
///    `SyntaxKind`.
/// 3. Produce or convert nodes into an [error node](`SyntaxNode::error`) when
///    something expected is missing or something unexpected is found.
///
/// Overall the parser produces a nested tree of SyntaxNodes as a "_Concrete_
/// Syntax Tree." The raw Concrete Syntax Tree should contain the entire source
/// text, and is used as-is for e.g. syntax highlighting and IDE features. In
/// `ast.rs` the CST is interpreted as a lazy view over an "_Abstract_ Syntax
/// Tree." The AST module skips over irrelevant tokens -- whitespace, comments,
/// code parens, commas in function args, etc. -- as it iterates through the
/// tree.
///
/// ### Modes
///
/// The parser manages the transitions between the three modes of Typst through
/// [syntax modes](`SyntaxMode`) and [newline modes](`AtNewline`).
///
/// The syntax modes map to the three Typst modes and are stored in the lexer,
/// changing which `SyntaxKind`s it will generate.
///
/// The newline mode is used to determine whether a newline should end the
/// current expression. If so, the parser temporarily changes `token`'s kind to
/// a fake [`SyntaxKind::End`]. When the parser exits the mode the original
/// `SyntaxKind` is restored.
pub(super) struct Parser<'s> {
    /// The source text shared with the lexer.
    pub(super) text: &'s str,
    /// A lexer over the source text with multiple modes. Defines the boundaries
    /// of tokens and determines their [`SyntaxKind`]. Contains the [`SyntaxMode`]
    /// defining our current Typst mode.
    pub(super) lexer: Lexer<'s>,
    /// The newline mode: whether to insert a temporary end at newlines.
    pub(super) nl_mode: AtNewline,
    /// The current token under inspection, not yet present in `nodes`. This
    /// acts like a single item of lookahead for the parser.
    ///
    /// When wrapping, this is _not_ included in the wrapped nodes.
    pub(super) token: Token,
    /// Whether the parser has the expected set of open/close delimiters. This
    /// only ever transitions from `true` to `false`.
    pub(super) balanced: bool,
    /// Nodes representing the concrete syntax tree of previously parsed text.
    /// In Code and Math, includes previously parsed trivia, but not `token`.
    pub(super) nodes: Vec<SyntaxNode>,
    /// Parser checkpoints for a given text index. Used for efficient parser
    /// backtracking similar to packrat parsing. See comments above in
    /// [`expr_with_paren`].
    pub(super) memo: MemoArena,
    /// The current expression nesting depth.
    pub(super) depth: u32,
}

/// A single token returned from the lexer with a cached [`SyntaxKind`] and a
/// record of preceding trivia.
#[derive(Debug, Clone)]
pub(super) struct Token {
    /// The [`SyntaxKind`] of the current token.
    pub(super) kind: SyntaxKind,
    /// The [`SyntaxNode`] of the current token, ready to be eaten and pushed
    /// onto the end of `nodes`.
    pub(super) node: SyntaxNode,
    /// The number of preceding trivia before this token.
    pub(super) n_trivia: usize,
    /// Whether this token's preceding trivia contained a newline.
    pub(super) newline: Option<Newline>,
    /// The index into `text` of the start of our current token (the end is
    /// stored as the lexer's cursor).
    pub(super) start: usize,
    /// The index into `text` of the end of the previous token.
    pub(super) prev_end: usize,
}

/// Information about newlines in a group of trivia.
#[derive(Debug, Copy, Clone)]
pub(super) struct Newline {
    /// The column of the start of the next token in its line.
    pub(super) column: Option<usize>,
    /// Whether any of our newlines were paragraph breaks.
    pub(super) parbreak: bool,
}

/// How to proceed with parsing when at a newline.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum AtNewline {
    /// Continue at newlines.
    Continue,
    /// Stop at any newline.
    Stop,
    /// Continue only if there is a continuation with `else` or `.` (Code only).
    ContextualContinue,
    /// Stop only at a parbreak, not normal newlines (Markup only).
    StopParBreak,
    /// Require that the token's column be greater or equal to a column (Markup
    /// only). If this is `0`, acts like `Continue`; if this is `usize::MAX`,
    /// acts like `Stop`.
    RequireColumn(usize),
}

impl AtNewline {
    /// Whether to stop at a newline or continue based on the current context.
    pub(super) fn stop_at(self, Newline { column, parbreak }: Newline, kind: SyntaxKind) -> bool {
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

/// A marker representing a node's position in the parser. Mainly used for
/// wrapping, but can also index into the parser to access the node, like
/// `p[m]`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct Marker(pub(super) usize);

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
    pub(super) fn new(text: &'s str, offset: usize, mode: SyntaxMode) -> Self {
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
    pub(super) fn finish(self) -> Vec<SyntaxNode> {
        self.nodes
    }

    /// Consume the parser, generating a single top-level node.
    pub(super) fn finish_into(self, kind: SyntaxKind) -> SyntaxNode {
        assert!(self.at(SyntaxKind::End));
        SyntaxNode::inner(kind, self.finish())
    }

    /// Similar to a `peek()` function: returns the `kind` of the next token to
    /// be eaten.
    pub(super) fn current(&self) -> SyntaxKind {
        self.token.kind
    }

    /// Whether the current token is a given [`SyntaxKind`].
    pub(super) fn at(&self, kind: SyntaxKind) -> bool {
        self.token.kind == kind
    }

    /// Whether the current token is contained in a [`SyntaxSet`].
    pub(super) fn at_set(&self, set: SyntaxSet) -> bool {
        set.contains(self.token.kind)
    }

    /// Whether we're at the end of the token stream.
    ///
    /// Note: This might be a fake end due to the newline mode.
    pub(super) fn end(&self) -> bool {
        self.at(SyntaxKind::End)
    }

    /// If we're at the given `kind` with no preceding trivia tokens.
    pub(super) fn directly_at(&self, kind: SyntaxKind) -> bool {
        self.token.kind == kind && !self.had_trivia()
    }

    /// Whether `token` had any preceding trivia.
    pub(super) fn had_trivia(&self) -> bool {
        self.token.n_trivia > 0
    }

    /// Whether `token` had a newline among any of its preceding trivia.
    pub(super) fn had_newline(&self) -> bool {
        self.token.newline.is_some()
    }

    /// The number of characters until the most recent newline from the start of
    /// the current token. Uses a cached value from the newline mode if present.
    pub(super) fn current_column(&self) -> usize {
        self.token
            .newline
            .and_then(|newline| newline.column)
            .unwrap_or_else(|| self.lexer.column(self.token.start))
    }

    /// The current token's text.
    pub(super) fn current_text(&self) -> &'s str {
        &self.text[self.token.start..self.current_end()]
    }

    /// The offset into `text` of the current token's start.
    pub(super) fn current_start(&self) -> usize {
        self.token.start
    }

    /// The offset into `text` of the current token's end.
    pub(super) fn current_end(&self) -> usize {
        self.lexer.cursor()
    }

    /// The offset into `text` of the previous token's end.
    #[allow(dead_code)] // usado por reparse_block — migração incremental futura
    pub(super) fn prev_end(&self) -> usize {
        self.token.prev_end
    }
}

// The main parsing interface for generating tokens and eating/modifying nodes.
impl<'s> Parser<'s> {
    /// A marker that will point to the current token in the parser once it's
    /// been eaten.
    pub(super) fn marker(&self) -> Marker {
        Marker(self.nodes.len())
    }

    /// A marker that will point to first trivia before this token in the
    /// parser (or the token itself if no trivia precede it).
    pub(super) fn before_trivia(&self) -> Marker {
        Marker(self.nodes.len() - self.token.n_trivia)
    }

    /// Eat the current node and return a reference for in-place mutation.
    #[track_caller]
    pub(super) fn eat_and_get(&mut self) -> &mut SyntaxNode {
        let offset = self.nodes.len();
        self.eat();
        &mut self.nodes[offset]
    }

    /// Eat the token if at `kind`. Returns `true` if eaten.
    ///
    /// Note: In Math and Code, this will ignore trivia in front of the
    /// `kind`, To forbid skipping trivia, consider using `eat_if_direct`.
    pub(super) fn eat_if(&mut self, kind: SyntaxKind) -> bool {
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
    pub(super) fn assert(&mut self, kind: SyntaxKind) {
        assert_eq!(self.token.kind, kind);
        self.eat();
    }

    /// Convert the current token's [`SyntaxKind`] and eat it.
    pub(super) fn convert_and_eat(&mut self, kind: SyntaxKind) {
        // Only need to replace the node here.
        self.token.node.convert_to_kind(kind);
        self.eat();
    }

    /// Eat the current token by saving it to the `nodes` vector, then move
    /// the lexer forward to prepare a new token.
    pub(super) fn eat(&mut self) {
        self.nodes.push(std::mem::take(&mut self.token.node));
        self.token = Self::lex(&mut self.nodes, &mut self.lexer, self.nl_mode);
    }

    /// Detach the parsed trivia nodes from this token (but not newline info) so
    /// that subsequent wrapping will include the trivia.
    pub(super) fn flush_trivia(&mut self) {
        self.token.n_trivia = 0;
        self.token.prev_end = self.token.start;
    }

    /// Wrap the nodes from a marker up to (but excluding) the current token in
    /// a new [inner node](`SyntaxNode::inner`) of the given kind. This is an
    /// easy interface for creating nested syntax nodes _after_ having parsed
    /// their children.
    pub(super) fn wrap(&mut self, from: Marker, kind: SyntaxKind) {
        let to = self.before_trivia().0;
        let from = from.0.min(to);
        let children = self.nodes.drain(from..to).collect();
        self.nodes.insert(from, SyntaxNode::inner(kind, children));
    }

    /// Wrap the nodes from a marker up to (but excluding) the current token in
    /// a new [error node](`SyntaxNode::error`) with the given message. This is
    /// an easy interface for creating a syntax error _after_ having parsed its
    /// children.
    pub(super) fn wrap_error(&mut self, from: Marker, message: impl Into<String>) {
        let to = self.before_trivia().0;
        let from = from.0.min(to);
        let mut s = String::new();
        for node in self.nodes.drain(from..to) {
            s.push_str(node.into_text().as_str());
        }
        self.nodes.insert(
            from,
            SyntaxNode::error(SyntaxError::new(message.into()), s.as_str()),
        );
    }

    /// Parse within the [`SyntaxMode`] for subsequent tokens (does not change the
    /// current token). This may re-lex the final token on exit.
    ///
    /// This function effectively repurposes the call stack as a stack of modes.
    pub(super) fn enter_modes(
        &mut self,
        mode: SyntaxMode,
        stop: AtNewline,
        func: impl FnOnce(&mut Parser<'s>),
    ) {
        let previous = self.lexer.mode();
        self.lexer.set_mode(mode);
        self.with_nl_mode(stop, func);
        if mode != previous {
            self.lexer.set_mode(previous);
            self.lexer.jump(self.token.prev_end);
            self.nodes.truncate(self.nodes.len() - self.token.n_trivia);
            self.token = Self::lex(&mut self.nodes, &mut self.lexer, self.nl_mode);
        }
    }

    /// Parse within the [`AtNewline`] mode for subsequent tokens (does not
    /// change the current token). This may re-lex the final token on exit.
    ///
    /// This function effectively repurposes the call stack as a stack of modes.
    pub(super) fn with_nl_mode(&mut self, mode: AtNewline, func: impl FnOnce(&mut Parser<'s>)) {
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
    pub(super) fn lex(nodes: &mut Vec<SyntaxNode>, lexer: &mut Lexer, nl_mode: AtNewline) -> Token {
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

/// Extra parser state for efficiently recovering from mispredicted parses.
///
/// This is the same idea as packrat parsing, but we use it only in the limited
/// case of parenthesized structures. See [`expr_with_paren`] for more.
#[derive(Default)]
pub(super) struct MemoArena {
    /// A single arena of previously parsed nodes (to reduce allocations).
    /// Memoized ranges refer to unique sections of the arena.
    pub(super) arena: Vec<SyntaxNode>,
    /// A map from the parser's current position to a range of previously parsed
    /// nodes in the arena and a checkpoint of the parser's state. These allow
    /// us to reset the parser to avoid parsing the same location again.
    pub(super) memo_map: FxHashMap<MemoKey, (Range<usize>, PartialState)>,
}

/// A type alias for the memo key so it doesn't get confused with other usizes.
///
/// The memo is keyed by the index into `text` of the current token's start.
type MemoKey = usize;

/// A checkpoint of the parser which can fully restore it to a previous state.
pub(super) struct Checkpoint {
    pub(super) node_len: usize,
    pub(super) state: PartialState,
}

/// State needed to restore the parser's current token and the lexer (but not
/// the nodes vector).
#[derive(Clone)]
pub(super) struct PartialState {
    pub(super) cursor: usize,
    pub(super) lex_mode: SyntaxMode,
    pub(super) token: Token,
}

/// The Memoization interface.
impl Parser<'_> {
    /// Store the already parsed nodes and the parser state into the memo map by
    /// extending the arena and storing the extended range and a checkpoint.
    pub(super) fn memoize_parsed_nodes(&mut self, key: MemoKey, prev_len: usize) {
        let Checkpoint { state, node_len } = self.checkpoint();
        let memo_start = self.memo.arena.len();
        self.memo.arena.extend_from_slice(&self.nodes[prev_len..node_len]);
        let arena_range = memo_start..self.memo.arena.len();
        self.memo.memo_map.insert(key, (arena_range, state));
    }

    /// Try to load a memoized result, return `None` if we did or `Some` (with a
    /// checkpoint and a key for the memo map) if we didn't.
    pub(super) fn restore_memo_or_checkpoint(&mut self) -> Option<(MemoKey, Checkpoint)> {
        // We use the starting index of the current token as our key.
        let key: MemoKey = self.current_start();
        match self.memo.memo_map.get(&key).cloned() {
            Some((range, state)) => {
                self.nodes.extend_from_slice(&self.memo.arena[range]);
                // It's important that we don't truncate the nodes vector since
                // it may have grown or shrunk (due to other memoization or
                // error reporting) since we made this checkpoint.
                self.restore_partial(state);
                None
            }
            None => Some((key, self.checkpoint())),
        }
    }

    /// Restore the parser to the state at a checkpoint.
    pub(super) fn restore(&mut self, checkpoint: Checkpoint) {
        self.nodes.truncate(checkpoint.node_len);
        self.restore_partial(checkpoint.state);
    }

    /// Restore parts of the checkpoint excluding the nodes vector.
    pub(super) fn restore_partial(&mut self, state: PartialState) {
        self.lexer.jump(state.cursor);
        self.lexer.set_mode(state.lex_mode);
        self.token = state.token;
    }

    /// Save a checkpoint of the parser state.
    pub(super) fn checkpoint(&self) -> Checkpoint {
        let node_len = self.nodes.len();
        let state = PartialState {
            cursor: self.lexer.cursor(),
            lex_mode: self.lexer.mode(),
            token: self.token.clone(),
        };
        Checkpoint { node_len, state }
    }
}

/// Functions for eating expected or unexpected tokens and generating errors if
/// we don't get what we expect.
impl Parser<'_> {
    /// Consume the given `kind` or produce an error.
    pub(super) fn expect(&mut self, kind: SyntaxKind) -> bool {
        let at = self.at(kind);
        if at {
            self.eat();
        } else if kind == SyntaxKind::Ident && self.token.kind.is_keyword() {
            self.trim_errors();
            self.eat_and_get().expected(kind.name());
        } else {
            self.balanced &= !kind.is_grouping();
            self.expected(kind.name());
        }
        at
    }

    /// Consume the given closing delimiter or produce an error for the matching
    /// opening delimiter at `open`.
    #[track_caller]
    pub(super) fn expect_closing_delimiter(&mut self, open: Marker, kind: SyntaxKind) {
        if !self.eat_if(kind) {
            self.nodes[open.0].convert_to_error("unclosed delimiter");
        }
    }

    /// Produce an error that the given `thing` was expected.
    pub(super) fn expected(&mut self, thing: &str) {
        if !self.after_error() {
            self.expected_at(self.before_trivia(), thing);
        }
    }

    /// Whether the last non-trivia node is an error.
    pub(super) fn after_error(&mut self) -> bool {
        let m = self.before_trivia();
        m.0 > 0 && self.nodes[m.0 - 1].kind().is_error()
    }

    /// Produce an error that the given `thing` was expected at the position
    /// of the marker `m`.
    pub(super) fn expected_at(&mut self, m: Marker, thing: &str) {
        let error =
            SyntaxNode::error(SyntaxError::new(format!("expected {thing}")), "");
        self.nodes.insert(m.0, error);
    }

    /// Add a hint to a trailing error.
    pub(super) fn hint(&mut self, hint: &str) {
        let m = self.before_trivia();
        if let Some(error) = self.nodes.get_mut(m.0 - 1) {
            error.hint(hint);
        }
    }

    /// Consume the next token (if any) and produce an error stating that it was
    /// unexpected.
    pub(super) fn unexpected(&mut self) {
        self.trim_errors();
        self.balanced &= !self.token.kind.is_grouping();
        self.eat_and_get().unexpected();
    }

    /// Remove trailing errors with zero length.
    pub(super) fn trim_errors(&mut self) {
        let Marker(end) = self.before_trivia();
        let mut start = end;
        while start > 0
            && self.nodes[start - 1].kind().is_error()
            && self.nodes[start - 1].is_empty()
        {
            start -= 1;
        }
        self.nodes.drain(start..end);
    }

    /// Check if the maximum depth has been exceeded. If so, generate an error
    /// and try to make a best effort recovery using the `stop_set` as a guide.
    ///
    /// This function isn't strictly necessary, but it is an optimization to
    /// generate one combined error instead of an error message for every
    /// balanced set of tokens. In the pathological case an error would be
    /// generated by [`Self::increase_depth`] for every single token until
    /// the `stop_set` of the parent function is reached.
    pub(super) fn check_depth_until(&mut self, stop_set: SyntaxSet) -> Option<&mut Self> {
        if self.depth < MAX_DEPTH {
            Some(self)
        } else {
            self.depth_check_error(Some(stop_set));
            None
        }
    }

    /// Check if the maximum depth has been exceeded. If so, generate an error
    /// and try to make a best effort recovery. Otherwise increase the depth and
    /// return a handle that automatically decreases it again when dropped.
    pub(super) fn increase_depth(&mut self) -> Option<impl DerefMut<Target = Self> + '_> {
        if self.depth < MAX_DEPTH {
            self.depth += 1;
            Some(defer(self, |p| p.depth -= 1))
        } else {
            self.depth_check_error(None);
            None
        }
    }

    /// Generate an error for an exceeded maximum depth check.
    pub(super) fn depth_check_error(&mut self, stop_set: Option<SyntaxSet>) {
        let m = self.marker();

        let mut balance: usize = 0;
        // This function has to guarantee some sort of forward progress,
        // otherwise the parser might loop indefinitely. One token is eaten in
        // all cases, if that token is an opening delimiter, try to balance the
        // opening and closing grouping delimiters before continuing.
        self.with_nl_mode(AtNewline::Continue, |p| {
            loop {
                if p.at_set(syntax_set!(LeftBracket, LeftBrace, LeftParen)) {
                    balance = balance.saturating_add(1);
                } else if p.at_set(syntax_set!(RightBracket, RightBrace, RightParen)) {
                    balance = balance.saturating_sub(1);
                }
                p.eat();

                let at_stop = stop_set.is_none_or(|s| p.at_set(s));
                if (balance == 0 && at_stop) || p.end() {
                    break;
                }
            }
        });

        self.wrap_error(m, "maximum parsing depth exceeded");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_new_smoke() {
        // Smoke test: criar um Parser não deve crashar em input vazio.
        let mut p = Parser::new("", 0, SyntaxMode::Markup);
        assert!(p.end());
    }

    #[test]
    fn marker_equality() {
        // Markers com o mesmo índice são iguais.
        assert_eq!(Marker(0), Marker(0));
        assert_ne!(Marker(0), Marker(1));
    }
}
