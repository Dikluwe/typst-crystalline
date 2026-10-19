//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/parse/parser.md
//! @prompt-hash a70cd194
//! @layer L1
//! @updated 2026-09-26

//! Tipos compartilhados e composição do motor; algoritmos nos módulos filhos.

use crate::compiler::lexer::Lexer;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::SyntaxNode;

mod checkpoints;
mod diagnostics;
mod state;

use checkpoints::MemoArena;

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
//
// Regra 3 (ADR-0037): os campos `pub(super)` abaixo (`text`, `lexer`, `token`,
// `balanced`, `nodes`, `memo`, `depth`) são acedidos por todos os submódulos
// de `parse/` — cada passagem do parser lê/escreve `token.node`, faz `nodes.push`,
// testa `balanced`, e consulta `lexer.cursor()` dezenas de vezes. Criar
// getter/setter por cada acesso triplicaria a linha count sem preservar
// invariante adicional. Bulk replace do Passo 96.4 validado no Passo 97
// (auditoria DEBT-47): cada campo referenciado em pelo menos 2 submódulos.
// `nl_mode` foi privatizado (acesso só via `with_nl_mode`).
pub(super) struct Parser<'s> {
    /// The source text shared with the lexer.
    pub(super) text: &'s str,
    /// A lexer over the source text with multiple modes. Defines the boundaries
    /// of tokens and determines their [`SyntaxKind`]. Contains the [`SyntaxMode`]
    /// defining our current Typst mode.
    pub(super) lexer: Lexer<'s>,
    /// The newline mode: whether to insert a temporary end at newlines.
    nl_mode: AtNewline,
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
    n_trivia: usize,
    /// Whether this token's preceding trivia contained a newline.
    newline: Option<Newline>,
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
    column: Option<usize>,
    /// Whether any of our newlines were paragraph breaks.
    parbreak: bool,
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

/// A marker representing a node's position in the parser. Mainly used for
/// wrapping, but can also index into the parser to access the node, like
/// `p[m]`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct Marker(pub(super) usize);
