//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/paint.md
//! @prompt-hash c60d4a59
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ── Primitivas geométricas (Passo 76) ────────────────────────────────────────

/// Extrai somente uma cor já tipada pelo avaliador.
pub(in crate::compiler::stdlib) fn parse_color(val: &Value) -> Option<Color> {
    match val {
        Value::Color(c) => Some(c.clone()),
        _ => None,
    }
}

/// Converte um `Value` em `Paint` (fill de shape).
pub(in crate::compiler::stdlib) fn parse_paint(
    val: &Value,
    span: Span,
) -> SourceResult<Option<Paint>> {
    match val {
        Value::Color(c) => Ok(Some(Paint::Solid(c.clone()))),
        Value::Tiling(t) => Ok(Some(Paint::Tiling((**t).clone()))),
        Value::Gradient(g) => Ok(Some(Paint::Gradient(g.clone()))),
        Value::None => Ok(None),
        Value::Str(_) => Err(vec![SourceDiagnostic::error(
            span,
            "expected color, gradient, tiling, or none, found string".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected color, gradient, tiling, or none, found {}",
                other.type_name()
            ),
        )]),
    }
}

/// Preserva o stroke rico já produzido pelo avaliador.
pub(super) fn parse_shape_stroke(val: &Value, fn_name: &str) -> SourceResult<Stroke> {
    if matches!(val, Value::Gradient(_) | Value::Tiling(_) | Value::Dict(_)) {
        return crate::compiler::stdlib::layout::positional_stroke(val, Span::detached());
    }
    if let Some(color) = parse_color(val) {
        return Ok(Stroke {
            paint: Paint::Solid(color),
            thickness: 1.0,
            overhang: true,
            ..Stroke::default()
        });
    }
    if matches!(val, Value::Str(_)) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "expected length, color, gradient, tiling, dictionary, or stroke, found string"
                .to_string(),
        )]);
    }
    crate::compiler::stdlib::layout::extract_stroke(val, fn_name, "stroke")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        assert_eq!(parse_color(&Value::Str("red".into())), None);
        assert!(parse_paint(&Value::Color(Color::rgb(1, 2, 3)), Span::detached())
            .unwrap()
            .is_some());
    }
}
