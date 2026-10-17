//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/trig.md
//! @prompt-hash b31107eb
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Aplica uma função matemática pura e envolve o resultado em [`guard_float`].
/// As famílias transcendentais usam a versão pinada de `libm`.
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
    unary_angle("calc.sin", args, libm::sin)
}

pub(crate) fn calc_cos(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_angle("calc.cos", args, libm::cos)
}

pub(crate) fn calc_tan(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_angle("calc.tan", args, libm::tan)
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
            angle_op(libm::asin, x)
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
            angle_op(libm::acos, x)
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
            angle_op(libm::atan, x)
        }
        _ => err(format!("calc.atan() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `calc.atan2(x, y)` — paridade vanilla na **ordem dos parâmetros** (`x` antes
/// de `y`); `libm` expõe `atan2(y, x)` portanto a chamada interna
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
            let r = libm::atan2(y, x);
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

    #[test]
    fn transcendental_angle_inputs_and_inverse_quadrants() {
        let Value::Float(sine) =
            call(calc_sin, vec![Value::Angle(Angle::deg(30.0))]).unwrap()
        else {
            panic!("sin(angle) must return float");
        };
        assert!((sine - 0.5).abs() < 1e-14);
        let Value::Angle(inverse) = call(calc_asin, vec![Value::Float(0.5)]).unwrap()
        else {
            panic!("asin must return angle");
        };
        assert!((inverse.to_deg() - 30.0).abs() < 1e-12);
        for (x, y, degrees) in
            [(1, 1, 45.0), (-1, 1, 135.0), (-1, -1, -135.0), (1, -1, -45.0)]
        {
            let Value::Angle(angle) =
                call(calc_atan2, vec![Value::Int(x), Value::Int(y)]).unwrap()
            else {
                panic!("atan2 must return angle");
            };
            assert!((angle.to_deg() - degrees).abs() < 1e-12);
        }
        assert!(call(calc_asin, vec![Value::Float(1.01)]).is_err());
        assert!(call(calc_acos, vec![Value::Float(-1.01)]).is_err());
        assert!(call(calc_sin, vec![]).is_err());
    }
}
