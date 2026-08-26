//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc.md
//! @prompt-hash dcbd453e
//! @layer L1
//! @updated 2026-05-20
//!
//! Módulo `calc` — operações matemáticas escalares.
//! P27: base aritmética (abs, pow, sqrt, floor, ceil, round, min, max, clamp).
//! P96.5: extraído de `stdlib.rs` conforme ADR-0037.
//! P283: trig/hiperbólicas/log/exp + constantes (pi, tau, e, inf).
//! P306: aritmética inteira, combinatória, norma, raiz (15 funções).
//! P308: função erro de Gauss (`erf`) — paridade calc 41/41 = 100%.
//! P817: paridade fina (achado #4 de P810) — `asin/acos/atan/atan2` →
//! `Angle` + `sin/cos/tan` aceitam `Angle`; `quo` floored; `pow` expoente
//! int negativo → float + guards vanilla; `decimal` em abs/pow/floor/ceil/
//! trunc/fract/round + erro dedicado decimal×float; tipos de `round`/
//! `fract`; `log` com despacho de base; `root(radicand, index)` ordem
//! vanilla. Decisão E: `log10/deg/rad` mantidos como extensão cristalina.

use crate::entities::file_id::FileId;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use super::{err, expect_no_named};

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::func::Func;
use crate::entities::layout_types::Angle;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Módulo calc (Passo 27) ───────────────────────────────────────────────────

