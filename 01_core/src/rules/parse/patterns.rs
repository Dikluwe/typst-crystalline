//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-04-23
//!
//! Parse de expressões com parêntesis (arrays/dicts/paren/params),
//! argumentos, parâmetros e padrões de destructuring.
//! Extraído de `parse.rs` no Passo 96.4 conforme ADR-0037.

use rustc_hash::FxHashSet;
use std::mem;

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::syntax_set as set;

use super::parser::{AtNewline, Parser};
use super::code::{code_expr, code_expr_prec, content_block};

/// An expression that starts with a parenthesis.
pub(super) fn expr_with_paren(p: &mut Parser, atomic: bool) {
    if atomic {
        // Atomic expressions aren't modified by operators that follow them, so
        // our first guess of array/dict will be correct.
        parenthesized_or_array_or_dict(p);
        return;
    }

    // If we've seen this position before and have a memoized result, restore it
    // and return. Otherwise, get a key to this position and a checkpoint to
    // restart from in case we make a wrong prediction.
    let Some((memo_key, checkpoint)) = p.restore_memo_or_checkpoint() else { return };
    // The node length from when we restored.
    let prev_len = checkpoint.node_len;

    // When we reach a '(', we can't be sure what it is. First, we attempt to
    // parse as a simple parenthesized expression, array, or dictionary as
    // these are the most likely things. We can handle all of those in a single
    // pass.
    let kind = parenthesized_or_array_or_dict(p);

    // If, however, '=>' or '=' follows, we must backtrack and reparse as either
    // a parameter list or a destructuring. To be able to do that, we created a
    // parser checkpoint before our speculative parse, which we can restore.
    //
    // However, naive backtracking has a fatal flaw: It can lead to exponential
    // parsing time if we are constantly getting things wrong in a nested
    // scenario. The particular failure case for parameter parsing is the
    // following: `(x: (x: (x) => y) => y) => y`
    //
    // Such a structure will reparse over and over again recursively, leading to
    // a running time of O(2^n) for nesting depth n. To prevent this, we perform
    // a simple trick: When we have done the mistake of picking the wrong path
    // once and have subsequently parsed correctly, we save the result of that
    // correct parsing in the `p.memo` map. When we reach the same position
    // again, we can then just restore this result. In this way, no
    // parenthesized expression is parsed more than twice, leading to a worst
    // case running time of O(2n).
    if p.at(SyntaxKind::Arrow) {
        p.restore(checkpoint);
        let m = p.marker();
        params(p);
        if !p.expect(SyntaxKind::Arrow) {
            return;
        }
        code_expr(p);
        p.wrap(m, SyntaxKind::Closure);
    } else if p.at(SyntaxKind::Eq) && kind != SyntaxKind::Parenthesized {
        p.restore(checkpoint);
        let m = p.marker();
        destructuring_or_parenthesized(p, true, &mut FxHashSet::default());
        if !p.expect(SyntaxKind::Eq) {
            return;
        }
        code_expr(p);
        p.wrap(m, SyntaxKind::DestructAssignment);
    } else {
        return;
    }

    // Memoize result if we backtracked.
    p.memoize_parsed_nodes(memo_key, prev_len);
}

