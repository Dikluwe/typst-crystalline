//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc.md
//! @prompt-hash ac26a9a0
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
use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
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
    dict.insert("abs".into(), Value::Func(Func::native("abs", calc_abs)));
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
        [Value::Int(i)] => i.checked_abs().map(Value::Int).ok_or_else(|| {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| occurrences.iter().find(|arg| arg.name.is_none()))
                .map(|arg| arg.value_span)
                .unwrap_or_else(Span::detached);
            vec![SourceDiagnostic::error(span, "the result is too large")]
        }),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        // P817-D — paridade vanilla `calc.abs(decimal)` → decimal.
        [Value::Decimal(d)] => Ok(Value::Decimal(d.abs())),
        [Value::Length(v)] => {
            if v.abs.is_zero() || v.em == 0.0 {
                Ok(Value::Length(Length {
                    abs: Abs::pt(v.abs.to_pt().abs()),
                    em: v.em.abs(),
                }))
            } else {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|occurrences| {
                        occurrences.iter().find(|arg| arg.name.is_none())
                    })
                    .map(|arg| arg.value_span)
                    .unwrap_or_else(Span::detached);
                Err(vec![SourceDiagnostic::error(
                    span,
                    "cannot take absolute value of this length",
                )])
            }
        }
        [Value::Angle(v)] => Ok(Value::Angle(Angle::rad(v.to_rad().abs()))),
        [Value::Ratio(v)] => Ok(Value::Ratio(Ratio(v.get().abs()))),
        [Value::Fraction(v)] => Ok(Value::Fraction(v.abs())),
        [Value::Content(_) | Value::LocatedContent(_, _)] => {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| occurrences.iter().find(|arg| arg.name.is_none()))
                .map(|arg| arg.value_span)
                .unwrap_or_else(Span::detached);
            Err(vec![SourceDiagnostic::error(
                span,
                "expected integer, float, length, angle, ratio, fraction, or decimal, found content",
            )])
        }
        [other] => {
            let found = match other {
                Value::Str(_) => "string",
                Value::Bool(_) => "boolean",
                _ => other.type_name(),
            };
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| occurrences.iter().find(|arg| arg.name.is_none()))
                .map(|arg| arg.value_span)
                .unwrap_or_else(Span::detached);
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected integer, float, length, angle, ratio, fraction, or decimal, found {found}"
                ),
            )])
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