/// Constrói o módulo `calc` como `Value::Module` com 41 funções + 4 constantes.
///
/// **P731** — passou de `Value::Dict` a `Value::Module` (paridade vanilla —
/// medido: `type(calc)` → `module`). A representação antiga bloqueava
/// `#import calc: min, max` (cetz `aabb.typ:18`). Semântica de acesso
/// (`calc.abs`) mantém-se via `eval_field_access` sobre Module (P679).
///
/// P283 estendeu o módulo com trig/hiperbólicas/log/exp + constantes
/// (`pi`, `tau`, `e`, `inf`); decisão libm vs f64 e tratamento de domínio
/// documentados em `diagnostico-calc-passo-283.md`.
///
/// P306 adicionou 15 funções de aritmética inteira, combinatória, norma e
/// raiz (`trunc`, `fract`, `even`, `odd`, `rem`, `rem-euclid`, `div-euclid`,
/// `quo`, `gcd`, `lcm`, `fact`, `perm`, `binom`, `norm`, `root`).
///
/// P308 fecha a paridade `calc` 41/41 = 100% com `erf` (função erro de
/// Gauss). Vanilla delega a `libm::erf`; cristalino usa Caminho A
/// (Abramowitz & Stegun 7.1.26, polinomial puro f64; erro máximo 1.5e-7
/// — ADR-0054 graded). Diagnóstico inline em `stdlib.md` §"Função erro".
pub fn make_calc_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("abs".into(), Value::Func(Func::native("calc.abs", calc_abs)));
    dict.insert("pow".into(), Value::Func(Func::native("calc.pow", calc_pow)));
    dict.insert("sqrt".into(), Value::Func(Func::native("calc.sqrt", calc_sqrt)));
    dict.insert("floor".into(), Value::Func(Func::native("calc.floor", calc_floor)));
    dict.insert("ceil".into(), Value::Func(Func::native("calc.ceil", calc_ceil)));
    dict.insert("round".into(), Value::Func(Func::native("calc.round", calc_round)));
    dict.insert("min".into(), Value::Func(Func::native("calc.min", calc_min)));
    dict.insert("max".into(), Value::Func(Func::native("calc.max", calc_max)));
    dict.insert("clamp".into(), Value::Func(Func::native("calc.clamp", calc_clamp)));
    // P283 — trig; P817-A — `sin/cos/tan` aceitam `Angle` (AngleLike vanilla)
    // e `asin/acos/atan/atan2` devolvem `Angle`.
    dict.insert("sin".into(), Value::Func(Func::native("calc.sin", calc_sin)));
    dict.insert("cos".into(), Value::Func(Func::native("calc.cos", calc_cos)));
    dict.insert("tan".into(), Value::Func(Func::native("calc.tan", calc_tan)));
    dict.insert("asin".into(), Value::Func(Func::native("calc.asin", calc_asin)));
    dict.insert("acos".into(), Value::Func(Func::native("calc.acos", calc_acos)));
    dict.insert("atan".into(), Value::Func(Func::native("calc.atan", calc_atan)));
    dict.insert("atan2".into(), Value::Func(Func::native("calc.atan2", calc_atan2)));
    // P283 — hiperbólicas.
    dict.insert("sinh".into(), Value::Func(Func::native("calc.sinh", calc_sinh)));
    dict.insert("cosh".into(), Value::Func(Func::native("calc.cosh", calc_cosh)));
    dict.insert("tanh".into(), Value::Func(Func::native("calc.tanh", calc_tanh)));
    dict.insert("asinh".into(), Value::Func(Func::native("calc.asinh", calc_asinh)));
    dict.insert("acosh".into(), Value::Func(Func::native("calc.acosh", calc_acosh)));
    dict.insert("atanh".into(), Value::Func(Func::native("calc.atanh", calc_atanh)));
    // P283 — exponencial / logaritmos.
    dict.insert("exp".into(), Value::Func(Func::native("calc.exp", calc_exp)));
    dict.insert("ln".into(), Value::Func(Func::native("calc.ln", calc_ln)));
    dict.insert("log".into(), Value::Func(Func::native("calc.log", calc_log)));
    dict.insert("log10".into(), Value::Func(Func::native("calc.log10", calc_log10)));
    // P501 — conversões deg/rad (extensão cristalina — P817-E: o vanilla
    // não expõe `calc.deg`/`calc.rad`/`calc.log10`; decisão: manter).
    dict.insert("deg".into(), Value::Func(Func::native("calc.deg", calc_deg)));
    dict.insert("rad".into(), Value::Func(Func::native("calc.rad", calc_rad)));
    // P306 — aritmética inteira, divisão e partes.
    dict.insert("trunc".into(), Value::Func(Func::native("calc.trunc", calc_trunc)));
    dict.insert("fract".into(), Value::Func(Func::native("calc.fract", calc_fract)));
    dict.insert("rem".into(), Value::Func(Func::native("calc.rem", calc_rem)));
    dict.insert(
        "rem-euclid".into(),
        Value::Func(Func::native("calc.rem-euclid", calc_rem_euclid)),
    );
    dict.insert(
        "div-euclid".into(),
        Value::Func(Func::native("calc.div-euclid", calc_div_euclid)),
    );
    dict.insert("quo".into(), Value::Func(Func::native("calc.quo", calc_quo)));
    // P306 — predicados inteiros.
    dict.insert("even".into(), Value::Func(Func::native("calc.even", calc_even)));
    dict.insert("odd".into(), Value::Func(Func::native("calc.odd", calc_odd)));
    // P306 — teoria dos números.
    dict.insert("gcd".into(), Value::Func(Func::native("calc.gcd", calc_gcd)));
    dict.insert("lcm".into(), Value::Func(Func::native("calc.lcm", calc_lcm)));
    // P306 — combinatória.
    dict.insert("fact".into(), Value::Func(Func::native("calc.fact", calc_fact)));
    dict.insert("perm".into(), Value::Func(Func::native("calc.perm", calc_perm)));
    dict.insert("binom".into(), Value::Func(Func::native("calc.binom", calc_binom)));
    // P306 — norma vectorial e raiz n-ésima.
    dict.insert("norm".into(), Value::Func(Func::native("calc.norm", calc_norm)));
    dict.insert("root".into(), Value::Func(Func::native("calc.root", calc_root)));
    // P308 — função erro de Gauss (paridade calc 41/41).
    dict.insert("erf".into(), Value::Func(Func::native("calc.erf", calc_erf)));
    // P283 — constantes ergonómicas (paridade vanilla).
    dict.insert("pi".into(), Value::Float(std::f64::consts::PI));
    dict.insert("tau".into(), Value::Float(std::f64::consts::TAU));
    dict.insert("e".into(), Value::Float(std::f64::consts::E));
    dict.insert("inf".into(), Value::Float(f64::INFINITY));
    // P731 — `Value::Module` (paridade vanilla), não `Value::Dict`.
    let mut scope = crate::entities::scope::Scope::new();
    for (name, value) in dict {
        scope.define(name.as_str(), value);
    }
    Value::Module(crate::entities::module::Module::new("calc", scope))
}

