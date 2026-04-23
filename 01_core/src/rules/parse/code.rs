//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-04-23
//!
//! Parse de code blocks e expressões de código: code, code_exprs,
//! embedded_code_expr, code_expr, code_expr_prec, code_primary,
//! block, code_block, content_block, reparse_block.
//! Extraído de `parse.rs` no Passo 96.4 conforme ADR-0037.

use std::ops::Range;

use crate::entities::operators::{Assoc, BinOp, UnOp};
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::syntax_set::SyntaxSet;
use crate::entities::syntax_set as set;
use crate::syntax_set;

use super::parser::{AtNewline, Parser};
use super::markup::{equation, markup};
use super::patterns::{args, expr_with_paren};
use super::rules::{
    break_stmt, conditional, continue_stmt, contextual, for_loop, let_binding,
    module_import, module_include, return_stmt, set_rule, show_rule, while_loop,
};

/// Parses the contents of a code block.
pub(super) fn code(p: &mut Parser, stop_set: SyntaxSet) {
    let m = p.marker();
    code_exprs(p, stop_set);
    p.wrap(m, SyntaxKind::Code);
}

/// Parses a sequence of code expressions.
pub(super) fn code_exprs(p: &mut Parser, stop_set: SyntaxSet) {
    debug_assert!(stop_set.contains(SyntaxKind::End));
    let Some(p) = p.check_depth_until(stop_set) else { return };

    while !p.at_set(stop_set) {
        p.with_nl_mode(AtNewline::ContextualContinue, |p| {
            if !p.at_set(set::CODE_EXPR) {
                p.unexpected();
                return;
            }
            code_expr(p);
            if !p.at_set(stop_set) && !p.eat_if(SyntaxKind::Semicolon) {
                p.expected("semicolon or line break");
                if p.at(SyntaxKind::Label) {
                    p.hint("labels can only be applied in markup mode");
                    p.hint("try wrapping your code in a markup block (`[ ]`)");
                }
            }
        });
    }
}

/// Parses an atomic code expression embedded in markup or math.
pub(super) fn embedded_code_expr(p: &mut Parser) {
    p.enter_modes(SyntaxMode::Code, AtNewline::Stop, |p| {
        p.assert(SyntaxKind::Hash);
        if p.had_trivia() || p.end() {
            p.expected("expression");
            return;
        }

        let stmt = p.at_set(set::STMT);
        let at = p.at_set(set::ATOMIC_CODE_EXPR);
        code_expr_prec(p, true, 0);

        // Consume error for things like `#12p` or `#"abc\"`.#
        if !at {
            p.unexpected();
        }

        let semi = (stmt || p.directly_at(SyntaxKind::Semicolon))
            && p.eat_if(SyntaxKind::Semicolon);

        if stmt && !semi && !p.end() && !p.at(SyntaxKind::RightBracket) {
            p.expected("semicolon or line break");
        }
    });
}

/// Parses a single code expression.
pub(super) fn code_expr(p: &mut Parser) {
    code_expr_prec(p, false, 0)
}

/// Parses a code expression with at least the given precedence.
pub(super) fn code_expr_prec(p: &mut Parser, atomic: bool, min_prec: u8) {
    let Some(p) = &mut p.increase_depth() else { return };

    let m = p.marker();
    if !atomic && p.at_set(set::UNARY_OP) {
        let op = UnOp::from_kind(p.current()).unwrap();
        p.eat();
        code_expr_prec(p, atomic, op.precedence());
        p.wrap(m, SyntaxKind::Unary);
    } else {
        code_primary(p, atomic);
    }

    loop {
        if p.directly_at(SyntaxKind::LeftParen) || p.directly_at(SyntaxKind::LeftBracket)
        {
            args(p);
            p.wrap(m, SyntaxKind::FuncCall);
            continue;
        }

        let at_field_or_method = p.directly_at(SyntaxKind::Dot)
            && p.lexer.clone().next().0 == SyntaxKind::Ident;

        if atomic && !at_field_or_method {
            break;
        }

        if p.eat_if(SyntaxKind::Dot) {
            p.expect(SyntaxKind::Ident);
            p.wrap(m, SyntaxKind::FieldAccess);
            continue;
        }

        let binop = if p.at_set(set::BINARY_OP) {
            BinOp::from_kind(p.current())
        } else if min_prec <= BinOp::NotIn.precedence() && p.eat_if(SyntaxKind::Not)
        {
            if p.at(SyntaxKind::In) {
                Some(BinOp::NotIn)
            } else {
                p.expected("keyword `in`");
                break;
            }
        } else {
            None
        };

        if let Some(op) = binop {
            let mut prec = op.precedence();
            if prec < min_prec {
                break;
            }

            match op.assoc() {
                Assoc::Left => prec += 1,
                Assoc::Right => {}
            }

            p.eat();
            code_expr_prec(p, false, prec);
            p.wrap(m, SyntaxKind::Binary);
            continue;
        }

        break;
    }
}

