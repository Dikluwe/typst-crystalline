//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de transformações (move, rotate, scale).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::content::Content;
use crate::entities::layout_types::TransformMatrix;
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

/// `move(dx?, dy?, body)` → `Content::Transform { matrix: translate(dx, dy), body }`.
pub fn native_move(_ctx: &mut EvalContext<'_>, args: &Args, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f)  => *f,
            Value::Int(i)    => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }
    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);
    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "move() exige um corpo de conteúdo".to_string())])?;
    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::translate(dx, dy),
        body:   Box::new(body),
    }))
}

/// `rotate(angle, body)` → `Content::Transform { matrix: rotate(rad), body }`.
///
/// `angle` pode ser `Value::Angle` (graus→radianos via `to_rad()`) ou `Value::Float` (radianos).
pub fn native_rotate(_ctx: &mut EvalContext<'_>, args: &Args, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let angle_rad = match args.named.get("angle") {
        Some(Value::Angle(a)) => a.to_rad(),
        Some(Value::Float(f)) => *f,
        _ => {
            // Fallback: primeiro arg posicional que seja Angle ou Float.
            match args.items.iter().find(|v| matches!(v, Value::Angle(_) | Value::Float(_))) {
                Some(Value::Angle(a)) => a.to_rad(),
                Some(Value::Float(f)) => *f,
                _ => 0.0,
            }
        }
    };
    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "rotate() exige um corpo de conteúdo".to_string())])?;
    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::rotate(angle_rad),
        body:   Box::new(body),
    }))
}

/// `scale(x?, y?, body)` → `Content::Transform { matrix: scale(sx, sy), body }`.
///
/// `x` e `y` são factores de escala (Float ou Int). Se `y` omitido, escala uniforme.
pub fn native_scale(_ctx: &mut EvalContext<'_>, args: &Args, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    fn extract_factor(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i)   => *i as f64,
            _               => 1.0,
        }
    }
    let sx = args.named.get("x").map(extract_factor).unwrap_or(1.0);
    let sy = args.named.get("y").map(extract_factor).unwrap_or(sx);
    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "scale() exige um corpo de conteúdo".to_string())])?;
    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::scale(sx, sy),
        body:   Box::new(body),
    }))
}