pub(crate) fn calc_abs(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(i.saturating_abs())),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        // P817-D — paridade vanilla `calc.abs(decimal)` → decimal.
        [Value::Decimal(d)] => Ok(Value::Decimal(d.abs())),
        [other] => {
            err(format!("calc.abs() requer Int ou Float, recebeu {}", other.type_name()))
        }
        _ => err(format!("calc.abs() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// P817-C/D — paridade vanilla (`calc.rs:108-156`):
/// - `0^0` → Err ("zero to the power of zero is undefined");
/// - expoente `Int` que não cabe em `i32` → Err ("exponent is too large");
/// - expoente `Float` não-normal (inf/subnormal/NaN) → Err;
/// - `(Int, Int≥0)` → `Int` (`checked_pow`; overflow → Err "too large");
/// - `(Int, Int<0)` → `Float` via `powi` (`calc.pow(2, -1) = 0.5`);
/// - `(Decimal, Int)` → `Decimal` via `checked_powi`;
/// - `(Decimal, Float)` → erro dedicado decimal×float + hint (verbatim vanilla);
/// - resto → `Float` via `powf` (`guard_float`, ADR-0101: inf/NaN → Err).
pub(crate) fn calc_pow(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let [base, exp] = args.items.as_slice() else {
        return err(format!(
            "calc.pow() requer 2 argumentos, recebeu {}",
            args.items.len()
        ));
    };

    let base_zero = match base {
        Value::Int(i) => *i == 0,
        Value::Float(f) => *f == 0.0,
        Value::Decimal(d) => d.0.is_zero(),
        _ => false, // neutro: N16[β] — tipos não numéricos retornam false no predicado is_zero
    };
    let exp_zero = match exp {
        Value::Int(i) => *i == 0,
        Value::Float(f) => *f == 0.0,
        _ => false, // neutro: N16[β] — tipos não numéricos retornam false no predicado is_zero
    };
    if base_zero && exp_zero {
        return err("calc.pow() zero elevado a zero é indefinido");
    }
    if let Value::Int(i) = exp {
        if i32::try_from(*i).is_err() {
            return err("calc.pow() expoente demasiado grande");
        }
    }
    if let Value::Float(f) = exp {
        if !f.is_normal() && *f != 0.0 {
            return err("calc.pow() expoente não pode ser infinito, subnormal ou NaN");
        }
    }

    match (base, exp) {
        (Value::Int(a), Value::Int(b)) if *b >= 0 => {
            a.checked_pow(*b as u32).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.pow() resultado fora do alcance i64".to_string(),
                )]
            })
        }
        (Value::Int(a), Value::Int(b)) => {
            // Expoente inteiro negativo → Float (paridade vanilla `powi`).
            #[allow(clippy::disallowed_methods)]
            guard_float((*a as f64).powi(*b as i32))
        }
        (Value::Decimal(d), Value::Int(b)) => d.checked_powi(*b).map(Value::Decimal).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.pow() resultado fora do alcance decimal".to_string(),
            )]
        }),
        (Value::Decimal(_), Value::Float(_)) => Err(vec![SourceDiagnostic::error(
            args.span,
            "cannot apply this operation to a decimal and a float".to_string(),
        )
        .with_hint(
            "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
        )]),
        _ => {
            let b = coerce_to_f64(base, "calc.pow() base")?;
            let e = coerce_to_f64(exp, "calc.pow() expoente")?;
            // DEBT: migrar para libm::pow quando libm for dependência do workspace (ADR-0018)
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e))
        }
    }
}

