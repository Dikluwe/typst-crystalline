//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/footnote.md
//! @prompt-hash f4d72ebd
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p295_native_footnote_body_posicional() {
    use super::native_footnote;
    null_ctx!(ctx);
    let body = Value::Content(Content::text("nota"));
    let r =
        native_footnote(&mut ctx, &p(vec![body]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Footnote(e)) = r {
        assert_eq!(e.body.plain_text(), "nota");
    } else {
        panic!("esperava Content::Footnote");
    }
}

#[test]
fn p295_native_footnote_body_string_converte_para_text() {
    use super::native_footnote;
    null_ctx!(ctx);
    let r = native_footnote(
        &mut ctx,
        &p(vec![Value::Str("texto".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Footnote(e)) = r {
        assert_eq!(e.body.plain_text(), "texto");
    } else {
        panic!("esperava Content::Footnote");
    }
}

#[test]
fn p295_native_footnote_sem_body_retorna_err() {
    use super::native_footnote;
    null_ctx!(ctx);
    let r = native_footnote(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err());
    assert!(format!("{:?}", r).contains("posicional"));
}

#[test]
fn p295_native_footnote_numbering_aceite_p502() {
    use super::native_footnote;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("a".into())]);
    args.named.insert("numbering".into(), Value::Str("*".into()));
    let r = native_footnote(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_ok(), "P502: numbering deve ser aceite em footnote()");
    if let Value::Content(Content::Footnote(e)) = r.unwrap() {
        assert_eq!(e.numbering.as_deref(), Some("*"));
    } else {
        panic!("esperado Content::Footnote");
    }
}

#[test]
fn p295_native_footnote_named_arg_desconhecido_rejeitado() {
    use super::native_footnote;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("a".into())]);
    args.named.insert("foo".into(), Value::Str("bar".into()));
    let r = native_footnote(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido deve ser rejeitado");
}

#[test]
fn p295_native_footnote_body_content_complexo_preservado() {
    use super::native_footnote;
    // Body é Sequence de Content; preserved estructuralmente.
    null_ctx!(ctx);
    let body = Value::Content(Content::sequence(vec![
        Content::text("hello "),
        Content::text("world"),
    ]));
    let r =
        native_footnote(&mut ctx, &p(vec![body]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Footnote(e)) = r {
        assert_eq!(e.body.plain_text(), "hello world");
    }
}

#[test]
fn p295_footnote_partial_eq_por_body() {
    let a = Content::footnote(Content::text("x"));
    let b = Content::footnote(Content::text("x"));
    let c = Content::footnote(Content::text("y"));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn p295_footnote_is_empty_sempre_false() {
    // Marker [N] é sempre observable; footnote nunca vazia.
    let f_empty_body = Content::footnote(Content::Empty);
    assert!(
        !f_empty_body.is_empty(),
        "footnote nunca é is_empty mesmo com body vazio (marker sempre observable)"
    );
}