#[cfg(test)]
mod p1328_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::content::Content;
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{Angle, Length, Ratio};
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::{IntrospectedContent, Value};
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.source,
            &ElementRegistry::new(),
            false,
            EvalTarget::Paged,
            features,
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }
    fn content_values() -> Vec<Value> {
        let c = Content::strong(Content::text("-1"));
        vec![
            Value::Content(c.clone()),
            Value::LocatedContent(
                IntrospectedContent::new(c, None),
                Locator::new().next(),
            ),
        ]
    }
    #[test]
    fn p1328_native_content_both_representations_value_span_only() {
        let source = Source::detached("aggregated occurrence value other");
        let aggregate = Span::from_range(source.id(), 0..32);
        let occurrence = Span::from_range(source.id(), 11..21);
        let origin = Span::from_range(source.id(), 22..27);
        for features in profiles() {
            for value in content_values() {
                assert_eq!(value.type_name(), "content");
                let args = Args::from_occurrences(
                    aggregate,
                    vec![ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: origin,
                    }],
                );
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    origin,
                );
            }
        }
    }
    #[test]
    fn p1328_native_missing_or_detached_origin_is_not_fabricated() {
        let source = Source::detached("aggregate occurrence");
        let aggregate = Span::from_range(source.id(), 0..20);
        let occurrence = Span::from_range(source.id(), 10..20);
        for features in profiles() {
            for value in content_values() {
                let args =
                    Args::from_parts(vec![value.clone()], Default::default(), aggregate);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    Span::detached(),
                );
                let args = Args::from_occurrences(
                    aggregate,
                    vec![ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: Span::detached(),
                    }],
                );
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    Span::detached(),
                );
            }
        }
    }
    #[test]
    fn p1328_native_guards_before_content_and_numeric_controls() {
        for features in profiles() {
            let value = Value::Content(Content::empty());
            let mut args = Args::positional(vec![value.clone(), value.clone()]);
            args.named.insert("unexpected".into(), Value::Int(1));
            bare_diagnostic(
                &native(&args, features).unwrap_err(),
                "argumento nomeado inesperado: 'unexpected'",
                Span::detached(),
            );
            for count in [0, 2] {
                let args = Args::positional(vec![value.clone(); count]);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    &format!("calc.abs() requer 1 argumento, recebeu {count}"),
                    Span::detached(),
                );
            }
            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features)
                    .unwrap_err(),
                "the result is too large",
                Span::detached(),
            );
            for (input, expected) in [
                (Value::Int(-19), Value::Int(19)),
                (Value::Float(-1.25), Value::Float(1.25)),
                (
                    Value::Decimal(Decimal::new(-125, 2)),
                    Value::Decimal(Decimal::new(125, 2)),
                ),
            ] {
                assert_eq!(
                    native(&Args::positional(vec![input]), features).unwrap(),
                    expected
                );
            }
        }
    }
    fn public_error(text: &str, anchor: &str, call_trace: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, CONTENT_ERROR, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}: {d:?}");
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}: {d:?}"
            );
            assert_eq!(d.trace.len(), usize::from(call_trace.is_some()), "{text}: {d:?}");
            if let Some(call) = call_trace {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                // Dispatcher name is a measured pre-existing debt; native adds no trace.
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }
    #[test]
    fn p1328_public_math_markup_alias_origins_utf8_all_profiles() {
        for (text, anchor) in [
            ("#calc.abs([])", "[]"),
            ("#calc.abs([*bold*])", "[*bold*]"),
            ("$std.calc.abs(-1)$", "-1"),
            ("#let ab = calc.abs\n$ab(-1)$", "-1"),
            ("#let ab = calc.abs\n#ab([x])", "[x]"),
            ("#let ab = calc.abs.with()\n#ab([x])", "[x]"),
            ("#let x = [x]\n#let y = [x]\n#calc.abs(y)", "y"),
            ("#let café = [á]\n#calc.abs(\n café\n)", "café"),
            ("texto á\n#calc.abs([β])", "[β]"),
            ("#calc.abs(..([x],))", "..([x],)"),
        ] {
            public_error(text, anchor, None);
        }
    }
    #[test]
    fn p1328_public_with_and_argument_spread_keep_prebound_origin() {
        for (text, anchor, call) in [
            ("#let ab = calc.abs.with([x])\n#ab()", "[x]", "ab()"),
            ("#let ab = calc.abs.with([x]).with()\n#ab()", "[x]", "ab()"),
            ("#let aa = arguments([x])\n#calc.abs(..aa)", "[x]", "calc.abs(..aa)"),
        ] {
            public_error(text, anchor, Some(call));
        }
    }
    #[test]
    fn p1328_public_warning_survives_error_all_profiles() {
        let text = "#import std\n#calc.abs([x])";
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), 1, "{warnings:?}");
            let w = &warnings[0];
            assert_eq!(w.message, "this import has no effect");
            assert_eq!(w.severity, Severity::Warning);
            assert!(w.hints.is_empty());
            assert!(w.trace.is_empty());
            assert_eq!(source.span_byte_range(w.span), Some(8..11));
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, CONTENT_ERROR);
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            let start = text.find("[x]").unwrap();
            assert_eq!(source.span_byte_range(d.span), Some(start..start + 3));
        }
    }
    #[test]
    fn p1328_public_numeric_and_untouched_rejections_all_profiles() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-2pt)", Value::Length(Length::pt(2.0))),
                ("calc.abs(-2deg)", Value::Angle(Angle::deg(2.0))),
                ("calc.abs(-2%)", Value::Ratio(Ratio::from_percent(2.0))),
                ("calc.abs(-2fr)", Value::Fraction(2.0)),
                ("calc.abs(-19)", Value::Int(19)),
                ("calc.abs(-1.25)", Value::Float(1.25)),
                ("calc.abs(decimal(\"-1.25\"))", Value::Decimal(Decimal::new(125, 2))),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                assert_eq!(result.unwrap(), expected, "{expr}");
            }
            let expr = "calc.abs(-9223372036854775807 - 1)";
            let (result, warnings) =
                eval_expression_with_features(&world, expr, features);
            assert!(warnings.is_empty(), "{expr}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, "the result is too large");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            let source = Source::new_with_parser(
                world.main(),
                expr.into(),
                crate::compiler::parse::parse_code,
            );
            assert_eq!(source.span_byte_range(d.span), Some(9..expr.len() - 1));
            for (expr, message, name) in [
                (
                    "calc.abs(\"x\")",
                    "expected integer, float, length, angle, ratio, fraction, or decimal, found string",
                    "abs",
                ),
                (
                    "calc.abs(sym.alpha)",
                    "expected integer, float, length, angle, ratio, fraction, or decimal, found symbol",
                    "abs",
                ),
                (
                    "calc.sqrt([x])",
                    "calc.sqrt(): esperava Int ou Float, recebeu content",
                    "calc.sqrt",
                ),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                let errors = result.unwrap_err();
                assert_eq!(errors.len(), 1, "{expr}: {errors:?}");
                let d = &errors[0];
                assert_eq!(d.message, message);
                assert_eq!(d.severity, Severity::Error);
                assert!(d.hints.is_empty());
                let source = Source::new_with_parser(
                    world.main(),
                    expr.into(),
                    crate::compiler::parse::parse_code,
                );
                if name == "abs" {
                    assert_eq!(source.span_byte_range(d.span), Some(9..expr.len() - 1));
                    assert!(d.trace.is_empty(), "{expr}: {d:?}");
                } else {
                    assert!(d.span.is_detached(), "{expr}: {d:?}");
                    assert_eq!(d.trace.len(), 1);
                    assert_eq!(d.trace[0].v, Tracepoint::Call(Some(name.into())));
                    assert_eq!(source.span_byte_range(d.trace[0].span), Some(0..expr.len()));
                }
            }
        }
    }
}