pub(crate) fn calc_sqrt(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let f = coerce_to_f64(v, "calc.sqrt()")?;
            if f < 0.0 {
                return err("calc.sqrt() argumento negativo");
            }
            guard_float(f.sqrt())
        }
        _ => err(format!("calc.sqrt() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_floor(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.floor() as i64)),
        // P817-D — paridade vanilla `calc.floor(decimal)` → int.
        [Value::Decimal(d)] => d.floor().to_i64().map(Value::Int).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.floor() resultado fora do alcance i64".to_string(),
            )]
        }),
        [other] => err(format!(
            "calc.floor() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.floor() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_ceil(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.ceil() as i64)),
        // P817-D — paridade vanilla `calc.ceil(decimal)` → int.
        [Value::Decimal(d)] => d.ceil().to_i64().map(Value::Int).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.ceil() resultado fora do alcance i64".to_string(),
            )]
        }),
        [other] => {
            err(format!("calc.ceil() requer Int ou Float, recebeu {}", other.type_name()))
        }
        _ => err(format!("calc.ceil() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_round(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P491 — arg nomeado `digits` (default 0). Apenas `digits` é aceite como named arg.
    let digits: i32 = match args.named.get("digits") {
        Some(v) => match v {
            Value::Int(i) => *i as i32,
            other => {
                return err(format!(
                    "calc.round() argumento 'digits' requer Int, recebeu {}",
                    other.type_name()
                ))
            }
        },
        None => 0,
    };
    if args.named.len() > 1
        || (args.named.len() == 1 && !args.named.contains_key("digits"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "digits")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("calc.round() argumento nomeado desconhecido: '{bad}'"));
    }

    match args.items.as_slice() {
        // P817-D — paridade vanilla: `round(Int)` → `Int`. `digits` > 0 é
        // no-op; `digits` < 0 arredonda antes do ponto (away from zero).
        [Value::Int(i)] => {
            if digits >= 0 {
                Ok(Value::Int(*i))
            } else {
                round_int_com_precisao(*i, digits).map(Value::Int).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.round() resultado fora do alcance i64".to_string(),
                    )]
                })
            }
        }
        // P817-D — paridade vanilla: `round(Float)` → `Float` (não `Int`).
        [Value::Float(f)] => {
            if digits == 0 {
                Ok(Value::Float(f.round()))
            } else if digits >= 15 {
                // Paridade vanilla `round_with_precision`: precisão ≥ 15 é no-op.
                Ok(Value::Float(*f))
            } else if digits < -308 {
                // Além de qualquer dígito representável → zero com sinal.
                Ok(Value::Float(*f * 0.0))
            } else {
                let factor = 10f64.powi(digits);
                #[allow(clippy::disallowed_methods)]
                guard_float((f * factor).round() / factor)
            }
        }
        // P817-D — paridade vanilla: `round(Decimal)` → `Decimal`
        // (midpoint away from zero).
        [Value::Decimal(d)] => {
            d.round_with_digits(digits).map(Value::Decimal).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.round() resultado fora do alcance decimal".to_string(),
                )]
            })
        }
        [other] => err(format!(
            "calc.round() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.round() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

/// Port de `round_int_with_precision` (vanilla `typst-utils/round.rs:82`):
/// arredonda um inteiro `digits` casas **antes** do ponto decimal
/// (`digits < 0`), away from zero. `None` em overflow.
fn round_int_com_precisao(value: i64, digits: i32) -> Option<i64> {
    if digits >= 0 {
        return Some(value);
    }
    let n = u32::try_from(digits.checked_neg()?).ok()?;
    let Some(dez) = 10i64.checked_pow(n.checked_sub(1)?) else {
        // Mais dígitos do que qualquer inteiro representável.
        return Some(0);
    };
    let truncado = value / dez;
    if truncado == 0 {
        return Some(0);
    }
    let arredondado = if (truncado % 10).abs() >= 5 {
        truncado.checked_add(truncado.signum() * (10 - (truncado % 10).abs()))?
    } else {
        truncado - (truncado % 10)
    };
    arredondado.checked_mul(dez)
}

pub(crate) fn calc_min(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.min() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a), Value::Int(b)) => Value::Int(*a.min(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).min(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.min(*b as f64)),
            (_, other) => {
                return err(format!(
                    "calc.min() tipos incompatíveis: {} e {}",
                    result.type_name(),
                    other.type_name()
                ))
            }
        };
    }
    Ok(result)
}

pub(crate) fn calc_max(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.max() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a), Value::Int(b)) => Value::Int(*a.max(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).max(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.max(*b as f64)),
            (_, other) => {
                return err(format!(
                    "calc.max() tipos incompatíveis: {} e {}",
                    result.type_name(),
                    other.type_name()
                ))
            }
        };
    }
    Ok(result)
}