/// Parses either
/// - a parenthesized expression: `(1 + 2)`, or
/// - an array: `(1, "hi", 12cm)`, or
/// - a dictionary: `(thickness: 3pt, dash: "solid")`.
pub(super) fn parenthesized_or_array_or_dict(p: &mut Parser) -> SyntaxKind {
    let mut state = GroupState {
        count: 0,
        maybe_just_parens: true,
        kind: None,
        seen: FxHashSet::default(),
    };

    // An edge case with parens is whether we can interpret a leading spread
    // expression as a dictionary, e.g. if we want `(..dict1, ..dict2)` to join
    // the two dicts.
    //
    // The issue is that we decide on the type of the parenthesized expression
    // here in the parser by the `SyntaxKind` we wrap with, instead of in eval
    // based on the type of the spread item.
    //
    // The current fix is that we allow a leading colon to force the
    // parenthesized value into a dict:
    // - `(..arr1, ..arr2)` is wrapped as an `Array`.
    // - `(: ..dict1, ..dict2)` is wrapped as a `Dict`.
    //
    // This does allow some unexpected expressions, such as `(: key: val)`, but
    // it's currently intentional.
    let m = p.marker();
    p.with_nl_mode(AtNewline::Continue, |p| {
        p.assert(SyntaxKind::LeftParen);
        if p.eat_if(SyntaxKind::Colon) {
            state.kind = Some(SyntaxKind::Dict);
        }

        while !p.current().is_terminator() {
            if !p.at_set(set::ARRAY_OR_DICT_ITEM) {
                p.unexpected();
                continue;
            }

            array_or_dict_item(p, &mut state);
            state.count += 1;

            if !p.current().is_terminator() && p.expect(SyntaxKind::Comma) {
                state.maybe_just_parens = false;
            }
        }

        p.expect_closing_delimiter(m, SyntaxKind::RightParen);
    });

    let kind = if state.maybe_just_parens && state.count == 1 {
        SyntaxKind::Parenthesized
    } else {
        state.kind.unwrap_or(SyntaxKind::Array)
    };

    p.wrap(m, kind);
    kind
}

/// State for array/dictionary parsing.
pub(super) struct GroupState {
    count: usize,
    /// Whether this is just a single expression in parens: `(a)`. Single
    /// element arrays require an explicit comma: `(a,)`, unless we're
    /// spreading: `(..a)`.
    maybe_just_parens: bool,
    /// The `SyntaxKind` to wrap as (if we've figured it out yet).
    kind: Option<SyntaxKind>,
    /// Store named arguments so we can give an error if they're repeated.
    seen: FxHashSet<String>,
}

/// Parses a single item in an array or dictionary.
pub(super) fn array_or_dict_item(p: &mut Parser, state: &mut GroupState) {
    let m = p.marker();

    if p.eat_if(SyntaxKind::Dots) {
        // Parses a spread item: `..item`.
        code_expr(p);
        p.wrap(m, SyntaxKind::Spread);
        state.maybe_just_parens = false;
        return;
    }

    code_expr(p);

    if p.eat_if(SyntaxKind::Colon) {
        // Parses a named/keyed pair: `name: item` or `"key": item`.
        code_expr(p);

        let node = &mut p[m];
        let pair_kind = match node.kind() {
            SyntaxKind::Ident => SyntaxKind::Named,
            _ => SyntaxKind::Keyed,
        };

        if let Some(key) = node_key(node) {
            if !state.seen.insert(key.clone()) {
                node.convert_to_error(format!("duplicate key: {key}"));
            }
        }

        p.wrap(m, pair_kind);
        state.maybe_just_parens = false;

        if state.kind == Some(SyntaxKind::Array) {
            p[m].expected("expression");
        } else {
            state.kind = Some(SyntaxKind::Dict);
        }
    } else {
        // Parses a positional item.
        if state.kind == Some(SyntaxKind::Dict) {
            p[m].expected("named or keyed pair");
        } else {
            state.kind = Some(SyntaxKind::Array)
        }
    }
}

/// Extract a dictionary key from a node, if it is an identifier or string.
pub(super) fn node_key(node: &SyntaxNode) -> Option<String> {
    match node.kind() {
        SyntaxKind::Ident | SyntaxKind::Str => Some(node.text().as_str().to_string()),
        _ => None,
    }
}

