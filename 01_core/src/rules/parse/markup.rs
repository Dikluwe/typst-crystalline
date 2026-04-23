//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-04-23
//!
//! Parse de markup: markup, markup_exprs, markup_expr, strong, emph,
//! heading, list/enum/term_item, reference, equation, reparse_markup.
//! Extraído de `parse.rs` no Passo 96.4 conforme ADR-0037.

use std::ops::Range;

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::syntax_set::SyntaxSet;
use crate::syntax_set;

use super::parser::{AtNewline, Parser};
use super::code::{content_block, embedded_code_expr};
use super::math::math;

/// Parses markup expressions until a stop condition is met.
pub(super) fn markup(p: &mut Parser, at_start: bool, wrap_trivia: bool, stop_set: SyntaxSet) {
    let m = if wrap_trivia { p.before_trivia() } else { p.marker() };
    markup_exprs(p, at_start, stop_set);
    if wrap_trivia {
        p.flush_trivia();
    }
    p.wrap(m, SyntaxKind::Markup);
}

/// Parses a sequence of markup expressions.
pub(super) fn markup_exprs(p: &mut Parser, mut at_start: bool, stop_set: SyntaxSet) {
    debug_assert!(stop_set.contains(SyntaxKind::End));
    let Some(p) = p.check_depth_until(stop_set) else { return };

    at_start |= p.had_newline();
    let mut nesting: usize = 0;
    // Keep going if we're at a nested right-bracket regardless of the stop set.
    while !p.at_set(stop_set) || (nesting > 0 && p.at(SyntaxKind::RightBracket)) {
        markup_expr(p, at_start, &mut nesting);
        at_start = p.had_newline();
    }
}

/// Reparses a subsection of markup incrementally.
#[allow(dead_code)] // API de reparse incremental — migração futura
pub(super) fn reparse_markup(
    text: &str,
    range: Range<usize>,
    at_start: &mut bool,
    nesting: &mut usize,
    top_level: bool,
) -> Option<Vec<SyntaxNode>> {
    let mut p = Parser::new(text, range.start, SyntaxMode::Markup);
    *at_start |= p.had_newline();
    while !p.end() && p.current_start() < range.end {
        // If not top-level and at a new RightBracket, stop the reparse.
        if !top_level && *nesting == 0 && p.at(SyntaxKind::RightBracket) {
            break;
        }
        markup_expr(&mut p, *at_start, nesting);
        *at_start = p.had_newline();
    }
    (p.balanced && p.current_start() == range.end).then(|| p.finish())
}

/// Parses a single markup expression. This includes markup elements like text,
/// headings, strong/emph, lists/enums, etc. This is also the entry point for
/// parsing math equations and embedded code expressions.
pub(super) fn markup_expr(p: &mut Parser, at_start: bool, nesting: &mut usize) {
    let Some(p) = &mut p.increase_depth() else { return };

    match p.current() {
        SyntaxKind::LeftBracket => {
            *nesting += 1;
            p.convert_and_eat(SyntaxKind::Text);
        }
        SyntaxKind::RightBracket if *nesting > 0 => {
            *nesting -= 1;
            p.convert_and_eat(SyntaxKind::Text);
        }
        SyntaxKind::RightBracket => {
            p.unexpected();
            p.hint("try using a backslash escape: \\]");
        }

        SyntaxKind::Shebang => p.eat(),

        SyntaxKind::Text
        | SyntaxKind::Linebreak
        | SyntaxKind::Escape
        | SyntaxKind::Shorthand
        | SyntaxKind::SmartQuote
        | SyntaxKind::Link
        | SyntaxKind::Label => p.eat(),

        SyntaxKind::Raw => p.eat(), // Raw is handled entirely in the Lexer.

        SyntaxKind::Hash => embedded_code_expr(p),
        SyntaxKind::Star => strong(p),
        SyntaxKind::Underscore => emph(p),
        SyntaxKind::HeadingMarker if at_start => heading(p),
        SyntaxKind::ListMarker if at_start => list_item(p),
        SyntaxKind::EnumMarker if at_start => enum_item(p),
        SyntaxKind::TermMarker if at_start => term_item(p),
        SyntaxKind::RefMarker => reference(p),
        SyntaxKind::Dollar => equation(p),

        SyntaxKind::HeadingMarker
        | SyntaxKind::ListMarker
        | SyntaxKind::EnumMarker
        | SyntaxKind::TermMarker
        | SyntaxKind::Colon => p.convert_and_eat(SyntaxKind::Text),

        _ => p.unexpected(),
    }
}

