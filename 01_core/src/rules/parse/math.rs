//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-04-23
//!
//! Parse de expressões matemáticas: `math_expr`, `math_expr_prec`,
//! `math_attach`, `math_frac`, `math_delimited`, `math_args`.
//! Extraído de `parse.rs` no Passo 96.4 conforme ADR-0037.

use rustc_hash::FxHashSet;

use crate::entities::math_class::{default_math_class, MathClass};
use crate::entities::operators::Assoc;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_set::SyntaxSet;
use crate::entities::syntax_set as set;
use crate::syntax_set;

use super::parser::{Marker, Parser};
use super::code::embedded_code_expr;

/// Parses the contents of a mathematical equation: `x^2 + 1`.
pub(super) fn math(p: &mut Parser, stop_set: SyntaxSet) {
    let m = p.marker();
    math_exprs(p, stop_set);
    p.wrap(m, SyntaxKind::Math);
}

/// Parses a sequence of math expressions. Returns the number of expressions
/// parsed (including errors).
pub(super) fn math_exprs(p: &mut Parser, stop_set: SyntaxSet) -> usize {
    debug_assert!(stop_set.contains(SyntaxKind::End));
    let Some(p) = p.check_depth_until(stop_set) else { return 1 };

    let mut count = 0;
    while !p.at_set(stop_set) {
        if p.at_set(set::MATH_EXPR) {
            math_expr(p);
        } else {
            p.unexpected();
        }
        count += 1;
    }
    count
}

/// Parses a single math expression: This includes math elements like
/// attachment, fractions, roots, and embedded code expressions.
fn math_expr(p: &mut Parser) {
    math_expr_prec(p, 0, syntax_set!())
}

/// Parses a math expression with at least the given precedence, possibly
/// chaining with another operator by returning early.
fn math_expr_prec(p: &mut Parser, min_prec: u8, stop_set: SyntaxSet) {
    let Some(p) = &mut p.increase_depth() else { return };

    let m = p.marker();
    let mut continuable = false;
    match p.current() {
        SyntaxKind::Hash => embedded_code_expr(p),
        // The lexer manages creating full FieldAccess nodes if needed.
        SyntaxKind::MathIdent | SyntaxKind::FieldAccess => {
            continuable = true;
            p.eat();
            // Parse a function call for an identifier or field access.
            if MATH_FUNC_PREC >= min_prec && p.directly_at(SyntaxKind::LeftParen) {
                math_args(p);
                p.wrap(m, SyntaxKind::FuncCall);
                continuable = false;
            }
        }

        SyntaxKind::LeftBrace | SyntaxKind::LeftParen => {
            math_delimited(p);
        }
        SyntaxKind::RightBrace if p.current_text() == "|]" => {
            p.convert_and_eat(SyntaxKind::MathShorthand);
        }
        SyntaxKind::Dot
        | SyntaxKind::Bang
        | SyntaxKind::Comma
        | SyntaxKind::Semicolon
        | SyntaxKind::RightBrace
        | SyntaxKind::RightParen => {
            p.convert_and_eat(SyntaxKind::MathText);
        }

        SyntaxKind::MathText => {
            continuable = is_math_alphabetic(p.current_text());
            p.eat();
        }

        SyntaxKind::Linebreak
        | SyntaxKind::MathAlignPoint
        | SyntaxKind::MathShorthand => p.eat(),

        SyntaxKind::MathPrimes | SyntaxKind::Escape | SyntaxKind::Str => {
            continuable = true;
            p.eat();
        }

        SyntaxKind::Root => {
            p.eat();
            let m2 = p.marker();
            math_expr_prec(p, MATH_ROOT_PREC, syntax_set!());
            math_unparen(p, m2);
            p.wrap(m, SyntaxKind::MathRoot);
        }

        _ => p.expected("expression"),
    }

    // Maybe recognize an implicit function call: a 'continuable' token followed
    // by delimiters will group as one with the precedence of a normal function.
    // E.g. `a(b)/c` parses as `(a(b))/c` when `a` is continuable.
    if continuable
        && MATH_FUNC_PREC >= min_prec
        && !p.had_trivia()
        && p.at_set(syntax_set!(LeftBrace, LeftParen))
    {
        math_delimited(p);
        p.wrap(m, SyntaxKind::Math);
    }

    // Parse infix and postfix operators. The general form of a parsed op looks
    // like: `MathAttach[ MathText("x"), Hat("^"), MathText("2") ]`.
    loop {
        if p.at_set(stop_set) { break; }
        let op_kind = p.current();
        let had_trivia = p.had_trivia();
        let Some((wrapper, infix_assoc, prec)) = math_op(op_kind, had_trivia) else { break };
        if prec < min_prec { break; }
        // Prepare a chaining set for the attachment operators.
        let mut chain_set = if wrapper == SyntaxKind::MathAttach {
            // Hat can chain with Underscore, Underscore can chain with Hat, and
            // Prime can chain with either (but prime can't interrupt a chain,
            // see below).
            syntax_set!(Hat, Underscore).remove(op_kind)
        } else {
            syntax_set!()
        };

        // Eat the operator itself.
        if op_kind == SyntaxKind::Bang {
            p.convert_and_eat(SyntaxKind::MathText);
        } else {
            p.eat();
        }

        // Slash is the only operator that removes parens from its left operand.
        if wrapper == SyntaxKind::MathFrac {
            math_unparen(p, m);
        }

        // Parse the operator's right operand.
        if let Some(assoc) = infix_assoc {
            let prec = match assoc {
                Assoc::Left => prec + 1,
                Assoc::Right => prec,
            };
            let m_rhs = p.marker();
            math_expr_prec(p, prec, chain_set);
            math_unparen(p, m_rhs);
        }

        // Avoid interrupting a chain when initially parsing a prime.
        // For `a^b'_c^d` the grouping is `(a^(b')_c)^d` and not `a^(b'_c^d)`.
        if !(op_kind == SyntaxKind::MathPrimes && p.at_set(stop_set)) {
            // Parse chained attachment operators as a single attachment.
            while p.at_set(chain_set) {
                chain_set = chain_set.remove(p.current());
                p.eat();
                let m_chain_rhs = p.marker();
                math_expr_prec(p, prec, chain_set);
                math_unparen(p, m_chain_rhs);
            }
        }

        // Finish the operator by wrapping from its left operand.
        p.wrap(m, wrapper);
    }
}

