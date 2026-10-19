//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/foundations.md
//! @prompt-hash b6c8c56a
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn native_type_directo() {
    null_ctx!(ctx);
    // P685 — type() devolve Value::Type, não Value::Str.
    assert_eq!(
        native_type(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Type(crate::entities::value::Type::Int)
    );
    assert_eq!(
        native_type(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Type(crate::entities::value::Type::Bool)
    );
    assert_eq!(
        native_type(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id())
            .unwrap(),
        Value::Type(crate::entities::value::Type::None)
    );
    assert!(native_type(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    assert!(native_type(
        &mut ctx,
        &p(vec![Value::Int(1), Value::Int(2)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn native_type_named_arg_retorna_err() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Int(1)], "extra", Value::Bool(true));
    assert!(
        native_type(&mut ctx, &args, &null_world(), test_file_id()).is_err(),
        "named arg inesperado deve retornar Err"
    );
}

#[test]
fn stdlib_rgb_tres_args() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let r = native_rgb(
        &mut ctx,
        &p(vec![Value::Int(255), Value::Int(0), Value::Int(128)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Color(Color::rgb(255, 0, 128)));
}

#[test]
fn stdlib_rgb_quatro_args() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let r = native_rgb(
        &mut ctx,
        &p(vec![Value::Int(255), Value::Int(0), Value::Int(0), Value::Int(200)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(r, Value::Color(Color::rgba(255, 0, 0, 200)));
}

#[test]
fn stdlib_rgb_out_of_range() {
    null_ctx!(ctx);
    assert!(native_rgb(
        &mut ctx,
        &p(vec![Value::Int(300), Value::Int(0), Value::Int(0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn stdlib_luma() {
    // P257 (ADR-0083 PROPOSTO) — `luma()` agora constrói
    // `Color::Luma` (paridade vanilla D65Gray); paridade
    // observable preservada via `to_srgb()` que expande Luma
    // para sRGB cinza bit-equivalente.
    null_ctx!(ctx);
    let r =
        native_luma(&mut ctx, &p(vec![Value::Int(128)]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Color(c) = r {
        let (r_, g_, b_, a_) = c.to_srgb();
        assert_eq!(r_, 128);
        assert_eq!(g_, 128);
        assert_eq!(b_, 128);
        assert_eq!(a_, 255);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn p257_native_oklab_3_args() {
    null_ctx!(ctx);
    let r = native_oklab(
        &mut ctx,
        &p(vec![Value::Ratio(Ratio(0.5)), Value::Float(0.0), Value::Float(0.0)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Color(c) = r {
        // L=0.5 → cinza médio aproximado.
        let (r_, g_, b_, _) = c.to_srgb();
        assert!(r_ > 80 && r_ < 180, "oklab(0.5, 0, 0) → cinza médio; obtido r={}", r_);
        // a=b=0 → sem chroma → r ≈ g ≈ b.
        assert!((r_ as i32 - g_ as i32).abs() <= 3);
        assert!((g_ as i32 - b_ as i32).abs() <= 3);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn p257_native_oklab_4_args_com_alpha() {
    null_ctx!(ctx);
    let r = native_oklab(
        &mut ctx,
        &p(vec![
            Value::Ratio(Ratio(1.0)),
            Value::Float(0.0),
            Value::Float(0.0),
            Value::Ratio(Ratio(0.5)),
        ]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Color(c) = r {
        let (_, _, _, a_) = c.to_srgb();
        // alpha=0.5 → 127 ou 128 conforme arredondamento.
        assert!(a_ == 127 || a_ == 128, "alpha=0.5 → 127/128; obtido {}", a_);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn p257_native_oklch_3_args() {
    null_ctx!(ctx);
    let r = native_oklch(
        &mut ctx,
        &p(vec![
            Value::Ratio(Ratio(0.5)),
            Value::Float(0.0),
            Value::Angle(Angle::deg(0.0)),
        ]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(matches!(r, Value::Color(_)));
}

#[test]
fn p257_native_linear_rgb_3_args() {
    // **P736** — paridade vanilla medida: `linear-rgb` rejeita Float
    // ("expected integer or ratio, found float"); aceita Int [0, 255].
    null_ctx!(ctx);
    let r = native_linear_rgb(
        &mut ctx,
        &p(vec![Value::Int(255), Value::Int(0), Value::Int(0)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Color(c) = r {
        let (r_, g_, b_, _) = c.to_srgb();
        assert_eq!(r_, 255);
        assert_eq!(g_, 0);
        assert_eq!(b_, 0);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn p257_native_cmyk_branco() {
    null_ctx!(ctx);
    let r = native_cmyk(
        &mut ctx,
        &p(vec![
            Value::Ratio(Ratio(0.0)),
            Value::Ratio(Ratio(0.0)),
            Value::Ratio(Ratio(0.0)),
            Value::Ratio(Ratio(0.0)),
        ]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Color(c) = r {
        let (r_, g_, b_, _) = c.to_srgb();
        assert_eq!(r_, 255);
        assert_eq!(g_, 255);
        assert_eq!(b_, 255);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn p257_native_cmyk_4_args_obrigatorios() {
    null_ctx!(ctx);
    let r = native_cmyk(
        &mut ctx,
        &p(vec![
            Value::Ratio(Ratio(0.0)),
            Value::Ratio(Ratio(0.0)),
            Value::Ratio(Ratio(0.0)),
        ]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "cmyk requer 4 args; 3 args deve falhar");
}

#[test]
fn p257_native_hsl_vermelho_puro() {
    null_ctx!(ctx);
    let r = native_hsl(
        &mut ctx,
        &p(vec![
            Value::Angle(Angle::deg(0.0)),
            Value::Ratio(Ratio(1.0)),
            Value::Ratio(Ratio(0.5)),
        ]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Color(c) = r {
        let (r_, g_, b_, _) = c.to_srgb();
        assert_eq!(r_, 255);
        assert_eq!(g_, 0);
        assert_eq!(b_, 0);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn p257_native_hsv_branco_s0_v1() {
    null_ctx!(ctx);
    let r = native_hsv(
        &mut ctx,
        &p(vec![
            Value::Angle(Angle::deg(0.0)),
            Value::Ratio(Ratio(0.0)),
            Value::Ratio(Ratio(1.0)),
        ]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Color(c) = r {
        let (r_, g_, b_, _) = c.to_srgb();
        assert_eq!(r_, 255);
        assert_eq!(g_, 255);
        assert_eq!(b_, 255);
    } else {
        panic!("esperado Value::Color");
    }
}

#[test]
fn native_str_de_int() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Str("42".into())
    );
}

#[test]
#[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
fn native_str_de_float() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(&mut ctx, &p(vec![Value::Float(3.14)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Str("3.14".into())
    );
}

#[test]
fn native_str_de_bool() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Str("true".into())
    );
    assert_eq!(
        native_str(&mut ctx, &p(vec![Value::Bool(false)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Str("false".into())
    );
}

#[test]
fn native_str_identity() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(
            &mut ctx,
            &p(vec![Value::Str("hello".into())]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Str("hello".into())
    );
}

#[test]
fn native_str_de_none() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id())
            .unwrap(),
        Value::Str("none".into())
    );
}

#[test]
fn p495_str_base_named() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(
            &mut ctx,
            &pn(vec![Value::Int(255)], "base", Value::Int(16)),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Str("ff".into())
    );
    assert_eq!(
        native_str(
            &mut ctx,
            &pn(vec![Value::Int(42)], "base", Value::Int(2)),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Str("101010".into())
    );
    assert_eq!(
        native_str(&mut ctx, &p(vec![Value::Int(255)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Str("255".into())
    );
}

#[test]
fn p495_str_base_negativo() {
    null_ctx!(ctx);
    assert_eq!(
        native_str(
            &mut ctx,
            &pn(vec![Value::Int(-255)], "base", Value::Int(16)),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Str("-ff".into())
    );
}

#[test]
fn p495_str_base_invalida() {
    null_ctx!(ctx);
    assert!(native_str(
        &mut ctx,
        &pn(vec![Value::Int(255)], "base", Value::Int(37)),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(native_str(
        &mut ctx,
        &pn(vec![Value::Int(255)], "base", Value::Int(1)),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(native_str(
        &mut ctx,
        &pn(vec![Value::Float(3.125)], "base", Value::Int(16)),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn p495_str_named_desconhecido_rejeitado() {
    null_ctx!(ctx);
    assert!(native_str(
        &mut ctx,
        &pn(vec![Value::Int(255)], "foo", Value::Int(16)),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn native_int_de_int() {
    null_ctx!(ctx);
    assert_eq!(
        native_int(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(42)
    );
}

#[test]
fn native_int_de_str() {
    null_ctx!(ctx);
    assert_eq!(
        native_int(
            &mut ctx,
            &p(vec![Value::Str("42".into())]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(42)
    );
    assert!(native_int(
        &mut ctx,
        &p(vec![Value::Str("abc".into())]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn native_int_de_bool() {
    null_ctx!(ctx);
    assert_eq!(
        native_int(&mut ctx, &p(vec![Value::Bool(true)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        native_int(&mut ctx, &p(vec![Value::Bool(false)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(0)
    );
}

#[test]
fn native_int_float_trunca_para_zero() {
    null_ctx!(ctx);
    assert_eq!(
        native_int(&mut ctx, &p(vec![Value::Float(3.7)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(3)
    );
}

#[test]
fn native_float_de_int() {
    null_ctx!(ctx);
    assert_eq!(
        native_float(&mut ctx, &p(vec![Value::Int(3)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(3.0)
    );
}

#[test]
#[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
fn native_float_de_str() {
    null_ctx!(ctx);
    assert_eq!(
        native_float(
            &mut ctx,
            &p(vec![Value::Str("3.14".into())]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Float(3.14)
    );
    assert!(native_float(
        &mut ctx,
        &p(vec![Value::Str("abc".into())]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn native_range_directo() {
    null_ctx!(ctx);
    assert_eq!(
        native_range(&mut ctx, &p(vec![Value::Int(3)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)])
    );
    assert_eq!(
        native_range(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)])
    );
    assert_eq!(
        native_range(
            &mut ctx,
            &p(vec![Value::Int(3), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Array(vec![])
    );
    // P704 — corrigido: medido directamente contra o vanilla,
    // `range(-1)`/`range(-5)` devolvem array vazio, não Err. O `if n < 0`
    // anterior era uma divergência do cristalino, não paridade vanilla.
    assert_eq!(
        native_range(&mut ctx, &p(vec![Value::Int(-1)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Array(vec![])
    );
}
