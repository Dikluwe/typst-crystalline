//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/color.md
//! @prompt-hash 1c9dbcc1
//! @layer L1
//! @updated 2026-06-27
//!
//! Módulo `color` — operadores de cor: `lighten`, `darken`, `mix`, `negate`.
//! P476 — fecho parcial ADR-0083 §"Operadores cor" scope-out (4/6 implementados).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::layout_types::Color;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

use super::err;

fn err_typed<T>(msg: impl Into<String>) -> SourceResult<T> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// Constrói o módulo `color` como `Value::Dict` com 6 operadores de cor.
///
/// P476: lighten, darken, mix, negate.
/// P477: saturate, desaturate — fecha ADR-0083 §"Operadores cor" totalmente.
pub fn make_color_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("lighten".into(),    Value::Func(Func::native("color.lighten",    native_color_lighten)));
    dict.insert("darken".into(),     Value::Func(Func::native("color.darken",     native_color_darken)));
    dict.insert("mix".into(),        Value::Func(Func::native("color.mix",        native_color_mix)));
    dict.insert("negate".into(),     Value::Func(Func::native("color.negate",     native_color_negate)));
    dict.insert("saturate".into(),   Value::Func(Func::native("color.saturate",   native_color_saturate)));
    dict.insert("desaturate".into(), Value::Func(Func::native("color.desaturate", native_color_desaturate)));
    Value::Dict(dict)
}

/// P492 — bindings de cores predefinidas para injeção no scope global.
///
/// Vanilla expõe `red`, `blue`, `green`, etc. como atalhos globais para as
/// cores do módulo `color`. O subset abaixo cobre os gaps D4/D5 do P490.
pub fn predefined_color_bindings() -> Vec<(EcoString, Value)> {
    vec![
        ("red".into(),     Value::Color(Color::rgb(0xEF, 0x23, 0x11))),
        ("blue".into(),    Value::Color(Color::rgb(0x00, 0x5E, 0xE5))),
        ("green".into(),   Value::Color(Color::rgb(0x00, 0xB3, 0x00))),
        ("black".into(),   Value::Color(Color::rgb(0x00, 0x00, 0x00))),
        ("white".into(),   Value::Color(Color::rgb(0xFF, 0xFF, 0xFF))),
        ("yellow".into(),  Value::Color(Color::rgb(0xF5, 0xD8, 0x00))),
        ("cyan".into(),    Value::Color(Color::rgb(0x00, 0xB3, 0xB3))),
        ("magenta".into(), Value::Color(Color::rgb(0xE5, 0x00, 0xE5))),
        ("none".into(),    Value::None),
    ]
}

fn extract_color_arg(val: &Value, fn_name: &str, arg_name: &str) -> SourceResult<Color> {
    match val {
        Value::Color(c) => Ok(*c),
        other => err_typed(format!(
            "{}: argumento '{}' deve ser Color, recebeu {}",
            fn_name, arg_name, other.type_name()
        )),
    }
}

fn extract_ratio_arg(val: &Value, fn_name: &str, arg_name: &str) -> SourceResult<f32> {
    match val {
        Value::Float(f)   => Ok(*f as f32),
        Value::Int(i)     => Ok(*i as f32 / 100.0),
        Value::Relative(r) if r.abs.is_zero() => Ok(r.rel as f32),
        other => err_typed(format!(
            "{}: argumento '{}' deve ser Float ou Percentage, recebeu {}",
            fn_name, arg_name, other.type_name()
        )),
    }
}

/// `color.lighten(col, amount)` — aumenta luminância por `amount` via Oklch.
pub(crate) fn native_color_lighten(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.lighten() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col,    "color.lighten", "col")?;
            let a = extract_ratio_arg(amount, "color.lighten", "amount")?;
            Ok(Value::Color(c.lighten(a)))
        }
        _ => err(format!(
            "color.lighten() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.darken(col, amount)` — diminui luminância por `amount` via Oklch.
pub(crate) fn native_color_darken(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.darken() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col,    "color.darken", "col")?;
            let a = extract_ratio_arg(amount, "color.darken", "amount")?;
            Ok(Value::Color(c.darken(a)))
        }
        _ => err(format!(
            "color.darken() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.mix(col1, col2, weight: 0.5)` — interpolação linear em Oklab.
pub(crate) fn native_color_mix(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let weight = match args.named.get("weight") {
        Some(v) => extract_ratio_arg(v, "color.mix", "weight")?,
        None    => 0.5_f32,
    };
    for key in args.named.keys() {
        if key.as_str() != "weight" {
            return err(format!("color.mix(): argumento nomeado inesperado '{}'", key));
        }
    }
    match args.items.as_slice() {
        [col1, col2] => {
            let c1 = extract_color_arg(col1, "color.mix", "col1")?;
            let c2 = extract_color_arg(col2, "color.mix", "col2")?;
            Ok(Value::Color(c1.mix(c2, weight)))
        }
        _ => err(format!(
            "color.mix() requer 2 argumentos posicionais (col1, col2), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.negate(col)` — complementar em sRGB `(1-r, 1-g, 1-b)`.
pub(crate) fn native_color_negate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.negate() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col] => {
            let c = extract_color_arg(col, "color.negate", "col")?;
            Ok(Value::Color(c.negate()))
        }
        _ => err(format!(
            "color.negate() requer 1 argumento (col), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.saturate(col, amount)` — aumenta chroma Oklch por `amount`.
pub(crate) fn native_color_saturate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.saturate() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col,    "color.saturate", "col")?;
            let a = extract_ratio_arg(amount, "color.saturate", "amount")?;
            Ok(Value::Color(c.saturate(a)))
        }
        _ => err(format!(
            "color.saturate() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.desaturate(col, amount)` — diminui chroma Oklch por `amount`.
pub(crate) fn native_color_desaturate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.desaturate() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col,    "color.desaturate", "col")?;
            let a = extract_ratio_arg(amount, "color.desaturate", "amount")?;
            Ok(Value::Color(c.desaturate(a)))
        }
        _ => err(format!(
            "color.desaturate() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}