// These are declared here so they're easier to compare with `math_op`.
const MATH_FUNC_PREC: u8 = 2;
const MATH_ROOT_PREC: u8 = 2;

/// Precedence and wrapper kinds for infix and postfix math operators.
fn math_op(
    kind: SyntaxKind,
    had_trivia: bool,
) -> Option<(SyntaxKind, Option<Assoc>, u8)> {
    let op = match kind {
        SyntaxKind::Slash => (SyntaxKind::MathFrac, Some(Assoc::Left), 1),
        SyntaxKind::Underscore => (SyntaxKind::MathAttach, Some(Assoc::Right), 2),
        SyntaxKind::Hat => (SyntaxKind::MathAttach, Some(Assoc::Right), 2),
        SyntaxKind::MathPrimes if !had_trivia => (SyntaxKind::MathAttach, None, 2),
        SyntaxKind::Bang if !had_trivia => (SyntaxKind::Math, None, 3),
        _ => return None,
    };
    Some(op)
}

/// Whether text counts as alphabetic in math. For the `Text` and `MathText`
/// kinds, this causes them to group with parens as an implicit function call.
fn is_math_alphabetic(text: &str) -> bool {
    if let Some((0, c)) = text.char_indices().next_back() {
        // Just a single character.
        c.is_alphabetic() || default_math_class(c) == Some(MathClass::Alphabetic)
    } else {
        // Multiple characters.
        text.chars().all(char::is_alphabetic)
    }
}

/// Parse matched delimiters in math: `[x + y]`.
///
/// The lexer produces `{Left,Right}{Brace,Paren}` for delimiters, and it's our
/// job to convert them back to `MathText` or `MathShorthand` before eating.
fn math_delimited(p: &mut Parser) {
    let m = p.marker();
    if p.current_text() == "[|" {
        p.convert_and_eat(SyntaxKind::MathShorthand);
    } else {
        p.convert_and_eat(SyntaxKind::MathText);
    }
    let m_body = p.marker();
    math_exprs(p, syntax_set!(Dollar, End, RightBrace, RightParen));
    if p.at_set(syntax_set!(RightBrace, RightParen)) {
        p.wrap(m_body, SyntaxKind::Math);
        if p.current_text() == "|]" {
            p.convert_and_eat(SyntaxKind::MathShorthand);
        } else {
            p.convert_and_eat(SyntaxKind::MathText);
        }
        p.wrap(m, SyntaxKind::MathDelimited);
    } else {
        // If we had no closing delimiter, just produce a math sequence.
        p.wrap(m, SyntaxKind::Math);
    }
}

