//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/parse/parser/checkpoints.md
//! @prompt-hash 8c2e2670
//! @layer L1
//! @updated 2026-09-26

use super::*;
use crate::entities::syntax_mode::SyntaxMode;
use rustc_hash::FxHashMap;
use std::ops::Range;

/// Extra parser state for efficiently recovering from mispredicted parses.
///
/// This is the same idea as packrat parsing, but we use it only in the limited
/// case of parenthesized structures. See [`expr_with_paren`] for more.
#[derive(Default)]
pub(in crate::compiler::parse) struct MemoArena {
    /// A single arena of previously parsed nodes (to reduce allocations).
    /// Memoized ranges refer to unique sections of the arena.
    arena: Vec<SyntaxNode>,
    /// A map from the parser's current position to a range of previously parsed
    /// nodes in the arena and a checkpoint of the parser's state. These allow
    /// us to reset the parser to avoid parsing the same location again.
    memo_map: FxHashMap<MemoKey, (Range<usize>, PartialState)>,
}

/// A type alias for the memo key so it doesn't get confused with other usizes.
///
/// The memo is keyed by the index into `text` of the current token's start.
type MemoKey = usize;

/// A checkpoint of the parser which can fully restore it to a previous state.
pub(in crate::compiler::parse) struct Checkpoint {
    pub(in crate::compiler::parse) node_len: usize,
    pub(in crate::compiler::parse) state: PartialState,
}

/// State needed to restore the parser's current token and the lexer (but not
/// the nodes vector).
#[derive(Clone)]
pub(in crate::compiler::parse) struct PartialState {
    cursor: usize,
    lex_mode: SyntaxMode,
    pub(in crate::compiler::parse) token: Token,
}

/// The Memoization interface.
impl Parser<'_> {
    /// Store the already parsed nodes and the parser state into the memo map by
    /// extending the arena and storing the extended range and a checkpoint.
    pub(in crate::compiler::parse) fn memoize_parsed_nodes(
        &mut self,
        key: MemoKey,
        prev_len: usize,
    ) {
        let Checkpoint { state, node_len } = self.checkpoint();
        let memo_start = self.memo.arena.len();
        self.memo.arena.extend_from_slice(&self.nodes[prev_len..node_len]);
        let arena_range = memo_start..self.memo.arena.len();
        self.memo.memo_map.insert(key, (arena_range, state));
    }

    /// Try to load a memoized result, return `None` if we did or `Some` (with a
    /// checkpoint and a key for the memo map) if we didn't.
    pub(in crate::compiler::parse) fn restore_memo_or_checkpoint(
        &mut self,
    ) -> Option<(MemoKey, Checkpoint)> {
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
    pub(in crate::compiler::parse) fn restore(&mut self, checkpoint: Checkpoint) {
        self.nodes.truncate(checkpoint.node_len);
        self.restore_partial(checkpoint.state);
    }

    /// Restore parts of the checkpoint excluding the nodes vector.
    fn restore_partial(&mut self, state: PartialState) {
        self.lexer.jump(state.cursor);
        self.lexer.set_mode(state.lex_mode);
        self.token = state.token;
    }

    /// Save a checkpoint of the parser state.
    pub(in crate::compiler::parse) fn checkpoint(&self) -> Checkpoint {
        let node_len = self.nodes.len();
        let state = PartialState {
            cursor: self.lexer.cursor(),
            lex_mode: self.lexer.mode(),
            token: self.token.clone(),
        };
        Checkpoint { node_len, state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_restores_lookahead_and_consumed_text() {
        let mut p = Parser::new("a + b", 0, SyntaxMode::Code);
        let checkpoint = p.checkpoint();
        p.eat();
        p.eat();
        p.restore(checkpoint);
        assert_eq!(p.current_text(), "a");
        assert_eq!(p.current_start(), 0);
        assert!(p.nodes.is_empty());
        while !p.end() {
            p.eat();
        }
        let text: String =
            p.finish().into_iter().map(|n| n.into_text().to_string()).collect();
        assert_eq!(text, "a + b");
    }

    #[test]
    fn memo_hit_replays_nodes_and_restores_lookahead() {
        let mut p = Parser::new("a + b", 0, SyntaxMode::Code);
        let (key, checkpoint) = p.restore_memo_or_checkpoint().unwrap();
        p.eat();
        let next = p.current_start();
        p.memoize_parsed_nodes(key, checkpoint.node_len);
        p.restore(checkpoint);
        p.expected("expression");
        assert!(p.restore_memo_or_checkpoint().is_none());
        assert_eq!(p.current_start(), next);
        assert!(p.nodes[0].kind().is_error());
        let text: String =
            p.nodes.iter().cloned().map(|n| n.into_text().to_string()).collect();
        assert_eq!(text, "a ");
    }
}
