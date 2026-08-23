//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/parse.md
//! @prompt-hash eb0d60e1
//! @layer L1
//! @updated 2026-03-23

use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::SyntaxNode;
use crate::syntax_set;

// Submódulos por domínio (Passo 96.4, ADR-0037).
mod code;
mod markup;
mod math;
mod parser;
mod patterns;
mod rules;
use crate::compiler::parse::code::code_exprs;
use crate::compiler::parse::markup::markup_exprs;
use crate::compiler::parse::math::math_exprs;
use crate::compiler::parse::parser::Parser;

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

/// **P814** — parseia `text` no `mode` indicado e ancora todos os spans dos
/// nós (incl. nós de erro) ao `anchor`, via [`SyntaxNode::synthesize`].
///
/// Equivalente cristalino do `SpanMode::Uniform(span)` do vanilla
/// (`typst-eval/src/lib.rs::eval_string`): erros de sintaxe/semântica dentro
/// de strings avaliadas sinteticamente (`#eval`, e futuros consumidores —
/// P815, P819) apontam para o callsite no documento real em vez de
/// `<detached>`. Se `anchor` for detached, a árvore fica como o parser a
/// produziu (fallback para contextos sem callsite real).
///
/// O span âncora disponível no cristalino é o da **lista de argumentos** da
/// chamada (`Args::span`, P772s — span por chamada, não por argumento); o
/// vanilla ancora ao literal string. A nuance de coluna está registada no
/// relatório de P814.
pub fn parse_anchored(
    text: &str,
    mode: SyntaxMode,
    anchor: crate::entities::span::Span,
) -> SyntaxNode {
    let mut root = match mode {
        SyntaxMode::Code => parse_code(text),
        SyntaxMode::Markup => parse(text),
        SyntaxMode::Math => parse_math(text),
    };
    if !anchor.is_detached() {
        root.synthesize(anchor);
    }
    root
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
        let children = node.children().collect::<Vec<_>>();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].kind(), SyntaxKind::Text);
        assert_eq!(children[0].text().as_str(), "Hello, world!");
    }

    #[test]
    fn texto_com_espaco_interno_um_no_p1137() {
        let node = parse("a b");
        let children = node.children().collect::<Vec<_>>();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].kind(), SyntaxKind::Text);
        assert_eq!(children[0].text().as_str(), "a b");
    }

    #[test]
    fn espaco_antes_de_pontuacao_permanece_trivia_p1137() {
        let node = parse("a !");
        let children = node.children().collect::<Vec<_>>();
        assert_eq!(children.len(), 3);
        assert_eq!(children[0].kind(), SyntaxKind::Text);
        assert_eq!(children[1].kind(), SyntaxKind::Space);
        assert_eq!(children[2].kind(), SyntaxKind::Text);
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
        let binding = node.children().find(|n| n.kind() == SyntaxKind::LetBinding);
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
            "parse de #let f(x,y) gerou erros: {:?}",
            node.errors()
        );
        let binding = node.children().find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some(), "deve gerar LetBinding");
    }

    #[test]
    fn parse_let_funcao_sem_parametros() {
        // #let f() = 42 — closure sem parâmetros
        let node = parse("#let f() = 42");
        assert!(
            node.errors().is_empty(),
            "parse de #let f() = 42 gerou erros: {:?}",
            node.errors()
        );
        let binding = node.children().find(|n| n.kind() == SyntaxKind::LetBinding);
        assert!(binding.is_some(), "deve gerar LetBinding");
    }

    #[test]
    fn parse_let_funcao_recursiva() {
        // #let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
        let node =
            parse("#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }");
        assert!(
            node.errors().is_empty(),
            "parse de #let fib(n) gerou erros: {:?}",
            node.errors()
        );
    }

    // ── P814 — parse_anchored (span sintético, SpanMode::Uniform) ──────────

    #[test]
    fn p814_parse_anchored_aplica_span_a_todos_os_nos() {
        use crate::entities::file_id::FileId;
        use crate::entities::span::Span;
        use std::num::NonZeroU16;
        let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
        let anchor = Span::from_range(id, 5..12);
        let root = parse_anchored("1 + 2", SyntaxMode::Code, anchor);
        fn todos_com_span(node: &SyntaxNode, span: Span) -> bool {
            node.span() == span && node.children().all(|c| todos_com_span(c, span))
        }
        assert!(todos_com_span(&root, anchor), "todos os nós devem ter o span âncora");
    }

    #[test]
    fn p814_parse_anchored_erros_herdam_span() {
        use crate::entities::file_id::FileId;
        use crate::entities::span::Span;
        use std::num::NonZeroU16;
        let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
        let anchor = Span::from_range(id, 5..12);
        let root = parse_anchored("1 +", SyntaxMode::Code, anchor);
        let errors = root.errors();
        assert!(!errors.is_empty());
        assert!(
            errors.iter().all(|e| e.span == anchor),
            "erros de sintaxe devem herdar o span âncora"
        );
    }

    #[test]
    fn p814_parse_anchored_anchor_detached_nao_sintetiza() {
        use crate::entities::span::Span;
        let root = parse_anchored("1 + 2", SyntaxMode::Code, Span::detached());
        assert_eq!(root.kind(), SyntaxKind::Code);
        assert!(root.span().is_detached());
    }

    #[test]
    fn p814_parse_anchored_modos() {
        use crate::entities::span::Span;
        assert_eq!(
            parse_anchored("x", SyntaxMode::Markup, Span::detached()).kind(),
            SyntaxKind::Markup
        );
        assert_eq!(
            parse_anchored("x", SyntaxMode::Math, Span::detached()).kind(),
            SyntaxKind::Math
        );
        assert_eq!(
            parse_anchored("x", SyntaxMode::Code, Span::detached()).kind(),
            SyntaxKind::Code
        );
    }
}
