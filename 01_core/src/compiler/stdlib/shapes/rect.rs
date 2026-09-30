//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/shapes/rect.md
//! @prompt-hash 6e3e8492
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `rect(width?, height?, fill?, stroke?, radius?)` → `Content::Shape`.
///
/// Fallback determinístico: sem `fill` nem `stroke` → stroke preta de 1pt.
/// Este é o único local onde este fallback existe — nem o layouter nem o
/// exportador têm permissão para inventar cores ou espessuras.
pub fn native_rect(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke", "radius"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em rect(): '{}'", key),
            )]);
        }
    }

    let width = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    // A forma por-lado é aceita pela superfície existente, mas a entidade
    // uniforme ainda não a representa; preserva o scope-out anterior.
    let parsed_stroke = match args.named.get("stroke") {
        Some(Value::Dict(_)) => None,
        Some(value) => Some(parse_shape_stroke(value, "rect")?),
        None => None,
    };

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let radius = match args.named.get("radius") {
        Some(val) => {
            crate::compiler::stdlib::layout::extract_corners_length_value(val, "rect")?
        }
        None => Corners::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::shape_with_radius(
        ShapeKind::Rect,
        width,
        height,
        fill,
        final_stroke,
        radius,
    )))
}

/// `square(width, height: auto, fill?, stroke?, radius?)` → `Content::Shape { kind: Rect, ... }`.
///
/// Helper morfológico sobre `Rect`: `square(w)` é equivalente a
/// `rect(width: w, height: w)`. Se `height` for omitido, assume o valor de
/// `width`. O fallback de stroke preta 1pt segue o mesmo determinismo de
/// `native_rect`.
pub fn native_square(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke", "radius"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em square(): '{}'", key),
            )]);
        }
    }

    let width = if let Some(w) = args.items.first() {
        Some(Box::new(w.clone()))
    } else {
        args.named.get("width").cloned().map(Box::new)
    };

    let Some(width) = width else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "square() requer um argumento de largura",
        )]);
    };

    let height = args
        .named
        .get("height")
        .cloned()
        .map(Box::new)
        .unwrap_or_else(|| width.clone());

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "square"))
        .transpose()?;

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let radius = match args.named.get("radius") {
        Some(val) => {
            crate::compiler::stdlib::layout::extract_corners_length_value(val, "square")?
        }
        None => Corners::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::shape_with_radius(
        ShapeKind::Rect,
        Some(width),
        Some(height),
        fill,
        final_stroke,
        radius,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        assert!(matches!(
            call(native_rect, Args::positional(vec![])).unwrap(),
            Value::Content(Content::Shape(_))
        ));
        assert!(matches!(
            call(native_square, Args::positional(vec![Value::Length(Length::pt(4.0))]))
                .unwrap(),
            Value::Content(Content::Shape(_))
        ));
    }
}
