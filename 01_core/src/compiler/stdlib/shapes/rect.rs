//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/rect.md
//! @prompt-hash 25a363db
//! @layer L1
//! @updated 2026-09-30

use super::*;

fn absent_stroke(value: &Value) -> bool {
    matches!(value, Value::None) || matches!(value, Value::Dict(dict) if dict.is_empty())
}

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

    let rect_stroke = rect_side_strokes(args.named.get("stroke"), args.span)?;
    let parsed_stroke = if rect_stroke.is_some() {
        None
    } else {
        args.named
            .get("stroke")
            .filter(|v| !absent_stroke(v))
            .map(|v| parse_shape_stroke(v, "rect"))
            .transpose()?
    };

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none()
        && parsed_stroke.is_none()
        && rect_stroke.is_none()
        && !args.named.get("stroke").is_some_and(absent_stroke)
    {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: crate::entities::layout_types::Length::pt(1.0),
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let rect_radius = relative_rect_radius(args.named.get("radius"), args.span)?;
    let radius = if rect_radius.is_some() {
        Corners::uniform(Length::ZERO)
    } else {
        match args.named.get("radius") {
            Some(value) => crate::compiler::stdlib::layout::extract_corners_length_value(
                value, "rect",
            )?,
            None => Corners::uniform(Length::ZERO),
        }
    };
    let mut content = Content::shape_with_radius(
        ShapeKind::Rect,
        width,
        height,
        fill,
        final_stroke,
        radius,
    );
    if let Content::Shape(shape) = &mut content {
        let shape = Arc::make_mut(shape);
        shape.rect_radius = rect_radius;
        shape.rect_stroke = rect_stroke;
    }
    Ok(Value::Content(content))
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
        .filter(|value| !absent_stroke(value))
        .map(|value| parse_shape_stroke(value, "square"))
        .transpose()?;

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none()
        && parsed_stroke.is_none()
        && !args.named.get("stroke").is_some_and(absent_stroke)
    {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: crate::entities::layout_types::Length::pt(1.0),
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
            let Value::Content(Content::Shape(shape)) = call(native_rect, args).unwrap()
            else {
                panic!("expected shape");
            };
            assert!(shape.stroke.is_none());
        }
        let Value::Content(Content::Shape(shape)) =
            call(native_rect, Args::positional(vec![])).unwrap()
        else {
            panic!("expected shape");
        };
        assert!(shape.stroke.is_some(), "omission keeps the unfilled default");
        for fill in [None, Some(Value::Color(Color::rgb(0, 0, 0)))] {
            let mut args = Args::positional(vec![Value::Length(Length::pt(20.0))]);
            args.named.insert("stroke".into(), Value::None);
            if let Some(fill) = fill {
                args.named.insert("fill".into(), fill);
            }
            let Value::Content(Content::Shape(shape)) =
                call(native_square, args).unwrap()
            else {
                panic!("expected shape");
            };
            assert!(shape.stroke.is_none());
        }
        let Value::Content(Content::Shape(shape)) =
            call(native_square, Args::positional(vec![Value::Length(Length::pt(20.0))]))
                .unwrap()
        else {
            panic!("expected shape");
        };
        assert!(shape.stroke.is_some(), "omission keeps the unfilled default");
    }
}

