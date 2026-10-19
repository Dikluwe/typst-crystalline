//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/calc.md
//! @prompt-hash 40a89674
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn calc_abs_int() {
    null_ctx!(ctx);
    assert_eq!(
        calc_abs(&mut ctx, &p(vec![Value::Int(-5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(5)
    );
    assert_eq!(
        calc_abs(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(5)
    );
    assert_eq!(
        calc_abs(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(0)
    );
}

#[test]
#[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
fn calc_abs_float() {
    null_ctx!(ctx);
    assert_eq!(
        calc_abs(&mut ctx, &p(vec![Value::Float(-3.14)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(3.14)
    );
}

#[test]
fn calc_pow_int() {
    null_ctx!(ctx);
    assert_eq!(
        calc_pow(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Int(10)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1024)
    );
    assert_eq!(
        calc_pow(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
}

#[test]
fn calc_pow_float() {
    null_ctx!(ctx);
    let r = calc_pow(
        &mut ctx,
        &p(vec![Value::Float(2.0), Value::Float(0.5)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(matches!(r, Value::Float(f) if (f - std::f64::consts::SQRT_2).abs() < 1e-10));
}

#[test]
fn calc_pow_expoente_negativo_devolve_float() {
    // P817-C — paridade vanilla: `calc.pow(2, -1)` → `0.5` (Float).
    null_ctx!(ctx);
    assert_eq!(
        calc_pow(
            &mut ctx,
            &p(vec![Value::Int(2), Value::Int(-1)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Float(0.5)
    );
}

#[test]
fn calc_sqrt_positivo() {
    null_ctx!(ctx);
    assert_eq!(
        calc_sqrt(&mut ctx, &p(vec![Value::Float(4.0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(2.0)
    );
    assert_eq!(
        calc_sqrt(&mut ctx, &p(vec![Value::Int(4)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(2.0)
    );
}

#[test]
fn calc_sqrt_negativo_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_sqrt(
        &mut ctx,
        &p(vec![Value::Float(-1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_floor_ceil_round() {
    null_ctx!(ctx);
    assert_eq!(
        calc_floor(&mut ctx, &p(vec![Value::Float(3.7)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(3)
    );
    assert_eq!(
        calc_ceil(&mut ctx, &p(vec![Value::Float(3.2)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(4)
    );
    assert_eq!(
        calc_round(&mut ctx, &p(vec![Value::Float(3.5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(4.0)
    );
    assert_eq!(
        calc_round(&mut ctx, &p(vec![Value::Float(3.4)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(3.0)
    );
}

#[test]
fn p495_calc_round_digits_named() {
    null_ctx!(ctx);
    approx_float(
        calc_round(
            &mut ctx,
            &pn(vec![Value::Float(3.567)], "digits", Value::Int(2)),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.57,
    );
    approx_float(
        calc_round(
            &mut ctx,
            &pn(vec![Value::Float(1234.0)], "digits", Value::Int(-2)),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        1200.0,
    );
    // default 0 preservado — P817-D: `round(Float)` → `Float` (paridade vanilla).
    assert_eq!(
        calc_round(&mut ctx, &p(vec![Value::Float(3.5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(4.0)
    );
}

#[test]
fn p495_calc_round_digits_int_input() {
    null_ctx!(ctx);
    // P817-D — paridade vanilla: `round(Int)` → `Int` (digits > 0 é no-op).
    assert_eq!(
        calc_round(
            &mut ctx,
            &pn(vec![Value::Int(255)], "digits", Value::Int(2)),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        Value::Int(255),
    );
}

#[test]
fn p495_calc_round_named_desconhecido_rejeitado() {
    null_ctx!(ctx);
    assert!(calc_round(
        &mut ctx,
        &pn(vec![Value::Float(3.5)], "foo", Value::Int(2)),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_min_max_int() {
    null_ctx!(ctx);
    assert_eq!(
        calc_min(
            &mut ctx,
            &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_max(
            &mut ctx,
            &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(3)
    );
}

#[test]
fn calc_min_vazio_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_min(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    assert!(calc_max(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
}

#[test]
fn calc_clamp_int() {
    null_ctx!(ctx);
    assert_eq!(
        calc_clamp(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(0), Value::Int(10)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(5)
    );
    assert_eq!(
        calc_clamp(
            &mut ctx,
            &p(vec![Value::Int(-5), Value::Int(0), Value::Int(10)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(0)
    );
    assert_eq!(
        calc_clamp(
            &mut ctx,
            &p(vec![Value::Int(15), Value::Int(0), Value::Int(10)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(10)
    );
}

#[test]
fn calc_clamp_min_maior_max_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_clamp(
        &mut ctx,
        &p(vec![Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_sin_zero_e_pi() {
    null_ctx!(ctx);
    approx_float(
        calc_sin(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_float(
        calc_sin(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_float(
        calc_sin(
            &mut ctx,
            &p(vec![Value::Float(std::f64::consts::PI)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        0.0,
    );
}

#[test]
fn calc_sin_arity_errada_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_sin(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    assert!(calc_sin(
        &mut ctx,
        &p(vec![Value::Float(1.0), Value::Float(2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_cos_zero_e_pi() {
    null_ctx!(ctx);
    approx_float(
        calc_cos(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        1.0,
    );
    approx_float(
        calc_cos(
            &mut ctx,
            &p(vec![Value::Float(std::f64::consts::PI)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        -1.0,
    );
}

#[test]
fn calc_tan_zero_e_pi_quarto() {
    null_ctx!(ctx);
    approx_float(
        calc_tan(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_float(
        calc_tan(
            &mut ctx,
            &p(vec![Value::Float(std::f64::consts::FRAC_PI_4)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        1.0,
    );
}

#[test]
fn calc_asin_dominio_valido() {
    null_ctx!(ctx);
    approx_angle(
        calc_asin(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_angle(
        calc_asin(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap(),
        std::f64::consts::FRAC_PI_2,
    );
    approx_angle(
        calc_asin(&mut ctx, &p(vec![Value::Float(-1.0)]), &null_world(), test_file_id())
            .unwrap(),
        -std::f64::consts::FRAC_PI_2,
    );
}

#[test]
fn calc_asin_fora_dominio_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_asin(
        &mut ctx,
        &p(vec![Value::Float(1.5)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_asin(
        &mut ctx,
        &p(vec![Value::Float(-1.5)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_acos_dominio_valido() {
    null_ctx!(ctx);
    approx_angle(
        calc_acos(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_angle(
        calc_acos(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        std::f64::consts::FRAC_PI_2,
    );
    approx_angle(
        calc_acos(&mut ctx, &p(vec![Value::Float(-1.0)]), &null_world(), test_file_id())
            .unwrap(),
        std::f64::consts::PI,
    );
}

#[test]
fn calc_acos_fora_dominio_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_acos(
        &mut ctx,
        &p(vec![Value::Float(2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_acos(
        &mut ctx,
        &p(vec![Value::Float(-2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_atan_basico() {
    null_ctx!(ctx);
    approx_angle(
        calc_atan(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_angle(
        calc_atan(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap(),
        std::f64::consts::FRAC_PI_4,
    );
}

#[test]
fn calc_atan2_ordem_xy_paridade_vanilla() {
    null_ctx!(ctx);
    // calc.atan2(x=1, y=1) → π/4 (paridade vanilla).
    approx_angle(
        calc_atan2(
            &mut ctx,
            &p(vec![Value::Float(1.0), Value::Float(1.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        std::f64::consts::FRAC_PI_4,
    );
    // calc.atan2(x=0, y=1) → π/2 (eixo +y).
    approx_angle(
        calc_atan2(
            &mut ctx,
            &p(vec![Value::Float(0.0), Value::Float(1.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        std::f64::consts::FRAC_PI_2,
    );
}

#[test]
fn calc_atan2_arity_errada_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_atan2(
        &mut ctx,
        &p(vec![Value::Float(1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_atan2(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
}

#[test]
fn calc_sinh_cosh_tanh_zero() {
    null_ctx!(ctx);
    approx_float(
        calc_sinh(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_float(
        calc_cosh(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        1.0,
    );
    approx_float(
        calc_tanh(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
}

#[test]
fn calc_sinh_cosh_identidade_pitagorica() {
    null_ctx!(ctx);
    // cosh²(x) - sinh²(x) = 1 (identidade hiperbólica fundamental).
    let s = match calc_sinh(
        &mut ctx,
        &p(vec![Value::Float(1.5)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap()
    {
        Value::Float(f) => f,
        _ => panic!(),
    };
    let c = match calc_cosh(
        &mut ctx,
        &p(vec![Value::Float(1.5)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap()
    {
        Value::Float(f) => f,
        _ => panic!(),
    };
    assert!((c * c - s * s - 1.0).abs() < 1e-10, "cosh²-sinh² = {}", c * c - s * s);
}

#[test]
fn calc_asinh_inverso_de_sinh() {
    null_ctx!(ctx);
    approx_float(
        calc_asinh(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    // sinh(asinh(2)) = 2.
    let v =
        calc_asinh(&mut ctx, &p(vec![Value::Float(2.0)]), &null_world(), test_file_id())
            .unwrap();
    let back = calc_sinh(&mut ctx, &p(vec![v]), &null_world(), test_file_id()).unwrap();
    approx_float(back, 2.0);
}

#[test]
fn calc_acosh_dominio_valido() {
    null_ctx!(ctx);
    approx_float(
        calc_acosh(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    // cosh(acosh(2.5)) = 2.5.
    let v =
        calc_acosh(&mut ctx, &p(vec![Value::Float(2.5)]), &null_world(), test_file_id())
            .unwrap();
    let back = calc_cosh(&mut ctx, &p(vec![v]), &null_world(), test_file_id()).unwrap();
    approx_float(back, 2.5);
}

#[test]
fn calc_acosh_fora_dominio_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_acosh(
        &mut ctx,
        &p(vec![Value::Float(0.5)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_acosh(
        &mut ctx,
        &p(vec![Value::Float(-1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_atanh_dominio_valido() {
    null_ctx!(ctx);
    approx_float(
        calc_atanh(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    // tanh(atanh(0.5)) = 0.5.
    let v =
        calc_atanh(&mut ctx, &p(vec![Value::Float(0.5)]), &null_world(), test_file_id())
            .unwrap();
    let back = calc_tanh(&mut ctx, &p(vec![v]), &null_world(), test_file_id()).unwrap();
    approx_float(back, 0.5);
}

#[test]
fn calc_atanh_fora_dominio_retorna_err() {
    null_ctx!(ctx);
    // Fronteiras (-1 e 1) são exclusivas — Err em ambos.
    assert!(calc_atanh(
        &mut ctx,
        &p(vec![Value::Float(1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_atanh(
        &mut ctx,
        &p(vec![Value::Float(-1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_atanh(
        &mut ctx,
        &p(vec![Value::Float(1.5)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_exp_zero_e_um() {
    null_ctx!(ctx);
    approx_float(
        calc_exp(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        1.0,
    );
    approx_float(
        calc_exp(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
            .unwrap(),
        std::f64::consts::E,
    );
}

#[test]
fn calc_exp_overflow_retorna_err() {
    null_ctx!(ctx);
    // exp(1e10) → Inf → guard_float devolve Err.
    assert!(calc_exp(
        &mut ctx,
        &p(vec![Value::Float(1e10)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_ln_e_e_dominio() {
    null_ctx!(ctx);
    approx_float(
        calc_ln(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.0,
    );
    approx_float(
        calc_ln(
            &mut ctx,
            &p(vec![Value::Float(std::f64::consts::E)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        1.0,
    );
}

#[test]
fn calc_ln_nao_positivo_retorna_err() {
    null_ctx!(ctx);
    assert!(calc_ln(
        &mut ctx,
        &p(vec![Value::Float(0.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_ln(
        &mut ctx,
        &p(vec![Value::Float(-1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_log_base_default_dez() {
    null_ctx!(ctx);
    approx_float(
        calc_log(&mut ctx, &p(vec![Value::Float(100.0)]), &null_world(), test_file_id())
            .unwrap(),
        2.0,
    );
    approx_float(
        calc_log(&mut ctx, &p(vec![Value::Float(1000.0)]), &null_world(), test_file_id())
            .unwrap(),
        3.0,
    );
}

#[test]
fn calc_log_base_explicita() {
    null_ctx!(ctx);
    approx_float(
        calc_log(
            &mut ctx,
            &p(vec![Value::Float(8.0), Value::Float(2.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.0,
    );
    approx_float(
        calc_log(
            &mut ctx,
            &p(vec![Value::Float(27.0), Value::Float(3.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.0,
    );
}

#[test]
fn calc_log_dominio_e_base_invalidos_retorna_err() {
    null_ctx!(ctx);
    // valor ≤ 0.
    assert!(calc_log(
        &mut ctx,
        &p(vec![Value::Float(0.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_log(
        &mut ctx,
        &p(vec![Value::Float(-1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    // base inválida (≤0, 1, Inf, NaN).
    assert!(calc_log(
        &mut ctx,
        &p(vec![Value::Float(10.0), Value::Float(1.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_log(
        &mut ctx,
        &p(vec![Value::Float(10.0), Value::Float(0.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_log(
        &mut ctx,
        &p(vec![Value::Float(10.0), Value::Float(-2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_log(
        &mut ctx,
        &p(vec![Value::Float(10.0), Value::Float(f64::INFINITY)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn p495_calc_log_base_named() {
    null_ctx!(ctx);
    approx_float(
        calc_log(
            &mut ctx,
            &pn(vec![Value::Float(100.0)], "base", Value::Int(10)),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        2.0,
    );
    approx_float(
        calc_log(
            &mut ctx,
            &pn(vec![Value::Float(8.0)], "base", Value::Int(2)),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.0,
    );
    // posicional preservado
    approx_float(
        calc_log(
            &mut ctx,
            &p(vec![Value::Float(27.0), Value::Float(3.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.0,
    );
}

#[test]
fn p495_calc_log_base_e_posicional_sao_mutuamente_exclusivos() {
    null_ctx!(ctx);
    assert!(calc_log(
        &mut ctx,
        &Args::from_parts(
            vec![Value::Float(100.0), Value::Float(10.0)],
            [("base".into(), Value::Int(10))].into_iter().collect(),
            Span::detached(),
        ),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn p495_calc_log_named_desconhecido_rejeitado() {
    null_ctx!(ctx);
    assert!(calc_log(
        &mut ctx,
        &pn(vec![Value::Float(100.0)], "foo", Value::Int(10)),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_trunc_int_e_float() {
    null_ctx!(ctx);
    assert_eq!(
        calc_trunc(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(5)
    );
    assert_eq!(
        calc_trunc(&mut ctx, &p(vec![Value::Float(3.7)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(3)
    );
    assert_eq!(
        calc_trunc(&mut ctx, &p(vec![Value::Float(-3.7)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(-3)
    );
    assert!(calc_trunc(
        &mut ctx,
        &p(vec![Value::Str("x".into())]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_trunc(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
}

#[test]
fn calc_fract_int_e_float() {
    null_ctx!(ctx);
    // P817-D — paridade vanilla: `calc.fract(int)` → int `0`.
    assert_eq!(
        calc_fract(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(0)
    );
    approx_float(
        calc_fract(&mut ctx, &p(vec![Value::Float(3.7)]), &null_world(), test_file_id())
            .unwrap(),
        0.7,
    );
    approx_float(
        calc_fract(&mut ctx, &p(vec![Value::Float(-3.7)]), &null_world(), test_file_id())
            .unwrap(),
        -0.7,
    );
    assert!(calc_fract(
        &mut ctx,
        &p(vec![Value::Bool(true)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_even_odd_int_apenas() {
    null_ctx!(ctx);
    assert_eq!(
        calc_even(&mut ctx, &p(vec![Value::Int(4)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        calc_even(&mut ctx, &p(vec![Value::Int(-3)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        calc_even(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Bool(true)
    );
    assert!(calc_even(
        &mut ctx,
        &p(vec![Value::Float(2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());

    assert_eq!(
        calc_odd(&mut ctx, &p(vec![Value::Int(3)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        calc_odd(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        calc_odd(&mut ctx, &p(vec![Value::Int(-3)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Bool(true)
    );
    assert!(calc_odd(
        &mut ctx,
        &p(vec![Value::Float(2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_rem_truncado() {
    null_ctx!(ctx);
    assert_eq!(
        calc_rem(
            &mut ctx,
            &p(vec![Value::Int(7), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_rem(
            &mut ctx,
            &p(vec![Value::Int(-7), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(-1)
    );
    approx_float(
        calc_rem(
            &mut ctx,
            &p(vec![Value::Float(7.5), Value::Float(2.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        1.5,
    );
    assert!(calc_rem(
        &mut ctx,
        &p(vec![Value::Int(1), Value::Int(0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_rem(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
        .is_err());
}

#[test]
fn calc_rem_euclid_sempre_positivo() {
    null_ctx!(ctx);
    assert_eq!(
        calc_rem_euclid(
            &mut ctx,
            &p(vec![Value::Int(-7), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(2)
    );
    assert_eq!(
        calc_rem_euclid(
            &mut ctx,
            &p(vec![Value::Int(7), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
    approx_float(
        calc_rem_euclid(
            &mut ctx,
            &p(vec![Value::Float(-7.5), Value::Float(2.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        0.5,
    );
    assert!(calc_rem_euclid(
        &mut ctx,
        &p(vec![Value::Int(1), Value::Int(0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_div_euclid_arredonda_para_menos_inf() {
    null_ctx!(ctx);
    assert_eq!(
        calc_div_euclid(
            &mut ctx,
            &p(vec![Value::Int(-7), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(-3)
    );
    assert_eq!(
        calc_div_euclid(
            &mut ctx,
            &p(vec![Value::Int(7), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(2)
    );
    assert!(calc_div_euclid(
        &mut ctx,
        &p(vec![Value::Int(1), Value::Int(0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_quo_floored() {
    // P817-B — paridade vanilla: quociente floored (`calc.quo(-7, 2) = -4`).
    null_ctx!(ctx);
    assert_eq!(
        calc_quo(
            &mut ctx,
            &p(vec![Value::Int(7), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(3)
    );
    assert_eq!(
        calc_quo(
            &mut ctx,
            &p(vec![Value::Int(-7), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(-4)
    );
    assert_eq!(
        calc_quo(
            &mut ctx,
            &p(vec![Value::Float(7.5), Value::Float(2.0)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(3)
    );
    assert!(calc_quo(
        &mut ctx,
        &p(vec![Value::Int(1), Value::Int(0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_gcd_lcm_basico() {
    null_ctx!(ctx);
    assert_eq!(
        calc_gcd(
            &mut ctx,
            &p(vec![Value::Int(12), Value::Int(18)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(6)
    );
    assert_eq!(
        calc_gcd(
            &mut ctx,
            &p(vec![Value::Int(0), Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(5)
    );
    assert_eq!(
        calc_gcd(
            &mut ctx,
            &p(vec![Value::Int(0), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(0)
    );
    assert_eq!(
        calc_gcd(
            &mut ctx,
            &p(vec![Value::Int(-12), Value::Int(18)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(6)
    );

    assert_eq!(
        calc_lcm(
            &mut ctx,
            &p(vec![Value::Int(4), Value::Int(6)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(12)
    );
    assert_eq!(
        calc_lcm(
            &mut ctx,
            &p(vec![Value::Int(0), Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(0)
    );
    assert!(calc_lcm(
        &mut ctx,
        &p(vec![Value::Int(i64::MAX), Value::Int(2)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_fact_basico_e_overflow() {
    null_ctx!(ctx);
    assert_eq!(
        calc_fact(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_fact(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_fact(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(120)
    );
    assert_eq!(
        calc_fact(&mut ctx, &p(vec![Value::Int(20)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Int(2_432_902_008_176_640_000)
    );
    assert!(calc_fact(&mut ctx, &p(vec![Value::Int(-1)]), &null_world(), test_file_id())
        .is_err());
    assert!(calc_fact(&mut ctx, &p(vec![Value::Int(21)]), &null_world(), test_file_id())
        .is_err());
    assert!(calc_fact(
        &mut ctx,
        &p(vec![Value::Float(5.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_perm_arranjos() {
    null_ctx!(ctx);
    assert_eq!(
        calc_perm(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(20)
    );
    assert_eq!(
        calc_perm(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_perm(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(120)
    );
    assert_eq!(
        calc_perm(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(8)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(0)
    );
    assert!(calc_perm(
        &mut ctx,
        &p(vec![Value::Int(-1), Value::Int(2)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_binom_combinacoes() {
    null_ctx!(ctx);
    assert_eq!(
        calc_binom(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(2)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(10)
    );
    assert_eq!(
        calc_binom(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(0)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_binom(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(5)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(1)
    );
    assert_eq!(
        calc_binom(
            &mut ctx,
            &p(vec![Value::Int(5), Value::Int(8)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(0)
    );
    // C(20, 10) = 184756 — caso clássico, fica longe do limite i64.
    assert_eq!(
        calc_binom(
            &mut ctx,
            &p(vec![Value::Int(20), Value::Int(10)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(184_756)
    );
    // Simetria: C(30, 28) deve usar k_eff = 2 internamente.
    assert_eq!(
        calc_binom(
            &mut ctx,
            &p(vec![Value::Int(30), Value::Int(28)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Int(435)
    );
    assert!(calc_binom(
        &mut ctx,
        &p(vec![Value::Int(-1), Value::Int(2)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    // Overflow real (n grande): C(67, 33) sai do alcance i64.
    assert!(calc_binom(
        &mut ctx,
        &p(vec![Value::Int(67), Value::Int(33)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_norm_p2_default_pitagoras() {
    null_ctx!(ctx);
    // Sem args: norma de vector vazio = 0.0.
    assert_eq!(
        calc_norm(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap(),
        Value::Float(0.0)
    );
    // Pitágoras: norma 2 de (3, 4) = 5.
    approx_float(
        calc_norm(
            &mut ctx,
            &p(vec![Value::Int(3), Value::Int(4)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        5.0,
    );
    // abs interno: norma 2 de (-3, 4) = 5.
    approx_float(
        calc_norm(
            &mut ctx,
            &p(vec![Value::Float(-3.0), Value::Float(4.0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        5.0,
    );
}

#[test]
fn calc_norm_p_named() {
    null_ctx!(ctx);
    // Taxicab (p=1) de (1, 1, 1) = 3.
    approx_float(
        calc_norm(
            &mut ctx,
            &pn(
                vec![Value::Int(1), Value::Int(1), Value::Int(1)],
                "p",
                Value::Float(1.0),
            ),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.0,
    );
    // Chebyshev-aproximação (p grande) de (3, 4) ≈ 4.
    let v = calc_norm(
        &mut ctx,
        &pn(vec![Value::Int(3), Value::Int(4)], "p", Value::Float(100.0)),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    match v {
        Value::Float(f) => assert!((f - 4.0).abs() < 1e-2, "esperado ≈4, obtido {f}"),
        _ => panic!(),
    }
}

#[test]
fn calc_norm_rejeita_named_desconhecido() {
    null_ctx!(ctx);
    assert!(calc_norm(
        &mut ctx,
        &pn(vec![Value::Int(3)], "q", Value::Float(2.0)),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_root_raiz_quadrada_e_cubica() {
    // P817 — ordem vanilla: `calc.root(radicand, index)`.
    null_ctx!(ctx);
    approx_float(
        calc_root(
            &mut ctx,
            &p(vec![Value::Int(9), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        3.0,
    );
    approx_float(
        calc_root(
            &mut ctx,
            &p(vec![Value::Int(8), Value::Int(3)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        2.0,
    );
    approx_float(
        calc_root(
            &mut ctx,
            &p(vec![Value::Int(-8), Value::Int(3)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        -2.0,
    );
    approx_float(
        calc_root(
            &mut ctx,
            &p(vec![Value::Float(0.0), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap(),
        0.0,
    );
}

#[test]
fn calc_root_erros() {
    null_ctx!(ctx);
    // Radicando negativo com índice par → Err.
    assert!(calc_root(
        &mut ctx,
        &p(vec![Value::Int(-1), Value::Int(2)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    // Índice zero → Err.
    assert!(calc_root(
        &mut ctx,
        &p(vec![Value::Int(5), Value::Int(0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    // Índice não-Int → Err.
    assert!(calc_root(
        &mut ctx,
        &p(vec![Value::Int(9), Value::Float(2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    assert!(calc_root(&mut ctx, &p(vec![Value::Int(2)]), &null_world(), test_file_id())
        .is_err());
}

#[test]
fn calc_erf_identidade_e_int_coerce() {
    null_ctx!(ctx);
    // erf(0) = 0 — identidade exacta (sign·0·exp(0) = 0).
    assert_eq!(
        calc_erf(&mut ctx, &p(vec![Value::Float(0.0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(0.0),
    );
    // Int coerce: erf(Int 0) idêntico a erf(Float 0.0).
    assert_eq!(
        calc_erf(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap(),
        Value::Float(0.0),
    );
    // Int 1 coerce: deve coincidir com Float 1.0 dentro da tolerância.
    let a = calc_erf(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
        .unwrap();
    let b =
        calc_erf(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap();
    match (a, b) {
        (Value::Float(fa), Value::Float(fb)) => assert!(
            (fa - fb).abs() < 1e-15,
            "Int/Float coerce deve produzir o mesmo bit-pattern: fa={fa}, fb={fb}",
        ),
        _ => panic!("ambos devem ser Value::Float"),
    }
}

#[test]
fn calc_erf_valores_conhecidos() {
    null_ctx!(ctx);
    // Valores tabulares clássicos (Abramowitz & Stegun Tabela 7.1).
    approx_float_erf(
        calc_erf(&mut ctx, &p(vec![Value::Float(0.5)]), &null_world(), test_file_id())
            .unwrap(),
        0.520_499_877_813_046_5,
    );
    approx_float_erf(
        calc_erf(&mut ctx, &p(vec![Value::Float(1.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.842_700_792_949_715,
    );
    approx_float_erf(
        calc_erf(&mut ctx, &p(vec![Value::Float(2.0)]), &null_world(), test_file_id())
            .unwrap(),
        0.995_322_265_018_952_7,
    );
}

#[test]
fn calc_erf_simetria_impar() {
    null_ctx!(ctx);
    for x in [0.1_f64, 0.5, 1.0, 1.7, 3.3] {
        let pos =
            calc_erf(&mut ctx, &p(vec![Value::Float(x)]), &null_world(), test_file_id())
                .unwrap();
        let neg =
            calc_erf(&mut ctx, &p(vec![Value::Float(-x)]), &null_world(), test_file_id())
                .unwrap();
        match (pos, neg) {
            (Value::Float(p_), Value::Float(n_)) => assert!(
                (p_ + n_).abs() < 2e-7,
                "simetria ímpar violada em x={x}: erf(x)={p_}, erf(-x)={n_}",
            ),
            _ => panic!("erf de Float deve retornar Float"),
        }
    }
}

#[test]
fn calc_erf_limites_finitos() {
    null_ctx!(ctx);
    // |x| grande mas finito: erf(±5) ≈ ±1 dentro da tolerância A&S.
    approx_float_erf(
        calc_erf(&mut ctx, &p(vec![Value::Float(5.0)]), &null_world(), test_file_id())
            .unwrap(),
        1.0,
    );
    approx_float_erf(
        calc_erf(&mut ctx, &p(vec![Value::Float(-5.0)]), &null_world(), test_file_id())
            .unwrap(),
        -1.0,
    );
}

#[test]
fn calc_erf_infinito_short_circuit() {
    null_ctx!(ctx);
    // Short-circuit no infinito: retorna ±1.0 exacto sem passar pela
    // fórmula (que produziria 0·∞ = NaN).
    assert_eq!(
        calc_erf(
            &mut ctx,
            &p(vec![Value::Float(f64::INFINITY)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Float(1.0),
    );
    assert_eq!(
        calc_erf(
            &mut ctx,
            &p(vec![Value::Float(f64::NEG_INFINITY)]),
            &null_world(),
            test_file_id()
        )
        .unwrap(),
        Value::Float(-1.0),
    );
}

#[test]
fn calc_erf_nan_err() {
    null_ctx!(ctx);
    // Divergência consciente vs vanilla `libm::erf(NaN) = NaN`:
    // cristalino mapeia para Err (paridade convenção `guard_float`).
    assert!(calc_erf(
        &mut ctx,
        &p(vec![Value::Float(f64::NAN)]),
        &null_world(),
        test_file_id()
    )
    .is_err(),);
}

#[test]
fn calc_erf_arity_e_tipos_invalidos() {
    null_ctx!(ctx);
    // Arity 0 → Err.
    assert!(calc_erf(&mut ctx, &p(vec![]), &null_world(), test_file_id()).is_err());
    // Arity 2 → Err.
    assert!(calc_erf(
        &mut ctx,
        &p(vec![Value::Float(1.0), Value::Float(2.0)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    // Tipo inválido (Str) → Err.
    assert!(calc_erf(
        &mut ctx,
        &p(vec![Value::Str("x".into())]),
        &null_world(),
        test_file_id()
    )
    .is_err());
    // Tipo inválido (Bool) → Err.
    assert!(calc_erf(
        &mut ctx,
        &p(vec![Value::Bool(true)]),
        &null_world(),
        test_file_id()
    )
    .is_err());
}

#[test]
fn calc_erf_named_arg_rejeitado() {
    null_ctx!(ctx);
    // Paridade `expect_no_named` — nenhum named arg é aceite.
    let args = pn(vec![Value::Float(1.0)], "p", Value::Float(0.5));
    assert!(calc_erf(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn calc_erf_registado_no_modulo() {
    // Paridade `make_calc_module`: chave "erf" presente como Func.
    let module = p283_make_calc_module();
    let dict = match module {
        Value::Module(m) => m.scope().clone(),
        other => panic!("esperado Module, obtido {other:?}"),
    };
    assert!(
        matches!(dict.get("erf"), Some(Value::Func(_))),
        "calc.erf não está registado como Func no módulo (P308)",
    );
    // P501: módulo tem 44 funções + 4 constantes = 48 entradas.
    assert_eq!(
        dict.len(),
        48,
        "esperava 44 funções + 4 constantes = 48, obtido {}",
        dict.len()
    );
}

#[test]
fn calc_modulo_contem_funcoes_p306() {
    let module = p283_make_calc_module();
    let dict = match module {
        Value::Module(m) => m.scope().clone(),
        other => panic!("esperado Module, obtido {other:?}"),
    };
    for key in [
        "trunc",
        "fract",
        "even",
        "odd",
        "rem",
        "rem-euclid",
        "div-euclid",
        "quo",
        "gcd",
        "lcm",
        "fact",
        "perm",
        "binom",
        "norm",
        "root",
    ] {
        assert!(
            matches!(dict.get(key), Some(Value::Func(_))),
            "calc.{key} não está registado como Func"
        );
    }
}

#[test]
fn calc_constantes_pi_tau_e_inf() {
    let module = p283_make_calc_module();
    let dict = match module {
        Value::Module(m) => m.scope().clone(),
        other => panic!("esperado Module, obtido {other:?}"),
    };
    assert_eq!(dict.get("pi").cloned(), Some(Value::Float(std::f64::consts::PI)));
    assert_eq!(dict.get("tau").cloned(), Some(Value::Float(std::f64::consts::TAU)));
    assert_eq!(dict.get("e").cloned(), Some(Value::Float(std::f64::consts::E)));
    assert_eq!(dict.get("inf").cloned(), Some(Value::Float(f64::INFINITY)));
}

#[test]
fn calc_modulo_expoe_41_funcoes() {
    let module = p283_make_calc_module();
    let dict = match module {
        Value::Module(m) => m.scope().clone(),
        other => panic!("esperado Module, obtido {other:?}"),
    };
    // 9 herdadas (abs/pow/sqrt/floor/ceil/round/min/max/clamp)
    // + 16 P283 (trig/hyperbolic/log/exp)
    // + 15 P306 (trunc/fract/even/odd/rem/rem-euclid/div-euclid/quo/
    //            gcd/lcm/fact/perm/binom/norm/root)
    // + 1  P308 (erf)
    // + 3  P501 (log10/deg/rad) = 44 funções.
    let n_funcs = dict
        .iter()
        .map(|(_, b)| b.value())
        .filter(|v| matches!(v, Value::Func(_)))
        .count();
    assert_eq!(n_funcs, 44, "esperava 44 funções calc, encontrei {n_funcs}");
    // + 4 constantes (pi/tau/e/inf).
    let n_floats = dict
        .iter()
        .map(|(_, b)| b.value())
        .filter(|v| matches!(v, Value::Float(_)))
        .count();
    assert_eq!(n_floats, 4, "esperava 4 constantes Float, encontrei {n_floats}");
}
