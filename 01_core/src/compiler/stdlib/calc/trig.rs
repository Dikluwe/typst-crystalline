//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/trig.md
//! @prompt-hash 62f77b89
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ── P283 — trig, hiperbólicas, log/exp ──────────────────────────────────────
//
// Decisão de A.2 (ver diagnostico-calc-passo-283.md): usar `f64::*` agora com
// helper `trig_op` que centraliza `#[allow(clippy::disallowed_methods)]`. A
// migração futura a `libm` (ADR-0018 §DEBT) toca 4 sítios (este helper +
// `calc_atan2` + `calc_ln` + `calc_log`) em vez de 17.

/// Aplica uma função `f64 -> f64` da família matemática vanilla e envolve o
/// resultado em [`guard_float`]. Único ponto que tolera os métodos `f64::*`
/// banidos por `clippy.toml`.
#[allow(clippy::disallowed_methods)]
pub(super) fn trig_op(op: fn(f64) -> f64, x: f64) -> SourceResult<Value> {
    guard_float(op(x))
}

/// Captura o padrão `expect_no_named + 1 arg + coerce_to_f64 + trig_op` que
/// se repetiria 13 vezes nas funções trig/hiperbólicas/exp.
pub(super) fn unary_f64(
    name: &str,
    args: &Args,
    op: fn(f64) -> f64,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, name)?;
            trig_op(op, x)
        }
        _ => err(format!("{name}() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// P817-A — aplica `op` e envolve o resultado em [`Value::Angle`] (paridade
/// vanilla: `asin`/`acos`/`atan` devolvem `angle`). Resultado NaN não é
/// alcançável: `asin`/`acos` têm checagem de domínio e `atan` é finito.
#[allow(clippy::disallowed_methods)]
fn angle_op(op: fn(f64) -> f64, x: f64) -> SourceResult<Value> {
    Ok(Value::Angle(Angle::rad(op(x))))
}

/// P817-A — paridade vanilla `AngleLike`: `sin`/`cos`/`tan` aceitam `angle`
/// (além de `int`/`float`, interpretados como radianos). Sem isto,
/// `calc.sin(calc.asin(0.5))` falharia — composabilidade da língua.
fn unary_angle(name: &str, args: &Args, op: fn(f64) -> f64) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = match v {
                Value::Angle(a) => a.to_rad(),
                other => coerce_to_f64(other, name)?,
            };
            trig_op(op, x)
        }
        _ => err(format!("{name}() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_sin(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_angle("calc.sin", args, f64::sin)
}

pub(crate) fn calc_cos(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_angle("calc.cos", args, f64::cos)
}

pub(crate) fn calc_tan(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_angle("calc.tan", args, f64::tan)
}

/// P817-A — paridade vanilla: `asin` devolve `Angle` (não `float`).
/// Checagem de domínio mantida (vanilla: "value must be between -1 and 1").
pub(crate) fn calc_asin(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.asin()")?;
            if !(-1.0..=1.0).contains(&x) {
                return err(format!(
                    "calc.asin() valor deve estar entre -1 e 1, recebeu {x}"
                ));
            }
            angle_op(f64::asin, x)
        }
        _ => err(format!("calc.asin() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// P817-A — paridade vanilla: `acos` devolve `Angle` (não `float`).
pub(crate) fn calc_acos(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.acos()")?;
            if !(-1.0..=1.0).contains(&x) {
                return err(format!(
                    "calc.acos() valor deve estar entre -1 e 1, recebeu {x}"
                ));
            }
            angle_op(f64::acos, x)
        }
        _ => err(format!("calc.acos() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// P817-A — paridade vanilla: `atan` devolve `Angle` (não `float`).
pub(crate) fn calc_atan(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.atan()")?;
            angle_op(f64::atan, x)
        }
        _ => err(format!("calc.atan() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `calc.atan2(x, y)` — paridade vanilla na **ordem dos parâmetros** (`x` antes
/// de `y`); a stdlib Rust expõe `f64::atan2(y, x)` portanto a chamada interna
/// passa-os trocados (cf. diagnóstico §A.1).
/// P817-A — paridade vanilla: `atan2` devolve `Angle` (não `float`).
pub(crate) fn calc_atan2(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [vx, vy] => {
            let x = coerce_to_f64(vx, "calc.atan2() x")?;
            let y = coerce_to_f64(vy, "calc.atan2() y")?;
            #[allow(clippy::disallowed_methods)]
            let r = f64::atan2(y, x);
            Ok(Value::Angle(Angle::rad(r)))
        }
        _ => {
            err(format!("calc.atan2() requer 2 argumentos, recebeu {}", args.items.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_sin, vec![Value::Int(0)]).unwrap(), Value::Float(0.0));
    }
}
