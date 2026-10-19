//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/ellipse.md
//! @prompt-hash 5d69dfe0
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `ellipse(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, ... }`.
///
/// Mesmo padrão de fallback que `native_rect`.
pub fn native_ellipse(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em ellipse(): '{}'", key),
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

    let parsed_stroke = args
        .named
        .get("stroke")
        .filter(|value| !matches!(value, Value::None))
        .map(|value| parse_shape_stroke(value, "ellipse"))
        .transpose()?;

    let final_stroke = if fill.is_none()
        && parsed_stroke.is_none()
        && !matches!(args.named.get("stroke"), Some(Value::None))
    {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::shape(
        ShapeKind::Ellipse,
        width,
        height,
        fill,
        final_stroke,
    )))
}

/// `circle(radius?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, width==height }`.
///
/// `radius` em pt. Converte para `width = height = radius * 2`.
pub fn native_circle(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["radius", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em circle(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0, // neutro: N16[β] — Value não-numérico retorna 0.0 na extracção de dimensões de polígono
        }
    }

    let (width, height) = match args.named.get("radius") {
        Some(r) => {
            let diameter = Value::Float(extract_pt(r) * 2.0);
            (Some(Box::new(diameter.clone())), Some(Box::new(diameter)))
        }
        None => (None, None),
    };

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .filter(|value| !matches!(value, Value::None))
        .map(|value| parse_shape_stroke(value, "circle"))
        .transpose()?;

    let final_stroke = if fill.is_none()
        && parsed_stroke.is_none()
        && !matches!(args.named.get("stroke"), Some(Value::None))
    {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::shape(
        ShapeKind::Ellipse,
        width,
        height,
        fill,
        final_stroke,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        assert!(matches!(
            call(native_ellipse, Args::positional(vec![])).unwrap(),
            Value::Content(Content::Shape(_))
        ));
        assert!(matches!(
            call(native_circle, Args::positional(vec![])).unwrap(),
            Value::Content(Content::Shape(_))
        ));
    }
}

#[cfg(test)]
mod optional_stroke_regressions {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn explicit_none_stays_absent_with_and_without_fill() {
        for fill in [None, Some(Value::Color(Color::rgb(0, 0, 0)))] {
            let mut args = Args::positional(vec![]);
            args.named.insert("stroke".into(), Value::None);
            if let Some(fill) = fill {
                args.named.insert("fill".into(), fill);
            }
            let Value::Content(Content::Shape(shape)) =
                call(native_ellipse, args).unwrap()
            else {
                panic!("expected shape");
            };
            assert!(shape.stroke.is_none());
        }
        let Value::Content(Content::Shape(shape)) =
            call(native_ellipse, Args::positional(vec![])).unwrap()
        else {
            panic!("expected shape");
        };
        assert!(shape.stroke.is_some(), "omission keeps the unfilled default");
        for fill in [None, Some(Value::Color(Color::rgb(0, 0, 0)))] {
            let mut args = Args::positional(vec![]);
            args.named.insert("stroke".into(), Value::None);
            if let Some(fill) = fill {
                args.named.insert("fill".into(), fill);
            }
            let Value::Content(Content::Shape(shape)) =
                call(native_circle, args).unwrap()
            else {
                panic!("expected shape");
            };
            assert!(shape.stroke.is_none());
        }
        let Value::Content(Content::Shape(shape)) =
            call(native_circle, Args::positional(vec![])).unwrap()
        else {
            panic!("expected shape");
        };
        assert!(shape.stroke.is_some(), "omission keeps the unfilled default");
    }
}