pub(crate) fn calc_clamp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(v), Value::Int(lo), Value::Int(hi)] => {
            Ok(Value::Int((*v).clamp(*lo, *hi)))
        }
        [v, lo, hi] => {
            let vf = coerce_to_f64(v, "calc.clamp() value")?;
            let lof = coerce_to_f64(lo, "calc.clamp() min")?;
            let hif = coerce_to_f64(hi, "calc.clamp() max")?;
            if lof > hif {
                return err(format!("calc.clamp() min ({lof}) > max ({hif})"));
            }
            Ok(Value::Float(vf.clamp(lof, hif)))
        }
        _ => {
            err(format!("calc.clamp() requer 3 argumentos, recebeu {}", args.items.len()))
        }
    }
}

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
fn trig_op(op: fn(f64) -> f64, x: f64) -> SourceResult<Value> {
    guard_float(op(x))
}

/// Captura o padrão `expect_no_named + 1 arg + coerce_to_f64 + trig_op` que
/// se repetiria 13 vezes nas funções trig/hiperbólicas/exp.
fn unary_f64(name: &str, args: &Args, op: fn(f64) -> f64) -> SourceResult<Value> {
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

pub(crate) fn calc_sinh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.sinh", args, f64::sinh)
}

pub(crate) fn calc_cosh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.cosh", args, f64::cosh)
}

pub(crate) fn calc_tanh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.tanh", args, f64::tanh)
}

pub(crate) fn calc_asinh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.asinh", args, f64::asinh)
}

