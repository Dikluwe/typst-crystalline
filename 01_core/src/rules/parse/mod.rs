//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/parse.md
//! @prompt-hash 8191e20b
//! @layer L1
//! @updated 2026-03-23

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::SyntaxNode;
use crate::syntax_set;

// Submódulos por domínio (Passo 96.4, ADR-0037).
mod parser;
mod math;
mod markup;
mod code;
mod rules;
mod patterns;
use crate::rules::parse::parser::Parser;
use crate::rules::parse::math::math_exprs;
use crate::rules::parse::markup::markup_exprs;
use crate::rules::parse::code::code_exprs;

/// Parses a source file as top-level markup.
pub fn parse(text: &str) -> SyntaxNode {
    // ADR-0006: timing removed — ver 00_nucleo/DEBT.md
    let mut p = Parser::new(text, 0, SyntaxMode::Markup);
    markup_exprs(&mut p, true, syntax_set!(End));
    p.finish_into(SyntaxKind::Markup)
}

/// Parses top-level code.
pub fn parse_code(text: &str) -> SyntaxNode {
    // ADR-0006: timing removed — ver 00_nucleo/DEBT.md
    let mut p = Parser::new(text, 0, SyntaxMode::Code);
    code_exprs(&mut p, syntax_set!(End));
    p.finish_into(SyntaxKind::Code)
}

/// Parses top-level math.
pub fn parse_math(text: &str) -> SyntaxNode {
    // ADR-0006: timing removed — ver 00_nucleo/DEBT.md
    let mut p = Parser::new(text, 0, SyntaxMode::Math);
    math_exprs(&mut p, syntax_set!(End));
    p.finish_into(SyntaxKind::Math)
}

// Markup parsing extraído para parse/markup.rs (Passo 96.4, ADR-0037).

// Math parsing extraído para parse/math.rs (Passo 96.4, ADR-0037).

// Code parsing extraído para parse/code.rs (Passo 96.4, ADR-0037).


// Statements de controlo extraídos para parse/rules.rs (Passo 96.4, ADR-0037).


// Expressões com parêntesis, args, params e patterns extraídos para parse/patterns.rs (Passo 96.4, ADR-0037).


#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::syntax_kind::SyntaxKind;

    #[test]
    fn texto_simples() {
        let node = parse("Hello, world!");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(!node.erroneous());
    }

    #[test]
    fn texto_vazio() {
        let node = parse("");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(!node.erroneous());
        assert_eq!(node.len(), 0);
    }

    #[test]
    fn parse_nunca_falha() {
        let node = parse("#{{{broken");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        assert!(node.erroneous());
        assert!(!node.errors().is_empty());
    }

    #[test]
    fn expressao_matematica() {
        let node = parse("$x^2 + 1$");
        assert_eq!(node.kind(), SyntaxKind::Markup);
        let eq = node.children().find(|n| n.kind() == SyntaxKind::Equation);
        assert!(eq.is_some());
    }

    #[test]
    fn codigo_typst() {
        let node = parse("#let x = 1");
        let binding = node.children()
            .find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some());
    }

    #[test]
    fn parse_math_basico() {
        let node = parse_math("x^2");
        assert_eq!(node.kind(), SyntaxKind::Math);
        assert!(!node.erroneous());
    }

    #[test]
    fn parse_code_basico() {
        let node = parse_code("let x = 1");
        assert_eq!(node.kind(), SyntaxKind::Code);
        assert!(!node.erroneous());
    }

    // ── Testes de Passo 32 — sintaxe #let f(params) = ... ────────────────

    #[test]
    fn parse_let_funcao_com_parametros() {
        // #let f(x, y) = x + y deve gerar LetBinding sem erros de parse
        let node = parse("#let f(x, y) = x + y");
        assert!(
            node.errors().is_empty(),
            "parse de #let f(x,y) gerou erros: {:?}", node.errors()
        );
        let binding = node.children()
            .find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some(), "deve gerar LetBinding");
    }

    #[test]
    fn parse_let_funcao_sem_parametros() {
        // #let f() = 42 — closure sem parâmetros
        let node = parse("#let f() = 42");
        assert!(
            node.errors().is_empty(),
            "parse de #let f() = 42 gerou erros: {:?}", node.errors()
        );
        let binding = node.children()
            .find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some(), "deve gerar LetBinding");
    }

    #[test]
    fn parse_let_funcao_recursiva() {
        // #let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
        let node = parse("#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }");
        assert!(
            node.errors().is_empty(),
            "parse de #let fib(n) gerou erros: {:?}", node.errors()
        );
    }
}