/// Parses a function call's argument list: `(12pt, y)`.
pub(super) fn args(p: &mut Parser) {
    if !p.directly_at(SyntaxKind::LeftParen) && !p.directly_at(SyntaxKind::LeftBracket) {
        p.expected("argument list");
        if p.at(SyntaxKind::LeftParen) || p.at(SyntaxKind::LeftBracket) {
            p.hint("there may not be any spaces before the argument list");
        }
    }

    let m = p.marker();
    if p.at(SyntaxKind::LeftParen) {
        let m2 = p.marker();
        p.with_nl_mode(AtNewline::Continue, |p| {
            p.assert(SyntaxKind::LeftParen);

            let mut seen = FxHashSet::default();
            while !p.current().is_terminator() {
                if !p.at_set(set::ARG) {
                    p.unexpected();
                    continue;
                }

                arg(p, &mut seen);

                if !p.current().is_terminator() {
                    p.expect(SyntaxKind::Comma);
                }
            }

            p.expect_closing_delimiter(m2, SyntaxKind::RightParen);
        });
    }

    while p.directly_at(SyntaxKind::LeftBracket) {
        content_block(p);
    }

    p.wrap(m, SyntaxKind::Args);
}

/// Parses a single argument in an argument list.
pub(super) fn arg<'s>(p: &mut Parser<'s>, seen: &mut FxHashSet<&'s str>) {
    let m = p.marker();

    // Parses a spread argument: `..args`.
    if p.eat_if(SyntaxKind::Dots) {
        code_expr(p);
        p.wrap(m, SyntaxKind::Spread);
        return;
    }

    // Parses a normal positional argument or an argument name.
    let was_at_expr = p.at_set(set::CODE_EXPR);
    let text = p.current_text();
    code_expr(p);

    // Parses a named argument: `thickness: 12pt`.
    if p.eat_if(SyntaxKind::Colon) {
        // Recover from bad argument name.
        if was_at_expr {
            if p[m].kind() != SyntaxKind::Ident {
                p[m].expected("identifier");
            } else if !seen.insert(text) {
                p[m].convert_to_error(format!("duplicate argument: {text}"));
            }
        }

        code_expr(p);
        p.wrap(m, SyntaxKind::Named);
    }
}

/// Parses a closure's parameters: `(x, y)`.
pub(super) fn params(p: &mut Parser) {
    let m = p.marker();
    p.with_nl_mode(AtNewline::Continue, |p| {
        p.assert(SyntaxKind::LeftParen);

        let mut seen = FxHashSet::default();
        let mut sink = false;

        while !p.current().is_terminator() {
            if !p.at_set(set::PARAM) {
                p.unexpected();
                continue;
            }

            param(p, &mut seen, &mut sink);

            if !p.current().is_terminator() {
                p.expect(SyntaxKind::Comma);
            }
        }

        p.expect_closing_delimiter(m, SyntaxKind::RightParen);
    });
    p.wrap(m, SyntaxKind::Params);
}

/// Parses a single parameter in a parameter list.
pub(super) fn param<'s>(p: &mut Parser<'s>, seen: &mut FxHashSet<&'s str>, sink: &mut bool) {
    let m = p.marker();

    // Parses argument sink: `..sink`.
    if p.eat_if(SyntaxKind::Dots) {
        if p.at_set(set::PATTERN_LEAF) {
            pattern_leaf(p, false, seen, Some("parameter"));
        }
        p.wrap(m, SyntaxKind::Spread);
        if mem::replace(sink, true) {
            p[m].convert_to_error("only one argument sink is allowed");
        }
        return;
    }

    // Parses a normal positional parameter or a parameter name.
    let was_at_pat = p.at_set(set::PATTERN);
    pattern(p, false, seen, Some("parameter"));

    // Parses a named parameter: `thickness: 12pt`.
    if p.eat_if(SyntaxKind::Colon) {
        // Recover from bad parameter name.
        if was_at_pat && p[m].kind() != SyntaxKind::Ident {
            p[m].expected("identifier");
        }

        code_expr(p);
        p.wrap(m, SyntaxKind::Named);
    }
}

/// Parses a binding or reassignment pattern.
pub(super) fn pattern<'s>(
    p: &mut Parser<'s>,
    reassignment: bool,
    seen: &mut FxHashSet<&'s str>,
    dupe: Option<&'s str>,
) {
    let Some(p) = &mut p.increase_depth() else { return };

    match p.current() {
        SyntaxKind::Underscore => p.eat(),
        SyntaxKind::LeftParen => destructuring_or_parenthesized(p, reassignment, seen),
        _ => pattern_leaf(p, reassignment, seen, dupe),
    }
}

