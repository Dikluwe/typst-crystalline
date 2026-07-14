//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/color.md
//! @prompt-hash e64cf59f
//! @layer L1
//! @updated 2026-06-27
//!
//! Módulo `color` — operadores de cor: `lighten`, `darken`, `mix`, `negate`.
//! P476 — fecho parcial ADR-0083 §"Operadores cor" scope-out (4/6 implementados).

use ecow::EcoString;

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

/// Devolve o valor associado a `field` no tipo `color` (P736).
///
/// `color` é `Value::Type(Type::Color)` no scope global (paridade vanilla
/// — medido: `type(color)` → `type`); os fields resolvem-se por field
/// access em `Value::Type` (`eval/bindings.rs`), que delega aqui.
///
/// 14 fields: 8 constructors (as mesmas nativas registadas globalmente:
/// `rgb`, `linear-rgb`, `luma`, `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`)
/// + 6 operadores P476/P477. `None` para campo inexistente — o chamador
/// emite "type color does not contain field `<f>`" (verbatim vanilla).
///
/// Histórico: até P736 era `make_color_module() -> Value` (`Value::Dict`
/// com os 6 operadores; os constructors não eram acessíveis via `color.*`).
pub fn color_type_field(field: &str) -> Option<Value> {
    use super::foundations::{
        native_cmyk, native_hsl, native_hsv, native_linear_rgb, native_luma,
        native_oklab, native_oklch, native_rgb,
    };
    Some(match field {
        "rgb" => Value::Func(Func::native("color.rgb", native_rgb)),
        "linear-rgb" => Value::Func(Func::native("color.linear-rgb", native_linear_rgb)),
        "luma" => Value::Func(Func::native("color.luma", native_luma)),
        "cmyk" => Value::Func(Func::native("color.cmyk", native_cmyk)),
        "hsl" => Value::Func(Func::native("color.hsl", native_hsl)),
        "hsv" => Value::Func(Func::native("color.hsv", native_hsv)),
        "oklab" => Value::Func(Func::native("color.oklab", native_oklab)),
        "oklch" => Value::Func(Func::native("color.oklch", native_oklch)),
        "lighten" => Value::Func(Func::native("color.lighten", native_color_lighten)),
        "darken" => Value::Func(Func::native("color.darken", native_color_darken)),
        "mix" => Value::Func(Func::native("color.mix", native_color_mix)),
        "negate" => Value::Func(Func::native("color.negate", native_color_negate)),
        "saturate" => Value::Func(Func::native("color.saturate", native_color_saturate)),
        "desaturate" => Value::Func(Func::native("color.desaturate", native_color_desaturate)),
        _ => return None,
    })
}