pub(crate) fn calc_acosh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.acosh()")?;
            if x < 1.0 {
                return err(format!("calc.acosh() valor deve ser >= 1, recebeu {x}"));
            }
            trig_op(f64::acosh, x)
        }
        _ => {
            err(format!("calc.acosh() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_atanh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.atanh()")?;
            if x <= -1.0 || x >= 1.0 {
                return err(format!(
                    "calc.atanh() valor deve estar em (-1, 1), recebeu {x}"
                ));
            }
            trig_op(f64::atanh, x)
        }
        _ => {
            err(format!("calc.atanh() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_exp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.exp", args, f64::exp)
}

pub(crate) fn calc_ln(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.ln()")?;
            if x <= 0.0 {
                return err(format!(
                    "calc.ln() valor deve ser estritamente positivo, recebeu {x}"
                ));
            }
            #[allow(clippy::disallowed_methods)]
            let r = f64::ln(x);
            guard_float(r)
        }
        _ => err(format!("calc.ln() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `calc.log(x)` (base 10), `calc.log(x, base)` ou `calc.log(x, base: base)`.
/// P491: suporte ao arg nomeado `base:` (default 10), preservando a forma
/// posicional legada.
pub(crate) fn calc_log(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P491 — arg nomeado `base` (default 10). Apenas `base` é aceite como named arg.
    let base_from_named = args
        .named
        .get("base")
        .map(|v| coerce_to_f64(v, "calc.log() base"))
        .transpose()?;
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("base"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "base")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("calc.log() argumento nomeado desconhecido: '{bad}'"));
    }

    let (x, base) = match args.items.as_slice() {
        [v] => {
            (coerce_to_f64(v, "calc.log() valor")?, base_from_named.unwrap_or(10.0_f64))
        }
        [v, b] => {
            if base_from_named.is_some() {
                return err(
                    "calc.log() não pode especificar base tanto posicional como nomeado"
                        .to_string(),
                );
            }
            (coerce_to_f64(v, "calc.log() valor")?, coerce_to_f64(b, "calc.log() base")?)
        }
        _ => {
            return err(format!(
                "calc.log() requer 1 ou 2 argumentos, recebeu {}",
                args.items.len()
            ))
        }
    };
    if x <= 0.0 {
        return err(format!(
            "calc.log() valor deve ser estritamente positivo, recebeu {x}"
        ));
    }
    let base = base_from_named.unwrap_or(base);
    if !base.is_finite() || base <= 0.0 || base == 1.0 {
        return err(format!("calc.log() base inválida: {base}"));
    }
    // P817-F — paridade vanilla (`calc.rs:506-515`): despacho por base
    // exacta. `ln(x)/ln(10)` dava `log(1000) = 2.9999999999999996`; o
    // vanilla despacha base 10 para `libm::log10` (resultado `3`).
    #[allow(clippy::disallowed_methods)]
    let r = if base == std::f64::consts::E {
        f64::ln(x)
    } else if base == 2.0 {
        f64::log2(x)
    } else if base == 10.0 {
        f64::log10(x)
    } else {
        f64::ln(x) / f64::ln(base)
    };
    guard_float(r)
}

/// `calc.log10(x)` — logaritmo base 10.
pub(crate) fn calc_log10(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.log10()")?;
            if x <= 0.0 {
                return err(format!(
                    "calc.log10() valor deve ser estritamente positivo, recebeu {x}"
                ));
            }
            #[allow(clippy::disallowed_methods)]
            guard_float(f64::log10(x))
        }
        _ => {
            err(format!("calc.log10() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

/// `calc.deg(rad)` — converte radianos para graus.
pub(crate) fn calc_deg(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.deg", args, f64::to_degrees)
}

/// `calc.rad(deg)` — converte graus para radianos.
pub(crate) fn calc_rad(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.rad", args, f64::to_radians)
}

// ── P306 — aritmética inteira, combinatória, norma, raiz ────────────────────
//
// Funções puramente aritméticas em `std::*` (`i64::checked_*`,
// `f64::rem_euclid`, etc.). `calc_norm`/`calc_root` usam `f64::powf` —
// DEBT-libm partilhado com `calc_pow` (ADR-0018).

pub(crate) fn calc_trunc(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.trunc() as i64)),
        // P817-D — paridade vanilla `calc.trunc(decimal)` → int.
        [Value::Decimal(d)] => d.trunc().to_i64().map(Value::Int).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.trunc() resultado fora do alcance i64".to_string(),
            )]
        }),
        [other] => err(format!(
            "calc.trunc() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.trunc() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_fract(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        // P817-D — paridade vanilla: `calc.fract(int)` → int `0` (não `0.0`).
        [Value::Int(_)] => Ok(Value::Int(0)),
        [Value::Float(f)] => Ok(Value::Float(f.fract())),
        // P817-D — paridade vanilla `calc.fract(decimal)` → decimal.
        [Value::Decimal(d)] => Ok(Value::Decimal(d.fract())),
        [other] => err(format!(
            "calc.fract() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.fract() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_even(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => Ok(Value::Bool(n % 2 == 0)),
        [other] => err(format!("calc.even() requer Int, recebeu {}", other.type_name())),
        _ => err(format!("calc.even() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_odd(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => Ok(Value::Bool(n % 2 != 0)),
        [other] => err(format!("calc.odd() requer Int, recebeu {}", other.type_name())),
        _ => err(format!("calc.odd() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Resto truncado: sinal acompanha o dividendo (paridade operador `%`).
pub(crate) fn calc_rem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.rem() divisão por zero");
            }
            // `checked_rem` evita panic em `i64::MIN % -1` (overflow).
            a.checked_rem(*b).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.rem() resto fora do alcance i64".to_string(),
                )]
            })
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.rem() dividendo")?;
            let bf = coerce_to_f64(b, "calc.rem() divisor")?;
            if bf == 0.0 {
                return err("calc.rem() divisão por zero");
            }
            Ok(Value::Float(af % bf))
        }
        _ => err(format!("calc.rem() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

/// Resto Euclidiano: sempre ≥ 0 para divisor > 0.
pub(crate) fn calc_rem_euclid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.rem-euclid() divisão por zero");
            }
            a.checked_rem_euclid(*b).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.rem-euclid() resto fora do alcance i64".to_string(),
                )]
            })
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.rem-euclid() dividendo")?;
            let bf = coerce_to_f64(b, "calc.rem-euclid() divisor")?;
            if bf == 0.0 {
                return err("calc.rem-euclid() divisão por zero");
            }
            Ok(Value::Float(af.rem_euclid(bf)))
        }
        _ => err(format!(
            "calc.rem-euclid() requer 2 argumentos, recebeu {}",
            args.items.len()
        )),
    }
}