fn affine_radius(
    value: &Value,
    span: Span,
) -> SourceResult<crate::entities::rel::Rel<Length>> {
    use crate::entities::rel::Rel;
    let value = match value {
        Value::Length(l) => Rel { rel: 0.0, abs: *l },
        Value::Ratio(r) => Rel { rel: r.0, abs: Length::ZERO },
        Value::Relative(r) => *r,
        Value::Int(n) => Rel { rel: 0.0, abs: Length::pt(*n as f64) },
        Value::Float(n) => Rel { rel: 0.0, abs: Length::pt(*n) },
        Value::None
        | Value::Bool(..)
        | Value::Str(..)
        | Value::Array(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "rect(radius): expected relative length, found {}",
                    value.type_name()
                ),
            )])
        }
    };
    if value.rel < 0.0 || value.abs.abs.0 < 0.0 || value.abs.em < 0.0 {
        return Err(vec![SourceDiagnostic::error(
            span,
            "rect(radius): negativo rejeitado",
        )]);
    }
    Ok(value)
}
fn relative_rect_radius(
    value: Option<&Value>,
    span: Span,
) -> SourceResult<Option<Corners<crate::entities::rel::Rel<Length>>>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let has_relative = matches!(value, Value::Ratio(_) | Value::Relative(_))
        || matches!(value,Value::Dict(d) if d.values().any(|v|matches!(v,Value::Ratio(_) | Value::Relative(_))));
    if !has_relative {
        return Ok(None);
    }
    if let Value::Dict(d) = value {
        for key in d.keys() {
            if ![
                "top-left",
                "top-right",
                "bottom-right",
                "bottom-left",
                "top",
                "right",
                "bottom",
                "left",
                "rest",
            ]
            .contains(&key.as_str())
            {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("rect(radius): chave de canto inesperada '{key}'"),
                )]);
            }
        }
        // Validate every supplied value even if a more specific key overrides it.
        for v in d.values() {
            affine_radius(v, span)?;
        }
        let get = |keys: [&str; 4]| -> SourceResult<_> {
            keys.into_iter()
                .find_map(|k| d.get(k))
                .map(|v| affine_radius(v, span))
                .transpose()
                .map(|v| v.unwrap_or_default())
        };
        Ok(Some(Corners::new(
            get(["top-left", "top", "left", "rest"])?,
            get(["top-right", "top", "right", "rest"])?,
            get(["bottom-right", "bottom", "right", "rest"])?,
            get(["bottom-left", "bottom", "left", "rest"])?,
        )))
    } else {
        Ok(Some(Corners::uniform(affine_radius(value, span)?)))
    }
}
fn rect_side_strokes(
    value: Option<&Value>,
    span: Span,
) -> SourceResult<Option<crate::entities::sides::Sides<Option<Stroke>>>> {
    let Some(Value::Dict(d)) = value else {
        return Ok(None);
    };
    if !d.keys().any(|k| {
        ["left", "top", "right", "bottom", "x", "y", "rest"].contains(&k.as_str())
    }) {
        return Ok(None);
    }
    for (key, value) in d {
        if !["left", "top", "right", "bottom", "x", "y", "rest"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("rect(stroke): chave inesperada '{key}'"),
            )]);
        }
        if !absent_stroke(value) {
            parse_shape_stroke(value, "rect")?;
        }
    }
    let get = |keys: [&str; 3]| -> SourceResult<Option<Stroke>> {
        let value = keys.into_iter().find_map(|k| d.get(k));
        match value {
            None => Ok(None),
            Some(v) if absent_stroke(v) => Ok(None),
            Some(v) => parse_shape_stroke(v, "rect").map(Some),
        }
    };
    Ok(Some(crate::entities::sides::Sides::new(
        get(["left", "x", "rest"])?,
        get(["top", "y", "rest"])?,
        get(["right", "x", "rest"])?,
        get(["bottom", "y", "rest"])?,
    )))
}

#[cfg(test)]
mod issue8583_constructor_tests {
    use super::super::atomization_test_support::call;
    use super::*;
    #[test]
    fn issue8583_preserves_affine_corners_and_side_precedence() {
        let mut args = Args::positional(vec![]);
        let radius = Value::Relative(crate::entities::rel::Rel {
            rel: 0.5,
            abs: Length {
                abs: crate::entities::layout_types::Abs(2.0),
                em: 1.0,
            },
        });
        args.named.insert(
            "radius".into(),
            Value::Dict([("bottom".into(), radius)].into_iter().collect()),
        );
        let red = Stroke {
            paint: Paint::Solid(Color::rgb(255, 65, 54)),
            thickness: Length::em(2.0),
            cap: crate::entities::geometry::LineCap::Round,
            ..Default::default()
        };
        args.named.insert(
            "stroke".into(),
            Value::Dict(
                [
                    ("rest".into(), Value::Color(Color::rgb(0, 116, 217))),
                    ("x".into(), Value::from(red.clone())),
                    ("bottom".into(), Value::None),
                ]
                .into_iter()
                .collect(),
            ),
        );
        let Value::Content(Content::Shape(e)) = call(native_rect, args).unwrap() else {
            panic!("shape");
        };
        let r = e.rect_radius.unwrap();
        assert_eq!(r.top_left.rel, 0.0);
        assert_eq!(r.bottom_left.rel, 0.5);
        assert_eq!(r.bottom_left.abs.em, 1.0);
        assert_eq!(r.bottom_left.abs.abs.0, 2.0);
        let sides = e.rect_stroke.as_ref().unwrap();
        assert_eq!(sides.left, Some(red.clone()));
        assert_eq!(sides.right, Some(red));
        assert!(sides.bottom.is_none());
        assert!(sides.top.is_some());
        assert!(e.stroke.is_none());
        assert_eq!(e.clone().rect_radius, e.rect_radius);
    }
    #[test]
    fn issue8583_all_none_and_invalid_side_inputs_are_distinct() {
        let mut args = Args::positional(vec![]);
        args.named.insert(
            "stroke".into(),
            Value::Dict([("rest".into(), Value::None)].into_iter().collect()),
        );
        let Value::Content(Content::Shape(e)) = call(native_rect, args).unwrap() else {
            panic!("shape");
        };
        assert!(e.stroke.is_none());
        assert_eq!(
            e.rect_stroke.as_ref().unwrap(),
            &crate::entities::sides::Sides::uniform(None)
        );
        for value in [
            Value::Dict([("bottom".into(), Value::Bool(true))].into_iter().collect()),
            Value::Dict(
                [
                    ("bottom".into(), Value::None),
                    ("paint".into(), Value::Color(Color::rgb(0, 0, 0))),
                ]
                .into_iter()
                .collect(),
            ),
        ] {
            let mut args = Args::positional(vec![]);
            args.named.insert("stroke".into(), value);
            assert!(call(native_rect, args).is_err());
        }
    }
}