/// Cores nomeadas globais para injeção no scope de eval.
///
/// **P687** — paridade vanilla 0.15.0 (969087ec): as 18 cores oficiais
/// (`lib.rs:359-376` / `visualize/color.rs:291-322`) com bytes sRGB exactos
/// (confirmados via `#repr`). `cyan`/`magenta`/`none` são extras pré-P687 mantidos
/// sem regressão (não fazem parte do conjunto oficial vanilla).
pub fn predefined_color_bindings() -> Vec<(EcoString, Value)> {
    vec![
        // ── 18 cores oficiais vanilla (sRGB exacto) ────────────────────────
        ("black".into(),   Value::Color(Color::rgb(0x00, 0x00, 0x00))),
        ("gray".into(),    Value::Color(Color::rgb(0xAA, 0xAA, 0xAA))),
        ("silver".into(),  Value::Color(Color::rgb(0xDD, 0xDD, 0xDD))),
        ("white".into(),   Value::Color(Color::rgb(0xFF, 0xFF, 0xFF))),
        ("navy".into(),    Value::Color(Color::rgb(0x00, 0x1F, 0x3F))),
        ("blue".into(),    Value::Color(Color::rgb(0x00, 0x74, 0xD9))),
        ("aqua".into(),    Value::Color(Color::rgb(0x7F, 0xDB, 0xFF))),
        ("teal".into(),    Value::Color(Color::rgb(0x39, 0xCC, 0xCC))),
        ("eastern".into(), Value::Color(Color::rgb(0x23, 0x9D, 0xAD))),
        ("purple".into(),  Value::Color(Color::rgb(0xB1, 0x0D, 0xC9))),
        ("fuchsia".into(), Value::Color(Color::rgb(0xF0, 0x12, 0xBE))),
        ("maroon".into(),  Value::Color(Color::rgb(0x85, 0x14, 0x4B))),
        ("red".into(),     Value::Color(Color::rgb(0xFF, 0x41, 0x36))),
        ("orange".into(),  Value::Color(Color::rgb(0xFF, 0x85, 0x1B))),
        ("yellow".into(),  Value::Color(Color::rgb(0xFF, 0xDC, 0x00))),
        ("olive".into(),   Value::Color(Color::rgb(0x3D, 0x99, 0x70))),
        ("green".into(),   Value::Color(Color::rgb(0x2E, 0xCC, 0x40))),
        ("lime".into(),    Value::Color(Color::rgb(0x01, 0xFF, 0x70))),
        // ── extras pré-P687 (não-vanilla; sem regressão) ───────────────────
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

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes_of(name: &str) -> Option<(u8, u8, u8)> {
        for (n, v) in predefined_color_bindings() {
            if n == name {
                if let Value::Color(c) = v {
                    let (r, g, b, _a) = c.to_srgb();
                    return Some((r, g, b));
                }
            }
        }
        None
    }

    fn has(name: &str) -> bool {
        predefined_color_bindings().iter().any(|(n, _)| n == name)
    }

    #[test]
    fn p687_cores_vanilla_18_srgb_exacto() {
        let esperado: [(&str, (u8, u8, u8)); 18] = [
            ("black",   (0x00, 0x00, 0x00)),
            ("gray",    (0xAA, 0xAA, 0xAA)),
            ("silver",  (0xDD, 0xDD, 0xDD)),
            ("white",   (0xFF, 0xFF, 0xFF)),
            ("navy",    (0x00, 0x1F, 0x3F)),
            ("blue",    (0x00, 0x74, 0xD9)),
            ("aqua",    (0x7F, 0xDB, 0xFF)),
            ("teal",    (0x39, 0xCC, 0xCC)),
            ("eastern", (0x23, 0x9D, 0xAD)),
            ("purple",  (0xB1, 0x0D, 0xC9)),
            ("fuchsia", (0xF0, 0x12, 0xBE)),
            ("maroon",  (0x85, 0x14, 0x4B)),
            ("red",     (0xFF, 0x41, 0x36)),
            ("orange",  (0xFF, 0x85, 0x1B)),
            ("yellow",  (0xFF, 0xDC, 0x00)),
            ("olive",   (0x3D, 0x99, 0x70)),
            ("green",   (0x2E, 0xCC, 0x40)),
            ("lime",    (0x01, 0xFF, 0x70)),
        ];
        for (nome, bytes) in esperado {
            assert_eq!(
                bytes_of(nome),
                Some(bytes),
                "cor '{nome}' deve estar ligada com sRGB exacto {bytes:?}"
            );
        }
    }

    #[test]
    fn p687_cores_inexistentes_no_vanilla_ausentes() {
        // Confirmado por sonda: vanilla 0.15.0 não tem `ostrich` nem `pink`.
        assert!(!has("ostrich"));
        assert!(!has("pink"));
    }

    #[test]
    fn p687_extras_pre_p687_sem_regressao() {
        // Mantidos para não regredir documentos/testes anteriores a P687.
        assert_eq!(bytes_of("cyan"), Some((0x00, 0xB3, 0xB3)));
        assert_eq!(bytes_of("magenta"), Some((0xE5, 0x00, 0xE5)));
        assert!(has("none"));
    }
}
