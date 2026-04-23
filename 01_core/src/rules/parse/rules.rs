//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-04-23
//!
//! Parse de statements de controlo: let, set, show, context,
//! if/while/for, import/include, break/continue/return.
//! Extraído de `parse.rs` no Passo 96.4 conforme ADR-0037.

use rustc_hash::FxHashSet;

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_set as set;

use super::parser::{AtNewline, Parser};
use super::code::{block, code_expr, code_expr_prec};
use super::patterns::{args, params, pattern};

/// Parses a let binding: `let x = 1`.
pub(super) fn let_binding(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Let);

    let m2 = p.marker();
    let mut closure = false;
    let mut other = false;

    if p.eat_if(SyntaxKind::Ident) {
        if p.directly_at(SyntaxKind::LeftParen) {
            params(p);
            closure = true;
        }
    } else {
        pattern(p, false, &mut FxHashSet::default(), None);
        other = true;
    }

    let f = if closure || other { Parser::expect } else { Parser::eat_if };
    if f(p, SyntaxKind::Eq) {
        code_expr(p);
    }

    if closure {
        p.wrap(m2, SyntaxKind::Closure);
    }

    p.wrap(m, SyntaxKind::LetBinding);
}

/// Parses a set rule: `set text(...)`.
pub(super) fn set_rule(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Set);

    let m2 = p.marker();
    p.expect(SyntaxKind::Ident);
    while p.eat_if(SyntaxKind::Dot) {
        p.expect(SyntaxKind::Ident);
        p.wrap(m2, SyntaxKind::FieldAccess);
    }

    args(p);
    if p.eat_if(SyntaxKind::If) {
        code_expr(p);
    }
    p.wrap(m, SyntaxKind::SetRule);
}

/// Parses a show rule: `show heading: it => emph(it.body)`.
pub(super) fn show_rule(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Show);
    let m2 = p.before_trivia();

    if !p.at(SyntaxKind::Colon) {
        code_expr(p);
    }

    if p.eat_if(SyntaxKind::Colon) {
        code_expr(p);
    } else {
        p.expected_at(m2, "colon");
    }

    p.wrap(m, SyntaxKind::ShowRule);
}

/// Parses a contextual expression: `context text.lang`.
pub(super) fn contextual(p: &mut Parser, atomic: bool) {
    let m = p.marker();
    p.assert(SyntaxKind::Context);
    code_expr_prec(p, atomic, 0);
    p.wrap(m, SyntaxKind::Contextual);
}

/// Parses an if-else conditional: `if x { y } else { z }`.
pub(super) fn conditional(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::If);
    code_expr(p);
    block(p);
    if p.eat_if(SyntaxKind::Else) {
        if p.at(SyntaxKind::If) {
            conditional(p);
        } else {
            block(p);
        }
    }
    p.wrap(m, SyntaxKind::Conditional);
}

/// Parses a while loop: `while x { y }`.
pub(super) fn while_loop(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::While);
    code_expr(p);
    block(p);
    p.wrap(m, SyntaxKind::WhileLoop);
}

/// Parses a for loop: `for x in y { z }`.
pub(super) fn for_loop(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::For);

    let mut seen = FxHashSet::default();
    pattern(p, false, &mut seen, None);

    if p.at(SyntaxKind::Comma) {
        let node = p.eat_and_get();
        node.unexpected();
        node.hint("destructuring patterns must be wrapped in parentheses");
        if p.at_set(set::PATTERN) {
            pattern(p, false, &mut seen, None);
        }
    }

    p.expect(SyntaxKind::In);
    code_expr(p);
    block(p);
    p.wrap(m, SyntaxKind::ForLoop);
}

/// Parses a module import: `import "utils.typ": a, b, c`.
pub(super) fn module_import(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Import);
    code_expr(p);
    if p.eat_if(SyntaxKind::As) {
        // Allow renaming a full module import.
        // If items are included, both the full module and the items are
        // imported at the same time.
        p.expect(SyntaxKind::Ident);
    }

    if p.eat_if(SyntaxKind::Colon) {
        if p.at(SyntaxKind::LeftParen) {
            p.with_nl_mode(AtNewline::Continue, |p| {
                let m2 = p.marker();
                p.assert(SyntaxKind::LeftParen);

                import_items(p);

                p.expect_closing_delimiter(m2, SyntaxKind::RightParen);
            });
        } else if !p.eat_if(SyntaxKind::Star) {
            import_items(p);
        }
    }

    p.wrap(m, SyntaxKind::ModuleImport);
}

/// Parses items to import from a module: `a, b, c`.
pub(super) fn import_items(p: &mut Parser) {
    let m = p.marker();
    while !p.current().is_terminator() {
        let item_marker = p.marker();
        if !p.eat_if(SyntaxKind::Ident) {
            p.unexpected();
        }

        // Nested import path: `a.b.c`
        while p.eat_if(SyntaxKind::Dot) {
            p.expect(SyntaxKind::Ident);
        }

        p.wrap(item_marker, SyntaxKind::ImportItemPath);

        // Rename imported item.
        if p.eat_if(SyntaxKind::As) {
            p.expect(SyntaxKind::Ident);
            p.wrap(item_marker, SyntaxKind::RenamedImportItem);
        }

        if !p.current().is_terminator() {
            p.expect(SyntaxKind::Comma);
        }
    }

    p.wrap(m, SyntaxKind::ImportItems);
}

/// Parses a module include: `include "chapter1.typ"`.
pub(super) fn module_include(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Include);
    code_expr(p);
    p.wrap(m, SyntaxKind::ModuleInclude);
}

/// Parses a break from a loop: `break`.
pub(super) fn break_stmt(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Break);
    p.wrap(m, SyntaxKind::LoopBreak);
}

/// Parses a continue in a loop: `continue`.
pub(super) fn continue_stmt(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Continue);
    p.wrap(m, SyntaxKind::LoopContinue);
}

/// Parses a return from a function: `return`, `return x + 1`.
pub(super) fn return_stmt(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::Return);
    if p.at_set(set::CODE_EXPR) {
        code_expr(p);
    }
    p.wrap(m, SyntaxKind::FuncReturn);
}
