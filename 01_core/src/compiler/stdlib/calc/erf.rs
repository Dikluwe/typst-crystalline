//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/erf.md
//! @prompt-hash c49a5033
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ── P308 — função erro de Gauss (calc.erf) ───────────────────────────────
//
// Vanilla delega a `libm::erf`. Cristalino usa Caminho A — Abramowitz &
// Stegun 7.1.26, aproximação polinomial pura f64 com erro máximo 1.5e-7.
// Justificação no L0 §"Função erro (P308)" e em CLAUDE.md ADR-0054 graded.
// DEBT-libm partilhado com `calc_pow`/`trig_op`/`calc_root` (ADR-0018):
// futura migração agregada substitui o helper por `libm::erf`.

pub(crate) fn calc_erf(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.erf()")?;
            // NaN → Err (paridade convenção `guard_float`; divergência
            // consciente vs vanilla `libm::erf(NaN) = NaN`).
            if x.is_nan() {
                return err("calc.erf() valor é NaN");
            }
            // ±∞ → short-circuit ao limite analítico. Sem este desvio a
            // fórmula produziria `0 · ∞ = NaN` em `poly · exp(-x²)`.
            if x.is_infinite() {
                return Ok(Value::Float(if x > 0.0 { 1.0 } else { -1.0 }));
            }
            // ±0 → short-circuit ao limite exacto. A&S 7.1.26 produz
            // ~1e-9 (soma dos 5 coeficientes ≠ 1.0 bit-exact). Aqui
            // devolvemos ±0.0 preservando o sinal (paridade vanilla
            // `libm::erf(0) = 0`).
            if x == 0.0 {
                return Ok(Value::Float(x));
            }
            Ok(Value::Float(erf_approx_as(x)))
        }
        _ => err(format!("calc.erf() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Aproximação Abramowitz & Stegun 7.1.26 da função erro de Gauss.
///
/// Pré-condições (asseguradas pelo chamador): `x` é finito e não-NaN.
/// Erro máximo absoluto: 1.5e-7. Determinístico sob IEEE 754.
///
/// DEBT-libm: usa `f64::exp` directo (ADR-0018 — migração agregada futura
/// para `libm::erf` resolve este sítio juntamente com `pow`/trig/log).
#[allow(clippy::disallowed_methods)]
fn erf_approx_as(x: f64) -> f64 {
    const A1: f64 = 0.254_829_592;
    const A2: f64 = -0.284_496_736;
    const A3: f64 = 1.421_413_741;
    const A4: f64 = -1.453_152_027;
    const A5: f64 = 1.061_405_429;
    const P: f64 = 0.327_591_1;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x_abs = x.abs();
    let t = 1.0 / (1.0 + P * x_abs);
    // Horner do polinomial de grau 5 em t.
    let poly = t * (A1 + t * (A2 + t * (A3 + t * (A4 + t * A5))));
    let y = 1.0 - poly * f64::exp(-x_abs * x_abs);
    sign * y
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_erf, vec![Value::Float(-0.0)]).unwrap(), Value::Float(-0.0));
    }
}