/// Parses a destructuring pattern or just a parenthesized pattern.
pub(super) fn destructuring_or_parenthesized<'s>(
    p: &mut Parser<'s>,
    reassignment: bool,
    seen: &mut FxHashSet<&'s str>,
) {
    let mut sink = false;
    let mut count = 0;
    let mut maybe_just_parens = true;

    let m = p.marker();
    p.with_nl_mode(AtNewline::Continue, |p| {
        p.assert(SyntaxKind::LeftParen);

        while !p.current().is_terminator() {
            if !p.at_set(set::DESTRUCTURING_ITEM) {
                p.unexpected();
                continue;
            }

            destructuring_item(p, reassignment, seen, &mut maybe_just_parens, &mut sink);
            count += 1;

            if !p.current().is_terminator() && p.expect(SyntaxKind::Comma) {
                maybe_just_parens = false;
            }
        }

        p.expect_closing_delimiter(m, SyntaxKind::RightParen);
    });

    if maybe_just_parens && count == 1 && !sink {
        p.wrap(m, SyntaxKind::Parenthesized);
    } else {
        p.wrap(m, SyntaxKind::Destructuring);
    }
}

/// Parses an item in a destructuring pattern.
pub(super) fn destructuring_item<'s>(
    p: &mut Parser<'s>,
    reassignment: bool,
    seen: &mut FxHashSet<&'s str>,
    maybe_just_parens: &mut bool,
    sink: &mut bool,
) {
    let m = p.marker();

    // Parse destructuring sink: `..rest`.
    if p.eat_if(SyntaxKind::Dots) {
        if p.at_set(set::PATTERN_LEAF) {
            pattern_leaf(p, reassignment, seen, None);
        }
        p.wrap(m, SyntaxKind::Spread);
        if mem::replace(sink, true) {
            p[m].convert_to_error("only one destructuring sink is allowed");
        }
        return;
    }

    // Parse a normal positional pattern or a destructuring key.
    let was_at_pat = p.at_set(set::PATTERN);

    // We must use a full checkpoint here (can't just clone the lexer) because
    // there may be trivia between the identifier and the colon we need to skip.
    let checkpoint = p.checkpoint();
    if !(p.eat_if(SyntaxKind::Ident) && p.at(SyntaxKind::Colon)) {
        p.restore(checkpoint);
        pattern(p, reassignment, seen, None);
    }

    // Parse named destructuring item.
    if p.eat_if(SyntaxKind::Colon) {
        // Recover from bad named destructuring.
        if was_at_pat && p[m].kind() != SyntaxKind::Ident {
            p[m].expected("identifier");
        }

        pattern(p, reassignment, seen, None);
        p.wrap(m, SyntaxKind::Named);
        *maybe_just_parens = false;
    }
}

/// Parses a leaf in a pattern - either an identifier or an expression
/// depending on whether it's a binding or reassignment pattern.
pub(super) fn pattern_leaf<'s>(
    p: &mut Parser<'s>,
    reassignment: bool,
    seen: &mut FxHashSet<&'s str>,
    dupe: Option<&'s str>,
) {
    if p.current().is_keyword() {
        p.eat_and_get().expected("pattern");
        return;
    } else if !p.at_set(set::PATTERN_LEAF) {
        p.expected("pattern");
        return;
    }

    let m = p.marker();
    let text = p.current_text();

    // We parse an atomic expression even though we only want an identifier for
    // better error recovery. We can mark the whole expression as unexpected
    // instead of going through its pieces one by one.
    code_expr_prec(p, true, 0);

    if !reassignment {
        let node = &mut p[m];
        if node.kind() == SyntaxKind::Ident {
            if !seen.insert(text) {
                node.convert_to_error(format!(
                    "duplicate {}: {text}",
                    dupe.unwrap_or("binding"),
                ));
            }
        } else {
            node.expected("pattern");
        }
    }
}

// Parser + tipos de apoio extraídos para parse/parser.rs (Passo 96.4, ADR-0037).
