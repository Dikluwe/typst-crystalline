//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/arguments.md
//! @prompt-hash 7d5b8d9e
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn expect_no_named_retorna_err_com_named_arg() {
    let mut a = p(vec![]);
    a.named.insert("foo".into(), Value::Int(1));
    let result: SourceResult<()> = expect_no_named(&a.named);
    assert!(result.is_err());
    let err_msg = &result.unwrap_err()[0].message;
    assert!(err_msg.contains("inesperado"), "mensagem: {:?}", err_msg);
}

#[test]
fn expect_no_named_ok_com_vazio() {
    let a = p(vec![]);
    assert!(expect_no_named(&a.named).is_ok());
}