/// Parses an primary in a code expression. These are the atoms that unary and
/// binary operations, functions calls, and field accesses start with / are
/// composed of.
pub(super) fn code_primary(p: &mut Parser, atomic: bool) {
    let m = p.marker();
    match p.current() {
        SyntaxKind::Ident => {
            p.eat();
            if !atomic && p.at(SyntaxKind::Arrow) {
                p.wrap(m, SyntaxKind::Params);
                p.assert(SyntaxKind::Arrow);
                code_expr(p);
                p.wrap(m, SyntaxKind::Closure);
            }
        }
        SyntaxKind::Underscore if !atomic => {
            p.eat();
            if p.at(SyntaxKind::Arrow) {
                p.wrap(m, SyntaxKind::Params);
                p.eat();
                code_expr(p);
                p.wrap(m, SyntaxKind::Closure);
            } else if p.eat_if(SyntaxKind::Eq) {
                code_expr(p);
                p.wrap(m, SyntaxKind::DestructAssignment);
            } else {
                p[m].expected("expression");
            }
        }

        SyntaxKind::LeftBrace => code_block(p),
        SyntaxKind::LeftBracket => content_block(p),
        SyntaxKind::LeftParen => expr_with_paren(p, atomic),
        SyntaxKind::Dollar => equation(p),
        SyntaxKind::Let => let_binding(p),
        SyntaxKind::Set => set_rule(p),
        SyntaxKind::Show => show_rule(p),
        SyntaxKind::Context => contextual(p, atomic),
        SyntaxKind::If => conditional(p),
        SyntaxKind::While => while_loop(p),
        SyntaxKind::For => for_loop(p),
        SyntaxKind::Import => module_import(p),
        SyntaxKind::Include => module_include(p),
        SyntaxKind::Break => break_stmt(p),
        SyntaxKind::Continue => continue_stmt(p),
        SyntaxKind::Return => return_stmt(p),

        SyntaxKind::Raw => p.eat(), // Raw is handled entirely in the Lexer.

        SyntaxKind::None
        | SyntaxKind::Auto
        | SyntaxKind::Int
        | SyntaxKind::Float
        | SyntaxKind::Bool
        | SyntaxKind::Numeric
        | SyntaxKind::Str
        | SyntaxKind::Label => p.eat(),

        _ => p.expected("expression"),
    }
}

/// Reparses a full content or code block.
#[allow(dead_code)] // API de reparse incremental — migração futura
pub(super) fn reparse_block(text: &str, range: Range<usize>) -> Option<SyntaxNode> {
    let mut p = Parser::new(text, range.start, SyntaxMode::Code);
    assert!(p.at(SyntaxKind::LeftBracket) || p.at(SyntaxKind::LeftBrace));
    block(&mut p);
    (p.balanced && p.prev_end() == range.end)
        .then(|| p.finish().into_iter().next().unwrap())
}

/// Parses a content or code block.
pub(super) fn block(p: &mut Parser) {
    match p.current() {
        SyntaxKind::LeftBracket => content_block(p),
        SyntaxKind::LeftBrace => code_block(p),
        _ => p.expected("block"),
    }
}

/// Parses a code block: `{ let x = 1; x + 2 }`.
pub(super) fn code_block(p: &mut Parser) {
    let m = p.marker();
    p.enter_modes(SyntaxMode::Code, AtNewline::Continue, |p| {
        p.assert(SyntaxKind::LeftBrace);
        code(p, syntax_set!(RightBrace, RightBracket, RightParen, End));
        p.expect_closing_delimiter(m, SyntaxKind::RightBrace);
    });
    p.wrap(m, SyntaxKind::CodeBlock);
}

/// Parses a content block: `[*Hi* there!]`.
pub(super) fn content_block(p: &mut Parser) {
    let m = p.marker();
    p.enter_modes(SyntaxMode::Markup, AtNewline::Continue, |p| {
        p.assert(SyntaxKind::LeftBracket);
        markup(p, true, true, syntax_set!(RightBracket, End));
        p.expect_closing_delimiter(m, SyntaxKind::RightBracket);
    });
    p.wrap(m, SyntaxKind::ContentBlock);
}
