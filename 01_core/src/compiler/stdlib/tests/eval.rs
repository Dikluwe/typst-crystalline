//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/eval.md
//! @prompt-hash 81e3da9e
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p394_eval_retorna_valor_da_ultima_expressao() {
    assert_eq!(run_eval("1 + 2").unwrap(), Value::Int(3));
    assert_eq!(run_eval("7 * 8").unwrap(), Value::Int(56));
    assert_eq!(run_eval("\"foo\"").unwrap(), Value::Str("foo".into()));
}

#[test]
fn p394_eval_nao_ve_escopo_exterior() {
    // P830 (decisão do dono, 2026-07-22) — paridade vanilla: `eval` corre
    // num `Scopes` fresco; a variável do scope do chamador não é visível.
    let diags = run_eval_with_var("x + 3", "x", Value::Int(7))
        .expect_err("eval não deve ver a variável do scope exterior");
    assert!(
        diags.iter().any(|d| d.message.contains("unknown variable `x`")),
        "esperado 'unknown variable: x': {diags:?}"
    );
}

#[test]
fn p394_eval_content_block_produz_content() {
    let r = run_eval("[*bold*]").unwrap();
    match r {
        Value::Content(c) => {
            // O content block deve conter texto strong com "bold".
            assert_eq!(c.plain_text(), "bold");
        }
        other => panic!("esperado Value::Content, obteve {other:?}"),
    }
}

#[test]
fn p394_eval_identificador_desconhecido_erro() {
    let r = run_eval("variavel_inexistente");
    assert!(r.is_err(), "identificador desconhecido deve falhar");
}

#[test]
fn p394_eval_sintaxe_invalida_erro() {
    let r = run_eval("let x =");
    assert!(r.is_err(), "sintaxe inválida deve falhar");
}

#[test]
fn p394_eval_tipo_errado_erro() {
    let r = with_engine(|engine, world| {
        native_eval(
            &mut EvalContext::new(),
            &p(vec![Value::Int(42)]),
            world,
            test_file_id(),
            &mut Scopes::new(None),
            engine,
        )
    });
    assert!(r.is_err(), "eval com argumento não-string deve falhar");
}

#[test]
fn p394_eval_arity_errado_erro() {
    let r = with_engine(|engine, world| {
        native_eval(
            &mut EvalContext::new(),
            &p(vec![]),
            world,
            test_file_id(),
            &mut Scopes::new(None),
            engine,
        )
    });
    assert!(r.is_err(), "eval sem argumentos deve falhar");
}

#[test]
fn p394_eval_named_arg_inesperado_erro() {
    // P814 — `mode:`/`scope:` passaram a ser aceites (paridade vanilla);
    // o contrato de rejeição mantém-se para named args desconhecidos.
    let mut args = p(vec![Value::Str("1".into())]);
    args.named.insert("foo".into(), Value::Str("code".into()));
    let r = with_engine(|engine, world| {
        native_eval(
            &mut EvalContext::new(),
            &args,
            world,
            test_file_id(),
            &mut Scopes::new(None),
            engine,
        )
    });
    assert!(r.is_err(), "named arg inesperado deve falhar");
}