/// Parses strong content: `*Strong*`.
pub(super) fn strong(p: &mut Parser) {
    p.with_nl_mode(AtNewline::StopParBreak, |p| {
        let m = p.marker();
        p.assert(SyntaxKind::Star);
        markup(p, false, true, syntax_set!(Star, RightBracket, End));
        p.expect_closing_delimiter(m, SyntaxKind::Star);
        p.wrap(m, SyntaxKind::Strong);
    });
}

/// Parses emphasized content: `_Emphasized_`.
pub(super) fn emph(p: &mut Parser) {
    p.with_nl_mode(AtNewline::StopParBreak, |p| {
        let m = p.marker();
        p.assert(SyntaxKind::Underscore);
        markup(p, false, true, syntax_set!(Underscore, RightBracket, End));
        p.expect_closing_delimiter(m, SyntaxKind::Underscore);
        p.wrap(m, SyntaxKind::Emph);
    });
}

/// Parses a section heading: `= Introduction`.
pub(super) fn heading(p: &mut Parser) {
    p.with_nl_mode(AtNewline::Stop, |p| {
        let m = p.marker();
        p.assert(SyntaxKind::HeadingMarker);
        markup(p, false, false, syntax_set!(Label, RightBracket, End));
        p.wrap(m, SyntaxKind::Heading);
    });
}

/// Parses an item in a bullet list: `- ...`.
pub(super) fn list_item(p: &mut Parser) {
    p.with_nl_mode(AtNewline::RequireColumn(p.current_column()), |p| {
        let m = p.marker();
        p.assert(SyntaxKind::ListMarker);
        markup(p, true, false, syntax_set!(RightBracket, End));
        p.wrap(m, SyntaxKind::ListItem);
    });
}

/// Parses an item in an enumeration (numbered list): `+ ...` or `1. ...`.
pub(super) fn enum_item(p: &mut Parser) {
    p.with_nl_mode(AtNewline::RequireColumn(p.current_column()), |p| {
        let m = p.marker();
        p.assert(SyntaxKind::EnumMarker);
        markup(p, true, false, syntax_set!(RightBracket, End));
        p.wrap(m, SyntaxKind::EnumItem);
    });
}

/// Parses an item in a term list: `/ Term: Details`.
pub(super) fn term_item(p: &mut Parser) {
    p.with_nl_mode(AtNewline::RequireColumn(p.current_column()), |p| {
        let m = p.marker();
        p.with_nl_mode(AtNewline::Stop, |p| {
            p.assert(SyntaxKind::TermMarker);
            markup(p, false, false, syntax_set!(Colon, RightBracket, End));
        });
        p.expect(SyntaxKind::Colon);
        markup(p, true, false, syntax_set!(RightBracket, End));
        p.wrap(m, SyntaxKind::TermItem);
    });
}

/// Parses a reference: `@target`, `@target[..]`.
pub(super) fn reference(p: &mut Parser) {
    let m = p.marker();
    p.assert(SyntaxKind::RefMarker);
    if p.directly_at(SyntaxKind::LeftBracket) {
        content_block(p);
    }
    p.wrap(m, SyntaxKind::Ref);
}

/// Parses a mathematical equation: `$x$`, `$ x^2 $`.
pub(super) fn equation(p: &mut Parser) {
    let m = p.marker();
    p.enter_modes(SyntaxMode::Math, AtNewline::Continue, |p| {
        p.assert(SyntaxKind::Dollar);
        math(p, syntax_set!(Dollar, End));
        p.expect_closing_delimiter(m, SyntaxKind::Dollar);
    });
    p.wrap(m, SyntaxKind::Equation);
}

