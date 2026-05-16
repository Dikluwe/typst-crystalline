//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 68fc3823
//! @layer L1
//! @updated 2026-05-15
//!
//! Stdlib `gradient.linear` — P262 (ADR-0087 Gradient Linear-only).
//!
//! Constrói `Value::Gradient(Gradient::Linear)` a partir de stops
//! variadic positional + named `angle: Angle`.
//!
//! Aceita formatos de stop:
//! - `Color` directo (offset = None; auto-spacing).
//! - `Array [Color, Ratio]` (offset explícito).
//!
//! ColorSpace fixo Oklab (scope-out ADR-0087 — paridade vanilla
//! default). Interpolação L1 via `Linear::sample(t)`.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::gradient::{Gradient, GradientStop};
use crate::entities::layout_types::{Angle, Ratio};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

/// Constrói o módulo `gradient` como `Value::Dict` (paridade
/// `make_calc_module`). Acesso `gradient.linear(...)` via
/// `eval_field_access` sobre Dict.
pub fn make_gradient_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("linear".into(),
        Value::Func(Func::native("gradient.linear", native_gradient_linear)));
    Value::Dict(dict)
}

/// `gradient.linear(stops..., angle: ?)` → `Value::Gradient(Gradient::Linear)`.
pub fn native_gradient_linear(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let stops = parse_stops(&args.items)?;
    if stops.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "gradient.linear: pelo menos 1 stop requerido".to_string(),
        )]);
    }

    let angle = match args.named.get("angle") {
        Some(Value::Angle(a)) => *a,
        Some(Value::Float(f)) => Angle::rad(*f),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("gradient.linear(angle): espera Angle ou Float, recebeu {}",
                    other.type_name()),
        )]),
        None => Angle::rad(0.0),
    };

    for key in args.named.keys() {
        if key != "angle" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.linear: argumento nomeado inesperado '{}' (esperado: angle)", key),
            )]);
        }
    }

    Ok(Value::Gradient(Gradient::linear(stops, angle)))
}

/// Parse cada item posicional para `GradientStop`.
///
/// Aceita:
/// - `Value::Color(c)` → `GradientStop::unspaced(c)` (offset auto).
/// - `Value::Array([Color, Ratio])` → `GradientStop::new(c, r)`.
fn parse_stops(items: &[Value]) -> SourceResult<Vec<GradientStop>> {
    let mut stops = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        let stop = match item {
            Value::Color(c) => GradientStop::unspaced(*c),
            Value::Array(arr) if arr.len() == 2 => {
                let c = match &arr[0] {
                    Value::Color(c) => *c,
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("gradient.linear: stop {}: primeiro elemento deve ser Color, recebeu {}",
                                i, other.type_name()),
                    )]),
                };
                let r = match &arr[1] {
                    Value::Ratio(r) => *r,
                    Value::Float(f) => Ratio(*f),
                    Value::Int(i64v) => Ratio(*i64v as f64),
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("gradient.linear: stop {}: segundo elemento deve ser Ratio/Float, recebeu {}",
                                i, other.type_name()),
                    )]),
                };
                if r.0 < 0.0 || r.0 > 1.0 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("gradient.linear: stop {}: offset {} fora de [0, 1]", i, r.0),
                    )]);
                }
                GradientStop::new(c, r)
            }
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("gradient.linear: stop {} deve ser Color ou [Color, Ratio], recebeu {}",
                        i, other.type_name()),
            )]),
        };
        stops.push(stop);
    }
    Ok(stops)
}

// Tests para `native_gradient_linear` em `01_core/src/rules/stdlib/mod.rs`
// tests module (usa `null_ctx!` + `null_world()` + `test_file_id()`
// helpers existentes).
