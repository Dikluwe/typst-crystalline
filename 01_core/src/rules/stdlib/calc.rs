//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f45bcc3a
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

/// Constrói o módulo `calc` como `Value::Dict` com 9 funções.
///
/// Divergência: original usa `Value::Module`. Cristalino usa `Value::Dict`
/// porque não temos stdlib Module sem world. Semântica de acesso (`calc.abs`)
/// é idêntica via `eval_field_access` sobre Dict.
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

