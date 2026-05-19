//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 7df1ee98
//! @layer L1
//! @updated 2026-04-23
//!
//! Módulo `calc` — operações matemáticas escalares (abs, pow, sqrt, floor, ceil, round, min, max, clamp).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use ecow::EcoString;
use crate::entities::file_id::FileId;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use super::{err, expect_no_named};

use crate::entities::args::Args;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── Módulo calc (Passo 27) ───────────────────────────────────────────────────

/// Constrói o módulo `calc` como `Value::Dict` com 25 funções + 4 constantes.
///
/// Divergência: original usa `Value::Module`. Cristalino usa `Value::Dict`
/// porque não temos stdlib Module sem world. Semântica de acesso (`calc.abs`)
/// é idêntica via `eval_field_access` sobre Dict.
///
/// P283 estendeu o módulo com trig/hiperbólicas/log/exp + constantes
/// (`pi`, `tau`, `e`, `inf`); decisão libm vs f64 e tratamento de domínio
/// documentados em `diagnostico-calc-passo-283.md`.
pub fn make_calc_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("abs".into(),   Value::Func(Func::native("calc.abs",   calc_abs)));
    dict.insert("pow".into(),   Value::Func(Func::native("calc.pow",   calc_pow)));
    dict.insert("sqrt".into(),  Value::Func(Func::native("calc.sqrt",  calc_sqrt)));
    dict.insert("floor".into(), Value::Func(Func::native("calc.floor", calc_floor)));
    dict.insert("ceil".into(),  Value::Func(Func::native("calc.ceil",  calc_ceil)));
    dict.insert("round".into(), Value::Func(Func::native("calc.round", calc_round)));
    dict.insert("min".into(),   Value::Func(Func::native("calc.min",   calc_min)));
    dict.insert("max".into(),   Value::Func(Func::native("calc.max",   calc_max)));
    dict.insert("clamp".into(), Value::Func(Func::native("calc.clamp", calc_clamp)));
    // P283 — trig (radianos puros; sem tipo Angle — ver diagnóstico §A.1).
    dict.insert("sin".into(),   Value::Func(Func::native("calc.sin",   calc_sin)));
    dict.insert("cos".into(),   Value::Func(Func::native("calc.cos",   calc_cos)));
    dict.insert("tan".into(),   Value::Func(Func::native("calc.tan",   calc_tan)));
    dict.insert("asin".into(),  Value::Func(Func::native("calc.asin",  calc_asin)));
    dict.insert("acos".into(),  Value::Func(Func::native("calc.acos",  calc_acos)));
    dict.insert("atan".into(),  Value::Func(Func::native("calc.atan",  calc_atan)));
    dict.insert("atan2".into(), Value::Func(Func::native("calc.atan2", calc_atan2)));
    // P283 — hiperbólicas.
    dict.insert("sinh".into(),  Value::Func(Func::native("calc.sinh",  calc_sinh)));
    dict.insert("cosh".into(),  Value::Func(Func::native("calc.cosh",  calc_cosh)));
    dict.insert("tanh".into(),  Value::Func(Func::native("calc.tanh",  calc_tanh)));
    dict.insert("asinh".into(), Value::Func(Func::native("calc.asinh", calc_asinh)));
    dict.insert("acosh".into(), Value::Func(Func::native("calc.acosh", calc_acosh)));
    dict.insert("atanh".into(), Value::Func(Func::native("calc.atanh", calc_atanh)));
    // P283 — exponencial / logaritmos.
    dict.insert("exp".into(),   Value::Func(Func::native("calc.exp",   calc_exp)));
    dict.insert("ln".into(),    Value::Func(Func::native("calc.ln",    calc_ln)));
    dict.insert("log".into(),   Value::Func(Func::native("calc.log",   calc_log)));
    // P283 — constantes ergonómicas (paridade vanilla).
    dict.insert("pi".into(),    Value::Float(std::f64::consts::PI));
    dict.insert("tau".into(),   Value::Float(std::f64::consts::TAU));
    dict.insert("e".into(),     Value::Float(std::f64::consts::E));
    dict.insert("inf".into(),   Value::Float(f64::INFINITY));
    Value::Dict(dict)
}