/// Quociente Euclidiano: arredonda para -∞ quando divisor > 0.
pub(crate) fn calc_div_euclid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.div-euclid() divisão por zero");
            }
            a.checked_div_euclid(*b).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.div-euclid() quociente fora do alcance i64".to_string(),
                )]
            })
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.div-euclid() dividendo")?;
            let bf = coerce_to_f64(b, "calc.div-euclid() divisor")?;
            if bf == 0.0 {
                return err("calc.div-euclid() divisão por zero");
            }
            Ok(Value::Float(af.div_euclid(bf)))
        }
        _ => err(format!(
            "calc.div-euclid() requer 2 argumentos, recebeu {}",
            args.items.len()
        )),
    }
}

/// Quociente **floored** em Int (paridade vanilla `calc.quo(-7, 2) = -4`).
///
/// P817-B — o cristalino truncava em direcção a zero (`-3`); o vanilla
/// arredonda para -∞ (`calc.rs:1140-1177`: divisão com ajuste quando os
/// sinais diferem e há resto, `floor` no caminho float). O resultado é
/// sempre `int` (vanilla: `SourceResult<i64>`).
pub(crate) fn calc_quo(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.quo() divisão por zero");
            }
            let q = a.checked_div(*b).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.quo() quociente fora do alcance i64".to_string(),
                )]
            })?;
            // Arredonda para -∞ quando a fracção é negativa (paridade vanilla).
            let floored = if (*a < 0) != (*b < 0) && a % b != 0 { q - 1 } else { q };
            Ok(Value::Int(floored))
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.quo() dividendo")?;
            let bf = coerce_to_f64(b, "calc.quo() divisor")?;
            if bf == 0.0 {
                return err("calc.quo() divisão por zero");
            }
            let q = (af / bf).floor();
            if !q.is_finite()
                || q < -9_223_372_036_854_775_808.0
                || q >= 9_223_372_036_854_775_808.0
            {
                return err("calc.quo() resultado fora do alcance i64");
            }
            Ok(Value::Int(q as i64))
        }
        _ => err(format!("calc.quo() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

/// Algoritmo de Euclides iterativo sobre valores absolutos.
fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub(crate) fn calc_gcd(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => Ok(Value::Int(gcd_impl(*a, *b))),
        [_, _] => err("calc.gcd() requer 2 inteiros"),
        _ => err(format!("calc.gcd() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_lcm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *a == 0 || *b == 0 {
                return Ok(Value::Int(0));
            }
            let g = gcd_impl(*a, *b);
            // Dividir antes de multiplicar evita overflow intermédio.
            let aa = a.checked_abs().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.lcm() valor fora do alcance i64".to_string(),
                )]
            })?;
            let bb = b.checked_abs().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.lcm() valor fora do alcance i64".to_string(),
                )]
            })?;
            (aa / g).checked_mul(bb).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.lcm() resultado fora do alcance i64".to_string(),
                )]
            })
        }
        [_, _] => err("calc.lcm() requer 2 inteiros"),
        _ => err(format!("calc.lcm() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_fact(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => {
            if *n < 0 {
                return err("calc.fact() factorial de número negativo");
            }
            let mut acc: i64 = 1;
            for i in 1..=*n {
                acc = acc.checked_mul(i).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.fact() resultado fora do alcance i64".to_string(),
                    )]
                })?;
            }
            Ok(Value::Int(acc))
        }
        [other] => err(format!("calc.fact() requer Int, recebeu {}", other.type_name())),
        _ => err(format!("calc.fact() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Arranjos: `P(n, k) = n * (n-1) * ... * (n-k+1)`.
pub(crate) fn calc_perm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n), Value::Int(k)] => {
            if *n < 0 || *k < 0 {
                return err("calc.perm() argumento negativo");
            }
            if *k > *n {
                return Ok(Value::Int(0));
            }
            let mut acc: i64 = 1;
            for i in 0..*k {
                acc = acc.checked_mul(n - i).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.perm() resultado fora do alcance i64".to_string(),
                    )]
                })?;
            }
            Ok(Value::Int(acc))
        }
        [_, _] => err("calc.perm() requer 2 inteiros"),
        _ => {
            err(format!("calc.perm() requer 2 argumentos, recebeu {}", args.items.len()))
        }
    }
}

