//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/transforms.md
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de transformações (move, rotate, scale).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::TransformMatrix;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// `move(dx?, dy?, body)` → `Content::Transform { matrix: translate(dx, dy), body }`.
pub fn native_move(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0, // neutro: N16[β] — Value não-numérico retorna 0.0 na extracção de deslocamento
        }
    }
    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);
    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "move() exige um corpo de conteúdo".to_string(),
            )]
        })?;
    Ok(Value::Content(Content::transform(TransformMatrix::translate(dx, dy), body)))
}

/// `rotate(angle, body)` → `Content::Transform { matrix: rotate(rad), body }`.
///
/// `angle` pode ser `Value::Angle` (graus→radianos via `to_rad()`) ou `Value::Float` (radianos).
pub fn native_rotate(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let angle_rad = match args.named.get("angle") {
        Some(Value::Angle(a)) => a.to_rad(),
        Some(Value::Float(f)) => *f,
        _ => {
            // Fallback: primeiro arg posicional que seja Angle ou Float.
            match args
                .items
                .iter()
                .find(|v| matches!(v, Value::Angle(_) | Value::Float(_)))
            {
                Some(Value::Angle(a)) => a.to_rad(),
                Some(Value::Float(f)) => *f,
                _ => 0.0,
            }
        }
    };
    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "rotate() exige um corpo de conteúdo".to_string(),
            )]
        })?;
    Ok(Value::Content(Content::transform(TransformMatrix::rotate(angle_rad), body)))
}

/// `scale(x?, y?, body)` → `Content::Transform { matrix: scale(sx, sy), body }`.
///
/// `x` e `y` são factores de escala (Float, Int ou `Ratio`), **posicionais ou
/// nomeados** (o vanilla liga o primeiro posicional a `x` — `ScaleElem.x` com
/// `#[positional]`, P953). Se `y` omitido, escala uniforme.
pub fn native_scale(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    fn extract_factor(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            // **P953** — `Ratio` é o tipo natural do factor (`#scale(150%)`).
            Value::Ratio(r) => r.0,
            // intencional: fatores não-numéricos preservam o fator de escala identidade (1.0)
            Value::None
            | Value::Bool(_)
            | Value::Str(_)
            | Value::Array(_)
            | Value::Dict(_)
            | Value::Module(_)
            | Value::Datetime(_)
            | Value::Func(_)
            | Value::Content(_)
            | Value::Auto
            | Value::Length(_)
            | Value::Relative(_)
            | Value::Angle(_)
            | Value::Color(_)
            | Value::Stroke(_)
            | Value::Fraction(_)
            | Value::Align(_)
            | Value::Location(_)
            | Value::Gradient(_)
            | Value::Regex(_)
            | Value::Tiling(_)
            | Value::Bytes(_)
            | Value::Decimal(_)
            | Value::Duration(_)
            | Value::Version(_)
            | Value::Selector(_)
            | Value::Symbol(_)
            | Value::Args(_)
            | Value::State(_)
            | Value::Counter(_)
            | Value::Label(_)
            | Value::Dir(_)
            | Value::Type(_) => 1.0,
        }
    }
    let mut posicionais = args
        .items
        .iter()
        .filter(|v| matches!(v, Value::Float(_) | Value::Int(_) | Value::Ratio(_)));
    let sx = args
        .named
        .get("x")
        .map(extract_factor)
        .or_else(|| posicionais.next().map(extract_factor))
        .unwrap_or(1.0);
    let sy = args
        .named
        .get("y")
        .map(extract_factor)
        .or_else(|| posicionais.next().map(extract_factor))
        .unwrap_or(sx);
    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "scale() exige um corpo de conteúdo".to_string(),
            )]
        })?;
    Ok(Value::Content(Content::transform(TransformMatrix::scale(sx, sy), body)))
}

// ── Passo 156F (ADR-0061 Fase 1 sub-passo 4) — skew via TransformMatrix ─────

/// `skew(ax?, ay?, body)` → `Content::Transform { matrix: skew(ax, ay), body }`.
///
/// Distorção (skew) com ângulos `ax` (horizontal) e `ay` (vertical).
/// Análogo a vanilla `SkewElem`; cristalino reusa arquitectura matriz cm
/// já estabelecida em P78 (sem novo variant Content nem TransformKind).
///
/// **Atributos**:
/// - `ax: Angle` — ângulo de distorção horizontal (default 0).
/// - `ay: Angle` — ângulo de distorção vertical (default 0).
/// - `body` — posicional obrigatório (Content).
///
/// **Limitações conscientes (P156F)**:
/// - `origin` (ponto de pivot) scope-out; análogo aos move/rotate/scale
///   actuais que também não têm `origin` (refino futuro per ADR-0061).
/// - Ângulos próximos de ±π/2 produzem `tan` infinito; rejeitados via
///   validação `|ax|, |ay| < π/2 - epsilon` (epsilon=1e-3 rad ≈ 0.057°).
/// - Ângulos negativos aceites (skew em direcção oposta — semantic
///   simétrica natural).
pub fn native_skew(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    /// Coage `Value` para ângulo em radianos. Aceita `Angle` (idiomático)
    /// e `Float` (radianos directos, consistente com `native_rotate`).
    fn extract_angle_rad(val: &Value) -> Option<f64> {
        match val {
            Value::Angle(a) => Some(a.to_rad()),
            Value::Float(f) => Some(*f),
            _ => None, // neutro: N16[β] — Value não-angular retorna None na extracção de ângulo
        }
    }

    let ax_rad = match args.named.get("ax") {
        Some(v) => extract_angle_rad(v).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                format!("skew(ax:) espera angle, recebeu {}", v.type_name()),
            )]
        })?,
        None => 0.0,
    };
    let ay_rad = match args.named.get("ay") {
        Some(v) => extract_angle_rad(v).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                format!("skew(ay:) espera angle, recebeu {}", v.type_name()),
            )]
        })?,
        None => 0.0,
    };

    // Rejeitar named args desconhecidos (consistente com pad/h/v/pagebreak).
    for key in args.named.keys() {
        if !["ax", "ay"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("skew(): argumento nomeado inesperado '{}'", key),
            )]);
        }
    }

    // Validação: rejeitar ângulos próximos de ±π/2 onde tan diverge.
    // Threshold 0.001 rad (~0.057°) abaixo de π/2 deixa margem para
    // valores razoáveis (até ~89.94°) e rejeita só os divergentes.
    const LIMIT: f64 = std::f64::consts::FRAC_PI_2 - 1e-3;
    if ax_rad.abs() >= LIMIT || ay_rad.abs() >= LIMIT {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "skew(): ângulo demasiado próximo de ±π/2 (tan diverge)".to_string(),
        )]);
    }

    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "skew() exige um corpo de conteúdo".to_string(),
            )]
        })?;

    Ok(Value::Content(Content::transform(TransformMatrix::skew(ax_rad, ay_rad), body)))
}
