//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/assertions.md
//! @prompt-hash 6f3616f9
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn native_assert_true_nao_gera_erro() {
    null_ctx!(ctx);
    let args = p(vec![Value::Bool(true)]);
    assert!(
        native_assert(&mut ctx, &args, &null_world(), test_file_id()).is_ok(),
        "assert(true) deve ter sucesso"
    );
}

#[test]
fn native_assert_false_gera_erro_com_mensagem_padrao() {
    null_ctx!(ctx);
    let args = p(vec![Value::Bool(false)]);
    let result = native_assert(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err());
    let err = result.unwrap_err();
    // P843 (F7) — paridade vanilla: "assertion failed".
    assert_eq!(err[0].message, "assertion failed");
}

#[test]
fn native_assert_false_gera_erro_com_mensagem_personalizada() {
    null_ctx!(ctx);
    // Mensagem sem acentos para evitar problemas de codificação em CI.
    let args =
        pn(vec![Value::Bool(false)], "message", Value::Str("Matematica falhou".into()));
    let result = native_assert(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err());
    assert!(result.unwrap_err()[0].message.contains("Matematica falhou"));
}

#[test]
fn native_assert_rejeita_named_arg_invalido() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Bool(true)], "bla", Value::Str("bla".into()));
    let result = native_assert(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err[0].message.contains("inesperado") && err[0].message.contains("bla"),
        "named arg desconhecido deve gerar erro: {:?}",
        err[0].message
    );
}

#[test]
fn p723_assert_eq_iguais_ok() {
    null_ctx!(ctx);
    let args = p(vec![Value::Int(1), Value::Int(1)]);
    assert!(
        native_assert_eq(&mut ctx, &args, &null_world(), test_file_id()).is_ok(),
        "assert.eq(1, 1) deve passar"
    );
}

#[test]
fn p723_assert_eq_diferentes_erro_com_repr_vanilla() {
    null_ctx!(ctx);
    let args = p(vec![Value::Int(1), Value::Int(2)]);
    let err = native_assert_eq(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("assert.eq(1, 2) deve falhar");
    // Mensagem default do vanilla (foundations/mod.rs:215-219) — a
    // mecânica é o observável.
    assert_eq!(err[0].message, "equality assertion failed: value 1 was not equal to 2");
}

#[test]
fn p723_assert_eq_com_message_customizada() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Int(1), Value::Int(2)], "message", Value::Str("x".into()));
    let err = native_assert_eq(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("assert.eq com message deve falhar com a mensagem");
    assert_eq!(err[0].message, "equality assertion failed: x");
}

#[test]
fn p723_assert_eq_coercao_int_float() {
    // Paridade com o == da linguagem (coerção Int↔Float, vanilla
    // PartialEq de Value): assert.eq(1, 1.0) passa.
    null_ctx!(ctx);
    let args = p(vec![Value::Int(1), Value::Float(1.0)]);
    assert!(
        native_assert_eq(&mut ctx, &args, &null_world(), test_file_id()).is_ok(),
        "assert.eq(1, 1.0) deve passar (coerção Int↔Float)"
    );
}

#[test]
fn p723_assert_ne_diferentes_ok() {
    null_ctx!(ctx);
    let args = p(vec![Value::Int(1), Value::Int(2)]);
    assert!(
        native_assert_ne(&mut ctx, &args, &null_world(), test_file_id()).is_ok(),
        "assert.ne(1, 2) deve passar"
    );
}

#[test]
fn p723_assert_ne_iguais_erro_com_repr_vanilla() {
    null_ctx!(ctx);
    let args = p(vec![Value::Int(1), Value::Int(1)]);
    let err = native_assert_ne(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("assert.ne(1, 1) deve falhar");
    assert_eq!(err[0].message, "inequality assertion failed: value 1 was equal to 1");
}

#[test]
fn p723_assert_ne_com_message_customizada() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Int(1), Value::Int(1)], "message", Value::Str("x".into()));
    let err = native_assert_ne(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("assert.ne com message deve falhar com a mensagem");
    assert_eq!(err[0].message, "inequality assertion failed: x");
}

#[test]
fn p723_assert_eq_requer_2_posicionais() {
    null_ctx!(ctx);
    let args = p(vec![Value::Int(1)]);
    assert!(
        native_assert_eq(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
        "assert.eq com 1 posicional deve ser erro"
    );
}

#[test]
fn native_panic_aborta_com_mensagem() {
    null_ctx!(ctx);
    let result = native_panic(
        &mut ctx,
        &p(vec![Value::Str("fail".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(result.is_err());
    let err = result.unwrap_err();
    // P843 (F6) — paridade vanilla: prefixo "panicked with: ".
    assert_eq!(err[0].message, "panicked with: fail");
}

#[test]
fn native_panic_aceita_mensagem_vazia() {
    null_ctx!(ctx);
    let result = native_panic(
        &mut ctx,
        &p(vec![Value::Str("".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err[0].message, "panicked with: ");
}

#[test]
fn native_panic_rejeita_tipo_errado() {
    null_ctx!(ctx);
    let result =
        native_panic(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(result.is_err());
}

#[test]
fn native_panic_rejeita_named_arg() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Str("x".into())], "foo", Value::Int(1));
    assert!(native_panic(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}