#[cfg(test)]
mod p1329_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.source,
            &ElementRegistry::new(),
            false,
            EvalTarget::Paged,
            features,
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    const MIXED_ERROR: &str = "cannot take absolute value of this length";

    // Observe the public language species and scalar magnitude, not render bytes
    // or Value's structural PartialEq. NaN is observed as a class, never equality.
    fn magnitude(actual: f64, expected: f64) {
        if expected.is_nan() {
            assert!(actual.is_nan(), "expected NaN, got {actual}");
        } else if expected.is_infinite() {
            assert!(actual.is_infinite() && actual.is_sign_positive(), "{actual}");
        } else {
            assert!(
                (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
                "expected {expected}, got {actual}"
            );
        }
    }
    fn semantic(actual: Value, expected: Value) {
        assert_eq!(actual.type_name(), expected.type_name());
        match (actual, expected) {
            (Value::Length(a), Value::Length(e)) => {
                magnitude(a.abs.to_pt(), e.abs.to_pt());
                magnitude(a.em, e.em);
            }
            (Value::Angle(a), Value::Angle(e)) => magnitude(a.to_rad(), e.to_rad()),
            (Value::Ratio(a), Value::Ratio(e)) => magnitude(a.get(), e.get()),
            (Value::Fraction(a), Value::Fraction(e)) => magnitude(a, e),
            (Value::Float(a), Value::Float(e)) => magnitude(a, e),
            (a, e) => panic!("unexpected semantic comparison: {a:?} / {e:?}"),
        }
    }
    #[test]
    fn p1329_native_scalar_species_signs_zero_and_overturn() {
        for features in profiles() {
            for scalar in [-720.25_f64, -3.5, -0.0, 0.0, 2.25, 810.0] {
                for (input, expected) in [
                    (
                        Value::Length(Length::pt(scalar)),
                        Value::Length(Length::pt(scalar.abs())),
                    ),
                    (
                        Value::Length(Length::em(scalar)),
                        Value::Length(Length::em(scalar.abs())),
                    ),
                    (
                        Value::Angle(Angle::rad(scalar)),
                        Value::Angle(Angle::rad(scalar.abs())),
                    ),
                    (Value::Ratio(Ratio(scalar)), Value::Ratio(Ratio(scalar.abs()))),
                    (Value::Fraction(scalar), Value::Fraction(scalar.abs())),
                ] {
                    semantic(
                        native(&Args::positional(vec![input]), features).unwrap(),
                        expected,
                    );
                }
            }
            for (abs, em) in [(-0.0, -4.5), (-4.5, -0.0), (0.0, -0.0), (-0.0, -0.0)] {
                semantic(
                    native(
                        &Args::positional(vec![Value::Length(Length {
                            abs: Abs::pt(abs),
                            em,
                        })]),
                        features,
                    )
                    .unwrap(),
                    Value::Length(Length { abs: Abs::pt(abs.abs()), em: em.abs() }),
                );
            }
        }
    }
    #[test]
    fn p1329_native_nonfinite_values_are_already_constructed() {
        for features in profiles() {
            for scalar in [f64::NAN, f64::NEG_INFINITY, f64::INFINITY] {
                for (input, expected) in [
                    (
                        Value::Length(Length::pt(scalar)),
                        Value::Length(Length::pt(scalar.abs())),
                    ),
                    (
                        Value::Length(Length::em(scalar)),
                        Value::Length(Length::em(scalar.abs())),
                    ),
                    (
                        Value::Angle(Angle::rad(scalar)),
                        Value::Angle(Angle::rad(scalar.abs())),
                    ),
                    (Value::Ratio(Ratio(scalar)), Value::Ratio(Ratio(scalar.abs()))),
                    (Value::Fraction(scalar), Value::Fraction(scalar.abs())),
                    (Value::Float(scalar), Value::Float(scalar.abs())),
                ] {
                    semantic(
                        native(&Args::positional(vec![input]), features).unwrap(),
                        expected,
                    );
                }
                for (abs, em) in [(scalar, 1.0), (1.0, scalar), (scalar, scalar)] {
                    bare_diagnostic(
                        &native(
                            &Args::positional(vec![Value::Length(Length {
                                abs: Abs::pt(abs),
                                em,
                            })]),
                            features,
                        )
                        .unwrap_err(),
                        MIXED_ERROR,
                        Span::detached(),
                    );
                }
            }
        }
    }
    #[test]
    fn p1329_native_mixed_all_signs_value_origin_and_synthetic_traps() {
        let source = Source::detached("aggregate occurrence value other");
        let aggregate = Span::from_range(source.id(), 0..32);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..26);
        for features in profiles() {
            for (abs, em) in
                [(2.0, 3.0), (-2.0, -3.0), (2.0, -3.0), (-2.0, 3.0), (1e-100, 1e-100)]
            {
                let value = Value::Length(Length { abs: Abs::pt(abs), em });
                for value_span in [origin, Span::detached()] {
                    let args = Args::from_occurrences(
                        aggregate,
                        vec![ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: occurrence,
                            value_span,
                        }],
                    );
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        MIXED_ERROR,
                        value_span,
                    );
                }
                let args =
                    Args::from_parts(vec![value.clone()], Default::default(), aggregate);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    MIXED_ERROR,
                    Span::detached(),
                );
                let mut empty_occurrences = args.clone();
                empty_occurrences.occurrences = Some(Vec::new());
                bare_diagnostic(
                    &native(&empty_occurrences, features).unwrap_err(),
                    MIXED_ERROR,
                    Span::detached(),
                );
                // Synthetic occurrence metadata deliberately precedes the first
                // positional origin: it must not hijack the argument's anchor.
                let mut positional_after_named = args;
                positional_after_named.occurrences = Some(vec![
                    ArgOccurrence {
                        name: Some("metadata".into()),
                        value: Value::Int(1),
                        span: aggregate,
                        value_span: occurrence,
                    },
                    ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: origin,
                    },
                ]);
                bare_diagnostic(
                    &native(&positional_after_named, features).unwrap_err(),
                    MIXED_ERROR,
                    origin,
                );
            }
        }
    }
    #[test]
    fn p1329_native_guards_precede_dimension_dispatch() {
        for features in profiles() {
            for value in [
                Value::Length(Length::pt(-2.0)),
                Value::Length(Length { abs: Abs::pt(2.0), em: 3.0 }),
                Value::Angle(Angle::deg(-2.0)),
                Value::Ratio(Ratio(-2.0)),
                Value::Fraction(-2.0),
            ] {
                let mut args = Args::positional(vec![value.clone(), value.clone()]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    "argumento nomeado inesperado: 'bad'",
                    Span::detached(),
                );
                args.named.clear();
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    "calc.abs() requer 1 argumento, recebeu 2",
                    Span::detached(),
                );
            }
        }
    }
    #[test]
    fn p1329_public_units_species_alias_with_spread() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-2cm)", Value::Length(Length::pt(7200.0 / 127.0))),
                ("calc.abs(-2mm)", Value::Length(Length::pt(720.0 / 127.0))),
                ("calc.abs(-2in)", Value::Length(Length::pt(144.0))),
                ("calc.abs(-2em)", Value::Length(Length::em(2.0))),
                ("calc.abs(0pt - 3em)", Value::Length(Length::em(3.0))),
                ("calc.abs(-2pt + 0em)", Value::Length(Length::pt(2.0))),
                ("calc.abs(-0em)", Value::Length(Length::ZERO)),
                ("calc.abs(-2rad)", Value::Angle(Angle::rad(2.0))),
                ("calc.abs(-810deg)", Value::Angle(Angle::deg(810.0))),
                ("calc.abs(-250%)", Value::Ratio(Ratio(2.5))),
                ("calc.abs(-2.5fr)", Value::Fraction(2.5)),
                ("{let ab=calc.abs; ab(-2em)}", Value::Length(Length::em(2.0))),
                (
                    "{let ab=calc.abs.with(-2rad).with(); ab()}",
                    Value::Angle(Angle::rad(2.0)),
                ),
                ("calc.abs(..(-250%,))", Value::Ratio(Ratio(2.5))),
                ("{let aa=arguments(-2.5fr); calc.abs(..aa)}", Value::Fraction(2.5)),
                ("{let café = -2em;\n calc.abs(café)}", Value::Length(Length::em(2.0))),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                semantic(result.unwrap(), expected);
            }
        }
    }
    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }
    #[test]
    fn p1329_public_mixed_origins_all_signs_utf8_alias_with_spread() {
        for (text, anchor, call) in [
            ("#calc.abs(2pt + 3em)", "2pt + 3em", None),
            ("#calc.abs(-2pt - 3em)", "-2pt - 3em", None),
            ("#calc.abs(2pt - 3em)", "2pt - 3em", None),
            ("#calc.abs(-2pt + 3em)", "-2pt + 3em", None),
            ("#let ab = calc.abs\n#ab(2pt + 3em)", "2pt + 3em", None),
            ("#let ab = calc.abs.with()\n#ab(2pt + 3em)", "2pt + 3em", None),
            ("#let ab = calc.abs.with(2pt + 3em)\n#ab()", "2pt + 3em", Some("ab()")),
            (
                "#let ab = calc.abs.with(2pt + 3em).with()\n#ab()",
                "2pt + 3em",
                Some("ab()"),
            ),
            ("#calc.abs(..(2pt + 3em,))", "..(2pt + 3em,)", None),
            (
                "#let aa = arguments(2pt + 3em)\n#calc.abs(..aa)",
                "2pt + 3em",
                Some("calc.abs(..aa)"),
            ),
            ("#let x = 2pt + 3em\n#let y = 2pt + 3em\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = 2pt + 3em\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, MIXED_ERROR, call);
        }
    }
    #[test]
    fn p1329_math_dimensional_text_stays_content() {
        for (text, anchor) in [
            ("$std.calc.abs(-2deg)$", "-2deg"),
            ("#let ab = calc.abs\n$ab(-2deg)$", "-2deg"),
        ] {
            public_error(text, anchor, CONTENT_ERROR, None);
        }
    }
    #[test]
    fn p1329_warning_survives_both_success_and_mixed_error() {
        for features in profiles() {
            for (text, success) in [
                ("#import std\n#calc.abs(-2em)", true),
                ("#import std\n#calc.abs(2pt + 3em)", false),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if success {
                    assert!(result.is_ok());
                } else {
                    let start = text.find("2pt + 3em").unwrap();
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    assert_eq!(errors[0].message, MIXED_ERROR);
                    assert_eq!(errors[0].severity, Severity::Error);
                    assert!(errors[0].hints.is_empty());
                    assert!(errors[0].trace.is_empty());
                    assert_eq!(
                        source.span_byte_range(errors[0].span),
                        Some(start..start + 9)
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod p1330_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.source,
            &ElementRegistry::new(),
            false,
            EvalTarget::Paged,
            features,
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }

    const OVERFLOW: &str = "the result is too large";

    #[test]
    fn p1330_native_large_noninteger_species_do_not_inherit_integer_limit() {
        for features in profiles() {
            for (input, expected) in [
                (-9223372036854775808.0, 9223372036854775808.0),
                (-1e30, 1e30),
                (-0.0, 0.0),
            ] {
                match native(&Args::positional(vec![Value::Float(input)]), features)
                    .unwrap()
                {
                    Value::Float(actual) => assert_eq!(actual, expected),
                    other => panic!("float abs changed species: {other:?}"),
                }
            }
            match native(
                &Args::positional(vec![Value::Decimal(Decimal::new(i64::MIN, 0))]),
                features,
            )
            .unwrap()
            {
                Value::Decimal(actual) => {
                    assert_eq!(actual.to_string(), "9223372036854775808");
                }
                other => panic!("decimal abs changed species: {other:?}"),
            }
        }
    }

    #[test]
    fn p1330_native_integer_limits_exact_species_and_magnitude() {
        for features in profiles() {
            // Explicit expected values avoid reusing the implementation formula.
            for (input, expected) in [
                (i64::MIN + 1, i64::MAX),
                (i64::MIN + 2, i64::MAX - 1),
                (-9007199254740993, 9007199254740993),
                (-2147483648, 2147483648),
                (-1, 1),
                (0, 0),
                (1, 1),
                (2147483648, 2147483648),
                (9007199254740993, 9007199254740993),
                (i64::MAX - 1, i64::MAX - 1),
                (i64::MAX, i64::MAX),
            ] {
                match native(&Args::positional(vec![Value::Int(input)]), features)
                    .unwrap()
                {
                    Value::Int(actual) => assert_eq!(actual, expected, "input {input}"),
                    other => panic!("integer abs changed species: {other:?}"),
                }
            }
            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features)
                    .unwrap_err(),
                OVERFLOW,
                Span::detached(),
            );
        }
    }

    #[test]
    fn p1330_native_overflow_origin_first_positional_and_no_fabrication() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for value_span in [origin, Span::detached()] {
                let mut args = Args::from_parts(
                    vec![Value::Int(i64::MIN)],
                    Default::default(),
                    aggregate,
                );
                // Metadata is intentionally inconsistent with the named view to
                // distinguish occurrence filtering from the named-argument guard.
                args.occurrences = Some(vec![
                    ArgOccurrence {
                        name: Some("metadata".into()),
                        value: Value::Int(1),
                        span: aggregate,
                        value_span: decoy,
                    },
                    ArgOccurrence {
                        name: None,
                        value: Value::Int(i64::MIN),
                        span: occurrence,
                        value_span,
                    },
                    ArgOccurrence {
                        name: None,
                        value: Value::Int(i64::MIN),
                        span: decoy,
                        value_span: decoy,
                    },
                ]);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    OVERFLOW,
                    value_span,
                );
            }
            for occurrences in [None, Some(vec![])] {
                let mut args = Args::from_parts(
                    vec![Value::Int(i64::MIN)],
                    Default::default(),
                    aggregate,
                );
                args.occurrences = occurrences;
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    OVERFLOW,
                    Span::detached(),
                );
            }
        }
    }

    #[test]
    fn p1330_native_named_and_arity_guards_precede_overflow() {
        for features in profiles() {
            for count in [0, 1, 2] {
                let mut args = Args::positional(vec![Value::Int(i64::MIN); count]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    "argumento nomeado inesperado: 'bad'",
                    Span::detached(),
                );
                if count != 1 {
                    args.named.clear();
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &format!("calc.abs() requer 1 argumento, recebeu {count}"),
                        Span::detached(),
                    );
                }
            }
        }
    }

    #[test]
    fn p1330_public_integer_boundaries_and_positive_routes() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-9223372036854775807)", i64::MAX),
                ("calc.abs(-9223372036854775807 + 1)", i64::MAX - 1),
                ("calc.abs(9223372036854775807)", i64::MAX),
                ("calc.abs(-9007199254740993)", 9007199254740993),
                ("calc.abs(-0)", 0),
                ("{let ab=calc.abs; ab(-1)}", 1),
                ("{let ab=calc.abs.with(-19).with(); ab()}", 19),
                ("{let aa=arguments(-19); calc.abs(..aa)}", 19),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                match result.unwrap() {
                    Value::Int(actual) => assert_eq!(actual, expected, "{expr}"),
                    other => panic!("{expr}: wrong species {other:?}"),
                }
            }
        }
    }

    #[test]
    fn p1330_public_overflow_real_origins_alias_with_arguments_utf8() {
        for (text, anchor, trace) in [
            ("#calc.abs(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with()\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1)\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1).with()\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#calc.abs(..(-9223372036854775807 - 1,))", "..(-9223372036854775807 - 1,)", None),
            ("#let aa = arguments(-9223372036854775807 - 1)\n#calc.abs(..aa)", "-9223372036854775807 - 1", Some("calc.abs(..aa)")),
            ("#let x = -9223372036854775807 - 1\n#let y = -9223372036854775807 - 1\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = -9223372036854775807 - 1\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, OVERFLOW, trace);
        }
    }

    #[test]
    fn p1330_math_spelling_remains_content_without_integer_coercion() {
        for text in [
            "$std.calc.abs(-9223372036854775807 - 1)$",
            "#let ab = calc.abs\n$ab(-9223372036854775807 - 1)$",
        ] {
            public_error(text, "-9223372036854775807 - 1", CONTENT_ERROR, None);
        }
    }

    #[test]
    fn p1330_warning_survives_overflow_and_boundary_success() {
        for features in profiles() {
            for (text, fails) in [
                ("#import std\n#calc.abs(-9223372036854775807)", false),
                ("#import std\n#calc.abs(-9223372036854775807 - 1)", true),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if fails {
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    let d = &errors[0];
                    assert_eq!(d.message, OVERFLOW);
                    assert_eq!(d.severity, Severity::Error);
                    assert!(d.hints.is_empty());
                    assert!(d.trace.is_empty());
                    let start = text.find("-9223372036854775807 - 1").unwrap();
                    assert_eq!(
                        source.span_byte_range(d.span),
                        Some(start..start + "-9223372036854775807 - 1".len())
                    );
                } else {
                    assert!(result.is_ok());
                }
            }
        }
    }
}