/// Remove one set of parentheses (if any) from a previously parsed expression
/// by converting to non-expression SyntaxKinds.
fn math_unparen(p: &mut Parser, m: Marker) {
    let Some(node) = p.nodes.get_mut(m.0) else { return };
    if node.kind() != SyntaxKind::MathDelimited {
        return;
    }

    if let [first, .., last] = node.children_mut() {
        if first.text() == "(" && last.text() == ")" {
            first.convert_to_kind(SyntaxKind::LeftParen);
            last.convert_to_kind(SyntaxKind::RightParen);
            // Only convert if we did have regular parens.
            node.convert_to_kind(SyntaxKind::Math);
        }
    }
}

/// Parse an argument list in math: `(a, b; c, d; size: #50%)`.
fn math_args(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::LeftParen);

    let mut positional = true;
    let mut has_arrays = false;

    let mut maybe_array_start = p.marker();
    let mut seen = FxHashSet::default();
    while !p.at_set(syntax_set!(End, Dollar, RightParen)) {
        positional = math_arg(p, &mut seen);

        match p.current() {
            SyntaxKind::Comma => {
                p.eat();
                if !positional {
                    maybe_array_start = p.marker();
                }
            }
            SyntaxKind::Semicolon => {
                if !positional {
                    maybe_array_start = p.marker();
                }

                // Parses an array: `a, b, c;`.
                // The semicolon merges preceding arguments separated by commas
                // into an array argument.
                p.wrap(maybe_array_start, SyntaxKind::Array);
                p.eat();
                maybe_array_start = p.marker();
                has_arrays = true;
            }
            SyntaxKind::End | SyntaxKind::Dollar | SyntaxKind::RightParen => {}
            _ => p.expected("comma or semicolon"),
        }
    }

    // Check if we need to wrap the preceding arguments in an array.
    if maybe_array_start != p.marker() && has_arrays && positional {
        p.wrap(maybe_array_start, SyntaxKind::Array);
    }

    p.expect_closing_delimiter(m, SyntaxKind::RightParen);
    p.wrap(m, SyntaxKind::Args);
}

/// Parses a single argument in a math argument list.
///
/// Returns whether the parsed argument was positional or not.
fn math_arg<'s>(p: &mut Parser<'s>, seen: &mut FxHashSet<&'s str>) -> bool {
    let m = p.marker();
    let start = p.current_start();

    let mut arg_kind = None;

    if p.at(SyntaxKind::Dot) {
        if let Some(spread) = p.lexer.maybe_math_spread_arg(start) {
            // Parses a spread argument: `..args`.
            arg_kind = Some(SyntaxKind::Spread);
            p.token.node = spread;
            p.eat();
        }
    } else if p.at_set(syntax_set!(MathText, MathIdent, Underscore)) {
        if let Some(named) = p.lexer.maybe_math_named_arg(start) {
        // Parses a named argument: `thickness: #12pt`.
        arg_kind = Some(SyntaxKind::Named);
        p.token.node = named;
        let text = p.current_text();
        p.eat();
        p.convert_and_eat(SyntaxKind::Colon);
        if !seen.insert(text) {
            p[m].convert_to_error(format!("duplicate argument: {text}"));
        }
        }
    }

    // Parses the argument itself.
    let m_arg = p.marker();
    let count = math_exprs(p, syntax_set!(End, Dollar, Comma, Semicolon, RightParen));

    if count == 0 {
        // This can't happen due to checks in `Lexer::maybe_math_spread_arg`.
        assert_ne!(arg_kind, Some(SyntaxKind::Spread));

        // Named arguments require a value.
        if arg_kind == Some(SyntaxKind::Named) {
            p.expected("expression");
        }

        // Flush trivia so that the new empty Math node will be wrapped _inside_
        // any `SyntaxKind::Array` elements created in `math_args`.
        // (And if we don't follow by wrapping in an array, it has no effect.)
        // The difference in node layout without this would look like:
        // - Expression: `$ mat( ;) $`
        // - Correct:    [ .., Space(" "), Array[Math[], ], Semicolon(";"), .. ]
        // - Incorrect:  [ .., Math[], Array[], Space(" "), Semicolon(";"), .. ]
        p.flush_trivia();
    }

    // Wrap math function arguments to join adjacent math content or create an
    // empty 'Math' node for when we have 0 args. We don't wrap when
    // `count == 1`, since wrapping would change the type of the expression
    // from potentially non-content to content. E.g. `$ func(#12pt) $` would
    // change the type of `#12pt` from size to content if wrapped.
    if count != 1 {
        p.wrap(m_arg, SyntaxKind::Math);
    }

    if let Some(kind) = arg_kind {
        p.wrap(m, kind);
    }
    arg_kind != Some(SyntaxKind::Named)
}