/// Combinações: algoritmo iterativo com divisão exacta a cada passo.
///
/// `C(n, k) = ∏_{i=0..k} (n - i) / (i + 1)`. A divisão exacta a cada
/// iteração é garantida pela propriedade combinatória: o produto parcial
/// após `i` passos é divisível por `(i + 1)`.
pub(crate) fn calc_binom(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n), Value::Int(k)] => {
            if *n < 0 || *k < 0 {
                return err("calc.binom() argumento negativo");
            }
            if *k > *n {
                return Ok(Value::Int(0));
            }
            // Simetria: C(n, k) = C(n, n-k). Reduz iterações.
            let k_eff = (*k).min(*n - *k);
            let mut acc: i64 = 1;
            for i in 0..k_eff {
                acc = acc.checked_mul(n - i).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.binom() resultado fora do alcance i64".to_string(),
                    )]
                })?;
                acc /= i + 1; // divisão exacta garantida pela propriedade binomial.
            }
            Ok(Value::Int(acc))
        }
        [_, _] => err("calc.binom() requer 2 inteiros"),
        _ => {
            err(format!("calc.binom() requer 2 argumentos, recebeu {}", args.items.len()))
        }
    }
}

/// Norma p de um vector: `(Σ |x_i|^p)^(1/p)`. `p` é named arg, default `2.0`.
pub(crate) fn calc_norm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // `p` é o único named arg aceite.
    let p = match args.named.get("p") {
        Some(Value::Float(f)) => *f,
        Some(Value::Int(i)) => *i as f64,
        Some(other) => {
            return err(format!(
                "calc.norm() argumento 'p' requer Int ou Float, recebeu {}",
                other.type_name(),
            ))
        }
        None => 2.0,
    };
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("p")) {
        // Detecta outros named args para erro claro.
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "p")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("calc.norm() argumento nomeado desconhecido: '{bad}'"));
    }

    if args.items.is_empty() {
        return Ok(Value::Float(0.0));
    }
    let mut sum: f64 = 0.0;
    for v in &args.items {
        let x = coerce_to_f64(v, "calc.norm() valor")?;
        // DEBT-libm: f64::powf banido — partilhado com calc_pow / calc_root.
        #[allow(clippy::disallowed_methods)]
        let term = x.abs().powf(p);
        sum += term;
    }
    #[allow(clippy::disallowed_methods)]
    let r = sum.powf(1.0 / p);
    guard_float(r)
}

/// Raiz n-ésima: `root(radicand, index)` — **ordem vanilla** (radicando
/// primeiro, índice depois; medido em P817: vanilla `calc.root(27.0, 3) = 3`,
/// `calc.root(-8, 3) = -2`). Preserva sinal para `index` ímpar com radicando
/// negativo; índice negativo equivale a `x^(1/index)` (float).
pub(crate) fn calc_root(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [x, Value::Int(index)] => {
            if *index == 0 {
                return err("calc.root() índice de raiz zero");
            }
            let xf = coerce_to_f64(x, "calc.root() valor")?;
            let inv = 1.0 / (*index as f64);
            let r = if xf < 0.0 {
                if index.rem_euclid(2) == 0 {
                    return err("calc.root() raiz par de número negativo");
                }
                // DEBT-libm: f64::powf banido — partilhado com calc_pow / calc_norm.
                #[allow(clippy::disallowed_methods)]
                let v = (-xf).powf(inv);
                -v
            } else {
                #[allow(clippy::disallowed_methods)]
                let v = xf.powf(inv);
                v
            };
            guard_float(r)
        }
        [_, other] => {
            err(format!("calc.root() índice deve ser Int, recebeu {}", other.type_name()))
        }
        _ => {
            err(format!("calc.root() requer 2 argumentos, recebeu {}", args.items.len()))
        }
    }
}

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

fn coerce_to_f64(v: &Value, ctx: &str) -> SourceResult<f64> {
    match v {
        Value::Int(i) => Ok(*i as f64),
        Value::Float(f) => Ok(*f),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{ctx}: esperava Int ou Float, recebeu {}", other.type_name()),
        )]),
    }
}

fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan() {
        err("resultado não é um número (NaN)")
    } else if f.is_infinite() {
        err("resultado é infinito")
    } else {
        Ok(Value::Float(f))
    }
}

// ── `upper()` / `lower()` / `replace()` — motor map_text (Passo 67) ─────────