#[cfg(test)]
mod p1331_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::Length;
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
    use crate::entities::rel::Rel;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        // The fixture has one virtual file at the project root. Resolution
        // constructs a value in RAM; it never reads a file or host path.
        fn resolve_path(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<RootedPath, String> {
            if current_file != self.main() {
                return Err("unknown virtual file in P1331 fixture".into());
            }
            let vpath = VirtualPath::new(path)
                .map_err(|error| format!("invalid virtual path: {error:?}"))?;
            Ok(RootedPath::new(VirtualRoot::Project, vpath))
        }

        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.source,
            &ElementRegistry::new(),
            false,
            EvalTarget::Paged,
            features,
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }

    fn rejection(kind: &str) -> String {
        format!("expected integer, float, length, angle, ratio, fraction, or decimal, found {kind}")
    }

    fn family_values(features: Features) -> Vec<(Value, &'static str)> {
        let world = TestWorld::new("");
        let mut values = Vec::new();
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
        ] {
            let (result, warnings) =
                eval_expression_with_features(&world, expression, features);
            assert!(warnings.is_empty(), "constructor {expression}: {warnings:?}");
            values.push((result.expect(expression), kind));
        }
        values.push((Value::Relative(Rel::zero()), "relative length"));
        values.push((
            Value::Relative(Rel { rel: 0.5, abs: Length::pt(2.0) }),
            "relative length",
        ));
        values.push((
            Value::Path(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("/p1331.typ").unwrap(),
            )),
            "path",
        ));
        // Preconstructed Location only: this does not test introspection production.
        values.push((Value::Location(Locator::new().next()), "location"));
        values
    }

    #[test]
    fn p1331_native_every_rejected_family_complete_diagnostic_and_origins() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for (value, kind) in family_values(features) {
                for value_span in [origin, Span::detached()] {
                    let mut args = Args::from_parts(
                        vec![value.clone()],
                        Default::default(),
                        aggregate,
                    );
                    args.occurrences = Some(vec![
                        ArgOccurrence {
                            name: Some("metadata".into()),
                            value: Value::Int(1),
                            span: aggregate,
                            value_span: decoy,
                        },
                        ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: occurrence,
                            value_span,
                        },
                        ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: decoy,
                            value_span: decoy,
                        },
                    ]);
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &rejection(kind),
                        value_span,
                    );
                }
                for occurrences in [
                    None,
                    Some(vec![]),
                    Some(vec![ArgOccurrence {
                        name: Some("metadata".into()),
                        value: value.clone(),
                        span: aggregate,
                        value_span: decoy,
                    }]),
                ] {
                    let mut args = Args::from_parts(
                        vec![value.clone()],
                        Default::default(),
                        aggregate,
                    );
                    args.occurrences = occurrences;
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &rejection(kind),
                        Span::detached(),
                    );
                }
            }
        }
    }

    #[test]
    fn p1331_native_guards_still_precede_every_rejected_family() {
        for features in profiles() {
            for (value, _) in family_values(features) {
                for count in [0, 1, 2] {
                    let mut args = Args::positional(vec![value.clone(); count]);
                    args.named.insert("bad".into(), Value::Int(1));
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        "argumento nomeado inesperado: 'bad'",
                        Span::detached(),
                    );
                    if count != 1 {
                        args.named.clear();
                        bare_diagnostic(
                            &native(&args, features).unwrap_err(),
                            &format!("calc.abs() requer 1 argumento, recebeu {count}"),
                            Span::detached(),
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn p1331_public_rejected_families_anchor_the_whole_argument() {
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
            ("0% + 0pt", "relative length"),
            ("50% + 2pt", "relative length"),
            ("path(\"p1331.typ\")", "path"),
        ] {
            let text = format!("#calc.abs({expression})");
            public_error(&text, expression, &rejection(kind), None);
        }
    }

    #[test]
    fn p1331_public_real_origins_alias_with_spread_arguments_utf8() {
        for (text, anchor, kind, trace) in [
            ("#let ab=calc.abs\n#ab(false)", "false", "boolean", None),
            ("#let ab=calc.abs.with()\n#ab(\"x\")", "\"x\"", "string", None),
            ("#let ab=calc.abs.with(\"x\")\n#ab()", "\"x\"", "string", Some("ab()")),
            (
                "#let ab=calc.abs.with(false).with()\n#ab()",
                "false",
                "boolean",
                Some("ab()"),
            ),
            ("#calc.abs(..(false,))", "..(false,)", "boolean", None),
            (
                "#let aa=arguments(\"x\")\n#calc.abs(..aa)",
                "\"x\"",
                "string",
                Some("calc.abs(..aa)"),
            ),
            ("#let x=\"x\"\n#let y=\"x\"\n#calc.abs(y)", "y", "string", None),
            ("texto á\n#let café=\"á\"\n#calc.abs(\n café\n)", "café", "string", None),
        ] {
            public_error(text, anchor, &rejection(kind), trace);
        }
    }

    #[test]
    fn p1331_math_explicit_string_remains_string_without_numeric_coercion() {
        for text in ["$std.calc.abs(\"-1\")$", "#let ab=calc.abs\n$ab(\"-1\")$"] {
            public_error(text, "\"-1\"", &rejection("string"), None);
        }
    }

    #[test]
    fn p1331_warning_survives_rejection_with_all_fields() {
        let text = "#import std\n#calc.abs(false)";
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), 1);
            let w = &warnings[0];
            assert_eq!(w.message, "this import has no effect");
            assert_eq!(w.severity, Severity::Warning);
            assert!(w.hints.is_empty());
            assert!(w.trace.is_empty());
            assert_eq!(source.span_byte_range(w.span), Some(8..11));
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, rejection("boolean"));
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            assert_eq!(source.span_byte_range(d.span), Some(22..27));
        }
    }

    #[test]
    fn p1331_existing_numeric_species_and_special_errors_remain() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expression, expected) in [
                ("(type(calc.abs(-19)), calc.abs(-19))", "(int, 19)"),
                ("(type(calc.abs(-1.25)), calc.abs(-1.25))", "(float, 1.25)"),
                (
                    "(type(calc.abs(decimal(\"-1.25\"))), calc.abs(decimal(\"-1.25\")))",
                    "(decimal, decimal(\"1.25\"))",
                ),
                ("(type(calc.abs(-2pt)), calc.abs(-2pt))", "(length, 2pt)"),
                ("(type(calc.abs(-2em)), calc.abs(-2em))", "(length, 2em)"),
                ("(type(calc.abs(-810deg)), calc.abs(-810deg))", "(angle, 810deg)"),
                ("(type(calc.abs(-250%)), calc.abs(-250%))", "(ratio, 250%)"),
                ("(type(calc.abs(-2fr)), calc.abs(-2fr))", "(fraction, 2fr)"),
            ] {
                let (result, warnings) = eval_expression_with_features(
                    &world,
                    &format!("repr({expression})"),
                    features,
                );
                assert!(warnings.is_empty(), "{expression}: {warnings:?}");
                match result.unwrap() {
                    Value::Str(actual) => assert_eq!(actual.as_str(), expected),
                    other => panic!("repr returned non-string {other:?}"),
                }
            }
            match native(
                &Args::positional(vec![Value::Decimal(Decimal::new(-125, 2))]),
                features,
            )
            .unwrap()
            {
                Value::Decimal(actual) => assert_eq!(actual.to_string(), "1.25"),
                other => panic!("decimal species changed: {other:?}"),
            }
        }
        public_error(
            "#calc.abs(-9223372036854775807 - 1)",
            "-9223372036854775807 - 1",
            "the result is too large",
            None,
        );
        public_error(
            "#calc.abs(2pt + 3em)",
            "2pt + 3em",
            "cannot take absolute value of this length",
            None,
        );
        public_error("#calc.abs([x])", "[x]", CONTENT_ERROR, None);
    }
    #[test]
    fn p1331_fixture_path_constructor_succeeds_before_abs() {
        // This sentinel does not call calc.abs. It establishes the constructor
        // precondition for the unchanged public rejection assertion.
        let world = TestWorld::new("");
        for features in profiles() {
            let (result, warnings) =
                eval_expression_with_features(&world, "path(\"p1331.typ\")", features);
            assert!(warnings.is_empty(), "Path fixture warnings: {warnings:?}");
            match result.expect("Path must be constructed before testing abs") {
                Value::Path(path) => {
                    assert!(matches!(path.root(), VirtualRoot::Project));
                    assert_eq!(path.vpath().get_with_slash(), "/p1331.typ");
                }
                other => panic!("Path constructor changed language type: {other:?}"),
            }
        }
    }
}

