//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/tests/sequence.md
//! @prompt-hash 6b83bee1
//! @layer L1
//! @updated 2026-09-26

use super::super::*;

#[test]
fn sequence_zero_partes_e_empty() {
    let c = Content::sequence(vec![]);
    assert!(c.is_empty());
    assert_eq!(c, Content::Empty);
}

#[test]
fn sequence_uma_parte_desembrulha() {
    let c = Content::sequence(vec![Content::text("a")]);
    assert_eq!(c, Content::text("a"));
}

#[test]
fn sequence_multiplas_partes() {
    let c =
        Content::sequence(vec![Content::text("a"), Content::Space, Content::text("b")]);
    assert_eq!(c.plain_text(), "a b");
    assert!(!c.is_empty());
}

#[test]
fn sequence_is_empty_para_vec_vazio() {
    let c = Content::Sequence(Arc::from(Vec::<Content>::new().into_boxed_slice()));
    assert!(c.is_empty());
}

#[test]
fn nested_sequence_plain_text() {
    let inner = Content::sequence(vec![Content::text("x"), Content::text("y")]);
    let outer = Content::sequence(vec![inner, Content::Space, Content::text("z")]);
    assert_eq!(outer.plain_text(), "xy z");
}

// ── Passo 23 ────────────────────────────────────────────────────────────

#[test]
fn content_math_sequence_plain_text() {
    let seq = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::MathText("+".into()),
            Content::MathIdent("y".into()),
        ]
        .into_boxed_slice(),
    ));
    assert_eq!(seq.plain_text(), "x+y");
}

#[test]
fn sequence_clone_e_o1() {
    let seq = Content::sequence(vec![
        Content::text("a"),
        Content::text("b"),
        Content::text("c"),
    ]);
    let clone = seq.clone();
    // PartialEq por conteúdo — não por ponteiro
    assert_eq!(seq, clone);
}

#[test]
fn sequence_partialeq_por_conteudo() {
    let s1 = Content::sequence(vec![Content::text("hello")]);
    let s2 = Content::sequence(vec![Content::text("hello")]);
    // Dois Arc distintos com mesmo conteúdo → iguais
    assert_eq!(s1, s2);
}

#[test]
fn sequence_partialeq_conteudos_diferentes() {
    let s1 = Content::sequence(vec![Content::text("a")]);
    let s2 = Content::sequence(vec![Content::text("b")]);
    assert_ne!(s1, s2);
}

// ── Passo 67 — map_text ───────────────────────────────────────────────────