pub(crate) fn calc_abs(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(i.saturating_abs())),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        [other] => err(format!("calc.abs() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.abs() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_pow(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(base), Value::Int(exp)] => {
            if *exp < 0 {
                return err("calc.pow() expoente negativo requer Float");
            }
            Ok(Value::Int(base.saturating_pow(*exp as u32)))
        }
        [base, exp] => {
            let b = coerce_to_f64(base, "calc.pow() base")?;
            let e = coerce_to_f64(exp,  "calc.pow() expoente")?;
            // DEBT: migrar para libm::pow quando libm for dependência do workspace (ADR-0018)
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e))
        }
        _ => err(format!("calc.pow() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_sqrt(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
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

pub(crate) fn calc_floor(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.floor() as i64)),
        [other] => err(format!("calc.floor() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.floor() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_ceil(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.ceil() as i64)),
        [other] => err(format!("calc.ceil() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.ceil() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_round(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.round() as i64)),
        [other] => err(format!("calc.round() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.round() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_min(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.min() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a),   Value::Int(b))   => Value::Int(*a.min(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
            (Value::Int(a),   Value::Float(b)) => Value::Float((*a as f64).min(*b)),
            (Value::Float(a), Value::Int(b))   => Value::Float(a.min(*b as f64)),
            (_, other) => return err(format!(
                "calc.min() tipos incompatíveis: {} e {}", result.type_name(), other.type_name()
            )),
        };
    }
    Ok(result)
}

pub(crate) fn calc_max(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.max() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a),   Value::Int(b))   => Value::Int(*a.max(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
            (Value::Int(a),   Value::Float(b)) => Value::Float((*a as f64).max(*b)),
            (Value::Float(a), Value::Int(b))   => Value::Float(a.max(*b as f64)),
            (_, other) => return err(format!(
                "calc.max() tipos incompatíveis: {} e {}", result.type_name(), other.type_name()
            )),
        };
    }
    Ok(result)
}

pub(crate) fn calc_clamp(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(v), Value::Int(lo), Value::Int(hi)] =>
            Ok(Value::Int((*v).clamp(*lo, *hi))),
        [v, lo, hi] => {
            let vf  = coerce_to_f64(v,  "calc.clamp() value")?;
            let lof = coerce_to_f64(lo, "calc.clamp() min")?;
            let hif = coerce_to_f64(hi, "calc.clamp() max")?;
            if lof > hif {
                return err(format!("calc.clamp() min ({lof}) > max ({hif})"));
            }
            Ok(Value::Float(vf.clamp(lof, hif)))
        }
        _ => err(format!("calc.clamp() requer 3 argumentos, recebeu {}", args.items.len())),
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
fn unary_f64(
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

pub(crate) fn calc_sin(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.sin", args, f64::sin)
}

pub(crate) fn calc_cos(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.cos", args, f64::cos)
}

pub(crate) fn calc_tan(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.tan", args, f64::tan)
}

pub(crate) fn calc_asin(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.asin()")?;
            if !(-1.0..=1.0).contains(&x) {
                return err(format!("calc.asin() valor deve estar entre -1 e 1, recebeu {x}"));
            }
            trig_op(f64::asin, x)
        }
        _ => err(format!("calc.asin() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_acos(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.acos()")?;
            if !(-1.0..=1.0).contains(&x) {
                return err(format!("calc.acos() valor deve estar entre -1 e 1, recebeu {x}"));
            }
            trig_op(f64::acos, x)
        }
        _ => err(format!("calc.acos() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_atan(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.atan", args, f64::atan)
}

/// `calc.atan2(x, y)` — paridade vanilla na **ordem dos parâmetros** (`x` antes
/// de `y`); a stdlib Rust expõe `f64::atan2(y, x)` portanto a chamada interna
/// passa-os trocados (cf. diagnóstico §A.1).
pub(crate) fn calc_atan2(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [vx, vy] => {
            let x = coerce_to_f64(vx, "calc.atan2() x")?;
            let y = coerce_to_f64(vy, "calc.atan2() y")?;
            #[allow(clippy::disallowed_methods)]
            let r = f64::atan2(y, x);
            guard_float(r)
        }
        _ => err(format!("calc.atan2() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_sinh(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.sinh", args, f64::sinh)
}

pub(crate) fn calc_cosh(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.cosh", args, f64::cosh)
}

pub(crate) fn calc_tanh(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.tanh", args, f64::tanh)
}

pub(crate) fn calc_asinh(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.asinh", args, f64::asinh)
}

pub(crate) fn calc_acosh(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.acosh()")?;
            if x < 1.0 {
                return err(format!("calc.acosh() valor deve ser >= 1, recebeu {x}"));
            }
            trig_op(f64::acosh, x)
        }
        _ => err(format!("calc.acosh() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_atanh(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.atanh()")?;
            if x <= -1.0 || x >= 1.0 {
                return err(format!("calc.atanh() valor deve estar em (-1, 1), recebeu {x}"));
            }
            trig_op(f64::atanh, x)
        }
        _ => err(format!("calc.atanh() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_exp(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    unary_f64("calc.exp", args, f64::exp)
}

pub(crate) fn calc_ln(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.ln()")?;
            if x <= 0.0 {
                return err(format!("calc.ln() valor deve ser estritamente positivo, recebeu {x}"));
            }
            #[allow(clippy::disallowed_methods)]
            let r = f64::ln(x);
            guard_float(r)
        }
        _ => err(format!("calc.ln() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `calc.log(x)` (base 10) ou `calc.log(x, base)`. Vanilla usa argumento nomeado
/// `base:` (default 10); cristalino diverge para posicional por simplicidade
/// — registado no L0 e em diagnóstico §A.1.
pub(crate) fn calc_log(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let (x, base) = match args.items.as_slice() {
        [v]    => (coerce_to_f64(v, "calc.log() valor")?, 10.0_f64),
        [v, b] => (
            coerce_to_f64(v, "calc.log() valor")?,
            coerce_to_f64(b, "calc.log() base")?,
        ),
        _ => return err(format!("calc.log() requer 1 ou 2 argumentos, recebeu {}", args.items.len())),
    };
    if x <= 0.0 {
        return err(format!("calc.log() valor deve ser estritamente positivo, recebeu {x}"));
    }
    if !base.is_finite() || base <= 0.0 || base == 1.0 {
        return err(format!("calc.log() base inválida: {base}"));
    }
    #[allow(clippy::disallowed_methods)]
    let r = f64::ln(x) / f64::ln(base);
    guard_float(r)
}

fn coerce_to_f64(v: &Value, ctx: &str) -> SourceResult<f64> {
    match v {
        Value::Int(i)   => Ok(*i as f64),
        Value::Float(f) => Ok(*f),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{ctx}: esperava Int ou Float, recebeu {}", other.type_name()),
        )]),
    }
}

fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan()           { err("resultado não é um número (NaN)") }
    else if f.is_infinite() { err("resultado é infinito") }
    else                    { Ok(Value::Float(f)) }
}

// ── `upper()` / `lower()` / `replace()` — motor map_text (Passo 67) ─────────