#[cfg(test)]
mod p1332_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;
    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.source,
            &ElementRegistry::new(),
            false,
            EvalTarget::Paged,
            features,
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }

    #[test]
    fn p1332_intrinsic_name_follows_lookup_alias_import_and_nested_with() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expression in [
                "calc.abs",
                "std.calc.abs",
                "{let a=calc.abs; a}",
                "{import calc: abs; abs}",
                "calc.abs.with()",
                "calc.abs.with(-19)",
                "calc.abs.with().with(-19)",
                "{import calc: abs; let a=abs.with(-19).with(); a}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty(), "{expression}: {warnings:?}");
                match result.expect(expression) {
                    Value::Func(function) => {
                        assert_eq!(function.name(), Some("abs"), "{expression}")
                    }
                    other => panic!("function species lost: {expression}: {other:?}"),
                }
            }
            for (expression, expected) in [
                ("calc.sqrt", "calc.sqrt"),
                ("calc.pow", "calc.pow"),
                ("calc.sqrt.with(-1)", "calc.sqrt"),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.unwrap() {
                    Value::Func(function) => assert_eq!(function.name(), Some(expected)),
                    other => panic!("other function species lost: {other:?}"),
                }
            }
        }
    }
    #[test]
    fn p1332_language_identity_and_with_result_are_preserved() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expression in [
                "calc.abs == std.calc.abs",
                "{import calc: abs; let a=abs; a == calc.abs}",
                "calc.abs != calc.sqrt",
                "calc.abs != calc.pow",
                "calc.abs != (x => x)",
                "{let a=calc.abs.with(-19); let b=a; a == b}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                assert!(
                    matches!(result.expect(expression), Value::Bool(true)),
                    "{expression}"
                );
            }
            for expression in [
                "{import calc: abs; abs(-9007199254740993)}",
                "{import calc: abs; let a=abs.with().with(-9007199254740993); a()}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.expect(expression) {
                    Value::Int(actual) => assert_eq!(actual, 9007199254740993),
                    other => panic!("exact integer result changed: {other:?}"),
                }
            }
            for (expression, expected) in [
                ("repr(calc.abs)", "abs"),
                ("{import calc: abs; repr(abs)}", "abs"),
                ("{let a=calc.abs; repr(a)}", "abs"),
                ("repr(calc.abs.with(-1))", "(..) => .."),
                ("repr(calc.abs.with().with(-1))", "(..) => .."),
                ("repr(calc.sqrt)", "sqrt"),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.expect(expression) {
                    Value::Str(actual) => assert_eq!(actual.as_str(), expected),
                    other => panic!("repr result species changed: {other:?}"),
                }
            }
        }
    }

    fn traced_error(
        text: &str,
        anchor: Option<&str>,
        message: &str,
        call: &str,
        name: &str,
        warning: bool,
    ) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), usize::from(warning), "{text}: {warnings:?}");
            if warning {
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                let start = text.find("std").unwrap();
                assert_eq!(source.span_byte_range(w.span), Some(start..start + 3));
            }
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            if let Some(anchor) = anchor {
                let start = text.rfind(anchor).unwrap();
                assert_eq!(
                    source.span_byte_range(d.span),
                    Some(start..start + anchor.len()),
                    "{text}"
                );
            } else {
                assert!(d.span.is_detached(), "{text}: {d:?}");
            }
            assert_eq!(d.trace.len(), 1, "{text}: {d:?}");
            assert_eq!(d.trace[0].v, Tracepoint::Call(Some(name.into())));
            let start = text.rfind(call).unwrap();
            assert_eq!(
                source.span_byte_range(d.trace[0].span),
                Some(start..start + call.len())
            );
        }
    }

    #[test]
    fn p1332_imported_abs_external_origins_all_error_families() {
        for (value, message) in [
            ("[x]", "expected integer, float, length, angle, ratio, fraction, or decimal, found content"),
            ("2pt + 3em", "cannot take absolute value of this length"),
            ("-9223372036854775807 - 1", "the result is too large"),
            ("false", "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean"),
            ("\"x\"", "expected integer, float, length, angle, ratio, fraction, or decimal, found string"),
        ] {
            for text in [
                format!("#import calc: abs\n#let a=abs.with({value})\n#a()"),
                format!("#import calc: abs\n#let a=abs.with({value}).with()\n#a()"),
                format!("#import calc: abs\n#let aa=arguments({value})\n#abs(..aa)"),
            ] {
                let call = if text.ends_with("#a()") { "a()" } else { "abs(..aa)" };
                traced_error(&text, Some(value), message, call, "abs", false);
            }
        }
    }
    #[test]
    fn p1332_trace_rename_retains_warning_utf8_and_guard_debt() {
        traced_error(
            "#import std\n#let café=calc.abs.with([á])\n#café()",
            Some("[á]"),
            "expected integer, float, length, angle, ratio, fraction, or decimal, found content",
            "café()", "abs", true,
        );
        for (text, message, call) in [
            ("#calc.abs()", "calc.abs() requer 1 argumento, recebeu 0", "calc.abs()"),
            (
                "#let a=calc.abs.with()\n#a()",
                "calc.abs() requer 1 argumento, recebeu 0",
                "a()",
            ),
            (
                "#let a=calc.abs.with(false, 1)\n#a()",
                "calc.abs() requer 1 argumento, recebeu 2",
                "a()",
            ),
            (
                "#let a=calc.abs.with(false, bad: 1)\n#a()",
                "argumento nomeado inesperado: 'bad'",
                "a()",
            ),
        ] {
            traced_error(text, None, message, call, "abs", false);
        }
        traced_error(
            "#let a=calc.sqrt.with(\"x\")\n#a()",
            None,
            "calc.sqrt(): esperava Int ou Float, recebeu str",
            "a()",
            "calc.sqrt",
            false,
        );
    }
}
